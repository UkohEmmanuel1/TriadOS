//! Triad Core: local Unix-socket control plane. Read-only protocol in this milestone.
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, time::Duration};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::UnixListener, signal};

pub const SOCKET_PATH: &str = "/run/triados/triad-core.sock";
const MAX_REQUEST_BYTES: usize = 16 * 1024;

#[derive(Debug, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
enum Request { Ping, SystemStatus, Applications }
#[derive(Debug, Serialize)]
struct Response { ok: bool, message: String, data: Option<serde_json::Value> }
fn response(message: impl Into<String>, data: Option<serde_json::Value>) -> Response { Response { ok: true, message: message.into(), data } }
fn system_snapshot() -> serde_json::Value {
    let uptime = fs::read_to_string("/proc/uptime").unwrap_or_default().split_whitespace().next().unwrap_or("unknown").to_owned();
    let load = fs::read_to_string("/proc/loadavg").unwrap_or_default().split_whitespace().take(3).collect::<Vec<_>>().join(" ");
    let memory = fs::read_to_string("/proc/meminfo").unwrap_or_default().lines().filter(|l| l.starts_with("MemTotal:") || l.starts_with("MemAvailable:")).map(str::to_owned).collect::<Vec<_>>();
    serde_json::json!({"uptime_seconds": uptime, "load_average": load, "memory": memory, "kernel": std::env::consts::OS})
}
fn applications() -> serde_json::Value {
    let mut names=Vec::new();
    for dir in ["/usr/share/applications", "/usr/local/share/applications"] {
        if let Ok(entries)=fs::read_dir(dir) {
            for entry in entries.flatten() {
                let p=entry.path();
                if p.extension().is_some_and(|e| e=="desktop") {
                    let text=fs::read_to_string(p).unwrap_or_default();
                    let name=text.lines().find_map(|line|line.strip_prefix("Name=")).unwrap_or("Unknown");
                    names.push(name.to_owned());
                }
            }
        }
    }
    names.sort(); serde_json::json!({"applications": names})
}
fn handle(bytes: &[u8]) -> Response {
    if bytes.len()>MAX_REQUEST_BYTES { return Response{ok:false,message:"request too large".into(),data:None}; }
    let req: Request=match serde_json::from_slice(bytes) { Ok(r)=>r, Err(e)=>return Response{ok:false,message:format!("invalid request: {e}"),data:None} };
    match req { Request::Ping=>response("Triad Core is alive",None), Request::SystemStatus=>response("system snapshot",Some(system_snapshot())), Request::Applications=>response("desktop applications",Some(applications())) }
}
pub async fn serve() -> Result<()> {
    let socket=Path::new(SOCKET_PATH);
    if let Some(parent)=socket.parent(){fs::create_dir_all(parent).with_context(||format!("create socket directory {}",parent.display()))?;}
    if socket.exists(){anyhow::bail!("socket already exists at {}; stop any running Triad Core and remove stale socket after verifying it is unused",SOCKET_PATH);}
    let listener=UnixListener::bind(socket).with_context(||format!("bind {}",SOCKET_PATH))?;
    #[cfg(unix)] {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(socket,fs::Permissions::from_mode(0o660))?;
    }
    println!("Triad Core listening on {}",SOCKET_PATH);
    loop {
        tokio::select! {
            accept=listener.accept()=> {
                let (mut stream,_)=accept?;
                tokio::spawn(async move {
                    let mut buf=vec![0u8;MAX_REQUEST_BYTES+1];
                    let n=match tokio::time::timeout(Duration::from_secs(5),stream.read(&mut buf)).await {Ok(Ok(n))=>n,_=>return};
                    if n==0{return;}
                    let resp=handle(&buf[..n]);
                    if let Ok(mut bytes)=serde_json::to_vec(&resp){bytes.push(b'\n');let _=stream.write_all(&bytes).await;}
                });
            }
            _=signal::ctrl_c()=>{println!("Triad Core shutting down");break;}
        }
    }
    let _=fs::remove_file(socket);
    Ok(())
}
pub fn status() -> Result<()> {
    if !Path::new(SOCKET_PATH).exists(){println!("Triad Core: not running (socket missing)");return Ok(())}
    println!("Triad Core socket present: {SOCKET_PATH}");
    println!("Use the client protocol or system service to verify responsiveness.");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn ping_works(){let r=handle(br#"{"command":"ping"}"#);assert!(r.ok);assert_eq!(r.message,"Triad Core is alive");}
    #[test] fn status_is_json(){let r=handle(br#"{"command":"system_status"}"#);assert!(r.ok);assert!(r.data.unwrap().get("uptime_seconds").is_some());}
    #[test] fn invalid_request_is_rejected(){let r=handle(b"not-json");assert!(!r.ok);}
    #[test] fn oversized_request_rejected(){let r=handle(&vec![b'a';MAX_REQUEST_BYTES+1]);assert!(!r.ok);}
}

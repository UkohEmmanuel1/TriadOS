//! Triad Core: read-only local control plane over a Unix socket.
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, time::Duration};
use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader}, net::{UnixListener, UnixStream}, signal};

pub const SOCKET_PATH: &str = "/run/triados/triad-core.sock";
const MAX_REQUEST_BYTES: usize = 16 * 1024;

#[derive(Debug, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
enum Request { Ping, SystemStatus, Applications }
#[derive(Debug, Serialize)]
struct Response { ok: bool, message: String, data: Option<serde_json::Value> }
fn response(message: impl Into<String>, data: Option<serde_json::Value>) -> Response { Response { ok: true, message: message.into(), data } }
fn snapshot() -> serde_json::Value {
    let uptime=fs::read_to_string("/proc/uptime").unwrap_or_default().split_whitespace().next().unwrap_or("unknown").to_owned();
    let load=fs::read_to_string("/proc/loadavg").unwrap_or_default().split_whitespace().take(3).collect::<Vec<_>>().join(" ");
    let memory=fs::read_to_string("/proc/meminfo").unwrap_or_default().lines().filter(|l|l.starts_with("MemTotal:")||l.starts_with("MemAvailable:")).map(str::to_owned).collect::<Vec<_>>();
    let kernel=std::process::Command::new("uname").arg("-r").output().map(|o|String::from_utf8_lossy(&o.stdout).trim().to_owned()).unwrap_or_default();
    serde_json::json!({"uptime_seconds":uptime,"load_average":load,"memory":memory,"kernel_release":kernel})
}
fn applications() -> serde_json::Value {
    let mut names=Vec::new();
    for dir in ["/usr/share/applications","/usr/local/share/applications"] {
        if let Ok(entries)=fs::read_dir(dir) { for entry in entries.flatten() { let p=entry.path(); if p.extension().is_some_and(|e|e=="desktop") { let text=fs::read_to_string(p).unwrap_or_default(); let name=text.lines().find_map(|line|line.strip_prefix("Name=")).unwrap_or("Unknown"); names.push(name.to_owned()); } } }
    }
    names.sort(); serde_json::json!({"applications":names})
}
fn handle(bytes:&[u8])->Response {
    if bytes.len()>MAX_REQUEST_BYTES {return Response{ok:false,message:"request too large".into(),data:None};}
    let req:Request=match serde_json::from_slice(bytes){Ok(r)=>r,Err(e)=>return Response{ok:false,message:format!("invalid request: {e}"),data:None}};
    match req {Request::Ping=>response("Triad Core is alive",None),Request::SystemStatus=>response("system snapshot",Some(snapshot())),Request::Applications=>response("desktop applications",Some(applications()))}
}
pub async fn serve()->Result<()> {
    let socket=Path::new(SOCKET_PATH);
    if let Some(parent)=socket.parent(){fs::create_dir_all(parent).with_context(||format!("create socket directory {}",parent.display()))?;}
    if socket.exists(){anyhow::bail!("socket already exists at {}; verify no daemon is running before removing stale socket",SOCKET_PATH);}
    let listener=UnixListener::bind(socket).with_context(||format!("bind {SOCKET_PATH}"))?;
    #[cfg(unix)] {use std::os::unix::fs::PermissionsExt;fs::set_permissions(socket,fs::Permissions::from_mode(0o660))?;}
    println!("Triad Core listening at {SOCKET_PATH}");
    loop { tokio::select! {
        accepted=listener.accept()=>{let(mut stream,_)=accepted?;tokio::spawn(async move{
            let mut buf=vec![0u8;MAX_REQUEST_BYTES+1];
            let n=match tokio::time::timeout(Duration::from_secs(5),stream.read(&mut buf)).await{Ok(Ok(n))=>n,_=>return};if n==0{return;}
            let out=handle(&buf[..n]);if let Ok(mut b)=serde_json::to_vec(&out){b.push(b'\n');let _=stream.write_all(&b).await;}
        });}
        _=signal::ctrl_c()=>{println!("Triad Core stopping");break;}
    }}
    let _=fs::remove_file(socket);Ok(())
}
pub async fn request(command:&str)->Result<()> {
    let request=match command{"ping"=>serde_json::json!({"command":"ping"}),"system_status"=>serde_json::json!({"command":"system_status"}),"applications"=>serde_json::json!({"command":"applications"}),_=>anyhow::bail!("unknown IPC command")};
    let mut stream=UnixStream::connect(SOCKET_PATH).await.with_context(||format!("connect to Core at {SOCKET_PATH}; is it running?"))?;
    let mut bytes=serde_json::to_vec(&request)?;bytes.push(b'\n');stream.write_all(&bytes).await?;
    let mut reader=BufReader::new(stream);let mut response=String::new();reader.read_line(&mut response).await?;
    if response.is_empty(){anyhow::bail!("Core closed connection without a response");}
    let value:serde_json::Value=serde_json::from_str(&response)?;
    if !value["ok"].as_bool().unwrap_or(false){anyhow::bail!("Core request rejected: {}",value["message"]);}
    println!("{}",serde_json::to_string_pretty(&value)?);Ok(())
}
pub fn status()->Result<()> {if Path::new(SOCKET_PATH).exists(){println!("Triad Core socket exists at {SOCKET_PATH}; run 'triad core ping' to verify liveness.");}else{println!("Triad Core: not running (socket missing)");}Ok(())}
#[cfg(test)]
mod tests{use super::*;#[test]fn ping(){assert!(handle(br#"{"command":"ping"}"#).ok);}#[test]fn status_has_kernel(){assert!(handle(br#"{"command":"system_status"}"#).data.unwrap().get("kernel_release").is_some());}#[test]fn invalid_json_rejected(){assert!(!handle(b"broken").ok);}#[test]fn oversized_rejected(){assert!(!handle(&vec![0;MAX_REQUEST_BYTES+1]).ok);}}

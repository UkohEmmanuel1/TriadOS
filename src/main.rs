use anyhow::{Context, Result};
use clap::{Parser,Subcommand};
use std::{fs,path::{Path,PathBuf},process::Command};

#[derive(Parser)]
#[command(name="triad",version,about="Rust-powered TriadOS system toolkit")]
struct Cli { #[command(subcommand)] command: Cmd }
#[derive(Subcommand)]
enum Cmd {
 Info,
 System { #[command(subcommand)] action: SystemCmd },
 Apps { #[command(subcommand)] action: AppsCmd },
 Compat,
 Doctor,
 Config,
 Logs,
}
#[derive(Subcommand)]
enum SystemCmd { Status, Update }
#[derive(Subcommand)]
enum AppsCmd { List, Search { query:String }, Inspect { path:String }, Run { path:String, #[arg(trailing_var_arg=true)] args:Vec<String> } }

fn main()->Result<()> {
 match Cli::parse().command {
  Cmd::Info=>info(),
  Cmd::System{action}=>match action {SystemCmd::Status=>status(),SystemCmd::Update=>update()},
  Cmd::Apps{action}=>match action {AppsCmd::List=>apps_list(),AppsCmd::Search{query}=>apps_search(&query),AppsCmd::Inspect{path}=>inspect(&path),AppsCmd::Run{path,args}=>launch(&path,&args)},
  Cmd::Compat=>compat(), Cmd::Doctor=>doctor(),Cmd::Config=>config(),Cmd::Logs=>logs()
 }
}
fn read(p:&str)->String{fs::read_to_string(p).unwrap_or_default()}
fn os_name()->String{read("/etc/os-release").lines().find_map(|l|l.strip_prefix("PRETTY_NAME=")).map(|s|s.trim_matches('"').to_owned()).unwrap_or("Linux".into())}
fn uname(arg:&str)->String{Command::new("uname").arg(arg).output().ok().map(|o|String::from_utf8_lossy(&o.stdout).trim().to_owned()).unwrap_or("unknown".into())}
fn info()->Result<()>{
 println!("╔══════════════════════════════════════╗\n║              TriadOS              ║\n║       Rust-powered Linux           ║\n╚══════════════════════════════════════╝");
 println!("Version:       {}",env!("CARGO_PKG_VERSION"));println!("OS:           {}",os_name());println!("Kernel:       {}",uname("-r"));println!("Architecture:  {}",uname("-m"));println!("Hostname:     {}",read("/etc/hostname").trim());println!("Desktop:      {}",std::env::var("XDG_CURRENT_DESKTOP").unwrap_or("unknown".into()));Ok(())
}
fn status()->Result<()>{info()?;let mem=read("/proc/meminfo");for line in mem.lines().filter(|l|l.starts_with("MemTotal:")||l.starts_with("MemAvailable:")){println!("{line}");}println!("Uptime: {} seconds",read("/proc/uptime").split_whitespace().next().unwrap_or("?"));Ok(())}
fn update()->Result<()>{if !has("apt-get"){anyhow::bail!("apt-get not found; this command is for Debian-based TriadOS")}println!("Refreshing APT indexes (sudo required)…");let s=Command::new("sudo").args(["apt-get","update"]).status()?;if !s.success(){anyhow::bail!("APT refresh failed ({s})")}println!("Indexes refreshed. Review updates with: sudo apt-get upgrade");Ok(())}
fn has(bin:&str)->bool{std::env::var_os("PATH").unwrap_or_default().to_string_lossy().split(':').any(|d|Path::new(d).join(bin).is_file())}
fn apps_list()->Result<()>{let mut n=0;for dir in ["/usr/share/applications","/usr/local/share/applications"]{if let Ok(entries)=fs::read_dir(dir){for e in entries.flatten(){if e.path().extension().is_some_and(|x|x=="desktop"){println!("{}",e.path().display());n+=1}}}}println!("Desktop entries: {n}");Ok(())}
fn apps_search(q:&str)->Result<()>{let q=q.to_lowercase();let mut n=0;for d in ["/usr/share/applications","/usr/local/share/applications"]{if let Ok(entries)=fs::read_dir(d){for e in entries.flatten(){let p=e.path();if p.extension().is_some_and(|x|x=="desktop"){let text=fs::read_to_string(&p).unwrap_or_default();if text.to_lowercase().contains(&q)||p.to_string_lossy().to_lowercase().contains(&q){println!("{}",p.display());n+=1}}}}}println!("Matches: {n}");Ok(())}
fn executable(p:&Path)->bool{#[cfg(unix)]{use std::os::unix::fs::PermissionsExt;p.metadata().map(|m|m.permissions().mode()&0o111!=0).unwrap_or(false)}#[cfg(not(unix))]{false}}
fn detect(p:&Path)->&'static str{let f=p.file_name().unwrap_or_default().to_string_lossy().to_lowercase();if f.ends_with(".exe"){"Windows executable"}else if f.ends_with(".msi"){"Windows installer"}else if f.ends_with(".appimage"){"AppImage"}else if f.ends_with(".deb"){"Debian package"}else if f.ends_with(".rpm"){"RPM package"}else if f.ends_with(".dmg"){"macOS disk image (inspection only)"}else if f.ends_with(".app"){"macOS app bundle (unsupported)"}else if f.ends_with(".flatpakref"){"Flatpak reference"}else if f.ends_with(".sh"){"Shell script (inspection only)"}else if executable(p){"Executable (unknown format)"}else{"Unknown file"}}
fn inspect(path:&str)->Result<()>{let p=Path::new(path);if !p.exists(){anyhow::bail!("file not found: {path}")}let m=p.metadata()?;println!("Path: {}",p.canonicalize().unwrap_or(p.to_path_buf()).display());println!("Size: {} bytes",m.len());println!("Type: {}",detect(p));println!("Route: {}",match detect(p){"Windows executable"|"Windows installer"=>"Wine, if installed (not sandboxed)","AppImage"=>"Direct launch after executable permission check","Debian package"=>"APT/dpkg package workflow","Flatpak reference"=>"Flatpak workflow","RPM package"=>"Unsupported package on Debian base","macOS disk image (inspection only)"|"macOS app bundle (unsupported)"|"Shell script (inspection only)"=>"Inspection only","Executable (unknown format)"=>"Manual review and confirmation","_"=>"No automatic execution"});Ok(())}
fn launch(path:&str,args:&[String])->Result<()>{let p=Path::new(path);if !p.exists(){anyhow::bail!("file not found: {path}")}let kind=detect(p);let bin=match kind{"Windows executable"|"Windows installer"=>{if !has("wine"){anyhow::bail!("Wine is not installed")} "wine"},"AppImage"=>{if !executable(p){anyhow::bail!("AppImage needs executable permission; inspect it first, then chmod +x")}path},"Executable (unknown format)"=>path,_=>anyhow::bail!("This file type is inspection-only. Run triad apps inspect first.")};println!("Launching {path} ({kind}). Wine is not a sandbox.");print!("Continue? [y/N] ");use std::io::{self,Write};io::stdout().flush()?;let mut answer=String::new();io::stdin().read_line(&mut answer)?;if !matches!(answer.trim().to_lowercase().as_str(),"y"|"yes"){println!("Cancelled.");return Ok(())}let status=Command::new(bin).arg(path).args(args).status().context("process launch failed")?;if !status.success(){eprintln!("Process exited: {status}")}Ok(())}
fn compat()->Result<()>{println!("TriadOS compatibility\nLinux native: supported\nAppImage: permission-checked launch\nWindows: {}",if has("wine"){"Wine detected"}else{"Wine not installed"});println!("macOS: inspection only\nAndroid: future integration");Ok(())}
fn doctor()->Result<()>{println!("TriadOS Doctor");for (name,cmd) in [("APT","apt-get"),("Wine","wine"),("Flatpak","flatpak"),("systemd","systemctl"),("journal","journalctl")]{println!("[{}] {name}",if has(cmd){"OK"}else{"--"})}println!("Kernel: {}",uname("-r"));println!("Desktop: {}",std::env::var("XDG_CURRENT_DESKTOP").unwrap_or("unknown".into()));println!("Session: {}",std::env::var("XDG_SESSION_TYPE").unwrap_or("unknown".into()));Ok(())}
fn config()->Result<()>{let path=std::env::var("XDG_CONFIG_HOME").map(PathBuf::from).unwrap_or_else(|_|PathBuf::from(std::env::var("HOME").unwrap_or(".".into())).join(".config")).join("triados/triados.toml");println!("Config file: {}",path.display());match fs::read_to_string(&path){Ok(t)=>print!("{t}"),Err(e)if e.kind()==std::io::ErrorKind::NotFound=>println!("No config found; defaults active (launch confirmation enabled)."),Err(e)=>return Err(e.into())}Ok(())}
fn logs()->Result<()>{if !has("journalctl"){println!("journalctl unavailable");return Ok(())}println!("Recent system journal (permissions may limit output):");let s=Command::new("journalctl").args(["-n","40","--no-pager"]).status()?;if !s.success(){eprintln!("journalctl exited {s}")}Ok(())}

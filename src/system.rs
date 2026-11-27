use anyhow::Result;
use std::{fs, process::Command};

fn read(path: &str) -> String { fs::read_to_string(path).unwrap_or_default() }
fn uname(flag: &str) -> String {
    Command::new("uname").arg(flag).output().map(|o| String::from_utf8_lossy(&o.stdout).trim().into()).unwrap_or_else(|_| "unknown".into())
}
pub fn info() -> Result<()> {
    let release = read("/etc/os-release");
    let name = release.lines().find_map(|l| l.strip_prefix("PRETTY_NAME=")).unwrap_or("Linux").trim_matches('"');
    println!("TriadOS Rust system toolkit\nHost OS: {name}\nKernel: {}\nArchitecture: {}\nHostname: {}\nDesktop: {}", uname("-r"), uname("-m"), read("/etc/hostname").trim(), std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "unknown".into()));
    Ok(())
}
pub fn status() -> Result<()> {
    info()?;
    for line in read("/proc/meminfo").lines().filter(|l| l.starts_with("MemTotal:") || l.starts_with("MemAvailable:")) { println!("{line}"); }
    println!("Uptime (seconds): {}", read("/proc/uptime").split_whitespace().next().unwrap_or("?"));
    println!("Load average: {}", read("/proc/loadavg").split_whitespace().take(3).collect::<Vec<_>>().join(" "));
    Ok(())
}
pub fn hardware() -> Result<()> {
    println!("CPU:");
    let cpu = read("/proc/cpuinfo");
    if let Some(model) = cpu.lines().find(|l| l.starts_with("model name")) { println!("{model}"); }
    println!("Memory:");
    for line in read("/proc/meminfo").lines().filter(|l| l.starts_with("MemTotal:")) { println!("{line}"); }
    println!("PCI devices (if lspci is installed):");
    match Command::new("lspci").output() {
        Ok(output) if output.status.success() => print!("{}", String::from_utf8_lossy(&output.stdout)),
        _ => println!("lspci not found"),
    }
    println!("USB devices (if lsusb is installed):");
    match Command::new("lsusb").output() {
        Ok(output) if output.status.success() => print!("{}", String::from_utf8_lossy(&output.stdout)),
        _ => println!("lsusb not found"),
    }
    Ok(())
}

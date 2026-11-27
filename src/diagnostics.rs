use anyhow::Result;
use std::{env, path::Path, process::Command};
fn command_exists(cmd: &str) -> bool { env::var_os("PATH").unwrap_or_default().to_string_lossy().split(':').any(|d| Path::new(d).join(cmd).is_file()) }
pub fn doctor() -> Result<()> {
    for (label, command) in [("APT", "apt-get"), ("Flatpak", "flatpak"), ("systemd", "systemctl"), ("Journal", "journalctl"), ("Live image tooling", "lb"), ("Wi-Fi tooling", "nmcli"), ("Hardware inventory", "lspci")] {
        println!("{label}: {}", if command_exists(command) { "available" } else { "not installed" });
    }
    let uname = Command::new("uname").arg("-r").output();
    println!("Kernel: {}", uname.map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_else(|_| "unknown".into()));
    println!("OS release file: {}", if Path::new("/etc/os-release").exists() { "present" } else { "missing" });
    println!("TriadOS status: toolkit diagnostics only; this is not yet a TriadOS booted system.");
    Ok(())
}

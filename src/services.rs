use anyhow::Result;
use std::process::Command;
fn available() -> bool { Command::new("systemctl").arg("--version").output().is_ok() }
pub fn list() -> Result<()> {
    if !available() { anyhow::bail!("systemd/systemctl is not available on this host"); }
    let out = Command::new("systemctl").args(["list-units", "--type=service", "--all", "--no-pager"]).output()?;
    print!("{}", String::from_utf8_lossy(&out.stdout));
    if !out.status.success() { anyhow::bail!("systemctl returned {}", out.status); } Ok(())
}
pub fn status(name: &str) -> Result<()> {
    if name.is_empty() || name.chars().any(|c| !(c.is_ascii_alphanumeric() || "_.@-".contains(c)) ) { anyhow::bail!("invalid service unit name"); }
    if !available() { anyhow::bail!("systemd/systemctl is not available"); }
    let out = Command::new("systemctl").args(["status", "--no-pager", name]).output()?;
    print!("{}", String::from_utf8_lossy(&out.stdout)); eprint!("{}", String::from_utf8_lossy(&out.stderr));
    if !out.status.success() { anyhow::bail!("service status command exited {}", out.status); } Ok(())
}

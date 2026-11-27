use anyhow::Result;
use std::{process::Command, io::{self, Write}};
fn run(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd).args(args).status()?;
    if !status.success() { anyhow::bail!("{cmd} exited with {status}"); } Ok(())
}
fn confirm(action: &str) -> Result<bool> {
    print!("{action} [y/N] "); io::stdout().flush()?;
    let mut answer = String::new(); io::stdin().read_line(&mut answer)?;
    Ok(matches!(answer.trim().to_lowercase().as_str(), "y" | "yes"))
}
fn valid_package(name: &str) -> bool { !name.is_empty() && name.len() <= 200 && name.chars().all(|c| c.is_ascii_alphanumeric() || "+.-".contains(c)) }
pub fn update() -> Result<()> { if !confirm("Run apt package index update (sudo may prompt)?")? { return Ok(()); } run("sudo", &["apt-get", "update"]) }
pub fn search(query: &str) -> Result<()> { run("apt-cache", &["search", query]) }
pub fn install(name: &str) -> Result<()> { if !valid_package(name) { anyhow::bail!("invalid package name"); } if !confirm(&format!("Install package {name} using APT?"))? { return Ok(()); } run("sudo", &["apt-get", "install", "-y", name]) }
pub fn remove(name: &str) -> Result<()> { if !valid_package(name) { anyhow::bail!("invalid package name"); } if !confirm(&format!("Remove package {name} using APT?"))? { return Ok(()); } run("sudo", &["apt-get", "remove", "-y", name]) }
pub fn build_image() -> Result<()> {
    if Command::new("lb").arg("--version").output().is_err() { anyhow::bail!("live-build is missing. Install it before image build."); }
    println!("ISO generation is a placeholder: a validated TriadOS live-build configuration is not yet provided.");
    Ok(())
}

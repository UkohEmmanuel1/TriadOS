use anyhow::Result;
use std::process::Command;
pub fn show(lines: usize) -> Result<()> {
    let count = lines.clamp(1, 1000).to_string();
    let output = Command::new("journalctl").args(["-n", &count, "--no-pager"]).output()?;
    print!("{}", String::from_utf8_lossy(&output.stdout)); eprint!("{}", String::from_utf8_lossy(&output.stderr));
    if !output.status.success() { anyhow::bail!("journalctl exited {}", output.status); } Ok(())
}

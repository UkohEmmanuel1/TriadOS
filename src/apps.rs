use anyhow::{Context, Result};
use std::{fs, path::Path, process::Command};

fn entries() -> Vec<String> {
    let mut list = Vec::new();
    for dir in ["/usr/share/applications", "/usr/local/share/applications", "/var/lib/flatpak/exports/share/applications"] {
        if let Ok(read) = fs::read_dir(dir) {
            for entry in read.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "desktop") { list.push(path.display().to_string()); }
            }
        }
    }
    list.sort(); list.dedup(); list
}
fn desktop_name(contents: &str) -> &str { contents.lines().find_map(|l| l.strip_prefix("Name=")).unwrap_or("(unnamed)") }
pub fn list() -> Result<()> {
    let files = entries();
    for path in &files { println!("{} — {}", desktop_name(&fs::read_to_string(path).unwrap_or_default()), path); }
    println!("Applications found: {}", files.len()); Ok(())
}
pub fn search(query: &str) -> Result<()> {
    let mut matches = 0;
    for path in entries() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        if content.to_lowercase().contains(&query.to_lowercase()) { println!("{} — {}", desktop_name(&content), path); matches += 1; }
    }
    println!("Matches: {matches}"); Ok(())
}
pub fn inspect(path: &str) -> Result<()> {
    let file = Path::new(path);
    if !file.is_file() { anyhow::bail!("not a regular file"); }
    let meta = file.metadata()?;
    println!("Path: {}\nSize: {} bytes\nExtension: {}\nExecution policy: inspection only; launch requires explicit confirmation", file.canonicalize().unwrap_or_else(|_| file.to_path_buf()).display(), meta.len(), file.extension().and_then(|x| x.to_str()).unwrap_or("none"));
    Ok(())
}
pub fn launch_desktop(path: &str) -> Result<()> {
    let contents = fs::read_to_string(path).context("cannot read desktop entry")?;
    if !contents.lines().any(|l| l.trim() == "[Desktop Entry]") { anyhow::bail!("not a desktop-entry file"); }
    let exec = contents.lines().find_map(|l| l.strip_prefix("Exec=")).context("Exec field missing")?;
    if exec.contains(';') || exec.contains('|') || exec.contains('&') || exec.contains("$(") { anyhow::bail!("unsafe Exec field rejected"); }
    let args: Vec<String> = exec.split_whitespace().filter(|s| !s.starts_with('%')).map(str::to_owned).collect();
    if args.is_empty() { anyhow::bail!("empty Exec command"); }
    println!("Application command: {}", args.join(" "));
    if !confirm()? { println!("Cancelled."); return Ok(()); }
    let status = Command::new(&args[0]).args(&args[1..]).status()?;
    if !status.success() { anyhow::bail!("application exited with {status}"); }
    Ok(())
}
fn confirm() -> Result<bool> {
    use std::io::{self, Write};
    print!("Launch application? [y/N] "); io::stdout().flush()?;
    let mut input = String::new(); io::stdin().read_line(&mut input)?;
    Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"))
}

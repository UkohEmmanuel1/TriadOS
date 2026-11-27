use anyhow::{Context, Result};
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
struct Settings {
    system_name: Option<String>,
    confirm_launch: Option<bool>,
    #[serde(default)]
    telemetry: Option<bool>,
}
pub fn show() -> Result<()> {
    let paths = ["/etc/triados/triados.toml", "config/triados.toml"];
    let path = paths.iter().find(|p| Path::new(p).exists()).context("TriadOS configuration file not found")?;
    let contents = fs::read_to_string(path)?;
    let settings: Settings = toml::from_str(&contents).context("configuration syntax invalid")?;
    println!("Loaded: {path}\nSystem name: {}\nLaunch confirmation: {}\nTelemetry: {}", settings.system_name.as_deref().unwrap_or("TriadOS"), settings.confirm_launch.unwrap_or(true), settings.telemetry.unwrap_or(false));
    println!("Raw configuration:\n{contents}");
    Ok(())
}

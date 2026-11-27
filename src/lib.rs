//! TriadOS Rust system layer.
//! The first milestone is a safe user-space toolkit over the Linux host.
//! Privileged operations are delegated to established Linux tools and require explicit confirmation.

pub const SYSTEM_NAME: &str = "TriadOS";
pub const CONFIG_PATH: &str = "/etc/triados/triados.toml";

pub fn runtime_language() -> &'static str { "Rust" }

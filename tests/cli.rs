use std::process::Command;fn cli(a:&[&str])->std::process::Output{Command::new(env!("CARGO_BIN_EXE_triad")).args(a).output().unwrap()}
#[test]fn help(){assert!(cli(&["--help"]).status.success())}
#[test]fn info(){let o=cli(&["info"]);assert!(o.status.success());assert!(String::from_utf8_lossy(&o.stdout).contains("TriadOS"))}
#[test]fn doctor(){assert!(cli(&["doctor"]).status.success())}
#[test]fn compat(){assert!(cli(&["compat"]).status.success())}
#[test]fn list_apps(){assert!(cli(&["apps","list"]).status.success())}
#[test]fn bad_command(){assert!(!cli(&["unknown"]).status.success())}
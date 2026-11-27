use std::process::Command;
#[test] fn cli_help(){let o=Command::new(env!("CARGO_BIN_EXE_triad")).arg("--help").output().unwrap();assert!(o.status.success());assert!(String::from_utf8_lossy(&o.stdout).contains("TriadOS"));}
#[test] fn info_runs(){let o=Command::new(env!("CARGO_BIN_EXE_triad")).arg("info").output().unwrap();assert!(o.status.success());assert!(String::from_utf8_lossy(&o.stdout).contains("TriadOS"));}
#[test] fn compatibility_runs(){let o=Command::new(env!("CARGO_BIN_EXE_triad")).arg("compat").output().unwrap();assert!(o.status.success());assert!(String::from_utf8_lossy(&o.stdout).contains("Linux native"));}

# TriadOS roadmap

Rust-first Linux distribution based on Debian Trixie and the upstream Linux kernel.

## Foundation
- [x] Rust CLI and modular system toolkit
- [x] System status, hardware, network and storage diagnostics
- [x] Desktop application discovery and confirmed launch
- [x] Read-only systemd service inspection
- [x] APT wrappers with confirmation
- [x] TOML configuration and journal diagnostics
- [x] Triad Core Unix socket prototype
- [x] Initial Debian Trixie amd64 live-build profile
- [x] Initial Sway/Waybar/LightDM desktop session configuration
- [x] Rust binary staging into the live filesystem
- [ ] Successful clean ISO build
- [ ] QEMU boot and login validation
- [ ] Core daemon protocol hardening and service authorization

## Desktop and applications
- [ ] Custom Rust Triad Shell, dock/panel, launcher and workspace UI
- [ ] Settings application and accessibility
- [ ] Files GUI, Store, task manager, monitor and native apps
- [ ] Notifications and quick settings integration
- [ ] Audio, display, power and peripheral controls

## Distribution and installation
- [x] Initial live-build profile
- [ ] Installer configuration, disk/partition workflow and encryption
- [ ] First-boot wizard and account setup
- [ ] Reproducible signed releases and package repository
- [ ] Upgrade, rollback, backup and recovery workflows
- [ ] Secure Boot validation and hardware compatibility matrix

## Security, AI and developer platform
- [ ] AppArmor profiles and sandboxed application launch
- [ ] Permission and security center
- [ ] Signed update and repository verification
- [ ] AI assistant and optional local inference integration
- [ ] Developer environment and container/VM management

## Build sequence
1. Verify Rust CI and fix any compiler/lint failures.
2. Build the live ISO on Debian Trixie amd64.
3. Boot and test the ISO in QEMU.
4. Configure and test Calamares on disposable virtual disks.
5. Build the custom Rust desktop shell and native applications.
6. Harden Core IPC and system integration before enabling privileged operations.

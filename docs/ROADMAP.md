# TriadOS roadmap

Rust-first Linux distribution built on the upstream Linux kernel.

## Toolkit and core prototype
- [x] Rust CLI foundations
- [x] System status and basic inventory
- [x] Desktop application discovery/inspection/confirmed launch
- [x] Read-only systemd service inspection
- [x] APT search/update/install/remove wrappers with confirmation
- [x] TOML config and journal/diagnostics
- [x] Core daemon prototype over local Unix socket
- [x] IPC ping/system status/application list commands
- [x] Network interface and storage/space CLI views
- [ ] CI green on this foundation (current run failure needs diagnosis)
- [ ] Socket activation, user/group policy and formal protocol versioning

## Distribution features still to implement
- [ ] Custom desktop shell, dock/panel, launcher, workspace UI and notifications
- [ ] Settings application and accessibility
- [ ] Files GUI, store, task manager, monitor, terminal and native apps
- [ ] Hardware controls: Wi-Fi, Bluetooth, audio, display, power, storage, peripherals
- [ ] Security center, app sandbox and permissions model
- [ ] Update manager, signed packages, backup, snapshots, recovery
- [ ] AI assistant, local inference/model UI, developer platform
- [ ] Live ISO, installer, first-boot wizard and QEMU boot validation
- [ ] Release automation, reproducible builds, signing and hardware test matrix

## Capability inventory
Core; boot/recovery; desktop shell; app store; package integration; file/storage; hardware; networking; security/privacy; accounts; settings/accessibility; audio; power; notifications; built-in apps; developer tools; AI; updates; backup; virtualization; web/PWA; peripherals; distribution infrastructure; installer; localization; monitoring; services; optional cloud; docs/community.

## Build order
1. Rust system core and service framework
2. IPC client/protocol and system integrations
3. Desktop shell and settings
4. Files, Store, Monitor and apps
5. Hardware/network/storage/security
6. Updates, backup and recovery
7. Installer, ISO and QEMU boot testing
8. AI/developer ecosystem

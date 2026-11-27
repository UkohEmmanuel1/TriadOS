# TriadOS roadmap

TriadOS is an ambitious Rust-first Linux distribution. It will use the upstream Linux kernel and reuse battle-tested infrastructure where sensible.

## Milestones
- [x] Rust CLI/toolkit foundation
- [x] System info/status, basic hardware inventory, app discovery
- [x] Read-only systemd service inspection
- [x] Explicitly-confirmed APT wrappers
- [x] TOML configuration reader, journal access, diagnostics
- [ ] Core daemon and structured service runtime
- [ ] D-Bus service APIs and permission boundaries
- [ ] Hardware/network/storage APIs with robust error handling
- [ ] Settings application
- [ ] Desktop shell (panel, launcher, dock, workspaces, notifications)
- [ ] Files application
- [ ] Store with trusted package sources, install progress and rollback
- [ ] Monitor/task manager
- [ ] Security center, sandbox integration and firewall UI
- [ ] Update manager and recovery tooling
- [ ] AI assistant (local inference optional)
- [ ] Installer, live ISO and reproducible build pipeline
- [ ] QEMU automated boot test and physical hardware test matrix

## 30 capability areas
1. Core architecture, lifecycle, IPC, configuration and recovery
2. Boot flow, splash, safe mode and Secure Boot compatibility
3. Desktop shell, dock, launcher, workspace and window integration
4. App store, discovery, updates, permissions and rollback
5. Package manager UI, APT/Flatpak and repo policy
6. Files, search, preview, archives, trash, mounts and USB
7. Hardware inventory, drivers, sensors, battery and thermal
8. Network settings, Wi-Fi, VPN, firewall and diagnostics
9. Security, sandbox, audit, encryption and permissions
10. User accounts, groups, sessions and identity
11. Centralized settings and accessibility
12. Audio routing and media controls
13. Power profiles, sleep, battery and screen timeouts
14. Notifications, history, focus and DND
15. Accessibility features and assistive technology
16. Built-in apps (terminal, editor, calculator, calendar, notes, backup, etc.)
17. Developer tools, containers, VMs, SSH, Git and CI/CD
18. AI assistant, local model management and automation
19. Signed updates, history, rollback and maintenance
20. Backup, snapshots and recovery environment
21. Virtualization and application isolation
22. Browser/PWA integration and URL handling
23. Printer/scanner/camera and peripherals
24. Distribution releases, CI, signing, mirrors and reproducible builds
25. Installer, disk partitioning, encryption and first-run wizard
26. Localization, keyboard, timezone and regional settings
27. Monitoring, process control, resource charts and alerting
28. Rust service/daemon ecosystem and IPC
29. Optional cloud/device services; offline-first behavior
30. Documentation, contributor workflow, support and community

## Build sequence
1. Rust core + service contracts
2. D-Bus daemon and system integration APIs
3. Desktop experience + settings
4. Files/store/monitor apps
5. Hardware, networking, storage and security
6. Updates, backup and recovery
7. Installer, ISO, QEMU boot tests and release automation
8. Developer and AI ecosystem

## Non-goals for early milestones
- Rewriting Linux kernel
- Reimplementing APT dependency resolution
- Building a browser engine
- Building a Wayland compositor before validating UX needs
- Treating Wine compatibility as application sandboxing

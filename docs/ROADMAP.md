# TriadOS roadmap

TriadOS is a Rust-first Linux distribution using the upstream Linux kernel and established components.

## Implemented toolkit
- [x] CLI foundation
- [x] System status and basic hardware inventory
- [x] Application discovery, search and inspection
- [x] Read-only systemd service inspection
- [x] Confirmed APT wrappers
- [x] TOML config reader, journal access, diagnostics
- [x] Core read-only Unix-socket daemon prototype and JSON IPC handlers
- [x] systemd unit draft for Core

## Not yet delivered
- [ ] IPC client and stable versioned protocol
- [ ] Production authorization, socket ownership/groups, audit and policy
- [ ] Core service manager and event bus
- [ ] GUI shell and settings
- [ ] Files app, Store, Monitor and native apps
- [ ] Hardware, network, storage, power and audio control surfaces
- [ ] App sandbox policy and security center
- [ ] Update, backup, restore and recovery
- [ ] AI assistant and local model management
- [ ] Live image, installer and boot tests

## Full capability areas
1. Core architecture, service lifecycle, IPC, configuration and recovery
2. Boot flow, splash, safe mode and Secure Boot
3. Desktop shell, dock, launcher, workspaces and notifications
4. App store, discovery, updates, permissions and rollback
5. Package manager integration, APT/Flatpak and repo policy
6. Files, search, preview, archives, trash, mounts and USB
7. Hardware inventory, drivers, sensors, battery and thermal
8. Network settings, Wi-Fi, VPN, firewall and diagnostics
9. Security, sandbox, audit, encryption and permissions
10. Users, groups, sessions and identity
11. Centralized settings and accessibility
12. Audio routing and media controls
13. Power profiles, sleep, battery and timeouts
14. Notifications, history, focus and DND
15. Accessibility and assistive technology
16. Built-in applications
17. Developer tools, containers, VMs, SSH, Git and CI/CD
18. AI assistant, local model management and automation
19. Signed updates, rollback and maintenance
20. Backup, snapshots and recovery environment
21. Virtualization and application isolation
22. Browser/PWA integration
23. Printers, scanners, cameras and peripherals
24. Distribution releases, CI, signing, mirrors and reproducible builds
25. Installer, partitioning, encryption and first-run
26. Localization and regional settings
27. Monitoring, process control and resource charts
28. Rust service ecosystem and IPC
29. Optional cloud/device services, offline-first
30. Documentation, contributors and community

## Build phases
1. Core Rust architecture and service contracts
2. IPC and read-only system integration
3. Desktop shell and settings
4. Files, store and monitor applications
5. Hardware, networking, storage and security
6. Updates, backups and recovery
7. Installer, ISO, QEMU boot tests and release pipeline
8. AI and developer ecosystem

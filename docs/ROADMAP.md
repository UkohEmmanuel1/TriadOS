# TriadOS roadmap

TriadOS is a Rust-first Linux distribution based on Debian Trixie and the upstream Linux kernel. This roadmap is an acceptance plan: a checkbox is complete only after implementation and the stated validation evidence exists.

## Milestone A — bootable Alpha 1

### ISO and virtual-machine validation
- [ ] Build a clean amd64 hybrid ISO from a clean Debian Trixie environment.
- [ ] Publish the ISO and SHA-256 checksum as CI artifacts/releases.
- [ ] Boot the ISO in QEMU using both BIOS and UEFI firmware.
- [ ] Verify the live user reaches a graphical login and can start the Triad session.
- [ ] Verify keyboard, display modes, audio, networking, storage detection, suspend/resume where virtual hardware supports it.
- [ ] Add automated boot smoke tests and retain logs/screenshots as artifacts.

### Triad desktop environment
- [ ] Replace the temporary Sway-only session with a coherent Triad desktop shell written in Rust.
- [ ] Implement panel, application launcher, dock/task switcher, workspace controls and system tray.
- [ ] Implement notifications, quick settings, session controls and accessibility basics.
- [ ] Define stable desktop IPC and D-Bus integration; keep compositor responsibilities separate from the shell.
- [ ] Provide a fallback session and safe-mode boot option.

### Native graphical applications
- [ ] Triad Settings: appearance, display, keyboard, sound, network, power, users and privacy.
- [ ] Triad Files: browse files, removable media, mounts and basic file operations.
- [ ] Triad Terminal and system monitor.
- [ ] Triad Software: discover, install, update and remove signed packages with clear permissions.
- [ ] Consistent design system, localization, keyboard navigation and accessibility.

### Hardware and settings
- [ ] Hardware inventory and driver/firmware status.
- [ ] NetworkManager integration for Wi-Fi, Ethernet and VPN status.
- [ ] Audio device selection and volume controls via PipeWire.
- [ ] Display arrangement, resolution, scaling and brightness where supported.
- [ ] Bluetooth, printer, battery, power profiles and removable-device management.
- [ ] Settings changes must use documented system APIs and least-privilege helpers.

### Installer and first boot
- [ ] Configure Calamares (or a deliberately selected alternative) with Triad branding and tested modules.
- [ ] Support guided disk selection, partitioning, EFI, bootloader installation and filesystem choices.
- [ ] Offer encryption with clear recovery-key guidance.
- [ ] Validate install, reboot, bootloader and installed-system startup in disposable QEMU disks.
- [ ] First-boot wizard: create user, set password, hostname, timezone, keyboard and privacy choices.
- [ ] Create a non-root daily user, configure sudo policy and remove live-session credentials.

### Security and application isolation
- [ ] Enable and test AppArmor profiles for system services and desktop applications.
- [ ] Establish application sandbox policy (Flatpak/bubblewrap or another maintained isolation layer).
- [ ] Permission prompts for filesystem, devices, network and background execution.
- [ ] Harden systemd units, IPC authorization, polkit rules and privileged helpers.
- [ ] Signed packages, signed repository metadata, secure update verification and vulnerability response.
- [ ] Threat model, security tests and documented disclosure process.

### Updates, recovery and release engineering
- [ ] Host an APT repository with signed Release metadata and reproducible package publishing.
- [ ] Define versioning, release channels, changelog and supported upgrade paths.
- [ ] Implement safe update flow, interrupted-update handling and rollback strategy.
- [ ] Provide recovery environment, filesystem checks and documented backup/restore.
- [ ] Generate SBOMs, checksums, signed release artifacts and build provenance.
- [ ] Hardware compatibility matrix and release qualification checklist.

## Existing foundation
- [x] Rust CLI and modular system toolkit.
- [x] System status, hardware, network and storage diagnostics.
- [x] Desktop application discovery and confirmed launch.
- [x] Read-only systemd service inspection.
- [x] APT wrappers with confirmation.
- [x] TOML configuration and journal diagnostics.
- [x] Triad Core Unix socket prototype.
- [x] Initial Debian Trixie amd64 live-build profile.
- [x] Initial Sway/Waybar/LightDM session configuration.
- [x] Rust binary staging into the live filesystem.

## Engineering sequence
1. Make CI compile, lint and test the Rust code with zero warnings.
2. Build the ISO in CI and retain it as a downloadable artifact.
3. Boot BIOS/UEFI images in QEMU; verify graphical login and core hardware paths.
4. Configure and test the installer on disposable virtual disks.
5. Build the custom desktop shell and native applications incrementally.
6. Complete hardware/settings integrations and first-boot experience.
7. Harden sandboxing, IPC, privilege boundaries and update verification.
8. Establish signed repository, recovery and release processes.

## Release gate

Do not label TriadOS a complete OS release until the ISO has been built from a clean environment, booted and tested in QEMU, installed to a virtual disk, rebooted into the installed system, and passed the security/update/recovery checks. Configuration files and CI syntax checks alone are not proof of a working distribution.

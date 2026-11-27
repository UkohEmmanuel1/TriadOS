# TriadOS

**TriadOS is a Rust-first Linux distribution in development, based on Debian Trixie and the upstream Linux kernel.**

The repository currently contains a Rust system-management toolkit/Core prototype and an initial Debian live-build profile. It is not yet a validated, complete desktop operating system: a successful ISO build, QEMU graphical-login test, installer workflow and installed-system test are still release gates.

## Current foundation
- Rust CLI for system status, hardware, network and storage diagnostics
- Application discovery and confirmed desktop-entry launch
- Read-only systemd service inspection and confirmed APT operations
- TOML configuration, journal diagnostics and a local Unix-socket Core prototype
- Debian Trixie amd64 live-build profile
- Initial Sway/Waybar/LightDM graphical session configuration
- Calamares package included as an installer starting point (not yet configured)

## Build the Rust toolkit

Requirements: Rust stable and Cargo.

```sh
cargo build --release
cargo test --all-targets
cargo run -- info
cargo run -- doctor
```

## Build a live ISO

Use a Debian Trixie amd64 build host. See [docs/DISTRIBUTION_BUILD.md](docs/DISTRIBUTION_BUILD.md).

```sh
sudo apt-get install -y live-build debootstrap xorriso squashfs-tools
sudo ./scripts/build-iso.sh
```

The expected output is `live-image-amd64.hybrid.iso`. A successful build and QEMU graphical-login validation have not yet been established. Never install an unvalidated image on a physical machine.

## Target Alpha 1 scope

The next milestone is to deliver:
1. A successfully built ISO with repeatable CI artifacts and checksum.
2. Verified BIOS/UEFI boot and graphical login in QEMU.
3. A custom Rust Triad desktop shell, beyond the current temporary Sway session.
4. Native graphical apps and a complete settings/hardware-management experience.
5. A configured and tested installer, user creation and first-boot wizard.
6. App sandboxing, least-privilege system integration, signed updates and recovery.
7. A signed package repository and release infrastructure.

See [docs/ROADMAP.md](docs/ROADMAP.md) for acceptance criteria and implementation status.

## Safety and release status

The current Core daemon is a prototype, not a privileged command-execution API. Desktop-entry execution is not a sandbox. Calamares is only included as a package; its installation workflow still needs configuration and testing. Sway is a temporary desktop foundation, not the custom Triad desktop. TriadOS uses the upstream Linux kernel and Debian package infrastructure rather than claiming to implement its own kernel or package solver.

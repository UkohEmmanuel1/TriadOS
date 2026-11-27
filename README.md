# TriadOS

**TriadOS is a Rust-first Linux distribution built on Debian Trixie and the upstream Linux kernel.**

The repository contains a Rust system-management toolkit/Core prototype and an initial Debian live-build distribution profile. The live image configuration is new and has not yet been validated by a complete ISO build and QEMU boot test.

## Current components

- Rust CLI for system status, hardware, network and storage diagnostics
- Application discovery and confirmed desktop-entry launch
- Read-only systemd service inspection
- APT package operations with confirmation
- TOML configuration, journal logs and diagnostics
- Triad Core prototype using a local Unix-domain socket
- Debian Trixie amd64 live-build configuration
- Initial Sway/Waybar/LightDM desktop session
- systemd unit for Triad Core
- Calamares package included as an installer starting point

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

Expected output: `live-image-amd64.hybrid.iso`.

## Desktop

The first graphical milestone uses Sway (Wayland), Waybar, Wofi, Foot and Mako. This is a practical foundation while the custom Rust Triad Shell is developed. Log in through LightDM and choose the TriadOS session.

## Safety and release status

The live-build profile is not yet a validated release. Build and boot it in QEMU before using it on hardware. Calamares is only included as a package; its installation workflow still needs configuration and testing. Package operations use APT. The Core daemon is a prototype, not a privileged command-execution API. Desktop-entry execution is not a sandbox.

TriadOS does not replace the Linux kernel, Debian package solver, Wayland compositor, or browser engine.

## Roadmap

See [docs/ROADMAP.md](docs/ROADMAP.md) for implementation status.

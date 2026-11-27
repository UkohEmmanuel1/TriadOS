# TriadOS

**TriadOS is a Rust-first Linux distribution in development, based on Debian Trixie and the upstream Linux kernel.**

The OS management tools and distribution-build orchestration are implemented in Rust. Debian live-build and standard Linux utilities provide image assembly and host integration. This is not yet a validated desktop OS: ISO boot, graphical login, installer and installed-system tests remain release gates.

## Rust implementation
- Rust CLI for system, hardware, network and storage diagnostics
- Application discovery and confirmed desktop-entry launch
- systemd service inspection and confirmed APT operations
- TOML configuration and a local Unix-socket Core prototype
- Rust-owned Debian Trixie live-image configuration, orchestration and SHA-256 generation

## Build and test
```sh
cargo build --release
cargo test --all-targets
cargo run -- info
cargo run -- doctor
```

## Build a live ISO
On a Debian Trixie amd64 host, install live-build, debootstrap, xorriso, squashfs-tools, GRUB utilities, mtools and dosfstools. Then:
```sh
cargo build --release
sudo target/release/triad image
```
The command produces `live-image-amd64.hybrid.iso` and its checksum. A successful build and QEMU graphical-login validation are not yet guaranteed. Do not install an unvalidated image on physical hardware.

See [docs/ROADMAP.md](docs/ROADMAP.md) and [docs/DISTRIBUTION_BUILD.md](docs/DISTRIBUTION_BUILD.md).

# TriadOS Rust Edition
Rust-first system toolkit and compatibility launcher for a Debian-based Linux desktop.

**Status:** Early rewrite. CLI features are implemented in source; ISO generation is an experimental path and not boot-tested.

## Features
- System information and status
- APT index refresh helper
- Application list/search from desktop-entry folders
- File inspection and compatibility detection
- Confirmation-gated execution for AppImage and native executable files
- Wine routing for EXE/MSI (Wine is not a sandbox)
- Diagnostics, config and journal commands
- Debian live-build integration scaffold

## Build
Install Rust stable, then:
```sh
cargo build --release
cargo test
bash ./scripts/check.sh
```

## Commands
```sh
triad info
triad system status
triad system update
triad apps list
triad apps search firefox
triad apps inspect ./installer.exe
triad apps run ./setup.exe
triad compat
triad doctor
triad config
triad logs
```

macOS app bundles and DMGs are inspection-only. Unknown file types are not auto-run.

## ISO experiment
On Debian 13: install `live-build debootstrap squashfs-tools xorriso`, then run `sudo bash ./scripts/build-iso.sh`. The output must be tested in QEMU before release. TriadOS uses the Linux kernel and Debian build tools; it does not implement a new kernel.

MIT license.

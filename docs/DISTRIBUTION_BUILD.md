# TriadOS live image build

The Rust `triad image` command owns the Debian Trixie live-build configuration,
binary staging, build orchestration and checksum generation. Debian live-build
still performs the distro-specific chroot and boot-image assembly.

## Build host

Use Debian Trixie amd64 with Rust stable, Cargo, live-build, debootstrap,
xorriso, squashfs-tools, GRUB utilities, mtools and dosfstools. Root is required
for chroot construction.

```sh
sudo apt-get update
sudo apt-get install -y live-build debootstrap xorriso squashfs-tools grub-pc-bin grub-efi-amd64-bin mtools dosfstools
cargo build --release
sudo target/release/triad image
```

Outputs: `live-image-amd64.hybrid.iso` and
`live-image-amd64.hybrid.iso.sha256`.

## QEMU test

```sh
qemu-system-x86_64 -m 4096 -smp 2 -cdrom live-image-amd64.hybrid.iso -boot d
```

## Limitations

This is not yet a validated release ISO. Sway is a temporary desktop, not the
custom Triad Rust shell. Calamares is only included as a package and still
needs a complete installer configuration. Boot, graphical login, hardware,
installation, encryption, updates and recovery require testing. TriadOS uses
the Debian Linux kernel and Debian package infrastructure. Never install an
unvalidated image on a production machine.

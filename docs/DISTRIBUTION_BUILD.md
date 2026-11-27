# TriadOS live image build

This repository now contains an initial Debian Trixie live-build profile for amd64.
It uses the upstream Debian kernel and boot stack, Sway/Waybar for the initial
Wayland desktop, LightDM for graphical login, NetworkManager for networking,
Calamares as the installer frontend, and the Rust `triad` binary as the system
management entry point.

## Build host

Use a Debian Trixie amd64 build host (or a compatible Debian environment) with
Rust stable, Cargo, live-build, debootstrap, xorriso, and sufficient disk space.
The build must run as root because live-build mounts and configures a chroot.

Install the build tools:

```sh
sudo apt-get update
sudo apt-get install -y live-build debootstrap xorriso squashfs-tools   grub-pc-bin grub-efi-amd64-bin mtools
```

Build:

```sh
sudo ./scripts/build-iso.sh
```

Expected output: `live-image-amd64.hybrid.iso`.

## Test in QEMU

```sh
qemu-system-x86_64 -m 4096 -smp 2 -enable-kvm   -cdrom live-image-amd64.hybrid.iso -boot d
```

On hosts without KVM, remove `-enable-kvm`.

## Current scope and limitations

- This is a first live-build profile, not a validated release ISO.
- The desktop is Sway with Waybar, Wofi, Foot and Mako; it is not yet a custom
  Triad Rust shell.
- Calamares is included as a frontend package; a complete TriadOS installer
  configuration, partitioning policy, branding, encryption flow and install
  hooks still need to be authored and tested.
- The live session, login manager, firmware availability, package names and
  bootloader behavior must be verified by running a full image build and QEMU.
- No kernel fork is maintained. TriadOS uses the Debian Linux kernel.
- Do not install this image on a production machine until installation and
  recovery have been tested on disposable virtual disks.

## Build outputs

The ISO and checksum are local build artifacts and should be attached to a
versioned GitHub release after validation, not committed to the source tree.

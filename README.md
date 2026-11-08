# TriadOS — Linux desktop with cross-platform compatibility helpers

**Status: prototype source project. Not a finished operating system or a tested ISO.**

TriadOS uses Debian live-build and KDE Plasma to create a Linux desktop live ISO. It includes a cross-platform format launcher.

## Compatibility
- Linux: native executable files and packages.
- Windows .exe/.msi: uses Wine, which cannot run every Windows program.
- macOS .app/.dmg: detection only. Darling is experimental and graphical macOS applications are **not supported** in this prototype. Final Cut Pro and Logic Pro are unsupported.

## Build
On Debian 13 (trixie), with suitable disk space and root access:

```sh
sudo apt update
sudo apt install live-build debootstrap squashfs-tools xorriso python3
sudo ./scripts/build-iso.sh
```

If successful, test `live-image-amd64.hybrid.iso` with QEMU:

```sh
qemu-system-x86_64 -m 4096 -smp 4 -enable-kvm -cdrom live-image-amd64.hybrid.iso -boot d
```

This is a live image, not a complete disk installer; no ISO has been built or verified yet.

## Launcher

```sh
python3 launcher/triad_launcher.py --help
python3 launcher/triad_launcher.py --inspect ~/Downloads/MyApp.app
python3 launcher/triad_launcher.py ~/Downloads/setup.exe
sudo ./scripts/install-launcher.sh
```

The launcher prompts before launching. Wine is not a security sandbox. Do not launch unknown executables.

## Checks

```sh
python3 -m unittest discover -s tests -v
bash -n scripts/*.sh
```

## Roadmap
1. Build and boot-test the live ISO.
2. Add installer and UEFI integration.
3. Improve Wine application isolation and guidance.
4. Research Darling's current supported command-line subset.
5. Add package updates, signed releases and accessibility.

Project code MIT-licensed; third-party components retain their own licenses.

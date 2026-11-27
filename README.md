# TriadOS

**TriadOS is a Rust-first Linux distribution project.** The current repository is a user-space Rust toolkit and architectural foundation—not yet a custom bootable OS or desktop environment.

## Current foundation
- Rust CLI with system status and basic hardware inventory
- Application discovery from desktop-entry directories, search and inspect
- Launching desktop entries with confirmation
- systemd service listing/status (read-only)
- APT search/update/install/remove wrappers with confirmation
- TOML configuration reader
- Journal log viewing and basic host diagnostics

## Requirements
- Rust stable
- Linux host for the system integration commands
- Debian/Ubuntu-like host for APT operations
- systemd for service inspection

## CLI
```sh
triad info
triad system status
triad system hardware
triad apps list
triad apps search firefox
triad apps inspect /path/to/file
triad apps launch /usr/share/applications/example.desktop
triad services list
triad services status NetworkManager.service
triad packages search editor
triad packages update
triad packages install package-name
triad packages remove package-name
triad logs --lines 100
triad doctor
triad image
```

## Roadmap
See [docs/ROADMAP.md](docs/ROADMAP.md) for the full distribution plan and milestone status.

## Safety
Privileged package actions require confirmation and use established package tools. Service management is read-only in this milestone. Desktop-entry execution is not a sandbox. The ISO command is not a validated image build pipeline.

## Architecture direction
Use the upstream Linux kernel and established components for boot, networking, audio, graphics, package solving, and application sandboxing while building TriadOS services, desktop experience, configuration, orchestration, and applications in Rust. Do not claim the project is a bootable OS until image builds and VM boot tests are passing.

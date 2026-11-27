# TriadOS — Rust rewrite
TriadOS is a Debian-based Linux distribution concept. Project-specific tooling is Rust. Linux kernel and Debian build tooling remain upstream. This is an early CLI, not a completed OS or tested ISO.

Commands: triad info; triad system status; triad apps list/search/inspect/launch/run; triad compat; triad doctor; triad config; triad logs; triad update; triad image.

The project shell helper scripts have been removed. Updates and image builds invoke external system tools through Rust process APIs. Application execution is confirmation-gated. Wine is not a sandbox.

Still incomplete: desktop shell, package transaction UI, isolation, installer, boot/QEMU tests.

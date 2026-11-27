#!/usr/bin/env bash
set -euo pipefail
if [[ "$(id -u)" -ne 0 ]]; then echo "Run with sudo." >&2; exit 1; fi
command -v lb >/dev/null || { echo "Install live-build first (apt install live-build)." >&2; exit 1; }
command -v cargo >/dev/null || { echo "Install Rust stable/Cargo first." >&2; exit 1; }
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT
cargo build --release
mkdir -p config/includes.chroot/usr/local/bin
install -m 0755 target/release/triad config/includes.chroot/usr/local/bin/triad
lb config --distribution trixie --architectures amd64 --archive-areas main contrib non-free non-free-firmware --binary-images iso-hybrid
lb build
echo "ISO built; boot-test in QEMU before distributing."

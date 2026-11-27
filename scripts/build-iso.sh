#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [[ "$(id -u)" -ne 0 ]]; then
  echo "Run this script as root: sudo ./scripts/build-iso.sh" >&2
  exit 1
fi

for tool in cargo lb; do
  command -v "$tool" >/dev/null 2>&1 || {
    echo "Missing required tool: $tool" >&2
    exit 1
  }
done

echo "[1/4] Building TriadOS Rust system manager"
cargo build --release
install -D -m 0755 target/release/triad config/includes.chroot/usr/bin/triad

echo "[2/4] Configuring Debian Trixie live-build"
lb config

echo "[3/4] Building hybrid ISO (this may take a while)"
lb build

ISO="$ROOT/live-image-amd64.hybrid.iso"
if [[ ! -s "$ISO" ]]; then
  echo "Build finished but expected ISO was not found: $ISO" >&2
  exit 1
fi

echo "[4/4] ISO created"
sha256sum "$ISO"
echo "Output: $ISO"

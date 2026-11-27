#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [[ "$(id -u)" -ne 0 ]]; then
  echo "Run this script as root: sudo ./scripts/build-iso.sh" >&2
  exit 1
fi

command -v lb >/dev/null 2>&1 || {
  echo "Missing required tool: lb" >&2
  exit 1
}

echo "[1/4] Checking TriadOS Rust system manager"
BINARY="$ROOT/target/release/triad"
if [[ ! -x "$BINARY" ]]; then
  if ! command -v cargo >/dev/null 2>&1; then
    echo "Missing target/release/triad and cargo; build the Rust binary before running as root." >&2
    exit 1
  fi
  cargo build --release
fi
install -D -m 0755 "$BINARY" config/includes.chroot/usr/bin/triad

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

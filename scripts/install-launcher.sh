#!/usr/bin/env bash
set -Eeuo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
install -Dm755 "$ROOT/launcher/triad_launcher.py" /usr/local/bin/triad-launch
install -Dm644 "$ROOT/config/includes.chroot/usr/share/applications/triad-launcher.desktop" /usr/share/applications/triad-launcher.desktop

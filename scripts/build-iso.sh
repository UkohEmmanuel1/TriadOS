#!/usr/bin/env bash
set -Eeuo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
if [[ "$EUID" -ne 0 ]]; then echo 'Run with sudo/root.' >&2; exit 1; fi
command -v lb >/dev/null || { echo 'Install live-build first.' >&2; exit 1; }
if [[ -e .build/config || -f config/binary ]]; then
  echo 'Existing live-build configuration detected. Review and clean manually if necessary.' >&2
fi
lb config --mode debian --distribution trixie --architectures amd64 \
  --binary-images iso-hybrid --archive-areas 'main contrib non-free-firmware' \
  --debian-installer none --bootappend-live 'boot=live components quiet splash' \
  --iso-application 'TriadOS Prototype Live' --iso-publisher 'TriadOS community prototype'
lb build
printf '\nBuild completed; inspect the generated ISO and test it in QEMU.\n'

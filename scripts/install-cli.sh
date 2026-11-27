#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
cargo build --release
install -Dm755 target/release/triad /usr/local/bin/triad
echo "Installed. Run triad info"

#!/usr/bin/env python3
"""TriadOS prototype cross-platform file launcher (not a security sandbox)."""
import argparse
import os
from pathlib import Path
import shutil
import subprocess
import sys


def detect(path: Path) -> str:
    """Classify formats, not trustworthiness or actual runtime compatibility."""
    if path.is_dir() and path.name.lower().endswith('.app'):
        return 'mac-app'
    if path.is_file():
        suffix = path.suffix.lower()
        if suffix in ('.exe', '.msi'):
            return 'windows'
        if suffix == '.dmg':
            return 'mac-dmg'
        if suffix in ('.appimage', '.deb', '.rpm'):
            return 'linux-package'
        if os.access(path, os.X_OK):
            return 'linux-executable'
    return 'unknown'


def build_command(path: Path, kind: str):
    """Construct a command only for supported launchable formats."""
    if kind == 'windows':
        if not shutil.which('wine'):
            raise RuntimeError('Wine is not installed. Install wine first.')
        if path.suffix.lower() == '.msi':
            return ['wine', 'msiexec', '/i', str(path)]
        return ['wine', str(path)]
    if kind == 'linux-executable':
        return [str(path)]
    if kind == 'linux-package' and path.suffix.lower() == '.appimage':
        if not os.access(path, os.X_OK):
            raise RuntimeError('AppImage is not executable. Review it before changing permissions.')
        return [str(path)]
    raise RuntimeError(f'{kind}: no safe direct launcher implemented. Use the appropriate package manager or supported platform.')


def pick_file():
    if not shutil.which('zenity'):
        raise RuntimeError('Zenity is not installed; pass a path on the command line.')
    res = subprocess.run(['zenity', '--file-selection', '--title=Select an app to inspect'],
                         capture_output=True, text=True, check=False)
    return Path(res.stdout.strip()) if res.returncode == 0 and res.stdout.strip() else None


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('path', nargs='?', type=Path)
    parser.add_argument('--inspect', action='store_true', help='Only classify; do not run')
    parser.add_argument('--pick', action='store_true', help='Select file with a graphical picker')
    parser.add_argument('--yes', action='store_true', help='Skip execution confirmation')
    args = parser.parse_args(argv)
    try:
        target = pick_file() if args.pick else args.path
        if target is None:
            if args.pick:
                return 0
            parser.error('Supply a file path or --pick')
        target = target.expanduser().resolve(strict=True)
        kind = detect(target)
        print(f'Format: {kind}\nPath: {target}')
        if kind.startswith('mac-'):
            print('macOS apps are not supported by this launcher. Darling currently has limited compatibility, especially for GUI apps.')
            return 0 if args.inspect else 2
        if args.inspect:
            return 0
        command = build_command(target, kind)
        print('Command:', ' '.join(command))
        if not args.yes and input('Run this application? [y/N] ').strip().lower() != 'y':
            return 0
        return subprocess.run(command, check=False).returncode
    except (OSError, RuntimeError) as exc:
        print(f'Error: {exc}', file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())

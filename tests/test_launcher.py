import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest import mock

MODULE = Path(__file__).resolve().parents[1] / 'launcher' / 'triad_launcher.py'
spec = importlib.util.spec_from_file_location('triad_launcher', MODULE)
launcher = importlib.util.module_from_spec(spec)
spec.loader.exec_module(launcher)


class LauncherTests(unittest.TestCase):
    def test_format_detection(self):
        with tempfile.TemporaryDirectory() as temp:
            base = Path(temp)
            for name, expected in [('setup.EXE', 'windows'), ('setup.msi', 'windows'),
                                   ('download.dmg', 'mac-dmg'), ('program.AppImage', 'linux-package'),
                                   ('other.txt', 'unknown')]:
                file = base / name
                file.touch()
                self.assertEqual(launcher.detect(file), expected)
            bundle = base / 'Example.app'
            bundle.mkdir()
            self.assertEqual(launcher.detect(bundle), 'mac-app')

    def test_msi_dispatch(self):
        with mock.patch.object(launcher.shutil, 'which', return_value='/usr/bin/wine'):
            command = launcher.build_command(Path('/tmp/setup.msi'), 'windows')
            self.assertEqual(command, ['wine', 'msiexec', '/i', '/tmp/setup.msi'])

    def test_no_fictitious_mac_support(self):
        with self.assertRaises(RuntimeError):
            launcher.build_command(Path('/tmp/App.app'), 'mac-app')


if __name__ == '__main__':
    unittest.main()

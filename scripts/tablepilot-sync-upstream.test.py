"""Integration checks against a temporary Git repository; no network or credentials."""
import importlib.util
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

spec = importlib.util.spec_from_file_location('sync', Path(__file__).with_name('tablepilot-sync-upstream.py'))
sync = importlib.util.module_from_spec(spec)
spec.loader.exec_module(sync)


class ProposalTest(unittest.TestCase):
    def test_applies_only_unchanged_regular_files_and_preserves_fork(self):
        original = Path.cwd()
        with tempfile.TemporaryDirectory() as tmp:
            os.chdir(tmp)
            try:
                sync.git('init', '-q')
                sync.git('config', 'user.name', 'Test')
                sync.git('config', 'user.email', 'test@example.com')
                files = ['src/query.ts', 'src/edited.ts', 'src/protected.ts', 'src/ai/runner.ts', 'src/removed.ts', '.github/workflows/upstream.yml']
                for path in files:
                    p = Path(path)
                    p.parent.mkdir(parents=True, exist_ok=True)
                    p.write_text('base\n')
                sync.git('add', '.')
                sync.git('commit', '-qm', 'base')
                base = sync.git('rev-parse', 'HEAD').stdout.strip()
                for path in files:
                    Path(path).write_text('upstream\n')
                Path('src/removed.ts').unlink()
                Path('src/added.ts').write_text('new file\n')
                Path('src/link.ts').symlink_to('query.ts')
                sync.git('add', '.')
                sync.git('commit', '-qm', 'upstream')
                latest = sync.git('rev-parse', 'HEAD').stdout.strip()
                sync.git('checkout', '-qb', 'fork', base)
                Path('src/edited.ts').write_text('TablePilot custom change\n')
                sync.git('add', '.')
                sync.git('commit', '-qm', 'customization')
                plan, review = sync.proposal_plan(base, latest, {'src/protected.ts'})
                self.assertEqual({p for p, _ in plan}, {'src/query.ts', 'src/removed.ts', 'src/added.ts'})
                self.assertEqual({p for p, _ in review}, {'src/edited.ts', 'src/protected.ts', 'src/ai/runner.ts', '.github/workflows/upstream.yml', 'src/link.ts'})
                sync.apply_plan(plan)
                self.assertEqual(Path('src/query.ts').read_text(), 'upstream\n')
                self.assertEqual(Path('src/added.ts').read_text(), 'new file\n')
                self.assertFalse(Path('src/removed.ts').exists())
                self.assertEqual(Path('src/edited.ts').read_text(), 'TablePilot custom change\n')
                self.assertEqual(Path('src/protected.ts').read_text(), 'base\n')
                self.assertEqual(Path('src/ai/runner.ts').read_text(), 'base\n')
                self.assertEqual(Path('.github/workflows/upstream.yml').read_text(), 'base\n')
                self.assertFalse(Path('src/link.ts').exists())
                sync.git('commit', '-qm', 'candidate')
                second_plan, _ = sync.proposal_plan(base, latest, {'src/protected.ts'})
                self.assertEqual(second_plan, [])
            finally:
                os.chdir(original)

    def test_protection_policy(self):
        for path in ['.git/config', '../outside', '/tmp/outside', 'package.json', 'src-tauri/src/main.rs', 'scripts/run.sh', 'crates/new/Cargo.toml', 'src/mcp/server.ts']:
            self.assertTrue(sync.protected(path, set()), path)
        self.assertFalse(sync.protected('apps/desktop/src/lib/sql/formatter.ts', set()))


if __name__ == '__main__':
    unittest.main()

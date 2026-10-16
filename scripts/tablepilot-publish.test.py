import importlib.util
import json
from pathlib import Path
from unittest import TestCase, main
from unittest.mock import patch
import base64

spec = importlib.util.spec_from_file_location('publish', Path(__file__).with_name('tablepilot-publish.py'))
publish = importlib.util.module_from_spec(spec)
spec.loader.exec_module(publish)


class ReleaseTargetTest(TestCase):
    def test_identical_workflows_use_exact_source_commit(self):
        def api(repo, path, payload=None):
            if path == 'commits/main': return {'sha': 'current'}
            if path in ['git/trees/source', 'git/trees/current']:
                return {'tree': [{'path': '.github', 'sha': 'github'}]}
            return {'tree': [{'path': 'workflows', 'sha': 'approved'}]}
        with patch.object(publish, 'api', side_effect=api):
            self.assertEqual(publish.release_target('owner/repo', 'source'), 'source')

    def test_snapshot_changes_only_workflows_and_their_manifest_entries(self):
        writes = []
        def api(repo, path, payload=None):
            if payload is not None:
                writes.append((path, payload))
                return {'sha': 'snapshot-tree' if path == 'git/trees' else 'snapshot-commit'}
            if path == 'commits/main': return {'sha': 'current'}
            if path in ['git/trees/source', 'git/trees/current']:
                return {'tree': [{'path': '.github', 'sha': path.split('/')[-1] + '-github'}]}
            if path.endswith('-github'):
                return {'tree': [{'path': 'workflows', 'sha': path.split('/')[-1] + '-workflows'}]}
            if path.startswith('contents/'):
                hashes = {'app.ts': 'original-app', '.github/workflows/old.yml': 'old'} if 'ref=source' in path else {'app.ts': 'new-app', '.github/workflows/new.yml': 'approved'}
                return {'content': base64.b64encode(json.dumps(hashes).encode()).decode()}
            if path == 'git/commits/source': return {'tree': {'sha': 'original-tree'}}
            raise AssertionError(path)
        with patch.object(publish, 'api', side_effect=api):
            self.assertEqual(publish.release_target('owner/repo', 'source'), 'snapshot-commit')
        tree = writes[0][1]
        self.assertEqual(tree['base_tree'], 'original-tree')
        self.assertEqual({e['path'] for e in tree['tree']}, {'.github/workflows', 'SOURCE_SHA256.json'})
        manifest = json.loads(tree['tree'][1]['content'])
        self.assertEqual(manifest, {'app.ts': 'original-app', '.github/workflows/new.yml': 'approved'})
        self.assertEqual(writes[1][1]['parents'], ['source'])


if __name__ == '__main__': main()

"""Update approved release descriptions without changing assets, tags or release status."""
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess

REPOSITORY = 'deckeeper/TablePilot'


def api(endpoint, payload=None):
    args = ['gh', 'api', f'repos/{REPOSITORY}/{endpoint}']
    if payload is not None:
        args += ['--method', 'PATCH', '--input', '-']
    result = subprocess.run(args, input=json.dumps(payload) if payload is not None else None,
                            text=True, capture_output=True, check=True)
    return json.loads(result.stdout)


def update_descriptions(items):
    pending = []
    seen = set()
    # Preflight every description before making any writes.
    for item in items:
        tag = item['tag']
        if not re.fullmatch(r'v0\.3\.[0-3]', tag) or tag in seen:
            raise ValueError('Unexpected or duplicate release tag')
        seen.add(tag)
        current = api(f'releases/tags/{tag}')
        if current['tag_name'] != tag:
            raise ValueError('Release tag mismatch')
        body = current.get('body') or ''
        if body == item['body']:
            print(f'{tag}: already updated')
            continue
        if hashlib.sha256(body.encode()).hexdigest() != item['expected_body_sha256']:
            raise ValueError(f'{tag}: description changed; review before retrying')
        pending.append((current['id'], tag, item['body']))
    for release_id, tag, body in pending:
        updated = api(f'releases/{release_id}', {'body': body})
        if updated.get('body') != body or updated.get('tag_name') != tag:
            raise RuntimeError(f'{tag}: update could not be verified')
        print(f'{tag}: description updated; installers and release status preserved')


if __name__ == '__main__':
    if os.environ.get('GITHUB_REPOSITORY') != REPOSITORY:
        raise SystemExit('This maintenance script is restricted to deckeeper/TablePilot')
    update_descriptions(json.loads(Path('.tablepilot/release-presentation.json').read_text()))

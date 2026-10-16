"""Publish validated TablePilot installers to this repository's GitHub Releases."""
import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import subprocess


def gh(*args, check=True):
    result = subprocess.run(['gh', *args], text=True, capture_output=True)
    if check and result.returncode:
        raise SystemExit(result.stderr.strip() or result.stdout.strip() or 'GitHub CLI request failed')
    return result


def api(repo, endpoint, payload=None):
    command = ['gh', 'api', f'repos/{repo}/{endpoint}']
    if payload is not None:
        command += ['--method', 'POST', '--input', '-']
    result = subprocess.run(command, input=json.dumps(payload) if payload is not None else None,
                            text=True, capture_output=True)
    if result.returncode:
        raise SystemExit(result.stderr.strip())
    return json.loads(result.stdout)


def release_target(repo, source):
    # GitHub Actions cannot release older workflow definitions. Retain the exact
    # app tree but use the currently approved default-branch workflows in the tag.
    current = api(repo, 'commits/main')['sha']
    def workflow_tree(commit):
        root = api(repo, f'git/trees/{commit}')
        github = next(e for e in root['tree'] if e['path'] == '.github')
        entries = api(repo, f"git/trees/{github['sha']}")['tree']
        return next(e for e in entries if e['path'] == 'workflows')['sha']
    if workflow_tree(source) == workflow_tree(current):
        return source
    workflows = workflow_tree(current)
    def file_content(commit, path):
        blob = api(repo, f'contents/{path}?ref={commit}')
        return base64.b64decode(blob['content']).decode()
    manifest = json.loads(file_content(source, 'SOURCE_SHA256.json'))
    current_manifest = json.loads(file_content(current, 'SOURCE_SHA256.json'))
    manifest = {p: h for p, h in manifest.items() if not p.startswith('.github/workflows/')}
    manifest.update({p: h for p, h in current_manifest.items() if p.startswith('.github/workflows/')})
    source_commit = api(repo, f'git/commits/{source}')
    tree = api(repo, 'git/trees', {
        'base_tree': source_commit['tree']['sha'],
        'tree': [
            {'path': '.github/workflows', 'mode': '040000', 'type': 'tree', 'sha': workflows},
            {'path': 'SOURCE_SHA256.json', 'mode': '100644', 'type': 'blob',
             'content': json.dumps(manifest, indent=2, ensure_ascii=False) + '\n'},
        ],
    })
    commit = api(repo, 'git/commits', {
        'message': f'Release packaging for {source}: retain app source, refresh approved workflows',
        'tree': tree['sha'], 'parents': [source],
    })
    return commit['sha']


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--artifacts', required=True)
    parser.add_argument('--version', required=True)
    parser.add_argument('--target', required=True)
    parser.add_argument('--existing-prototype', action='store_true')
    args = parser.parse_args()
    repo = os.environ['GITHUB_REPOSITORY']
    tag = f'v{args.version}'
    root = Path(args.artifacts)
    expected = {
        f'TablePilot_{args.version}_x64-setup.exe': None,
        f'TablePilot_{args.version}_amd64.deb': None,
    }
    if args.existing_prototype:
        if args.version != '0.3.0':
            raise SystemExit('Recorded prototype hashes are only for 0.3.0')
        expected = {
            'TablePilot_0.3.0_x64-setup.exe': '25bf6da3a545943d5aae4fe4024d79c8e682a3ae2da55021fe4db8f74ee67052',
            'TablePilot_0.3.0_amd64.deb': '7f49b47f7397b1f72f401eaac472080d13d15550205aeeb9a5e8ca20d4215ea9',
        }
    assets = []
    checksums = []
    for name, known_hash in expected.items():
        matches = list(root.rglob(name))
        if len(matches) != 1:
            raise SystemExit(f'Expected exactly one installer: {name}, found {len(matches)}')
        asset = matches[0]
        if asset.stat().st_size < 1_000_000:
            raise SystemExit(f'Unexpectedly small installer: {name}')
        digest = hashlib.sha256(asset.read_bytes()).hexdigest()
        if known_hash and digest != known_hash:
            raise SystemExit(f'Installer checksum mismatch: {name}')
        checksums.append(f'{digest}  {name}\n')
        assets.append(asset)
    checksum_file = root/'SHA256SUMS.txt'
    checksum_file.write_text(''.join(checksums))
    assets.append(checksum_file)
    # Reruns are allowed only when every published byte matches; never overwrite a release.
    existing = gh('release', 'view', tag, '--repo', repo, '--json', 'assets,isDraft', check=False)
    if existing.returncode == 0:
        release = json.loads(existing.stdout)
        published = {a['name']: a for a in release['assets']}
        for asset in assets:
            info = published.get(asset.name)
            if not info:
                raise SystemExit(f'Release {tag} exists but is incomplete; inspect it before retrying')
            import tempfile
            with tempfile.TemporaryDirectory() as tmp:
                gh('release', 'download', tag, '--repo', repo, '--pattern', asset.name, '--dir', tmp)
                if hashlib.sha256((Path(tmp)/asset.name).read_bytes()).digest() != hashlib.sha256(asset.read_bytes()).digest():
                    raise SystemExit(f'Release {tag} has different assets. Bump the application version.')
        if release['isDraft']:
            raise SystemExit(f'Release {tag} is a draft; inspect it before publishing')
        print(f'Release {tag} already contains these exact installers.')
        return
    target = release_target(repo, args.target)
    notes = root/'release-notes.md'
    text = f'''TablePilot {args.version} desktop prototype, based on DBX with AI and MCP execution removed.

## Downloads
- Windows 11 x64: `TablePilot_{args.version}_x64-setup.exe`
- Ubuntu 24.04 / compatible Debian x64: `TablePilot_{args.version}_amd64.deb`
- `SHA256SUMS.txt`: installer checksums.

Installers are unsigned. Native installer builds passed; installation and live database/SSH tests still need user testing.
This repository is private, so downloads require access to it.

Source commit: `{args.target}`.
'''
    if args.existing_prototype:
        text += '\nWindows source: `8b0a975c5d5bc261bf170012f2d59db95bd7d031`; Linux source: `adfb658b14e90cbeb275216dd8c0c520c54910e8` (Linux packaging dependency fix). These are the existing 0.3.0 installers; GUI link removal arrives in 0.3.1.\n'
    else:
        changes = {
            '0.3.1': 'Remove GitHub navigation from the GUI; add reviewed upstream update proposals and installer releases.',
            '0.3.2': 'Integrate reviewed DBX 0.6.45 database and interface improvements, excluding AI/MCP changes.',
            '0.3.3': 'Remove Plugin Center navigation and marketplace updates while preserving Driver Manager/JDBC. Use faster prototype build settings and expanded compilation caching.',
        }
        text += '\nChanges: ' + changes.get(args.version, 'See FORK_CHANGES.md and VALIDATION.md in the tagged source for this version.') + '\n'
    if target != args.target:
        text += f'\nThe release tag `{target}` retains the application source above; only workflow definitions and their source-manifest hashes were refreshed to the current approved workflows.\n'
    notes.write_text(text)
    result = gh('release', 'create', tag, *map(str, assets), '--repo', repo,
                '--target', target, '--title', f'TablePilot {args.version} prototype',
                '--prerelease', '--notes-file', str(notes))
    print(result.stdout)


if __name__ == '__main__':
    main()

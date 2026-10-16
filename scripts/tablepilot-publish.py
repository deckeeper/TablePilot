"""Publish validated TablePilot installers to this repository's GitHub Releases."""
import argparse
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
        text += '\nChanges: remove GitHub navigation from the GUI; add reviewed upstream update proposals and installer releases.\n'
    notes.write_text(text)
    result = gh('release', 'create', tag, *map(str, assets), '--repo', repo,
                '--target', args.target, '--title', f'TablePilot {args.version} prototype',
                '--prerelease', '--notes-file', str(notes))
    print(result.stdout)


if __name__ == '__main__':
    main()

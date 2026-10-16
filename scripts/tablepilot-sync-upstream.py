"""Prepare conservative upstream proposals. Never merge or execute upstream code."""
import json
import os
from pathlib import Path
import re
import subprocess


def run(*args, check=True, binary=False):
    return subprocess.run(args, check=check, capture_output=True, text=not binary)


def git(*args, **kwargs):
    return run('git', '--literal-pathspecs', *args, **kwargs)


def fork_paths():
    # Both the modification notice and omitted-file list must remain protected.
    return set(re.findall(r'^- `([^`]+)`', Path('FORK_CHANGES.md').read_text(), re.M))


def protected(path, overrides):
    parts = Path(path).parts
    if not parts or Path(path).is_absolute() or '..' in parts:
        return True
    if path in overrides or any(p.startswith('.') for p in parts):
        return True
    if path.startswith(('scripts/', 'src-tauri/', 'packages/', 'docs/', 'website/')):
        return True
    if len(parts) == 1 or Path(path).name in {'Cargo.toml', 'Cargo.lock', 'package.json', 'pnpm-lock.yaml'}:
        return True
    # New model/assistant code is held for manual review as well as known omitted files.
    return bool(re.search(r'ai|mcp|assistant|copilot|llm|agent|updater|telemetry', path, re.I))


def entry(revision, path):
    result = git('ls-tree', '-z', revision, '--', path).stdout
    if not result:
        return None
    metadata, name = result.rstrip('\0').split('\t', 1)
    mode, kind, sha = metadata.split()
    return mode, kind, sha


def proposal_plan(base, latest, overrides):
    changed = git('diff', '--name-only', '--no-renames', '-z', base, latest).stdout.split('\0')
    apply, review = [], []
    for path in filter(None, changed):
        if protected(path, overrides):
            review.append((path, 'protected fork, build, dependency or AI-related path'))
            continue
        old, new, current = entry(base, path), entry(latest, path), entry('HEAD', path)
        if current == new:
            continue
        if current != old:
            review.append((path, 'TablePilot differs from the upstream baseline'))
            continue
        if any(e and (e[0] not in {'100644', '100755'} or e[1] != 'blob') for e in (old, new, current)):
            review.append((path, 'non-regular file requires manual review'))
            continue
        target = Path(path)
        if any(p.is_symlink() or (p.exists() and not p.is_dir()) for p in target.parents):
            review.append((path, 'parent path requires manual review'))
            continue
        if target.is_dir() or target.is_symlink():
            review.append((path, 'working-tree path requires manual review'))
            continue
        apply.append((path, new))
    return apply, review


def apply_plan(plan):
    for path, new in plan:
        target = Path(path)
        if new is None:
            target.unlink()
        else:
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(git('cat-file', 'blob', new[2], binary=True).stdout)
            target.chmod(0o755 if new[0] == '100755' else 0o644)
        git('add', '--', path)


def summary(message):
    print(message)
    if os.environ.get('GITHUB_STEP_SUMMARY'):
        with open(os.environ['GITHUB_STEP_SUMMARY'], 'a') as f:
            f.write(message + '\n')


def main():
    config = json.loads(Path('.tablepilot/upstream.json').read_text())
    if config['repository'] != 't8y2/dbx':
        raise SystemExit('Unexpected upstream repository')
    repo = os.environ['GITHUB_REPOSITORY']
    if git('status', '--porcelain').stdout.strip():
        raise SystemExit('Upstream proposal requires a clean checkout')
    release = json.loads(run('gh', 'api', 'repos/t8y2/dbx/releases/latest').stdout)
    tag = release['tag_name']
    if not re.fullmatch(r'v\d+\.\d+\.\d+', tag):
        raise SystemExit('Only stable semantic-version tags are supported')
    version = lambda value: tuple(map(int, value.lstrip('v').split('.')))
    if version(tag) <= version(config['baseline_tag']):
        summary('TablePilot has no newer stable upstream release to review.')
        return
    branch = f'upstream/tablepilot-{tag}'
    if git('ls-remote', '--heads', 'origin', f'refs/heads/{branch}').stdout.strip():
        summary(f'Proposal already exists: https://github.com/{repo}/compare/main...{branch}. Existing review work was preserved.')
        return
    base = config['baseline_commit']
    if not re.fullmatch(r'[0-9a-f]{40}', base):
        raise SystemExit('Invalid upstream baseline commit')
    upstream = 'https://github.com/t8y2/dbx.git'
    git('fetch', '--no-tags', '--depth=1', upstream, base)
    git('fetch', '--no-tags', '--depth=1', upstream, f'refs/tags/{tag}')
    latest = git('rev-parse', 'FETCH_HEAD^{commit}').stdout.strip()
    plan, review = proposal_plan(base, latest, fork_paths())
    git('switch', '-c', branch)
    apply_plan(plan)
    report = Path(f'.tablepilot/proposals/{tag}.md')
    report.parent.mkdir(parents=True, exist_ok=True)
    body = f'''# Review upstream {tag} for TablePilot

Upstream: https://github.com/t8y2/dbx/releases/tag/{tag}
Compare: https://github.com/t8y2/dbx/compare/{config['baseline_commit']}...{latest}
Pinned target commit: `{latest}`.

This is a **draft**, never an automatic merge. Only regular files unchanged from the upstream baseline are copied. TablePilot customizations, omitted files, AI-related paths, dependencies, build scripts and workflows are held for manual review. This conservative filter is not proof that copied code cannot call AI; review all proposed code and test before merging.

The baseline remains `{config['baseline_tag']}`. After reviewing every skipped change, update `.tablepilot/upstream.json` to `{tag}` / `{latest}`. Regenerate `SOURCE_SHA256.json`, bump TablePilot's version, run frontend/native checks and test imports, exports and SSH before merging. Nothing from this branch has been executed by the sync workflow.

## Applied upstream files ({len(plan)})
'''
    body += ''.join(f'- `{path}`\n' for path, _ in plan) or '- None.\n'
    body += f'\n## Manual review required ({len(review)})\n'
    body += ''.join(f'- `{path}` — {reason}\n' for path, reason in review) or '- None.\n'
    report.write_text(body)
    git('add', '--', str(report))
    git('config', 'user.name', 'TablePilot Upstream Bot')
    git('config', 'user.email', '41898282+github-actions[bot]@users.noreply.github.com')
    git('commit', '-m', f'Propose upstream {tag} updates for manual TablePilot review')
    git('push', 'origin', f'HEAD:refs/heads/{branch}')
    result = run('gh', 'pr', 'create', '--repo', repo, '--base', 'main', '--head', branch,
                 '--draft', '--title', f'Review upstream {tag} updates for TablePilot', '--body-file', str(report), check=False)
    if result.returncode:
        summary(f'Candidate branch created: https://github.com/{repo}/compare/main...{branch}\n\nDraft PR creation was blocked: {result.stderr.strip()}\n\nIf GitHub reports Actions PR creation is disabled, enable Settings → Actions → General → Allow GitHub Actions to create and approve pull requests, or open the PR from the compare link.')
    else:
        summary(f'Draft upstream update prepared: {result.stdout.strip()}')


if __name__ == '__main__':
    main()

"""Regenerate hashes for tracked and new, non-ignored source files."""
import hashlib
import json
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parents[1]
paths = subprocess.check_output(['git', 'ls-files', '--cached', '--others', '--exclude-standard', '-z'], cwd=root).decode().split('\0')
manifest = {}
for name in sorted(set(filter(None, paths))):
    if name == 'SOURCE_SHA256.json':
        continue
    path = root/name
    if path.is_file():
        manifest[name] = hashlib.sha256(path.read_bytes()).hexdigest()
(root/'SOURCE_SHA256.json').write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + '\n')
print(f'Hashed {len(manifest)} source files.')

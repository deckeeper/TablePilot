"""One-time, checksum-verified import of the TablePilot source prototype."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import tempfile

UPSTREAM = "0db01f7f284ec6b0b090fe7369de7d05c37b25b0"
BUNDLE_SHA256 = "b3d2cf594ce2cea4f3a2e7e5d2a6305f654caa07ae5fc2e76a53598784271829"
SOURCE_SHA256 = "fca4d693860a00d75e405c26ffc0f4cc44cdebba32feac44bcf3471375830568"
root = Path.cwd()
bundle = root / ".tablepilot-bootstrap/changes.tar.gz"
assert hashlib.sha256(bundle.read_bytes()).hexdigest() == BUNDLE_SHA256, "Patch bundle checksum mismatch"
workflow = (root / ".github/workflows/tablepilot-build.yml").read_bytes()
with tempfile.TemporaryDirectory() as temp:
    temp = Path(temp)
    archive = temp / "upstream.tar.gz"
    if os.environ.get("TABLEPILOT_UPSTREAM_ARCHIVE"):
        shutil.copyfile(os.environ["TABLEPILOT_UPSTREAM_ARCHIVE"], archive)
    else:
        subprocess.run(["curl", "--fail", "--location", "--retry", "3", "--output", str(archive),
                        f"https://codeload.github.com/t8y2/dbx/tar.gz/{UPSTREAM}"], check=True)
    upstream = temp / "upstream"
    upstream.mkdir()
    with tarfile.open(archive) as tar:
        tar.extractall(upstream, filter="data")
    source = next(upstream.iterdir())
    shutil.copytree(source, root, dirs_exist_ok=True)
    shutil.copyfile(root / "README.md", root / "README-UPSTREAM.md")
    patch_dir = temp / "patch"
    with tarfile.open(bundle) as tar:
        tar.extractall(patch_dir, filter="data")
    subprocess.run(["git", "apply", "--check", str(patch_dir / "changes.patch")], check=True)
    subprocess.run(["git", "apply", str(patch_dir / "changes.patch")], check=True)
    for name in json.loads((patch_dir / "deleted.json").read_text()):
        path = root / name
        if path.exists():
            path.unlink()
    shutil.copytree(patch_dir / "overlay", root, dirs_exist_ok=True)

def source_hashes():
    excluded = {"SOURCE_SHA256.json", ".github/workflows/tablepilot-build.yml"}
    return {p.relative_to(root).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
            for p in sorted(root.rglob("*"))
            if p.is_file() and p.relative_to(root).parts[0] not in {".git", ".tablepilot-bootstrap"}
            and p.relative_to(root).as_posix() not in excluded}

hashes = source_hashes()
actual = hashlib.sha256(json.dumps(hashes, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
assert actual == SOURCE_SHA256, f"Source checksum mismatch: {actual}"
(root / ".github/workflows/tablepilot-build.yml").write_bytes(workflow)
hashes[".github/workflows/tablepilot-build.yml"] = hashlib.sha256(workflow).hexdigest()
(root / "SOURCE_SHA256.json").write_text(json.dumps(hashes, indent=2, sort_keys=True) + "\n")
shutil.rmtree(root / ".tablepilot-bootstrap")
print(f"Verified and imported {len(hashes)} TablePilot files from pinned upstream {UPSTREAM}.")

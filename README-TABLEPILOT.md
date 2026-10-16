# TablePilot 0.3.2 prototype

An independent desktop fork of [DBX v0.6.45](https://github.com/t8y2/dbx/tree/v0.6.45), commit `13b6acafed41e1ce65d5e97344e7e93cb88202c2`, licensed under Apache-2.0.

The database workbench, SQL editor, CSV/Excel import/export, field mapping, SSH tunnelling, database drivers, themes and ordinary plugin tools are retained. Database driver processes called “agents” are database components and are retained.

The AI provider runtime and MCP services are excluded from the desktop build. Their native commands and startup servers are removed. AI chat/settings/menu controls are removed, plugin `host.ai` access is denied, and the shared frontend API rejects AI/MCP calls. Some passive legacy configuration types remain to keep the shared database/settings format readable; they cannot execute models or start an MCP server.

TablePilot has its own app identifier, credential-store service and portable-data marker. DBX's app updater is removed. Database driver downloads still use the upstream driver distribution service. This is not an entirely offline product, and third-party plugins retain their own declared network permissions.

## Changes in 0.3.2

Reviewed database and interface changes from DBX 0.6.45: grid filters and pagination, Oracle constraints and object rename previews, table structure editing, saved default queries, Save All Tabs, Redis value previews, SQL-file retry fixes, Thai translations, and database-driver/plugin cleanup. AI conversation changes, AI SQL execution feedback, AI provider updates and MCP discovery are excluded. TablePilot branding, hidden GitHub GUI links and its own installer/release workflows are preserved.

## Download installers

Open [TablePilot Releases](https://github.com/deckeeper/TablePilot/releases), expand **Assets**, and download the Windows `.exe` or Linux `.deb`. The repository remains private: you must be signed in with repository access. Releases retain downloads beyond the 30-day Actions artifact window. The original 0.3.0 installers are also published from their verified build artifacts; the GitHub GUI link cleanup is included in 0.3.1.

## Review upstream updates

**Review DBX upstream updates** checks the latest stable DBX release daily at 06:23 UTC (GitHub may delay scheduled runs), or from **Actions → Run workflow**. It prepares an `upstream/dbx-v…` branch and attempts to open a draft pull request. It never merges into `main`.

The proposal copies only regular files that still match the pinned DBX baseline. The fork modification/omission list, AI-related paths, dependencies, build scripts and workflows are protected. Its report lists everything requiring manual review. Filters do not guarantee compatibility or the absence of newly introduced AI calls: review the diff and test before merging. The DBX baseline is not advanced automatically. After reviewing skipped changes, update `.tablepilot/upstream.json`, run `python scripts/tablepilot-source-manifest.py` to refresh `SOURCE_SHA256.json`, bump TablePilot's version, and validate the candidate.

If GitHub blocks the bot from opening pull requests, the proposal branch is still created and its compare link appears in the workflow summary. To allow future automatic draft PR creation, enable **Settings → Actions → General → Allow GitHub Actions to create and approve pull requests**. Existing proposal branches are never overwritten.

## Build an installer

Use Node.js 24, pnpm 11.25.0 and stable Rust (at least 1.88). A development machine needs more memory than the installed application; allow at least 8 GB for the frontend type checker.

On Windows 11, install Visual Studio Build Tools with “Desktop development with C++”, the Windows SDK, and WebView2 Runtime. Use a Developer PowerShell:

```powershell
corepack enable
corepack prepare pnpm@11.25.0 --activate
pnpm install --frozen-lockfile
node scripts/build-tablepilot.mjs
```

The installer is written to `target/release/bundle/nsis/`.

On Ubuntu 24.04:

```sh
sudo apt-get install build-essential pkg-config libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libssl-dev patchelf cmake
corepack enable
corepack prepare pnpm@11.25.0 --activate
pnpm install --frozen-lockfile
node scripts/build-tablepilot.mjs
```

The Debian package is written to `target/release/bundle/deb/`.

The **Build TablePilot installers** GitHub Actions workflow builds Windows and Linux installers on pushes to `main` and on manual runs. Once both builds pass, the separate **Publish TablePilot installers** workflow publishes the version in `package.json` as a GitHub prerelease in this repository, with checksums. Bump the version in `package.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml` and the `dbx` entry in `Cargo.lock` before publishing different installers; existing release assets are never silently replaced. A `[linux-only]` build only uploads its workflow artifact. Installers are unsigned until you configure your own signing certificate. Release notes record the exact build source commit. If workflow definitions have since changed, the release tag retains that application source while refreshing only workflows and their manifest hashes to the approved `main` definitions, as required by GitHub’s release permissions.

Read `VALIDATION.md` for the checks actually completed on this prototype. Building source is not a substitute for testing an installer on Windows or testing against your database servers.

Original licensing is in `LICENSE`. Retain it, upstream copyright notices and third-party notices when sharing this fork. See `FORK_CHANGES.md` for modifications.

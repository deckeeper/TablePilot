# TablePilot 0.3.0 prototype

An independent desktop fork of [DBX v0.6.43](https://github.com/t8y2/dbx/tree/v0.6.43), commit `0db01f7f284ec6b0b090fe7369de7d05c37b25b0`, licensed under Apache-2.0.

The database workbench, SQL editor, CSV/Excel import/export, field mapping, SSH tunnelling, database drivers, themes and ordinary plugin tools are retained. Database driver processes called “agents” are database components and are retained.

The AI provider runtime and MCP services are excluded from the desktop build. Their native commands and startup servers are removed. AI chat/settings/menu controls are removed, plugin `host.ai` access is denied, and the shared frontend API rejects AI/MCP calls. Some passive legacy configuration types remain to keep the shared database/settings format readable; they cannot execute models or start an MCP server.

TablePilot has its own app identifier, credential-store service and portable-data marker. DBX's app updater is removed. Database driver downloads still use the upstream driver distribution service. This is not an entirely offline product, and third-party plugins retain their own declared network permissions.

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
sudo apt-get install build-essential pkg-config libwebkit2gtk-4.1-dev libgtk-3-dev librsvg2-dev libssl-dev patchelf cmake
corepack enable
corepack prepare pnpm@11.25.0 --activate
pnpm install --frozen-lockfile
node scripts/build-tablepilot.mjs
```

The Debian package is written to `target/release/bundle/deb/`.

Alternatively, the included **Build TablePilot installers** GitHub Actions workflow can be run manually in a repository you own. It builds installers as downloadable workflow artifacts; it does not publish releases or upload to DBX. Installers are unsigned until you configure your own signing certificate.

Read `VALIDATION.md` for the checks actually completed on this prototype. Building source is not a substitute for testing an installer on Windows or testing against your database servers.

Original licensing is in `LICENSE`. Retain it, upstream copyright notices and third-party notices when sharing this fork. See `FORK_CHANGES.md` for modifications.

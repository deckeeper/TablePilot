# Contributing to TablePilot

<!-- TablePilot modification: repository-specific documentation, 16 October 2026. -->

Report bugs and propose improvements in [this repository's issues](https://github.com/deckeeper/TablePilot/issues). Include the installed version, operating system, database type and safe reproduction steps. Follow [SECURITY.md](SECURITY.md) for security reports.

## Development

Use Node.js 24, pnpm 11.25.0 and stable Rust. Platform prerequisites and packaging commands are in [README.md](README.md#build-an-installer).

```sh
git clone https://github.com/deckeeper/TablePilot.git
cd TablePilot
pnpm install --frozen-lockfile
pnpm dev:tauri
```

## Changes and verification

Create a branch and open a pull request against `main`. Explain the problem, resulting behavior, validation and any limitations. Run focused tests for changed behavior, `pnpm typecheck`, and `pnpm build` where relevant. Installer CI checks the frontend before native compilation.

Keep AI and MCP features excluded. Preserve connection storage, import/export compatibility, SSH behavior, driver support and the application identity. Do not rename internal package or protocol identifiers solely for branding.

Retain applicable license and attribution notices and mark modified files. Record fork modifications in [FORK_CHANGES.md](FORK_CHANGES.md). Update the source manifest with `python scripts/tablepilot-source-manifest.py` before publishing source changes.

For a release, keep all application version declarations consistent and use a new version rather than replacing published installers. Upstream proposals require manual review and must not reintroduce excluded features.

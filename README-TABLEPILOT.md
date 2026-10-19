# TablePilot

<!-- TablePilot modification: repository-specific documentation, 16 October 2026. -->

A desktop database workbench for Windows and Linux. Connect to databases, write SQL, browse and edit tables, and move data between files and tables.

## Features

- Database connections with Driver Manager and JDBC support.
- SQL editor, query results, and table structure tools.
- CSV and Excel import/export with field mapping and multi-table selection.
- SSH tunnelling and jump-host connection options.
- Light and dark themes.
- Checks for official TablePilot releases at startup and every hour.

The desktop edition excludes AI and MCP features. Plugin Center is not available; Driver Manager remains available for database connectivity. Database driver downloads may use external distribution services.

## Download

Get the Windows `.exe` or Linux `.deb` from [TablePilot Releases](https://github.com/deckeeper/TablePilot/releases). Each release includes installer checksums. Installers are currently unsigned.

Run the newer installer to update an existing installation. The app's update check opens the release page for a manual download; it does not install updates unattended. Only complete, stable releases are offered.

## Recent changes

- **0.4.2:** Import several files or Excel worksheets into selected existing tables in one batch, with individual field mappings, destination review, and adjustable import order.

- **0.4.1:** Remove three vulnerable JavaScript development dependency chains while preserving the UI styles. The root JavaScript audit reports zero findings; other dependency ecosystems are not covered by that result.
- **0.4.0:** Official releases, refreshed About/settings branding, and update checks against this repository.
- **0.3.3:** Remove Plugin Center navigation and marketplace updates, and improve build caching.
- **0.3.2:** Improve grid filtering, pagination, database object editing, saved queries, and Redis previews.

See [release notes](https://github.com/deckeeper/TablePilot/releases) for each version and [validation notes](VALIDATION.md) for testing scope.

## Import and export several tables

In the sidebar, Ctrl-click individual tables or Shift-click a range, then right-click a selected table.

- **Export Data (N):** choose CSV, JSON, SQL INSERT or XLSX, then choose one output folder. Each selected table gets its own file. SQL database export also supports selecting multiple tables.
- **Import into N selected tables (0.4.2+):** select several files, or one Excel workbook with several worksheets. The wizard matches filenames or worksheet names to the selected tables in the active database/schema. Unmatched items need an explicit destination.

Click each import item to review its destination and column mapping. Uncheck items to skip them. On Review, check the complete file-to-table list and use Up/Down to put parent tables before dependent child tables. Click **Start Import** once for the selected batch. Files without a matching table are never assigned by position.

Append is the default; choosing Truncate applies to every selected destination. Parsing options apply across the batch. Duplicate targets are blocked, and metadata must load before a table can be imported. A failure or cancellation stops remaining items; rows already written are kept. This is not a transaction across all tables. Update-existing-row conflict handling remains available for single-table imports.

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

## Development and releases

See [CONTRIBUTING.md](CONTRIBUTING.md) for the development workflow and [SECURITY.md](SECURITY.md) for vulnerability reporting.

**Build TablePilot installers** builds Windows and Linux installers. After both pass, **Publish TablePilot installers** publishes a separate version with checksums. Stable versions from 0.4.0 onward are official releases. Existing release assets are never silently replaced.

Before publishing changed installers, update the frontend package version, native package version, installer configuration and application entry in the workspace lockfile together. Refresh `SOURCE_SHA256.json` using `python scripts/tablepilot-source-manifest.py`. Documentation-only commits marked `[automation-only]` skip installer compilation.

**Review upstream updates for TablePilot** checks for new stable dependency-project releases and prepares proposals for manual review. It never automatically merges changes. Application customizations, AI-related code, dependencies and build configuration require review. Source provenance and the pinned baseline are recorded in the maintenance configuration and legal notices.

## License

Distributed under [Apache License 2.0](LICENSE). See [NOTICE](NOTICE) for attribution and third-party notices, and [FORK_CHANGES.md](FORK_CHANGES.md) for modification records. Required notices are included with the installers.

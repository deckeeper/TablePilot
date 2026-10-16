# Prototype validation

Validated in a Linux development environment on 14 October 2026.

- Production frontend build: passed. Vite reports upstream chunk-size and dynamic-import warnings.
- Full Vue/TypeScript type check: **passed**, including the final code cleanup.
- Focused test suite: **65 tests passed across 8 files**. This covers import source handling, import into an existing table, batch selection, import-dialog theme classes, CSV null-export options, Excel header options, SSH authentication selection, and the TablePilot capability boundary.
- Dependency graph: the desktop graph contains no `dbx-ai-provider`, `dbx-mcp`, or `tauri-plugin-updater` dependency.
- Native compilation: **passed** (`cargo check -p dbx`, all default desktop features, Rust 1.99.0). This checks the Linux desktop command layer and database core; it does not produce or run an installer. The compiler reports 18 warnings in the desktop crate, principally unused compatibility helpers.

## Limits

Windows and Linux native installer builds for 0.3.0 subsequently passed in GitHub Actions (runs 38564008206, Windows job; and 38568811551, Linux). Installation, end-to-end GUI operation, live database connections, SSH jump-host connections, and round-trip CSV/Excel transfers have not been tested in this environment. The full upstream test suite was not run; tests that expect upstream AI/MCP behavior do not describe this edition.

The retained drivers and database operations are upstream code, not a guarantee that every supported database server/version works without configuration. Driver packages may be downloaded at runtime. No RAM or performance benchmark has been performed.

AI execution is removed from the native command surface and dependency graph. Passive legacy storage schemas remain for compatibility. Ordinary plugins can use their declared capabilities, so a third-party plugin could independently access an external service; this fork is not a general network sandbox.

## Reproduce selected tests

```sh
node node_modules/vitest/vitest.mjs run --maxWorkers=1 \
  apps/desktop/src/lib/__tests__/tablepilot.spec.ts \
  apps/desktop/src/components/import/__tests__/TableImportDialog.existingTarget.spec.ts \
  apps/desktop/src/components/import/__tests__/TableImportDialog.batchSelection.spec.ts \
  apps/desktop/src/lib/__tests__/import/importSource.spec.ts \
  apps/desktop/src/lib/export/__tests__/csvNullMode.spec.ts \
  apps/desktop/src/lib/export/__tests__/xlsxHeader.spec.ts \
  apps/desktop/src/lib/connection/__tests__/sshAuthMethod.spec.ts \
  apps/desktop/src/components/import/__tests__/TableImportDialogTheme.spec.ts
```

## 0.3.1 changes — 16 October 2026

- Production frontend build and full Vue/TypeScript type check passed after removing GitHub GUI navigation. The first type-check attempt exceeded the local memory limit while running alongside the build; the final standalone run passed with a 6 GB Node heap limit.
- Two focused GUI link tests passed; changelog text is retained when GitHub anchors are removed.
- Two upstream-proposal integration tests passed in a temporary Git repository, covering unchanged files, fork edits, protected/AI paths, additions, deletions, symlinks, and repeated proposals.
- All four GitHub workflow files parsed successfully; Python automation scripts compiled.
- No Rust implementation changes were made in this update (only the package version). Native installers are validated by the Windows/Linux CI builds before publication.

- Two release-packaging tests passed, verifying that unchanged workflows preserve the original source commit and packaging snapshots only refresh workflow definitions and corresponding hashes.

## 0.3.2 reviewed DBX 0.6.45 update — 16 October 2026

- Full Vue/TypeScript type check passed, including a final run after the implementation changes.
- Production frontend build passed (existing chunk-size/dynamic-import warnings).
- Consolidated regression results: **5,396 tests passed across 149 executed files**; one AI-only locale suite was skipped. **69 AI/MCP tests skipped** in total. The initial 150-file run was followed by an isolated seven-file rerun after adapting inherited test expectations for TablePilot's disabled AI/updater, branding and omitted web server. One UI test timed out during CPU contention and passed in isolation without changing its timeout.
- Coverage includes changed database/grid/schema/query/locale features, CSV/Excel import/export options, field mapping, SSH authentication, removed GitHub links and the TablePilot capability boundary.
- Desktop/core/workspace dependency declarations retain the AI/MCP/updater exclusions. Native command/API and plugin host.ai blocks remain in place.

Native Rust compilation was not run for this update because this session has no Rust toolchain. The earlier 0.3.0 native check does not validate these newer Rust changes. Windows/Linux installer compilation runs in GitHub Actions after merge; publication requires both builds to succeed. Live database connections, SSH jump-host operation, installer launch and end-to-end CSV/Excel round trips still require manual testing.

## 0.3.3 Plugin Center removal — 16 October 2026

- Full Vue/TypeScript type check and production frontend build passed (existing chunk-size/dynamic-import warnings).
- Focused regression results: **203 tests passed across 9 files**, with 45 existing AI/MCP tests skipped. Initial failures were inherited expectations for DBX branding and an AI toolbar setting; the two affected files passed after their expectations were updated for TablePilot.
- New tests verify that expanded/collapsed toolbars hide Plugin Center even with legacy settings, Driver Manager still opens, old plugin preferences normalize to disabled, marketplace checks/installations do not run, and driver/JDBC checks and updates still work.
- Two release-packaging tests passed. The build workflow parsed successfully and retains non-cancelling concurrency; Python release automation compiles.
- Windows/Linux CI builds now use ThinLTO, 16 code-generation units, workspace-crate caching and pnpm caching. The first build with this profile needs a fresh compiled cache. Build duration and runtime performance have not been benchmarked.

Native installer compilation and installation remain pending GitHub Actions/manual testing for this version. No Rust implementation changes were made (only the package version). Existing native build jobs are not cancelled.

## 0.4.0 official release — 16 October 2026

- Production frontend build passed (existing chunk-size/dynamic-import warnings).
- Focused regression results: **58 tests passed across 10 files** after an isolated two-file rerun. Coverage includes startup/hourly checks, failure reporting, manual release downloads, ignored versions, stale response handling, repository links, About branding/version, localization, driver updates and settings behavior. Two initial failures were an old upstream URL expectation and a test hook accidentally returning a mock as a cleanup callback; both were corrected.
- Four release-packaging tests passed, including actual mocked publication commands for normal/latest 0.4.0 and prerelease 0.3.3. Python compilation and packaging configuration checks passed.
- Local full Vue/TypeScript checks reported cleanup errors while source edits were in progress (unused link helpers, a settings search field and a temporarily removed debug-copy helper). These were corrected; the final source has not completed a standalone full type check locally. The installer CI runs the full frontend/type-check build before native compilation.
- Release checks use a dedicated native client with a fixed public TablePilot GitHub API endpoint and a 30-second timeout. It filters drafts/prereleases, nonstandard tags and incomplete installer releases, then compares semantic versions. No token is used; executable download/install commands remain blocked.
- Native unit coverage was added for release filtering but cannot run locally because the Rust toolchain is unavailable. Windows/Linux compilation remains a CI gate. Live update checks and installer upgrades require manual testing of the built application.
- Required LICENSE/NOTICE resources are included in the installer configuration. The application identifier and installer mode are preserved for upgrades.

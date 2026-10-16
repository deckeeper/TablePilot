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
- All three GitHub workflow files parsed successfully; Python automation scripts compiled.
- No Rust implementation changes were made in this update (only the package version). Native installers are validated by the Windows/Linux CI builds before publication.

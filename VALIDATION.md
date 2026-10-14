# Prototype validation

Validated in a Linux development environment on 14 October 2026.

- Production frontend build: passed. Vite reports upstream chunk-size and dynamic-import warnings.
- Full Vue/TypeScript type check: **passed**, including the final code cleanup.
- Focused test suite: **65 tests passed across 8 files**. This covers import source handling, import into an existing table, batch selection, import-dialog theme classes, CSV null-export options, Excel header options, SSH authentication selection, and the TablePilot capability boundary.
- Dependency graph: the desktop graph contains no `dbx-ai-provider`, `dbx-mcp`, or `tauri-plugin-updater` dependency.
- Native compilation: **passed** (`cargo check -p dbx`, all default desktop features, Rust 1.99.0). This checks the Linux desktop command layer and database core; it does not produce or run an installer. The compiler reports 18 warnings in the desktop crate, principally unused compatibility helpers.

## Limits

This delivery is source code with a manual installer-build workflow, not a prebuilt installer. Windows installation, Windows compilation, end-to-end GUI operation, live database connections, SSH jump-host connections, and round-trip CSV/Excel transfers have not been tested in this environment. The full upstream test suite was not run; tests that expect upstream AI/MCP behavior do not describe this edition.

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

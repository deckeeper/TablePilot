// TablePilot: exercise public API isolation while preserving database calls.
import { describe, expect, it, vi } from "vitest";
import { isExcludedOperation } from "../tablepilot";
vi.mock("@/lib/backend/tauriRuntime", () => ({ isTauriRuntime: () => true }));
vi.mock("@/lib/backend/debugLog", () => ({ appendDebugLog: vi.fn() }));
const backend = vi.hoisted(() => ({ aiComplete: vi.fn(), installMcpServer: vi.fn(), testSshTunnel: vi.fn(async () => "connected"), listTables: vi.fn(async () => ["customers"]) }));
vi.mock("@/lib/backend/tauri", () => backend);

describe("TablePilot capability boundary", () => {
  it("rejects AI execution and MCP installation before reaching the backend", async () => {
    const api = await import("../backend/api");
    await expect((api.aiComplete as (...args: unknown[]) => unknown)({})).rejects.toThrow("not included");
    await expect((api.installMcpServer as (...args: unknown[]) => unknown)()).rejects.toThrow("not included");
    expect(backend.aiComplete).not.toHaveBeenCalled();
    expect(backend.installMcpServer).not.toHaveBeenCalled();
  });
  it("permits database, import/export, driver and SSH operations", () => {
    for (const operation of ["testSshTunnel", "connectDb", "listTables", "previewImportFile", "importTableData", "exportTable", "installAgent", "getSystemProxyUrl", "checkForUpdates", "fetchChangelog"]) {
      expect(isExcludedOperation(operation)).toBe(false);
    }
  });
  it("blocks replacement by the upstream application updater", () => {
    for (const operation of ["downloadUpdate", "installDownloadedUpdate"]) expect(isExcludedOperation(operation)).toBe(true);
  });
});

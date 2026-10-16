import { beforeEach, describe, expect, it, vi } from "vitest";
import { useComponentUpdates } from "@/composables/useComponentUpdates";

const api = vi.hoisted(() => ({
  listInstalledAgents: vi.fn(), jdbcPluginStatus: vi.fn(), checkMcpServerStatus: vi.fn(),
  listPlugins: vi.fn(), fetchPluginMarketplaceCatalogs: vi.fn(), installMarketplacePlugin: vi.fn(),
  checkAgentUpdateBlockers: vi.fn(), upgradeAllAgents: vi.fn(), installJdbcPlugin: vi.fn(),
}));
vi.mock("@/lib/backend/api", () => api);
vi.mock("@/stores/settingsStore", () => ({ useSettingsStore: () => ({
  // Deliberately simulate an old settings file that still enables plugin updates.
  editorSettings: { autoUpdateDrivers: true, autoUpdateJdbc: true, autoUpdatePlugins: true },
}) }));
vi.mock("@/i18n", () => ({ currentLocale: () => "en" }));
vi.mock("@/lib/updates/updatePreviewMock", () => ({ isUpdatePreviewMockEnabled: () => false }));

beforeEach(() => {
  vi.resetAllMocks();
  api.listInstalledAgents.mockResolvedValue([{ db_type: "oracle", update_available: true }]);
  api.jdbcPluginStatus.mockResolvedValue({ installed: true, update_available: true });
  api.checkMcpServerStatus.mockResolvedValue(null);
  api.checkAgentUpdateBlockers.mockResolvedValue([]);
  api.upgradeAllAgents.mockResolvedValue({ upgraded: 1, failed: [], cancelled: 0 });
  api.installJdbcPlugin.mockResolvedValue({ installed: true });
});

describe("TablePilot component updates without Plugin Center", () => {
  it("checks database drivers and JDBC without requesting marketplace catalogs", async () => {
    const updates = useComponentUpdates({ isDesktop: true });
    expect(await updates.refresh()).toBe(true);
    expect(updates.driverUpdateCount.value).toBe(1);
    expect(updates.jdbcUpdateAvailable.value).toBe(true);
    expect(updates.pluginUpdateCount.value).toBe(0);
    expect(updates.totalUpdateCount.value).toBe(2);
    expect(api.listPlugins).not.toHaveBeenCalled();
    expect(api.fetchPluginMarketplaceCatalogs).not.toHaveBeenCalled();
  });

  it("updates drivers/JDBC while ignoring stale plugin update preferences", async () => {
    const updates = useComponentUpdates({ isDesktop: true });
    await updates.refresh();
    const result = await updates.autoUpdateEnabledComponents();
    expect(result.drivers).toBe(1);
    expect(result.jdbc).toBe(true);
    expect(result.plugins).toBe(0);
    expect(api.upgradeAllAgents).toHaveBeenCalledOnce();
    expect(api.installJdbcPlugin).toHaveBeenCalledOnce();
    expect(api.installMarketplacePlugin).not.toHaveBeenCalled();
    await updates.installCategory("plugins");
    expect(api.installMarketplacePlugin).not.toHaveBeenCalled();
  });
});

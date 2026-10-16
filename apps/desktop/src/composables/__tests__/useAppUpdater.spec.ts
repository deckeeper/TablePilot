// @vitest-environment happy-dom
import { createApp, defineComponent, h, reactive, nextTick, type App } from "vue";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import i18n from "@/i18n";
import { isNewerRemoteVersion, resolveReleaseTagUrl, resolveUpdateReleaseUrl, useAppUpdater } from "@/composables/useAppUpdater";
const mocks = vi.hoisted(() => ({
  checkForUpdates: vi.fn(),
  downloadUpdate: vi.fn(),
  cancelUpdateDownload: vi.fn(),
  installDownloadedUpdate: vi.fn(),
  getDownloadedUpdate: vi.fn(),
  discardDownloadedUpdate: vi.fn(),
  getAppVersion: vi.fn(),
  listen: vi.fn(),
  relaunch: vi.fn(),
  persist: vi.fn(),
  toast: vi.fn(),
}));
vi.mock("@/lib/backend/api", () => mocks);
vi.mock("@/lib/backend/tauriRuntime", () => ({ isTauriRuntime: () => true }));
vi.mock("@tauri-apps/api/event", () => ({ listen: mocks.listen }));
vi.mock("@tauri-apps/plugin-process", () => ({ relaunch: mocks.relaunch }));
vi.mock("@/composables/useToast", () => ({ useToast: () => ({ toast: mocks.toast }) }));
const settings = reactive({ updateDownloadSource: "official", ignoredUpdateVersion: "", updateNotificationsEnabled: true, autoDownloadUpdates: true, autoUpdateApp: true });
vi.mock("@/stores/settingsStore", () => ({ useSettingsStore: () => ({ editorSettings: settings, updateEditorSettingsAndPersist: mocks.persist }) }));
const info = { current_version: "1.0.0", latest_version: "1.1.0", update_available: true, portable_mode: false, manual_update_only: false, release_name: "v1.1.0", release_url: "https://example.com", release_notes: "Changes" };
const cache = { cache_id: "cached", version: "1.1.0", portable_mode: false, release_url: "https://example.com", release_notes: "Changes", downloaded_at: 1 };
let app: App;
function mount(options: Parameters<typeof useAppUpdater>[0] = {}) {
  let updater!: ReturnType<typeof useAppUpdater>;
  app = createApp(
    defineComponent({
      setup() {
        updater = useAppUpdater(options);
        return () => h("div");
      },
    }),
  );
  app.use(i18n);
  app.mount(document.createElement("div"));
  return updater;
}
function deferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (error: Error) => void;
  const promise = new Promise<T>((res, rej) => {
    resolve = res;
    reject = rej;
  });
  return { promise, resolve, reject };
}
async function flush() {
  for (let i = 0; i < 12; i++) await Promise.resolve();
  await nextTick();
}
beforeEach(() => {
  vi.resetAllMocks();
  settings.updateDownloadSource = "official";
  settings.ignoredUpdateVersion = "";
  settings.updateNotificationsEnabled = true;
  settings.autoDownloadUpdates = true;
  settings.autoUpdateApp = true;
  mocks.checkForUpdates.mockResolvedValue(info);
  mocks.downloadUpdate.mockResolvedValue(cache);
  mocks.getDownloadedUpdate.mockResolvedValue(null);
  mocks.getAppVersion.mockResolvedValue("1.0.0");
  mocks.listen.mockResolvedValue(vi.fn());
  mocks.persist.mockImplementation(async (values) => Object.assign(settings, values));
});
afterEach(() => {
  app?.unmount();
  vi.useRealTimers();
});
describe("TablePilot release checks", () => {
  beforeEach(() => { mocks.checkForUpdates.mockResolvedValue({ ...info, manual_update_only: true }); });
  it("uses only TablePilot release pages, including old mirror preferences", () => {
    for (const source of ["official", "cnb", "atomgit"]) {
      expect(resolveReleaseTagUrl("0.4.1", source)).toBe("https://github.com/deckeeper/TablePilot/releases/tag/v0.4.1");
    }
    expect(resolveUpdateReleaseUrl({ ...info, latest_version: "0.4.1", release_url: "https://example.com/foreign-release" }, "cnb", "https://example.com/fallback")).toBe("https://github.com/deckeeper/TablePilot/releases/tag/v0.4.1");
    expect(isNewerRemoteVersion("0.4.1", "v0.4.0")).toBe(true);
    expect(isNewerRemoteVersion("0.4.0", "v0.4.0")).toBe(false);
    expect(isNewerRemoteVersion("0.4.0-beta.1", "v0.4.0")).toBe(false);
  });
  it("checks at startup and hourly, shows a badge, and never downloads automatically", async () => {
    vi.useFakeTimers();
    const updater = mount();
    await updater.initialize(); await flush();
    expect(mocks.checkForUpdates).toHaveBeenCalledOnce();
    expect(updater.hasUpdateAvailable.value).toBe(true);
    expect(updater.showUpdateDialog.value).toBe(false);
    await vi.advanceTimersByTimeAsync(3_600_000); await flush();
    expect(mocks.checkForUpdates).toHaveBeenCalledTimes(2);
    expect(mocks.downloadUpdate).not.toHaveBeenCalled();
    expect(mocks.getDownloadedUpdate).not.toHaveBeenCalled();
    updater.dispose();
    await vi.advanceTimersByTimeAsync(3_600_000);
    expect(mocks.checkForUpdates).toHaveBeenCalledTimes(2);
  });
  it("opens the update dialog on request and leaves installation to the release page", async () => {
    const updater = mount();
    await updater.checkUpdates();
    expect(updater.showUpdateDialog.value).toBe(true);
    expect(updater.updateInfo.value?.manual_update_only).toBe(true);
    await updater.downloadUpdateInBackground();
    expect(mocks.downloadUpdate).not.toHaveBeenCalled();
    expect(mocks.installDownloadedUpdate).not.toHaveBeenCalled();
  });
  it("reports a failed check without claiming the installation is up to date", async () => {
    mocks.checkForUpdates.mockRejectedValue(new Error("Repository must be public"));
    const updater = mount(); await updater.checkUpdates();
    expect(updater.updateCheckFailed.value).toBe(true);
    expect(updater.updateCheckMessage.value).toContain("Repository must be public");
    expect(updater.updateInfo.value).toBeNull();
  });
  it("allows ignoring a release while retaining manual checks", async () => {
    const updater = mount(); await updater.checkUpdates();
    await updater.ignoreCurrentVersion();
    expect(mocks.persist).toHaveBeenCalledWith({ ignoredUpdateVersion: "1.1.0" });
    expect(updater.hasUpdateAvailable.value).toBe(false);
    await updater.checkUpdates();
    expect(updater.showUpdateDialog.value).toBe(true);
  });
  it("keeps the newest completed check when an older response arrives later", async () => {
    const pending = deferred<typeof info>();
    mocks.checkForUpdates.mockReturnValueOnce(pending.promise);
    const updater = mount();
    const old = updater.checkUpdates();
    await updater.cancelDownload();
    mocks.checkForUpdates.mockResolvedValue({ ...info, latest_version: "1.2.0", manual_update_only: true });
    await updater.checkUpdates();
    pending.resolve({ ...info, manual_update_only: true }); await old;
    expect(updater.updateInfo.value?.latest_version).toBe("1.2.0");
  });
});

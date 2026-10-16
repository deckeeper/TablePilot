// @vitest-environment happy-dom
import { createApp, nextTick } from "vue";
import { createPinia, setActivePinia } from "pinia";
import { createI18n } from "vue-i18n";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

vi.mock("@/lib/backend/tauriRuntime", () => ({ isTauriRuntime: () => false }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn(async () => undefined) }));
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(async () => () => {}),
  emit: vi.fn(async () => undefined),
}));
vi.mock("@tauri-apps/api/webviewWindow", () => ({
  getAllWebviewWindows: vi.fn(async () => []),
  WebviewWindow: { getByLabel: vi.fn(async () => null) },
}));
vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({
    label: "main",
    isMaximized: async () => false,
    isFullscreen: async () => false,
    isAlwaysOnTop: async () => false,
    setAlwaysOnTop: async () => {},
    onResized: async () => () => {},
    onFocusChanged: async () => () => {},
    minimize: async () => {},
    toggleMaximize: async () => {},
  }),
}));

vi.mock("@/components/ui/button", () => ({
  Button: { name: "ButtonStub", template: `<button><slot /></button>` },
}));
vi.mock("@/components/ui/tooltip", () => ({
  Tooltip: { name: "TooltipStub", template: `<span><slot /></span>` },
  TooltipTrigger: { name: "TooltipTriggerStub", template: `<span><slot /></span>` },
  TooltipContent: { name: "TooltipContentStub", template: `<span><slot /></span>` },
}));
vi.mock("@/components/ui/LightDropdown.vue", () => ({
  default: { name: "LightDropdownStub", props: ["items"], emits: ["update:modelValue"], template: `<div><button v-for="item in items" :key="item.value" @click="$emit('update:modelValue', item.value)">{{ item.label }}</button></div>` },
}));
vi.mock("@/components/layout/WindowControls.vue", () => ({
  default: { name: "WindowControlsStub", template: `<div />` },
}));
vi.mock("@/components/export/ExportProgressPopover.vue", () => ({
  default: { name: "ExportProgressPopoverStub", template: `<div />` },
}));
vi.mock("@/components/layout/ToolbarUpdateIcon.vue", () => ({
  default: { name: "ToolbarUpdateIconStub", template: `<span data-toolbar-update-idle />` },
}));

import AppToolbar from "../AppToolbar.vue";
import { useSettingsStore } from "@/stores/settingsStore";

const defaultToolbarProps = {
  isDark: false,
  themeMode: "system" as const,
  showSidebarExpand: false,
  showAiPanel: false,
  activeAiRunCount: 0,
  awaitingAiRunCount: 0,
  showHistory: false,
  showSqlLibrary: false,
  sqlLibrarySaveFeedbackId: 0,
  showSqlFilePanel: false,
  showDriverStore: false,
  showPluginCenter: false,
  showSettingsPage: false,
  checkingUpdates: false,
  hasUpdateAvailable: false,
  isDownloadingUpdate: false,
  downloadProgress: null,
  updateReadyToInstall: false,
  updateReady: false,
  agentDriverUpdateCount: 0,
  hasMcpUpdateAvailable: false,
  hasConnections: false,
  canNewQuery: false,
  hasSqlFileConnections: false,
  immediateSyncing: false,
};

let pinia: ReturnType<typeof createPinia>;
let mounted: { host: HTMLDivElement; unmount: () => void } | null = null;

function mount(props: Record<string, unknown> = {}) {
  const host = document.createElement("div");
  document.body.appendChild(host);
  const app = createApp(AppToolbar, {
    ...defaultToolbarProps,
    ...props,
  });
  app.use(pinia);
  app.use(
    createI18n({
      legacy: false,
      locale: "en",
      missingWarn: false,
      fallbackWarn: false,
      messages: {
        en: {
          settings: { title: "Settings" },
          toolbar: { theme: "Theme" },
          updates: { check: "Check for updates", updateAction: "Update" },
          exportProgress: { tooltip: "Export" },
        },
      },
    }),
  );
  app.mount(host);
  return {
    host,
    unmount: () => {
      app.unmount();
      host.remove();
    },
  };
}


vi.mock("@/components/plugins/PluginShortcutToolbar.vue", () => ({
  default: { template: `<div data-plugin-shortcut-toolbar />` },
}));

describe("TablePilot hides Plugin Center while retaining Driver Manager", () => {
  beforeEach(() => {
    pinia = createPinia();
    setActivePinia(pinia);
  });
  afterEach(() => {
    mounted?.unmount();
    mounted = null;
    vi.restoreAllMocks();
  });
  it.each([1200, 1])("removes plugin entries from the toolbar and menus at width %i despite old settings", async (width) => {
    vi.spyOn(HTMLElement.prototype, "clientWidth", "get").mockReturnValue(width);
    const settings = useSettingsStore();
    // Simulate legacy persisted settings and bypass normalization deliberately.
    settings.editorSettings.toolbarItems.pluginCenter = true;
    settings.editorSettings.pluginShortcuts.enabled = true;
    settings.editorSettings.pluginShortcuts.position = "plugin-center";
    const openDrivers = vi.fn();
    const openPlugins = vi.fn();
    mounted = mount({ onOpenDriverStore: openDrivers, onOpenPluginCenter: openPlugins });
    await nextTick();
    await new Promise((resolve) => setTimeout(resolve, 50));
    expect(mounted.host.textContent).not.toContain("toolbar.pluginCenter");
    expect(mounted.host.querySelector("[data-plugin-shortcut-toolbar]")).toBeNull();
    const driverButton = Array.from(mounted.host.querySelectorAll("button")).find((button) => button.textContent?.includes("toolbar.driverManager"));
    expect(driverButton).toBeTruthy();
    driverButton!.click();
    expect(openDrivers).toHaveBeenCalledOnce();
    expect(openPlugins).not.toHaveBeenCalled();
  });
});

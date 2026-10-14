<!-- TablePilot modification: independent desktop edition without AI/MCP; see FORK_CHANGES.md. -->
<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { Database, Loader2, X } from "@lucide/vue";
import * as api from "@/lib/backend/api";

import { Button } from "@/components/ui/button";

import { useToast } from "@/composables/useToast";
import type { InstalledPlugin } from "@/types/database";
import type { PluginDataGrant } from "@/types/pluginData";
import { PLUGIN_DATA_READ_PERMISSION } from "@/types/pluginData";
import type { PluginToolPreview } from "@/types/pluginAiTools";

/**
 * Plugin Center controls for what a plugin may reach beyond its own UI:
 * built-in AI access to its MCP tools, and the connections it may read
 * through `host.data:read`. AI tool access is opt-in — off until the user
 * turns it on, and the choice survives restarts and plugin updates. Data
 * reads are off until the user consents.
 */
const props = defineProps<{ plugin: InstalledPlugin }>();

const { t } = useI18n();
const { toast } = useToast();

const pluginId = computed(() => props.plugin.manifest.id);

const declaresDataRead = computed(() => (props.plugin.manifest.permissions || []).includes(PLUGIN_DATA_READ_PERMISSION));



const preview = ref<PluginToolPreview | null>(null);

const previewError = ref("");
const grants = ref<PluginDataGrant[]>([]);
const revoking = ref<string | null>(null);

function errorText(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

async function load() {
  preview.value = null;
  previewError.value = "";
  grants.value = [];
  const id = pluginId.value;
  const currentGrants = declaresDataRead.value ? await api.getPluginDataGrants(id).catch(() => [] as PluginDataGrant[]) : [];
  // The selection may have moved on while the requests were in flight.
  if (id !== pluginId.value) return;
  grants.value = currentGrants;
}





async function revokeGrant(connectionId: string) {
  revoking.value = connectionId;
  try {
    grants.value = await api.setPluginDataGrant(pluginId.value, connectionId, false);
  } catch (error) {
    toast(t("pluginPlatform.dataAccess.revokeFailed", { message: errorText(error) }), 5000);
  } finally {
    revoking.value = null;
  }
}

watch(pluginId, () => void load(), { immediate: true });
</script>

<template>
  <div v-if="declaresDataRead" class="space-y-3">

    <section v-if="declaresDataRead" class="space-y-3 rounded-lg border p-4" data-plugin-data-access>
      <div class="flex min-w-0 items-start gap-2">
        <Database class="mt-0.5 size-4 shrink-0 text-primary" />
        <div class="min-w-0">
          <div class="text-sm font-medium">{{ t("pluginPlatform.dataAccess.title") }}</div>
          <p class="mt-1 text-xs leading-5 text-muted-foreground">{{ t("pluginPlatform.dataAccess.description") }}</p>
        </div>
      </div>
      <div v-if="!grants.length" class="rounded-md border border-dashed px-3 py-2 text-xs text-muted-foreground">{{ t("pluginPlatform.dataAccess.none") }}</div>
      <ul v-else class="divide-y rounded-md border">
        <li v-for="grant in grants" :key="grant.connectionId" class="flex items-center gap-2 px-3 py-2">
          <div class="min-w-0 flex-1 truncate text-xs">
            <span v-if="grant.connectionName">{{ grant.connectionName }}</span>
            <span v-else class="text-muted-foreground">{{ t("pluginPlatform.dataAccess.missingConnection", { id: grant.connectionId }) }}</span>
          </div>
          <Button size="sm" variant="ghost" class="h-6 gap-1 px-2 text-xs text-destructive" :disabled="revoking === grant.connectionId" @click="revokeGrant(grant.connectionId)">
            <Loader2 v-if="revoking === grant.connectionId" class="size-3 animate-spin" />
            <X v-else class="size-3" />
            {{ t("pluginPlatform.dataAccess.revoke") }}
          </Button>
        </li>
      </ul>
    </section>
  </div>
</template>

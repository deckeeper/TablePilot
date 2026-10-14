// TablePilot modification: MCP is not shipped; no background status requests.
import { ref } from "vue";
export function useMcpUpdateBadge(_options: {isDesktop: boolean; updateNotificationsEnabled: () => boolean; shouldDeferRefresh?: () => boolean}) {
 const mcpUpdateAvailable = ref(false);
 const refreshMcpUpdateStatus = async () => {};
 const applyMcpStatus = (_available: boolean, _requestId?: number) => {};
 const invalidateMcpUpdateStatus = () => {};
 const handleMcpStatusChanged = (_event: Event) => {};
 return {mcpUpdateAvailable,refreshMcpUpdateStatus,applyMcpStatus,invalidateMcpUpdateStatus,handleMcpStatusChanged};
}

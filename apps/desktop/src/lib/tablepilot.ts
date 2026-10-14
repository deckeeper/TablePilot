// TablePilot desktop edition. These are build capabilities, not user settings.
export const AI_ENABLED: boolean = false;
export const MCP_ENABLED: boolean = false;
export const APP_UPDATES_ENABLED: boolean = false;
export const APP_NAME = "TablePilot";
export function isExcludedOperation(name: string): boolean {
 return /(?:^ai|Ai|Mcp|PromptTemplate|UserSkills|MaxAgentTurns|MaxRetries|CcSwitch)/.test(name)
   || ["checkForUpdates", "fetchChangelog", "downloadUpdate", "cancelUpdateDownload", "installDownloadedUpdate", "getDownloadedUpdate", "discardDownloadedUpdate"].includes(name);
}

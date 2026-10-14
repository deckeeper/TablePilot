// TablePilot modification: reproducible desktop packaging, without upstream publishing.
import { spawnSync } from "node:child_process";
const platform = process.platform;
if (!["win32", "linux"].includes(platform)) throw new Error("This prototype packages Windows and Linux.");
const args = ["node_modules/@tauri-apps/cli/tauri.js", "build", "--bundles", platform === "win32" ? "nsis" : "deb"];
if (platform === "linux") args.push("--config", "src-tauri/tauri.linux.conf.json");
const result = spawnSync(process.execPath, args, { stdio: "inherit", env: { ...process.env, NODE_OPTIONS: process.env.NODE_OPTIONS || "--max-old-space-size=8192" } });
process.exit(result.status ?? 1);

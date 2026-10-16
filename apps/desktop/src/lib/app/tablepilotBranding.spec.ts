import { describe, expect, it } from "vitest";
import { tablepilotMessages } from "./tablepilotBranding";

describe("TablePilot locale branding", () => {
  it("covers non-Latin suffixes and nested messages without changing environment keys", () => {
    const original = { title: "About DBX", nested: { ja: "DBXを終了", ko: "DBX를 종료", identifier: "DBX_SECRET_KEY_FILE" }, rows: ["DBX", 42] };
    expect(tablepilotMessages(original)).toEqual({ title: "About TablePilot", nested: { ja: "TablePilotを終了", ko: "TablePilot를 종료", identifier: "DBX_SECRET_KEY_FILE" }, rows: ["TablePilot", 42] });
    expect(original.title).toBe("About DBX");
  });
});

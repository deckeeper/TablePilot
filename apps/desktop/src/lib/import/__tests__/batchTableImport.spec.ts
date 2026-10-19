import { describe, expect, it } from "vitest";
import { matchImportTable } from "@/lib/import/batchTableImport";

describe("matchImportTable", () => {
  it("matches exported filenames and unique case-insensitive table names", () => {
    expect(matchImportTable("C:\\exports\\orders.csv", "", 1, ["orders", "customers"])).toBe("orders");
    expect(matchImportTable("CUSTOMERS.xlsx", "Sheet1", 1, ["customers"])).toBe("customers");
  });
  it("prefers worksheet names and never guesses by position", () => {
    expect(matchImportTable("backup.xlsx", "orders", 2, ["customers", "orders"])).toBe("orders");
    expect(matchImportTable("orders.xlsx", "Sheet1", 2, ["orders"])).toBe("");
    expect(matchImportTable("random.csv", "", 1, ["orders"])).toBe("");
  });
  it("leaves ambiguous case-folded names for explicit selection", () => {
    expect(matchImportTable("ORDERS.csv", "", 1, ["Orders", "orders"])).toBe("");
    expect(matchImportTable("Orders.csv", "", 1, ["Orders", "orders"])).toBe("Orders");
  });
});

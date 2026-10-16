/** Replace product prose in every locale while preserving technical identifiers. */
export function tablepilotMessages<T>(value: T): T {
  if (typeof value === "string") return value.replace(/(?<![A-Za-z0-9_])DBX(?![A-Za-z0-9_])/g, "TablePilot") as T;
  if (Array.isArray(value)) return value.map(tablepilotMessages) as T;
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, tablepilotMessages(item)])) as T;
  return value;
}

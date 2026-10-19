/** Match by name only. Never use file order to guess where existing data belongs. */
export function matchImportTable(fileName: string, sheetName: string, sheetCount: number, tables: readonly string[]): string {
  const baseName = (fileName.split(/[\\/]/).pop() || fileName).replace(/\.[^.]+$/, "");
  const candidates = sheetName ? [sheetName, ...(sheetCount === 1 ? [baseName] : [])] : [baseName];
  for (const candidate of candidates) {
    const exact = tables.filter((table) => table === candidate);
    if (exact.length === 1) return exact[0]!;
    const folded = tables.filter((table) => table.toLowerCase() === candidate.toLowerCase());
    if (folded.length === 1) return folded[0]!;
  }
  return "";
}

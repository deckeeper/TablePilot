// TablePilot: hide GitHub navigation in the desktop interface.
export function showExternalLink(url?: string): boolean {
  if (!url) return false;
  try {
    const parsed = new URL(url);
    const host = parsed.hostname.toLowerCase();
    return ["http:", "https:"].includes(parsed.protocol)
      && host !== "github.com" && !host.endsWith(".github.com");
  } catch { return false; }
}

// Preserve changelog text while removing navigation to GitHub.
export function removeGithubAnchors(html: string): string {
  const template = document.createElement("template");
  template.innerHTML = html;
  template.content.querySelectorAll("a[href]").forEach((anchor) => {
    if (!showExternalLink(anchor.getAttribute("href") ?? undefined)) {
      anchor.replaceWith(...anchor.childNodes);
    }
  });
  return template.innerHTML;
}

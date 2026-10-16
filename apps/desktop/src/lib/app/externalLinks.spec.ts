// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";
import { removeGithubAnchors, showExternalLink } from "./externalLinks";

describe("TablePilot external navigation", () => {
  it("hides GitHub links and rejects unsupported protocols", () => {
    for (const url of [undefined, "", "https://github.com/t8y2/dbx", "https://WWW.GITHUB.COM/project", "javascript:alert(1)", "bad-url"]) {
      expect(showExternalLink(url)).toBe(false);
    }
    expect(showExternalLink("https://docs.example.com/guide")).toBe(true);
  });
  it("keeps changelog text and non-GitHub links", () => {
    const result = removeGithubAnchors('<a href="https://github.com/t8y2/dbx/pull/1"><strong>Fix</strong></a> <a href="https://example.com">Docs</a>');
    expect(result).toBe('<strong>Fix</strong> <a href="https://example.com">Docs</a>');
  });
});

import { describe, expect, it } from "vitest";

import { renderMarkdown } from "./markdown";

describe("the renderer", () => {
  it("renders markdown and a fenced code block", () => {
    const html = renderMarkdown("Ecco **un** esempio.\n\n```rust\nfn main() {}\n```\n");
    expect(html).toContain("<strong>un</strong>");
    expect(html).toContain('<pre><code class="language-rust">');
  });

  it("never renders HTML the model wrote: it is escaped", () => {
    const html = renderMarkdown('<script>alert(1)</script> <img src="x" onerror="alert(1)">');
    expect(html).not.toContain("<script");
    expect(html).not.toContain("<img");
    expect(html).toContain("&lt;script&gt;");
  });

  it("renders a link as text that shows its target, with nothing to follow", () => {
    const html = renderMarkdown("vedi [qui](https://example.com/x)");
    expect(html).not.toContain("<a");
    // the SPACE is load-bearing: `data-href=` contains `href=` (R13-1).
    expect(html).not.toContain(" href=");
    expect(html).toContain('data-href="https://example.com/x"');
    expect(html).toContain("qui");
  });

  it("renders an image as text and never fetches it", () => {
    const html = renderMarkdown("![una foto](https://example.com/i.png)");
    expect(html).not.toContain("<img");
    expect(html).toContain("[una foto] https://example.com/i.png");
  });

  it("drops a javascript: target the way validateLink does", () => {
    const html = renderMarkdown("[x](javascript:alert(1))");
    expect(html).not.toContain("data-href=\"javascript");
  });

  it("escapes what its own two rules write: the alt, the image's address and the link's (AUD-719 of the audit of 2026-09-30)", () => {
    // ⛔ THE THREE PLACES WHERE OUR RULES PUT THE MODEL'S TEXT INTO HTML WE WRITE, and `html: false` covers none of them:
    // the probe above reaches the library's escaping, not ours. The ALT is the one that carries today -- the rule gets its
    // SOURCE (`token.content`), unescaped by markdown-it, so without `escapeHtml` an <img> with its handler would reach the
    // Chat's v-html alive.
    const image = renderMarkdown("![<img src=x onerror=alert(1)>](https://example.com/i.png?a=1&b=2)");
    expect(image).not.toContain("<img");
    expect(image).toContain("[&lt;img src=x onerror=alert(1)&gt;] https://example.com/i.png?a=1&amp;b=2");
    // ⚠️ THE ADDRESSES: `normalizeLink` already percent-encodes the quote and the angle brackets, so it is the `&` that
    // shows our escape at work -- and our escape is the barrier left the day the library's normalizer changes.
    const link = renderMarkdown('[qui](https://example.com/?a=1&b="><b>x)');
    expect(link).not.toContain("<b>");
    expect(link).toContain('data-href="https://example.com/?a=1&amp;b=%22%3E%3Cb%3Ex"');
  });
});

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
});

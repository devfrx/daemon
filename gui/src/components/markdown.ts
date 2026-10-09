import MarkdownIt from "markdown-it";

/**
 * The renderer of the chat (decision 51 of the north star; §9 row 1 of the sub-project 2 design).
 *
 * ⛔ PRESET `default` WITH `html: false` AND `linkify: false` WRITTEN OUT, not relied on -- read
 * INSIDE the 15.0.2 package on 2026-09-15 (task 14, step 2): raw HTML in the text is escaped,
 * nothing becomes a link on its own, and `validateLink` refuses `vbscript:`, `javascript:`,
 * `file:` and `data:` (images apart). The text of a model is untrusted (ADR-0014) and is rendered
 * as TEXT AND CODE, NEVER AS HTML.
 *
 * ⛔ AND TWO RULES OF OURS, because "no link opens by itself" (§6a) is not what the defaults do:
 * - a link becomes a `<span>` that SHOWS its target (`data-href`, drawn by the stylesheet) and has
 *   no `href`: nothing to follow, and the address is visible instead of hidden behind the text;
 * - an image becomes text -- the alt and the address -- and NEVER `<img>`: an `<img src>` is
 *   fetched by the browser with nobody clicking, which would let an untrusted text make the gui
 *   reach a URL of its choosing (P-87). ⚠️ THIS RULE IS THE ONLY DEFENCE OF THAT CHANNEL TODAY:
 *   the exfiltration canary of ADR-0016 is staggered, not built (row Q17 of §8 of the sub-project 1
 *   spec, ⏳), and as that ADR decides it -- a sensor's verdict on content going out of the kernel's
 *   loop -- it would not see a request the browser makes by itself (deduced, not measured).
 */
const md = new MarkdownIt("default", { html: false, linkify: false });

md.renderer.rules.link_open = (tokens, index) => {
  // `attrGet` is typed `string | number | null` in the shipped `.d.mts` (R7-1): `String` narrows it.
  const href = String(tokens[index]?.attrGet("href") ?? "");
  return `<span class="link" data-href="${md.utils.escapeHtml(href)}">`;
};

md.renderer.rules.link_close = () => "</span>";

md.renderer.rules.image = (tokens, index) => {
  const token = tokens[index];
  const alt = md.utils.escapeHtml(token?.content ?? "");
  const src = md.utils.escapeHtml(String(token?.attrGet("src") ?? ""));
  return `<span class="image">[${alt}] ${src}</span>`;
};

export function renderMarkdown(text: string): string {
  return md.render(text);
}

import axe from "axe-core";

/**
 * Every violation axe finds under a node, as `rule: targets`, and nothing else. Its second user came with the design
 * system's task 3 -- `components/kit.test.ts`, after `a11y.test.ts` -- so it lives here once.
 *
 * ⛔ `color-contrast` IS OFF UNLESS ASKED, AND NOT IGNORED: under jsdom axe files it under `incomplete` every time --
 * there is no layout to read a background from (measured on 2026-09-15, P-86 of part 2) -- so a green from it would
 * prove nothing there. `tokens/contrast.test.ts` holds the families of the pairs; the probes of the kit page, in the
 * real browser, turn it ON with `contrast: true` (task 4).
 */
export async function violations(node: Element, options: { contrast?: boolean } = {}): Promise<string[]> {
  const results = await axe.run(node, { rules: { "color-contrast": { enabled: options.contrast === true } } });
  return results.violations.map((violation) => `${violation.id}: ${violation.nodes.map((n) => n.target.join(" ")).join(", ")}`);
}

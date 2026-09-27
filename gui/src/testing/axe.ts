import axe from "axe-core";

/**
 * Every violation axe finds under a node, as `rule: targets`, and nothing else. Its second user came with the design
 * system's task 3 -- `components/kit.test.ts`, after `a11y.test.ts` -- so it lives here once.
 *
 * ⛔ `color-contrast` IS OFF UNLESS ASKED, AND NOT IGNORED: under jsdom axe files it under `incomplete` every time --
 * there is no layout to read a background from (measured on 2026-09-15, P-86 of part 2) -- so a green from it would
 * prove nothing there. `tokens/contrast.test.ts` holds the families of the pairs; the probes in the real browser turn it
 * ON with `contrast: true`, since task 4.
 */
export async function violations(node: Element, options: { contrast?: boolean } = {}): Promise<string[]> {
  const results = await axe.run(node, { rules: { "color-contrast": { enabled: options.contrast === true } } });
  return results.violations.map((violation) => `${violation.id}: ${violation.nodes.map((n) => n.target.join(" ")).join(", ")}`);
}

/**
 * ⛔ THE NON-VACUITY OF `axe` (R3-7 of the review): an empty list of violations says something only if the contrast was
 * JUDGED -- nodes among the passes, none left incomplete. Under jsdom axe files every contrast as incomplete; in the
 * browser it must not. Its second user came with the design system's task 8 -- the frame, after the kit page -- so it
 * lives here once.
 */
export async function contrastJudged(node: Element): Promise<{ passes: number; incomplete: number }> {
  const results = await axe.run(node, { runOnly: ["color-contrast"] });
  const count = (list: axe.Result[]): number => list.find((rule) => rule.id === "color-contrast")?.nodes.length ?? 0;
  return { passes: count(results.passes), incomplete: count(results.incomplete) };
}

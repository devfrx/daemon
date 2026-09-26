import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

const HERE = dirname(fileURLToPath(import.meta.url));
const DOCKVIEW = join(HERE, "..", "..", "node_modules", "dockview", "dist", "styles", "dockview.css");

/** Every `--dv-*` a stylesheet SETS in the blocks that open with `selector {`, in the order they come. */
function setIn(css: string, selector: string): string[] {
  const found: string[] = [];
  let from = css.indexOf(`${selector} {`);
  while (from >= 0) {
    const body = css.slice(from, css.indexOf("}", from));
    for (const match of body.matchAll(/(--dv-[a-z0-9-]+)\s*:/g)) found.push(match[1] ?? "");
    from = css.indexOf(`${selector} {`, from + selector.length);
  }
  return found;
}

/**
 * ⛔ OUR THEME REPLACES THE ABYSS THEME, SO IT SETS WHAT THAT ONE SETS (design system, section (c); P-6 of the plan).
 * The reference is `.dockview-theme-abyss`, in its two blocks of `dockview.css` 8.3.1: a variable it sets and ours
 * forgot would fall back to `dockview`'s own default -- a colour by hand from a stylesheet we do not own. Two families
 * are out, each with its reason, measured in `dockview.css` 8.3.1 on 2026-09-23:
 *   `--dv-tab-group-*`  the TAB GROUPS, a feature the SPA does not turn on: only `.dv-tab-group-chip` and the
 *                       group's underline read them -- the nine colours and the five sizes alike;
 *   `--dv-color-*`      a theme's own PALETTE (`--dv-color-abyss-dark`, ...): every rule that reads one sits under a
 *                       `.dockview-theme-*` selector, the one that carries it.
 */
describe("our dockview theme", () => {
  const reference = setIn(readFileSync(DOCKVIEW, "utf8"), ".dockview-theme-abyss");
  const ours = new Set(setIn(readFileSync(join(HERE, "dock.css"), "utf8"), ".dockview-theme-harness"));

  it("sees the reference theme it replaces", () => {
    // ⛔ NON-VACUITY: a `dockview` that renamed its theme would leave nothing to compare, and a green.
    expect(reference.length).toBeGreaterThan(40);
  });

  it("sets every variable the reference theme sets, but the tab groups' and the palette's", () => {
    const needed = reference.filter((name) => !name.startsWith("--dv-tab-group-") && !name.startsWith("--dv-color-"));
    expect(needed.filter((name) => !ours.has(name))).toEqual([]);
  });
});

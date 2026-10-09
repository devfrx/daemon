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

/** Every `--dv-*` a stylesheet READS with a fallback of its own that is a COLOUR: `#…`, `rgb(…)`, `hsl(…)`, `transparent`. */
function readWithAColour(css: string): string[] {
  const found = new Set<string>();
  for (const match of css.matchAll(/var\((--dv-[a-z0-9-]+),\s*(?:#|rgba?\(|hsla?\(|transparent\b)/g)) found.add(match[1] ?? "");
  return [...found].sort();
}

/**
 * ⛔ OUR THEME REPLACES THE ABYSS THEME, SO IT SETS WHAT THAT ONE SETS (design system, section (c); P-6 of the plan).
 * The reference is `.dockview-theme-abyss`, in its two blocks of `dockview.css` 8.3.1: a variable it sets and ours
 * forgot would fall back to `dockview`'s own default -- a colour by hand from a stylesheet we do not own. Two families
 * are out, each with its reason:
 *   `--dv-tab-group-*`  the TAB GROUPS, a feature the SPA does not turn on: what reads them is the groups' own pieces --
 *                       the chip and its continuation, the group's underline, its colour in the overflow list, the
 *                       swatch of the colours' menu (AUD-2075 of the audit of 2026-09-30) -- the nine colours and the
 *                       five sizes alike. Read in `dockview.css` 8.3.1 on 2026-10-09, from `gui/`, with
 *                       `awk '/\{$/{sel=$0} /var\(--dv-tab-group-/{print sel}' node_modules/dockview/dist/styles/dockview.css | sort -u`;
 *   `--dv-color-*`      a theme's own PALETTE (`--dv-color-abyss-dark`, ...): every rule that reads one sits under a
 *                       `.dockview-theme-*` selector, the one that carries it.
 * ⛔ AND THE REFERENCE DOES NOT SEE EVERY SUCH COLOUR (AUD-2213 of the audit of 2026-09-30): a variable no theme sets, read
 * with a colour written in `dockview.css`, is the same escape -- so that list is a reference of its own, below.
 */
describe("our dockview theme", () => {
  const sheet = readFileSync(DOCKVIEW, "utf8");
  const reference = setIn(sheet, ".dockview-theme-abyss");
  const ours = new Set(setIn(readFileSync(join(HERE, "dock.css"), "utf8"), ".dockview-theme-harness"));

  it("sees the reference theme it replaces", () => {
    // ⛔ NON-VACUITY: a `dockview` that renamed its theme would leave nothing to compare, and a green.
    expect(reference.length).toBeGreaterThan(40);
  });

  it("sets every variable the reference theme sets, but the tab groups' and the palette's", () => {
    const needed = reference.filter((name) => !name.startsWith("--dv-tab-group-") && !name.startsWith("--dv-color-"));
    expect(needed.filter((name) => !ours.has(name))).toEqual([]);
  });

  it("sets every variable dockview reads with a colour of its own, but those of the modules the SPA does not turn on (AUD-2213 of the audit of 2026-09-30)", () => {
    // Out, with their reason: the drop compass and the smart guides, modules of `dockview-core` 8.3.1 that only their own
    // option turns on -- `dndCompass` and `smartGuides` -- and `createDock` in `frame/dock.ts` passes neither.
    const read = readWithAColour(sheet).filter((name) => !name.startsWith("--dv-dnd-compass-") && !name.startsWith("--dv-smart-guides-"));
    // ⛔ NON-VACUITY: the sheet does read variables so, and some of them are ours already.
    expect(read.filter((name) => ours.has(name)).length).toBeGreaterThan(0);
    expect(read.filter((name) => !ours.has(name))).toEqual([]);
  });
});

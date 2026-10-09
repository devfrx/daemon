import { readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative, sep } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

const SRC = dirname(dirname(fileURLToPath(import.meta.url)));

/** Every file under `gui/src` ending in one of the extensions, relative to `gui/src`, with `/`. */
function sources(extensions: readonly string[]): string[] {
  const found: string[] = [];
  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const path = join(dir, entry.name);
      if (entry.isDirectory()) walk(path);
      else if (extensions.some((extension) => entry.name.endsWith(extension))) found.push(relative(SRC, path).split(sep).join("/"));
    }
  };
  walk(SRC);
  return found.sort();
}

const read = (file: string): string => readFileSync(join(SRC, file), "utf8");

/** The design-system rules on how the tokens are USED -- controls 4 and 5 of the design. */
describe("the token discipline", () => {
  const all = sources([".vue", ".ts", ".css"]);

  it("sees the files it judges", () => {
    // ⛔ NON-VACUITY: a walk that found nothing would pass both probes below.
    expect(all.filter((file) => file.endsWith(".vue")).length).toBeGreaterThan(10);
    expect(all).toContain("tokens/dock.css");
  });

  it("keeps the scales to `tokens/`: no component reads a `--ref-*`", () => {
    expect(all.filter((file) => !file.startsWith("tokens/") && read(file).includes("var(--ref-"))).toEqual([]);
  });

  /**
   * A colour by hand. Control 5 of the design, widened to the colour functions of CSS Color 4 (E6 of the design-system plan,
   * the owner's choice A, 2026-09-24): `rgb()`, `hsl()`, `hwb()`, `lab()`, `lch()`, their `ok` forms and `color()`, with
   * `color-mix()` of Color 5 -- in any case, as CSS reads a function's name (AUD-2214 of the audit of 2026-09-30).
   * ⚠️ A colour BY NAME -- `white`, `red` -- is NOT seen: catching it would take a list or a guess, with false positives.
   */
  const HAND = /#[0-9a-f]{3,8}\b|rgba?\(|hsla?\(|\bhwb\(|\b(?:ok)?lab\(|\b(?:ok)?lch\(|\bcolor\(|\bcolor-mix\(/i;

  it("sees a colour by hand in every form it names, and not a role or a word of code", () => {
    // ⛔ THE CONTROL PROVED IN BOTH DIRECTIONS: the probe below is green on today's files, which says nothing of what it sees.
    const hand = ["#fff", "#1F9CF0", "rgb(0 0 0)", "RGBA(0, 0, 0, 0.5)", "hsl(0 0% 0%)", "hwb(0 0% 0%)", "lab(50% 0 0)", "OKLCH(0.5 0.1 20)", "color(display-p3 1 0 0)", "color-mix(in srgb, red, blue)"];
    const not = ["var(--color-text)", "currentColor", "background-color: var(--color-bg)", "label(block)", "colourOf(token)"];
    expect(hand.filter((text) => !HAND.test(text))).toEqual([]);
    expect(not.filter((text) => HAND.test(text))).toEqual([]);
  });

  it("writes no colour by hand outside the token files", () => {
    // ⚠️ THE `.ts` FILES ARE NOT JUDGED: control 5 names the `.vue` and `tokens/dock.css`, and a colour written in
    // TypeScript -- a canvas, tomorrow -- passes it.
    const judged = all.filter((file) => file.endsWith(".vue") || file === "tokens/dock.css");
    const offenders = judged.flatMap((file) =>
      read(file)
        .split(/\r?\n/)
        .flatMap((line, index) => (HAND.test(line) ? [`${file}:${index + 1}: ${line.trim()}`] : [])),
    );
    expect(offenders).toEqual([]);
  });
});

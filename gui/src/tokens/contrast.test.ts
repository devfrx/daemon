import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

const css = readFileSync(join(dirname(fileURLToPath(import.meta.url)), "tokens.css"), "utf8");

/** Every `--name: #rrggbb;` of the file. ⛔ READ FROM THE FILE AND NOT RETYPED: a probe that carried
 * its own copy of the palette would go on passing after someone edits the real one. */
const tokens: Record<string, string> = Object.fromEntries(
  [...css.matchAll(/--([a-z-]+):\s*(#[0-9a-fA-F]{6});/g)].map((found) => [found[1] ?? "", found[2] ?? ""]),
);

/** WCAG 2.1 relative luminance, and the contrast ratio built on it. */
function luminance(hex: string): number {
  const channel = (index: number): number => {
    const value = Number.parseInt(hex.slice(index, index + 2), 16) / 255;
    return value <= 0.03928 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * channel(1) + 0.7152 * channel(3) + 0.0722 * channel(5);
}

export function contrast(foreground: string, background: string): number {
  const [high, low] = [luminance(foreground), luminance(background)].sort((a, b) => b - a) as [number, number];
  return (high + 0.05) / (low + 0.05);
}

/** The tokens that colour TEXT, and the tokens that are drawn UNDER text. ⛔ EVERY PAIR AND NOT A
 * HAND-KEPT LIST OF "the pairs the components use": a list would rot the day a template drew a
 * colour on a surface it never had before, and the whole point of tokens is that any text colour
 * may land on any surface. `--line` is a border and is not here. */
const TEXT = ["ink", "ink-dim", "accent", "warn", "stop"];
const SURFACES = Object.keys(tokens).filter((name) => name.startsWith("surface"));

describe("the tokens", () => {
  it("name every colour this probe reasons about", () => {
    for (const name of TEXT) expect(tokens[name], name).toMatch(/^#[0-9a-fA-F]{6}$/);
    expect(SURFACES.length).toBeGreaterThan(0);
  });

  it("read AA (4.5:1) for every text colour on every surface", () => {
    const failing: string[] = [];
    for (const text of TEXT) {
      for (const surface of SURFACES) {
        const ratio = contrast(tokens[text] ?? "#000000", tokens[surface] ?? "#000000");
        if (ratio < 4.5) failing.push(`${text} on ${surface}: ${ratio.toFixed(2)}`);
      }
    }
    expect(failing).toEqual([]);
  });
});

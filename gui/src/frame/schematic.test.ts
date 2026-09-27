import type { SerializedDockview } from "dockview-core";
import { describe, expect, it } from "vitest";

import { VIEWS } from "../panels/views";

import { schematic } from "./schematic";

/** A leaf of the serialized grid: one group with its panels. */
function leaf(size: number, ...views: string[]) {
  return { type: "leaf" as const, size, data: { views, activeView: views[0], id: views.join("+") } };
}

/**
 * ⛔ A LAYOUT WRITTEN BY HAND, with sizes chosen so the fractions are exact: the root is VERTICAL -- `a` above, a branch
 * below -- and the level under it is HORIZONTAL, `b` a quarter and `c` three quarters. And a floating group, which a
 * miniature does not draw (D5 of the plan).
 */
const HAND = {
  grid: {
    orientation: "VERTICAL",
    width: 800,
    height: 600,
    root: { type: "branch", size: 800, data: [leaf(30, "a"), { type: "branch", size: 70, data: [leaf(1, "b"), leaf(3, "c", "d")] }] },
  },
  panels: {},
  floatingGroups: [{ data: { views: ["floating"], id: "f" }, position: { left: 0, top: 0, width: 100, height: 100 } }],
} as unknown as SerializedDockview;

describe("schematic (answer 19 of the design system)", () => {
  it("cuts the unit square along the tree, the orientation alternating at every level, and leaves the floating out", () => {
    expect(schematic(HAND)).toEqual([
      { x: 0, y: 0, width: 1, height: 0.3, views: ["a"], active: "a" },
      { x: 0, y: 0.3, width: 0.25, height: 0.7, views: ["b"], active: "b" },
      { x: 0.25, y: 0.3, width: 0.75, height: 0.7, views: ["c", "d"], active: "c" },
    ]);
  });

  it("tiles each view that ships without a gap or an overlap, the strip across the bottom", () => {
    for (const [name, view] of Object.entries(VIEWS)) {
      const tiles = schematic(view);
      // ⛔ NON-VACUITY: every view has panels, so every view has tiles.
      expect(tiles.length, name).toBeGreaterThan(1);
      const area = tiles.reduce((sum, tile) => sum + tile.width * tile.height, 0);
      expect(Math.abs(area - 1), `${name}: the areas add up to the square`).toBeLessThan(1e-9);
      for (const tile of tiles) {
        expect(tile.x >= 0 && tile.y >= 0 && tile.x + tile.width <= 1 + 1e-9 && tile.y + tile.height <= 1 + 1e-9, `${name}: ${tile.views}`).toBe(true);
      }
      const strip = tiles.find((tile) => tile.views.includes("strip"));
      expect(strip && strip.x === 0 && Math.abs(strip.width - 1) < 1e-9 && Math.abs(strip.y + strip.height - 1) < 1e-9, `${name}: the strip`).toBe(true);
    }
  });
});

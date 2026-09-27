import { describe, expect, it } from "vitest";

import { nearest, type Box, type Direction } from "./nearest";

/** A card of a grid, as `nearest` sees it: a name for the probe, and a rectangle written by hand -- under jsdom every
 * rectangle is zero (P-97 of part 2). */
function card(name: string, left: number, top: number, width = 200, height = 120): { name: string; rect: Box } {
  return { name, rect: { left, top, right: left + width, bottom: top + height } };
}

/** Three columns and two rows, 24 px apart: the overview's grid, and the case R3-18 of the review measured. */
const GRID = [0, 1].flatMap((row) => [0, 1, 2].map((column) => card(`r${row}c${column}`, column * 224, row * 144)));

function from(name: string): Box {
  const found = GRID.find((candidate) => candidate.name === name);
  if (found === undefined) throw new Error(`no card ${name}`);
  return found.rect;
}

describe("nearest (decision 19 of the design system)", () => {
  it("goes to the card beyond, in each of the four directions", () => {
    const moves: [string, Direction, string][] = [
      ["r0c1", "right", "r0c2"],
      ["r0c1", "left", "r0c0"],
      ["r1c2", "up", "r0c2"],
      ["r0c0", "down", "r1c0"],
    ];
    for (const [start, direction, end] of moves) expect(nearest(from(start), GRID, direction)?.name, `${start} ${direction}`).toBe(end);
  });

  it("breaks a tie on the other axis: down from the middle column is the middle card below (R3-18)", () => {
    // ⛔ EVERY CARD OF THE ROW BELOW IS EQUALLY FAR on the vertical axis: the gap alone sends "down" to the first column.
    expect(nearest(from("r0c1"), GRID, "down")?.name).toBe("r1c1");
    expect(nearest(from("r0c2"), GRID, "down")?.name).toBe("r1c2");
    expect(nearest(from("r1c2"), GRID, "up")?.name).toBe("r0c2");
  });

  it("finds nothing beyond an edge, and never the card it starts from", () => {
    expect(nearest(from("r0c0"), GRID, "up")).toBeUndefined();
    expect(nearest(from("r0c2"), GRID, "right")).toBeUndefined();
  });
});

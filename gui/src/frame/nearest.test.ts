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

  it("breaks a tie on the other axis horizontally too: across from the second row stays on the second row (AUD-2196 of the audit of 2026-09-30)", () => {
    // ⛔ THE ROW ABOVE COMES FIRST IN `GRID`, so the first card at the smallest gap is the row above's: only the tie-break on
    // the vertical centre keeps the move on its row. Every move of the probes above found its answer first in `GRID`'s order,
    // and the horizontal half of the tie-break could go without one going red.
    expect(nearest(from("r1c1"), GRID, "right")?.name).toBe("r1c2");
    expect(nearest(from("r1c1"), GRID, "left")?.name).toBe("r1c0");
  });

  it("finds nothing beyond an edge", () => {
    expect(nearest(from("r0c0"), GRID, "up")).toBeUndefined();
    expect(nearest(from("r0c2"), GRID, "right")).toBeUndefined();
  });

  it("counts as beyond a neighbour that crosses the edge by less than a pixel, and not one that crosses it by more (AUD-2197 of the audit of 2026-09-30)", () => {
    const start: Box = { left: 0, top: 0, right: 600, bottom: 200 };
    const touching = (overlap: number): { name: string; rect: Box }[] => [
      { name: "right", rect: { left: 600 - overlap, top: 0, right: 800, bottom: 200 } },
      { name: "left", rect: { left: -200, top: 0, right: overlap, bottom: 200 } },
      { name: "down", rect: { left: 0, top: 200 - overlap, right: 600, bottom: 400 } },
      { name: "up", rect: { left: 0, top: -200, right: 600, bottom: overlap } },
    ];
    const directions: Direction[] = ["right", "left", "down", "up"];
    // ⛔ BOTH SIDES OF THE PIXEL, in all four directions: under it the neighbour is found, over it it is not.
    expect(directions.map((direction) => nearest(start, touching(0.4), direction)?.name)).toEqual(directions);
    expect(directions.map((direction) => nearest(start, touching(1.5), direction))).toEqual([undefined, undefined, undefined, undefined]);
  });

  it("takes the card it starts from when that card is no larger than the slack: the caller leaves it out (AUD-2117 of the audit of 2026-09-30)", () => {
    // ⛔ THE LIMIT THE CONTRACT EXISTS FOR, PINNED: a zero rectangle -- every one under jsdom -- lies beyond itself in every
    // direction. `moveActive` and the overview leave `from` out by identity, and `nearest` cannot.
    const zero = card("self", 0, 0, 0, 0);
    const directions: Direction[] = ["left", "right", "up", "down"];
    expect(directions.map((direction) => nearest(zero.rect, [zero], direction)?.name)).toEqual(["self", "self", "self", "self"]);
    // ⛔ THE SECOND DIRECTION: a card of the grid's size is not beyond itself.
    const real = card("self", 0, 0);
    expect(directions.map((direction) => nearest(real.rect, [real], direction))).toEqual([undefined, undefined, undefined, undefined]);
  });
});

import { afterEach, expect, it } from "vitest";

import { computed, concentricRadii } from "./probes";

// ⛔ THE RADIUS PROBE ON BOXES DRAWN BY HAND (E30 and E33 of the design-system plan): the kit page gives every piece four
// equal corners and puts it IN its corner, so two things would stay unproven there -- that each corner is read with its
// own radius, and the rule OFF the corner; and, from task 6, two more -- the radius a corner is DRAWN with (E46), and
// what reaches a corner through a box that scrolls (E44). ⛔ NON-VACUITY (trap 1): every case says the WHOLE report,
// `near` included, so a probe that met no corner cannot pass.

afterEach(() => document.body.replaceChildren());

function box(className: string, css: string, parent: Element): HTMLElement {
  const element = document.createElement("div");
  element.className = className;
  element.style.cssText = css;
  parent.append(element);
  return element;
}

/** The outer box: 300 × 120, every corner rounded 20. */
const outer = (): HTMLElement =>
  box("outer", "position:absolute;left:0;top:0;width:300px;height:120px;border-radius:20px", document.body);

it("judges the rounded corner of a piece whose top-left is straight (E30)", () => {
  const root = outer();
  // Only the bottom-left is rounded, 30, in the outer corner of 20 at 0/0, where it should be 20.
  box("piece", "position:absolute;left:0;bottom:0;width:120px;height:60px;border-radius:0 0 0 30px", root);
  expect(concentricRadii([root])).toEqual({
    near: 1,
    bad: ["piece in outer, bottom-left: radius 30.0, outer 20.0, distance 0.0/0.0"],
  });
});

it("never compares a straight corner -- a sheet's r r 0 0 (answer 20, E30)", () => {
  const root = outer();
  // The sheet's straight bottom corners sit 12/12 from the outer ones; the piece in the top-left corner is the one judged.
  box("sheet", "position:absolute;left:12px;right:12px;bottom:12px;height:40px;border-radius:16px 16px 0 0", root);
  box("piece", "position:absolute;left:12px;top:12px;width:60px;height:30px;border-radius:8px", root);
  expect(concentricRadii([root])).toEqual({ near: 1, bad: [] });
});

it("off the corner, refuses a piece rounder than the outer radius minus the smaller distance (E33)", () => {
  const root = outer();
  // 17/13 from the bottom-left corner: at most 20 - 13, with the probe's half pixel of slack (E61).
  box("piece", "position:absolute;left:17px;bottom:13px;width:120px;height:40px;border-radius:16px", root);
  expect(concentricRadii([root])).toEqual({
    near: 1,
    bad: ["piece in outer, bottom-left: radius 16.0, outer 20.0, distance 17.0/13.0"],
  });
});

it("off the corner, lets a smaller radius be (E33)", () => {
  const root = outer();
  // At most 20 - 13, and the bound itself passes. It was 8 here, green only by the old slack of 1.5 (E61).
  box("piece", "position:absolute;left:17px;bottom:13px;width:120px;height:40px;border-radius:7px", root);
  expect(concentricRadii([root])).toEqual({ near: 1, bad: [] });
});

it("reads the radius a corner is DRAWN with: a short bar's r r 0 0 keeps it (E46)", () => {
  const root = outer();
  // 24 px tall, `19px 19px 0 0`, 1/1 inside the top corners: each side's two radii fit in the side, so CSS draws 19,
  // which is what the outer corner wants. Halving each corner on its own read 12.
  box("bar", "position:absolute;left:1px;right:1px;top:1px;height:24px;border-radius:19px 19px 0 0", root);
  expect(concentricRadii([root])).toEqual({ near: 2, bad: [] });
});

it("sees the pixel of a border: the same bar at the outer radius is one pixel too round (E61)", () => {
  const root = outer();
  // What the dock's floating title bar was: the card's radius, 1 px inside the card's edge. The old slack of 1.5 kept it
  // green; half a pixel is under the smallest defect the rule wants to see.
  box("bar", "position:absolute;left:1px;right:1px;top:1px;height:24px;border-radius:20px 20px 0 0", root);
  expect(concentricRadii([root])).toEqual({
    near: 2,
    bad: [
      "bar in outer, top-left: radius 20.0, outer 20.0, distance 1.0/1.0",
      "bar in outer, top-right: radius 20.0, outer 20.0, distance 1.0/1.0",
    ],
  });
});

it("still shrinks the radii where CSS does: the same bar with four corners (E46)", () => {
  const root = outer();
  // Four corners of 20 in 24 px: each side would hold 40, CSS draws 12 -- and 12 is not 19.
  box("pill", "position:absolute;left:1px;right:1px;top:1px;height:24px;border-radius:20px", root);
  expect(concentricRadii([root])).toEqual({
    near: 2,
    bad: [
      "pill in outer, top-left: radius 12.0, outer 20.0, distance 1.0/1.0",
      "pill in outer, top-right: radius 12.0, outer 20.0, distance 1.0/1.0",
    ],
  });
});

it("does not judge what reaches the corner through a box that scrolls, and judges it while the box does not (E44)", () => {
  const root = outer();
  const scroller = box("scroller", "position:absolute;inset:0;overflow:auto", root);
  // Too round for the corner it sits in: 15 drawn, 20 wanted.
  box("piece", "position:absolute;left:0;bottom:0;width:60px;height:30px;border-radius:30px", scroller);
  expect(concentricRadii([root])).toEqual({
    near: 1,
    bad: ["piece in outer, bottom-left: radius 15.0, outer 20.0, distance 0.0/0.0"],
  });
  // The same piece in the same place, once the box holds more than it shows: where it lands is the scroll's.
  box("filler", "position:absolute;left:0;top:0;width:1px;height:400px", scroller);
  expect(concentricRadii([root])).toEqual({ near: 0, bad: [] });
});

// ⛔ THE ORACLE OF A TOKEN ANSWERS ONLY FOR A TOKEN THE PAGE DEFINES (E108 of the design-system plan), as `readToken` does:
// a misspelt one computed the property's initial value -- a plausible oracle, and a green that looked at nothing.
it("refuses a token the page does not define, and computes one it does (E108)", () => {
  expect(() => computed("color", "--color-txet")).toThrow(/not defined/);
  document.documentElement.style.setProperty("--probe-colour", "rgb(1, 2, 3)");
  try {
    expect(computed("color", "--probe-colour")).toBe("rgb(1, 2, 3)");
  } finally {
    document.documentElement.style.removeProperty("--probe-colour");
  }
});

import { afterEach, expect, it } from "vitest";

import { concentricRadii } from "./probes";

// ⛔ THE RADIUS PROBE ON BOXES DRAWN BY HAND (E30 and E33 of the design-system plan): the kit page gives every piece four
// equal corners and puts it IN its corner, so two things would stay unproven there -- that each corner is read with its
// own radius, and the rule OFF the corner. ⛔ NON-VACUITY (trap 1): every case says the WHOLE report, `near` included,
// so a probe that met no corner cannot pass.

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
  // 17/13 from the bottom-left corner: at most 20 - 13, with the probe's 1.5 px of slack.
  box("piece", "position:absolute;left:17px;bottom:13px;width:120px;height:40px;border-radius:16px", root);
  expect(concentricRadii([root])).toEqual({
    near: 1,
    bad: ["piece in outer, bottom-left: radius 16.0, outer 20.0, distance 17.0/13.0"],
  });
});

it("off the corner, lets a smaller radius be (E33)", () => {
  const root = outer();
  box("piece", "position:absolute;left:17px;bottom:13px;width:120px;height:40px;border-radius:8px", root);
  expect(concentricRadii([root])).toEqual({ near: 1, bad: [] });
});

import "dockview/dist/styles/dockview.css";
import "../tokens";

import type { DockviewApi } from "dockview-core";
import { createPinia, setActivePinia } from "pinia";
import { afterEach, beforeEach, describe, expect, it } from "vitest";

import { registerModules } from "../panels/modules";
import { concentricRadii } from "../testing/probes";
import { readToken } from "../tokens/readToken";

import { createDock } from "./dock";

// ⛔ THE DRESSED DOCK IN THE INSTALLED CHROME (design system, section (c)): what only a layout engine can judge -- the
// space between the cards, their radius and surface, the grab's height, the level and the surface of a floating group.
// The stylesheets are the SPA's own, in the order `main.ts` loads them: `dockview.css` first, our tokens after it.

const docks: DockviewApi[] = [];

beforeEach(() => {
  setActivePinia(createPinia());
  registerModules();
});

afterEach(() => {
  // ⛔ DISPOSED, NOT ONLY DETACHED: `dockview-core` 8.3.1 stacks the floating groups of the whole PAGE in one module-level
  // list, `+ 2` per group, and a group left there lifts the next test's to 52 (measured on 2026-09-23).
  for (const api of docks.splice(0)) api.dispose();
  document.body.replaceChildren();
  delete document.documentElement.dataset.theme;
});

/** The dock of the Home view in one theme, on a host of the boards' size, once `dockview` has laid it out. */
async function dock(theme: "light" | "dark") {
  document.documentElement.dataset.theme = theme;
  const host = document.createElement("div");
  host.style.cssText = "width:1400px;height:800px";
  document.body.append(host);
  const api = createDock(host);
  docks.push(api);
  await new Promise((resolve) => setTimeout(resolve, 50));
  return { host, api };
}

/** The Status panel floated where the grab's first command puts it, and its container once laid out. */
async function floatStatus(host: HTMLElement, api: DockviewApi): Promise<Element> {
  const panel = api.getPanel("status");
  expect(panel).toBeDefined();
  if (panel !== undefined) api.addFloatingGroup(panel, { x: 60, y: 60, width: 460, height: 320 });
  await new Promise((resolve) => setTimeout(resolve, 50));
  const floating = [...host.querySelectorAll(".dv-resize-container")];
  expect(floating).toHaveLength(1);
  return floating[0] as Element;
}

/** What a declaration of `property: var(token)` computes to here: the oracle of a role, with no copy of its value. */
function computed(property: string, token: string): string {
  const probe = document.createElement("div");
  probe.style.setProperty(property, `var(${token})`);
  document.body.append(probe);
  const value = getComputedStyle(probe).getPropertyValue(property);
  probe.remove();
  return value;
}

for (const theme of ["light", "dark"] as const) {
  describe(`the dressed dock, ${theme} theme`, () => {
    it("draws every group as a card, `--space-3` from its neighbours", async () => {
      const { host } = await dock(theme);
      const groups = [...host.querySelectorAll(".dv-groupview")];
      // ⛔ NON-VACUITY: the Home view has groups side by side and one above the other.
      expect(groups.length).toBeGreaterThan(2);
      for (const group of groups) {
        const style = getComputedStyle(group);
        expect(style.borderTopLeftRadius).toBe(computed("border-top-left-radius", "--radius-card"));
        expect(style.backgroundColor).toBe(computed("background-color", "--color-bg-surface"));
      }
      // The nearest neighbour on the right and below, where the two overlap: the distance between the facing edges.
      const boxes = groups.map((group) => group.getBoundingClientRect());
      const gaps: number[] = [];
      for (const a of boxes) {
        const right = boxes.filter((b) => b.left >= a.right && Math.min(a.bottom, b.bottom) > Math.max(a.top, b.top));
        const below = boxes.filter((b) => b.top >= a.bottom && Math.min(a.right, b.right) > Math.max(a.left, b.left));
        if (right.length > 0) gaps.push(Math.min(...right.map((b) => b.left - a.right)));
        if (below.length > 0) gaps.push(Math.min(...below.map((b) => b.top - a.bottom)));
      }
      expect(gaps.length).toBeGreaterThan(1);
      const space = Number.parseFloat(computed("width", "--space-3"));
      expect(gaps.filter((gap) => Math.abs(gap - space) > 0.5)).toEqual([]);
    });

    it("keeps every radius of its own concentric, a floating group's too (answer 4)", async () => {
      const { host, api } = await dock(theme);
      const floating = await floatStatus(host, api);
      // A card too, with the tokens' radius -- not the 8px the spaced themes write under their own class (the (c)).
      expect(getComputedStyle(floating).borderTopLeftRadius).toBe(computed("border-top-left-radius", "--radius-card"));
      // ⛔ NON-VACUITY ON THE CASE THAT JUDGES (trap 1, E29): in the Home view nothing the dock draws sits near a corner,
      // and what did -- the dot in a radio, a radio scrolled to the card's edge -- is not the dock's (E44). The floating
      // container is: its title bar in the two top corners, its group in the two bottom ones (E30, E46).
      const own = concentricRadii([floating]);
      expect(own.near).toBeGreaterThanOrEqual(4);
      expect(own.bad).toEqual([]);
      expect(concentricRadii([host]).bad).toEqual([]);
    });

    it("keeps the grab as tall as its strip, `--size-control-lg` (move 5 of SP-8, E45)", async () => {
      const { host } = await dock(theme);
      const grabs = [...host.querySelectorAll(".dv-tabs-container > .dv-tab .bigtab")].filter(
        (grab) => grab.getBoundingClientRect().width > 0,
      );
      // ⛔ NON-VACUITY: the Home view shows the tabs of five of its groups.
      expect(grabs.length).toBeGreaterThan(0);
      const tall = Number.parseFloat(computed("height", "--size-control-lg"));
      const heights = grabs.map((grab) => grab.getBoundingClientRect().height);
      expect(heights.filter((height) => Math.abs(height - tall) > 0.5)).toEqual([]);
    });

    it("floats a group BELOW the dialogs: at `--z-floating`, under `--z-overlay` (trap 9, R3-17)", async () => {
      const { host, api } = await dock(theme);
      const floating = await floatStatus(host, api);
      const level = Number(getComputedStyle(floating).zIndex);
      expect(level).toBe(Number(readToken("--z-floating")));
      expect(level).toBeLessThan(Number(readToken("--z-overlay")));
      // And raised, its tab too (the (c), E47): a tab left on the surface draws a darker block on the raised header.
      const raised = computed("background-color", "--color-bg-raised");
      for (const part of [".dv-tabs-and-actions-container", ".dv-tab"]) {
        const element = floating.querySelector(part);
        expect(element).not.toBeNull();
        expect(getComputedStyle(element as Element).backgroundColor).toBe(raised);
      }
    });
  });
}

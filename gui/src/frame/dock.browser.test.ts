import "dockview/dist/styles/dockview.css";
import "../tokens";

import type { DockviewApi } from "dockview-core";
import { createPinia, setActivePinia } from "pinia";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { userEvent } from "vitest/browser";

import { registerModules } from "../panels/modules";
import { VIEWS } from "../panels/views";
import { computed, concentricRadii } from "../testing/probes";
import { readToken } from "../tokens/readToken";

import { canonical, createDock, type Dock } from "./dock";
import { moveActive } from "./moveActive";
import type { Direction } from "./nearest";

// ⛔ THE DRESSED DOCK IN THE INSTALLED CHROME (design system, section (c)): what only a layout engine can judge -- the
// space between the cards, their radius and surface, the grab's height, the level and the surface of a floating group.
// The stylesheets are the SPA's own, in the order `main.ts` loads them: `dockview.css` first, our tokens after it.

const docks: Dock[] = [];

beforeEach(() => {
  setActivePinia(createPinia());
  registerModules();
});

afterEach(() => {
  // ⛔ DISPOSED, NOT ONLY DETACHED: `dockview-core` 8.3.1 stacks the floating groups of the whole PAGE in one module-level
  // list, `+ 2` per group, and a group left there lifts the next test's to 52 (measured on 2026-09-23). And `dispose` of
  // the dock takes its listeners off `window` too (AUD-2116 of the audit of 2026-09-30).
  for (const dock of docks.splice(0)) dock.dispose();
  document.body.replaceChildren();
  delete document.documentElement.dataset.theme;
});

/** The dock of the Home view in one theme, on a host of the boards' size, once `dockview` has laid it out. */
async function dock(theme: "light" | "dark") {
  document.documentElement.dataset.theme = theme;
  const host = document.createElement("div");
  host.style.cssText = "width:1400px;height:800px";
  document.body.append(host);
  const dock = createDock(host);
  docks.push(dock);
  await new Promise((resolve) => setTimeout(resolve, 50));
  return { host, api: dock.api };
}

/** The Status panel floated where the grab's first command puts it, and its container once laid out. */
async function floatStatus(host: HTMLElement, api: DockviewApi): Promise<Element> {
  const panel = api.getPanel("status");
  expect(panel).toBeDefined();
  if (panel !== undefined) api.addFloatingGroup(panel, { x: 60, y: 60, width: 460, height: 320 });
  const floating = [...host.querySelectorAll(".dv-resize-container")];
  expect(floating).toHaveLength(1);
  const container = floating[0] as HTMLElement;
  // ⛔ LAID OUT, NOT 50 MS LATER (E130 of the design-system plan): `dockview-core` 8.3.1 sizes a floating group from a
  // ResizeObserver whose callback waits for an animation frame, and until then the group is 100 px wide, its minimum --
  // three corners near instead of four. A fixed wait counted on frames that a busy suite does not always draw: the one
  // red of five runs drew none in its 63 ms, measured on 2026-09-28. A layout that never comes stays red, as
  // `expected 100 to be 460`.
  await expect
    .poll(() => container.querySelector(".dv-groupview")?.getBoundingClientRect().width)
    .toBe(container.clientWidth);
  return container;
}

for (const theme of ["light", "dark"] as const) {
  describe(`the dressed dock, ${theme} theme`, () => {
    it("draws every group as a card, and the strip as a pill, `--space-3` from its neighbours", async () => {
      const { host } = await dock(theme);
      const groups = [...host.querySelectorAll(".dv-groupview")];
      // ⛔ NON-VACUITY: the Home view has groups side by side and one above the other.
      expect(groups.length).toBeGreaterThan(2);
      // ⛔ THE STRIP IS THE ONE PILL (the (d), task 8 of the plan): its group holds `Strip.vue`, and every other is a card.
      const strips = groups.filter((group) => group.querySelector(".strip") !== null);
      expect(strips).toHaveLength(1);
      for (const group of groups) {
        const style = getComputedStyle(group);
        const radius = strips.includes(group) ? "--radius-full" : "--radius-card";
        expect(style.borderTopLeftRadius).toBe(computed("border-top-left-radius", radius));
        // ⛔ THE NUCLEUS IS NOT A CARD (E58): the page shows through it, and its test is its own, below.
        if (group.querySelector(".nucleus") !== null) continue;
        expect(style.backgroundColor).toBe(computed("background-color", "--color-bg-surface"));
        // The border the dark theme draws, the shadow the light one casts (E52): tokens that are transparent or `none`
        // in the other theme, so each bites in its own.
        expect(style.borderTopColor).toBe(computed("border-top-color", "--color-border-card"));
        expect(style.boxShadow).toBe(computed("box-shadow", "--shadow-card"));
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

    it("paints nothing between the cards: the page shows through (E56)", async () => {
      const { host } = await dock(theme);
      // ⛔ FROM EVERY CARD UP TO THE HOST, NOTHING PAINTS (the owner at step 8 of task 6): `dockview.css` 8.3.1 paints the
      // whole grid in the cards' surface, a second background under them and around them, where the light cards sink.
      const painted = new Set<string>();
      let seen = 0;
      for (const group of [...host.querySelectorAll(".dv-groupview")]) {
        for (let element = group.parentElement; element !== null && element !== host; element = element.parentElement) {
          seen += 1;
          const { backgroundColor } = getComputedStyle(element);
          if (backgroundColor !== "rgba(0, 0, 0, 0)") painted.add(`${element.className}: ${backgroundColor}`);
        }
      }
      // ⛔ NON-VACUITY: the layers between the cards and the host were walked.
      expect(seen).toBeGreaterThan(0);
      expect([...painted]).toEqual([]);
    });

    it("draws the nucleus on the page, not as a card (E58)", async () => {
      const { host } = await dock(theme);
      const groups = [...host.querySelectorAll(".dv-groupview")];
      const nuclei = groups.filter((group) => group.querySelector(".nucleus") !== null);
      // ⛔ NON-VACUITY: the Home view has its nucleus, and one.
      expect(nuclei).toHaveLength(1);
      const style = getComputedStyle(nuclei[0] as Element);
      expect(style.backgroundColor).toBe("rgba(0, 0, 0, 0)");
      expect(style.borderTopColor).toBe("rgba(0, 0, 0, 0)");
      expect(style.boxShadow).toBe("none");
    });

    it("rounds the drop zone like the cards, a tab dragged for real (E55)", async () => {
      const { host, api } = await dock(theme);
      // ⛔ THE DROP ZONE LIVES ONLY WHILE A TAB IS DRAGGED, so an observer reads it the moment `dockview` draws it: with
      // the default mounting of 8.3.1 it is `.dv-drop-target-selection`, inside the target group's content container --
      // measured with a real mouse on 2026-09-26, and NOT the `.dv-drop-target-anchor` of the absolute mounting.
      const radii = new Set<string>();
      const observer = new MutationObserver(() => {
        for (const selection of host.querySelectorAll(".dv-drop-target-selection")) {
          radii.add(getComputedStyle(selection).borderTopLeftRadius);
        }
      });
      observer.observe(host, { childList: true, subtree: true, attributes: true });
      const tab = api.getPanel("permissions")?.group.element.querySelector(".dv-tabs-container > .dv-tab");
      const target = api.getPanel("activity")?.group.element.querySelector(".dv-content-container");
      expect(tab).toBeInstanceOf(HTMLElement);
      expect(target).toBeInstanceOf(HTMLElement);
      // ⛔ IN STEPS: with `dndStrategy: "pointer"` a drag starts past a threshold, and one jump from tab to target never
      // draws the zone -- measured, the guard below red. Playwright 1.63 interpolates the moves, and `vitest` hands it
      // the option.
      await userEvent.dragAndDrop(tab as HTMLElement, target as HTMLElement, { steps: 12 });
      observer.disconnect();
      // ⛔ NON-VACUITY: the drag drew a drop zone.
      expect(radii.size).toBeGreaterThan(0);
      // Concentric with the card it sits in: one border inside the card's edge (answer 4).
      const card = Number.parseFloat(computed("border-top-left-radius", "--radius-card"));
      // `width`, not `border-top-width`: a border with no style computes to 0, whatever its width says.
      const border = Number.parseFloat(computed("width", "--border-width"));
      expect([...radii]).toEqual([`${card - border}px`]);
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

    it("keeps an always-rendered panel's content with its floating group, under the dialogs (E51)", async () => {
      const { host, api } = await dock(theme);
      // ⛔ A PANEL THE SPA DOES NOT HAVE YET, BROUGHT BY THE TEST: `renderer: "always"` keeps its content in an overlay of
      // its own, which `dockview.css` 8.3.1 gives the floating container's cycle (E51).
      api.addPanel({
        id: "always",
        component: "knowledge",
        renderer: "always",
        floating: { x: 100, y: 100, width: 400, height: 300 },
      });
      await new Promise((resolve) => setTimeout(resolve, 50));
      const overlays = [...host.querySelectorAll(".dv-render-overlay")];
      // ⛔ NON-VACUITY: the one panel that is rendered always.
      expect(overlays).toHaveLength(1);
      const level = Number(getComputedStyle(overlays[0] as Element).zIndex);
      expect(level).toBe(Number(readToken("--z-floating")) + 1);
      expect(level).toBeLessThan(Number(readToken("--z-overlay")));
    });
  });
}

/** The layout on screen in the form `dock.ts` compares: key order is not layout. */
function layoutOf(api: DockviewApi): string {
  return JSON.stringify(canonical(api.toJSON()));
}

/** Home again, as it ships, once `dockview` has laid it out: every probe below starts from it. */
async function home(api: DockviewApi): Promise<void> {
  api.fromJSON(VIEWS.home);
  await new Promise((resolve) => setTimeout(resolve, 50));
}

const DIRECTIONS: readonly Direction[] = ["left", "right", "up", "down"];

// ⛔ MOVE 6 OF SP-8 WITH THE LIBRARY THAT DOES THE MOVING (AUD-721 of the audit of 2026-09-30): `keys.test.ts` hands
// `moveActive` a dock of its own, which records the `moveTo` and cannot see what `dockview-core` 8.3.1 does with it -- and
// under jsdom every rectangle is zero. Here the geometry is Chrome's and the grid is `dockview`'s.
describe("the tile moved with the keyboard, in the installed Chrome (move 6 of SP-8)", () => {
  it("moves a tile alone in its group into the nearest unlocked group that way, past the nucleus, and leaves it where it is where nothing lies", async () => {
    const { api } = await dock("light");
    const group = (id: string) => api.getPanel(id)?.group;
    // Stato is alone in its group, at the top of the left column: Permessi lies below it, Attività's column right of it,
    // past the nucleus -- locked, so not a target -- and nothing left of it or above it.
    const moves: [Direction, string | null][] = [
      ["down", "permissions"],
      ["right", "activity"],
      ["left", null],
      ["up", null],
    ];
    for (const [direction, into] of moves) {
      await home(api);
      api.getPanel("status")?.api.setActive();
      const before = layoutOf(api);
      const groups = api.groups.length;
      if (into !== null) {
        expect(moveActive(api, direction), direction).toBe("moved");
        expect(group("status"), direction).toBe(group(into));
        expect(api.groups.length, direction).toBe(groups - 1);
        continue;
      }
      // ⛔ NOTHING THAT WAY, AND NOTHING TO SPLIT: a tile alone in its group is already on that side of it. In
      // `dockview-core` 8.3.1 a `moveTo` onto its own side either puts the group back where it was or takes it out of the
      // grid, and the return said "split" in both.
      expect(moveActive(api, direction), direction).toBe("none");
      expect(layoutOf(api), direction).toBe(before);
      expect(api.groups.length, direction).toBe(groups);
    }
  });

  it("splits a group of two on that side when nothing lies that way, in the four directions", async () => {
    const { api } = await dock("light");
    // An edge of Home for each direction, and a second tile brought into that group: the left column's top for left and
    // up, Attività's column's top for right, the left column's bottom -- above the strip, which is locked -- for down.
    const edges: Record<Direction, [string, string]> = {
      left: ["status", "permissions"],
      up: ["status", "permissions"],
      right: ["activity", "costs"],
      down: ["settings", "permissions"],
    };
    for (const direction of DIRECTIONS) {
      await home(api);
      const [stays, moves] = edges[direction];
      const edge = api.getPanel(stays)?.group;
      expect(edge, direction).toBeDefined();
      if (edge === undefined) continue;
      api.getPanel(moves)?.api.moveTo({ group: edge, position: "center" });
      await new Promise((resolve) => setTimeout(resolve, 50));
      // ⛔ NON-VACUITY: a group of two, the tile brought in is the active one.
      expect(edge.size, direction).toBe(2);
      expect(api.activePanel?.id, direction).toBe(moves);
      expect(moveActive(api, direction), direction).toBe("split");
      await new Promise((resolve) => setTimeout(resolve, 50));
      const split = api.getPanel(moves)?.group;
      expect(split, direction).toBeDefined();
      expect(split, direction).not.toBe(edge);
      expect(edge.panels.map((panel) => panel.id), direction).toEqual([stays]);
      // On that side of the group it left, and across from it.
      const from = edge.element.getBoundingClientRect();
      const to = (split as NonNullable<typeof split>).element.getBoundingClientRect();
      const beyond =
        direction === "left" ? to.right <= from.left + 1
        : direction === "right" ? to.left >= from.right - 1
        : direction === "up" ? to.bottom <= from.top + 1
        : to.top >= from.bottom - 1;
      expect(beyond, `${direction}: ${JSON.stringify(to)} from ${JSON.stringify(from)}`).toBe(true);
    }
  });

  it("never moves the nucleus or the strip, not even as the active panel (move 1, AUD-537 and AUD-538 of the audit of 2026-09-30)", async () => {
    const { host, api } = await dock("light");
    // ⛔ THE STRIP BECOMES THE ACTIVE PANEL BY ITS OWN BUTTON: «+ moduli» takes the focus with Tab, and `dockview` makes the
    // group whose content holds the focus the active one.
    const button = host.querySelector<HTMLElement>(".strip .base-button");
    expect(button).not.toBeNull();
    button?.focus();
    // ⛔ NON-VACUITY: the road the keyboard takes does make the strip active.
    expect(api.activePanel?.id).toBe("strip");
    const actives: [string, () => void][] = [
      ["strip", () => button?.focus()],
      ["knowledge", () => api.getPanel("knowledge")?.api.setActive()],
      // ⛔ COMPATTA OPENS WITH THE STRIP ACTIVE, and its two groups are both locked: there is nothing to move in it.
      ["strip", () => api.fromJSON(VIEWS.compact)],
    ];
    for (const [id, activate] of actives) {
      activate();
      await new Promise((resolve) => setTimeout(resolve, 50));
      expect(api.activePanel?.id).toBe(id);
      expect(api.activePanel?.group.locked).toBe(true);
      const before = layoutOf(api);
      for (const direction of DIRECTIONS) {
        expect(moveActive(api, direction), `${id} ${direction}`).toBe("none");
        expect(layoutOf(api), `${id} ${direction}`).toBe(before);
      }
    }
  });
});

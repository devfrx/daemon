import type { DockviewApi } from "dockview-core";
import { describe, expect, it } from "vitest";

import { directionOf, moveActive } from "./moveActive";

/** A group as `moveActive` sees it: a rectangle, a lock, how many tiles it holds, an element. One tile, as in every
 * group of the views that ship. */
function group(rect: { left: number; top: number; width: number; height: number }, { locked = false, size = 1 } = {}) {
  const full = { ...rect, right: rect.left + rect.width, bottom: rect.top + rect.height } as DOMRect;
  return { locked, size, element: { getBoundingClientRect: () => full } };
}

/** The members `moveActive` touches, and the `moveTo` it calls -- recorded. */
function dock(active: ReturnType<typeof group>, others: ReturnType<typeof group>[]) {
  const moves: unknown[] = [];
  const panel = { group: active, api: { moveTo: (options: unknown) => moves.push(options) } };
  const api = { activePanel: panel, groups: [active, ...others] } as unknown as DockviewApi;
  return { api, moves };
}

describe("moveActive", () => {
  it("moves into the NEAREST unlocked group in that direction", () => {
    const active = group({ left: 400, top: 0, width: 200, height: 200 });
    const near = group({ left: 700, top: 0, width: 200, height: 200 });
    const far = group({ left: 1000, top: 0, width: 200, height: 200 });
    const lockedNearer = group({ left: 620, top: 0, width: 60, height: 200 }, { locked: true });
    const { api, moves } = dock(active, [far, near, lockedNearer]);
    expect(moveActive(api, "right")).toBe("moved");
    expect(moves).toEqual([{ group: near, position: "center" }]);
  });

  it("splits its own group on that side when nothing lies there, if the group holds another tile", () => {
    const active = group({ left: 400, top: 0, width: 200, height: 200 }, { size: 2 });
    const left = group({ left: 0, top: 0, width: 200, height: 200 });
    const { api, moves } = dock(active, [left]);
    expect(moveActive(api, "down")).toBe("split");
    expect(moves).toEqual([{ group: active, position: "bottom" }]);
  });

  it("leaves a tile alone in its group where it is when nothing lies there (AUD-721 of the audit of 2026-09-30)", () => {
    // ⛔ NOTHING TO SPLIT: `dockview-core` 8.3.1 handed a `moveTo` onto its own side puts the group back where it was, or
    // takes it out of the grid and throws -- measured in `dock.browser.test.ts`. The tile is already on that side.
    const active = group({ left: 400, top: 0, width: 200, height: 200 });
    const left = group({ left: 0, top: 0, width: 200, height: 200 });
    const { api, moves } = dock(active, [left]);
    expect(moveActive(api, "down")).toBe("none");
    expect(moves).toEqual([]);
    // ⛔ THE SECOND DIRECTION: with a group that lies that way, the tile still goes into it.
    expect(moveActive(api, "left")).toBe("moved");
    expect(moves).toEqual([{ group: left, position: "center" }]);
  });

  it("never moves a tile out of a locked group -- the nucleus, the strip -- as it never moves one into it (move 1, AUD-537 and AUD-538)", () => {
    const right = group({ left: 700, top: 0, width: 200, height: 200 });
    // ⛔ BOTH ROADS: with a group that way, where the tile would go into it, and with none, where a group of two would split.
    for (const others of [[right], []]) {
      const active = group({ left: 400, top: 0, width: 200, height: 200 }, { locked: true, size: 2 });
      const { api, moves } = dock(active, others);
      expect(moveActive(api, "right")).toBe("none");
      expect(moves).toEqual([]);
    }
    // ⛔ THE SECOND DIRECTION: the same tile in a group that is not locked moves, on both roads.
    const unlocked = (): ReturnType<typeof group> => group({ left: 400, top: 0, width: 200, height: 200 }, { size: 2 });
    expect(moveActive(dock(unlocked(), [right]).api, "right")).toBe("moved");
    expect(moveActive(dock(unlocked(), []).api, "right")).toBe("split");
  });

  it("does nothing without an active tile", () => {
    const api = { activePanel: undefined, groups: [] } as unknown as DockviewApi;
    expect(moveActive(api, "left")).toBe("none");
  });
});

/** The chord pressed on `target`, as a keyboard sends it: the event is dispatched there, so `event.target` is that element. */
function pressed(target: Element, key: string): KeyboardEvent {
  const event = new KeyboardEvent("keydown", { key, ctrlKey: true, altKey: true, bubbles: true });
  target.dispatchEvent(event);
  return event;
}

describe("directionOf", () => {
  it("maps Ctrl+Alt+Arrow, each arrow to its own direction, and nothing else", () => {
    // ⛔ ALL FOUR, AND PRESSED ON AN ELEMENT THAT IS NOT A FIELD -- the case of every real key, which reaches `Frame.vue`'s
    // listener with an element for target (AUD-2195 of the audit of 2026-09-30): two arrows of four, on events never
    // dispatched, left a swap of the other two and a guard that refused every target to pass.
    const button = document.createElement("button");
    document.body.append(button);
    const arrows = ["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown"];
    expect(arrows.map((key) => directionOf(pressed(button, key)))).toEqual(["left", "right", "up", "down"]);
    expect(arrows.map((key) => directionOf(pressed(document.body, key)))).toEqual(["left", "right", "up", "down"]);
    button.remove();
    expect(directionOf(new KeyboardEvent("keydown", { key: "ArrowLeft", ctrlKey: true }))).toBeNull();
    expect(directionOf(new KeyboardEvent("keydown", { key: "a", ctrlKey: true, altKey: true }))).toBeNull();
  });

  it("stays out of every field being typed in: an input, a select, a text area", () => {
    for (const tag of ["input", "select", "textarea"]) {
      const field = document.createElement(tag);
      document.body.append(field);
      expect(directionOf(pressed(field, "ArrowUp")), tag).toBeNull();
      field.remove();
    }
  });
});

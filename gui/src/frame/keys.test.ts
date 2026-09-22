import type { DockviewApi } from "dockview-core";
import { describe, expect, it } from "vitest";

import { directionOf, moveActive } from "./moveActive";

/** A group as `moveActive` sees it: a rectangle, a lock, an element. */
function group(rect: { left: number; top: number; width: number; height: number }, locked = false) {
  const full = { ...rect, right: rect.left + rect.width, bottom: rect.top + rect.height } as DOMRect;
  return { locked, element: { getBoundingClientRect: () => full } };
}

/** The four members `moveActive` touches, and the `moveTo` it calls -- recorded. */
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
    const lockedNearer = group({ left: 620, top: 0, width: 60, height: 200 }, true);
    const { api, moves } = dock(active, [far, near, lockedNearer]);
    expect(moveActive(api, "right")).toBe("moved");
    expect(moves).toEqual([{ group: near, position: "center" }]);
  });

  it("splits its own group on that side when nothing lies there", () => {
    const active = group({ left: 400, top: 0, width: 200, height: 200 });
    const left = group({ left: 0, top: 0, width: 200, height: 200 });
    const { api, moves } = dock(active, [left]);
    expect(moveActive(api, "down")).toBe("split");
    expect(moves).toEqual([{ group: active, position: "bottom" }]);
  });

  it("does nothing without an active tile", () => {
    const api = { activePanel: undefined, groups: [] } as unknown as DockviewApi;
    expect(moveActive(api, "left")).toBe("none");
  });
});

describe("directionOf", () => {
  it("maps Ctrl+Alt+Arrow, and nothing else", () => {
    expect(directionOf(new KeyboardEvent("keydown", { key: "ArrowLeft", ctrlKey: true, altKey: true }))).toBe("left");
    expect(directionOf(new KeyboardEvent("keydown", { key: "ArrowDown", ctrlKey: true, altKey: true }))).toBe("down");
    expect(directionOf(new KeyboardEvent("keydown", { key: "ArrowLeft", ctrlKey: true }))).toBeNull();
    expect(directionOf(new KeyboardEvent("keydown", { key: "a", ctrlKey: true, altKey: true }))).toBeNull();
  });

  it("stays out of a field being typed in", () => {
    const input = document.createElement("input");
    document.body.append(input);
    const event = new KeyboardEvent("keydown", { key: "ArrowUp", ctrlKey: true, altKey: true, bubbles: true });
    input.dispatchEvent(event);
    expect(directionOf(event)).toBeNull();
    input.remove();
  });
});

import type { DockviewApi, Position } from "dockview-core";

export type Direction = "left" | "right" | "up" | "down";
export type Moved = "moved" | "split" | "none";

/**
 * Move 6 of SP-8: the active tile goes into the nearest group in that direction; with no
 * neighbour there, it splits its own group on that side.
 *
 * ⛔ GEOMETRY, NOT `dockview`'S NAVIGATION API: it is what a keyboard user SEES, and the library's
 * spatial navigation is a paid feature (§4 of the north star). Locked groups -- the nucleus, the
 * strip -- are never a target: move 1.
 *
 * ⚠️ UNDER jsdom EVERY RECT IS ZERO (task 13, step 3), so in a jsdom probe every other group
 * counts as "beyond" in every direction: the probe of `keys.test.ts` hands rectangles of its own
 * instead, and the browser is where the reviewer sees the real thing (rule 5 of the head).
 */
export function moveActive(api: DockviewApi, direction: Direction): Moved {
  const panel = api.activePanel;
  if (panel === undefined) return "none";
  const from = panel.group.element.getBoundingClientRect();
  const beyond = (rect: DOMRect): boolean =>
    direction === "left" ? rect.right <= from.left + 1
    : direction === "right" ? rect.left >= from.right - 1
    : direction === "up" ? rect.bottom <= from.top + 1
    : rect.top >= from.bottom - 1;
  const gap = (rect: DOMRect): number =>
    direction === "left" ? from.left - rect.right
    : direction === "right" ? rect.left - from.right
    : direction === "up" ? from.top - rect.bottom
    : rect.top - from.bottom;
  const target = api.groups
    .filter((group) => group !== panel.group && !group.locked)
    .map((group) => ({ group, rect: group.element.getBoundingClientRect() }))
    .filter(({ rect }) => beyond(rect))
    .sort((a, b) => gap(a.rect) - gap(b.rect))[0];
  if (target !== undefined) {
    panel.api.moveTo({ group: target.group, position: "center" });
    return "moved";
  }
  const side: Position = direction === "up" ? "top" : direction === "down" ? "bottom" : direction;
  panel.api.moveTo({ group: panel.group, position: side });
  return "split";
}

/** Ctrl+Alt+Arrow, and nothing while typing in a field. */
export function directionOf(event: KeyboardEvent): Direction | null {
  if (!event.ctrlKey || !event.altKey) return null;
  const tag = (event.target as HTMLElement | null)?.tagName;
  if (tag === "INPUT" || tag === "SELECT" || tag === "TEXTAREA") return null;
  switch (event.key) {
    case "ArrowLeft":
      return "left";
    case "ArrowRight":
      return "right";
    case "ArrowUp":
      return "up";
    case "ArrowDown":
      return "down";
    default:
      return null;
  }
}

import type { DockviewApi, Position } from "dockview-core";

import { nearest, type Direction } from "./nearest";

export type Moved = "moved" | "split" | "none";

/**
 * Move 6 of SP-8: the active tile goes into the nearest group in that direction; with no
 * neighbour there, it splits its own group on that side if the group holds another tile, and a
 * tile alone in its group -- as every group of the views that ship -- stays where it is (AUD-721
 * and AUD-2232 of the audit of 2026-09-30).
 *
 * ⛔ GEOMETRY, NOT `dockview`'S NAVIGATION API: it is what a keyboard user SEES, and the library's
 * spatial navigation is a paid feature (§4 of the north star). Locked groups -- the nucleus, the
 * strip -- are never a target, AND NEVER A SOURCE: move 1 says the two stay where they are, and in
 * `dockview-core` 8.3.1 `locked` vetoes only a drop, never a `moveTo` (AUD-537 and AUD-538 of the
 * audit of 2026-09-30). The strip does become the active panel: its «+ moduli» takes the focus.
 *
 * ⚠️ UNDER jsdom EVERY RECT IS ZERO (task 13, step 3), so in a jsdom probe every other group
 * counts as "beyond" in every direction: the probe of `keys.test.ts` hands rectangles of its own
 * instead, and the real geometry, with the real `moveTo`, is probed in `dock.browser.test.ts`.
 * The geometry itself is `nearest`, shared with the overview's grid from task 8 of the design system.
 */
export function moveActive(api: DockviewApi, direction: Direction): Moved {
  const panel = api.activePanel;
  if (panel === undefined || panel.group.locked) return "none";
  const candidates = api.groups
    .filter((group) => group !== panel.group && !group.locked)
    .map((group) => ({ group, rect: group.element.getBoundingClientRect() }));
  const target = nearest(panel.group.element.getBoundingClientRect(), candidates, direction);
  if (target !== undefined) {
    panel.api.moveTo({ group: target.group, position: "center" });
    return "moved";
  }
  // ⛔ A TILE ALONE IN ITS GROUP HAS NOTHING TO SPLIT: it is already on that side of it, and nothing moves (AUD-721 of
  // the audit of 2026-09-30). Measured in the installed Chrome on 2026-10-02 (`dock.browser.test.ts`): handed a `moveTo`
  // onto its own side, `dockview-core` 8.3.1 puts the group back where it was along its parent's axis, and across it
  // takes the group out of the grid and throws `Invalid grid element` -- the tile gone from the screen.
  if (panel.group.size < 2) return "none";
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

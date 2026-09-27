/** A rectangle, as `getBoundingClientRect` gives it and as a probe writes it by hand. */
export interface Box {
  left: number;
  top: number;
  right: number;
  bottom: number;
}

export type Direction = "left" | "right" | "up" | "down";

/**
 * The candidate nearest to `from` in a direction: the geometry of move 6 of SP-8, the tiles moved with the keyboard, and,
 * from task 8, of the arrows in the overview's grid (decision 19 of the design system) -- its second occurrence, so it
 * lives here once.
 * Only what lies BEYOND `from` in that direction counts, and the nearest is the smallest gap on that axis.
 *
 * ⛔ A TIE IS BROKEN ON THE OTHER AXIS, by the centre nearest to `from`'s (R3-18 of the design-system review): in a grid
 * every card of the row below is equally far, and the gap alone sent "down" to the first column from any column.
 *
 * ⚠️ UNDER jsdom EVERY RECT IS ZERO: the probes hand rectangles of their own (`nearest.test.ts`, `keys.test.ts`), and the
 * browser is where the real ones are seen.
 */
export function nearest<T extends { rect: Box }>(from: Box, candidates: readonly T[], direction: Direction): T | undefined {
  const beyond = (rect: Box): boolean =>
    direction === "left" ? rect.right <= from.left + 1
    : direction === "right" ? rect.left >= from.right - 1
    : direction === "up" ? rect.bottom <= from.top + 1
    : rect.top >= from.bottom - 1;
  const gap = (rect: Box): number =>
    direction === "left" ? from.left - rect.right
    : direction === "right" ? rect.left - from.right
    : direction === "up" ? from.top - rect.bottom
    : rect.top - from.bottom;
  const across = (rect: Box): number =>
    direction === "left" || direction === "right"
      ? Math.abs(rect.top + rect.bottom - from.top - from.bottom) / 2
      : Math.abs(rect.left + rect.right - from.left - from.right) / 2;
  return candidates
    .filter(({ rect }) => beyond(rect))
    .sort((a, b) => gap(a.rect) - gap(b.rect) || across(a.rect) - across(b.rect))[0];
}

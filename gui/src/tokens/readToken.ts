/**
 * A design token as the page computes it, for whoever needs it outside CSS (design system, section (c)): the dock's `gap`
 * today, the canvases of sub-projects 6, 7 and 12 tomorrow. The CSS variables are the truth (answer 14): this reads them,
 * it keeps no copy.
 *
 * ⛔ A TOKEN THE PAGE DOES NOT DEFINE IS AN ERROR, NOT AN EMPTY STRING. Measured under jsdom on 2026-09-23: a `gap`
 * parsed from "" is NaN, `dockview-core` 8.3.1 takes it without a word, and `toJSON()` then says `null` for every size --
 * a settle would have saved that into the core's package.
 */
export function readToken(name: string, element: Element = document.documentElement): string {
  const value = getComputedStyle(element).getPropertyValue(name).trim();
  if (value === "") throw new Error(`the token ${name} is not defined here: are the token sheets loaded?`);
  return value;
}

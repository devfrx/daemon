/**
 * The probes of the design-system boards, as functions over the real DOM (design system, section (f)). They were
 * scripts pasted into a console on 2026-09-23 -- `sonda-raggi.js`, `sonda-icone.js`, `sonda-caratteri.js`, next to the
 * approved boards -- and each was proven in both directions there. Here they take their ROOTS as a parameter (trap 4),
 * and each returns how much it looked at, so that a caller can refuse a green that looked at nothing (trap 1).
 *
 * ⛔ THEY NEED A LAYOUT ENGINE: under jsdom every rectangle is zero, so they run in the browser project alone.
 */

function describe(element: Element): string {
  const classes = (element.getAttribute("class") ?? "").trim().split(/\s+/)[0];
  return classes !== undefined && classes !== "" ? classes : element.tagName.toLowerCase();
}

/**
 * The owner's rule of answer 4 -- OUTER radius = INNER radius + distance. For every element with a rounded corner, the
 * nearest rounded ancestor inside a root, and their four corners, EACH WITH ITS OWN RADIUS (E30 of the design-system
 * plan): where the inner corner sits close to the outer one (within the larger radius, plus 2 px), an element IN the
 * corner must share the centre, and one OFF the corner must not be rounder than the outer radius minus the smaller
 * distance. A straight corner, inside or outside, is never compared (answer 20) -- a sheet's `r r 0 0` included. SVG
 * content is a drawing, not a surface.
 *
 * ⛔ THE RADIUS A CORNER IS DRAWN WITH, NOT THE ONE WRITTEN (E46 of the design-system plan): where the two radii of a side
 * add up to more than the side, CSS shrinks ALL the radii of the box by one factor (css-backgrounds-3, section 4.5,
 * "Overlapping Curves") -- so a pill's 9999px is half its height, and a 24 px bar's `20px 20px 0 0` stays 20. Halving
 * each corner on its own read that bar, the dock's floating title bar, as 12.
 * ⛔ WHAT SCROLLS IS NOT PLACED (E44): an element that reaches its ancestor through a box whose content scrolls sits where
 * the scroll put it, not at a distance anyone drew -- the dock's Impostazioni, taller than its card, put a radio 19 px
 * from a corner. It is not judged; what does not scroll still is. ⚠️ ONLY A BOX BETWEEN THE TWO (E53): an ancestor
 * that scrolls ITSELF is still judged, at the scroll it has -- today no root the tests judge holds one scrolled, and the
 * dialog's sheet, which scrolls, has its top corners placed by its padding at 0.
 */
export function concentricRadii(roots: Element[]): { near: number; bad: string[] } {
  // ⛔ HALF A PIXEL OF SLACK (E61 of the design-system plan): a slack is chosen tighter than the smallest defect the rule
  // wants to see, and that defect is the pixel of a border -- 1.5 kept green a card's radius 1 px inside the card's
  // edge, on the kit page and in the dock's floating group. The same half pixel decides "in the corner": two distances
  // further apart cannot share a centre, and the rule off the corner judges them (D25).
  const SLACK = 0.5;
  const CORNERS = ["TopLeft", "TopRight", "BottomLeft", "BottomRight"] as const;
  type Corner = (typeof CORNERS)[number];
  const radius = (element: Element, corner: Corner): number =>
    Number.parseFloat(getComputedStyle(element)[`border${corner}Radius` as const]) || 0;
  const rounded = (element: Element): boolean => CORNERS.some((corner) => radius(element, corner) > 0);
  const effective = (element: Element, corner: Corner): number => {
    const box = element.getBoundingClientRect();
    const [tl, tr, bl, br] = CORNERS.map((c) => radius(element, c)) as [number, number, number, number];
    const room = (side: number, radii: number): number => (radii > 0 ? side / radii : Number.POSITIVE_INFINITY);
    const across = Math.min(room(box.width, tl + tr), room(box.width, bl + br));
    const down = Math.min(room(box.height, tl + bl), room(box.height, tr + br));
    return radius(element, corner) * Math.min(1, across, down);
  };
  const scrolled = (element: Element, ancestor: Element): boolean => {
    for (let box = element.parentElement; box !== null && box !== ancestor; box = box.parentElement) {
      const style = getComputedStyle(box);
      const acrossScrolls = /auto|scroll/.test(style.overflowX) && box.scrollWidth > box.clientWidth + 1;
      const downScrolls = /auto|scroll/.test(style.overflowY) && box.scrollHeight > box.clientHeight + 1;
      if (acrossScrolls || downScrolls) return true;
    }
    return false;
  };
  const bad: string[] = [];
  let near = 0;
  for (const root of roots) {
    for (const element of [root, ...root.querySelectorAll("*")]) {
      if (element.closest("svg") !== null) continue;
      if (!rounded(element)) continue;
      let ancestor = element.parentElement;
      while (ancestor !== null && !rounded(ancestor)) ancestor = ancestor.parentElement;
      if (ancestor === null || !root.contains(ancestor)) continue;
      if (scrolled(element, ancestor)) continue;
      const b = element.getBoundingClientRect();
      const B = ancestor.getBoundingClientRect();
      const corners: [string, Corner, number, number][] = [
        ["top-left", "TopLeft", b.left - B.left, b.top - B.top],
        ["top-right", "TopRight", B.right - b.right, b.top - B.top],
        ["bottom-left", "BottomLeft", b.left - B.left, B.bottom - b.bottom],
        ["bottom-right", "BottomRight", B.right - b.right, B.bottom - b.bottom],
      ];
      for (const [name, corner, dx, dy] of corners) {
        const inner = effective(element, corner);
        const outer = effective(ancestor, corner);
        if (inner === 0 || outer === 0) continue;
        const reach = Math.max(outer, inner) + 2;
        if (!(dx < reach && dy < reach)) continue;
        near += 1;
        const inTheCorner = Math.abs(dx - dy) <= SLACK;
        const ok = inTheCorner ? Math.abs(inner - (outer - dx)) <= SLACK : inner <= outer - Math.min(dx, dy) + SLACK;
        if (!ok) {
          bad.push(`${describe(element)} in ${describe(ancestor)}, ${name}: radius ${inner.toFixed(1)}, outer ${outer.toFixed(1)}, distance ${dx.toFixed(1)}/${dy.toFixed(1)}`);
        }
      }
    }
  }
  return { near, bad };
}

/**
 * No text is cut -- an element's content is never wider than the element -- and nothing sticks out of the nearest
 * box that holds it: the fourth check of `sonda-caratteri.js`, with the boxes as a parameter.
 */
export function fits(roots: Element[], boxes: string): { seen: number; boxed: number; problems: string[] } {
  const problems: string[] = [];
  let seen = 0;
  let boxed = 0;
  for (const root of roots) {
    for (const element of root.querySelectorAll<HTMLElement>("*")) {
      if (element.closest("svg") !== null) continue;
      seen += 1;
      if (element.clientWidth > 0 && element.scrollWidth > element.clientWidth + 1) {
        problems.push(`cut: ${describe(element)} "${(element.textContent ?? "").trim().slice(0, 24)}" ${element.scrollWidth}>${element.clientWidth}`);
      }
      const box = element.parentElement?.closest(boxes);
      if (box === null || box === undefined) continue;
      const b = element.getBoundingClientRect();
      const B = box.getBoundingClientRect();
      if (b.width === 0 && b.height === 0) continue;
      boxed += 1;
      if (b.left < B.left - 1 || b.right > B.right + 1 || b.top < B.top - 1 || b.bottom > B.bottom + 1) {
        problems.push(`sticks out: ${describe(element)} of ${describe(box)}`);
      }
    }
  }
  return { seen, boxed, problems };
}

/**
 * Every icon is drawn, strokes with `currentColor` -- read on the COMPUTED stroke, since a CSS rule beats the
 * presentation attribute (E32 of the design-system plan) -- and, in a flex row that centres, sits within 0.75 px of the
 * centre of its parent's content box: `sonda-icone.js`, on the icons of `BaseIcon`. ⚠️ An icon whose parent is NOT a
 * centring flex row is counted and not judged: a violation that un-centres the PARENT falls through here, so the red
 * direction moves the icon inside a centred row (R3-3 of the review).
 */
export function iconsCentred(roots: Element[]): { icons: number; centred: number; problems: string[] } {
  const problems: string[] = [];
  let icons = 0;
  let centred = 0;
  for (const root of roots) {
    for (const svg of root.querySelectorAll("svg.base-icon")) {
      icons += 1;
      const name = svg.getAttribute("data-icon") ?? "?";
      const b = svg.getBoundingClientRect();
      if (!(b.width > 0 && b.height > 0)) problems.push(`not drawn: ${name}`);
      const drawn = getComputedStyle(svg);
      if (drawn.stroke !== drawn.color) problems.push(`stroke is not currentColor: ${name}`);
      const parent = svg.parentElement;
      if (parent === null) continue;
      const style = getComputedStyle(parent);
      if (!style.display.includes("flex") || style.alignItems !== "center" || style.flexDirection.startsWith("column")) continue;
      const P = parent.getBoundingClientRect();
      const top = P.top + Number.parseFloat(style.borderTopWidth) + Number.parseFloat(style.paddingTop);
      const bottom = P.bottom - Number.parseFloat(style.borderBottomWidth) - Number.parseFloat(style.paddingBottom);
      const off = (b.top + b.bottom) / 2 - (top + bottom) / 2;
      centred += 1;
      if (Math.abs(off) > 0.75) problems.push(`off centre by ${off.toFixed(2)} px: ${name} in ${describe(parent)}`);
    }
  }
  return { icons, centred, problems };
}

/** The first family an element computes -- the check "applied" of `sonda-caratteri.js`. */
export function firstFamily(element: Element): string {
  return (getComputedStyle(element).fontFamily.split(",")[0] ?? "").trim().replace(/^["']|["']$/g, "");
}

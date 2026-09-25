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
 * The owner's rule of answer 4 -- OUTER radius = INNER radius + distance. For every element with a radius, the nearest
 * rounded ancestor inside a root, and its four corners: where the inner corner sits close to the outer one (within the
 * larger radius, plus 2 px), an element IN the corner must share the centre, and one OFF the corner must not be rounder
 * than the outer radius minus the smaller distance. A straight corner is never compared (answer 20): an element or an
 * ancestor with no radius is skipped. SVG content is a drawing, not a surface.
 */
export function concentricRadii(roots: Element[]): { near: number; bad: string[] } {
  const radius = (element: Element): number => Number.parseFloat(getComputedStyle(element).borderTopLeftRadius) || 0;
  const effective = (element: Element): number => {
    const box = element.getBoundingClientRect();
    return Math.min(radius(element), box.height / 2, box.width / 2);
  };
  const bad: string[] = [];
  let near = 0;
  for (const root of roots) {
    for (const element of [root, ...root.querySelectorAll("*")]) {
      if (element.closest("svg") !== null) continue;
      const inner = effective(element);
      if (inner === 0) continue;
      let ancestor = element.parentElement;
      while (ancestor !== null && !(radius(ancestor) > 0)) ancestor = ancestor.parentElement;
      if (ancestor === null || (!root.contains(ancestor) && ancestor !== root)) continue;
      const outer = effective(ancestor);
      const b = element.getBoundingClientRect();
      const B = ancestor.getBoundingClientRect();
      const corners: [string, number, number][] = [
        ["top-left", b.left - B.left, b.top - B.top],
        ["top-right", B.right - b.right, b.top - B.top],
        ["bottom-left", b.left - B.left, B.bottom - b.bottom],
        ["bottom-right", B.right - b.right, B.bottom - b.bottom],
      ];
      for (const [corner, dx, dy] of corners) {
        const reach = Math.max(outer, inner) + 2;
        if (!(dx < reach && dy < reach)) continue;
        near += 1;
        const inTheCorner = Math.abs(dx - dy) <= 1.5;
        const ok = inTheCorner ? Math.abs(inner - (outer - dx)) <= 1.5 : inner <= outer - Math.min(dx, dy) + 1.5;
        if (!ok) {
          bad.push(`${describe(element)} in ${describe(ancestor)}, ${corner}: radius ${inner.toFixed(1)}, outer ${outer.toFixed(1)}, distance ${dx.toFixed(1)}/${dy.toFixed(1)}`);
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
 * Every icon is drawn, strokes with `currentColor`, and -- in a flex row that centres -- sits within 0.75 px of the
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
      if (svg.getAttribute("stroke") !== "currentColor") problems.push(`stroke is not currentColor: ${name}`);
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

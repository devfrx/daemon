// The concentric-radius probe of the design-system brainstorming, 2026-09-23.
//
// HOW TO RUN IT: open `stile-approvato.html` (next to this file) in a browser, paste this whole file into the
// developer console, and read the value it returns. On the approved page `bad` is EMPTY.
//
// WHAT IT CHECKS -- the owner's rule, "outer radius = inner radius + distance": for every element with a
// border radius, it finds the nearest rounded ancestor and looks at the four corners. When the inner corner
// sits close to the outer one (within the larger of the two radii, plus 2 px), then
//   - if the inner element sits IN the corner (the same distance from both edges), its radius must be the
//     outer radius minus that distance: the two arcs share one centre;
//   - if it sits off the corner (two different distances), its radius must not exceed the outer radius minus
//     the SMALLER distance: the inner corner is never rounder than the outer one next to it.
//
// ⚠️ IT WAS PROVEN IN BOTH DIRECTIONS on 2026-09-23: empty on the approved page, and on the page before it --
// not kept, it lived in the session's scratchpad -- it caught the dialog the owner had pointed at, a radius of
// 14 inside a radius of 10, ten pixels apart. The first version of this probe MISSED that very case, because
// it only looked inside the outer arc (distance < outer radius) and the dialog sat exactly on its edge: the
// "within the larger of the two radii" clause is the cure, and it is why the clause is there.
//
// ⛔ IT IS A PROBE ON A MOCKUP, NOT A TEST OF THE PRODUCT: the plan turns it into a test of the real kit, the
// way `gui/src/tokens/contrast.test.ts` holds the contrast. SVG content is skipped (a drawing, not a surface).
(() => {
  const radius = (e) => parseFloat(getComputedStyle(e).borderTopLeftRadius) || 0;
  const effective = (e) => {
    const b = e.getBoundingClientRect();
    return Math.min(radius(e), b.height / 2, b.width / 2);
  };
  const roots = [...document.querySelectorAll(".ds .m, .ds .kit")];
  const bad = [];
  let near = 0;
  for (const root of roots) {
    for (const e of [root, ...root.querySelectorAll("*")]) {
      if (e.closest("svg")) continue;
      const r = effective(e);
      if (!r) continue;
      let a = e.parentElement;
      while (a && !(radius(a) > 0)) a = a.parentElement;
      if (!a || (!root.contains(a) && a !== root)) continue;
      const R = effective(a);
      const b = e.getBoundingClientRect();
      const B = a.getBoundingClientRect();
      const corners = [
        ["top-left", b.left - B.left, b.top - B.top],
        ["top-right", B.right - b.right, b.top - B.top],
        ["bottom-left", b.left - B.left, B.bottom - b.bottom],
        ["bottom-right", B.right - b.right, B.bottom - b.bottom],
      ];
      for (const [corner, dx, dy] of corners) {
        const reach = Math.max(R, r) + 2;
        if (!(dx < reach && dy < reach)) continue;
        near += 1;
        const inTheCorner = Math.abs(dx - dy) <= 1.5;
        const ok = inTheCorner ? Math.abs(r - (R - dx)) <= 1.5 : r <= R - Math.min(dx, dy) + 1.5;
        if (!ok) {
          bad.push({
            element: String(e.className).slice(0, 32),
            inside: String(a.className).slice(0, 32),
            corner,
            radius: +r.toFixed(1),
            outer: +R.toFixed(1),
            distance: [+dx.toFixed(1), +dy.toFixed(1)],
          });
        }
      }
    }
  }
  return { near, bad };
})();

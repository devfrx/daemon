// The icon probe of the design-system brainstorming, 2026-09-23 (second session).
//
// HOW TO RUN IT: on a board whose icons are inline SVGs with class `i`, evaluate this file; it returns a report,
// one entry per row -- an element with class `fp` and a data-name.
//
// WHAT IT CHECKS, per row, on the Home and the kit (`.m`, `.kit`) -- the large specimen strip is not a surface:
//   1. DRAWN -- every icon has a box wider and taller than zero, and draws with `currentColor`.
//   2. CENTRED -- an icon whose parent is a flex row with `align-items: center` has its vertical centre within
//      0.75 px of the centre of the parent's content box. A misaligned icon is the detail the eye catches first.
//   3. PARITY -- every row has the same number of icons: the comparison is fair only if both sets fill the same
//      roles.
// The status-message icons are aligned to the first line of their text, not centred: they are skipped by 2.
//
// ⚠️ IT WAS PROVEN IN BOTH DIRECTIONS on 2026-09-23: clean on the icons board -- Lucide and Tabler, 21 icons each,
// 18 of them centred-checked -- at 1440 and 800 px, and on the approved board; and on the icons board with four
// defects injected by hand -- an icon pushed 3 px down, an icon of zero width, a stroke that is not
// `currentColor`, an icon removed from one row -- it reported every one: "off centre by 1.50 px", "not drawn",
// "stroke is not currentColor", "parity".
//
// ⛔ IT IS A PROBE ON A MOCKUP, NOT A TEST OF THE PRODUCT, like the two probes next to it: in the kit, answer 11
// puts every icon behind ONE component and ONE map, and the plan turns these checks into its tests.
(() => {
  const rows = [...document.querySelectorAll(".ds .fp")];
  const out = [];
  for (const row of rows) {
    const r = { row: row.dataset.name, problems: [], icons: 0, centred: 0 };
    for (const svg of row.querySelectorAll(".m svg.i, .kit svg.i")) {
      r.icons += 1;
      const b = svg.getBoundingClientRect();
      const name = svg.dataset.icon || "?";
      if (!(b.width > 0 && b.height > 0)) r.problems.push(`not drawn: ${name}`);
      if (svg.getAttribute("stroke") !== "currentColor") r.problems.push(`stroke is not currentColor: ${name}`);
      const p = svg.parentElement;
      const cs = getComputedStyle(p);
      if (p.classList.contains("ic")) continue;
      if (!cs.display.includes("flex") || cs.alignItems !== "center" || cs.flexDirection.startsWith("column")) continue;
      const P = p.getBoundingClientRect();
      const top = P.top + parseFloat(cs.borderTopWidth) + parseFloat(cs.paddingTop);
      const bottom = P.bottom - parseFloat(cs.borderBottomWidth) - parseFloat(cs.paddingBottom);
      const off = (b.top + b.bottom) / 2 - (top + bottom) / 2;
      r.centred += 1;
      if (Math.abs(off) > 0.75) r.problems.push(`off centre by ${off.toFixed(2)} px: ${name} in ${String(p.className).slice(0, 20)}`);
    }
    out.push(r);
  }
  const counts = [...new Set(out.map((r) => r.icons))];
  if (counts.length > 1) for (const r of out) r.problems.push(`parity: icon counts differ across rows ${counts}`);
  return out;
})();

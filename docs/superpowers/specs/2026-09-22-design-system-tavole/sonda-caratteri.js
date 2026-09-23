// The font probe of the design-system brainstorming, 2026-09-23 (second session).
//
// HOW TO RUN IT: on the fonts board, evaluate this file; it returns a promise of a report, one entry per row.
// A row is an element with class `fp` and the attributes data-name, data-text and data-tool: the two families
// the row must use. The reference row (the approved board's Windows fonts) carries data-system.
//
// WHAT IT CHECKS, per row:
//   1. LOADED -- each named family has at least one FontFace in document.fonts whose status is "loaded".
//      ⚠️ `document.fonts.check()` is NOT the oracle: it answers true for a family no @font-face declares.
//      System fonts have no FontFace, so the reference row skips this check.
//   2. APPLIED -- sample elements compute a font-family whose FIRST name is the expected family.
//   3. TABULAR -- the tool family, with tabular-nums, draws "111111" and "000000" at the same width.
//   4. FIT -- no text is cut: an element's content is never wider than the element (scrollWidth), and no
//      element sticks out of the nearest box that holds it.
// The widths of a sample label and of a sample sentence are returned too, relative to the reference row when
// the page has one (the fonts board of 2026-09-23 had it; the approved board does not).
//
// ⚠️ IT WAS PROVEN IN BOTH DIRECTIONS on 2026-09-23: clean on the fonts board -- four rows -- and on the approved
// board; and on the fonts board with four defects injected by hand -- a family no @font-face declares, a row that
// applies another font than it declares, a table cell whose text no longer fits, a button pushed out of its card --
// it reported every one, and the tabular check turned red too, on the fallback font of the undeclared family.
//
// ⛔ IT IS A PROBE ON A MOCKUP, NOT A TEST OF THE PRODUCT, like `sonda-raggi.js` next to it: the plan turns it into
// a test of the real kit.
(async () => {
  await document.fonts.ready;
  const first = (ff) => ff.split(",")[0].trim().replace(/^["']|["']$/g, "");
  const faces = [...document.fonts];
  const loaded = (fam) => faces.some((f) => first(f.family) === fam && f.status === "loaded");
  const cls = (e) => String(e.className).slice(0, 28) || e.tagName.toLowerCase();
  const width = (host, css, txt) => {
    const s = document.createElement("span");
    s.textContent = txt;
    s.style.cssText = "position:absolute;visibility:hidden;white-space:nowrap;left:0;top:0;" + css;
    host.appendChild(s);
    const w = s.getBoundingClientRect().width;
    s.remove();
    return w;
  };
  const SAMPLES = {
    text: [".m-row span", ".m .m-btn", ".kit .btn", ".kit .msg b", ".kit .li span"],
    tool: [".m-h", ".m-big", ".m-strip", ".m-tab td", ".kit .tag", ".kit .li em"],
  };
  const BOXES = ".m-mod,.k-card,.m-bar,.m-strip,.m-core,.dlg,.msg,.field,.btn,.m-btn,.chip,.pill,.m-pill,.tag,.m-chip,.m-search,.li,.m,.kit";
  const LABEL = "PERMESSI · ATTIVITÀ · 14:02";
  const SENTENCE = "Il core risponde e nessun degrado è attivo.";
  const rows = [...document.querySelectorAll(".ds .fp")];
  const out = [];
  for (const row of rows) {
    const want = { text: row.dataset.text, tool: row.dataset.tool };
    const r = { row: row.dataset.name, problems: [] };
    if (!("system" in row.dataset)) {
      for (const fam of [want.text, want.tool]) if (!loaded(fam)) r.problems.push(`not loaded: ${fam}`);
    }
    for (const [role, sels] of Object.entries(SAMPLES)) {
      for (const s of sels) {
        const e = row.querySelector(s);
        if (!e) { r.problems.push(`no sample ${s}`); continue; }
        const got = first(getComputedStyle(e).fontFamily);
        if (got !== want[role]) r.problems.push(`${s}: ${got}, wanted ${want[role]}`);
      }
    }
    const tool = `font-family:"${want.tool}";font-size:27px;font-weight:300;`;
    const tab = [width(row, tool + "font-variant-numeric:tabular-nums", "111111"),
                 width(row, tool + "font-variant-numeric:tabular-nums", "000000")];
    const prop = [width(row, tool + "font-variant-numeric:normal", "111111"),
                  width(row, tool + "font-variant-numeric:normal", "000000")];
    r.tabular = Math.abs(tab[0] - tab[1]) < 0.5;
    if (!r.tabular) r.problems.push(`tool digits not tabular: ${tab.map((x) => x.toFixed(1))}`);
    r.digits = { tabular: tab.map((x) => +x.toFixed(1)), normal: prop.map((x) => +x.toFixed(1)) };
    r.label = +width(row, `font-family:"${want.tool}";font-size:9px;font-weight:600;letter-spacing:.18em;`, LABEL).toFixed(1);
    r.sentence = +width(row, `font-family:"${want.text}";font-size:11px;font-weight:400;`, SENTENCE).toFixed(1);
    let seen = 0, boxed = 0;
    for (const e of row.querySelectorAll(".m *, .kit *")) {
      if (e.closest("svg")) continue;
      seen += 1;
      if (e.clientWidth > 0 && e.scrollWidth > e.clientWidth + 1) {
        r.problems.push(`cut: ${cls(e)} "${e.textContent.trim().slice(0, 24)}" ${e.scrollWidth}>${e.clientWidth}`);
      }
      const box = e.parentElement && e.parentElement.closest(BOXES);
      if (!box) continue;
      const b = e.getBoundingClientRect(), B = box.getBoundingClientRect();
      if (b.width === 0 && b.height === 0) continue;
      boxed += 1;
      if (b.left < B.left - 1 || b.right > B.right + 1 || b.top < B.top - 1 || b.bottom > B.bottom + 1) {
        r.problems.push(`sticks out: ${cls(e)} of ${cls(box)}`);
      }
    }
    r.checked = { seen, boxed };
    out.push(r);
  }
  const ref = out.find((r) => r.row === "ref");
  if (ref) {
    for (const r of out) {
      r.vs_ref = { label: +(r.label / ref.label).toFixed(3), sentence: +(r.sentence / ref.sentence).toFixed(3) };
    }
  }
  return out;
})();

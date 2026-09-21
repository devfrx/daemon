import { createDockview, themeAbyss, type DockviewApi, type SerializedDockview } from "dockview-core";
import { watch } from "vue";

import { componentFor, isBuilt, placeholderParams } from "../panels/registry";
import { useLayout, type LayoutPack, type ViewName } from "../stores/layout";
import { VIEWS } from "../panels/views";

import { BigTab } from "./BigTab";

const GRID = 24;

/** Sorts object keys recursively: key order is not layout. */
export function canonical(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(canonical);
  if (value !== null && typeof value === "object") {
    const source = value as Record<string, unknown>;
    return Object.fromEntries(Object.keys(source).sort().map((k) => [k, canonical(source[k])]));
  }
  return value;
}

/**
 * ⛔ MEASURED ON 2026-09-10 BY SP-8 (E4 of the part-1 plan): `toJSON()` after `fromJSON()` is
 * equal only in CANONICAL form -- `panels` comes back in another key order, a floating group
 * grows by 2 px per round trip, and minimum sizes are imposed on a narrow viewport. So "has the
 * layout settled?" is answered on the canonical form. Comparing the raw strings would save on
 * every idle tick, which is the behaviour decision 12 exists to avoid.
 */
function same(a: SerializedDockview, b: SerializedDockview): boolean {
  return JSON.stringify(canonical(a)) === JSON.stringify(canonical(b));
}

/**
 * ⛔ THE DOCK FOLLOWS THE STORE (D89), in the two ways the store changes under it: the bar writes
 * `layout.view`, and the core sends a package that is NOT the echo of our own save -- the welcome
 * after `Hello`, or the OLD package after a write that did not stick (decision 13). Both are shown
 * here and nowhere else: `Frame.vue` does not call `apply`, and a `Layout` that arrives after the
 * dock is up is not left in the store. Before D89 it was: `apply` ran once, before `Hello`, and
 * the saved layout never appeared at start-up -- it compiled and passed every probe.
 *
 * ⛔ SHOWING RESETS THE BASELINE: the buffered `onDidLayoutChange` that follows a `fromJSON`
 * compares equal and does not settle, so LOOKING at a view is not SAVING it -- decision 11, the
 * shipped views stay in `gui/` until the owner changes one.
 *
 * ⚠️ THE ACTIVE PANEL IS LAYOUT (D81): `activeGroup` is in `toJSON()`, and `dockview` fires
 * `onDidLayoutChange` on `onDidActiveChange` too (measured on the 8.2.0 in SP-8's `node_modules`,
 * R9b-12; if 8.3.1 differs, this line is an errata). So a click that changes the active panel
 * settles, and that is accepted: which panel is active is part of where things are.
 */
export function createDock(host: HTMLElement): DockviewApi {
  const layout = useLayout();
  const api = createDockview(host, {
    // The theme the eight moves were judged on. Our own tokens dress what WE draw -- the bar,
    // the band, the drawer, the strip, the placeholder -- and the design system is decided in
    // its own three moments, none of which is this task.
    theme: themeAbyss,
    defaultTabComponent: "bigtab",
    // Q4 of SP-8: the doc recommends `pointer` where HTML5 drag is unreliable and names embedded
    // webviews; and ADR-0039's hand needs it, because a script cannot start a native HTML5 drag.
    // ⚠️ It costs drag BETWEEN WINDOWS and the native drag image (decision 34) -- neither is a
    // sub-project 2 feature.
    dndStrategy: "pointer",
    floatingGroupBounds: "boundedWithinViewport",
    transformFloatingGroupDrag: ({ proposed }) => ({
      left: Math.round(proposed.left / GRID) * GRID,
      top: Math.round(proposed.top / GRID) * GRID,
    }),
    createComponent: ({ name }) => componentFor(name)(),
    createTabComponent: () => new BigTab(),
  });

  api.layout(host.clientWidth, host.clientHeight);

  function show(view: ViewName): SerializedDockview {
    apply(api, view, layout.saved);
    return api.toJSON();
  }

  let last = show(layout.view);
  watch([() => layout.view, () => layout.arrivals], () => {
    last = show(layout.view);
  });

  api.onDidLayoutChange(() => {
    const now = api.toJSON();
    if (same(last, now)) return;
    last = now;
    layout.settle(now);
  });

  window.addEventListener("beforeunload", () => {
    // Decision 12: and when the window closes. ⛔ ONLY IF SOMETHING CHANGED SINCE THE LAST SETTLE
    // (D81): before, an untouched gui copied the shipped Home into the archive at its first close.
    const now = api.toJSON();
    if (same(last, now)) return;
    last = now;
    layout.settle(now);
  });

  window.addEventListener("resize", () => api.layout(host.clientWidth, host.clientHeight));
  return api;
}

/**
 * ⛔ A SAVED VIEW WINS OVER THE DEFAULT BY NAME, AND A PACKAGE WE CANNOT READ LOSES TO IT: rows 6
 * and 8 of §2 of the north star -- the gui copes rather than complains. ⛔ BY NAME (D80): the
 * package holds one layout PER VIEW, and a view it does not hold falls back to the shipped one.
 * Before D80 the one saved layout was applied under every tab.
 */
export function apply(api: DockviewApi, view: ViewName, pack: LayoutPack | null): void {
  api.fromJSON(pack?.layouts[view] ?? VIEWS[view]);
  for (const panel of api.panels) {
    // ⛔ ONLY WHAT NOBODY BUILT (R6-17): the strip is a piece of the frame, carries no `params`, and
    // is not a module type -- without this line it got `{ missing: true }` at every `apply`, and
    // that value entered the saved package at the first `settle`. Unseen, because `Strip.vue`
    // ignores its params.
    if (!isBuilt(panel.id) && Object.keys(panel.params ?? {}).length === 0) {
      panel.api.updateParameters(placeholderParams(panel.id));
    }
  }
}

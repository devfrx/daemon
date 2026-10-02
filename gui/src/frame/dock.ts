import { createDockview, type DockviewApi, type DockviewTheme, type SerializedDockview } from "dockview-core";
import { watch } from "vue";

import { componentFor, isBuilt, placeholderParams } from "../panels/registry";
import { useLayout, type LayoutPack, type ViewName } from "../stores/layout";
import { VIEWS } from "../panels/views";
import { readToken } from "../tokens/readToken";
import { shownTheme } from "../tokens/theme";

import { BigTab } from "./BigTab";

const GRID = 24;

/** A token that is a length in px, as the number `dockview` wants. ⛔ THE UNIT IS CHECKED, NOT
 * DROPPED: `parseFloat("0.75rem")` is 0.75, and a board that moved a space to `rem` would shrink
 * the gap to nothing without a word. */
function pixels(name: string): number {
  const value = readToken(name);
  const found = /^(\d+(?:\.\d+)?)px$/.exec(value);
  if (found === null) throw new Error(`the token ${name} is "${value}", not a length in px`);
  return Number(found[1]);
}

/**
 * OUR `dockview` THEME (design system, section (c)): `tokens/dock.css` dresses its class, and the
 * space between the groups is the token `--space-3`, read and not retyped.
 *
 * ⚠️ `colorScheme` FOLLOWS THE THEME ON SCREEN, AS THE (c) WANTS, AND DRAWS NOTHING: in
 * `dockview-core` 8.3.1 `updateTheme` reads the class, the gap, the edge groups' size, the drop
 * border, the overlay's mounting and the tab groups' indicator -- not `colorScheme`, which the
 * library keeps for whoever reads its options (R3-21 of the design-system review).
 * ⚠️ NO `dndOverlayBorder`: the drop zone's border is `--dv-drag-over-border` in `dock.css`, which
 * `updateTheme` leaves to the sheet when the field is absent -- one house for a colour.
 */
export function harnessTheme(): DockviewTheme {
  return { name: "harness", className: "dockview-theme-harness", colorScheme: shownTheme.value, gap: pixels("--space-3") };
}

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
 * ⛔ THE DOCK FOLLOWS THE STORE (D89), in the three ways the store changes under it: one of the
 * three views is shown (`showView`), a named view opens or closes (`openNamed`, the (d) of the
 * design system), and the core sends a package that is NOT the echo of our own save -- the welcome
 * after `Hello`, or the OLD package after a write that did not stick (decision 13). All three are
 * shown here and nowhere else: `Frame.vue` does not call `apply`, and a `Layout` that arrives after
 * the dock is up is not left in the store. Before D89 it was: `apply` ran once, before `Hello`, and
 * the saved layout never appeared at start-up -- it compiled and passed every probe.
 *
 * ⛔ SHOWING RESETS THE BASELINE: the buffered `onDidLayoutChange` that follows a `fromJSON`, and
 * a maximize deferred like it (E104), compare equal and do not settle, so LOOKING at a view is not
 * SAVING it -- decision 11, the shipped views stay in `gui/` until the owner changes one.
 *
 * ⚠️ THE ACTIVE PANEL IS LAYOUT (D81): `activeGroup` is in `toJSON()`, and `dockview` fires
 * `onDidLayoutChange` on `onDidActiveChange` too (measured on the 8.2.0 in SP-8's `node_modules`,
 * R9b-12; if 8.3.1 differs, this line is an errata). So a click that changes the active panel
 * settles, and that is accepted: which panel is active is part of where things are.
 */
export function createDock(host: HTMLElement): DockviewApi {
  const layout = useLayout();
  const api = createDockview(host, {
    // ⛔ OUR THEME AND NOT `dockview`'s abyss theme, on which the eight moves were judged: that one
    // is dark only, and the design system has two themes (section (c), answers 5 and 18).
    theme: harnessTheme(),
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

  // ⚠️ `clientWidth` IS THE CONTENT: the space around the dock is a MARGIN of `.dock` (`Frame.vue`),
  // outside the box, so the grid gets exactly the room it has (P-7 of the design-system plan).
  api.layout(host.clientWidth, host.clientHeight);

  // The theme follows the one on screen (the (c)): `updateOptions` hands it to `updateTheme` again.
  watch(shownTheme, () => api.updateOptions({ theme: harnessTheme() }));

  function show(): SerializedDockview {
    apply(api, layout.view, layout.saved, layout.openNamed);
    return api.toJSON();
  }

  // ⛔ AND THE NAMED VIEW THAT IS OPEN (the (d)): opening one, or closing it for one of the three, shows it here too.
  let last = show();
  watch([() => layout.view, () => layout.openNamed, () => layout.arrivals], () => {
    last = show();
  });

  /** What is on screen, saved only if it moved since the last save or the last showing. */
  function settleIfMoved(): void {
    const now = api.toJSON();
    if (same(last, now)) return;
    last = now;
    layout.settle(now);
  }

  api.onDidLayoutChange(settleIfMoved);

  // ⛔ A MAXIMIZE IS A MOVE, AND NOT AN `onDidLayoutChange` (E104 of the design-system plan): in `dockview-core` 8.3.1
  // `maximizeGroup` saved only through the change of the active group it makes, when it makes one, and
  // `exitMaximizedGroup` never -- a group restored stayed maximized in the package. ⛔ DEFERRED TO A MICROTASK, AS
  // `dockview` DEFERS `onDidLayoutChange`, SO THAT SHOWING STAYS NOT SAVING: `fromJSON` fires this event inside `apply` --
  // restoring the group of the view it leaves, maximizing the one of the view it opens -- before `show` resets the
  // baseline; heard at once, it saved the view left under the name of the view opened (measured on 2026-09-27).
  api.onDidMaximizedGroupChange(() => queueMicrotask(settleIfMoved));

  // Decision 12: and when the window closes. ⛔ ONLY IF SOMETHING CHANGED SINCE THE LAST SETTLE
  // (D81): before, an untouched gui copied the shipped Home into the archive at its first close.
  window.addEventListener("beforeunload", settleIfMoved);

  window.addEventListener("resize", () => api.layout(host.clientWidth, host.clientHeight));
  return api;
}

/**
 * ⛔ A SAVED VIEW WINS OVER THE DEFAULT BY NAME, AND A PACKAGE WE CANNOT READ LOSES TO IT: rows 6
 * and 8 of §2 of the north star -- the gui copes rather than complains. ⛔ BY NAME (D80): the
 * package holds one layout PER VIEW, and a view it does not hold falls back to the shipped one.
 * Before D80 the one saved layout was applied under every tab.
 * ⛔ A NAMED VIEW WINS WHILE IT IS OPEN (the (d) of the design system), and a name the package no
 * longer holds falls back to the view of always.
 *
 * ⛔ AND A LAYOUT `dockview` CANNOT READ LOSES AS ONE THE PACKAGE DOES NOT HOLD (AUD-536 and AUD-543
 * of the audit of 2026-09-30): `unpack` keeps any object as a layout, and only `fromJSON` knows what
 * it can open -- `dockview-core` 8.3.1 refuses `{}` before touching the grid, and a group it cannot
 * rebuild after clearing it, which leaves the dock empty. Uncaught, the refusal left the dock on the
 * view before, under the new view's name, and the next move saved it there; a dock built over such
 * a package was not born at all. ⛔ THE SHIPPED VIEW IS NOT GUARDED: it is committed, and a refusal
 * of it is a defect of this build, to be seen.
 */
export function apply(api: DockviewApi, view: ViewName, pack: LayoutPack | null, named: string | null = null): void {
  const chosen = named === null ? undefined : pack?.named?.find((entry) => entry.name === named)?.layout;
  openFirst(api, [chosen, pack?.layouts[view]], VIEWS[view]);
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

/** The first of the saved layouts `dockview` opens, in order, or else the shipped one. */
function openFirst(api: DockviewApi, saved: readonly (SerializedDockview | undefined)[], shipped: SerializedDockview): void {
  for (const layout of saved) {
    if (layout === undefined) continue;
    try {
      api.fromJSON(layout);
      return;
    } catch {
      // Refused: the next one wins.
    }
  }
  api.fromJSON(shipped);
}

"""patch_c13.py -- task 13 (R9b-11/D80, R9b-12/D81, R10-9, R10-10, D75, D76), the coordinator's finding C13-1 with
the new decision D89 (the dock follows the store), R7-7 applied to task 15, the D89 row in the plan's D table, the
position row of task 13, and the ledger (tasks 13, 14, 15, the D table, the state line).
Scoped, every anchor asserted right before its write, atomic.

⚠️ C13-1 was found READING the dictated code to write D80, not by a reviewer (R6 fell): `createDock` applied the layout
ONCE, in `onMounted`, i.e. BEFORE `main.ts` sends `Hello`; the `Layout` of the welcome then updated the store and nothing
showed it -- the saved layout never appeared at start-up. And `switchTo` re-applied from `Frame.vue`, so the `settle`
that followed copied the shipped view into the archive (decision 11 of the north star's coordinator). D80 touches
exactly `layout.ts`, `dock.ts` and `Frame.vue`, so the three are rewritten once, whole (decision 77).
"""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
ROOT = r"C:\Users\zagor\Desktop\harness"
PLAN = ROOT + r"\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima.md"
LEDGER = ROOT + r"\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione\ledger.md"

raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw
t = raw


def sub(seg, old, new, n=1):
    c = seg.count(old)
    assert c == n, f"expected {n}, found {c}: {old[:110]!r}"
    return seg.replace(old, new)


def between(seg, start, end, new):
    """Replace seg[start .. end) -- from the start marker's first byte to the end marker's first byte -- with new."""
    cs, ce = seg.count(start), seg.count(end)
    assert cs == 1, f"start marker found {cs} times: {start[:90]!r}"
    assert ce == 1, f"end marker found {ce} times: {end[:90]!r}"
    lo, hi = seg.index(start), seg.index(end)
    assert lo < hi, "markers out of order"
    return seg[:lo] + new + seg[hi:]


def scoped(text, start_marker, end_marker, fn):
    lo = text.index(start_marker)
    hi = text.index(end_marker, lo)
    return text[:lo] + fn(text[lo:hi]) + text[hi:]


# ============================================================================================ the rewritten blocks

LAYOUT_TS = r'''import type { SerializedDockview } from "dockview-core";
import { defineStore } from "pinia";
import { ref } from "vue";

import type { IpcMessage, LayoutState } from "../schema/messages";
import type { Bridge } from "../transport/bridge";

export type ViewName = "home" | "work" | "compact";

/**
 * ⛔ ONE LAYOUT PER VIEW, AND THE PACKAGE CARRIES THEM ALL (D80). Row 1 of §2 of the north star
 * makes the layout "which view is open, FOR EVERY VIEW where the panels are", and row 6 has a
 * saved view win over the default BY NAME. A package with a single `layout` put Home's layout
 * under the Lavoro tab and lost Home at the next settle -- and it compiled and passed every probe.
 *
 * `layouts` IS PARTIAL ON PURPOSE: a view the owner never touched has NO entry and falls back to
 * the shipped one, which is what keeps decision 11 true -- the shipped views stay in `gui/`, and
 * an update that improves one still reaches whoever has not touched it.
 */
export interface LayoutPack {
  view: ViewName;
  layouts: Partial<Record<ViewName, SerializedDockview>>;
}

const VIEWS: readonly ViewName[] = ["home", "work", "compact"];

function isViewName(value: unknown): value is ViewName {
  return typeof value === "string" && (VIEWS as readonly string[]).includes(value);
}

function sameBytes(a: readonly number[], b: readonly number[]): boolean {
  return a.length === b.length && a.every((byte, index) => byte === b[index]);
}

/**
 * ⛔ THE PACKAGE IS OPAQUE TO THE CORE AND STRUCTURED ONLY HERE (row 1 of §2 of the north star):
 * the core keeps bytes and hands them back, and if `dockview` changes format the core does not
 * change. So the shape above is the gui's business alone, and the wire carries `number[]`.
 *
 * ⛔ AND WHAT COMES BACK IS NOT TRUSTED TO BE OURS: an archive can hold a package written by an
 * older build. `unpack` returns `null` on anything it does not recognise, and the caller falls
 * back to the committed views -- the same shape as row 8 of §2, where a panel pointing at a type
 * that is gone says so and closes.
 */
export const useLayout = defineStore("layout", () => {
  const state = ref<LayoutState>({ state: "Nothing" });
  const view = ref<ViewName>("home");
  /** The package we hold: the last one the core sent, or our own last save while its echo is in
   * flight. What `apply` reads. */
  const saved = ref<LayoutPack | null>(null);
  /** ⛔ HOW MANY PACKAGES ARRIVED THAT ARE NOT THE ECHO OF OUR OWN SAVE (D89): the dock watches
   * it and shows what arrived. A counter and not an event, because the dock is a Vue watcher. */
  const arrivals = ref(0);
  let wire: Bridge | null = null;
  let sent: readonly number[] | null = null;

  function attach(bridge: Bridge): void {
    wire = bridge;
  }

  function receive(message: IpcMessage): void {
    if (message.kind !== "Layout") return;
    state.value = message.value;
    // ⛔ THE CORE ANSWERS EVERY `SaveLayout` WITH WHAT IT HOLDS (decision 13, task 7), so a package
    // equal to the bytes we last sent is our own save coming back: nothing new, nothing to show.
    // Anything else -- the welcome, the OLD package after a write that did not stick, a package
    // another build wrote -- replaces what we hold, and the dock shows it (D89).
    if (message.value.state === "Package" && sent !== null && sameBytes(message.value.bytes, sent)) return;
    saved.value = unpack(message.value);
    if (saved.value !== null) view.value = saved.value.view;
    arrivals.value += 1;
  }

  /** ⛔ AUTOMATIC, NOT A BUTTON (decision 12): when the layout settles, and when the window
   * closes. The cadence is the gui's -- it is presentation, not a kernel decision.
   *
   * ⛔ AND IT MERGES (D80): the open view's entry is replaced and the other views keep theirs. A
   * `settle` that replaced the whole package lost every view but the open one. */
  function settle(layout: SerializedDockview): void {
    const pack: LayoutPack = {
      view: view.value,
      layouts: { ...(saved.value?.layouts ?? {}), [view.value]: layout },
    };
    saved.value = pack;
    const bytes = [...pack_(pack)];
    sent = bytes;
    wire?.send({ kind: "SaveLayout", value: bytes });
  }

  return { state, view, saved, arrivals, attach, receive, settle };
});

/** The package as bytes: UTF-8 of the JSON. ⚠️ Exported for the probes, which must be able to
 * build one without a store and a bridge. */
export function pack_(pack: LayoutPack): Uint8Array {
  return new TextEncoder().encode(JSON.stringify(pack));
}

export function unpack(state: LayoutState): LayoutPack | null {
  if (state.state !== "Package") return null;
  try {
    const value: unknown = JSON.parse(new TextDecoder().decode(Uint8Array.from(state.bytes)));
    if (typeof value !== "object" || value === null) return null;
    const candidate = value as { view?: unknown; layouts?: unknown };
    if (!isViewName(candidate.view)) return null;
    if (typeof candidate.layouts !== "object" || candidate.layouts === null) return null;
    const held = candidate.layouts as Record<string, unknown>;
    const layouts: LayoutPack["layouts"] = {};
    // ⛔ ONLY THE THREE VIEWS THIS BUILD KNOWS ARE READ (decision 11): an entry under another
    // name is another build's, and is neither shown nor kept -- row 8 of §2, for views.
    for (const name of VIEWS) {
      const layout = held[name];
      if (typeof layout === "object" && layout !== null) layouts[name] = layout as SerializedDockview;
    }
    return { view: candidate.view, layouts };
  } catch {
    // ⛔ A PACKAGE THAT DOES NOT PARSE IS NOT AN ERROR TO SHOW: it is an old build's layout, and
    // the answer is the committed views. Row 8 of §2 asks the gui to cope, not to complain.
    return null;
  }
}
'''

LAYOUT_DESCRIBE = r'''describe("the layout", () => {
  it("takes a Package and reads the view out of it", () => {
    const layout = useLayout();
    const bytes = [...pack_({ view: "work", layouts: {} })];
    layout.receive({ kind: "Layout", value: { state: "Package", bytes } });
    expect(layout.state.state).toBe("Package");
    expect(layout.view).toBe("work");
  });

  it("keeps the default view on Nothing and on Unavailable", () => {
    // ⛔ THE TWO CASES THE FIXTURES CANNOT REACH (D46): `stamp_set` carries ONE message per
    // variant, so `deliver("Layout")` only ever delivers `Package`. The type system is what
    // keeps this honest -- `LayoutState` has three variants and not one more.
    for (const value of [{ state: "Nothing" }, { state: "Unavailable" }] as const) {
      setActivePinia(createPinia());
      const layout = useLayout();
      layout.receive({ kind: "Layout", value });
      expect(layout.state.state).toBe(value.state);
      expect(layout.view).toBe("home");
    }
  });

  it("refuses a package it cannot read, instead of half-applying it", () => {
    expect(unpack({ state: "Package", bytes: [0x7b, 0x7d] })).toBeNull();
    expect(unpack({ state: "Package", bytes: [0x00, 0x01] })).toBeNull();
    expect(unpack({ state: "Nothing" })).toBeNull();
    // ⛔ THE SHAPE BEFORE D80 -- one `layout` for every view -- is an old build's package now,
    // and is refused like any other: the committed views win over a package nobody can read.
    const before = [...new TextEncoder().encode('{"view":"home","layout":{}}')];
    expect(unpack({ state: "Package", bytes: before })).toBeNull();
  });

  it("sends SaveLayout as bytes when the layout settles", () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    layout.settle({} as never);
    const sent = bridge.sent[0];
    expect(sent?.kind).toBe("SaveLayout");
    expect(sent?.kind === "SaveLayout" && sent.value.length).toBeGreaterThan(0);
  });

  it("merges the open view into the package it holds, and keeps the other views (D80)", () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    const home = { marker: "home, as the owner left it" } as never;
    layout.receive({ kind: "Layout", value: { state: "Package", bytes: [...pack_({ view: "home", layouts: { home } })] } });
    layout.view = "work";
    const work = { marker: "work, just settled" } as never;
    layout.settle(work);
    const sent = bridge.sent[0];
    // ⛔ NOT "something was sent": the package must carry BOTH views and name the open one. A
    // `settle` that replaced the package would pass the probe above and lose Home here.
    const pack = sent?.kind === "SaveLayout" ? unpack({ state: "Package", bytes: sent.value }) : null;
    expect(pack).toEqual({ view: "work", layouts: { home, work } });
  });

  it("does not count its own save coming back, and does count any other package (D89)", () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    layout.settle({ marker: "mine" } as never);
    const sent = bridge.sent[0];
    const echo = sent?.kind === "SaveLayout" ? sent.value : [];
    expect(echo.length).toBeGreaterThan(0);
    layout.receive({ kind: "Layout", value: { state: "Package", bytes: echo } });
    // ⛔ THE ECHO OF DECISION 13: what the core holds is what we sent. Nothing arrived.
    expect(layout.arrivals).toBe(0);
    // A different package -- the welcome, or the OLD one after a write that did not stick -- did.
    const theirs = { marker: "theirs" } as never;
    layout.receive({ kind: "Layout", value: { state: "Package", bytes: [...pack_({ view: "compact", layouts: { compact: theirs } })] } });
    expect(layout.arrivals).toBe(1);
    expect(layout.view).toBe("compact");
    expect(layout.saved).toEqual({ view: "compact", layouts: { compact: theirs } });
  });
});'''

DOCK_TS = r'''import { createDockview, themeAbyss, type DockviewApi, type SerializedDockview } from "dockview-core";
import { watch } from "vue";

import { componentFor, placeholderParams } from "../panels/registry";
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
    if (Object.keys(panel.params ?? {}).length === 0) {
      panel.api.updateParameters(placeholderParams(panel.id));
    }
  }
}
'''

FRAME_SCRIPT = r'''import { onMounted, ref } from "vue";

import { createDock } from "./dock";
import { useLayout, type ViewName } from "../stores/layout";

import Band from "./Band.vue";
import ViewBar from "./ViewBar.vue";

const host = ref<HTMLElement | null>(null);
const layout = useLayout();

onMounted(() => {
  if (host.value !== null) createDock(host.value);
});

function switchTo(view: ViewName): void {
  // ⛔ ONE LINE, AND THE DOCK FOLLOWS (D89): the open view lives in the store and `createDock`
  // watches it, so the bar, the keyboard of task 14 and a package from the core all take the same
  // path -- and none of them saves a view for merely showing it (decision 11).
  layout.view = view;
}
'''

FRAME_TEST_IMPORTS_OLD = r'''import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";

import { i18n } from "../i18n";
import { PANEL_TYPES, componentFor, isModule, placeholderParams } from "../panels/registry";
import { VIEWS } from "../panels/views";
import Placeholder from "../panels/Placeholder.vue";
import { useConnection } from "../stores/connection";

beforeEach(() => {'''

FRAME_TEST_IMPORTS_NEW = r'''import { mount } from "@vue/test-utils";
import { createDockview, type DockviewApi, type SerializedDockview } from "dockview-core";
import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";
import { nextTick } from "vue";

import { i18n } from "../i18n";
import { PANEL_TYPES, componentFor, isModule, placeholderParams } from "../panels/registry";
import { VIEWS } from "../panels/views";
import Placeholder from "../panels/Placeholder.vue";
import type { IpcMessage } from "../schema/messages";
import { useConnection } from "../stores/connection";
import { pack_, useLayout, type LayoutPack } from "../stores/layout";
import { createFakeBridge, type FakeBridge } from "../transport/fakeBridge";

import { createDock } from "./dock";

beforeEach(() => {'''

DOCK_DESCRIBE = r'''function host(): HTMLElement {
  const element = document.createElement("div");
  document.body.append(element);
  return element;
}

/** A grid of the size the views were generated at (Passo 13). */
function grid(): DockviewApi {
  const api = createDockview(host(), { createComponent: ({ name }) => componentFor(name)() });
  api.layout(1600, 1000);
  return api;
}

/** A layout the owner could have saved under Home: ONE panel, which no shipped view has. */
function ownersHome(): SerializedDockview {
  const api = grid();
  api.addPanel({ id: "status", component: "status", title: "status", params: placeholderParams("status") });
  return api.toJSON();
}

function packageFromTheCore(pack: LayoutPack): IpcMessage {
  return { kind: "Layout", value: { state: "Package", bytes: [...pack_(pack)] } };
}

/** The ids on the grid, sorted: the oracle of WHICH view is showing, blind to sizes (E4). */
function showing(api: DockviewApi): string[] {
  return api.panels.map((panel) => panel.id).sort();
}

function saves(bridge: FakeBridge): number {
  return bridge.sent.filter((message) => message.kind === "SaveLayout").length;
}

/** Lets Vue's watchers and `dockview`'s buffered `onDidLayoutChange` run. */
async function flush(): Promise<void> {
  await nextTick();
  await new Promise((resolve) => setTimeout(resolve, 0));
}

/** ⛔ THESE MOUNT A GRID, and say so: if Passo 3 measured that `dockview` does not run under
 * jsdom, this `describe` is what moves to the reviewer in the browser (the table of Passo 3), and
 * the rest of the file stays. */
describe("the dock", () => {
  it("shows a saved view under its own name, the shipped one under another, and saves neither (D80, D89)", async () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    const home = ownersHome();
    const api = createDock(host());
    api.layout(1600, 1000);
    // ⛔ THE PACKAGE ARRIVES AFTER THE DOCK IS UP, as the welcome does in `main.ts` (D89): before,
    // it updated the store and nothing showed it.
    layout.receive(packageFromTheCore({ view: "home", layouts: { home } }));
    await flush();
    expect(showing(api)).toEqual(["status"]);
    layout.view = "work";
    await flush();
    // ⛔ THE SHIPPED LAVORO, NOT HOME'S LAYOUT UNDER THE LAVORO TAB: row 6 of §2 of the north star
    // has a saved view win over the default BY NAME, and Lavoro was never saved.
    expect(showing(api)).toEqual(Object.keys(VIEWS.work.panels ?? {}).sort());
    layout.view = "home";
    await flush();
    expect(showing(api)).toEqual(["status"]);
    // ⛔ AND NOTHING WAS SAVED: showing a view is not changing it, and the shipped Lavoro must not
    // reach the archive for having been looked at (decision 11).
    expect(saves(bridge)).toBe(0);
  });

  it("saves nothing when the window closes untouched, and once when it closes changed (D81)", async () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    const api = createDock(host());
    await flush();
    window.dispatchEvent(new Event("beforeunload"));
    // ⛔ THE FIRST DIRECTION: an untouched gui closing must not copy the shipped Home into the
    // archive (decision 11) -- before D81 it did, at every first close.
    expect(saves(bridge)).toBe(0);
    api.getPanel("costs")?.api.close();
    window.dispatchEvent(new Event("beforeunload"));
    expect(saves(bridge)).toBe(1);
    await flush();
    // ⛔ AND THE BUFFERED `onDidLayoutChange` THAT FOLLOWS DOES NOT SAVE IT AGAIN: the baseline
    // moved with the save.
    expect(saves(bridge)).toBe(1);
  });
});

'''

PASSO17_NOTE_OLD = r'''⚠️ **Se il Passo 3 ha misurato che `dockview` NON regge `jsdom`**, questo file resta com'è — nessuna delle sue
sonde monta una griglia — e ciò che manca passa al revisore nel browser, con la voce d'errata che lo dice.
'''
PASSO17_NOTE_NEW = r'''⚠️ **Se il Passo 3 ha misurato che `dockview` NON regge `jsdom`**, il solo `describe("the dock")` — l'unico che monta
una griglia, e lo dice di sé — passa al revisore nel browser, con la voce d'errata che lo dice; il resto del file resta
com'è. ⛔ Qui stava *«nessuna delle sue sonde monta una griglia»*, vero fino a **D80**, **D81** e **D89** (revisione del
piano intero, 2026-09-15): le tre proprietà nuove — una vista salvata vince **per nome**, la chiusura non copia una vista
mai toccata nell'archivio, il dock mostra un pacchetto arrivato **dopo** — non si provano senza una griglia.
'''

CRITERIA_NEW = r'''- [ ] ⛔ **la disposizione è PER VISTA e il dock SEGUE lo store (D80, D89):** `grep -c 'layouts: Partial<Record<ViewName, SerializedDockview>>' gui/src/stores/layout.ts` → **1**; `grep -c 'watch(\[' gui/src/frame/dock.ts` → **1**; `grep -c 'apply(' gui/src/frame/Frame.vue` → **0**; le due sonde di `describe("the dock")` in `frame.test.ts` e le due nuove di `describe("the layout")` in `stores.test.ts` sono verdi — le prime due **se il Passo 3 ha misurato verde**, altrimenti stanno nella sua voce d'errata, col revisore nel browser
- [ ] ⛔ **`beforeunload` non copia una vista mai toccata nell'archivio (D81):** la sonda `saves nothing when the window closes untouched` è verde in **entrambe** le metà — zero prima, uno dopo un `close()`, e ancora uno dopo l'evento bufferizzato
'''

# ============================================================================================ task 13
def task13(seg):
    # Files (R10-9)
    seg = sub(seg, "- Create: `gui/src/panels/views/index.ts`, `gui/src/panels/views/views.test.ts` (**LF**)\n",
              "- Create: `gui/src/panels/views/index.ts`, `gui/src/panels/views/generate-views.test.ts` (**LF**) — il generatore delle tre viste, **saltato** senza `REGENERATE_VIEWS=1` (R10-9). ⚠️ Qui stava `views.test.ts`, che **nessun passo detta**: la sonda sulle tre viste vive in `frame.test.ts` (Passo 17)\n")
    # Interfaces: connection (R10-10)
    seg = sub(seg, "`hello(): void`, `retry(): void`, `receive(message: IpcMessage): void`\n",
              "`hello(): void`, `retry(): void`, `receive(message: IpcMessage): void`; e, fuori dallo store, `type Phase = \"waiting\" | \"connected\" | \"stale\"` (R10-10)\n")
    # Interfaces: layout (D80, D89)
    seg = sub(seg, "`receive(message: IpcMessage): void`, `settle(pack: LayoutPack): void`; e, fuori dallo store, `pack_(pack: LayoutPack): Uint8Array`",
              "`receive(message: IpcMessage): void`, `saved: LayoutPack | null`, `arrivals: number`, `settle(layout: SerializedDockview): void` — ⛔ **fonde** la vista aperta nel pacchetto tenuto (**D80**; qui stava `settle(pack: LayoutPack)`), e `arrivals` conta i pacchetti che non sono l'eco del proprio `SaveLayout` (**D89**); e, fuori dallo store, `pack_(pack: LayoutPack): Uint8Array`")
    seg = sub(seg, "`interface LayoutPack { view: ViewName; layout: SerializedDockview }`\n",
              "`interface LayoutPack { view: ViewName; layouts: Partial<Record<ViewName, SerializedDockview>> }` — ⛔ **una disposizione PER VISTA, D80**: qui stava `layout: SerializedDockview`, una sola per tutte le viste\n")
    # Interfaces: VueContent + BigTab + stamp (R10-10)
    seg = sub(seg, "  - `gui/src/frame/VueContent.ts` — `class VueContent implements IContentRenderer`, costruita con un `Component` di Vue\n",
              "  - `gui/src/frame/VueContent.ts` — `class VueContent implements IContentRenderer`, costruita con un `Component` di Vue\n"
              "  - `gui/src/frame/BigTab.ts` — `class BigTab implements ITabRenderer` — ⛔ **il 14 lo consuma E lo riscrive** (R10-10)\n"
              "  - `gui/src/schema/stamp.ts` — `buildStamp(): U64` (**D52**, R10-10)\n")
    # Interfaces: dock (D80, D89)
    seg = sub(seg, "  - `gui/src/frame/dock.ts` — `createDock(host: HTMLElement): DockviewApi`, `apply(api: DockviewApi, view: string, pack: LayoutPack | null): void`, `canonical(value: unknown): unknown`\n",
              "  - `gui/src/frame/dock.ts` — `createDock(host: HTMLElement): DockviewApi`, `apply(api: DockviewApi, view: ViewName, pack: LayoutPack | null): void` — legge `pack?.layouts[view] ?? VIEWS[view]` (**D80**; qui stava `view: string`), `canonical(value: unknown): unknown`. ⛔ **E il dock SEGUE lo store (D89):** `createDock` osserva `view` e `arrivals` di `useLayout` e mostra la vista da sé — la barra scrive `layout.view` e basta, e un pacchetto che il core manda **dopo** che il dock è su (l'accoglienza; o il pacchetto **vecchio** dopo una scrittura non riuscita, decisione 13) viene mostrato invece di restare nello store\n")
    # Interfaces: the re-census line
    seg = sub(seg, "`PanelType` **non** ha un campo `built`, e il suo `who` è un **numero**\n",
              "`PanelType` **non** ha un campo `built`, e il suo `who` è un **numero**. ✅ **Ricensito di nuovo il 2026-09-15, alla revisione del piano intero (R10-10):** mancavano tre `export` che i Passi dettano — `BigTab`, `buildStamp`, `Phase` — e il 14 consuma il primo; le righe di `layout.ts` e `dock.ts` sono riscritte con **D80** e **D89**\n")
    # Passo 4, tokens.css (D76)
    seg = sub(seg, "ONE file (§6a of the milestone-2\n   design), because a kit is extracted at the SECOND occurrence and milestone 2 has none.",
              "ONE file (§6a of the sub-project 2\n   design), because a kit is extracted at the SECOND occurrence and sub-project 2 has none.")
    # Passo 9, layout.ts rewritten whole (D80, D89)
    seg = between(seg,
                  "```ts\nimport type { SerializedDockview } from \"dockview-core\";\nimport { defineStore } from \"pinia\";\nimport { ref } from \"vue\";\n\nimport type { IpcMessage, LayoutState } from \"../schema/messages\";\n",
                  "\n⚠️ **`pack_` col trattino basso in coda e non `pack`:**",
                  "```ts\n" + LAYOUT_TS + "```\n")
    seg = sub(seg, "`pack` è già il nome del parametro in `settle`, e due cose\ncon lo stesso nome",
              "`pack` è già il nome del pacchetto che `settle` costruisce, e due cose\ncon lo stesso nome")
    # Passo 10, the layout describe rewritten (D80, D89)
    seg = between(seg, "describe(\"the layout\", () => {\n",
                  "\n```\n\n- [ ] **Passo 11: il ponte fra Vue e i pannelli, e la presa grande**",
                  LAYOUT_DESCRIBE)
    # Passo 11, BigTab comment (D75: no dated recall in a file that is born)
    seg = sub(seg, "keeps the title it was given. Richiamo del\n    // 2026-09-15, P-95.\n",
              "keeps the title it was given (P-95 of the\n    // part-2 plan, found at the pre-check of task 14).\n")
    # Passo 12 (D76)
    seg = sub(seg, "every module the milestone\n// does not build says", "every module the sub-project\n// does not build says")
    seg = sub(seg, "// ⚠️ ONLY WHAT IS ALIVE IN MILESTONE 2 (decision 16", "// ⚠️ ONLY WHAT IS ALIVE IN SUB-PROJECT 2 (decision 16")
    seg = sub(seg, " * - a module type that milestone 2 does not build -> the placeholder", " * - a module type that sub-project 2 does not build -> the placeholder")
    # Passo 14, dock.ts rewritten whole (D80, D81, D89, D76)
    seg = between(seg,
                  "```ts\nimport { createDockview, themeAbyss, type DockviewApi, type SerializedDockview } from \"dockview-core\";\n\nimport { componentFor, placeholderParams }",
                  "\n⛔ **Il giro dei parametri dopo `fromJSON` non è una pezza:**",
                  "```ts\n" + DOCK_TS + "```\n")
    # Passo 15, Frame.vue script (D89)
    seg = between(seg,
                  "```vue\n<script setup lang=\"ts\">\nimport { onMounted, ref } from \"vue\";\n\nimport { apply, createDock } from \"./dock\";\n",
                  "</script>\n\n<template>\n  <div class=\"frame\">\n",
                  "```vue\n<script setup lang=\"ts\">\n" + FRAME_SCRIPT)
    # Passo 16, main.ts (D76)
    seg = sub(seg, " * ⛔ THE FAKE BRIDGE IS WHAT MILESTONE 2's SPA RUNS AGAINST IN A BROWSER", " * ⛔ THE FAKE BRIDGE IS WHAT SUB-PROJECT 2's SPA RUNS AGAINST IN A BROWSER")
    seg = sub(seg, "(§8 of the milestone-2 design).", "(§8 of the sub-project 2 design).")
    # Passo 17, frame.test.ts: imports, the dock describe, the note under the block
    seg = sub(seg, FRAME_TEST_IMPORTS_OLD, FRAME_TEST_IMPORTS_NEW)
    seg = sub(seg, "\ndescribe(\"the band\", () => {\n", "\n" + DOCK_DESCRIBE + "describe(\"the band\", () => {\n")
    seg = sub(seg, PASSO17_NOTE_OLD, PASSO17_NOTE_NEW)
    # Criteria
    seg = sub(seg, "gui/src/stores/stores.test.ts` → **più di due**\n", "gui/src/stores/stores.test.ts` → **più di due**\n" + CRITERIA_NEW)
    return seg


t = scoped(t, "\n## Compito 13:", "\n## Compito 14:", task13)

# ============================================================================================ task 15 (R7-7)
t = scoped(t, "\n## Compito 15:", "\n## Compito 16:", lambda seg: sub(
    seg, "le diciotto chiavi `modules.*` di `gui/src/locales/it.json` (compito **14**)",
    "le diciotto chiavi `modules.*` di `gui/src/locales/it.json` (compito **13**, Passo 12 — ⚠️ qui stava «14», R7-7)"))

# ============================================================================================ the head: D89 and the position row
D89 = ("| **D89** | ⛔ **il dock SEGUE lo store**: `createDock` osserva `view` e `arrivals` di `useLayout` — la barra scrive `layout.view` e basta; un pacchetto che il core manda **dopo** che il dock è su viene mostrato; l'**eco** del proprio `SaveLayout` (decisione 13: il core risponde con ciò che tiene) non conta come arrivo, perché lo store confronta i byte con quelli che ha mandato; e mostrare una vista **azzera la baseline** del `settle`, così guardare una vista non la salva | "
       "revisione del piano intero, 2026-09-15, scrivendo D80 (coordinatore; R6 caduto): il 13 dettava `apply` una volta sola in `createDock`, cioè **prima** del `Hello` di `main.ts` — e `layout.receive` del `Layout` dell'accoglienza aggiornava lo store senza che nulla lo mostrasse: la disposizione salvata **non compariva mai** all'avvio; e col `switchTo` che rilanciava `apply` da `Frame.vue`, il `settle` che seguiva copiava la vista spedita nell'archivio, contro la decisione 11 del coordinatore della stella. Compilava e passava tutte le sonde del 13 |\n")
t = sub(t, "\n\n**La baseline di partenza, misurata il 2026-09-11 su `42b50d8` e da NON citare nei compiti:**",
        "\n" + D89 + "\n**La baseline di partenza, misurata il 2026-09-11 su `42b50d8` e da NON citare nei compiti:**")
t = sub(t, "non mostra nulla — con `npm run build` verde | uno | ⬜ |",
        "non mostra nulla — con `npm run build` verde. ⛔ **La disposizione salvata è PER VISTA, e il dock SEGUE lo store — D80, D89, 2026-09-15**: qui il `Layout` dell'accoglienza non veniva mai mostrato | uno | ⬜ |")

# ============================================================================================ ledger
lraw = io.open(LEDGER, encoding="utf-8", newline="").read()
assert "\r\n" not in lraw
l = lraw

l = sub(l, "**Sessione 14 (2026-09-15):** compiti 11 e 12 applicati (✅).",
        "**Sessione 14 (2026-09-15):** compiti 11 e 12 applicati (✅). **Sessione 15 (2026-09-15):** compito 13 applicato (✅) — con **D89** nuova (il dock segue lo store), R7-7 applicata al 15, e la cascata sul `Frame.vue` del 14 registrata come riga del 14.")
l = sub(l, "R3-18, R5-15, R5-16 |\n\n## Per compito",
        "R3-18, R5-15, R5-16 |\n"
        "| D89 | il dock SEGUE lo store: `createDock` osserva `view` e `arrivals` di `useLayout` e mostra la vista da sé (`show()` azzera la baseline del `settle`); lo store conta gli **arrivi** che non sono l'eco del proprio `SaveLayout` (decisione 13: il core risponde con ciò che tiene — i byte si confrontano con quelli mandati); `Frame.switchTo` è `layout.view = view`. Trovato scrivendo D80: il `Layout` dell'accoglienza arrivava **dopo** l'unico `apply` e non veniva mai mostrato; e il `settle` dopo uno `switchTo` copiava la vista spedita nell'archivio (decisione 11) | coordinatore, ondata del 13 (R6 caduto) |\n"
        "\n## Per compito")

OLD_C13 = """### Compito 13 — NON RIVISTO IN PROFONDITÀ (R6 caduto): resta da fare
- R9b-11 (D80) LayoutPack per vista ⬜ · R9b-12 (D81) ⬜ · R10-9 Files `generate-views.test.ts` ⬜ · R10-10 Produces: `BigTab`, `buildStamp`, `Phase` ⬜ · R7-7 (chiavi `modules.*` sono del 13: correggere il 15) ⬜ · D75 `2026-09-15` → `<data>` ⬜
"""
NEW_C13 = """### Compito 13 — NON RIVISTO IN PROFONDITÀ (R6 caduto): le righe note sono applicate, la revisione in profondità resta da fare
- R9b-11 (D80) `LayoutPack { view; layouts: Partial<Record<ViewName, SerializedDockview>> }`; `settle(layout)` **fonde** la vista aperta nel pacchetto tenuto (`saved`); `apply(api, view: ViewName, pack)` legge `pack?.layouts[view] ?? VIEWS[view]`; `unpack` legge le sole tre viste e rifiuta la forma vecchia `{view, layout}`; Interfaces riscritto; la sonda del merge in `stores.test.ts` e la sonda D80 in `frame.test.ts` (salvata sotto `home` → `view = "work"` mostra la spedita di Lavoro → `view = "home"` la salvata, e **zero** `SaveLayout`), con l'oracolo sugli **id dei pannelli** e non sul JSON canonico (E4: la taglia del viewport cambia le misure) ✅
- R9b-12 (D81) `beforeunload` salva solo se `!same(last, now)` e sposta `last`; il doc di `createDock` dichiara che il pannello attivo **è** disposizione (misurato sulla 8.2.0 dello spike, R9b-12; se la 8.3.1 differisce è errata); sonda nelle due direzioni in `frame.test.ts` — chiusa intatta → 0; chiusa dopo il `close()` di un pannello → 1, e l'evento bufferizzato non la ripete ✅
- ⛔ **C13-1 (coordinatore, scrivendo D80 — R6 caduto): il `Layout` dell'accoglienza non veniva MAI mostrato.** `createDock` chiamava `apply` una volta, in `onMounted`, cioè **prima** del `Hello` di `main.ts`; `layout.receive` aggiornava `state` e `view` e nessuno riapplicava. Compilava e passava le sonde. Rimedio, **D89**: il dock **segue lo store** — `watch([view, arrivals])` in `createDock`, `show()` che azzera la baseline; lo store conta gli arrivi che **non** sono l'eco del proprio `SaveLayout` (decisione 13: il core risponde con ciò che tiene; i byte si confrontano con quelli mandati); `Frame.switchTo` diventa `layout.view = view`; la sonda dell'eco in `stores.test.ts`; la sonda D80 di `frame.test.ts` parte da un pacchetto ricevuto **dopo** `createDock` ✅
- R10-9 Files: `views.test.ts` (che nessun passo dettava) → `generate-views.test.ts`, col perché ✅
- R10-10 Produces: `BigTab` (consumato e riscritto dal 14), `buildStamp` (D52), `type Phase`; la riga «ricensito» porta la data della seconda ricensione ✅
- R7-7 applicata **al 15** (la riga del 15, in questa ondata): «(compito **13**, Passo 12 — qui stava «14»)» ✅
- D75: la sola data dettata nel codice del 13 era «Richiamo del 2026-09-15, P-95» nel commento di `BigTab.ts` — tolta: in un file che nasce non c'è nulla da richiamare, resta il puntatore «P-95 of the part-2 plan, found at the pre-check of task 14»; le altre date del 13 sono fatti (P-2, E2, E4) e restano ✅
- D76 «sub-project 2» negli otto punti dei commenti dettati (`tokens.css` ×2, `Placeholder.vue`, `Strip.vue`, `registry.ts`, `dock.ts`, `main.ts` ×2) — non era in questa lista: applicata per coerenza coi compiti 5–12 ✅
- ⚠️ Il capoverso sotto il Passo 17 diceva «nessuna delle sue sonde monta una griglia»: ora `describe("the dock")` la monta e lo dice di sé — se il Passo 3 misura rosso, è quel `describe` a passare al revisore nel browser; due criteri di chiusura nuovi (D80/D89 e D81) ✅
- ⚠️ **Non compilato:** `layout.ts`, `dock.ts`, `Frame.vue` e le sonde sono riscritti sul modello letto (il compito 11 non esiste nel repo, e `dockview` sotto `jsdom` lo misura il Passo 3): chi esegue il 13 li compila per primi, e ogni rosso è una voce d'errata — come il modulo delle sonde del 12
- Attrezzo: `patch_c13.py` accanto a questo file (tocca anche la tabella D, la posizione, la riga del 15 e questo registro)
"""
l = sub(l, OLD_C13, NEW_C13)
l = sub(l, "### Compito 14\n- R7-1 ",
        "### Compito 14\n- ⚠️ **cascata di D80/D89 dal 13 corretto (decisione 78):** il `Frame.vue` ridettato al Passo 10 si ridiffa contro il 13 di adesso — via `apply` e `unpack`, `switchTo` è `layout.view = view`; `api` resta, perché `moveActive` lo usa ⬜\n- R7-1 ")
l = sub(l, " · R7-7 «(compito 13, Passo 12)» ⬜ · ", " · R7-7 «(compito 13, Passo 12)» ✅ (applicata nell'ondata del 13) · ")

# ============================================================================================ write, atomically
for path, content in ((PLAN, t), (LEDGER, l)):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(content)
    os.replace(tmp, path)
print("ok: task 13 patched (plus task 15's R7-7, D89, the position row and the ledger);",
      t.count("\n") - raw.count("\n"), "plan lines added;", l.count("\n") - lraw.count("\n"), "ledger lines added")

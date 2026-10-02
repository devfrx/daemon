import type { SerializedDockview } from "dockview-core";
import { defineStore } from "pinia";
import { computed, ref } from "vue";

import type { IpcMessage, LayoutState } from "../schema/messages";
import { isThemeChoice, type ThemeChoice } from "../tokens/theme";
import type { Bridge } from "../transport/bridge";

export type ViewName = "home" | "work" | "compact";

/**
 * ⛔ ONE LAYOUT PER VIEW, AND THE PACKAGE CARRIES THEM ALL (D80). The opening paragraph of §2 of the
 * north star makes the layout "which view is open, FOR EVERY VIEW where the panels are", and row 6 has a
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
  /** ⛔ OPTIONAL, AND THAT IS THE COMPATIBILITY (answer 16 of the design system): a package written before
   * the field existed opens as `system`, and the core keeps the bytes without opening them -- the kernel
   * does not change. */
  theme?: ThemeChoice;
  /** The views the owner saved under a name (the (d); §2 of the north star). ⛔ OPTIONAL, AND A LIST OF THEIR
   * OWN, not keys of `layouts` (D3 of the design-system plan): `view` stays one of the three, so a package
   * written by this build opens in an older one on the view of always. */
  named?: NamedView[];
  /** The named view that is open, by name; absent while one of the three is. */
  openNamed?: string;
}

export interface NamedView {
  name: string;
  layout: SerializedDockview;
}

const VIEWS: readonly ViewName[] = ["home", "work", "compact"];

function isViewName(value: unknown): value is ViewName {
  return typeof value === "string" && (VIEWS as readonly string[]).includes(value);
}

/** Two names are the same name when they differ only in the spaces around them or in case: side by side in the overview
 * they would read as one (D12 of the design-system plan, chosen by the owner on 2026-09-23). */
function sameName(a: string, b: string): boolean {
  return a.trim().toLocaleLowerCase("it") === b.trim().toLocaleLowerCase("it");
}

/** The named views of a package. ⛔ AN ENTRY WITHOUT A NAME OR A LAYOUT IS DROPPED, AND SO IS A SECOND ENTRY UNDER A NAME
 * ALREADY READ -- the first stays: two views never share a name (D4). */
function readNamed(value: unknown): NamedView[] {
  if (!Array.isArray(value)) return [];
  const read: NamedView[] = [];
  for (const entry of value) {
    if (typeof entry !== "object" || entry === null) continue;
    const { name, layout } = entry as { name?: unknown; layout?: unknown };
    if (typeof name !== "string" || name.trim() === "" || typeof layout !== "object" || layout === null) continue;
    if (read.some((kept) => sameName(kept.name, name))) continue;
    read.push({ name, layout: layout as SerializedDockview });
  }
  return read;
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
 * older build. `unpack` returns `null` on a package it does not recognise, and the caller falls
 * back to the committed views -- the same shape as row 8 of §2, where a panel pointing at a type
 * that is gone says so and closes. ⛔ A LAYOUT INSIDE A PACKAGE IS KEPT UNREAD: only `dockview`
 * knows which layouts it can open, so one it cannot open loses to the committed view where the dock
 * opens it (`apply` in `dock.ts`, AUD-536 and AUD-543 of the audit of 2026-09-30), and the overview
 * draws it empty (E90 of the design-system plan).
 */
export const useLayout = defineStore("layout", () => {
  const state = ref<LayoutState>({ state: "Nothing" });
  const view = ref<ViewName>("home");
  /** The named view on screen, or `null` while one of the three is: the dock watches it as it watches `view`. */
  const openNamed = ref<string | null>(null);
  /** The package we hold: the last one the core sent, or our own last save while its echo is in
   * flight. What `apply` reads. */
  const saved = ref<LayoutPack | null>(null);
  /** ⛔ HOW MANY PACKAGES ARRIVED THAT ARE NOT THE ECHO OF OUR OWN SAVE (D89): the dock watches
   * it and shows what arrived. A counter and not an event, because the dock is a Vue watcher. */
  const arrivals = ref(0);
  let wire: Bridge | null = null;
  /** ⛔ EVERY SAVE SENT SINCE THE LAST ARRIVAL WHOSE ANSWER HAS NOT COME, OLDEST FIRST, and not the last one alone (AUD-542
   * of the audit of 2026-09-30): a settle and a theme chosen at once put two in flight, and with one slot the answer to
   * the first was shown as a package of someone else's -- the theme back to `system`, the screen back a move.
   * ⚠️ A SAVE THAT GETS NO ANSWER -- one no core hears, or one the core refuses without a word before the handshake --
   * stays here until the next arrival empties the list: the cost is the bytes of the saves made while no core listens. */
  let inFlight: (readonly number[])[] = [];

  function attach(bridge: Bridge): void {
    wire = bridge;
  }

  function receive(message: IpcMessage): void {
    if (message.kind !== "Layout") return;
    state.value = message.value;
    // ⛔ THE CORE ANSWERS EVERY `SaveLayout` WITH WHAT IT HOLDS, IN ORDER (decision 13, task 7), so a
    // package equal to a save in flight is our own save coming back: nothing new, nothing to show --
    // we hold that one, or a later one built on it. It answers that save, and every older one has had
    // its answer or will have none: they leave the list together.
    if (message.value.state === "Package") {
      const bytes = message.value.bytes;
      const echo = inFlight.findIndex((sent) => sameBytes(bytes, sent));
      if (echo !== -1) {
        inFlight = inFlight.slice(echo + 1);
        return;
      }
    }
    // Anything else -- the welcome, the OLD package after a write that did not stick, a package
    // another build wrote -- replaces what we hold, and the dock shows it (D89). ⛔ AND THE SAVES STILL
    // IN FLIGHT ARE NO LONGER ECHOES: they were built on what we held before, so each answer to come
    // says what the core holds after it, and is shown in its turn -- the last leaves the gui holding
    // what the core holds.
    inFlight = [];
    saved.value = unpack(message.value);
    if (saved.value !== null) view.value = saved.value.view;
    // ⛔ AND THE NAMED VIEW THAT IS OPEN (R3-19 of the design-system review), or none -- ALSO WHEN NOTHING
    // READABLE ARRIVED (E75 of the design-system plan): the dock then shows the view of always, and a name left
    // open would send the next move nowhere.
    openNamed.value = saved.value?.openNamed ?? null;
    arrivals.value += 1;
  }

  /** The package we hold, with what is on screen: the view of always, and the named view if one is open. */
  function onScreen(): LayoutPack {
    const pack: LayoutPack = { ...(saved.value ?? { layouts: {} }), view: view.value };
    if (openNamed.value === null) delete pack.openNamed;
    else pack.openNamed = openNamed.value;
    return pack;
  }

  /** ⛔ AUTOMATIC, NOT A BUTTON (decision 12): when the layout settles, and when the window
   * closes. The cadence is the gui's -- it is presentation, not a kernel decision.
   *
   * ⛔ AND IT MERGES (D80): the open view's entry is replaced and the other views keep theirs. A
   * `settle` that replaced the whole package lost every view but the open one. The rest of the package
   * -- the theme, the named views -- is kept whole. */
  function settle(layout: SerializedDockview): void {
    const pack = onScreen();
    const open = pack.openNamed;
    if (open === undefined) {
      keep({ ...pack, layouts: { ...pack.layouts, [view.value]: layout } });
      return;
    }
    // ⛔ A MOVE IN A NAMED VIEW GOES TO THAT VIEW (R3-19): writing `layouts[view]` here would overwrite the view of
    // always -- Home or Lavoro -- with the named one, without an error.
    keep({ ...pack, named: (pack.named ?? []).map((entry) => (entry.name === open ? { name: open, layout } : entry)) });
  }

  /** One of the three views on screen, which closes the named one. ⛔ SHOWING IS NOT SAVING (decision 11): the choice
   * reaches the package at the next settle. */
  function showView(next: ViewName): void {
    openNamed.value = null;
    view.value = next;
  }

  /**
   * A named view shown (the (d)). ⛔ ONLY BY A NAME THE LIST HOLDS (E90 of the design-system plan): with an unknown name
   * the dock falls back to the view of always, and `settle` writes the next move into a view the package does not have
   * -- the loss of E75, by another door. Refused, and nothing changes: the twin of `showView`.
   */
  function showNamed(name: string): boolean {
    if (!(saved.value?.named ?? []).some((entry) => entry.name === name)) return false;
    openNamed.value = name;
    return true;
  }

  /**
   * The layout on screen saved under a name, and opened ("Salva questa vista", the (d)). ⛔ SAVED AT ONCE, like a theme:
   * it is a decision. ⛔ A NAME ALREADY TAKEN IS REFUSED, NOT OVERWRITTEN (D4): overwriting would lose a view in silence.
   * Taken are the named views' names and `shown` -- the names the frame shows for the three views, which are the locale's
   * words, and a store reads none.
   */
  function saveNamed(name: string, layout: SerializedDockview, shown: readonly string[]): "saved" | "empty" | "taken" {
    const wanted = name.trim();
    if (wanted === "") return "empty";
    const taken = [...shown, ...(saved.value?.named ?? []).map((entry) => entry.name)];
    if (taken.some((other) => sameName(other, wanted))) return "taken";
    openNamed.value = wanted;
    keep({ ...onScreen(), named: [...(saved.value?.named ?? []), { name: wanted, layout }] });
    return "saved";
  }

  /** The theme of the package, and `system` when it has none (answer 16). */
  const theme = computed<ThemeChoice>(() => saved.value?.theme ?? "system");

  /** ⛔ SAVED AT ONCE, NOT AT THE NEXT SETTLE: a choice made in Impostazioni is a decision, not a movement of
   * panels, and closing the window right after it must not lose it. */
  function chooseTheme(choice: ThemeChoice): void {
    keep({ ...onScreen(), theme: choice });
  }

  function keep(pack: LayoutPack): void {
    saved.value = pack;
    if (wire === null) return;
    const bytes = [...pack_(pack)];
    inFlight.push(bytes);
    wire.send({ kind: "SaveLayout", value: bytes });
  }

  return { state, view, openNamed, saved, arrivals, theme, attach, receive, settle, showView, showNamed, saveNamed, chooseTheme };
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
    // name is another build's, and is neither shown nor kept -- row 8 of §2, for views. ⚠️ The
    // layout under a known name is any object, kept unread: `dockview` judges it, in `apply`.
    for (const name of VIEWS) {
      const layout = held[name];
      if (typeof layout === "object" && layout !== null) layouts[name] = layout as SerializedDockview;
    }
    const pack: LayoutPack = { view: candidate.view, layouts };
    // ⛔ A CHOICE THIS BUILD DOES NOT KNOW IS READ AS ABSENT, and the package still opens: the layouts in it are
    // worth more than a word we cannot read.
    const theme = (candidate as { theme?: unknown }).theme;
    if (isThemeChoice(theme)) pack.theme = theme;
    const named = readNamed((candidate as { named?: unknown }).named);
    if (named.length > 0) pack.named = named;
    // ⛔ AN OPEN NAME THE LIST DOES NOT HOLD IS READ AS ABSENT, and the view of always opens.
    const open = (candidate as { openNamed?: unknown }).openNamed;
    if (typeof open === "string" && named.some((entry) => entry.name === open)) pack.openNamed = open;
    return pack;
  } catch {
    // ⛔ A PACKAGE THAT DOES NOT PARSE IS NOT AN ERROR TO SHOW: it is an old build's layout, and
    // the answer is the committed views. Row 8 of §2 asks the gui to cope, not to complain.
    return null;
  }
}

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
    // ⛔ THE REST OF THE PACKAGE IS KEPT: a settle that rebuilt it from `view` and `layouts` alone would drop
    // the theme -- and, from task 7, the named views -- at the first move of a panel.
    keep({
      ...(saved.value ?? {}),
      view: view.value,
      layouts: { ...(saved.value?.layouts ?? {}), [view.value]: layout },
    });
  }

  /** The theme of the package, and `system` when it has none (answer 16). */
  const theme = computed<ThemeChoice>(() => saved.value?.theme ?? "system");

  /** ⛔ SAVED AT ONCE, NOT AT THE NEXT SETTLE: a choice made in Impostazioni is a decision, not a movement of
   * panels, and closing the window right after it must not lose it. */
  function chooseTheme(choice: ThemeChoice): void {
    keep({ layouts: {}, ...(saved.value ?? {}), view: view.value, theme: choice });
  }

  function keep(pack: LayoutPack): void {
    saved.value = pack;
    const bytes = [...pack_(pack)];
    sent = bytes;
    wire?.send({ kind: "SaveLayout", value: bytes });
  }

  return { state, view, saved, arrivals, theme, attach, receive, settle, chooseTheme };
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
    // ⛔ A CHOICE THIS BUILD DOES NOT KNOW IS READ AS ABSENT, and the package still opens: the layouts in it are
    // worth more than a word we cannot read.
    const theme = (candidate as { theme?: unknown }).theme;
    return isThemeChoice(theme) ? { view: candidate.view, layouts, theme } : { view: candidate.view, layouts };
  } catch {
    // ⛔ A PACKAGE THAT DOES NOT PARSE IS NOT AN ERROR TO SHOW: it is an old build's layout, and
    // the answer is the committed views. Row 8 of §2 asks the gui to cope, not to complain.
    return null;
  }
}

import { mount } from "@vue/test-utils";
import { createDockview, type DockviewApi, type SerializedDockview } from "dockview-core";
import { createPinia, setActivePinia } from "pinia";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { nextTick } from "vue";

import { i18n } from "../i18n";
import { VRAM_POLICY } from "../panels/functions";
import { PANEL_TYPES, componentFor, isModule, placeholderParams } from "../panels/registry";
import { VIEWS } from "../panels/views";
import Placeholder from "../panels/Placeholder.vue";
import type { IpcMessage } from "../schema/messages";
import { useConnection } from "../stores/connection";
import { useCore } from "../stores/core";
import { useDrawer } from "../stores/drawer";
import { useInvoke } from "../stores/invoke";
import { pack_, unpack, useLayout, type LayoutPack } from "../stores/layout";
import { shownTheme } from "../tokens/theme";
import { createFakeBridge, type FakeBridge } from "../transport/fakeBridge";

import Frame from "./Frame.vue";
import { createDock, harnessTheme } from "./dock";

beforeEach(() => {
  setActivePinia(createPinia());
});

/** Every `component` a committed view names must be something the registry can build. ⛔ NO DOM
 * NEEDED, and that is deliberate: this is the property that actually protects the frame, and it
 * must hold whether or not `dockview` runs under jsdom. */
describe("the three committed views", () => {
  it("name only components the registry knows", () => {
    for (const [name, view] of Object.entries(VIEWS)) {
      const panels = Object.values(view.panels ?? {});
      expect(panels.length, `${name} has no panels`).toBeGreaterThan(0);
      for (const panel of panels) {
        const component = panel.contentComponent ?? panel.id;
        expect(isModule(component) || component === "strip", `${name}: ${component}`).toBe(true);
      }
    }
  });

  it("each carry the strip", () => {
    for (const [name, view] of Object.entries(VIEWS)) {
      expect(Object.keys(view.panels ?? {}), name).toContain("strip");
    }
  });
});

describe("the registry", () => {
  it("gives a placeholder that says who fills an unbuilt module", () => {
    const type = PANEL_TYPES[0];
    expect(type).toBeDefined();
    const wrapper = mount(Placeholder, {
      global: { plugins: [i18n] },
      props: { params: placeholderParams(type!.name) },
    });
    expect(wrapper.text()).toContain(String(type!.who));
  });

  it("says «niente ancora» on the nucleus, and still who fills it (R6-14)", () => {
    // ⛔ ROW HOME OF "IL MODELLO DELLA GUI": the nucleus has a phrase of its own, the other tiles
    // say who fills them. A placeholder that treated the centre of Home like any tile passed every
    // probe and read "arriva col sotto-progetto 6" in the browser.
    const params = placeholderParams("knowledge");
    const wrapper = mount(Placeholder, { global: { plugins: [i18n] }, props: { params } });
    expect(wrapper.text()).toContain(i18n.global.t("placeholder.nucleus"));
    expect(wrapper.text()).toContain(String(params.who));
  });

  it("says a type that is GONE is gone, does not promise a sub-project, and closes", async () => {
    // ⛔ ROW 8 OF §2 OF THE NORTH STAR, and the two cases are NOT the same: a saved package can
    // point at a type a later build removed, and telling the user to wait for a sub-project that
    // will never fill it would be worse than saying nothing.
    const params = placeholderParams("a-type-that-never-existed");
    expect(params).toEqual({ missing: true });
    let closed = false;
    const wrapper = mount(Placeholder, {
      global: { plugins: [i18n] },
      props: { params, api: { close: () => { closed = true; } } as never },
    });
    expect(wrapper.text()).toContain(i18n.global.t("placeholder.missing"));
    // ⛔ THE SECOND HALF OF ROW 8: it says so AND it closes. A probe that only read the words
    // would be green on a tile that never goes away.
    await wrapper.get("button").trigger("click");
    expect(closed).toBe(true);
  });

  it("builds something for every module type, and for the strip", () => {
    for (const type of [...PANEL_TYPES.map((t) => t.name), "strip"]) {
      expect(typeof componentFor(type), type).toBe("function");
    }
  });
});

function host(): HTMLElement {
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

  it("leaves the strip's params alone: only what nobody built gets them from the registry (R6-17)", async () => {
    const bridge = createFakeBridge();
    useLayout().attach(bridge);
    const api = createDock(host());
    await flush();
    // ⛔ THE STRIP CARRIES NO PARAMS AND MUST NOT GET `{ missing: true }`: before R6-17 it did, at
    // every `apply`, and the value entered the package at the first settle -- unseen, because
    // `Strip.vue` ignores its params.
    expect(api.getPanel("strip")?.params ?? {}).toEqual({});
    // And a module type nobody built still carries its own, from the registry.
    expect(api.getPanel("knowledge")?.params).toEqual(placeholderParams("knowledge"));
  });

  it("wears our theme, and hands it to dockview again when the theme on screen changes (design system, section (c))", async () => {
    useLayout().attach(createFakeBridge());
    const where = host();
    const api = createDock(where);
    // ⛔ THE SHELL WEARS OUR CLASS, the one `tokens/dock.css` dresses -- and the abyss theme's is gone (control 15).
    expect(where.querySelector(".dv-shell")?.classList.contains("dockview-theme-harness")).toBe(true);
    expect(where.querySelector(".dockview-theme-abyss")).toBeNull();
    const handed: unknown[] = [];
    const update = api.updateOptions.bind(api);
    api.updateOptions = (options) => {
      handed.push(options.theme?.colorScheme);
      update(options);
    };
    shownTheme.value = "light";
    await flush();
    shownTheme.value = "dark";
    await flush();
    // ⛔ WHAT THE DOCK WAS HANDED, NOT WHAT IT DRAWS: `colorScheme` draws nothing in `dockview-core` 8.3.1 (R3-21).
    expect(handed).toEqual(["light", "dark"]);
  });

  it("refuses a gap that is not a length in px, and builds the theme from the token that is", () => {
    document.documentElement.style.setProperty("--space-3", "0.75rem");
    try {
      // ⛔ `parseFloat` would read 0.75 and shrink the gap without a word.
      expect(() => harnessTheme()).toThrow(/not a length in px/);
    } finally {
      document.documentElement.style.removeProperty("--space-3");
    }
    // ⛔ THE SECOND DIRECTION: the token as the board has it, read from `base.css` (`src/jsdom-setup.ts`).
    expect(harnessTheme()).toMatchObject({ name: "harness", className: "dockview-theme-harness" });
    expect(harnessTheme().gap).toBeGreaterThan(0);
  });

  it("shows the named view that is open, and the view of always once it closes -- saving neither (the (d))", async () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    const api = createDock(host());
    api.layout(1600, 1000);
    // ⛔ TWO NAMED VIEWS, THE OPEN ONE SECOND (E82 of the plan): with one alone, "the one open" reads the same as
    // "the first".
    const first = grid();
    first.addPanel({ id: "costs", component: "costs", title: "costs", params: placeholderParams("costs") });
    layout.receive(packageFromTheCore({ view: "home", layouts: {}, named: [{ name: "Prima", layout: first.toJSON() }, { name: "Revisione", layout: ownersHome() }], openNamed: "Revisione" }));
    await flush();
    expect(showing(api)).toEqual(["status"]);
    layout.showView("home");
    await flush();
    // ⛔ THE SHIPPED HOME, NOT THE REVIEW UNDER ITS NAME: the dock watches `openNamed` as it watches `view`.
    expect(showing(api)).toEqual(Object.keys(VIEWS.home.panels ?? {}).sort());
    expect(saves(bridge)).toBe(0);
  });

  it("saves a group maximized and a group restored, and nothing for showing a view left or opened maximized (E104)", async () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    const api = createDock(host());
    api.layout(1600, 1000);
    await flush();
    /** Whether the last package sent opens Home with a group maximized: `dockview-core` 8.3.1 writes `grid.maximizedNode`,
     * which its public type does not declare (E85 of the plan). */
    const homeMaximized = (): boolean => {
      const last = bridge.sent.filter((message) => message.kind === "SaveLayout").at(-1);
      const pack = last?.kind === "SaveLayout" ? unpack({ state: "Package", bytes: last.value }) : null;
      return (pack?.layouts.home?.grid as { maximizedNode?: unknown } | undefined)?.maximizedNode !== undefined;
    };
    api.getPanel("status")?.group.api.setActive();
    await flush();
    const before = saves(bridge);
    // ⛔ A MAXIMIZE IS A MOVE, AND NOT AN `onDidLayoutChange` (E104 of the plan): `dockview-core` 8.3.1 saved it only
    // through the change of the active group it makes -- and here there is none, Stato is active already.
    api.getPanel("status")?.group.api.maximize();
    await flush();
    expect(saves(bridge)).toBe(before + 1);
    expect(homeMaximized()).toBe(true);
    // ⛔ SHOWING STAYS NOT SAVING (decision 11): leaving a view with a group maximized, and coming back to it, fire the
    // same event inside `fromJSON` -- and Home opens as it was left.
    layout.showView("work");
    await flush();
    layout.showView("home");
    await flush();
    expect(saves(bridge)).toBe(before + 1);
    expect(api.hasMaximizedGroup()).toBe(true);
    // ⛔ AND THE RESTORE IS A MOVE TOO: before, it never reached the package, and Home came back maximized.
    api.getPanel("status")?.group.api.exitMaximized();
    await flush();
    expect(saves(bridge)).toBe(before + 2);
    expect(homeMaximized()).toBe(false);
  });
});

describe("the band", () => {
  it("is a warning with «Riprova» while waiting, «Riprova» retries, and it is gone once connected (control 23)", async () => {
    const Band = (await import("./Band.vue")).default;
    const connection = useConnection();
    const retry = vi.spyOn(connection, "retry");
    const wrapper = mount(Band, { global: { plugins: [i18n] } });
    const notice = wrapper.get(".base-notice");
    expect(notice.attributes("data-tone")).toBe("warn");
    expect(notice.get(".title").text()).toBe(i18n.global.t("band.waiting"));
    await notice.get("button").trigger("click");
    expect(retry).toHaveBeenCalledOnce();
    connection.receive({ kind: "Accepted", value: "AsSystemAccount" });
    await wrapper.vm.$nextTick();
    // ⛔ THE SECOND DIRECTION, and it is the one that is forgotten: a band that never goes away
    // would pass the first assertion and be a permanent warning over a working app.
    expect(wrapper.text()).toBe("");
  });

  it("stops the window when the core speaks another protocol: no action, and the stamp the core expects (E60)", async () => {
    const Band = (await import("./Band.vue")).default;
    const connection = useConnection();
    const wrapper = mount(Band, { global: { plugins: [i18n] } });
    connection.receive({ kind: "StaleBuild", value: "81985529216486895" });
    await nextTick();
    const notice = wrapper.get(".base-notice");
    expect(notice.attributes("data-tone")).toBe("stop");
    // ON THE PAGE, LIKE THE WAITING BAND (E70 of the design-system plan): the browser measures the waiting one's place
    // only, so this one's class and mark are held here.
    expect(notice.classes()).toContain("band");
    expect(notice.attributes("data-on-page")).toBeDefined();
    expect(notice.get(".title").text()).toBe(i18n.global.t("band.stale"));
    expect(notice.get(".description").text()).toBe(i18n.global.t("band.expected", { stamp: "81985529216486895" }));
    // ⛔ NOTHING TO RETRY: the core stopped listening to this build.
    expect(notice.find("button").exists()).toBe(false);
  });

  it("keeps its status region while connected, and the words enter that same region (M-3 of E187)", async () => {
    const Band = (await import("./Band.vue")).default;
    const connection = useConnection();
    connection.receive({ kind: "Accepted", value: "AsSystemAccount" });
    const wrapper = mount(Band, { global: { plugins: [i18n] } });
    const region = wrapper.get('[role="status"]');
    expect(region.text()).toBe("");
    connection.receive({ kind: "StaleBuild", value: "81985529216486895" });
    await nextTick();
    // ⛔ THE SAME ELEMENT, NOW WITH WORDS: a region born with its text is the case many readers do not announce.
    expect(wrapper.get('[role="status"]').element).toBe(region.element);
    expect(region.text()).toContain(i18n.global.t("band.stale"));
  });
});

/** The overview is open: the whole-window variant of `BaseDialog`, rendered in its portal on `body`. */
function overviewOpen(): boolean {
  return document.querySelector('.base-dialog[data-variant="full"]') !== null;
}

/** The overview's cards, views first and «Salva questa vista» last. */
function cards(): HTMLElement[] {
  return [...document.querySelectorAll<HTMLElement>("[data-card]")];
}

/** A key pressed on the window, where the frame listens: the event says whether a listener took it from the browser. */
function press(key: string): KeyboardEvent {
  const event = new KeyboardEvent("keydown", { key, bubbles: true, cancelable: true });
  window.dispatchEvent(event);
  return event;
}

/** Writes in the field of a new view's name, as a keyboard does: the value, and the `input` event `v-model` listens to. */
function write(text: string): void {
  const field = document.querySelector<HTMLInputElement>(".naming input");
  if (field === null) throw new Error("no field for the name");
  field.value = text;
  field.dispatchEvent(new Event("input", { bubbles: true }));
}

/** ⛔ THE WHOLE FRAME, WITH ITS DOCK (the (d) of the design system): the bar, the windows, the strip inside the grid. The
 * frames are UNMOUNTED after each probe -- a frame listens on `window`, and one left mounted would answer the next F3. */
describe("the frame (the (d) of the design system)", () => {
  const frames: { unmount: () => void }[] = [];

  afterEach(() => {
    for (const frame of frames.splice(0)) frame.unmount();
    document.body.replaceChildren();
  });

  async function frame(): Promise<void> {
    frames.push(mount(Frame, { attachTo: document.body, global: { plugins: [i18n] } }));
    await flush();
  }

  it("names the view on screen in the bar, and opens the overview from the name", async () => {
    await frame();
    const name = document.querySelector<HTMLElement>(".view-name");
    expect(name?.textContent).toContain(i18n.global.t("views.home"));
    name?.click();
    await flush();
    expect(overviewOpen()).toBe(true);
    // The three views and «Salva questa vista»; the view on screen is the current card, in bordeaux.
    expect(cards().map((card) => card.getAttribute("data-card"))).toEqual(["view", "view", "view", "save"]);
    expect(cards().map((card) => card.getAttribute("aria-current"))).toEqual(["page", null, null, null]);
  });

  it("opens and closes the overview with F3, and keeps quiet while the confirmation or the drawer is open", async () => {
    await frame();
    // ⛔ F3 IS OURS (P-11): its default, the browser's "find next", is taken away.
    expect(press("F3").defaultPrevented).toBe(true);
    await flush();
    expect(overviewOpen()).toBe(true);
    press("F3");
    await flush();
    expect(overviewOpen()).toBe(false);
    // ⛔ THE SECOND DIRECTION: a window that asks something is not covered by the views.
    useDrawer().open = true;
    await flush();
    press("F3");
    await flush();
    expect(overviewOpen()).toBe(false);
    useDrawer().open = false;
    useInvoke().send({ function: VRAM_POLICY.name, argument: VRAM_POLICY.argument.local });
    useCore().receive({ kind: "PermissionRequired", value: { tool: "registry", resource: "arbiter", operation: "Write" } });
    await flush();
    expect(useInvoke().asking).toBe(true);
    press("F3");
    await flush();
    expect(overviewOpen()).toBe(false);
  });

  it("shows the view of the card chosen and closes, opens a named view by its name, and closes it with one of the three (E76, E82)", async () => {
    const layout = useLayout();
    await frame();
    press("F3");
    await flush();
    cards()[1]?.click();
    await flush();
    expect(layout.view).toBe("work");
    expect(layout.openNamed).toBeNull();
    expect(overviewOpen()).toBe(false);
    // ⛔ TWO NAMED VIEWS (E82): with one, "the one chosen" and "the first" are the same card.
    const named = [
      { name: "Revisione", layout: ownersHome() },
      { name: "Lettura", layout: ownersHome() },
    ];
    layout.receive(packageFromTheCore({ view: "work", layouts: {}, named }));
    await flush();
    press("F3");
    await flush();
    // The named views sit after the three, before «Salva questa vista», with a word that says they were saved.
    expect(cards()).toHaveLength(6);
    expect(cards()[4]?.textContent).toContain(i18n.global.t("overview.saved"));
    cards()[4]?.click();
    await flush();
    expect(layout.openNamed).toBe("Lettura");
    expect(document.querySelector(".view-name")?.textContent).toContain("Lettura");
    press("F3");
    await flush();
    // ⛔ ONE CARD IN BORDEAUX (the (d)): the named view on screen, and not Lavoro too -- the view of always under it.
    expect(cards().map((card) => card.getAttribute("aria-current"))).toEqual([null, null, null, null, "page", null]);
    // ⛔ ONE OF THE THREE CLOSES THE NAMED VIEW (E76, D13): the role `Frame.switchTo` had, the overview's now.
    cards()[0]?.click();
    await flush();
    expect(layout.openNamed).toBeNull();
    expect(layout.view).toBe("home");
  });

  it("says under the field a name that is empty or taken, and saves a new one and opens it (D4, D12)", async () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    // The core has answered, with nothing: before it «Salva questa vista» is off (E81).
    layout.receive({ kind: "Layout", value: { state: "Nothing" } });
    await frame();
    press("F3");
    await flush();
    document.querySelector<HTMLElement>('[data-card="save"]')?.click();
    await flush();
    // ⛔ THE FIELD SAYS TO THE EYE WHAT IT WANTS (E106): the board draws no visible label, and the placeholder speaks.
    expect(document.querySelector(".naming input")?.getAttribute("placeholder")).toBe(i18n.global.t("overview.name"));
    const confirm = (): void => document.querySelectorAll<HTMLElement>(".naming .base-button")[1]?.click();
    confirm();
    await flush();
    expect(document.querySelector(".naming")?.textContent).toContain(i18n.global.t("overview.empty"));
    expect(document.querySelector(".naming input")?.getAttribute("aria-invalid")).toBe("true");
    // ⛔ THE THREE VIEWS' NAMES ARE THE FRAME'S WORDS (D12): «Home» is taken in any case, spaces around or not.
    write("  home ");
    confirm();
    await flush();
    expect(overviewOpen(), "a name that is taken keeps the overview open").toBe(true);
    expect(document.querySelector(".naming")?.textContent).toContain(i18n.global.t("overview.taken"));
    expect(saves(bridge)).toBe(0);
    write("Revisione");
    confirm();
    await flush();
    expect(overviewOpen()).toBe(false);
    expect(layout.openNamed).toBe("Revisione");
    expect(layout.saved?.named?.map((entry) => entry.name)).toEqual(["Revisione"]);
    expect(saves(bridge)).toBe(1);
  });

  it("keeps «Salva questa vista» off until the core has answered (E81)", async () => {
    const layout = useLayout();
    await frame();
    press("F3");
    await flush();
    const save = (): HTMLButtonElement | null => document.querySelector<HTMLButtonElement>('[data-card="save"]');
    // ⛔ BEFORE THE ANSWER THE STORE HOLDS NO PACKAGE: a view saved now would send `layouts: {}`, and the core would keep
    // that -- every saved layout lost in silence, the window of E40.
    expect(save()?.disabled).toBe(true);
    layout.receive({ kind: "Layout", value: { state: "Nothing" } });
    await flush();
    expect(save()?.disabled).toBe(false);
  });

  it("draws a layout it cannot read as an empty miniature, and every other as before (E90)", async () => {
    const layout = useLayout();
    // ⛔ `unpack` keeps any object as a layout, and `schematic` throws on one it cannot read: that card goes empty, not the
    // whole overview.
    const unreadable = {} as SerializedDockview;
    layout.receive(packageFromTheCore({ view: "home", layouts: {}, named: [{ name: "Rotta", layout: unreadable }] }));
    await frame();
    press("F3");
    await flush();
    expect(cards()).toHaveLength(5);
    expect(cards()[3]?.querySelectorAll(".tile")).toHaveLength(0);
    expect(cards()[0]?.querySelectorAll(".tile").length).toBeGreaterThan(0);
  });

  it("draws each view in miniature from its layout: a tile per group, with its module's icon, and not the strip (D14)", async () => {
    await frame();
    press("F3");
    await flush();
    const icons = [...(cards()[0]?.querySelectorAll(".tile") ?? [])].map((tile) => tile.querySelector("svg")?.getAttribute("data-icon"));
    // ⛔ FROM THE LAYOUT, NOT A PICTURE (answer 19): Home ships one panel per group, and the strip is one of them.
    expect(icons.sort()).toEqual(Object.keys(VIEWS.home.panels ?? {}).filter((id) => id !== "strip").sort());
    // ⛔ AND EVERY VIEW THAT SHIPS SAYS THE TRUTH ABOUT ITS STRIP (E103): the strip's row is the strip's own height, and
    // the tiles reach down to it, as on screen -- a view written before `dockview` applied that height drew Compatta's
    // knowledge base in half the miniature.
    const percent = (value: string): number => Number(/^calc\(([^%]+)%/.exec(value)?.[1]);
    for (const [index, view] of (["home", "work", "compact"] as const).entries()) {
      const shipped = VIEWS[view];
      const strip = (shipped.panels.strip?.maximumHeight ?? 0) / shipped.grid.height;
      const tiles = [...(cards()[index]?.querySelectorAll<HTMLElement>(".tile") ?? [])];
      expect(strip, view).toBeGreaterThan(0);
      expect(tiles.length, view).toBeGreaterThan(0);
      const reach = Math.max(...tiles.map((tile) => percent(tile.style.top) + percent(tile.style.height)));
      expect(reach, view).toBeGreaterThanOrEqual(100 * (1 - strip) - 1e-6);
    }
  });

  it("opens the drawer from the strip's button", async () => {
    await frame();
    const button = document.querySelector<HTMLElement>(".strip .base-button");
    expect(button?.textContent).toContain(i18n.global.t("drawer.open"));
    button?.click();
    await flush();
    expect(useDrawer().open).toBe(true);
    expect(document.querySelector('.base-dialog[data-variant="sheet"]')).not.toBeNull();
    // ⛔ AND «CHIUDI» CLOSES IT THROUGH THE STORE (R3-20): the sheet's own button, not only Esc.
    const close = document.querySelector<HTMLElement>('.base-dialog[data-variant="sheet"] .base-button');
    expect(close?.textContent).toContain(i18n.global.t("drawer.close"));
    close?.click();
    await flush();
    expect(useDrawer().open).toBe(false);
    expect(document.querySelector('.base-dialog[data-variant="sheet"]')).toBeNull();
  });
});

import { mount } from "@vue/test-utils";
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
});

describe("the band", () => {
  it("is there while waiting, and gone once connected", async () => {
    const Band = (await import("./Band.vue")).default;
    const connection = useConnection();
    const wrapper = mount(Band, { global: { plugins: [i18n] } });
    expect(wrapper.text()).toContain(i18n.global.t("band.waiting"));
    connection.receive({ kind: "Accepted", value: "AsSystemAccount" });
    await wrapper.vm.$nextTick();
    // ⛔ THE SECOND DIRECTION, and it is the one that is forgotten: a band that never goes away
    // would pass the first assertion and be a permanent warning over a working app.
    expect(wrapper.text()).toBe("");
  });
});

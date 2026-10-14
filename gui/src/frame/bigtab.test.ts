import type { TabPartInitParameters } from "dockview-core";
import { describe, expect, it, vi } from "vitest";
import { nextTick } from "vue";

import { i18n } from "../i18n";

import { BigTab } from "./BigTab";

const t = i18n.global.t;

/** A module type whose `name` is not its `module`: the registry hands it out for this name, here only. Hoisted, as the
 * mock below is. */
const RENAMED = vi.hoisted(() => ({ name: "status-renamed", module: "status", who: 2 }));

// ⛔ EVERY ROW OF `PANEL_TYPES` HAS ITS NAME EQUAL TO ITS MODULE TODAY, so the one case that tells which of the two the tab
// reads needs a row the registry does not have: it is handed one more, and nothing else changes (AUD-2115 of the audit of
// 2026-09-30).
vi.mock("../panels/registry", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../panels/registry")>();
  return { ...actual, panelType: (name: string) => (name === RENAMED.name ? RENAMED : actual.panelType(name)) };
});

type Place = "grid" | "floating";
type Moved = (event: { location: { type: Place } }) => void;

function parameters(id: string, title?: string, maximized = false, at: Place = "grid") {
  const calls: string[] = [];
  let location: { type: Place } = { type: at };
  const listeners: Moved[] = [];
  const api = {
    id,
    isMaximized: () => maximized,
    maximize: () => calls.push("maximize"),
    exitMaximized: () => calls.push("exit"),
    get location() {
      return location;
    },
    onDidLocationChange: (listener: Moved) => {
      listeners.push(listener);
      return { dispose: () => void listeners.splice(listeners.indexOf(listener), 1) };
    },
  };
  const containerApi = {
    getPanel: (wanted: string) => (wanted === id ? { id } : undefined),
    addFloatingGroup: () => calls.push("float"),
  };
  /** The tile moves, as `dockview` moves it: the location changes, and then the event says where it settled. */
  const moveTo = (type: Place): void => {
    location = { type };
    for (const listener of [...listeners]) listener({ location });
  };
  return { calls, listeners, moveTo, init: { api, containerApi, title, params: {} } as unknown as TabPartInitParameters };
}

describe("the big tab", () => {
  it("shows a module's Italian name with the module's icon, and a plain panel's own title without one", () => {
    const status = new BigTab();
    status.init(parameters("status").init);
    expect(status.element.querySelector(".base-label")?.textContent?.trim()).toBe(t("modules.status"));
    expect(status.element.querySelector('.base-label svg[data-icon="status"]')).not.toBeNull();
    const other = new BigTab();
    other.init(parameters("not-a-module", "Titolo dato").init);
    expect(other.element.querySelector(".base-label")?.textContent?.trim()).toBe("Titolo dato");
    expect(other.element.querySelector(".base-label svg")).toBeNull();
  });

  it("names a module and draws its icon from its `module`, as the drawer does, and not from the panel's id (AUD-2115 of the audit of 2026-09-30)", () => {
    // `module` is the key of `modules.*` in the locale and the icon's name (`PanelType` in `registry.ts`); the probes of the
    // words and of the icons walk it, so the tab is covered only if it reads the same field.
    const tab = new BigTab();
    tab.init(parameters(RENAMED.name, "Titolo dato").init);
    expect(tab.element.querySelector(".base-label")?.textContent?.trim()).toBe(t("modules.status"));
    expect(tab.element.querySelector('.base-label svg[data-icon="status"]')).not.toBeNull();
  });

  it("carries two named commands of the kit, that run and do not start a drag", () => {
    const { calls, init } = parameters("status");
    const tab = new BigTab();
    tab.init(init);
    const buttons = [...tab.element.querySelectorAll("button")];
    expect(buttons.map((b) => b.getAttribute("aria-label"))).toEqual([t("menu.float"), t("menu.page")]);
    // ⛔ THE KIT'S PIECES, NOT GLYPHS IN A BUTTON BUILT BY HAND (section (c); trap 5 of the design system).
    expect(buttons.map((b) => b.querySelector("svg.base-icon")?.getAttribute("data-icon"))).toEqual(["float", "fullPage"]);
    let reachedTheTab = 0;
    tab.element.addEventListener("pointerdown", () => {
      reachedTheTab += 1;
    });
    buttons[0]?.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true }));
    // ⛔ THE SECOND DIRECTION: the same event on the label DOES reach the tab -- so the count below
    // is one because of the stop on the button, not because nothing bubbles.
    tab.element.querySelector(".base-label")?.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true }));
    expect(reachedTheTab).toBe(1);
    buttons[0]?.click();
    buttons[1]?.click();
    expect(calls).toEqual(["float", "maximize"]);
  });

  it("brings a tile at full page back with its second command, and does not maximize it again (AUD-2192 of the audit of 2026-09-30)", () => {
    // ⛔ THE OTHER HALF OF «A PAGINA INTERA, O RITORNO»: the probe above meets a tile that is not maximized, and the return
    // went unexercised -- a command that always maximized passed it.
    const { calls, init } = parameters("status", undefined, true);
    const tab = new BigTab();
    tab.init(init);
    tab.element.querySelectorAll("button")[1]?.click();
    expect(calls).toEqual(["exit"]);
  });

  it("puts «A pagina intera, o ritorno» off while the tile floats, and on again when it is back in the grid (AUD-2061 of the audit of 2026-09-30)", async () => {
    // ⛔ OFF THE GRID `maximize()` DOES NOTHING (M-1 of the review, E187), so the command that would call it is offered
    // off there -- the rule `dockview-core` 8.3.1 gives its own maximize item. BOTH DIRECTIONS, AND THE MOVE BETWEEN THEM:
    // a tile born floating starts with the command off, and the location event turns it on and off again.
    const { calls, moveTo, init } = parameters("status", undefined, false, "floating");
    const tab = new BigTab();
    tab.init(init);
    const page = (): HTMLButtonElement | undefined => tab.element.querySelectorAll("button")[1];
    expect(page()?.disabled).toBe(true);
    moveTo("grid");
    await nextTick();
    expect(page()?.disabled).toBe(false);
    page()?.click();
    expect(calls).toEqual(["maximize"]);
    moveTo("floating");
    await nextTick();
    expect(page()?.disabled).toBe(true);
  });

  it("takes its face away when dockview disposes of the tab, and stops listening to where the tile is", () => {
    const { listeners, init } = parameters("status");
    const tab = new BigTab();
    tab.init(init);
    expect(tab.element.childElementCount).toBeGreaterThan(0);
    tab.dispose();
    // ⛔ ONE VUE APP PER TAB: a tab `dockview` throws away that kept its app would leak a reactive tree, and a listener left
    // on the panel's location would keep that tree reachable.
    expect(tab.element.childElementCount).toBe(0);
    expect(listeners).toHaveLength(0);
  });
});

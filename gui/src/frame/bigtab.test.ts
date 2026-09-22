import type { TabPartInitParameters } from "dockview-core";
import { describe, expect, it } from "vitest";

import { i18n } from "../i18n";

import { BigTab } from "./BigTab";

const t = i18n.global.t;

function parameters(id: string, title?: string) {
  const calls: string[] = [];
  const api = {
    id,
    isMaximized: () => false,
    maximize: () => calls.push("maximize"),
    exitMaximized: () => calls.push("exit"),
  };
  const containerApi = {
    getPanel: (wanted: string) => (wanted === id ? { id } : undefined),
    addFloatingGroup: () => calls.push("float"),
  };
  return { calls, init: { api, containerApi, title, params: {} } as unknown as TabPartInitParameters };
}

describe("the big tab", () => {
  it("shows a module's Italian name, and a plain panel's own title", () => {
    const status = new BigTab();
    status.init(parameters("status").init);
    expect(status.element.querySelector(".bigtab-title")?.textContent).toBe(t("modules.status"));
    const other = new BigTab();
    other.init(parameters("not-a-module", "Titolo dato").init);
    expect(other.element.querySelector(".bigtab-title")?.textContent).toBe("Titolo dato");
  });

  it("carries two named commands that run, and do not start a drag", () => {
    const { calls, init } = parameters("status");
    const tab = new BigTab();
    tab.init(init);
    const buttons = [...tab.element.querySelectorAll("button")];
    expect(buttons.map((b) => b.getAttribute("aria-label"))).toEqual([t("menu.float"), t("menu.page")]);
    let reachedTheTab = 0;
    tab.element.addEventListener("pointerdown", () => { reachedTheTab += 1; });
    buttons[0]?.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true }));
    // ⛔ THE SECOND DIRECTION: the same event on the title DOES reach the tab -- so the count below
    // is zero because of `stopPropagation`, not because nothing bubbles.
    tab.element.querySelector(".bigtab-title")?.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true }));
    expect(reachedTheTab).toBe(1);
    buttons[0]?.click();
    buttons[1]?.click();
    expect(calls).toEqual(["float", "maximize"]);
  });
});

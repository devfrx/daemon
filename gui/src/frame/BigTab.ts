import type { ITabRenderer, TabPartInitParameters } from "dockview-core";

import { i18n } from "../i18n";
import { isModule } from "../panels/registry";

/**
 * The big grab handle (move 5): the tab element is what `dockview` drags, so a big tab is a big
 * grab -- which is what makes a pointer that is a HAND able to take it (move 8, ADR-0039).
 *
 * ⛔ A CLICK ON A COMMAND MUST NOT START A DRAG, and stopping `click` alone is not enough:
 * `dockview` begins the drag on `pointerdown`/`mousedown`, so both are stopped here. Measured in
 * SP-8; without it, every press of a command drags the tile a few pixels first.
 *
 * ⛔ TWO COMMANDS AND NOT THREE (D58): "float" and "full page" are the library's; "in a separate
 * window" needs `popoutUrl` and a page served from an http(s) origin, which is the shell's (Q3 of
 * SP-8, P-91), and the shell is outside this plan. Each command is a `<button>` with a name from
 * the locale: reachable with the tab key, read by a screen reader (G20).
 */
export class BigTab implements ITabRenderer {
  readonly element = document.createElement("div");

  init(parameters: TabPartInitParameters): void {
    this.element.className = "bigtab";
    const title = document.createElement("span");
    title.className = "bigtab-title";
    // ⛔ THE MODULE'S ITALIAN NAME AND NOT THE PANEL'S ID (G21, P-95): the views carry `title: id`,
    // and an id is code. A panel that is not a module type keeps the title it was given.
    title.textContent = isModule(parameters.api.id)
      ? i18n.global.t(`modules.${parameters.api.id}`)
      : (parameters.title ?? parameters.api.id);
    this.element.append(title);

    const command = (glyph: string, key: "float" | "page", run: () => void): void => {
      const button = document.createElement("button");
      button.type = "button";
      button.textContent = glyph;
      const name = i18n.global.t(`menu.${key}`);
      button.setAttribute("aria-label", name);
      button.title = name;
      button.addEventListener("pointerdown", (event) => event.stopPropagation());
      button.addEventListener("mousedown", (event) => event.stopPropagation());
      button.addEventListener("click", (event) => {
        event.stopPropagation();
        run();
      });
      this.element.append(button);
    };
    command("⧉", "float", () => {
      const panel = parameters.containerApi.getPanel(parameters.api.id);
      if (panel !== undefined) parameters.containerApi.addFloatingGroup(panel, { x: 60, y: 60, width: 460, height: 320 });
    });
    command("⤢", "page", () => {
      if (parameters.api.isMaximized()) parameters.api.exitMaximized();
      else parameters.api.maximize();
    });
  }
}

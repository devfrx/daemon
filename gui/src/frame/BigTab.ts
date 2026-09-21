import type { ITabRenderer, TabPartInitParameters } from "dockview-core";

import { i18n } from "../i18n";
import { isModule } from "../panels/registry";

/**
 * The big grab handle (move 5): the tab element is what `dockview` drags, so a big tab is a big
 * grab -- which is what makes a pointer that is a HAND able to take it (move 8, ADR-0039).
 *
 * ⛔ THE DAY THIS TAB CARRIES A COMMAND, A CLICK ON IT MUST NOT START A DRAG, and stopping `click`
 * alone is not enough: `dockview` begins the drag on `pointerdown`/`mousedown`, so both must be
 * stopped ON THE BUTTON -- measured in SP-8 (`spikes/gui-shell/app/src/home.ts`); without it, every
 * press of a command drags the tile a few pixels first. This tab carries NO command (the module's
 * commands are the menu of task 14), so nothing is stopped here yet: the trap is written so that
 * task 14 finds it instead of paying it again.
 */
export class BigTab implements ITabRenderer {
  readonly element = document.createElement("div");

  init(parameters: TabPartInitParameters): void {
    this.element.className = "bigtab";
    const title = document.createElement("span");
    title.className = "bigtab-title";
    // ⛔ THE MODULE'S ITALIAN NAME AND NOT THE PANEL'S ID (G21): the views carry `title: id`, and
    // an id is code. A panel that is not a module type keeps the title it was given (P-95 of the
    // part-2 plan, found at the pre-check of task 14).
    title.textContent = isModule(parameters.api.id)
      ? i18n.global.t(`modules.${parameters.api.id}`)
      : (parameters.title ?? parameters.api.id);
    this.element.append(title);
  }
}

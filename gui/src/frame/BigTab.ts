import type { ITabRenderer, TabPartInitParameters } from "dockview-core";
import { createApp, type App } from "vue";

import { isIconName } from "../components/icons";
import { i18n } from "../i18n";
import { isModule } from "../panels/registry";

import BigTabFace from "./BigTabFace.vue";

/**
 * The big grab handle (move 5): the tab element is what `dockview` drags, so a big tab is a big
 * grab -- which is what makes a pointer that is a HAND able to take it (move 8, ADR-0039).
 *
 * ⛔ THE FACE IS THE KIT'S (design system, section (c)): `BigTabFace.vue`, a `BaseLabel` with the
 * module's icon and two `BaseButton`s, mounted here as `VueContent` mounts a panel -- ONE VUE APP
 * PER TAB, AND `unmount` ON `dispose`. Before, the two commands were `<button>`s built with
 * `document.createElement`, which the template linter cannot see (trap 5 of the design).
 *
 * ⛔ TWO COMMANDS AND NOT THREE (D58): "float" and "full page" are the library's; "in a separate
 * window" needs `popoutUrl` and a page served from an http(s) origin, which is the shell's (Q3 of
 * SP-8, P-91), and the shell is outside this plan. Each command has a name from the locale:
 * reachable with the tab key, read by a screen reader (G20).
 *
 * ⚠️ DECLARED LIMIT (M-1 of the review, E187): on a FLOATING group `maximize()` is a no-op in
 * `dockview-core` 8.3.1 -- measured on 2026-09-22, rectangle unchanged, no error -- so the second
 * command does nothing on a tile the first one detached, and is not disabled there. A button that
 * follows `api.location` is a second occurrence of the same kit question, and waits for it.
 */
export class BigTab implements ITabRenderer {
  readonly element = document.createElement("div");
  private app?: App;

  init(parameters: TabPartInitParameters): void {
    this.element.className = "bigtab";
    const id = parameters.api.id;
    this.app = createApp(BigTabFace, {
      // ⛔ THE MODULE'S ITALIAN NAME AND NOT THE PANEL'S ID (G21, P-95): the views carry `title: id`,
      // and an id is code. A panel that is not a module type keeps the title it was given, and no icon.
      title: isModule(id) ? i18n.global.t(`modules.${parameters.api.id}`) : (parameters.title ?? id),
      icon: isModule(id) && isIconName(id) ? id : undefined,
      onFloat: () => {
        const panel = parameters.containerApi.getPanel(id);
        if (panel !== undefined) parameters.containerApi.addFloatingGroup(panel, { x: 60, y: 60, width: 460, height: 320 });
      },
      onPage: () => {
        if (parameters.api.isMaximized()) parameters.api.exitMaximized();
        else parameters.api.maximize();
      },
    });
    this.app.use(i18n);
    this.app.mount(this.element);
  }

  dispose(): void {
    this.app?.unmount();
    this.app = undefined;
  }
}

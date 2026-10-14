import type { DockviewIDisposable, DockviewPanelApi, ITabRenderer, TabPartInitParameters } from "dockview-core";
import { createApp, h, ref, type App } from "vue";

import { isIconName } from "../components/icons";
import { i18n } from "../i18n";
import { panelType } from "../panels/registry";

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
 * ⛔ THE SECOND COMMAND IS OFF WHERE IT WOULD DO NOTHING (M-1 of the review, E187): off the grid
 * `maximize()` is a no-op in `dockview-core` 8.3.1 -- on a floating group, measured on 2026-09-22,
 * rectangle unchanged, no error -- so the command goes off while the tile is not in the grid, and
 * comes back on when it returns. The rule is the library's own for its maximize item, read in
 * `dockview-core` 8.3.1: off when the group is not maximized and not in the grid. It follows
 * `api.onDidLocationChange`, and `bigtab.test.ts` holds both directions -- by the owner's decision
 * (AUD-2061 of the audit of 2026-09-30).
 */
export class BigTab implements ITabRenderer {
  readonly element = document.createElement("div");
  private app?: App;
  /** The listener on where the tile is: it goes with the tab, as the app does. */
  private located?: DockviewIDisposable;

  init(parameters: TabPartInitParameters): void {
    this.element.className = "bigtab";
    const api = parameters.api;
    const id = api.id;
    // ⛔ THE TYPE'S `module`, NOT THE PANEL'S ID (AUD-2115 of the audit of 2026-09-30): `module` is the key of the locale and
    // the icon's name, the field the drawer reads and the probes of the words and of the icons walk.
    const module = panelType(id)?.module;
    // ⛔ THE PROPS OF A ROOT APP DO NOT CHANGE ONCE IT IS CREATED, so the face is rendered by a root that reads the one prop
    // that moves from a ref -- the shape `VueContent` gives the params of a panel.
    const pageOff = ref(offTheGrid(api));
    this.located = api.onDidLocationChange(() => {
      pageOff.value = offTheGrid(api);
    });
    // ⛔ THE MODULE'S ITALIAN NAME AND NOT THE PANEL'S ID (G21, P-95): the views carry `title: id`,
    // and an id is code. A panel that is not a module type keeps the title it was given, and no icon.
    const title = module !== undefined ? i18n.global.t(`modules.${module}`) : (parameters.title ?? id);
    const icon = module !== undefined && isIconName(module) ? module : undefined;
    const onFloat = (): void => {
      const panel = parameters.containerApi.getPanel(id);
      if (panel !== undefined) parameters.containerApi.addFloatingGroup(panel, { x: 60, y: 60, width: 460, height: 320 });
    };
    const onPage = (): void => {
      if (api.isMaximized()) api.exitMaximized();
      else api.maximize();
    };
    this.app = createApp({ render: () => h(BigTabFace, { title, icon, pageOff: pageOff.value, onFloat, onPage }) });
    this.app.use(i18n);
    this.app.mount(this.element);
  }

  dispose(): void {
    this.located?.dispose();
    this.located = undefined;
    this.app?.unmount();
    this.app = undefined;
  }
}

/** Whether «A pagina intera, o ritorno» would do nothing here: a group not maximized and not in the grid. */
function offTheGrid(api: DockviewPanelApi): boolean {
  return !api.isMaximized() && api.location.type !== "grid";
}

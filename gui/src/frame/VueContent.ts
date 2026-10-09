import type { GroupPanelPartInitParameters, IContentRenderer, PanelUpdateEvent, Parameters } from "dockview-core";
import type { App, Component } from "vue";
import { createApp, h, shallowRef } from "vue";

import { i18n } from "../i18n";

/**
 * What a panel declares of what `dockview` hands over, and nothing else. ⛔ UNDECLARED, A PROP FALLS THROUGH TO THE PANEL'S
 * ROOT AS AN ATTRIBUTE (AUD-1114 of the audit of 2026-09-30): `title` with the panel's id -- a tooltip of code over the tile,
 * and a region named by it -- and the objects as "[object Object]". The panels declare only what they read.
 */
function declared(component: Component, given: Record<string, unknown>): Record<string, unknown> {
  const props = (component as { props?: readonly string[] | Record<string, unknown> }).props;
  const names = props === undefined ? [] : Array.isArray(props) ? props : Object.keys(props);
  return Object.fromEntries(Object.entries(given).filter(([name]) => names.includes(name)));
}

/**
 * Mounts one Vue component as the content of one `dockview` panel -- the bridge between Vue and
 * the panels that decision 2 of the north star says we write ourselves, rather than taking
 * `dockview-vue` (ADR-0030 prefers framework-agnostic libraries, and the adapter has few users).
 *
 * ⛔ ONE VUE APP PER PANEL, AND `unmount` ON `dispose`: a panel `dockview` throws away must
 * unmount its app or every closed tile leaks a reactive tree. Measured shape from SP-8's
 * `spikes/gui-shell/app/src/vue-bridge.ts`, which the owner exercised across the eight moves.
 *
 * ⚠️ THE APP GETS `i18n` AND NOT PINIA: pinia is installed on the ROOT app and its stores are
 * global to the page, so a panel reaches them through `useCore()` without a plugin. `i18n` is
 * per-app, so it has to be handed over here or every panel renders raw keys.
 *
 * ⛔ THE PARAMS FOLLOW `dockview`, AFTER `init` TOO (AUD-1115 of the audit of 2026-09-30): `apply` in `dock.ts` puts the
 * placeholder's params back on a panel that came without them, after the panel is made, and `dockview-core` 8.3.1 hands the
 * change to the renderer's `update` alone. The props of a root app do not change once it is created, so the panel is
 * rendered by a root that reads the params from a ref, and `update` writes it.
 */
export class VueContent implements IContentRenderer {
  readonly element = document.createElement("div");
  private app?: App;
  private readonly params = shallowRef<Parameters>({});

  constructor(private readonly component: Component) {
    this.element.className = "panel";
  }

  init(parameters: GroupPanelPartInitParameters): void {
    const component = this.component;
    const params = this.params;
    params.value = parameters.params;
    const fixed = { title: parameters.title, api: parameters.api, containerApi: parameters.containerApi };
    this.app = createApp({ render: () => h(component, declared(component, { ...fixed, params: params.value })) });
    this.app.use(i18n);
    this.app.mount(this.element);
  }

  update(event: PanelUpdateEvent): void {
    this.params.value = event.params;
  }

  dispose(): void {
    this.app?.unmount();
    this.app = undefined;
  }
}

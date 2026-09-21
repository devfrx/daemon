import type { GroupPanelPartInitParameters, IContentRenderer } from "dockview-core";
import type { App, Component } from "vue";
import { createApp } from "vue";

import { i18n } from "../i18n";

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
 */
export class VueContent implements IContentRenderer {
  readonly element = document.createElement("div");
  private app?: App;

  constructor(private readonly component: Component) {
    this.element.className = "panel";
  }

  init(parameters: GroupPanelPartInitParameters): void {
    this.app = createApp(this.component, {
      title: parameters.title,
      api: parameters.api,
      containerApi: parameters.containerApi,
      params: parameters.params,
    });
    this.app.use(i18n);
    this.app.mount(this.element);
  }

  dispose(): void {
    this.app?.unmount();
    this.app = undefined;
  }
}

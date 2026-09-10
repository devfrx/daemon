import { createApp, type App, type Component } from 'vue';
import type { GroupPanelPartInitParameters, IContentRenderer } from 'dockview-core';

/** Mounts a Vue component as the content of one dockview panel -- the bridge between Vue and the
 * panels that the north star (decision 2 of the coordinator) says we write ourselves. One Vue app
 * per panel: a panel that dockview disposes unmounts its app and nothing leaks. */
export class VueContent implements IContentRenderer {
  readonly element = document.createElement('div');
  private app?: App;

  constructor(private readonly component: Component) {
    this.element.className = 'tile';
  }

  init(parameters: GroupPanelPartInitParameters): void {
    this.app = createApp(this.component, {
      title: parameters.title,
      api: parameters.api,
      containerApi: parameters.containerApi,
      params: parameters.params,
    });
    this.app.mount(this.element);
  }

  dispose(): void {
    this.app?.unmount();
    this.app = undefined;
  }
}

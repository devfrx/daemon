import type { Component } from "vue";

import Chat from "./Chat.vue";
import Permissions from "./Permissions.vue";
import Settings from "./Settings.vue";
import Status from "./Status.vue";
import Steps from "./Steps.vue";
import { register } from "./registry";

/**
 * The modules sub-project 2 builds, by the name the views and the drawer use. ⛔ THE ONE PLACE THAT
 * PLUGS A BUILT MODULE INTO THE REGISTRY: task 13 left `register` as the seam so that
 * `registry.ts` would not change shape when a module is built -- a built module adds a row HERE.
 */
export const MODULES: Readonly<Record<string, Component>> = {
  chat: Chat,
  status: Status,
  permissions: Permissions,
  steps: Steps,
  settings: Settings,
};

export function registerModules(): void {
  for (const [name, component] of Object.entries(MODULES)) register(name, component);
}

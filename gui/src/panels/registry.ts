import type { IContentRenderer } from "dockview-core";
import type { Component } from "vue";

import { VueContent } from "../frame/VueContent";

import Placeholder from "./Placeholder.vue";
import Strip from "./Strip.vue";

export interface PanelType {
  /** The name a view's JSON carries, and the name `dockview` asks the factory for. */
  name: string;
  /** The key in `locales/it.json` under `modules`. */
  module: string;
  /** The sub-project that fills it -- §1 of the north star, "Chi". */
  who: number;
}

/**
 * The eighteen module types of §1 of the north star: five with a full table, thirteen with a
 * short one. ⛔ THE LIST IS THE CATALOGUE'S, NOT THIS FILE'S: a module type is added when §1
 * gains a row.
 *
 * ⛔ `who` IS THE FIRST SUB-PROJECT THAT TOUCHES THE MODULE, WHICH IS NOT ALWAYS THE WHOLE
 * "Chi" OF THE ROW: two rows of §1 carry more than one number -- "Voce e gesti | 8, 12" and
 * "Impostazioni | 2, poi 3 e 10" -- and this field is a single number on purpose, because the
 * placeholder and the drawer each render exactly one. The other sixteen match one to one, compared
 * row by row on 2026-09-21 (E163). Saying it here is what keeps the next census from reopening it.
 *
 * ⚠️ `strip` IS NOT HERE, and that is the distinction D50 draws: the strip is a panel of the
 * grid, so it is registered as a component, but it is not a MODULE -- it does not appear in the
 * drawer and no view lets the user close it.
 */
export const PANEL_TYPES: readonly PanelType[] = [
  { name: "chat", module: "chat", who: 3 },
  { name: "status", module: "status", who: 2 },
  { name: "permissions", module: "permissions", who: 2 },
  { name: "steps", module: "steps", who: 2 },
  { name: "activity", module: "activity", who: 3 },
  { name: "settings", module: "settings", who: 2 },
  { name: "scope", module: "scope", who: 5 },
  { name: "diff", module: "diff", who: 5 },
  { name: "preview", module: "preview", who: 3 },
  { name: "terminal", module: "terminal", who: 5 },
  { name: "sensors", module: "sensors", who: 4 },
  { name: "costs", module: "costs", who: 3 },
  { name: "knowledge", module: "knowledge", who: 6 },
  { name: "assets3d", module: "assets3d", who: 7 },
  { name: "voice", module: "voice", who: 8 },
  { name: "backup", module: "backup", who: 11 },
  { name: "checkpoint", module: "checkpoint", who: 5 },
  { name: "models", module: "models", who: 9 },
];

const BUILT = new Map<string, Component>([["strip", Strip]]);

/** Task 14 plugs the real modules in here. ⛔ ONE SEAM AND NOT EIGHTEEN IMPORTS in this file:
 * a module that is built must not make this file change shape, only its map gain a row. */
export function register(name: string, component: Component): void {
  BUILT.set(name, component);
}

export function isModule(name: string): boolean {
  return PANEL_TYPES.some((type) => type.name === name);
}

/** ⛔ WHAT THE REGISTRY CAN BUILD -- the strip, and the modules task 14 plugs in. `apply` in
 * `dock.ts` asks it before putting placeholder params back on a panel (R6-17): the strip is a
 * piece of the frame that lives in the grid (D50), carries no `params`, and is not a module type. */
export function isBuilt(name: string): boolean {
  return BUILT.has(name);
}

/**
 * What `dockview` gets for a name. Three cases, and they are NOT the same thing:
 *
 * - a built module -> its component;
 * - a module type that sub-project 2 does not build -> the placeholder, saying who fills it;
 * - ⛔ a name that is NOT A MODULE TYPE AT ALL -> the placeholder saying the type is gone. This
 *   is row 8 of §2 of the north star -- a saved package can point at a type a later build
 *   removed -- and conflating it with the case above would tell the user to wait for a
 *   sub-project that will never fill it.
 */
export function componentFor(name: string): () => IContentRenderer {
  const built = BUILT.get(name);
  if (built !== undefined) return () => new VueContent(built);
  return () => new VueContent(Placeholder);
}

/** The params a panel of `name` carries when nobody built it -- read by `Placeholder`. */
export function placeholderParams(name: string): Record<string, unknown> {
  const type = PANEL_TYPES.find((candidate) => candidate.name === name);
  return type === undefined ? { missing: true } : { module: type.module, who: type.who };
}

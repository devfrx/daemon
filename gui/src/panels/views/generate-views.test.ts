import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { createDockview, type DockviewApi, type SerializedDockview } from "dockview-core";
import { createPinia, setActivePinia } from "pinia";
import { beforeAll, expect, it } from "vitest";

import type { ViewName } from "../../stores/layout";
import { componentFor, placeholderParams } from "../registry";

import { VIEWS } from ".";

/**
 * ⛔ THE ONLY WRITER OF `src/panels/views/*.json`, and it is SKIPPED by default -- the twin of
 * `regenerate_the_fixtures` in `crates/kernel/tests/ipc_wire.rs`, and for the same reason: a
 * gate must not rewrite the artefacts it is checking.
 *
 *   REGENERATE_VIEWS=1 npx vitest run src/panels/views/generate-views.test.ts
 *
 * ⛔ AND THE JSON IS GENERATED RATHER THAN HAND-WRITTEN because the serialised shape is
 * `dockview`'s, not ours: a guessed file would be accepted and silently corrected on load (E4 of
 * the part-1 plan measured that `toJSON` after `fromJSON` differs raw). What this file dictates
 * is the CONTENT -- which panels, where, which are locked.
 *
 * ⛔ AND WHAT IT WOULD WRITE IS COMPARED WITH WHAT IS COMMITTED, AT EVERY RUN (AUD-2203 and AUD-2204 of the audit of
 * 2026-09-30), as `the_committed_fixtures_match_the_schema` is the twin of the fixtures' writer: each tile carries in its
 * `params` the `module` and the `who` of the registry, read when the view was written, and `apply` in `dock.ts` puts them
 * back only on a panel that has none -- so a `who` changed in `registry.ts` reached the drawer and not the tiles, with no
 * red. Composed in memory and laid out as the writer lays it out, then compared with the committed view.
 */
const OUT = join(dirname(fileURLToPath(import.meta.url)));

// ⛔ A PINIA MUST BE ACTIVE BEFORE A GRID IS BUILT (R6-2, measured on 2026-09-16): `Strip.vue`
// calls `useCore()` at setup, and `VueContent` mounts every panel as an app of its own WITHOUT
// pinia -- the root app installs it, and this file has no root app. Without this line the
// generator dies on `getActivePinia()`.
beforeAll(() => {
  setActivePinia(createPinia());
});

function dock(): DockviewApi {
  const host = document.createElement("div");
  document.body.append(host);
  const api = createDockview(host, {
    createComponent: ({ name }) => componentFor(name)(),
  });
  api.layout(1600, 1000);
  return api;
}

function lock(api: DockviewApi, id: string): void {
  const group = api.getPanel(id)?.group;
  if (group === undefined) throw new Error(`no panel ${id}`);
  // ⛔ TWO LINES AND NOT ONE: `locked` stops the drag, `header.hidden` takes away the tab that
  // would let it be closed. Measured in SP-8 (move 1); the second is the one that is forgotten.
  group.locked = true;
  group.header.hidden = true;
}

function tile(api: DockviewApi, id: string, where?: Parameters<DockviewApi["addPanel"]>[0]["position"]): void {
  api.addPanel({ id, component: id, title: id, params: placeholderParams(id), position: where });
}

/** The view as it is written. */
function serialised(api: DockviewApi): SerializedDockview {
  // ⛔ LAID OUT AGAIN BEFORE IT IS WRITTEN (E103 of the design-system plan): `dockview` applies a panel's limits -- the
  // strip's 56 px -- when it lays the grid out, not when the panel is added. Written at once, Compatta kept the strip at
  // half the grid, and its miniature drew the knowledge base in half the square, while the screen gave it all but 56 px.
  api.layout(api.width, api.height, true);
  return api.toJSON();
}

function write(name: string, api: DockviewApi): void {
  mkdirSync(OUT, { recursive: true });
  writeFileSync(join(OUT, `${name}.json`), `${JSON.stringify(serialised(api), null, 2)}\n`, "utf8");
}

it.skipIf(process.env.REGENERATE_VIEWS !== "1")("regenerates the three committed views", () => {
  for (const [name, api] of Object.entries(compose())) write(name, api);
});

it.skipIf(process.env.REGENERATE_VIEWS === "1")("finds each committed view to be what this file would write (AUD-2203, AUD-2204)", () => {
  const composed = compose();
  // ⛔ NON-VACUITY: the three views the app ships, each with tiles that carry the registry's `params`.
  expect(Object.keys(composed).sort()).toEqual(Object.keys(VIEWS).sort());
  expect(Object.values(VIEWS.work.panels).some((panel) => panel.params?.who !== undefined)).toBe(true);
  for (const [name, api] of Object.entries(composed)) expect(serialised(api), name).toEqual(VIEWS[name as ViewName]);
});

/** The three views the app ships, composed and not written. */
function compose(): Record<ViewName, DockviewApi> {
  // Home: the nucleus in the middle, locked; the strip at the bottom, locked; the tiles around.
  // The chat is NOT here -- it is a tab of Lavoro (question 7 of the north star).
  const home = dock();
  tile(home, "knowledge");
  home.addPanel({ id: "strip", component: "strip", title: "strip", position: { referencePanel: "knowledge", direction: "below" }, minimumHeight: 56, maximumHeight: 56 });
  tile(home, "status", { referencePanel: "knowledge", direction: "left" });
  tile(home, "permissions", { referencePanel: "status", direction: "below" });
  tile(home, "settings", { referencePanel: "permissions", direction: "below" });
  tile(home, "activity", { referencePanel: "knowledge", direction: "right" });
  tile(home, "costs", { referencePanel: "activity", direction: "below" });
  lock(home, "knowledge");
  lock(home, "strip");

  // Lavoro: Attività and Ambito on the left, the chat in the middle, Diff/Anteprima/Terminale on
  // the right, Passi and Sensori at the bottom -- "Il modello della GUI", row Lavoro.
  const work = dock();
  tile(work, "chat");
  work.addPanel({ id: "strip", component: "strip", title: "strip", position: { referencePanel: "chat", direction: "below" }, minimumHeight: 56, maximumHeight: 56 });
  tile(work, "activity", { referencePanel: "chat", direction: "left" });
  tile(work, "scope", { referencePanel: "activity", direction: "below" });
  tile(work, "diff", { referencePanel: "chat", direction: "right" });
  tile(work, "preview", { referencePanel: "diff", direction: "below" });
  tile(work, "terminal", { referencePanel: "preview", direction: "below" });
  tile(work, "steps", { referencePanel: "chat", direction: "below" });
  tile(work, "sensors", { referencePanel: "steps", direction: "right" });
  lock(work, "strip");

  // ⚠️ Compatta IS A PLACEHOLDER AND SAYS SO: its real shape -- a popout or a shrunk window --
  // is registered, not taken, and its closer is sub-project 10.
  const compact = dock();
  tile(compact, "knowledge");
  compact.addPanel({ id: "strip", component: "strip", title: "strip", position: { referencePanel: "knowledge", direction: "below" }, minimumHeight: 56, maximumHeight: 56 });
  lock(compact, "knowledge");
  lock(compact, "strip");
  return { home, work, compact };
}

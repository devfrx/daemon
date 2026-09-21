import type { SerializedDockview } from "dockview-core";

import type { ViewName } from "../../stores/layout";

import compact from "./compact.json";
import home from "./home.json";
import work from "./work.json";

/**
 * The three views that ship with the app (decision 11 of the north star): they stay in `gui/`
 * and are NOT copied into the archive at first run -- copied, an update that improves a view
 * would never reach anyone who had not touched it.
 *
 * ⚠️ THE `as` IS AN ASSERTION, NOT THE CHECK, AND IT IS LOAD-BEARING: `resolveJsonModule` widens a
 * `.json` to its literal shape, and that shape is NOT assignable to `SerializedDockview` -- measured
 * on 2026-09-16 (R6-9): with the three generated views and the `as` removed, `vue-tsc` answers three
 * `TS2322`. What the assertion catches is a GROSS mismatch (`TS2352` when a field changes type or
 * goes missing), not a field-by-field comparison: `panels` removed outright, or an unknown key,
 * passes. The check that the committed files are views the frame can build is the first probe of
 * `frame.test.ts` -- every `component` known to the registry, and no view without panels.
 */
export const VIEWS: Readonly<Record<ViewName, SerializedDockview>> = {
  home: home as SerializedDockview,
  work: work as SerializedDockview,
  compact: compact as SerializedDockview,
};

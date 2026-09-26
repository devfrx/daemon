import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

// ⛔ `jsdom` 30.0.1 DOES NOT IMPLEMENT `ResizeObserver`, and `dockview-core` calls it the moment
// a grid is created (`watchElementResize`): without this, every probe that mounts a grid dies on
// `ReferenceError: ResizeObserver is not defined` before any layout question is asked --
// measured on 2026-09-16 (R6-8 of the in-depth review of task 13). Mounted by `test.setupFiles`.
//
// ⚠️ THIS FAKES AN ABSENT API, NOT LAYOUT: it observes nothing, and `getBoundingClientRect` still
// answers zeros under jsdom, which is why the geometry of `moveActive` is probed with rectangles
// of our own (P-97). The day jsdom ships a `ResizeObserver`, `??=` leaves it alone.
globalThis.ResizeObserver ??= class {
  observe(): void {}
  unobserve(): void {}
  disconnect(): void {}
};

// ⛔ THE TOKENS THAT DO NOT CHANGE WITH THE THEME, AS THE BROWSER HAS THEM (design system, task 6):
// under jsdom Vitest does not process an imported `.css` -- the import is empty -- so `readToken`
// would find no token, and `createDock` no `gap` (measured on 2026-09-23: `--space-3` reads "" when
// imported, "12px" from a <style>). `base.css` is READ, not retyped: the values stay in the board's
// copy. `themes.css` stays out: no probe under jsdom reads a colour.
const sheet = document.createElement("style");
sheet.textContent = readFileSync(join(dirname(fileURLToPath(import.meta.url)), "tokens", "base.css"), "utf8");
document.head.append(sheet);

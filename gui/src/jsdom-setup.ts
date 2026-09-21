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

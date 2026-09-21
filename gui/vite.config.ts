import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import vue from "@vitejs/plugin-vue";
import { defineConfig } from "vitest/config";

// ⛔ THE VITE ROOT IS `gui/` BECAUSE THE NPM SCRIPTS RUN THERE, and no line in this file pins
// it: Vite's default root is the PROCESS's working directory, not the folder holding this
// file -- measured on 2026-09-20 with `resolveConfig`, which answers the repository root when
// called from there. What the root buys is the DEV SERVER: `gui/schema/fixtures/` stays inside
// it, so the browser can reach the fixtures without widening `server.fs.allow`.
// ⚠️ NOT MEASURED: that half would want the dev server running, and nothing here has proved it.
//
// ⚠️ AND THE ROOT IS NOT WHAT MAKES THE PROBES WORK: `import.meta.glob` resolves against the
// MODULE that writes it. Measured on 2026-09-20 by moving BOTH `root` to "src" AND `include`
// with it -- moving `root` alone answers "No test files found", which looks like a denial and
// is not one -- after which the probes stay at 9 passed while the fixtures sit OUTSIDE the root.

/**
 * ⛔ THE BUILD STAMP IS READ HERE, IN NODE, AND HANDED TO THE SPA AS A CONSTANT (D52, D90): the
 * last line of `ipc_v1.map`, which the KERNEL wrote. Not a `?raw` import from the browser: the
 * dev server of `vite` 8.3.0 treats every URL that ends in `.map` as a source-map request and
 * serves the file statically as `application/json`, skipping the transform -- `npm run dev`
 * showed a blank page while `vite build` and the probes stayed green (R6-3, measured on
 * 2026-09-16). Read once, when the config loads: the map changes only when the kernel's schema
 * does, and then the SPA is rebuilt anyway.
 *
 * ⛔ THE REGEXP ANCHORS THE LINE with `^` and `$` in multiline mode: the map also carries the
 * `Debug` of every message, and one of them may contain the word.
 */
const MAP = readFileSync(fileURLToPath(new URL("./schema/fixtures/ipc_v1.map", import.meta.url)), "utf8");
const DIGITS = /^stamp 0x([0-9a-fA-F]{16})$/m.exec(MAP)?.[1];
if (DIGITS === undefined) {
  throw new Error("ipc_v1.map carries no `stamp 0x…` line: regenerate the fixtures");
}

export default defineConfig({
  plugins: [vue()],
  define: {
    // ⛔ `BigInt` AND NOT `Number`: the stamp is FNV-1a over the whole set and passes
    // `Number.MAX_SAFE_INTEGER` as a matter of course -- the doc of `U64` says it once for everyone.
    __BUILD_STAMP__: JSON.stringify(BigInt(`0x${DIGITS}`).toString(10)),
  },
  test: {
    // ⛔ `jsdom` FROM THIS TASK ON: task 11 ran on `node` because nothing it built touched a DOM,
    // and said so. The frame mounts components, so it needs one.
    environment: "jsdom",
    include: ["src/**/*.test.ts"],
    // ⛔ `jsdom` 30.0.1 has no `ResizeObserver`, and `dockview-core` wants one the moment a grid is
    // created: the fake in this file is what lets a probe mount a grid at all (R6-8).
    setupFiles: ["src/jsdom-setup.ts"],
  },
});

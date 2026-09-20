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
export default defineConfig({
  plugins: [vue()],
  test: {
    // ⚠️ `node` AND NOT `jsdom`: nothing in this task touches a DOM. `jsdom` arrives with the
    // components, in the task that has something to render -- a dependency without a consumer
    // is the trade P-2 already refused for `@playwright/test`.
    environment: "node",
    include: ["src/**/*.test.ts"],
  },
});

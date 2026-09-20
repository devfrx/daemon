import vue from "@vitejs/plugin-vue";
import { defineConfig } from "vitest/config";

// ⛔ THE VITE ROOT IS LEFT AT ITS DEFAULT, `gui/`, and what that buys is the DEV SERVER: the
// fixtures the kernel generates live in `gui/schema/fixtures/`, outside `src/` and inside the
// root, so the browser reaches them without widening `server.fs.allow`. ⚠️ IT IS NOT WHAT
// MAKES THE PROBES WORK: `import.meta.glob` resolves against the MODULE that writes it, and
// measured on 2026-09-20 the probes stay at 9 passed with the root moved to "src". And `root`
// is not set in this file: it is Vite's default, so no line here pins it.
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

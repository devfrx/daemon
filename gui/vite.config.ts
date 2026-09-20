import vue from "@vitejs/plugin-vue";
import { defineConfig } from "vitest/config";

// ⛔ THE VITE ROOT IS `gui/` AND NOT `gui/src/`, because the fixtures the kernel generates live
// in `gui/schema/fixtures/` -- outside `src/` and inside the root, which is what lets
// `import.meta.glob` reach them in the browser as well as under the probes.
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

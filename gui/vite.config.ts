import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import { playwright } from "@vitest/browser-playwright";
import vue from "@vitejs/plugin-vue";
import { configDefaults, defineConfig } from "vitest/config";
import type { BrowserCommand } from "vitest/node";

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

/** The media features a probe may emulate -- the three the design system reads. `null` gives one back. */
interface Media {
  colorScheme?: "light" | "dark" | null;
  reducedMotion?: "reduce" | "no-preference" | null;
  forcedColors?: "active" | "none" | null;
}

/**
 * ⛔ A COMMAND AND NOT A CONTEXT OPTION: `contextOptions` would fix one value for a whole file, while a probe
 * must see BOTH directions -- "reduce" and "no-preference" -- in one test. `page` is the page that holds the
 * test iframe (Vitest 4.1, "Custom Commands"), and `emulateMedia` applies to its frames.
 */
const emulateMedia: BrowserCommand<[media: Media]> = async ({ page }, media) => {
  await page.emulateMedia(media);
};

export default defineConfig({
  plugins: [vue()],
  define: {
    // ⛔ `BigInt` AND NOT `Number`: the stamp is FNV-1a over the whole set and passes
    // `Number.MAX_SAFE_INTEGER` as a matter of course -- the doc of `U64` says it once for everyone.
    __BUILD_STAMP__: JSON.stringify(BigInt(`0x${DIGITS}`).toString(10)),
  },
  test: {
    /**
     * ⛔ TWO PROJECTS (design system, section (f)): `npm test` runs both in one run, for the local loop. The gate
     * runs them ONE AT A TIME (E10 of the design-system plan): inside one run a project that finds no file is GREEN
     * -- `vitest` answers "No test files found" only when the whole run is empty -- so a renamed file or a wrong
     * glob would drop a whole project in silence. `extends: true` hands each one the `plugins` and the `define`
     * above; everything else is written per project, because an inline project inherits nothing it does not ask for.
     */
    projects: [
      {
        extends: true,
        test: {
          name: "jsdom",
          // ⛔ `jsdom` since task 11 of part 2: the frame mounts components, so it needs a DOM.
          environment: "jsdom",
          include: ["src/**/*.test.ts"],
          exclude: [...configDefaults.exclude, "src/**/*.browser.test.ts"],
          // ⛔ `jsdom` 30.0.1 has no `ResizeObserver`, and `dockview-core` wants one the moment a grid is
          // created: the fake in this file is what lets a probe mount a grid at all (R6-8).
          setupFiles: ["src/jsdom-setup.ts"],
        },
      },
      {
        extends: true,
        // ⛔ THE DEPENDENCIES ARE PRE-BUNDLED AFRESH ON EVERY RUN (E24 of the design-system plan). A cache left by a run
        // that never reached a library makes Vite re-bundle MID-RUN and reload, and the probes end up with two copies of
        // Vue: every one red with `'set' on proxy: trap returned falsish`, measured on 2026-09-25. `npm ci` empties the
        // cache, so the gate never sees it -- it bites whoever runs the probes by hand. Not `optimizeDeps.include`, the
        // list Vitest's warning suggests: it must grow with every library a probe reaches, and a forgotten name is red
        // only where the gate does not look. The cost, measured: none -- a run takes under 4 s, cold or warm; and Vite
        // 8.3.0's types mark the option `@experimental`, while its page documents it without reserve.
        optimizeDeps: { force: true },
        test: {
          name: "browser",
          include: ["src/**/*.browser.test.ts"],
          // ⛔ FIVE SECONDS FOR `expect.poll`, NOT VITEST'S ONE (P-21 of the design-system plan): the scheme probe went
          // red once, no event within 1 s, and no cause was found -- not the renderers' priority, not a full CPU,
          // not a full memory, measured on 2026-09-24. A probe here guards THAT a value comes, not how fast: a late one
          // passes, one that never comes stays red, and a green poll ends at once -- well inside a browser test's 15 s.
          expect: { poll: { timeout: 5000 } },
          browser: {
            enabled: true,
            // ⛔ EXPLICIT: the default is `process.env.CI`, which would open a window on every local gate.
            headless: true,
            // ⛔ NO PICTURES OF A RED (R2-3 of the design-system review): by default every failing probe leaves a PNG in
            // `__screenshots__` next to its file -- inside `src/`, which git does not ignore, on a path Windows' git cannot
            // add past 260 characters. Nobody looks at them: the gate reads the words of the failure.
            screenshotFailures: false,
            // ⛔ THE INSTALLED CHROME (decision 22 of the design): nothing is downloaded, and a machine without
            // it goes red at this step with Playwright's own message -- the prerequisite, declared.
            provider: playwright({ launchOptions: { channel: "chrome" } }),
            instances: [{ browser: "chromium" }],
            // The width the approved boards were probed at; the default is a phone's, 414 x 896.
            viewport: { width: 1440, height: 900 },
            commands: { emulateMedia },
          },
        },
      },
    ],
  },
});

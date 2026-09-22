import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

import it_ from "./it.json";

const GUI = join(dirname(fileURLToPath(import.meta.url)), "..", "..");

/**
 * ⛔ WHAT NO LINT CAN DO, AND THAT IS WHY THIS FILE OUTLIVED THE NET.
 *
 * Task 13 wrote two probes here and called the file a net until task 15. Task 15 replaced the
 * first one -- bare words in a template -- with `@intlify/vue-i18n/no-raw-text` at `error`
 * (D65). It could NOT replace this one: the SPA BUILDS these keys, `modules.${type.module}` in
 * the drawer and `modules.${parameters.api.id}` in `BigTab.ts`, and `no-missing-keys` is blind
 * to a built key -- measured on 2026-09-15, both directions in one file (P-105).
 *
 * ⚠️ NOT RENAMED: `src/frame/keys.test.ts` already exists (task 14), and two files of that name
 * in two folders is exactly the confusion this repository pays for when re-reading.
 *
 * ⛔ AND ONE THING THE LINT CAN DO ONLY WHILE ITS EYES ARE OPEN (I-2 of the review, E197):
 * `no-missing-keys` reads the locale through `settings["vue-i18n"].localeDir` of
 * `eslint.config.js`, and when that glob stops resolving -- the folder renamed, the pattern
 * mistyped -- the rule reports NOTHING: `npm run lint` green, the gate green, measured on
 * 2026-09-22 with `localeDir` pointed at a folder that does not exist. The second probe is the
 * non-vacuity guard of the other half of D65: the `settings` line of the config, verbatim, and
 * the file it points at, present. Nothing else in the gate watches that.
 * ⚠️ STATIC ON PURPOSE, AND THE BLIND SPOT IS DECLARED: linting a made-up key through the real
 * `ESLint` API was the first form, and it measured 4.6 s alone and 27 s -- a timeout -- inside
 * the suite on 2026-09-22, because it loads the whole chain, `typescript` included. A guard that
 * can go red for being slow guards nothing. This one costs milliseconds, and it cannot see a
 * plugin that renames the setting: that case is the review's, not this file's.
 */
describe("the strings", () => {
  it("has a name for every module type", async () => {
    const { PANEL_TYPES } = await import("../panels/registry");
    const modules = (it_ as { modules?: Record<string, string> }).modules ?? {};
    for (const type of PANEL_TYPES) expect(Object.keys(modules), type.module).toContain(type.module);
  });

  it("keeps the lint's eyes on the locale: the config names the folder, and the folder is there", () => {
    // Two houses for one path -- this line and the folder -- and the probe is what keeps them agreeing:
    // whoever moves the locale or retypes the glob turns this red and updates both.
    const config = readFileSync(join(GUI, "eslint.config.js"), "utf8");
    const setting = 'settings: { "vue-i18n": { localeDir: "./src/locales/*.json" } },';
    expect(config.split(setting).length - 1, "the localeDir line of eslint.config.js").toBe(1);
    expect(existsSync(join(GUI, "src/locales/it.json")), "src/locales/it.json").toBe(true);
  });
});

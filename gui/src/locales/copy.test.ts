import { describe, expect, it } from "vitest";

import it_ from "./it.json";

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
 */
describe("the strings", () => {
  it("has a name for every module type", async () => {
    const { PANEL_TYPES } = await import("../panels/registry");
    const modules = (it_ as { modules?: Record<string, string> }).modules ?? {};
    for (const type of PANEL_TYPES) expect(Object.keys(modules), type.module).toContain(type.module);
  });
});

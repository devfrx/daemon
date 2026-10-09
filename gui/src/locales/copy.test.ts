import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

// ⛔ AT THE TOP, NOT INSIDE A PROBE (P-20 of the design-system plan): an `await import` in a probe's body loads the
// module's whole graph inside the probe's 5 s -- the registry's is `vue`, `vue-i18n` and, from the design system on,
// the base pieces: from 0.5 to 5.2 s on 2026-09-24, and red twice for it. Here the file pays the load and each probe
// takes milliseconds: a guard that can go red for being slow guards nothing.
import { PANEL_TYPES } from "../panels/registry";
import { VIEWS } from "../panels/views";
import { loadFixtures } from "../schema/fixtures";
import { THEME_CHOICES } from "../tokens/theme";

import it_ from "./it.json";

const GUI = join(dirname(fileURLToPath(import.meta.url)), "..", "..");

/** The word at a dotted key of `it.json`, or `undefined` where there is none. */
function word(key: string): unknown {
  return key
    .split(".")
    .reduce<unknown>((node, part) => (node !== null && typeof node === "object" ? (node as Record<string, unknown>)[part] : undefined), it_);
}

/**
 * ⛔ WHAT NO LINT CAN DO, AND THAT IS WHY THIS FILE OUTLIVED THE NET.
 *
 * Task 13 wrote two probes here and called the file a net until task 15. Task 15 replaced the
 * first one -- bare words in a template -- with `@intlify/vue-i18n/no-raw-text` at `error`
 * (D65). It could NOT replace the probes of the keys the SPA BUILDS: a module's name,
 * `modules.${type.module}`, in the drawer and in `BigTab.ts`; a view's, `views.${view}`, in the
 * overview; a theme choice's, `settings.theme.${choice}`, in `Settings.vue`; and the words of what
 * the core sends -- an operation in Permessi and in the confirmation window, a verdict, a policy
 * and a protection in Stato, a provenance in the Chat. `no-missing-keys` is blind to a built key --
 * measured on 2026-09-15, both directions in one file (P-105).
 *
 * ⚠️ NOT RENAMED: `src/frame/keys.test.ts` already exists (task 14), and two files of that name
 * in two folders is exactly the confusion this repository pays for when re-reading.
 *
 * ⛔ AND ONE THING THE LINT CAN DO ONLY WHILE ITS EYES ARE OPEN (I-2 of the review, E197):
 * `no-missing-keys` reads the locale through `settings["vue-i18n"].localeDir` of
 * `eslint.config.js`, and when that glob stops resolving -- the folder renamed, the pattern
 * mistyped -- the rule reports NOTHING: `npm run lint` green, the gate green, measured on
 * 2026-09-22 with `localeDir` pointed at a folder that does not exist. «keeps the lint's eyes on
 * the locale» is the non-vacuity guard of that half of D65: the `settings` line of the config,
 * verbatim, and the file it points at, present. «keeps the raw-text rule at error» is the guard of
 * the other half: with the preset's `warn`, `eslint` exits 0 on a template full of bare words
 * (P-98). Nothing else in the gate watches either.
 * ⚠️ STATIC ON PURPOSE, AND THE BLIND SPOT IS DECLARED: linting a made-up key through the real
 * `ESLint` API was the first form, and it measured 4.6 s alone and 27 s -- a timeout -- inside
 * the suite on 2026-09-22, because it loads the whole chain, `typescript` included. A guard that
 * can go red for being slow guards nothing. This one costs milliseconds, and it cannot see a
 * plugin that renames the setting: that case is the review's, not this file's.
 */
describe("the strings", () => {
  it("has a name for every module type", () => {
    const modules = (it_ as { modules?: Record<string, string> }).modules ?? {};
    for (const type of PANEL_TYPES) expect(Object.keys(modules), type.module).toContain(type.module);
  });

  it("has a name for every view that ships (the overview and the bar build `views.${view}`)", () => {
    const names = (it_ as { views?: Record<string, string> }).views ?? {};
    // ⛔ NON-VACUITY: no views would leave nothing to check.
    expect(Object.keys(VIEWS).length).toBeGreaterThan(0);
    for (const view of Object.keys(VIEWS)) expect(Object.keys(names), view).toContain(view);
  });

  it("has a word for every theme choice", () => {
    const words = (it_ as { settings?: { theme?: Record<string, string> } }).settings?.theme ?? {};
    // ⛔ NON-VACUITY: no choices would leave nothing to check.
    expect(THEME_CHOICES.length).toBeGreaterThan(0);
    for (const choice of THEME_CHOICES) expect(Object.keys(words), choice).toContain(choice);
  });

  it("has a word for every value the core sends into a key the SPA builds (AUD-2199 of the audit of 2026-09-30)", () => {
    // ⛔ THE VALUES ARE THE KERNEL'S, READ FROM ITS CANONICAL SET AND NOT RETYPED: `stamp_set` carries every variant of every
    // nested enum (`every_variant_of_every_nested_enum_is_in_the_canonical_set`, `crates/kernel/tests/ipc_wire.rs`), so a
    // variant added on the wire brings its fixture, and this probe asks for its word. Missing, the word would be the key's
    // own path on screen -- and the probes of the panels, which read their oracle through the same `t`, stay green.
    const built: string[] = [];
    const provenances = new Set<string>();
    for (const { message } of loadFixtures()) {
      if (message.kind === "PermissionRequired") built.push(`permissions.operation.${message.value.operation}`);
      if (message.kind === "Approve") built.push(`permissions.operation.${message.triple.operation}`);
      if (message.kind === "Verdict") built.push(`status.verdict.${message.value.verdict}`);
      if (message.kind === "Policy") built.push(`status.policyName.${message.value.policy}`);
      if (message.kind === "Accepted") built.push(`status.protectionValue.${message.value}`);
      if (message.kind === "Token") provenances.add(message.provenance);
    }
    // The Chat's `label()` picks one of two keys by the provenance of a piece: the two, for the two provenances there are.
    expect([...provenances].sort()).toEqual(["Trusted", "Untrusted"]);
    built.push("chat.trusted", "chat.untrusted");
    // ⛔ NON-VACUITY: every family met a value.
    for (const family of ["permissions.operation.", "status.verdict.", "status.policyName.", "status.protectionValue."]) {
      expect(built.some((key) => key.startsWith(family)), family).toBe(true);
    }
    expect(built.filter((key) => typeof word(key) !== "string")).toEqual([]);
  });

  it("keeps the raw-text rule at error, and off for the kit page alone (AUD-2200 of the audit of 2026-09-30)", () => {
    // The first half of D65, held like the second: the rule's line verbatim -- its level, and the punctuation `ignoreText`
    // lets through (D91) -- and its one other mention, the kit page's `off` (D8 of the design-system plan). A block that
    // set the rule again, to `warn` or `off` for other files, is a third mention.
    const config = readFileSync(join(GUI, "eslint.config.js"), "utf8");
    const rule = '"@intlify/vue-i18n/no-raw-text": ["error", { ignoreText: [":", "—"] }],';
    expect(config.split(rule).length - 1, "the no-raw-text line of eslint.config.js").toBe(1);
    expect(config.split('"@intlify/vue-i18n/no-raw-text"').length - 1, "the mentions of no-raw-text").toBe(2);
    expect(config).toMatch(/files: \["src\/kit\/\*\*"\],\r?\n\s*rules: \{ "@intlify\/vue-i18n\/no-raw-text": "off" \},/);
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

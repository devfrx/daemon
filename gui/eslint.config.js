import i18n from "@intlify/eslint-plugin-vue-i18n";
import tsParser from "@typescript-eslint/parser";
import vue from "eslint-plugin-vue";

/** The one door of the icons (answer 11 of the design system). */
const LUCIDE = {
  name: "lucide",
  message: "icons pass through BaseIcon and the one map, src/components/icons.ts (answer 11 of the design system)",
};
/** What a base piece may not reach: the global state, and every layer above the kit (section (b); P-3 of its plan). */
const UPWARD = {
  regex: "^pinia$|(^|/)(stores|panels|frame|transport)(/|$)",
  message: "a base piece reads no global state and knows no layer above it (design system, section (b))",
};
/** What no file of the kit may reach: the layers above it (section (b); R2-11 of the design-system review). */
const ABOVE = {
  regex: "(^|/)(panels|frame|transport)(/|$)",
  message: "the kit knows no layer above it: panels, frame and transport use the kit, not the other way (design system, section (b))",
};

/**
 * ⛔ `flat/essential` AND NOT `flat/recommended`, AND `--max-warnings 0` IS NOT USED.
 *
 * `flat/recommended` carries 33 warn-level rules that are almost all FORMATTING -- `html-indent`,
 * `html-quotes`, `max-attributes-per-line` -- and line 8 of `scripts/gate.sh` forbids, at the top of
 * the gate, a red that means "questionable style" (§7.4.3 of the compendium). `--max-warnings 0`
 * looks like the cure and is the opposite: it would promote exactly those to blocking.
 * `flat/essential` carries 85 rules and ZERO warnings, so every line this step prints is an `error`
 * and every `error` stops the gate. The command that re-measures both numbers lives in P-100.
 */
export default [
  ...vue.configs["flat/essential"],
  ...i18n.configs["flat/base"],
  {
    /**
     * ⛔ THE TypeScript PARSER FOR THE `.vue` FILES: WITHOUT IT A `<script setup lang="ts">` DOES NOT PARSE.
     * `vue-eslint-parser` reads a `<script setup lang="ts">` block with espree unless it is given one,
     * and TypeScript syntax stops espree -- measured on 2026-09-16 on the thirteen `.vue` of this plan:
     * `Parsing error` on `Confirm`, `Frame`, `ViewBar`, `Chat`, `Placeholder`, `Settings`. A file that
     * does not parse gets NO rule at all, so without this block the scoped exception below could not be
     * proven either (R8-1). The `.ts` files are read from the design system on (block `harness/ts`, P-2 of its plan);
     * their TYPES stay with `vue-tsc` inside `npm run build`, the level 1 of the web world (R2-10 of its review).
     */
    name: "harness/ts-in-vue",
    files: ["**/*.vue"],
    languageOptions: { parserOptions: { parser: tsParser } },
  },
  {
    /**
     * ⛔ THE `.ts` FILES ARE READ FROM THE DESIGN SYSTEM ON (P-2 of its plan). P-101 of part 2 left them to `vue-tsc`,
     * and their TYPES stay there; but the import rules below must see `icons.ts`, `BigTab.ts`, `dock.ts`, or `lucide` --
     * and, from task 5, `reka-ui` -- could come in through a `.ts` unseen. The unscoped blocks now reach the `.ts` too,
     * and the gate says whether any of them objects.
     */
    name: "harness/ts",
    files: ["**/*.ts"],
    languageOptions: { parser: tsParser },
  },
  {
    name: "harness/settings",
    settings: { "vue-i18n": { localeDir: "./src/locales/*.json" } },
  },
  {
    name: "harness/rules",
    rules: {
      /**
       * ⛔ OFF, AND RENAMING IS NOT THE CURE. The file name of a panel IS the `module` of the
       * `PANEL_TYPES` registry (task 13) and IS the `modules.*` key of the locale (task 13 too, step 5).
       * Renaming to please a lint would move two houses that have nothing to do with the lint.
       * When it was turned off, every `.vue` file here but `ViewBar` was single-word (P-99;
       * recounted at the review, R7-11); from the design system's task 3 on, the kit's base
       * pieces are `Base*`, two words by name.
       */
      "vue/multi-word-component-names": "off",
      /**
       * ⛔ NOT IN `essential`, SO IT IS TURNED ON BY HAND. ADR-0016 cares about what the GUI is
       * made to render; the one legitimate `v-html` is Chat's, and it has its own block below.
       */
      "vue/no-v-html": "error",
      /**
       * ⛔ `error` AND NOT THE PRESET'S `warn`, OR THIS CONTROL CANNOT GO RED. Measured: with the
       * preset's level, `eslint` exits 0 on a template full of raw text (P-98). This rule is what
       * replaces the FIRST probe of `src/locales/copy.test.ts`; the second one survives, because it
       * watches keys the SPA BUILDS and no lint can see those (P-105).
       *
       * ⛔ `ignoreText` IS PUNCTUATION AND NOT AN ESCAPE HATCH (D91). Measured on 2026-09-16 on the
       * thirteen `.vue` of this plan: seven errors on bare `:` and `—` between two mustaches, in
       * `ViewBar`, `Status` and `Strip` -- which tasks 13 and 14 dictate and whose own net does not see
       * (it wants two letters), so 13 and 14 would close green and THIS task would be born red. The list
       * only silences a text node that IS one of those characters: `{{ a }}: ciao — {{ a }}` still goes
       * red with `raw text ': ciao —' is used`, measured in both directions.
       */
      "@intlify/vue-i18n/no-raw-text": ["error", { ignoreText: [":", "—"] }],
      /**
       * ⛔ The other half: a key written in a template and missing from `it.json`. Blind to a built
       * key -- measured, both directions in one file (P-105).
       */
      "@intlify/vue-i18n/no-missing-keys": "error",
    },
  },
  {
    /**
     * ⛔ THE ONE EXCEPTION, IN ONE PLACE. `Chat.vue` renders HTML WE produced, from text
     * `renderMarkdown` has already escaped, with links as visible text and images never as `<img>`
     * (D54). Scoped to the file rather than scattered across two `eslint-disable` comments, so a
     * reviewer finds every exception by reading this file.
     */
    name: "harness/chat-renders-our-own-html",
    files: ["src/panels/Chat.vue"],
    rules: { "vue/no-v-html": "off" },
  },
  {
    // ⛔ THE KIT PAGE'S WORDS ARE SPECIMENS (D8 of the design-system plan): a development page outside the package, whose
    // words in `it.json` would ship for nothing. The one exception to the raw-text rule, in one place, like Chat's above.
    name: "harness/kit-page-specimens",
    files: ["src/kit/**"],
    rules: { "@intlify/vue-i18n/no-raw-text": "off" },
  },
  /**
   * ⛔ THE IMPORT RULES OF THE KIT (design system, section (b)). One rule, a scope per block, and the ORDER MATTERS: in a flat
   * config a later block REPLACES an earlier one's options for the same rule, it does not merge them -- so every block
   * says the whole list for its files.
   */
  {
    name: "harness/imports",
    files: ["**/*.vue", "**/*.ts"],
    rules: { "no-restricted-imports": ["error", { paths: [LUCIDE] }] },
  },
  {
    // ⛔ THE WHOLE KIT KNOWS NO LAYER ABOVE IT (section (b); R2-11 of the design-system review, the owner's choice A). A
    // composed piece such as `Confirm.vue` may read the stores, which the base pieces may not -- their block below says so,
    // and comes AFTER this one because it replaces it for their files. The tests are out: `kit.test.ts` reads
    // `PANEL_TYPES`, to check that every module type has an icon.
    name: "harness/imports/components",
    files: ["src/components/**/*.{vue,ts}"],
    ignores: ["src/components/**/*.test.ts"],
    rules: { "no-restricted-imports": ["error", { paths: [LUCIDE], patterns: [ABOVE] }] },
  },
  {
    name: "harness/imports/base-pieces",
    files: ["src/components/Base*.vue"],
    rules: { "no-restricted-imports": ["error", { paths: [LUCIDE], patterns: [UPWARD] }] },
  },
  {
    // The one file that may import `lucide`, and the only rule it keeps is the base pieces' one.
    name: "harness/imports/the-icon-map",
    files: ["src/components/icons.ts"],
    rules: { "no-restricted-imports": ["error", { patterns: [UPWARD] }] },
  },
];

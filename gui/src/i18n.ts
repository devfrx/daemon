import { createI18n } from "vue-i18n";

import it from "./locales/it.json";

/**
 * ⛔ ITALIAN ONLY, AND THAT IS A DECISION (G21): one locale, no fallback chain, no language
 * picker. What this buys today is not translation -- it is that every string has a KEY, so the
 * day a second locale is wanted nothing has to be hunted for in templates.
 *
 * ⚠️ `legacy: false` because the SPA is composition API throughout; the legacy mode would put
 * `$t` on every component instance and make the probes depend on a global.
 */
export const i18n = createI18n({
  legacy: false,
  locale: "it",
  fallbackLocale: "it",
  messages: { it },
});

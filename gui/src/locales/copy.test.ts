import { readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

import it_ from "./it.json";

const SRC = join(dirname(fileURLToPath(import.meta.url)), "..");

/**
 * ⛔ A NET, NOT A LINT, AND IT DIES AT TASK 15. The real rule is `no-raw-text` of
 * `@intlify/eslint-plugin-vue-i18n` (decision 55), and it enters the gate with
 * `scripts/gate-gui.sh` -- which is task 15. Between this task and that one, nothing would watch
 * the strings that G21 requires, and whoever reviews would look for a probe that does not exist.
 *
 * ⚠️ AND IT IS WORTH SAYING WHAT IT CANNOT DO: it reads raw text, so it knows nothing of Vue's
 * syntax and nothing of the exceptions a lint rule declares. Task 15 REPLACES it.
 */
function templates(dir: string): string[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((entry) =>
    entry.isDirectory() ? templates(join(dir, entry.name))
    : entry.name.endsWith(".vue") ? [join(dir, entry.name)]
    : []
  );
}

describe("the strings", () => {
  it("are all keys: no bare words between tags in a template", () => {
    const offenders: string[] = [];
    for (const file of templates(SRC)) {
      const body = readFileSync(file, "utf8");
      const template = /<template>([\s\S]*)<\/template>/.exec(body)?.[1] ?? "";
      for (const text of template.matchAll(/>([^<>{}]*[A-Za-zÀ-ÿ]{2,}[^<>{}]*)</g)) {
        offenders.push(`${file}: ${text[1]?.trim()}`);
      }
    }
    expect(offenders).toEqual([]);
  });

  it("has a name for every module type", async () => {
    const { PANEL_TYPES } = await import("../panels/registry");
    const modules = (it_ as { modules?: Record<string, string> }).modules ?? {};
    for (const type of PANEL_TYPES) expect(Object.keys(modules), type.module).toContain(type.module);
  });
});

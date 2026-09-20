import type { IpcMessage } from "./messages";
import { parseIpcMessage } from "./parse";

export interface Fixture {
  /** The file name the kernel's generator produced, e.g. `00-hello.json`. */
  file: string;
  message: IpcMessage;
}

/**
 * ⛔ `import.meta.glob` AND NOT `fs`, and it is a requirement rather than a taste: the fake
 * bridge must run IN THE BROWSER (§6a of the sub-project 2 design), where there is no `fs`. Vite
 * inlines these at build time and vitest resolves them the same way, so the probes and the
 * browser read THE SAME files -- one loader, not two that can disagree.
 *
 * ⚠️ THE PATH REACHES OUTSIDE `src/` on purpose: `gui/schema/fixtures/` is written by
 * `regenerate_the_fixtures` in `crates/kernel/tests/ipc_wire.rs`, and nothing under `gui/src/`
 * may write there.
 */
const FILES = import.meta.glob<unknown>("../../schema/fixtures/*.json", {
  eager: true,
  import: "default",
});

export function loadFixtures(): Fixture[] {
  return Object.entries(FILES)
    .map(([path, value]) => ({
      file: path.slice(path.lastIndexOf("/") + 1),
      message: parseIpcMessage(value),
    }))
    .sort((left, right) => left.file.localeCompare(right.file));
}

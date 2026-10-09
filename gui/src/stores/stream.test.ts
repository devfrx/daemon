import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";

import { loadFixtures } from "../schema/fixtures";
import { createFakeBridge } from "../transport/fakeBridge";

import { FREEZE_AT, KEEP, useStream } from "./stream";

/** SP-8's chat tile, the one M4 was measured on: read, not retyped. */
const TILE = readFileSync(
  join(dirname(fileURLToPath(import.meta.url)), "..", "..", "..", "spikes", "gui-shell", "app", "src", "tiles", "Chat.vue"),
  "utf8",
);

beforeEach(() => {
  setActivePinia(createPinia());
});

describe("the stream", () => {
  it("takes the Token fixture the kernel generated, with its provenance", () => {
    const bridge = createFakeBridge();
    const stream = useStream();
    bridge.listen((message) => stream.receive(message));
    bridge.deliver("Token");
    expect(stream.current?.text).toBe("ciao");
    expect(stream.current?.provenance).toBe("Untrusted");
    expect(stream.blocks).toHaveLength(0);
  });

  it("keeps the threshold and the window M4 was measured with: SP-8's tile's own numbers (D61; AUD-2210 of the audit of 2026-09-30)", () => {
    // ⛔ THE NUMBERS A MEASURE SITS BEHIND, HELD TO THE TILE IT WAS TAKEN ON: every other probe reads FREEZE_AT and KEEP by
    // name, and a changed constant left them all green -- while D61 says that whoever changes one leaves the measure.
    const constant = (name: string): number => Number(new RegExp(`const ${name} = (\\d+);`).exec(TILE)?.[1]);
    expect([FREEZE_AT, KEEP]).toEqual([constant("FREEZE_AT"), constant("KEEP")]);
  });

  it("concatenates verbatim and freezes at the threshold", () => {
    const stream = useStream();
    const piece = "x".repeat(FREEZE_AT / 4);
    for (let n = 0; n < 4; n += 1) stream.receive({ kind: "Token", text: piece, provenance: "Untrusted" });
    // ⛔ THE SECOND DIRECTION IS IN THE COUNTS: one frozen block of exactly FREEZE_AT, and nothing
    // open -- a store that never froze would have a current of FREEZE_AT characters, the four pieces, and no blocks.
    expect(stream.blocks).toHaveLength(1);
    expect(stream.blocks[0]?.text).toHaveLength(FREEZE_AT);
    expect(stream.current).toBeNull();
  });

  it("closes a block when the provenance changes, so every piece carries one label", () => {
    const stream = useStream();
    stream.receive({ kind: "Token", text: "a", provenance: "Untrusted" });
    stream.receive({ kind: "Token", text: "b", provenance: "Trusted" });
    expect(stream.blocks.map((block) => [block.text, block.provenance])).toEqual([["a", "Untrusted"]]);
    expect(stream.current).toEqual({ id: 1, text: "b", provenance: "Trusted" });
  });

  it("keeps the last KEEP frozen blocks and drops the oldest, each keeping the id it opened with (AUD-539)", () => {
    const stream = useStream();
    for (let n = 0; n < KEEP + 3; n += 1) {
      stream.receive({ kind: "Token", text: `${n}:` + "y".repeat(FREEZE_AT), provenance: "Untrusted" });
    }
    expect(stream.blocks).toHaveLength(KEEP);
    expect(stream.blocks[0]?.text.startsWith("3:")).toBe(true);
    // ⛔ THE ID FOLLOWS THE BLOCK, NOT THE POSITION (AUD-539 of the audit of 2026-09-30): the three dropped took 0, 1 and
    // 2 with them, and the first block kept is still 3 -- what the Chat keys its frozen articles by.
    expect(stream.blocks.map((block) => block.id)).toEqual(Array.from({ length: KEEP }, (_, index) => index + 3));
  });

  it("ignores every other kind: each one the kernel's canonical set carries (AUD-2210 of the audit of 2026-09-30)", () => {
    const stream = useStream();
    const others = loadFixtures().filter(({ message }) => message.kind !== "Token");
    // ⛔ NON-VACUITY: every kind but the Token is here, not one of them.
    expect(new Set(others.map(({ message }) => message.kind)).size).toBeGreaterThan(1);
    for (const { message } of others) stream.receive(message);
    expect(stream.current).toBeNull();
    expect(stream.blocks).toHaveLength(0);
  });
});

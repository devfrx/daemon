import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";

import { createFakeBridge } from "../transport/fakeBridge";

import { FREEZE_AT, KEEP, useStream } from "./stream";

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

  it("concatenates verbatim and freezes at the threshold M4 was measured with", () => {
    const stream = useStream();
    const piece = "x".repeat(FREEZE_AT / 4);
    for (let n = 0; n < 4; n += 1) stream.receive({ kind: "Token", text: piece, provenance: "Untrusted" });
    // ⛔ THE SECOND DIRECTION IS IN THE COUNTS: one frozen block of exactly FREEZE_AT, and nothing
    // open -- a store that never froze would have a current of 4 * FREEZE_AT and no blocks.
    expect(stream.blocks).toHaveLength(1);
    expect(stream.blocks[0]?.text).toHaveLength(FREEZE_AT);
    expect(stream.current).toBeNull();
  });

  it("closes a block when the provenance changes, so every piece carries one label", () => {
    const stream = useStream();
    stream.receive({ kind: "Token", text: "a", provenance: "Untrusted" });
    stream.receive({ kind: "Token", text: "b", provenance: "Trusted" });
    expect(stream.blocks.map((block) => [block.text, block.provenance])).toEqual([["a", "Untrusted"]]);
    expect(stream.current).toEqual({ text: "b", provenance: "Trusted" });
  });

  it("keeps the last KEEP frozen blocks and drops the oldest", () => {
    const stream = useStream();
    for (let n = 0; n < KEEP + 3; n += 1) {
      stream.receive({ kind: "Token", text: `${n}:` + "y".repeat(FREEZE_AT), provenance: "Untrusted" });
    }
    expect(stream.blocks).toHaveLength(KEEP);
    expect(stream.blocks[0]?.text.startsWith("3:")).toBe(true);
  });

  it("ignores every other kind", () => {
    const stream = useStream();
    stream.receive({ kind: "Accepted", value: "AsSystemAccount" });
    expect(stream.current).toBeNull();
    expect(stream.blocks).toHaveLength(0);
  });
});

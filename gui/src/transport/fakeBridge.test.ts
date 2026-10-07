import { describe, expect, it } from "vitest";
import { reactive } from "vue";

import { MESSAGE_KINDS } from "../schema/parse";

import { createFakeBridge } from "./fakeBridge";

describe("the fake bridge", () => {
  it("delivers a fixture to whoever is listening", () => {
    const bridge = createFakeBridge();
    const heard: string[] = [];
    bridge.listen((message) => heard.push(message.kind));
    bridge.deliver("Policy");
    expect(heard).toEqual(["Policy"]);
  });

  it("delivers every kind, so the SPA can be built against all of them", () => {
    const bridge = createFakeBridge();
    const heard = new Set<string>();
    bridge.listen((message) => heard.add(message.kind));
    bridge.deliverAll();
    expect([...heard].sort()).toEqual([...MESSAGE_KINDS].sort());
  });

  it("stops delivering to a listener that unsubscribed", () => {
    const bridge = createFakeBridge();
    const heard: string[] = [];
    const stop = bridge.listen((message) => heard.push(message.kind));
    bridge.deliver("Policy");
    stop();
    bridge.deliver("Policy");
    // ⛔ THE SECOND DELIVERY IS THE POINT: without it the probe would be green on a `listen`
    // that never removes anything, because a listener that keeps hearing still heard once.
    expect(heard).toEqual(["Policy"]);
  });

  it("records what the SPA sends, and nothing else", () => {
    const bridge = createFakeBridge();
    bridge.send({ kind: "Hello", value: "1" });
    bridge.deliver("Accepted");
    expect(bridge.sent).toEqual([{ kind: "Hello", value: "1" }]);
  });

  it("takes a message the way the shell's IPC does: a copy of plain data, and never a store's proxy (AUD-541 of the audit of 2026-09-30)", () => {
    const bridge = createFakeBridge();
    const triple = { tool: "registry", resource: "arbiter", operation: "Write" } as const;
    const call = { function: "vram-policy", argument: "local" };
    bridge.send({ kind: "Approve", triple, call });
    // A copy: the same values, and not the sender's objects.
    expect(bridge.sent).toEqual([{ kind: "Approve", triple, call }]);
    expect(bridge.sent[0]?.kind === "Approve" && bridge.sent[0].triple).not.toBe(triple);
    // ⛔ THE OTHER DIRECTION, the one the shell would find first: a reactive value of a store cannot be cloned, and
    // nothing is recorded for it.
    expect(() => bridge.send({ kind: "Approve", triple: reactive({ ...triple }), call })).toThrow(/clone/);
    expect(bridge.sent).toHaveLength(1);
  });
});

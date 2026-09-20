import { describe, expect, it } from "vitest";

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
});

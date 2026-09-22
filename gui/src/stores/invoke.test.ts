import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";

import type { Call, Triple } from "../schema/messages";
import { createFakeBridge } from "../transport/fakeBridge";

import { useCore } from "./core";
import { useInvoke } from "./invoke";

const CALL: Call = { function: "vram-policy", argument: "local" };
const TRIPLE: Triple = { tool: "registry", resource: "arbiter", operation: "Write" };

beforeEach(() => {
  setActivePinia(createPinia());
});

describe("the invocation", () => {
  it("sends Invoke and keeps the call in flight", () => {
    const bridge = createFakeBridge();
    const invoke = useInvoke();
    invoke.attach(bridge);
    invoke.send(CALL);
    expect(bridge.sent).toEqual([{ kind: "Invoke", value: CALL }]);
    expect(invoke.inFlight).toEqual(CALL);
  });

  it("approves only with a triple asked AND a call in flight, and Approve carries both", () => {
    const bridge = createFakeBridge();
    const invoke = useInvoke();
    const core = useCore();
    invoke.attach(bridge);
    // ⛔ THE TWO NEGATIVE HALVES FIRST: neither alone sends anything.
    expect(invoke.approve()).toBe(false);
    core.receive({ kind: "PermissionRequired", value: TRIPLE });
    expect(invoke.approve()).toBe(false);
    expect(bridge.sent).toEqual([]);
    core.settled();
    invoke.send(CALL);
    expect(invoke.approve()).toBe(false);
    core.receive({ kind: "PermissionRequired", value: TRIPLE });
    expect(invoke.approve()).toBe(true);
    expect(bridge.sent.at(-1)).toEqual({ kind: "Approve", triple: TRIPLE, call: CALL });
    expect(core.pending).toBeNull();
    expect(invoke.inFlight).toBeNull();
    expect(invoke.approved).toEqual([TRIPLE]);
  });

  it("refuses locally: nothing is sent, and nothing stays pending", () => {
    const bridge = createFakeBridge();
    const invoke = useInvoke();
    const core = useCore();
    invoke.attach(bridge);
    invoke.send(CALL);
    core.receive({ kind: "PermissionRequired", value: TRIPLE });
    invoke.refuse();
    expect(bridge.sent.map((message) => message.kind)).toEqual(["Invoke"]);
    expect(core.pending).toBeNull();
    expect(invoke.inFlight).toBeNull();
    expect(invoke.approved).toEqual([]);
  });

  it("takes Policy as the call having landed", () => {
    const bridge = createFakeBridge();
    const invoke = useInvoke();
    invoke.attach(bridge);
    invoke.send(CALL);
    bridge.listen((message) => invoke.receive(message));
    bridge.deliver("Policy");
    expect(invoke.inFlight).toBeNull();
  });
});

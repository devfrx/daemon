import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";

import type { IpcMessage } from "../schema/messages";
import { createFakeBridge } from "../transport/fakeBridge";

import { useConnection } from "./connection";
import { useCore } from "./core";
import { pack_, unpack, useLayout } from "./layout";

beforeEach(() => {
  setActivePinia(createPinia());
});

describe("the connection", () => {
  it("starts waiting and sends Hello with the stamp from the map", () => {
    const bridge = createFakeBridge();
    const connection = useConnection();
    connection.attach(bridge);
    connection.hello();
    expect(connection.phase).toBe("waiting");
    expect(bridge.sent).toHaveLength(1);
    const sent = bridge.sent[0];
    expect(sent?.kind).toBe("Hello");
    // ⛔ THE ORACLE IS NOT "a string": it is a DECIMAL string that round-trips through BigInt.
    // "0x…" or a rounded Number would both be truthy and both wrong.
    expect(sent?.kind === "Hello" && /^[0-9]+$/.test(sent.value)).toBe(true);
  });

  it("becomes connected on Accepted, and stale on StaleBuild", () => {
    const connection = useConnection();
    connection.receive({ kind: "Accepted", value: "AsSystemAccount" });
    expect(connection.phase).toBe("connected");
    expect(connection.protection).toBe("AsSystemAccount");
    connection.receive({ kind: "StaleBuild", value: "81985529216486895" });
    expect(connection.phase).toBe("stale");
    expect(connection.expected).toBe("81985529216486895");
  });

  it("goes back to waiting on retry, and sends Hello again", () => {
    const bridge = createFakeBridge();
    const connection = useConnection();
    connection.attach(bridge);
    connection.receive({ kind: "Accepted", value: "AsSystemAccount" });
    connection.retry();
    expect(connection.phase).toBe("waiting");
    expect(connection.protection).toBeNull();
    expect(bridge.sent.map((m) => m.kind)).toEqual(["Hello"]);
  });
});

describe("the core state", () => {
  it("takes every fixture the kernel generated without throwing", () => {
    const bridge = createFakeBridge();
    const core = useCore();
    const connection = useConnection();
    bridge.listen((message) => {
      core.receive(message);
      connection.receive(message);
    });
    bridge.deliverAll();
    // ⛔ NOT A VACUOUS "it did not throw": the three fields the strip reads must be POPULATED,
    // because `deliverAll` carries a Degradation, a Policy and a Steps.
    expect(core.degradation).not.toBeNull();
    expect(core.policy).not.toBeNull();
    expect(core.steps.length).toBeGreaterThan(0);
  });

  it("replaces the step list instead of appending it", () => {
    const core = useCore();
    const one: IpcMessage = { kind: "Steps", value: [{ step: "1", function: "a", done: true }] };
    core.receive(one);
    core.receive(one);
    expect(core.steps).toHaveLength(1);
  });
});

describe("the layout", () => {
  it("takes a Package and reads the view out of it", () => {
    const layout = useLayout();
    const bytes = [...pack_({ view: "work", layouts: {} })];
    layout.receive({ kind: "Layout", value: { state: "Package", bytes } });
    expect(layout.state.state).toBe("Package");
    expect(layout.view).toBe("work");
  });

  it("keeps the default view on Nothing and on Unavailable", () => {
    // ⛔ THE TWO CASES THE FIXTURES CANNOT REACH (D46): `stamp_set` carries ONE message per
    // variant, so `deliver("Layout")` only ever delivers `Package`. The type system is what
    // keeps this honest -- `LayoutState` has three variants and not one more.
    for (const value of [{ state: "Nothing" }, { state: "Unavailable" }] as const) {
      setActivePinia(createPinia());
      const layout = useLayout();
      layout.receive({ kind: "Layout", value });
      expect(layout.state.state).toBe(value.state);
      expect(layout.view).toBe("home");
    }
  });

  it("refuses a package it cannot read, instead of half-applying it", () => {
    expect(unpack({ state: "Package", bytes: [0x7b, 0x7d] })).toBeNull();
    expect(unpack({ state: "Package", bytes: [0x00, 0x01] })).toBeNull();
    expect(unpack({ state: "Nothing" })).toBeNull();
    // ⛔ THE SHAPE BEFORE D80 -- one `layout` for every view -- is an old build's package now,
    // and is refused like any other: the committed views win over a package nobody can read.
    const before = [...new TextEncoder().encode('{"view":"home","layout":{}}')];
    expect(unpack({ state: "Package", bytes: before })).toBeNull();
  });

  it("sends SaveLayout as bytes when the layout settles", () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    layout.settle({} as never);
    const sent = bridge.sent[0];
    expect(sent?.kind).toBe("SaveLayout");
    expect(sent?.kind === "SaveLayout" && sent.value.length).toBeGreaterThan(0);
  });

  it("merges the open view into the package it holds, and keeps the other views (D80)", () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    const home = { marker: "home, as the owner left it" } as never;
    layout.receive({ kind: "Layout", value: { state: "Package", bytes: [...pack_({ view: "home", layouts: { home } })] } });
    layout.view = "work";
    const work = { marker: "work, just settled" } as never;
    layout.settle(work);
    const sent = bridge.sent[0];
    // ⛔ NOT "something was sent": the package must carry BOTH views and name the open one. A
    // `settle` that replaced the package would pass the probe above and lose Home here.
    const pack = sent?.kind === "SaveLayout" ? unpack({ state: "Package", bytes: sent.value }) : null;
    expect(pack).toEqual({ view: "work", layouts: { home, work } });
  });

  it("does not count its own save coming back, and does count any other package (D89)", () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    layout.settle({ marker: "mine" } as never);
    const sent = bridge.sent[0];
    const echo = sent?.kind === "SaveLayout" ? sent.value : [];
    expect(echo.length).toBeGreaterThan(0);
    layout.receive({ kind: "Layout", value: { state: "Package", bytes: echo } });
    // ⛔ THE ECHO OF DECISION 13: what the core holds is what we sent. Nothing arrived.
    expect(layout.arrivals).toBe(0);
    // A different package -- the welcome, or the OLD one after a write that did not stick -- did.
    const theirs = { marker: "theirs" } as never;
    layout.receive({ kind: "Layout", value: { state: "Package", bytes: [...pack_({ view: "compact", layouts: { compact: theirs } })] } });
    expect(layout.arrivals).toBe(1);
    expect(layout.view).toBe("compact");
    expect(layout.saved).toEqual({ view: "compact", layouts: { compact: theirs } });
  });
});

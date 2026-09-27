import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";

import type { IpcMessage } from "../schema/messages";
import { createFakeBridge } from "../transport/fakeBridge";

import { useConnection } from "./connection";
import { useCore } from "./core";
import { pack_, unpack, useLayout, type LayoutPack } from "./layout";

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

describe("the theme in the package (design system, section (a))", () => {
  it("keeps the choice a package carries, and opens one without it as `system`", () => {
    const layout = useLayout();
    layout.receive({ kind: "Layout", value: { state: "Package", bytes: [...pack_({ view: "home", layouts: {}, theme: "light" })] } });
    expect(layout.theme).toBe("light");
    // ⛔ THE SECOND DIRECTION: a package written before the field existed still opens -- as `system`.
    setActivePinia(createPinia());
    const older = useLayout();
    older.receive({ kind: "Layout", value: { state: "Package", bytes: [...pack_({ view: "work", layouts: {} })] } });
    expect(older.view).toBe("work");
    expect(older.theme).toBe("system");
  });

  it("reads a choice it does not know as absent, without refusing the package", () => {
    const bytes = [...new TextEncoder().encode('{"view":"home","layouts":{},"theme":"purple"}')];
    expect(unpack({ state: "Package", bytes })).toEqual({ view: "home", layouts: {} });
  });

  it("sends the choice at once, and a settle after it keeps it", () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    const work = { marker: "work, as the owner left it" } as never;
    layout.receive({ kind: "Layout", value: { state: "Package", bytes: [...pack_({ view: "home", layouts: { work } })] } });
    layout.chooseTheme("dark");
    const first = bridge.sent[0];
    const chosen = first?.kind === "SaveLayout" ? unpack({ state: "Package", bytes: first.value }) : null;
    // ⛔ NOT THE CHOICE ALONE: a choice that rebuilt the package around the theme would drop every layout the owner
    // saved, and the core would keep the loss (E2 of the design-system plan).
    expect(chosen).toEqual({ view: "home", layouts: { work }, theme: "dark" });
    expect(layout.theme).toBe("dark");
    const home = { marker: "home, as the owner left it" } as never;
    layout.settle(home);
    const second = bridge.sent[1];
    const settled = second?.kind === "SaveLayout" ? unpack({ state: "Package", bytes: second.value }) : null;
    // ⛔ NOT "something was sent": a settle that rebuilt the package from `view` and `layouts` alone would drop
    // the choice at the first move of a panel.
    expect(settled).toEqual({ view: "home", layouts: { work, home }, theme: "dark" });
  });
});

describe("the named views in the package (design system, section (d))", () => {
  /** A package from the core, as `receive` gets it. */
  function fromTheCore(pack: LayoutPack): IpcMessage {
    return { kind: "Layout", value: { state: "Package", bytes: [...pack_(pack)] } };
  }

  /** What the store sent, read back: the n-th `SaveLayout`. */
  function sentPack(bridge: ReturnType<typeof createFakeBridge>, index: number): LayoutPack | null {
    const message = bridge.sent[index];
    return message?.kind === "SaveLayout" ? unpack({ state: "Package", bytes: message.value }) : null;
  }

  it("keeps the named views and the open one a package carries, and opens one without them as before (control 18)", () => {
    const layout = useLayout();
    const review = { marker: "the owner's review" } as never;
    layout.receive(fromTheCore({ view: "work", layouts: {}, named: [{ name: "Revisione", layout: review }], openNamed: "Revisione" }));
    expect(layout.saved?.named).toEqual([{ name: "Revisione", layout: review }]);
    expect(layout.openNamed).toBe("Revisione");
    expect(layout.view).toBe("work");
    // ⛔ THE SECOND DIRECTION: a package written before the list existed opens as before, no named view open.
    setActivePinia(createPinia());
    const older = useLayout();
    older.receive(fromTheCore({ view: "compact", layouts: {} }));
    expect(older.saved).toEqual({ view: "compact", layouts: {} });
    expect(older.openNamed).toBeNull();
  });

  it("drops what it cannot read -- and a key it does not know inside `layouts` stays dropped (row 8 of §2)", () => {
    const text = JSON.stringify({
      view: "home",
      layouts: { home: {}, mine: {} },
      named: [{ name: "Revisione", layout: {} }, { name: "", layout: {} }, { name: "Senza" }, { layout: {} }, null, { name: "Nulla", layout: null }, { name: " revisione ", layout: { second: true } }],
      openNamed: "Sparita",
    });
    // ⛔ A NAME ALREADY READ IS A SECOND VIEW UNDER IT (D4): the first stays. An open name the list does not hold is
    // read as absent, and the view of always opens. And an entry that is `null`, or holds a `null` layout, is dropped
    // (E88 of the plan): without its guard the first throws inside `unpack`'s `try`, and the whole package is lost.
    expect(unpack({ state: "Package", bytes: [...new TextEncoder().encode(text)] })).toEqual({
      view: "home",
      layouts: { home: {} },
      named: [{ name: "Revisione", layout: {} }],
    });
  });

  it("writes a move in an open named view into THAT view, and leaves the three and the other named views as they were (R3-19)", () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    const home = { marker: "home, as the owner left it" } as never;
    // ⛔ TWO NAMED VIEWS, THE OPEN ONE SECOND (E82 of the plan): with one alone, "that view" reads the same as "every
    // named view" or "the first", and a move written into the wrong one loses it in silence (D4).
    const other = { marker: "another named view" } as never;
    layout.receive(fromTheCore({ view: "home", layouts: { home }, named: [{ name: "Altra", layout: other }, { name: "Revisione", layout: { marker: "before" } as never }], openNamed: "Revisione" }));
    const moved = { marker: "the review, one panel moved" } as never;
    layout.settle(moved);
    // ⛔ `layouts.home` AS IT WAS: before R3-19 a settle wrote `layouts[view]` whatever was on screen.
    expect(sentPack(bridge, 0)).toEqual({ view: "home", layouts: { home }, named: [{ name: "Altra", layout: other }, { name: "Revisione", layout: moved }], openNamed: "Revisione" });
    layout.chooseTheme("light");
    // And a theme chosen meanwhile keeps the named view open.
    expect(sentPack(bridge, 1)).toEqual({ view: "home", layouts: { home }, named: [{ name: "Altra", layout: other }, { name: "Revisione", layout: moved }], openNamed: "Revisione", theme: "light" });
  });

  it("shows one of the three by closing the named view, saves nothing for showing, and settles into the three after", () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    const review = { marker: "review" } as never;
    layout.receive(fromTheCore({ view: "home", layouts: {}, named: [{ name: "Revisione", layout: review }], openNamed: "Revisione" }));
    layout.showView("compact");
    expect(layout.openNamed).toBeNull();
    expect(layout.view).toBe("compact");
    // ⛔ SHOWING IS NOT SAVING (decision 11).
    expect(bridge.sent).toEqual([]);
    const compact = { marker: "compact, moved" } as never;
    layout.settle(compact);
    expect(sentPack(bridge, 0)).toEqual({ view: "compact", layouts: { compact }, named: [{ name: "Revisione", layout: review }] });
  });

  it("saves the layout on screen under a new name at once and opens it, keeps those saved before, and refuses an empty or a taken name (D4)", () => {
    const bridge = createFakeBridge();
    const layout = useLayout();
    layout.attach(bridge);
    // ⛔ A NAMED VIEW SAVED BEFORE (E82 of the plan): the new name joins the list, and the one before stays.
    const before = { marker: "saved before" } as never;
    layout.receive(fromTheCore({ view: "home", layouts: {}, named: [{ name: "Prima", layout: before }] }));
    // The names the frame shows for the three views: the words are the locale's, and the store reads none.
    const shown = ["Home", "Lavoro", "Compatta"];
    const now = { marker: "on screen" } as never;
    expect(layout.saveNamed("   ", now, shown)).toBe("empty");
    expect(layout.saveNamed("home", now, shown)).toBe("taken");
    expect(bridge.sent).toEqual([]);
    expect(layout.saveNamed(" Revisione ", now, shown)).toBe("saved");
    expect(layout.openNamed).toBe("Revisione");
    expect(sentPack(bridge, 0)).toEqual({ view: "home", layouts: {}, named: [{ name: "Prima", layout: before }, { name: "Revisione", layout: now }], openNamed: "Revisione" });
    // ⛔ NOT OVERWRITTEN: the same name again, in another case, is refused and nothing more is sent.
    expect(layout.saveNamed("REVISIONE", { marker: "another" } as never, shown)).toBe("taken");
    expect(bridge.sent).toHaveLength(1);
  });

  it("closes the named view when the core answers without it -- an old package, or none it can read -- and the next move goes into the three (E75)", () => {
    // ⛔ A WRITE THAT DID NOT STICK (decision 13): the core answers with what it holds, the same before and after the
    // save -- a package without the name, the case the rule is for (E83 of the plan), or nothing, where E75 was found.
    const answers: IpcMessage[] = [fromTheCore({ view: "home", layouts: {} }), { kind: "Layout", value: { state: "Nothing" } }];
    for (const held of answers) {
      setActivePinia(createPinia());
      const bridge = createFakeBridge();
      const layout = useLayout();
      layout.attach(bridge);
      layout.receive(held);
      expect(layout.saveNamed("Revisione", { marker: "on screen" } as never, ["Home", "Lavoro", "Compatta"])).toBe("saved");
      // The dock shows the view of always, and a name left open would send the next move nowhere: not into
      // `layouts`, and not into `named`, which no longer holds it.
      layout.receive(held);
      expect(layout.openNamed).toBeNull();
      const moved = { marker: "home, moved" } as never;
      layout.settle(moved);
      expect(sentPack(bridge, 1)).toEqual({ view: "home", layouts: { home: moved } });
    }
  });
});

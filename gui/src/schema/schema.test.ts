import { describe, expect, it } from "vitest";

import { loadFixtures } from "./fixtures";
import { MESSAGE_KINDS, SchemaError, parseIpcMessage } from "./parse";

describe("the committed fixtures and the TypeScript types", () => {
  it("parses every fixture the kernel generated", () => {
    // ⛔ `loadFixtures` PARSES, so this call IS the assertion: a field renamed on the Rust side,
    // a variant added, a `u64` written as a number -- each makes `SchemaError` come out of here
    // with the path of the field that disagreed, once the fixtures are regenerated (until then
    // `the_committed_fixtures_match_the_schema` is the red, on the Rust side).
    // ⚠️ THE RENAME REACHES THIS FILE ONLY BECAUSE THE GENERATOR'S KEYS ARE THE FIELDS' OWN
    // IDENTIFIERS -- `json_struct!` and `json_members!` in `crates/kernel/tests/ipc_wire.rs`
    // (AUD-726 of the audit of 2026-09-30). A key written there as a literal kept the old name,
    // and this probe stayed green on a translation table. A VARIANT renamed is not caught: its
    // word is still a literal, declared beside `variant_json`.
    const fixtures = loadFixtures();
    expect(fixtures.length).toBeGreaterThan(0);
  });

  it("has a fixture for every kind the union declares, and no fixture for any other", () => {
    // ⛔ ONE DIRECTION IS THIS ASSERTION, AND THE OTHER IS NOT (AUD-2207 of the audit of 2026-09-30), and the assertion is
    // why `MESSAGE_KINDS` exists at run time: a variant the kernel added and nobody generated is MISSING here. A fixture of
    // a kind the mirror does not know never reaches it -- `loadFixtures` parses, and `parseIpcMessage` throws on that kind
    // first: red through the throw, not through an EXTRA. And a file left behind by a variant the kernel REMOVED parses
    // here while the mirror still knows the kind: that red is the kernel's, the "left over" of
    // `the_committed_fixtures_match_the_schema` in `crates/kernel/tests/ipc_wire.rs`.
    const fromFixtures = new Set(loadFixtures().map((fixture) => fixture.message.kind));
    expect([...fromFixtures].sort()).toEqual([...MESSAGE_KINDS].sort());
  });

  it("carries a u64 past Number.MAX_SAFE_INTEGER, which is why D35 sends a string", () => {
    // ⛔ THE CANONICAL SET'S `Hello`, AND NOT THE BUILD STAMP (AUD-2209 of the audit of 2026-09-30): its value is the
    // arbitrary one the kernel's `stamp_set` chose, and `stamp.ts` says why it is not the stamp -- the stamp the SPA sends
    // is held to the map by `stores.test.ts`. What holds here is D35's premise, on a u64 the kernel wrote.
    const hello = loadFixtures().find((fixture) => fixture.message.kind === "Hello");
    expect(hello).toBeDefined();
    // ⛔ THE ORACLE IS THAT THIS U64 DOES NOT SURVIVE A JSON NUMBER INTACT, which is the
    // PREMISE of D35 -- not that it is a string, which `u64` in `parse.ts` refuses and `G1`
    // already covers. ⚠️ `Number.MAX_SAFE_INTEGER` AND NOT `Number.MAX_VALUE`: a JSON number
    // holds far larger magnitudes, it just stops holding them EXACTLY, and rounding is the
    // failure D35 exists to prevent.
    // Measured on 2026-09-20: the round trip through `BigInt` that stood here is IMPLIED by
    // the `^[0-9]+$` of `u64` for every stamp a `u64` can print, so it stayed green on a
    // stamp of "42" -- green under the very case this name promises to catch.
    const value = (hello?.message as { kind: "Hello"; value: string }).value;
    expect(BigInt(value) > BigInt(Number.MAX_SAFE_INTEGER)).toBe(true);
  });

  it("refuses a message whose kind it does not know, a name every object inherits among them (AUD-2208 of the audit of 2026-09-30)", () => {
    // ⛔ `hasOwnProperty` AND NOT `in`: a guard that asked `kind in PARSERS`, or `PARSERS[kind] !== undefined`, lets the names
    // of `Object.prototype` through -- `constructor` would call `Object` as a parser and hand back the raw object, typed
    // `IpcMessage`. `Whatever` alone is refused by every form of the guard, and tells none of them apart.
    for (const kind of ["Whatever", "constructor", "toString", "__proto__", "hasOwnProperty", "valueOf"]) {
      expect(() => parseIpcMessage({ kind }), kind).toThrow(SchemaError);
    }
  });

  it("refuses a u64 written as a number", () => {
    expect(() => parseIpcMessage({ kind: "Hello", value: 4096 })).toThrow(SchemaError);
  });

  it("refuses a u64 that is a string but not of decimal digits, and takes one past Number.MAX_SAFE_INTEGER (AUD-2206 of the audit of 2026-09-30)", () => {
    // ⛔ THE DIGITS, NOT ONLY THE TYPE: the number above is refused before the digits are read, and the map writes the stamp
    // `0x…` -- one of the forms they keep out.
    for (const value of ["0x1f", "-1", "1.5", "", " 42", "42 "]) {
      expect(() => parseIpcMessage({ kind: "Hello", value }), JSON.stringify(value)).toThrow(SchemaError);
    }
    expect(parseIpcMessage({ kind: "Hello", value: "18446744073709551615" })).toEqual({ kind: "Hello", value: "18446744073709551615" });
  });

  it("refuses a byte that is not an integer from 0 to 255, and bytes that are not a list (AUD-2205 and AUD-2206 of the audit of 2026-09-30)", () => {
    for (const value of [[256], [-1], [1.5], ["7"], "[1,2]", null]) {
      expect(() => parseIpcMessage({ kind: "SaveLayout", value }), JSON.stringify(value)).toThrow(SchemaError);
    }
    expect(parseIpcMessage({ kind: "SaveLayout", value: [0, 255] })).toEqual({ kind: "SaveLayout", value: [0, 255] });
  });

  it("refuses a value outside the closed list of every nested enum (AUD-2205 and AUD-2206 of the audit of 2026-09-30)", () => {
    // ⛔ THE SECOND DIRECTION IS THE FIRST PROBE: the canonical set carries every value of every list, and parses.
    const refused: unknown[] = [
      { kind: "Accepted", value: "AsRoot" },
      { kind: "Policy", value: { policy: "Hybrid", allocated: "1", total: "2" } },
      { kind: "PermissionRequired", value: { tool: "arbiter", resource: "policy", operation: "Execute" } },
      { kind: "Token", text: "ciao", provenance: "Maybe" },
      { kind: "Layout", value: { state: "Lost" } },
      { kind: "Request", value: { reserved_vram: "1", compute_class: "Idle", preemption: { kind: "Never" } } },
      { kind: "Request", value: { reserved_vram: "1", compute_class: "Batch", preemption: { kind: "Sometimes" } } },
      { kind: "Verdict", value: { verdict: "Maybe" } },
    ];
    for (const message of refused) expect(() => parseIpcMessage(message), JSON.stringify(message)).toThrow(SchemaError);
  });

  it("refuses something else where the schema wants an object, a flag or a list, naming the field (AUD-2205 of the audit of 2026-09-30)", () => {
    // ⛔ THE FIELD THAT DISAGREED IS IN THE ERROR, and it says which guard refused: an array where an object goes would be
    // refused a field later all the same, by the flag that is missing -- the path tells the two apart.
    const refused: [unknown, string][] = [
      ["a message that is a string", 'message: unexpected "a message that is a string"'],
      [{ kind: "Degradation", value: [] }, "Degradation.value: unexpected []"],
      [{ kind: "Degradation", value: null }, "Degradation.value: unexpected null"],
      [{ kind: "Degradation", value: { vram_exhausted: "true", routing_degraded: false } }, 'Degradation.value.vram_exhausted: unexpected "true"'],
      [{ kind: "Steps", value: { step: "1", function: "f", done: true } }, "Steps.value: unexpected"],
      [{ kind: "Steps", value: [{ step: "1", function: "f", done: 1 }] }, "Steps.value[0].done: unexpected 1"],
    ];
    for (const [message, where] of refused) {
      expect(() => parseIpcMessage(message), where).toThrow(SchemaError);
      expect(() => parseIpcMessage(message), where).toThrow(where);
    }
  });
});

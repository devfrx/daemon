import { describe, expect, it } from "vitest";

import { loadFixtures } from "./fixtures";
import { MESSAGE_KINDS, SchemaError, parseIpcMessage } from "./parse";

describe("the committed fixtures and the TypeScript types", () => {
  it("parses every fixture the kernel generated", () => {
    // ⛔ `loadFixtures` PARSES, so this call IS the assertion: a field renamed on the Rust side,
    // a variant added, a `u64` written as a number -- each makes `SchemaError` come out of here
    // with the path of the field that disagreed.
    const fixtures = loadFixtures();
    expect(fixtures.length).toBeGreaterThan(0);
  });

  it("has a fixture for every kind the union declares, and no fixture for any other", () => {
    // ⛔ BOTH DIRECTIONS IN ONE ASSERTION, and it is why `MESSAGE_KINDS` exists at run time.
    // Left to right: a variant the kernel added and nobody generated is MISSING here. Right to
    // left: a variant the kernel REMOVED leaves its file behind and shows up as EXTRA.
    const fromFixtures = new Set(loadFixtures().map((fixture) => fixture.message.kind));
    expect([...fromFixtures].sort()).toEqual([...MESSAGE_KINDS].sort());
  });

  it("carries a build stamp past Number.MAX_SAFE_INTEGER, which is why D35 sends a string", () => {
    const hello = loadFixtures().find((fixture) => fixture.message.kind === "Hello");
    expect(hello).toBeDefined();
    // ⛔ THE ORACLE IS THAT THE STAMP DOES NOT SURVIVE A JSON NUMBER INTACT, which is the
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

  it("refuses a message whose kind it does not know", () => {
    expect(() => parseIpcMessage({ kind: "Whatever" })).toThrow(SchemaError);
  });

  it("refuses a u64 written as a number", () => {
    expect(() => parseIpcMessage({ kind: "Hello", value: 4096 })).toThrow(SchemaError);
  });
});

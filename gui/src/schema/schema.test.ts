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

  it("carries the build stamp as a string, because it does not fit a JSON number", () => {
    const hello = loadFixtures().find((fixture) => fixture.message.kind === "Hello");
    expect(hello).toBeDefined();
    // ⛔ THE ORACLE IS THE ROUND TRIP THROUGH `BigInt`, not `typeof`. A stamp read as a number
    // would already have been rounded by `JSON.parse` before anything here could look at it,
    // and the rounded value compares equal to itself -- green and false (D35).
    const value = (hello?.message as { kind: "Hello"; value: string }).value;
    expect(BigInt(value).toString()).toBe(value);
  });

  it("refuses a message whose kind it does not know", () => {
    expect(() => parseIpcMessage({ kind: "Whatever" })).toThrow(SchemaError);
  });

  it("refuses a u64 written as a number", () => {
    expect(() => parseIpcMessage({ kind: "Hello", value: 4096 })).toThrow(SchemaError);
  });
});

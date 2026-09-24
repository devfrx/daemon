import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

const HERE = dirname(fileURLToPath(import.meta.url));
const BOARD = join(HERE, "..", "..", "..", "docs", "superpowers", "specs", "2026-09-22-design-system-tavole", "token.html");

/** The three markers of the approved board, verbatim: the plan's script cut the two files between them. */
const BASE = "/* ===== proposta/base.css ===== */";
const THEMES = "/* ===== proposta/themes.css ===== */";
const END = "/* ===== the board itself, only tokens ===== */";

/** LF, no blank line at the start, no whitespace at the end, one LF after: the terminator belongs to the
 * checkout, not to the tokens -- the files are CRLF on one machine and LF on another. */
function normal(text: string): string {
  return `${text.replace(/\r\n/g, "\n").replace(/^\n+/, "").replace(/\s+$/, "")}\n`;
}

function between(text: string, start: string, end: string): string {
  const from = text.indexOf(start);
  const to = text.indexOf(end);
  if (from < 0 || to < from) throw new Error(`the board lost a marker: ${start} … ${end}`);
  return text.slice(from + start.length, to);
}

/**
 * ⛔ TWO HOUSES FOR THE SAME VALUES, AND THIS PROBE IS WHAT KEEPS THEM AGREEING (D1 of the design-system plan):
 * the values live in the approved board -- decision 23 of the design -- and the SPA ships a copy. Whoever
 * changes a colour changes the board FIRST, then copies; a copy edited by hand turns this red.
 */
describe("the token files", () => {
  const board = readFileSync(BOARD, "utf8");

  it.each([
    ["base.css", BASE, THEMES],
    ["themes.css", THEMES, END],
  ])("%s is the board's block, byte for byte", (file, start, end) => {
    const block = normal(between(board, start, end));
    // ⛔ NON-VACUITY: two empty cuts would agree with two empty files.
    expect(block.length).toBeGreaterThan(1000);
    expect(normal(readFileSync(join(HERE, file), "utf8"))).toBe(block);
  });
});

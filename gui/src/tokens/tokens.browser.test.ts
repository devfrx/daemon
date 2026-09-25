import { commands, userEvent } from "vitest/browser";
import { afterEach, describe, expect, it } from "vitest";

import "./index";
import { watchTheme, type ThemeChoice } from "./theme";

// ⛔ THIS FILE RUNS IN THE INSTALLED CHROME (design system, section (f)): the tokens and the fonts are the
// SPA's own, imported above, and every probe asks the layout engine rather than a copy of the values.

/** Every emulated feature goes back to the browser, so a probe never inherits the previous one's. */
afterEach(async () => {
  await commands.emulateMedia({ colorScheme: null, reducedMotion: null, forcedColors: null });
  document.body.replaceChildren();
  delete document.documentElement.dataset.theme;
});

const rootStyle = (): CSSStyleDeclaration => getComputedStyle(document.documentElement);

/** `navigator.userAgentData` is not in TypeScript's `DOM` library: the shape the probe reads, written here. */
type BrandedNavigator = Navigator & { userAgentData?: { brands: readonly { brand: string }[] } };

describe("the browser the probes run in", () => {
  it("is a real one: it lays out, and it is Chrome", () => {
    // ⛔ THE NON-VACUITY OF THE WHOLE PROJECT (control 20 of the design): under jsdom every rectangle is zero,
    // so a box with a size proves a layout engine; and the channel is the installed Chrome (decision 22).
    const box = document.createElement("div");
    box.style.cssText = "width:120px;height:40px";
    document.body.append(box);
    expect(box.getBoundingClientRect().width).toBe(120);
    // ⛔ THE BRAND, NOT THE USER AGENT: `HeadlessChrome/` ends in `Chrome/`, and so would the Chromium Playwright
    // downloads; the installed Chrome is the one whose brands say "Google Chrome" (R2-6 of the design-system review).
    const brands = (navigator as BrandedNavigator).userAgentData?.brands.map((entry) => entry.brand) ?? [];
    expect(brands).toContain("Google Chrome");
  });
});

describe("the tokens, in a real browser (design system, sections (a) and (f))", () => {
  it("load both fonts, and draw the tool font's digits at one width", async () => {
    // ⛔ LOAD BEFORE MEASURING (trap 2): a face loads only when some text needs it. And the oracle is the
    // FontFace's status, not `document.fonts.check()`, which says yes for a family no @font-face declares.
    await document.fonts.load('400 14px "Geist Variable"');
    for (const weight of [300, 400, 500, 600]) await document.fonts.load(`${weight} 32px "Barlow"`);
    const loaded = (family: string): number =>
      [...document.fonts].filter((face) => face.family.replace(/"/g, "") === family && face.status === "loaded").length;
    expect(loaded("Geist Variable")).toBeGreaterThan(0);
    expect(loaded("Barlow")).toBeGreaterThanOrEqual(4);

    const width = (text: string, variant: string, family?: string): number => {
      const span = document.createElement("span");
      span.textContent = text;
      span.style.cssText = `position:absolute;white-space:nowrap;font:var(--font-display);font-variant-numeric:${variant}`;
      if (family !== undefined) span.style.fontFamily = family;
      document.body.append(span);
      return span.getBoundingClientRect().width;
    };
    expect(Math.abs(width("111111", "tabular-nums") - width("000000", "tabular-nums"))).toBeLessThan(0.5);
    // ⛔ THE SECOND DIRECTION: without `tabular-nums` the digits are proportional -- measured on the token board on
    // 2026-09-23, 67.34 against 108.10 px. It proves that `tabular-nums` acts, NOT which font draws: Bahnschrift, the
    // chain's first fallback on Windows, has proportional digits too (R2-4 of the design-system review).
    expect(Math.abs(width("111111", "normal") - width("000000", "normal"))).toBeGreaterThan(10);
    // ⛔ WHICH FONT DRAWS: the same text in the token's chain WITHOUT Barlow measures otherwise -- 67.33 against 63.84 px
    // on the plain ones, 100.42 against 102.19 on the tabular zeros, in the installed Chrome on 2026-09-23. A token
    // that drew with the fallback would measure the same in both.
    const chain = rootStyle().getPropertyValue("--font-family-tool").trim();
    const fallback = chain.replace(/^"Barlow",\s*/, "");
    expect(fallback, "the chain starts with Barlow").not.toBe(chain);
    expect(Math.abs(width("111111", "normal") - width("111111", "normal", fallback))).toBeGreaterThan(0.5);
    expect(Math.abs(width("000000", "tabular-nums") - width("000000", "tabular-nums", fallback))).toBeGreaterThan(0.5);
  });

  it("put the motion to zero when the system asks for less, and only then (WCAG 2.3.3)", async () => {
    await commands.emulateMedia({ reducedMotion: "reduce" });
    for (const name of ["--duration-fast", "--duration-moderate", "--duration-slow"]) {
      expect(rootStyle().getPropertyValue(name).trim(), name).toBe("0ms");
    }
    // ⛔ THE SECOND DIRECTION: without the request, every duration is a duration and not zero -- a VALUE, so a
    // name the base block lost is red here too, and not only in `board.test.ts` (E14 of the design-system plan).
    await commands.emulateMedia({ reducedMotion: "no-preference" });
    for (const name of ["--duration-fast", "--duration-moderate", "--duration-slow"]) {
      expect(rootStyle().getPropertyValue(name).trim(), name).toMatch(/^[1-9]\d*ms$/);
    }
  });

  it("keep the focus ring under Windows' high contrast: an outline, which forced colours keep (G20, trap 11)", async () => {
    document.documentElement.dataset.theme = "dark";
    await commands.emulateMedia({ forcedColors: "active" });
    // ⛔ THE EMULATION IS IN FORCE, or nothing below is about high contrast (R2-2 of the design-system review).
    expect(matchMedia("(forced-colors: active)").matches).toBe(true);
    const button = document.createElement("button");
    button.textContent = "focus";
    document.body.append(button);
    // A KEY and not `focus()`: `:focus-visible` is the keyboard's ring, and the probe must reach it the same way.
    await userEvent.tab();
    expect(document.activeElement).toBe(button);
    const ring = (element: HTMLElement): boolean => {
      const style = getComputedStyle(element);
      return style.outlineStyle !== "none" && Number.parseFloat(style.outlineWidth) >= 2;
    };
    expect(ring(button)).toBe(true);
    // ⛔ THE SECOND DIRECTION IS WHAT HIGH CONTRAST DOES: forced colours erase a ring drawn with `box-shadow` -- the
    // reason the (a) draws it with an outline. The same button, ringed that way, has no ring left.
    button.style.outline = "none";
    button.style.boxShadow = "0 0 0 2px currentColor";
    expect(getComputedStyle(button).boxShadow).toBe("none");
  });

  it("follow the system's scheme through the real query while the choice is `system`", async () => {
    // ⛔ A KNOWN START (P-21 of the design-system plan): `null` gives back the machine's own scheme -- dark on a
    // Windows set to dark, where a first flip to dark changed nothing and passed with `watchTheme` deaf to the
    // system. Light first, and BOTH flips need the event.
    await commands.emulateMedia({ colorScheme: "light" });
    const root = document.createElement("div");
    const stop = watchTheme((): ThemeChoice => "system", root);
    expect(root.dataset.theme).toBe("light");
    await commands.emulateMedia({ colorScheme: "dark" });
    await expect.poll(() => root.dataset.theme).toBe("dark");
    await commands.emulateMedia({ colorScheme: "light" });
    await expect.poll(() => root.dataset.theme).toBe("light");
    stop();
  });
});

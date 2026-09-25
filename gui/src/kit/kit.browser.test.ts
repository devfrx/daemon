import { mount } from "@vue/test-utils";
import axe from "axe-core";
import { userEvent } from "vitest/browser";
import { afterEach, describe, expect, it } from "vitest";
import { nextTick } from "vue";

import "../tokens";
import { violations } from "../testing/axe";
import { concentricRadii, firstFamily, fits, iconsCentred } from "../testing/probes";

import Kit from "./Kit.vue";

// ⛔ THE KIT PAGE IN THE INSTALLED CHROME (design system, section (f)): the probes of the boards, on the real pieces.

const ROOTS = ".kit-card, .kit-frame, .kit-strip";
const BOXES = ".kit-card, .kit-frame, .kit-strip, .base-button, .base-list-row, .base-text-field > .frame, .option, .base-dialog";

/** The kits the probes mounted: every one unmounted after its probe, red or green. */
const kits: { unmount(): void }[] = [];

afterEach(() => {
  // ⛔ UNMOUNTED HERE, NOT ON A PROBE'S LAST LINE (E25 of the plan): a probe that goes red stops before it, and the app
  // it leaves alive keeps an open window's `pointer-events: none` on the body -- the next probe that moves the real
  // pointer then times out, measured on 2026-09-25. The shape of the dock's and the frame's probes (tasks 6 and 8).
  for (const wrapper of kits.splice(0)) wrapper.unmount();
  document.body.replaceChildren();
});

async function kit(theme: "light" | "dark") {
  const wrapper = mount(Kit, { attachTo: document.body, props: { initialTheme: theme } });
  kits.push(wrapper);
  await nextTick();
  await document.fonts.ready;
  return wrapper;
}

const roots = (selector: string): Element[] => [...document.querySelectorAll(selector)];

/**
 * ⛔ THE NON-VACUITY OF `axe` (R3-7 of the review): an empty list of violations says something only if the contrast was
 * JUDGED -- nodes among the passes, none left incomplete. Under jsdom axe files every contrast as incomplete; here, in
 * the browser, it must not.
 */
async function contrastJudged(node: Element): Promise<{ passes: number; incomplete: number }> {
  const results = await axe.run(node, { runOnly: ["color-contrast"] });
  const count = (list: axe.Result[]): number => list.find((rule) => rule.id === "color-contrast")?.nodes.length ?? 0;
  return { passes: count(results.passes), incomplete: count(results.incomplete) };
}

/** A colour token as the page computes it -- `rgb(…)`, in the theme on the root -- never copied from the board. */
function colourOf(token: string): string {
  const probe = document.createElement("span");
  probe.style.color = `var(${token})`;
  document.body.append(probe);
  const colour = getComputedStyle(probe).color;
  probe.remove();
  return colour;
}

for (const theme of ["light", "dark"] as const) {
  describe(`the kit page, ${theme} theme`, () => {
    it("puts its theme on the root", async () => {
      await kit(theme);
      expect(document.documentElement.dataset.theme).toBe(theme);
    });

    it("keeps every radius concentric (answer 4)", async () => {
      await kit(theme);
      const report = concentricRadii(roots(ROOTS));
      // ⛔ NON-VACUITY (trap 1): a probe that met no corner near another is green for nothing.
      expect(report.near).toBeGreaterThan(0);
      expect(report.bad).toEqual([]);
    });

    it("cuts no text, and lets nothing stick out of its box", async () => {
      await kit(theme);
      const report = fits(roots(ROOTS), BOXES);
      expect(report.seen).toBeGreaterThan(0);
      expect(report.boxed).toBeGreaterThan(0);
      expect(report.problems).toEqual([]);
    });

    it("draws every icon in currentColor, and centres it", async () => {
      await kit(theme);
      const report = iconsCentred(roots(".kit"));
      expect(report.icons).toBeGreaterThan(0);
      expect(report.centred).toBeGreaterThan(0);
      expect(report.problems).toEqual([]);
    });

    it("dresses labels and numbers in Barlow, and the text in Geist", async () => {
      await kit(theme);
      const label = document.querySelector(".base-label");
      const text = document.querySelector(".base-list-row span");
      const number = document.querySelector(".kit em");
      expect(label !== null && text !== null && number !== null).toBe(true);
      expect(firstFamily(label as Element)).toBe("Barlow");
      expect(firstFamily(text as Element)).toBe("Geist Variable");
      expect(firstFamily(number as Element)).toBe("Barlow");
    });

    it("has no axe violation -- contrast included, on the drawn page", async () => {
      const wrapper = await kit(theme);
      expect(await violations(wrapper.element, { contrast: true })).toEqual([]);
      const judged = await contrastJudged(wrapper.element);
      expect(judged.passes).toBeGreaterThan(0);
      expect(judged.incomplete).toBe(0);
    });

    it("opens its window with the radii concentric, and no axe violation", async () => {
      await kit(theme);
      document.querySelector<HTMLButtonElement>('[data-kit="open-dialog"]')?.click();
      await nextTick();
      await nextTick();
      const dialog = roots(".base-dialog");
      expect(dialog).toHaveLength(1);
      const report = concentricRadii(dialog);
      expect(report.near).toBeGreaterThan(0);
      expect(report.bad).toEqual([]);
      expect(await violations(dialog[0] as Element, { contrast: true })).toEqual([]);
      const judged = await contrastJudged(dialog[0] as Element);
      expect(judged.passes).toBeGreaterThan(0);
      expect(judged.incomplete).toBe(0);
    });

    it("draws every button that is off in the disabled colour, whatever its variant (E19)", async () => {
      await kit(theme);
      const off = [...document.querySelectorAll<HTMLElement>(".kit .base-button:disabled")];
      // ⛔ NON-VACUITY (trap 1): one button off per variant, and the probe meets all three. jsdom applies no sheet, so
      // only here can a rule that weighs more than `:disabled` show -- the quiet one did (E19 of the plan).
      expect(off.map((button) => button.dataset.variant).sort()).toEqual(["primary", "quiet", "secondary"]);
      const disabled = colourOf("--color-text-disabled");
      expect(off.map((button) => `${button.dataset.variant}: ${getComputedStyle(button).color}`)).toEqual(
        off.map((button) => `${button.dataset.variant}: ${disabled}`),
      );
    });

    it("keeps the error's border under the pointer (E20)", async () => {
      await kit(theme);
      const frame = document.querySelector<HTMLElement>(".kit .base-text-field > .frame[data-error]");
      expect(frame).not.toBeNull();
      const stop = colourOf("--color-border-stop");
      expect(getComputedStyle(frame as HTMLElement).borderTopColor).toBe(stop);
      await userEvent.hover(frame as HTMLElement);
      // ⛔ NON-VACUITY: a pointer that never arrived would leave the border as it was, and the probe green for nothing.
      expect((frame as HTMLElement).matches(":hover")).toBe(true);
      expect(getComputedStyle(frame as HTMLElement).borderTopColor).toBe(stop);
      await userEvent.unhover(frame as HTMLElement);
    });
  });
}

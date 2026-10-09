import { mount } from "@vue/test-utils";
import { userEvent } from "vitest/browser";
import { afterEach, describe, expect, it } from "vitest";
import { nextTick } from "vue";

import "../tokens";
import it_ from "../locales/it.json";
import { contrastJudged, violations } from "../testing/axe";
import { computed, concentricRadii, firstFamily, fits, iconsCentred } from "../testing/probes";

import Kit from "./Kit.vue";
import kitSource from "./Kit.vue?raw";

// ⛔ THE KIT PAGE IN THE INSTALLED CHROME (design system, section (f)): the probes of the boards, on the real pieces.

const ROOTS = ".kit-card, .kit-frame, .kit-strip, .base-notice";
const BOXES =
  ".kit-card, .kit-frame, .kit-strip, .base-button, .base-list-row, .base-text-field > .frame, .option, .base-dialog, .base-notice";

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

/** A colour token as the page computes it -- `rgb(…)`, in the theme on the root -- never copied from the board. */
function colourOf(token: string): string {
  return computed("color", token);
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
      // ⛔ AND THE ONE CASE THAT JUDGES `BaseList` (E29 of the plan): its last row closes its card, in the corner. `near`
      // counts the whole page, so a stretched grid, or a note after the list, would take this case away and stay above 0.
      const list = document.querySelector(".kit-card:has(> .base-list)");
      expect(list).not.toBeNull();
      expect(concentricRadii([list as Element]).near).toBeGreaterThan(0);
      // ⛔ AND THE ONE CASE THAT JUDGES A MESSAGE'S ACTION (E60, E61): «Riprova», on the page, in the two right corners of
      // the message that holds it.
      const onPage = document.querySelector(".base-notice[data-on-page]");
      expect(onPage).not.toBeNull();
      expect(concentricRadii([onPage as Element]).near).toBeGreaterThan(0);
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

    it("opens its window with the radii concentric, nothing cut or sticking out, and no axe violation", async () => {
      await kit(theme);
      document.querySelector<HTMLButtonElement>('[data-kit="open-dialog"]')?.click();
      await nextTick();
      await nextTick();
      const dialog = roots(".base-dialog");
      expect(dialog).toHaveLength(1);
      const report = concentricRadii(dialog);
      expect(report.near).toBeGreaterThan(0);
      expect(report.bad).toEqual([]);
      // ⛔ THE OPEN WINDOW IS JUDGED FOR CUT TEXT TOO (AUD-2198 of the audit of 2026-09-30): it lives in a portal on `body`,
      // out of the roots the probe of the page reads, so its `.base-dialog` among the boxes met nothing there.
      const fit = fits(dialog, BOXES);
      expect(fit.seen).toBeGreaterThan(0);
      expect(fit.boxed).toBeGreaterThan(0);
      expect(fit.problems).toEqual([]);
      expect(await violations(dialog[0] as Element, { contrast: true })).toEqual([]);
      const judged = await contrastJudged(dialog[0] as Element);
      expect(judged.passes).toBeGreaterThan(0);
      expect(judged.incomplete).toBe(0);
    });

    for (const [variant, trigger] of [
      ["sheet", "open-sheet"],
      ["full", "open-full"],
    ] as const) {
      it(`opens its window as the ${variant}, nothing cut or sticking out, and no axe violation (AUD-1085 of the audit of 2026-09-30)`, async () => {
        await kit(theme);
        document.querySelector<HTMLButtonElement>(`[data-kit="${trigger}"]`)?.click();
        await nextTick();
        await nextTick();
        const dialog = roots(`.base-dialog[data-variant="${variant}"]`);
        expect(dialog).toHaveLength(1);
        const fit = fits(dialog, BOXES);
        expect(fit.seen).toBeGreaterThan(0);
        expect(fit.boxed).toBeGreaterThan(0);
        expect(fit.problems).toEqual([]);
        // ⛔ NO RADIUS JUDGED HERE, AND IT IS NOT AN OVERSIGHT: the rule judges a rounded piece in a rounded box, and the
        // sheet's two rounded corners hold nothing, while the whole page's corners are square -- Windows' (the (d)).
        expect(await violations(dialog[0] as Element, { contrast: true })).toEqual([]);
        const judged = await contrastJudged(dialog[0] as Element);
        expect(judged.passes).toBeGreaterThan(0);
        expect(judged.incomplete).toBe(0);
      });
    }

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

    it("dresses each tone of a message in its roles (control 23, E60)", async () => {
      await kit(theme);
      // The neutral tone is the accent's (N2 of E60); the three others are the states' own.
      const ROLE = { info: "accent", ok: "ok", warn: "warn", stop: "stop" } as const;
      const notices = [...document.querySelectorAll<HTMLElement>(".kit-card .base-notice")];
      // ⛔ NON-VACUITY: the card of the messages shows the four tones.
      expect(notices.map((notice) => notice.dataset.tone).sort()).toEqual(["info", "ok", "stop", "warn"]);
      for (const notice of notices) {
        const role = ROLE[notice.dataset.tone as keyof typeof ROLE];
        const icon = notice.querySelector("svg.base-icon");
        expect(icon).not.toBeNull();
        const style = getComputedStyle(notice);
        expect([style.backgroundColor, style.borderTopColor, getComputedStyle(icon as Element).color]).toEqual([
          colourOf(`--color-bg-${role}-subtle`),
          colourOf(`--color-border-${role}`),
          colourOf(`--color-text-${role}`),
        ]);
        // ⛔ AND THE WORDS IN THE TEXT'S OWN ROLES, WHATEVER THE TONE (the board's `.msg b` and `.msg span`; E65 of the
        // plan): without them the title takes the tone's colour from the root, which the icon draws with.
        expect(getComputedStyle(notice.querySelector(".title") as Element).color).toBe(colourOf("--color-text"));
        for (const description of notice.querySelectorAll(".description")) {
          expect(getComputedStyle(description).color).toBe(colourOf("--color-text-muted"));
        }
      }
      // ⛔ NON-VACUITY: a description was judged -- the refusal's, in the card of the messages.
      expect(document.querySelectorAll(".kit-card .base-notice .description").length).toBeGreaterThan(0);
    });

    it("sets a message's title and icon on the line of its action (the owner, 2026-09-27: the 2 px of E60)", async () => {
      await kit(theme);
      const notice = document.querySelector<HTMLElement>(".base-notice[data-action]");
      expect(notice).not.toBeNull();
      const centre = (selector: string): number => {
        const found = (notice as HTMLElement).querySelector(selector);
        expect(found).not.toBeNull();
        const box = (found as Element).getBoundingClientRect();
        return (box.top + box.bottom) / 2;
      };
      const action = centre("button");
      // ⛔ AT THE TOP, AS THE BOARD'S `.msg` WAS, THE TITLE SAT 2 PX ABOVE: the action is 24 high, the title's line 20.
      expect(Math.abs(centre(".title") - action)).toBeLessThanOrEqual(0.5);
      expect(Math.abs(centre("svg.base-icon") - action)).toBeLessThanOrEqual(0.5);
    });
  });
}

describe("the kit page's words", () => {
  it("shows the permission window with the phrase of `confirm.scope`, and keeps no copy of it (gotcha #68)", async () => {
    // ⛔ THE ONE KIT WORD THAT IS NOT A SPECIMEN (D8 of the design-system plan): the scope of a grant is a FACT of the
    // system -- who builds the session boundary -- and it changes with sub-project 3. The window that ships says it
    // from `it.json`, and so does this page. Audit of 2026-09-30.
    await kit("light");
    document.querySelector<HTMLButtonElement>('[data-kit="open-dialog"]')?.click();
    await nextTick();
    await nextTick();
    const description = document.querySelector(".base-dialog .description");
    expect(description).not.toBeNull();
    expect((description as Element).textContent?.trim()).toBe(it_.confirm.scope);
    // ⛔ AND THE PAGE DOES NOT SPELL IT: the equality above holds with a copy too, until the day the two drift apart.
    expect(kitSource.includes(it_.confirm.scope), "Kit.vue spells the phrase of `confirm.scope`").toBe(false);
  });
});

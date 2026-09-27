import "dockview/dist/styles/dockview.css";
import "../tokens";

import { createPinia, setActivePinia } from "pinia";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { createApp, type App as VueApp } from "vue";

import App from "../App.vue";
import { i18n } from "../i18n";
import { registerModules } from "../panels/modules";
import { concentricRadii } from "../testing/probes";
import { readToken } from "../tokens/readToken";

// ⛔ THE WHOLE FRAME IN THE INSTALLED CHROME (the (d) of the design system): the page of the SPA -- `App.vue`, whose rule
// gives `html`, `body` and `#app` the whole window -- at the probes' 1440 x 900, with the stylesheets in the order
// `main.ts` loads them. Born with task 6bis for the band on the page (E60); task 8 adds the strip and the overview.

const frames: VueApp[] = [];

beforeEach(() => {
  registerModules();
});

afterEach(() => {
  for (const frame of frames.splice(0)) frame.unmount();
  document.body.replaceChildren();
  delete document.documentElement.dataset.theme;
});

/**
 * The frame in one theme, once `dockview` has laid out the Home view. ⛔ MOUNTED AS `main.ts` MOUNTS IT, on `#app`
 * itself: `mount` of `@vue/test-utils` puts the app in a `div` of its own inside the element it is given, and that `div`
 * has no height -- the frame came out 116 px high, the dock 0, and the strip in the middle of the page, measured on
 * 2026-09-24 (P-23 of the plan).
 */
async function frame(theme: "light" | "dark"): Promise<void> {
  document.documentElement.dataset.theme = theme;
  const pinia = createPinia();
  setActivePinia(pinia);
  const host = document.createElement("div");
  host.id = "app";
  document.body.append(host);
  const app = createApp(App).use(pinia).use(i18n);
  app.mount(host);
  frames.push(app);
  await new Promise((resolve) => setTimeout(resolve, 50));
}

/** A length token, in px, as the page computes it. */
function px(token: string): number {
  return Number.parseFloat(readToken(token));
}

/** What a declaration of `property: var(token)` computes to here: the oracle of a token, with no copy of its value. */
function computed(property: string, token: string): string {
  const probe = document.createElement("div");
  probe.style.setProperty(property, `var(${token})`);
  document.body.append(probe);
  const value = getComputedStyle(probe).getPropertyValue(property);
  probe.remove();
  return value;
}

for (const theme of ["light", "dark"] as const) {
  describe(`the frame, ${theme} theme`, () => {
    it("lays the band on the page: under the bar, 12 px from the sides and from the cards, rounded like a card (control 23, E60)", async () => {
      await frame(theme);
      // With no core the connection waits (§6a): the band is there, a warning with «Riprova».
      const band = document.querySelector<HTMLElement>(".band");
      const bar = document.querySelector<HTMLElement>(".bar");
      expect(band).not.toBeNull();
      expect(bar).not.toBeNull();
      const box = (band as HTMLElement).getBoundingClientRect();
      // Aligned with the cards: the dock's own margin, `--space-3`, on each side.
      expect(box.left).toBeCloseTo(px("--space-3"), 1);
      expect(window.innerWidth - box.right).toBeCloseTo(px("--space-3"), 1);
      // Right under the bar, which leaves it its own 8 px: the band adds nothing above itself.
      expect(box.top).toBeCloseTo((bar as HTMLElement).getBoundingClientRect().bottom, 1);
      // `--space-3` above the highest card: the dock has none on top (answer 20).
      const cards = [...document.querySelectorAll(".dv-groupview")]
        .map((group) => group.getBoundingClientRect())
        .filter((card) => card.width > 0);
      expect(cards.length).toBeGreaterThan(0);
      expect(Math.min(...cards.map((card) => card.top)) - box.bottom).toBeCloseTo(px("--space-3"), 1);
      // Rounded like a card, and «Riprova» concentric in its corners at half a pixel (E61).
      expect(getComputedStyle(band as HTMLElement).borderTopLeftRadius).toBe(computed("border-top-left-radius", "--radius-card"));
      const radii = concentricRadii([band as HTMLElement]);
      expect(radii.near).toBeGreaterThan(0);
      expect(radii.bad).toEqual([]);
    });
  });
}

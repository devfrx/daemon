import "dockview/dist/styles/dockview.css";
import "../tokens";

import { createPinia, setActivePinia } from "pinia";
import { userEvent } from "vitest/browser";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { createApp, type App as VueApp } from "vue";

import App from "../App.vue";
import { i18n } from "../i18n";
import { registerModules } from "../panels/modules";
import { useConnection } from "../stores/connection";
import { useLayout } from "../stores/layout";
import { contrastJudged, violations } from "../testing/axe";
import { computed, concentricRadii, fits, iconsCentred } from "../testing/probes";
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

const overview = (): HTMLElement | null => document.querySelector('.base-dialog[data-variant="full"]');
const cards = (): HTMLElement[] => [...document.querySelectorAll<HTMLElement>("[data-card]")];

for (const theme of ["light", "dark"] as const) {
  describe(`the frame, ${theme} theme`, () => {
    it("lays both bands on the page -- under the bar, 12 px from the sides and from the cards, rounded like a card -- and the page never spills while one comes in (control 23, E60, E70, E43)", async () => {
      await frame(theme);
      const bar = document.querySelector<HTMLElement>(".bar");
      expect(bar).not.toBeNull();
      const onThePage = (band: HTMLElement): void => {
        const box = band.getBoundingClientRect();
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
        // Rounded like a card.
        expect(getComputedStyle(band).borderTopLeftRadius).toBe(computed("border-top-left-radius", "--radius-card"));
      };
      // With no core the connection waits (§6a): the band is there, a warning with «Riprova».
      const waiting = document.querySelector<HTMLElement>(".band");
      expect(waiting).not.toBeNull();
      onThePage(waiting as HTMLElement);
      // «Riprova» concentric in its corners at half a pixel (E61).
      const radii = concentricRadii([waiting as HTMLElement]);
      expect(radii.near).toBeGreaterThan(0);
      expect(radii.bad).toEqual([]);
      // ⛔ THE BAND THAT STOPS THE WINDOW IS ON THE PAGE TOO (E70), AND THE PAGE NEVER SPILLS WHILE IT COMES IN (E43): the
      // core welcomed, the band leaves; another stamp brings it back. `dockview` resizes one frame late, and for that
      // frame the grid spilled out of the dock and the page showed its scrollbars -- read on `scrollHeight`, which the
      // hidden scrollbars do not hide: 50 px on the first frame without the dock's `overflow: clip`, measured on
      // 2026-09-27.
      const connection = useConnection();
      connection.receive({ kind: "Accepted", value: "AsSystemAccount" });
      await expect.poll(() => document.querySelector(".band")).toBeNull();
      const frames = async (count: number): Promise<number[]> => {
        const spill: number[] = [];
        for (let step = 0; step < count; step += 1) {
          await new Promise((resolve) => requestAnimationFrame(resolve));
          const page = document.documentElement;
          spill.push(page.scrollHeight - page.clientHeight + page.scrollWidth - page.clientWidth);
        }
        return spill;
      };
      await frames(6);
      connection.receive({ kind: "StaleBuild", value: "81985529216486895" });
      expect(await frames(8)).toEqual([0, 0, 0, 0, 0, 0, 0, 0]);
      const stop = document.querySelector<HTMLElement>(".band");
      expect(stop?.getAttribute("data-tone")).toBe("stop");
      onThePage(stop as HTMLElement);
    });

    it("floats the strip as a pill 12 px from the sides and 24 from the bottom, and keeps every card off the page's corners (control 19)", async () => {
      await frame(theme);
      const strips = [...document.querySelectorAll(".dv-groupview")].filter((group) => group.querySelector(".strip") !== null);
      expect(strips).toHaveLength(1);
      const strip = strips[0] as Element;
      const box = strip.getBoundingClientRect();
      // Answer 20: aligned with the cards, `--space-3` from the sides, and `--space-6` from the bottom.
      expect(box.left).toBeCloseTo(px("--space-3"), 1);
      expect(window.innerWidth - box.right).toBeCloseTo(px("--space-3"), 1);
      expect(window.innerHeight - box.bottom).toBeCloseTo(px("--space-6"), 1);
      // A pill: the radius reaches half the height.
      expect(Number.parseFloat(getComputedStyle(strip).borderTopLeftRadius)).toBeGreaterThanOrEqual(box.height / 2);
      // ⛔ THE CORNERS ARE WINDOWS' (the (d)): no rounded box of ours comes within `--space-3` of a corner of the page on
      // both axes -- the square where the window's arc is drawn, 8 px in Windows 11.
      const reach = px("--space-3");
      let rounded = 0;
      const near: string[] = [];
      for (const element of document.body.querySelectorAll("*")) {
        if (element.closest("svg") !== null) continue;
        if (!(Number.parseFloat(getComputedStyle(element).borderTopLeftRadius) > 0)) continue;
        const b = element.getBoundingClientRect();
        if (b.width === 0 || b.height === 0) continue;
        rounded += 1;
        const gaps = [
          [b.left, b.top],
          [window.innerWidth - b.right, b.top],
          [b.left, window.innerHeight - b.bottom],
          [window.innerWidth - b.right, window.innerHeight - b.bottom],
        ];
        if (gaps.some(([dx, dy]) => (dx ?? 0) < reach && (dy ?? 0) < reach)) near.push(element.getAttribute("class") ?? element.tagName);
      }
      // ⛔ NON-VACUITY (trap 1 of the design): the cards, the strip and the controls are rounded.
      expect(rounded).toBeGreaterThan(0);
      expect(near).toEqual([]);
    });

    it("opens the drawer from the strip's pill, gives the focus back to it when the drawer closes, and from the keyboard opens it with the focus in sight", async () => {
      await frame(theme);
      const button = document.querySelector<HTMLElement>(".strip .base-button");
      expect(button).not.toBeNull();
      await userEvent.click(button as HTMLElement);
      await expect.poll(() => document.querySelector('.base-dialog[data-variant="sheet"]')).not.toBeNull();
      await userEvent.keyboard("{Escape}");
      await expect.poll(() => document.querySelector('.base-dialog[data-variant="sheet"]')).toBeNull();
      // ⛔ THE BUTTON IS IN ANOTHER VUE APP, AND NO `DialogTrigger`: the focus comes back all the same, measured on
      // 2026-09-24 with `reka-ui` 2.10.4 (P-22 of the plan) -- this probe is what keeps it true across an update.
      await expect.poll(() => document.activeElement).toBe(button);
      // ⛔ FROM THE KEYBOARD, THE FOCUS IN SIGHT (E38): `reka-ui` gives the first control the focus with `preventScroll`
      // -- "Chiudi", below a list longer than the sheet -- and the sheet keeps its actions stuck to its bottom edge.
      await userEvent.keyboard("{Enter}");
      await expect.poll(() => document.querySelector('.base-dialog[data-variant="sheet"]')).not.toBeNull();
      const sheet = document.querySelector('.base-dialog[data-variant="sheet"]') as HTMLElement;
      const focused = document.activeElement as HTMLElement;
      expect(sheet.contains(focused)).toBe(true);
      const seen = focused.getBoundingClientRect();
      expect(seen.top).toBeGreaterThanOrEqual(sheet.getBoundingClientRect().top);
      expect(seen.bottom).toBeLessThanOrEqual(Math.min(sheet.getBoundingClientRect().bottom, window.innerHeight));
    });

    it("draws the overview with its radii concentric, nothing cut or sticking out, the icons centred, and no axe violation", async () => {
      await frame(theme);
      // ⛔ THE CORE HAS ANSWERED (E81): before it, «Salva questa vista» is off.
      useLayout().receive({ kind: "Layout", value: { state: "Nothing" } });
      await userEvent.keyboard("{F3}");
      await expect.poll(overview).not.toBeNull();
      // And with the name of a new view asked, the field and its two buttons in the place of «Salva questa vista».
      for (const naming of [false, true]) {
        if (naming) {
          await userEvent.click(document.querySelector('[data-card="save"]') as HTMLElement);
          await expect.poll(() => document.querySelector(".naming")).not.toBeNull();
        }
        const dialog = overview() as HTMLElement;
        const radii = concentricRadii([dialog]);
        // ⛔ NON-VACUITY (trap 1): the miniatures sit in the cards, the tiles in the miniatures.
        expect(radii.near).toBeGreaterThan(0);
        expect(radii.bad).toEqual([]);
        const fit = fits([dialog], ".base-dialog, .base-button, .mini, .naming, .base-text-field > .frame");
        expect(fit.seen).toBeGreaterThan(0);
        expect(fit.boxed).toBeGreaterThan(0);
        expect(fit.problems).toEqual([]);
        const icons = iconsCentred([dialog]);
        expect(icons.icons).toBeGreaterThan(0);
        expect(icons.centred).toBeGreaterThan(0);
        expect(icons.problems).toEqual([]);
        expect(await violations(dialog, { contrast: true })).toEqual([]);
        const judged = await contrastJudged(dialog);
        expect(judged.passes).toBeGreaterThan(0);
        expect(judged.incomplete).toBe(0);
      }
    });
  });
}

describe("the overview's grid, under the keys (R3-23 of the review)", () => {
  it("moves by geometry with the arrows, enters a view with Enter, and gives the focus back to the view's name", async () => {
    await frame("light");
    // ⛔ THE CORE HAS ANSWERED (E81): before it, «Salva questa vista» is off.
    useLayout().receive({ kind: "Layout", value: { state: "Nothing" } });
    await userEvent.keyboard("{F3}");
    await expect.poll(overview).not.toBeNull();
    // Three columns: Home, Lavoro, Compatta above, «Salva questa vista» alone below, under Home. The focus opens on the
    // view on screen.
    await expect.poll(() => document.activeElement).toBe(cards()[0]);
    await userEvent.keyboard("{ArrowRight}");
    expect(document.activeElement).toBe(cards()[1]);
    // ⛔ DOWN GOES TO THE ROW BELOW, NOT TO THE NEXT CARD: `RovingFocusGroup` would have gone right (decision 19).
    await userEvent.keyboard("{ArrowDown}");
    expect(document.activeElement).toBe(cards()[3]);
    // Up again: three cards are as far, and the nearest centre wins -- Home, the column of the card below (R3-18).
    await userEvent.keyboard("{ArrowUp}");
    expect(document.activeElement).toBe(cards()[0]);
    // Nothing beyond: the focus stays.
    await userEvent.keyboard("{ArrowLeft}");
    expect(document.activeElement).toBe(cards()[0]);
    // One card in the tab order: the one the arrows reached.
    expect(cards().map((card) => card.tabIndex)).toEqual([0, -1, -1, -1]);
    await userEvent.keyboard("{ArrowRight}");
    await userEvent.keyboard("{Enter}");
    await expect.poll(() => useLayout().view).toBe("work");
    await expect.poll(overview).toBeNull();
    await expect.poll(() => document.activeElement?.classList.contains("view-name")).toBe(true);
  });
});

describe("the bar, and the focus in a panel that scrolls", () => {
  it("draws the bar as part of the page, and the search's whole placeholder in its field (E57, E39)", async () => {
    await frame("light");
    const bar = document.querySelector(".bar") as HTMLElement;
    // ⛔ NO SURFACE AND NO LINE UNDER IT (E57): the bar is part of the page, as the board's `.m-bar` is.
    expect(getComputedStyle(bar).backgroundColor).toBe("rgba(0, 0, 0, 0)");
    expect(getComputedStyle(bar).borderBottomStyle).toBe("none");
    // ⛔ THE BOX SAYS WHO FILLS IT (E39, decision 16 of the north star): the whole placeholder fits the field, measured
    // in the field's own font -- 315 px in 334 on 2026-09-27; in the 320 px of task 5 it was cut.
    const input = document.querySelector(".search input") as HTMLInputElement;
    const style = getComputedStyle(input);
    const pen = document.createElement("canvas").getContext("2d") as CanvasRenderingContext2D;
    pen.font = style.font;
    expect(input.placeholder.length).toBeGreaterThan(0);
    const room = input.clientWidth - Number.parseFloat(style.paddingLeft) - Number.parseFloat(style.paddingRight);
    expect(pen.measureText(input.placeholder).width).toBeLessThanOrEqual(room);
  });

  it("draws the focus ring of a panel that scrolls inside it, clear of its card, and its corners below follow the card's (E74)", async () => {
    await frame("light");
    // ⛔ CHROME PUTS A BOX THAT SCROLLS IN THE TAB ORDER, and in Home at 1440 x 900 Stato scrolls: the Tab reaches it.
    let panel: HTMLElement | null = null;
    for (let step = 0; step < 30 && panel === null; step += 1) {
      await userEvent.keyboard("{Tab}");
      const focused = document.activeElement;
      if (focused instanceof HTMLElement && focused.closest(".panel") !== null && focused.scrollHeight > focused.clientHeight) panel = focused;
    }
    // ⛔ NON-VACUITY: the Tab reached a panel that scrolls, and its ring is drawn.
    expect(panel).not.toBeNull();
    const box = panel as HTMLElement;
    const style = getComputedStyle(box);
    expect(style.outlineStyle).toBe("solid");
    const card = box.closest(".dv-groupview") as HTMLElement;
    const edge = Number.parseFloat(getComputedStyle(card).borderTopWidth);
    const inner = card.getBoundingClientRect();
    // How far the ring reaches out of the box -- its offset and its width; below zero it is inside. The card clips what
    // passes its border: on the three sides the panel touches, the ring stays within.
    const reach = Number.parseFloat(style.outlineOffset) + Number.parseFloat(style.outlineWidth);
    const ring = box.getBoundingClientRect();
    expect(ring.left - reach).toBeGreaterThanOrEqual(inner.left + edge);
    expect(ring.right + reach).toBeLessThanOrEqual(inner.right - edge);
    expect(ring.bottom + reach).toBeLessThanOrEqual(inner.bottom - edge);
    // ⛔ AND THE CORNERS BELOW ARE THE CARD'S LESS ITS BORDER (answer 4): a square ring in a round corner is cut there.
    const corner = Number.parseFloat(getComputedStyle(card).borderBottomLeftRadius) - edge;
    expect(Number.parseFloat(style.borderBottomLeftRadius)).toBeCloseTo(corner, 1);
    expect(Number.parseFloat(style.borderBottomRightRadius)).toBeCloseTo(corner, 1);
  });
});

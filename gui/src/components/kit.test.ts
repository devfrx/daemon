import { mount } from "@vue/test-utils";
import { beforeEach, describe, expect, it } from "vitest";
import { h, nextTick, ref, type Component } from "vue";

import { PANEL_TYPES } from "../panels/registry";
import { violations } from "../testing/axe";

import BaseButton from "./BaseButton.vue";
import BaseDialog from "./BaseDialog.vue";
import BaseIcon from "./BaseIcon.vue";
import BaseLabel from "./BaseLabel.vue";
import BaseList from "./BaseList.vue";
import BaseRadioGroup from "./BaseRadioGroup.vue";
import BaseStatus from "./BaseStatus.vue";
import BaseTextField from "./BaseTextField.vue";
import { ICONS, isIconName, type IconName } from "./icons";

// ⛔ THE WORDS BELOW ARE SPECIMENS: a base piece carries none of its own (section (b)), so a probe hands them in.

/** `BaseList` is generic for its callers' templates; a probe hands it plain props, so it sees a plain component --
 * through `unknown`, because to `vue-tsc` a generic component is a function, and the direct cast is TS2352. */
const List = BaseList as unknown as Component;

beforeEach(() => {
  document.body.replaceChildren();
});

describe("BaseIcon and the one map (design system, section (b); control 10)", () => {
  it("draws every icon of the map, with `currentColor`, hidden from the reader", () => {
    const names = Object.keys(ICONS) as IconName[];
    expect(names.length).toBeGreaterThan(0);
    for (const name of names) {
      const svg = mount(BaseIcon, { props: { name } }).get("svg");
      expect(svg.attributes("data-icon")).toBe(name);
      expect(svg.attributes("stroke")).toBe("currentColor");
      expect(svg.attributes("aria-hidden")).toBe("true");
      expect(svg.element.children.length, name).toBeGreaterThan(0);
    }
  });

  it("has an icon for every module type, by the same name (D6 of the plan)", () => {
    // ⛔ NON-VACUITY: with no module types, the filter below would be empty and green.
    expect(PANEL_TYPES.length).toBeGreaterThan(0);
    expect(PANEL_TYPES.filter((type) => !isIconName(type.module)).map((type) => type.module)).toEqual([]);
  });

  it("takes only a name of the map, and the compiler is the check", () => {
    // ⛔ `vue-tsc` in the build reads this file, and an UNUSED `@ts-expect-error` is an error too: the line below is
    // proven in both directions -- a name outside the map does not compile, and the directive is not idle.
    // @ts-expect-error -- "not-an-icon" is not an IconName
    const wrong: IconName = "not-an-icon";
    expect(isIconName(wrong)).toBe(false);
  });
});

describe("BaseButton", () => {
  it("is a button that never submits, with the words of whoever uses it", () => {
    const button = mount(BaseButton, { slots: { default: () => "Consenti" } }).get("button");
    expect(button.attributes("type")).toBe("button");
    expect(button.text()).toBe("Consenti");
  });

  it("takes `label` as the name of an icon alone, and only then (WCAG 2.5.3)", () => {
    const alone = mount(BaseButton, { props: { icon: "float", label: "Stacca" } }).get("button");
    expect(alone.attributes("aria-label")).toBe("Stacca");
    expect(alone.attributes("data-icon-only")).toBeDefined();
    const worded = mount(BaseButton, { props: { icon: "float", label: "Stacca" }, slots: { default: () => "Stacca la tessera" } }).get("button");
    // ⛔ THE SECOND DIRECTION: with visible words, the words are the name.
    expect(worded.attributes("aria-label")).toBeUndefined();
  });

  it("names an icon alone again when the words go away: the slot is read at every render", async () => {
    const words = ref(true);
    const wrapper = mount({
      render: () => h(BaseButton, { icon: "float", label: "Stacca" }, words.value ? { default: () => "Stacca la tessera" } : {}),
    });
    expect(wrapper.get("button").attributes("aria-label")).toBeUndefined();
    words.value = false;
    await nextTick();
    // ⛔ `useSlots()` is not reactive: a `computed` over it keeps its first answer, and this button stays without a name.
    expect(wrapper.get("button").attributes("aria-label")).toBe("Stacca");
    expect(wrapper.get("button").attributes("data-icon-only")).toBeDefined();
  });

  it("carries its shape to the element, and is off when disabled", () => {
    const button = mount(BaseButton, { props: { variant: "card", size: "lg", pill: true, disabled: true }, slots: { default: () => "Home" } }).get("button");
    expect(button.attributes("data-variant")).toBe("card");
    expect(button.attributes("data-size")).toBe("lg");
    expect(button.attributes("data-pill")).toBeDefined();
    expect((button.element as HTMLButtonElement).disabled).toBe(true);
  });
});

describe("BaseStatus -- M-3 of E187, closed by construction", () => {
  it("is in the DOM while empty, and the words enter the same region", async () => {
    const shown = ref(false);
    const wrapper = mount(() => h(BaseStatus, null, { default: () => (shown.value ? h("p", "Il core non ha risposto.") : null) }));
    const region = wrapper.get('[role="status"]');
    expect(region.text()).toBe("");
    shown.value = true;
    await nextTick();
    // ⛔ THE SAME ELEMENT, NOW WITH WORDS: a region born with its text is the case many readers do not announce.
    expect(wrapper.get('[role="status"]').element).toBe(region.element);
    expect(region.text()).toBe("Il core non ha risposto.");
  });
});

describe("BaseList and BaseLabel", () => {
  it("draws one row per item through the slot, as a list or an ordered one", () => {
    const items = ["uno", "due"];
    const slots = { item: ({ item }: { item: string }) => h("span", item) };
    const plain = mount(List, { props: { items, keyOf: (item: string) => item }, slots });
    expect(plain.get("ul").findAll("li").map((row) => row.text())).toEqual(items);
    const ordered = mount(List, { props: { items, keyOf: (item: string) => item, ordered: true }, slots });
    expect(ordered.find("ol").exists()).toBe(true);
  });

  it("puts the label in the element asked for, with its icon", () => {
    const label = mount(BaseLabel, { props: { icon: "permissions", as: "h3" }, slots: { default: () => "Permessi" } });
    expect(label.get("h3").text()).toBe("Permessi");
    expect(label.find('svg[data-icon="permissions"]').exists()).toBe(true);
  });
});

describe("BaseTextField", () => {
  it("binds its text, puts the caller's attributes on the input, and says an error under it", async () => {
    const text = ref("");
    const wrapper = mount(() =>
      h(BaseTextField, {
        label: "Nome della vista",
        placeholder: "Revisione",
        modelValue: text.value,
        "onUpdate:modelValue": (value: string) => (text.value = value),
        error: text.value === "Home" ? "esiste già" : undefined,
      }),
    );
    const input = wrapper.get("input");
    expect(input.attributes("aria-label")).toBe("Nome della vista");
    expect(input.attributes("placeholder")).toBe("Revisione");
    // ⛔ THE SECOND DIRECTION: and not on the root around it as well -- a caller's `@keydown` there would run twice, on
    // the input and on its bubble (E15 of the plan).
    expect(wrapper.element.hasAttribute("placeholder")).toBe(false);
    await input.setValue("Home");
    expect(text.value).toBe("Home");
    await nextTick();
    expect(wrapper.get("input").attributes("aria-invalid")).toBe("true");
    const describedBy = wrapper.get("input").attributes("aria-describedby");
    expect(wrapper.get(`[id="${describedBy}"]`).text()).toBe("esiste già");
  });
});

describe("BaseRadioGroup -- controlled (P-8 of the plan)", () => {
  const options = [
    { value: "remote", label: "OpenRouter, VRAM libera" },
    { value: "local", label: "Locale" },
  ];
  it("checks what it is given, asks on a click, and does not move by itself", async () => {
    const asked: string[] = [];
    const wrapper = mount(BaseRadioGroup, {
      attachTo: document.body,
      props: { modelValue: "remote", options, legend: "Policy VRAM", "onUpdate:modelValue": (value: string) => asked.push(value) },
    });
    const checked = (): (string | undefined)[] => wrapper.findAll('[role="radio"]').map((radio) => radio.attributes("aria-checked"));
    expect(checked()).toEqual(["true", "false"]);
    await wrapper.findAll('[role="radio"]')[1]?.trigger("click");
    expect(asked).toEqual(["local"]);
    // ⛔ WHOEVER HOLDS THE VALUE DECIDES: the prop did not change, so the check did not move.
    expect(checked()).toEqual(["true", "false"]);
    await wrapper.setProps({ modelValue: "local" });
    expect(checked()).toEqual(["false", "true"]);
    wrapper.unmount();
  });

  it("checks nothing on null, and names the group with its legend", () => {
    const wrapper = mount(BaseRadioGroup, { props: { modelValue: null, options, legend: "Policy VRAM" } });
    expect(wrapper.findAll('[role="radio"]').map((radio) => radio.attributes("aria-checked"))).toEqual(["false", "false"]);
    const group = wrapper.get('[role="radiogroup"]');
    expect(wrapper.get(`[id="${group.attributes("aria-labelledby")}"]`).text()).toBe("Policy VRAM");
  });
});

describe("BaseDialog", () => {
  it("opens bound, names itself with its title, and asks to close on Esc", async () => {
    const asked: boolean[] = [];
    const wrapper = mount(BaseDialog, {
      attachTo: document.body,
      props: { open: true, title: "Serve un permesso", "onUpdate:open": (value: boolean | undefined) => asked.push(value === true) },
      slots: { actions: () => h(BaseButton, null, () => "Rifiuta") },
    });
    await nextTick();
    const dialog = document.querySelector('[role="dialog"]');
    expect(dialog).not.toBeNull();
    expect(document.getElementById(dialog?.getAttribute("aria-labelledby") ?? "")?.textContent).toBe("Serve un permesso");
    // ⛔ NO DESCRIPTION, NO `aria-describedby` (R2-18 of the review; E16 of the plan): without the component's own
    // `undefined`, reka-ui 2.10.4 points it at a description that is not there, and warns.
    expect(dialog?.hasAttribute("aria-describedby")).toBe(false);
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
    await nextTick();
    expect(asked).toEqual([false]);
    wrapper.unmount();
  });
});

describe("the eight pieces, under axe", () => {
  // ⛔ ONE PROBE PER PIECE (the (b), "un test con axe per ciascuno"), each in the state a user meets.
  const pieces: [string, () => ReturnType<typeof h>][] = [
    ["BaseButton", () => h(BaseButton, null, () => "Consenti")],
    ["BaseButton, an icon alone", () => h(BaseButton, { icon: "fullPage", label: "A pagina intera" })],
    ["BaseIcon, inside a named button", () => h("button", { type: "button" }, [h(BaseIcon, { name: "search" }), "Cerca"])],
    ["BaseLabel", () => h(BaseLabel, { icon: "status", as: "h3" }, () => "Stato")],
    ["BaseList", () => h(List, { items: ["uno"], keyOf: (item: string) => item }, { item: ({ item }: { item: string }) => item })],
    ["BaseStatus", () => h(BaseStatus, null, () => "Richiesta inviata.")],
    ["BaseTextField", () => h(BaseTextField, { label: "Cerca", icon: "search", modelValue: "" })],
    ["BaseRadioGroup", () => h(BaseRadioGroup, { modelValue: "system", options: [{ value: "system", label: "Sistema" }, { value: "dark", label: "Scuro" }], legend: "Tema" })],
  ];
  for (const [name, render] of pieces) {
    it(`${name} has no violation`, async () => {
      const wrapper = mount(render, { attachTo: document.body });
      await nextTick();
      expect(await violations(wrapper.element)).toEqual([]);
      wrapper.unmount();
    });
  }

  it("BaseDialog, open, has no violation", async () => {
    const wrapper = mount(BaseDialog, { attachTo: document.body, props: { open: true, title: "Serve un permesso", description: "Vale per questa sessione." } });
    await nextTick();
    // The portal renders into `body`, so the whole document is the node under probe.
    expect(await violations(document.body)).toEqual([]);
    wrapper.unmount();
  });
});

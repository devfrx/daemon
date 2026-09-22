import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";
import { nextTick } from "vue";

import Confirm from "../components/Confirm.vue";
import { i18n } from "../i18n";
import type { Triple } from "../schema/messages";
import { useConnection } from "../stores/connection";
import { useCore } from "../stores/core";
import { useInvoke } from "../stores/invoke";
import { createFakeBridge } from "../transport/fakeBridge";

import { VRAM_POLICY } from "./functions";
import { MODULES, registerModules } from "./modules";
import { PANEL_TYPES, componentFor, placeholderParams } from "./registry";
import Permissions from "./Permissions.vue";
import Settings from "./Settings.vue";
import Status from "./Status.vue";
import Steps from "./Steps.vue";

/** The triple the `PermissionRequired` fixture carries -- the canonical set's ARBITRARY value
 * (task 3), not the registry's real triple: the fake replays fixtures and invents nothing. */
const TRIPLE_OF_THE_FIXTURE: Triple = { tool: "arbiter", resource: "policy", operation: "Write" };
const t = i18n.global.t;

function wire() {
  const bridge = createFakeBridge();
  const connection = useConnection();
  const core = useCore();
  const invoke = useInvoke();
  invoke.attach(bridge);
  bridge.listen((message) => {
    connection.receive(message);
    core.receive(message);
    invoke.receive(message);
  });
  return { bridge, connection, core, invoke };
}

beforeEach(() => {
  setActivePinia(createPinia());
});

describe("the registry, with the modules plugged in", () => {
  it("builds the five real modules and leaves the other types to the placeholder", () => {
    registerModules();
    // ⛔ WHAT TELLS A BUILT MODULE FROM A PLACEHOLDER IS WHAT IT DRAWS (R7-10): `componentFor`
    // answers a function in both cases, so the renderer is mounted and read. A placeholder says
    // who fills it; a real module never does.
    const drawn = (name: string): string => {
      const renderer = componentFor(name)();
      renderer.init({ api: { id: name }, containerApi: {}, params: placeholderParams(name), title: name } as never);
      const html = renderer.element.innerHTML;
      renderer.dispose?.();
      return html;
    };
    for (const name of Object.keys(MODULES)) expect(drawn(name), name).not.toContain("placeholder");
    // ⛔ THE SECOND DIRECTION: a module type nobody built is still the placeholder, saying who
    // fills it -- otherwise plugging five in would have to be checked in the browser.
    const unbuilt = PANEL_TYPES.filter((type) => !(type.name in MODULES));
    expect(unbuilt.length).toBeGreaterThan(0);
    for (const type of unbuilt) expect(drawn(type.name), type.name).toContain('class="placeholder"');
  });
});

describe("Stato", () => {
  it("says the core has not spoken, and shows no event row, before anything arrives", () => {
    wire();
    const wrapper = mount(Status, { global: { plugins: [i18n] } });
    expect(wrapper.text()).toContain(t("status.unknown"));
    expect(wrapper.find(".event").exists()).toBe(false);
  });

  it("shows the two degradation flags, the policy with its budget, and G16 from the value in Accepted", async () => {
    const { bridge } = wire();
    const wrapper = mount(Status, { global: { plugins: [i18n] } });
    bridge.deliver("Accepted");
    bridge.deliver("Degradation");
    bridge.deliver("Policy");
    await nextTick();
    const text = wrapper.text();
    expect(text).toContain(`${t("status.vram")}: ${t("status.yes")}`);
    expect(text).toContain(`${t("status.routing")}: ${t("status.no")}`);
    expect(text).toContain(t("status.policyName.Remote"));
    expect(text).toContain(t("status.budget", { allocated: "12288", total: "16384" }));
    expect(text).toContain(t("status.protectionValue.AsSystemAccount"));
    expect(text).not.toContain(t("status.unknown"));
  });

  it("shows one event row only once a Verdict has arrived, with Refused told apart", async () => {
    const { bridge } = wire();
    const wrapper = mount(Status, { global: { plugins: [i18n] } });
    bridge.deliver("Verdict");
    await nextTick();
    const event = wrapper.find(".event");
    expect(event.exists()).toBe(true);
    expect(event.text()).toContain(t("status.verdict.Refused"));
    expect(event.text()).toContain(t("status.refusedDetail", { asked: "4096", ceiling: "1024" }));
  });
});

describe("Permessi", () => {
  it("shows the pending triple in words, and the approved list after a yes", async () => {
    const { bridge, invoke } = wire();
    const wrapper = mount(Permissions, { global: { plugins: [i18n] } });
    expect(wrapper.text()).toContain(t("permissions.none"));
    invoke.send({ function: VRAM_POLICY.name, argument: VRAM_POLICY.argument.local });
    bridge.deliver("PermissionRequired");
    await nextTick();
    const words = t("permissions.triple", { tool: "arbiter", resource: "policy", operation: t("permissions.operation.Write") });
    expect(wrapper.find(".pending").text()).toBe(words);
    expect(invoke.approve()).toBe(true);
    await nextTick();
    expect(wrapper.find(".pending").exists()).toBe(false);
    expect(wrapper.find("ul").text()).toContain(words);
  });
});

describe("Passi", () => {
  it("lists the steps the core sent, closed or in doubt", async () => {
    const { bridge, core } = wire();
    const wrapper = mount(Steps, { global: { plugins: [i18n] } });
    expect(wrapper.text()).toContain(t("steps.none"));
    bridge.deliver("Steps");
    await nextTick();
    expect(wrapper.text()).toContain(t("steps.row", { step: "42", function: "arbiter.set_policy" }));
    expect(wrapper.text()).toContain(t("steps.done"));
    // ⛔ THE HALF THE FIXTURE CANNOT REACH (D46): a step in doubt is a typed message to the store.
    core.receive({ kind: "Steps", value: [{ step: "43", function: "vram-policy", done: false }] });
    await nextTick();
    expect(wrapper.text()).toContain(t("steps.inDoubt"));
    expect(wrapper.text()).not.toContain(t("steps.done"));
  });
});

describe("Impostazioni", () => {
  it("is off until the core has said which policy is active", () => {
    wire();
    const wrapper = mount(Settings, { global: { plugins: [i18n] } });
    expect(wrapper.get("fieldset").attributes("disabled")).toBeDefined();
  });

  it("sends Invoke with the registry's literals on a change, and nothing on the current value", async () => {
    const { bridge, invoke } = wire();
    const wrapper = mount(Settings, { global: { plugins: [i18n] } });
    bridge.deliver("Policy");
    await nextTick();
    const [remote, local] = wrapper.findAll("input[type=radio]");
    expect((remote?.element as HTMLInputElement).checked).toBe(true);
    await remote?.setValue(true);
    expect(bridge.sent).toEqual([]);
    await local?.setValue(true);
    expect(bridge.sent).toEqual([{ kind: "Invoke", value: { function: "vram-policy", argument: "local" } }]);
    expect(invoke.inFlight).not.toBeNull();
    expect(wrapper.text()).toContain(t("settings.inFlight"));
  });

  it("leaves the control on the core's policy until the core answers, then moves with it", async () => {
    // ⛔ E184: the probe above asserts what went ON THE WIRE and says nothing about the CONTROL,
    // which is what the reviewer looks at -- "the core decides". Measured in the browser on
    // 2026-09-22: with `v-model` the radio kept the click.
    const { bridge, core } = wire();
    const wrapper = mount(Settings, { global: { plugins: [i18n] } });
    bridge.deliver("Policy");
    await nextTick();
    const [remote, local] = wrapper.findAll("input[type=radio]");
    await local?.setValue(true);
    await nextTick();
    expect((local?.element as HTMLInputElement).checked).toBe(false);
    expect((remote?.element as HTMLInputElement).checked).toBe(true);
    // ⛔ THE SECOND DIRECTION: the core answers, and the control DOES move -- otherwise a control
    // nailed to `remote` would pass the half above.
    core.receive({ kind: "Policy", value: { policy: "Local", allocated: "12288", total: "16384" } });
    await nextTick();
    expect((local?.element as HTMLInputElement).checked).toBe(true);
    expect((remote?.element as HTMLInputElement).checked).toBe(false);
  });
});

describe("the confirmation window", () => {
  it("opens only when the core asked AND a call is in flight, and Approve carries the call", async () => {
    const { bridge, core, invoke } = wire();
    const wrapper = mount(Confirm, { global: { plugins: [i18n] }, attachTo: document.body });
    bridge.deliver("PermissionRequired");
    await nextTick();
    // ⛔ THE SECOND DIRECTION FIRST: a request that follows no Invoke of ours opens nothing.
    expect(document.querySelector(".confirm")).toBeNull();
    core.settled();
    invoke.send({ function: VRAM_POLICY.name, argument: VRAM_POLICY.argument.local });
    bridge.deliver("PermissionRequired");
    await nextTick();
    await nextTick();
    const dialog = document.querySelector(".confirm");
    expect(dialog).not.toBeNull();
    expect(dialog?.textContent).toContain(t("permissions.operation.Write"));
    // G20: the focus is INSIDE the window once it is open.
    expect(dialog?.contains(document.activeElement)).toBe(true);
    const yes = [...document.querySelectorAll(".confirm button")].find((b) => b.textContent?.trim() === t("confirm.yes"));
    expect(yes).toBeDefined();
    (yes as HTMLButtonElement).click();
    // reka-ui unmounts DialogContent through its own dismissable layer: THREE ticks, measured (R13-2).
    await nextTick();
    await nextTick();
    await nextTick();
    expect(bridge.sent.at(-1)).toEqual({ kind: "Approve", triple: TRIPLE_OF_THE_FIXTURE, call: { function: "vram-policy", argument: "local" } });
    expect(core.pending).toBeNull();
    expect(document.querySelector(".confirm")).toBeNull();
    wrapper.unmount();
  });

  it("sends nothing on a no, and closes", async () => {
    const { bridge, core, invoke } = wire();
    const wrapper = mount(Confirm, { global: { plugins: [i18n] }, attachTo: document.body });
    invoke.send({ function: VRAM_POLICY.name, argument: VRAM_POLICY.argument.local });
    bridge.deliver("PermissionRequired");
    await nextTick();
    await nextTick();
    const no = [...document.querySelectorAll(".confirm button")].find((b) => b.textContent?.trim() === t("confirm.no"));
    (no as HTMLButtonElement).click();
    // reka-ui unmounts DialogContent through its own dismissable layer: THREE ticks, measured (R13-3).
    await nextTick();
    await nextTick();
    await nextTick();
    expect(bridge.sent.map((message) => message.kind)).toEqual(["Invoke"]);
    expect(core.pending).toBeNull();
    expect(document.querySelector(".confirm")).toBeNull();
    wrapper.unmount();
  });
});

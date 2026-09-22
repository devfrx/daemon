import { mount } from "@vue/test-utils";
import axe from "axe-core";
import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";
import { nextTick, type Component } from "vue";

import Confirm from "./components/Confirm.vue";
import Band from "./frame/Band.vue";
import ViewBar from "./frame/ViewBar.vue";
import { i18n } from "./i18n";
import Chat from "./panels/Chat.vue";
import { VRAM_POLICY } from "./panels/functions";
import Permissions from "./panels/Permissions.vue";
import Placeholder from "./panels/Placeholder.vue";
import Settings from "./panels/Settings.vue";
import Status from "./panels/Status.vue";
import Steps from "./panels/Steps.vue";
import Strip from "./panels/Strip.vue";
import { useConnection } from "./stores/connection";
import { useCore } from "./stores/core";
import { useInvoke } from "./stores/invoke";
import { useStream } from "./stores/stream";
import { createFakeBridge } from "./transport/fakeBridge";

/**
 * Every violation axe finds under a node, as `rule: targets`, and nothing else.
 *
 * ⛔ `color-contrast` IS DISABLED HERE AND NOT IGNORED: under jsdom axe files it under
 * `incomplete` every time -- there is no layout to read a background from (measured on
 * 2026-09-15, P-86) -- so a green from this rule would prove nothing about contrast.
 * `tokens/contrast.test.ts` is what proves AA, on every text colour over every surface.
 */
async function violations(node: Element): Promise<string[]> {
  const results = await axe.run(node, { rules: { "color-contrast": { enabled: false } } });
  return results.violations.map((violation) => `${violation.id}: ${violation.nodes.map((n) => n.target.join(" ")).join(", ")}`);
}

/** Fills the stores the way a welcome does, so every component has something to draw. */
function welcome(): void {
  const bridge = createFakeBridge();
  const connection = useConnection();
  const core = useCore();
  const stream = useStream();
  bridge.listen((message) => {
    connection.receive(message);
    core.receive(message);
    stream.receive(message);
  });
  bridge.deliverAll();
}

async function mounted(component: Component): Promise<{ element: Element; unmount: () => void }> {
  const wrapper = mount(component, { global: { plugins: [i18n] }, attachTo: document.body });
  await nextTick();
  // One frame, where there is one: the Chat renders the streaming block on the animation frame.
  await new Promise<void>((resolve) => {
    if (typeof requestAnimationFrame === "function") requestAnimationFrame(() => resolve());
    else resolve();
  });
  await nextTick();
  return { element: wrapper.element, unmount: () => wrapper.unmount() };
}

beforeEach(() => {
  setActivePinia(createPinia());
  document.body.innerHTML = "";
});

describe("the probe itself", () => {
  it("catches a button with no name -- so a green below is a finding, not a silence", async () => {
    const host = document.createElement("div");
    host.innerHTML = "<button></button>";
    document.body.append(host);
    expect(await violations(host)).toEqual(expect.arrayContaining([expect.stringMatching(/^button-name/)]));
  });
});

describe("the SPA, with the welcome delivered", () => {
  const modules: [string, Component][] = [
    ["Stato", Status],
    ["Permessi", Permissions],
    ["Passi", Steps],
    ["Impostazioni", Settings],
    ["Chat", Chat],
    ["la striscia", Strip],
    ["la barra", ViewBar],
  ];

  for (const [name, component] of modules) {
    it(`${name} has no violation`, async () => {
      welcome();
      const { element, unmount } = await mounted(component);
      expect(await violations(element)).toEqual([]);
      unmount();
    });
  }

  it("the band, while waiting, has no violation", async () => {
    const { element, unmount } = await mounted(Band);
    expect(await violations(element)).toEqual([]);
    unmount();
  });

  it("the placeholder, in both of its states, has no violation", async () => {
    for (const params of [{ module: "costs", who: 3 }, { missing: true }]) {
      const wrapper = mount(Placeholder, { global: { plugins: [i18n] }, attachTo: document.body, props: { params } });
      await nextTick();
      expect(await violations(wrapper.element)).toEqual([]);
      wrapper.unmount();
    }
  });

  it("the confirmation window, open, has no violation", async () => {
    welcome();
    const core = useCore();
    const invoke = useInvoke();
    core.settled();
    invoke.send({ function: VRAM_POLICY.name, argument: VRAM_POLICY.argument.local });
    core.receive({ kind: "PermissionRequired", value: { tool: "registry", resource: "arbiter", operation: "Write" } });
    const { unmount } = await mounted(Confirm);
    const dialog = document.querySelector(".confirm");
    expect(dialog).not.toBeNull();
    // The portal renders into `body`, so the whole document is the node under probe.
    expect(await violations(document.body)).toEqual([]);
    unmount();
  });
});

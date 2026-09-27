import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";
import { nextTick, type Component } from "vue";

import Confirm from "./components/Confirm.vue";
import Band from "./frame/Band.vue";
import Drawer from "./frame/Drawer.vue";
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
import { VIEWS } from "./panels/views";
import { useConnection } from "./stores/connection";
import { useCore } from "./stores/core";
import { useDrawer } from "./stores/drawer";
import { useInvoke } from "./stores/invoke";
import { useStream } from "./stores/stream";
import { violations } from "./testing/axe";
import { createFakeBridge } from "./transport/fakeBridge";

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

async function mounted(component: Component, props: Record<string, unknown> = {}): Promise<{ element: Element; unmount: () => void }> {
  const wrapper = mount(component, { global: { plugins: [i18n] }, attachTo: document.body, props });
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
  // The bar holds the overview's trigger, which saves the layout on screen: a snapshot of Home stands for the dock.
  const bar = { snapshot: () => VIEWS.home, overview: false };
  const modules: [string, Component, Record<string, unknown>?][] = [
    ["Stato", Status],
    ["Permessi", Permissions],
    ["Passi", Steps],
    ["Impostazioni", Settings],
    ["Chat", Chat],
    ["la striscia", Strip],
    ["la barra", ViewBar, bar],
  ];

  for (const [name, component, props] of modules) {
    it(`${name} has no violation`, async () => {
      welcome();
      const { element, unmount } = await mounted(component, props);
      expect(await violations(element)).toEqual([]);
      unmount();
    });
  }

  it("the overview, open, has no violation", async () => {
    welcome();
    const { unmount } = await mounted(ViewBar, { ...bar, overview: true });
    expect(document.querySelector('.base-dialog[data-variant="full"]')).not.toBeNull();
    // The portal renders into `body`, so the whole document is the node under probe.
    expect(await violations(document.body)).toEqual([]);
    unmount();
  });

  it("the drawer, open, has no violation", async () => {
    useDrawer().open = true;
    const { unmount } = await mounted(Drawer);
    expect(document.querySelector('.base-dialog[data-variant="sheet"]')).not.toBeNull();
    expect(await violations(document.body)).toEqual([]);
    unmount();
  });

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
    const dialog = document.querySelector('[role="dialog"]');
    expect(dialog).not.toBeNull();
    // The portal renders into `body`, so the whole document is the node under probe.
    expect(await violations(document.body)).toEqual([]);
    unmount();
  });
});

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
import { useLayout } from "./stores/layout";
import { useStream } from "./stores/stream";
import { violations } from "./testing/axe";
import { createFakeBridge, type FakeBridge } from "./transport/fakeBridge";

/**
 * The welcome as the core sends it -- `greet` in `crates/kernel/src/serving.rs`, in the order sequence 1 of the north star
 * fixes -- handed to the five stores as `main.ts` hands every message.
 *
 * ⛔ A WELCOME AND NOT THE CANONICAL SET (AUD-2188 of the audit of 2026-09-30): `deliverAll` hands over every fixture, the two
 * branches of sequence 1 among them -- `StaleBuild` after `Accepted` -- and left the bar's chip `stale`, which no welcome
 * gives, and a permission waiting on no call of ours. What follows a welcome in a session is each probe's own, below.
 */
function welcome(): FakeBridge {
  const bridge = createFakeBridge();
  const connection = useConnection();
  const core = useCore();
  const layout = useLayout();
  const invoke = useInvoke();
  const stream = useStream();
  bridge.listen((message) => {
    connection.receive(message);
    core.receive(message);
    layout.receive(message);
    invoke.receive(message);
    stream.receive(message);
  });
  for (const kind of ["Accepted", "Degradation", "Policy", "Layout", "Steps"] as const) bridge.deliver(kind);
  return bridge;
}

/** A call of ours, and the core's question about it: what opens the confirmation window, and what Permessi shows waiting. */
function asked(bridge: FakeBridge): void {
  useInvoke().send({ function: VRAM_POLICY.name, argument: VRAM_POLICY.argument.local });
  bridge.deliver("PermissionRequired");
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
  /** Each module after the welcome, with what follows it in a session where the module draws that: a verdict for Stato, a
   * question about a call of ours for Permessi, a piece of the stream for the Chat. */
  const modules: { name: string; component: Component; props?: Record<string, unknown>; then?: (bridge: FakeBridge) => void }[] = [
    { name: "Stato", component: Status, then: (bridge) => bridge.deliver("Verdict") },
    { name: "Permessi", component: Permissions, then: asked },
    { name: "Passi", component: Steps },
    { name: "Impostazioni", component: Settings },
    { name: "Chat", component: Chat, then: (bridge) => bridge.deliver("Token") },
    { name: "la striscia", component: Strip },
    { name: "la barra", component: ViewBar, props: bar },
  ];

  for (const { name, component, props, then } of modules) {
    it(`${name} has no violation`, async () => {
      then?.(welcome());
      const { element, unmount } = await mounted(component, props);
      expect(await violations(element)).toEqual([]);
      unmount();
    });
  }

  it("leaves the core connected and nothing waiting: the state a user meets (AUD-2188 of the audit of 2026-09-30)", () => {
    // ⛔ NON-VACUITY OF THE PROBES ABOVE: they judge the SPA a welcome leaves, the bar's chip `connected` among it.
    welcome();
    expect(useConnection().phase).toBe("connected");
    expect(useCore().pending).toBeNull();
    expect(useCore().degradation).not.toBeNull();
    expect(useCore().steps.length).toBeGreaterThan(0);
  });

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
    asked(welcome());
    const { unmount } = await mounted(Confirm);
    const dialog = document.querySelector('[role="dialog"]');
    expect(dialog).not.toBeNull();
    // The portal renders into `body`, so the whole document is the node under probe.
    expect(await violations(document.body)).toEqual([]);
    unmount();
  });
});

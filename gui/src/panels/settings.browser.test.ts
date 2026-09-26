import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { userEvent } from "vitest/browser";
import { afterEach, beforeEach, expect, it } from "vitest";
import { nextTick } from "vue";

import "../tokens";
import { i18n } from "../i18n";
import { useConnection } from "../stores/connection";
import { useCore } from "../stores/core";
import { useInvoke } from "../stores/invoke";
import { createFakeBridge } from "../transport/fakeBridge";

import Settings from "./Settings.vue";

// ⛔ THE ARROW KEY'S WAY, IN THE INSTALLED CHROME (R3-24 of the design-system review): in `reka-ui` 2.10.4 an arrow reaches
// the radio through `RovingFocusGroup` and a `setTimeout` that clicks it -- another road than the click the jsdom probes
// take, and one an update of `reka-ui` could change with nothing going red. Under jsdom that focus and that timer are
// fragile, so the probe lives here.

beforeEach(() => {
  setActivePinia(createPinia());
});

afterEach(() => {
  document.body.replaceChildren();
});

it("asks the core on an arrow key as on a click, and keeps the check on the core's value", async () => {
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
  const wrapper = mount(Settings, { attachTo: document.body, global: { plugins: [i18n] } });
  bridge.deliver("Policy");
  await nextTick();
  const radios = (): HTMLElement[] => [...(document.querySelector('[role="radiogroup"]')?.querySelectorAll<HTMLElement>('[role="radio"]') ?? [])];
  // ⛔ NON-VACUITY (E37 of the design-system plan): the policy's two radios, or the arrow would have nowhere to go.
  expect(radios()).toHaveLength(2);
  await userEvent.click(radios()[0] as HTMLElement);
  // ⛔ THE SECOND DIRECTION: a click on the current value asks nothing.
  expect(bridge.sent).toEqual([]);
  // ⛔ THE KEY IS HELD, AS A HAND HOLDS IT: `reka-ui` 2.10.4 clicks the radio in a `setTimeout(0)` after the focus, and
  // only while an arrow is still down -- a `keydown` sets the flag, a `keyup` clears it. `{ArrowDown}` presses and
  // releases at once, and a `keyup` that arrived before the timer left nothing clicked: red in 1 run in 10 of the whole
  // suite, measured on 2026-09-23.
  await userEvent.keyboard("{ArrowDown>}");
  await expect.poll(() => bridge.sent.length).toBe(1);
  await userEvent.keyboard("{/ArrowDown}");
  expect(bridge.sent).toEqual([{ kind: "Invoke", value: { function: "vram-policy", argument: "local" } }]);
  // The focus sits on the radio the arrow reached; the check stays on the core's value until `Policy` comes back (P-8).
  expect(radios().map((radio) => radio.getAttribute("aria-checked"))).toEqual(["true", "false"]);
  expect(document.activeElement).toBe(radios()[1]);
  wrapper.unmount();
});

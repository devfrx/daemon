// ⛔ FIRST LINE, AND MEASURED: `dockview-core` does not ship the stylesheet and does not inject
// one (E2 of the part-1 plan, measured in the browser on 2026-09-10). Without it the groups
// stack in the document flow and a floating group leaves the viewport.
import "dockview/dist/styles/dockview.css";
import "./tokens/tokens.css";

import { createPinia } from "pinia";
import { createApp } from "vue";

import App from "./App.vue";
import { i18n } from "./i18n";
import { registerModules } from "./panels/modules";
import { useConnection } from "./stores/connection";
import { useCore } from "./stores/core";
import { useInvoke } from "./stores/invoke";
import { useLayout } from "./stores/layout";
import { useStream } from "./stores/stream";
import type { Bridge } from "./transport/bridge";
import { createFakeBridge, type FakeBridge } from "./transport/fakeBridge";

/**
 * ⛔ THE FAKE BRIDGE IS WHAT SUB-PROJECT 2's SPA RUNS AGAINST IN A BROWSER, and it is not a
 * shortcut: §6a says the SPA is developed and probed against a fake that replays the fixtures
 * BEFORE the shell exists, and the shell is outside this plan (§8 of the sub-project 2 design).
 * The day a shell exists it hands one in on `window`, and this line is all that changes.
 *
 * ⚠️ AND THE FAKE IS EXPOSED TO WHOEVER IS LOOKING (D57): it replays only on request, so in a
 * browser nothing arrives until someone asks -- `harnessFake.deliverAll()` in the console, or one
 * kind at a time. It is a review affordance by construction: a shell's bridge is not a fake and
 * has nothing to expose.
 */
declare global {
  interface Window {
    harnessBridge?: Bridge;
    harnessFake?: FakeBridge;
  }
}

function fakeForTheReviewer(): FakeBridge {
  const fake = createFakeBridge();
  window.harnessFake = fake;
  return fake;
}

const bridge: Bridge = window.harnessBridge ?? fakeForTheReviewer();

// ⛔ BEFORE THE DOCK MOUNTS: `createDock` asks the registry for a component per panel of the view,
// and a module registered after that would be a placeholder until the next `fromJSON`.
registerModules();

const app = createApp(App);
app.use(createPinia());
app.use(i18n);
app.mount("#app");

const connection = useConnection();
const core = useCore();
const layout = useLayout();
const invoke = useInvoke();
const stream = useStream();
connection.attach(bridge);
layout.attach(bridge);
invoke.attach(bridge);
bridge.listen((message) => {
  connection.receive(message);
  core.receive(message);
  layout.receive(message);
  invoke.receive(message);
  stream.receive(message);
});
connection.hello();

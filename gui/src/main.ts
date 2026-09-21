// ⛔ FIRST LINE, AND MEASURED: `dockview-core` does not ship the stylesheet and does not inject
// one (E2 of the part-1 plan, measured in the browser on 2026-09-10). Without it the groups
// stack in the document flow and a floating group leaves the viewport.
import "dockview/dist/styles/dockview.css";
import "./tokens/tokens.css";

import { createPinia } from "pinia";
import { createApp } from "vue";

import App from "./App.vue";
import { i18n } from "./i18n";
import { useConnection } from "./stores/connection";
import { useCore } from "./stores/core";
import { useLayout } from "./stores/layout";
import { createFakeBridge } from "./transport/fakeBridge";
import type { Bridge } from "./transport/bridge";

/**
 * ⛔ THE FAKE BRIDGE IS WHAT SUB-PROJECT 2's SPA RUNS AGAINST IN A BROWSER, and it is not a
 * shortcut: §6a says the SPA is developed and probed against a fake that replays the fixtures
 * BEFORE the shell exists, and the shell is outside this plan (§8 of the sub-project 2 design).
 * The day a shell exists it hands one in on `window`, and this line is all that changes.
 */
declare global {
  interface Window {
    harnessBridge?: Bridge;
  }
}

const bridge: Bridge = window.harnessBridge ?? createFakeBridge();

const app = createApp(App);
app.use(createPinia());
app.use(i18n);
app.mount("#app");

const connection = useConnection();
const core = useCore();
const layout = useLayout();
connection.attach(bridge);
layout.attach(bridge);
bridge.listen((message) => {
  connection.receive(message);
  core.receive(message);
  layout.receive(message);
});
connection.hello();

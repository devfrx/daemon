import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";
import { nextTick } from "vue";

import { i18n } from "../i18n";
import { FREEZE_AT, useStream } from "../stores/stream";
import { createFakeBridge } from "../transport/fakeBridge";

import Chat from "./Chat.vue";

const t = i18n.global.t;

/** One animation frame where there is one, nothing where there is none: the component renders
 * the streaming block on the frame, and immediately without one. */
async function frame(): Promise<void> {
  await new Promise<void>((resolve) => {
    if (typeof requestAnimationFrame === "function") requestAnimationFrame(() => resolve());
    else resolve();
  });
  await nextTick();
}

beforeEach(() => {
  setActivePinia(createPinia());
});

describe("the Chat", () => {
  it("says in words that there is no run, before any token", () => {
    const wrapper = mount(Chat, { global: { plugins: [i18n] } });
    expect(wrapper.text()).toContain(t("chat.noRun"));
    expect(wrapper.findAll("article")).toHaveLength(0);
  });

  it("renders the kernel's Token fixture as markdown, with its provenance on the piece", async () => {
    const bridge = createFakeBridge();
    const stream = useStream();
    bridge.listen((message) => stream.receive(message));
    const wrapper = mount(Chat, { global: { plugins: [i18n] } });
    bridge.deliver("Token");
    await frame();
    expect(wrapper.text()).not.toContain(t("chat.noRun"));
    const streaming = wrapper.get("article.streaming");
    expect(streaming.attributes("data-provenance")).toBe("Untrusted");
    expect(streaming.text()).toContain(t("chat.untrusted"));
    expect(streaming.find(".body").html()).toContain("<p>ciao</p>");
  });

  it("renders text and code, never the model's HTML, and no link that can be followed", async () => {
    const stream = useStream();
    const wrapper = mount(Chat, { global: { plugins: [i18n] } });
    stream.receive({ kind: "Token", text: "```rust\nfn main() {}\n```\n<b>x</b> [q](https://e.com)", provenance: "Untrusted" });
    await frame();
    const body = wrapper.get("article.streaming .body").html();
    expect(body).toContain('<code class="language-rust">');
    expect(body).toContain("&lt;b&gt;x&lt;/b&gt;");
    expect(body).not.toContain("<a");
    expect(body).toContain('data-href="https://e.com"');
  });

  it("renders a block that was ALREADY streaming when the tile mounted", async () => {
    // ⛔ THE DIRECTION EVERY OTHER PROBE HERE MISSES (E183): they all mount BEFORE the token, and
    // the watcher fires. The Chat is a tab of Lavoro, so in the browser it is mounted AFTER the
    // tokens landed -- and without `immediate` the piece was labelled and empty.
    const stream = useStream();
    stream.receive({ kind: "Token", text: "in volo prima del montaggio", provenance: "Untrusted" });
    const wrapper = mount(Chat, { global: { plugins: [i18n] } });
    await frame();
    expect(wrapper.get("article.streaming .body").html()).toContain("<p>in volo prima del montaggio</p>");
  });

  it("renders the text as it stands when the frame runs, not the first token of the frame", async () => {
    const stream = useStream();
    const wrapper = mount(Chat, { global: { plugins: [i18n] } });
    stream.receive({ kind: "Token", text: "primo ", provenance: "Untrusted" });
    stream.receive({ kind: "Token", text: "secondo", provenance: "Untrusted" });
    await frame();
    // ⛔ BOTH, in one frame (R7-2): a render that captured the text at scheduling time showed
    // "primo" alone until the next token arrived -- and no probe delivered two in one frame.
    expect(wrapper.get("article.streaming .body").html()).toContain("<p>primo secondo</p>");
  });

  it("freezes a block at the threshold and keeps its provenance label, as SP-8's tile did", async () => {
    const stream = useStream();
    const wrapper = mount(Chat, { global: { plugins: [i18n] } });
    stream.receive({ kind: "Token", text: "a".repeat(FREEZE_AT), provenance: "Untrusted" });
    await frame();
    expect(wrapper.findAll("article.block:not(.streaming)")).toHaveLength(1);
    expect(wrapper.find("article.streaming").exists()).toBe(false);
    expect(wrapper.get("[aria-live=polite] article").attributes("data-provenance")).toBe("Untrusted");
  });
});

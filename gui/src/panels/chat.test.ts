import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";
import { nextTick } from "vue";

import { i18n } from "../i18n";
import { FREEZE_AT, KEEP, useStream } from "../stores/stream";
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

  it("renders a FROZEN piece as the renderer's output, with its label in words, in both provenances (AUD-723 of the audit of 2026-09-30)", async () => {
    // ⛔ THE OTHER v-html OF THIS FILE: every probe above reads the piece being streamed, and each piece ends frozen,
    // through `block.html`. An HTML tag in the model's text must come out ESCAPED there too -- `block.text` in its place
    // would put it in the page alive -- and the label must be the words, not only the attribute.
    const stream = useStream();
    const wrapper = mount(Chat, { global: { plugins: [i18n] } });
    stream.receive({ kind: "Token", text: "<b>x</b> " + "a".repeat(FREEZE_AT), provenance: "Untrusted" });
    stream.receive({ kind: "Token", text: "**fidato**", provenance: "Trusted" });
    // A change of provenance closes the trusted piece: two frozen pieces, one per label.
    stream.receive({ kind: "Token", text: "dopo", provenance: "Untrusted" });
    await frame();
    const frozen = wrapper.findAll("[aria-live=polite] article");
    expect(frozen).toHaveLength(2);
    const [untrusted, trusted] = frozen;
    expect(untrusted?.get(".provenance").text()).toBe(t("chat.untrusted"));
    const body = untrusted?.get(".body").element.innerHTML ?? "";
    expect(body).toContain("<p>&lt;b&gt;x&lt;/b&gt; a");
    expect(body).not.toContain("<b>x</b>");
    expect(trusted?.get(".provenance").text()).toBe(t("chat.trusted"));
    expect(trusted?.get(".body").element.innerHTML).toContain("<strong>fidato</strong>");
  });

  it("drops the oldest frozen piece without rewriting the others: one node goes, one comes (AUD-539 of the audit of 2026-09-30)", async () => {
    // ⛔ THE LIVE REGION ANNOUNCES WHAT CHANGES IN IT (§6a: "with moderation"), so a freeze must change ONE piece. Once
    // KEEP pieces are frozen every freeze drops the oldest, and a list keyed by POSITION made every article take the next
    // one's content -- KEEP pieces re-read at each freeze. The probe above freezes one piece and never reaches the drop.
    const stream = useStream();
    const wrapper = mount(Chat, { global: { plugins: [i18n] } });
    const freeze = async (n: number): Promise<void> => {
      stream.receive({ kind: "Token", text: `${n} ` + "a".repeat(FREEZE_AT), provenance: "Untrusted" });
      await frame();
    };
    const articles = (): Element[] => wrapper.findAll("[aria-live=polite] article").map((article) => article.element);
    for (let n = 0; n < KEEP; n += 1) await freeze(n);
    const before = articles();
    // ⛔ NON-VACUITY: the list is full, so the next freeze drops a piece.
    expect(before).toHaveLength(KEEP);
    const contents = before.map((article) => article.innerHTML);
    await freeze(KEEP);
    const after = articles();
    expect(after).toHaveLength(KEEP);
    // Every piece that stays is the SAME node, holding what it held: nothing in the region was rewritten.
    for (let index = 0; index < KEEP - 1; index += 1) {
      expect(after[index], `piece ${index + 1}`).toBe(before[index + 1]);
      expect(after[index]?.innerHTML, `piece ${index + 1}`).toBe(contents[index + 1]);
    }
    // And the one that came in is new: the dropped piece's node did not take its place.
    expect(before).not.toContain(after[KEEP - 1]);
    expect(after[KEEP - 1]?.textContent).toContain(`${KEEP} a`);
  });
});

<script setup lang="ts">
import { computed, onUnmounted, ref, watch } from "vue";

import { renderMarkdown } from "../components/markdown";
import { useStream, type Block } from "../stores/stream";

// The Chat table of §1 of the north star, rows 1, 2 and 24 -- what sub-project 2 builds: the stream
// as markdown with code blocks (G4), the provenance on every piece (G13), and the words that say
// there is no run. The writing box is row 10, sub-project 3.
const stream = useStream();
const empty = computed(() => stream.blocks.length === 0 && stream.current === null);

// Frozen blocks are rendered once, when the list changes (a freeze every FREEZE_AT characters).
const frozen = computed(() => stream.blocks.map((block) => ({ ...block, html: renderMarkdown(block.text) })));

// ⛔ ONE RENDER PER ANIMATION FRAME AT MOST for the block being streamed, as SP-8's tile did and as
// M4 was measured: rendering markdown on every token pays the parser per token. Where there is no
// animation frame -- a probe without a visual jsdom -- the render is immediate: the probe sees the
// text, the browser sees the throttle.
const currentHtml = ref("");
let frame = 0;
watch(
  () => stream.current?.text,
  () => {
    // ⛔ THE TEXT IS READ WHEN THE FRAME RUNS, not captured when the frame was scheduled (R7-2):
    // the tokens that land in the same frame would otherwise render the first and hold the rest
    // until the next token. SP-8's tile read the current text, and that is the merit that rises.
    const render = (): void => {
      frame = 0;
      const text = stream.current?.text;
      currentHtml.value = text === undefined ? "" : renderMarkdown(text);
    };
    if (typeof requestAnimationFrame !== "function") {
      render();
    } else if (frame === 0) {
      frame = requestAnimationFrame(render);
    }
  },
  // ⛔ `immediate` BECAUSE A CHAT IS MOUNTED WHILE A BLOCK IS ALREADY IN FLIGHT (E183, measured in
  // the browser on 2026-09-22): the Chat is a tab of Lavoro, the tokens land while Home is open,
  // and a watcher that only fires on the NEXT token leaves the piece labelled and EMPTY until one
  // arrives -- which, with the fake replaying a single `Token`, is never.
  { immediate: true },
);
onUnmounted(() => {
  if (frame !== 0 && typeof cancelAnimationFrame === "function") cancelAnimationFrame(frame);
});

function label(block: Block): string {
  return block.provenance === "Untrusted" ? "chat.untrusted" : "chat.trusted";
}
</script>

<template>
  <section class="chat">
    <p v-if="empty" class="empty">{{ $t("chat.noRun") }}</p>
    <!-- aria-live ON THE FROZEN BLOCKS AND NOT ON THE STREAM (§6a: announced "with moderation"):
         a live region over the streaming block would read every token; a block is announced once,
         when it closes. -->
    <div aria-live="polite">
      <article v-for="(block, index) in frozen" :key="index" class="block" :data-provenance="block.provenance">
        <p class="provenance">{{ $t(label(block)) }}</p>
        <!-- v-html OF OUR OWN OUTPUT: `renderMarkdown` escapes the model's text (html: false), so
             what lands here is HTML the renderer wrote, never HTML the model wrote. Task 15's lint
             (`vue/no-v-html`, a warning in the recommended set) gets these two lines as its
             declared exception, with this reason. -->
        <div class="body" v-html="block.html"></div>
      </article>
    </div>
    <article v-if="stream.current !== null" class="block streaming" :data-provenance="stream.current.provenance">
      <p class="provenance">{{ $t(label(stream.current)) }}</p>
      <div class="body" v-html="currentHtml"></div>
    </article>
  </section>
</template>

<style scoped>
.chat {
  padding: var(--space-3);
  overflow: auto;
  height: 100%;
  box-sizing: border-box;
}
.empty,
.provenance {
  color: var(--color-text-muted);
}
.block {
  margin-bottom: var(--space-3);
}
.block[data-provenance="Untrusted"] {
  /* ⛔ A TEXT ROLE FOR A BORDER, ON PURPOSE (P-13 and E9 of the design-system plan): this border must read 3:1,
     and `--color-border-warn` is decoration. The words carry the provenance. */
  border-left: 3px solid var(--color-text-warn);
  padding-left: var(--space-2);
}
/* The renderer's two spans (D54): a link shows where it would have gone, an image says what it was. */
.body :deep(.link) {
  text-decoration: underline dotted;
}
.body :deep(.link)::after {
  content: " (" attr(data-href) ")";
  color: var(--color-text-muted);
}
.body :deep(.image) {
  color: var(--color-text-muted);
}
</style>

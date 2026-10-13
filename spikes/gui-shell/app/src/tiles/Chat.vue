<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import MarkdownIt from 'markdown-it';
import { wire, type Wire } from '../bridge';

// The chat tile: renders the stream of `Token` messages as markdown -- the message being streamed at most
// ONCE per animation frame; when it passes FREEZE_AT it is rendered once more, right away inside the message
// handler, and kept as frozen HTML. The preset 'default' has html:false and linkify:false (the coordinator's
// decision 51 of the north star): raw HTML in the text is escaped, and a bare address does not become a
// link. Explicit markdown still renders: a link becomes an `<a href>` that a click follows, and an image an
// `<img src>` that the browser fetches with nobody clicking -- so neither the preset nor the template's label
// («... mai come HTML») keeps an untrusted text from making the page reach an address of its choosing; the
// SPA's renderer, gui/src/components/markdown.ts, rewrites both (P-87). The text of a model is untrusted,
// and the tile says so. ⚠️ RECALL OF 2026-10-13 -- audit of 2026-09-30, AUD-2104, AUD-2126.
defineProps<{ title?: string; api?: unknown; containerApi?: unknown; params?: unknown }>();

const md = new MarkdownIt();
const FREEZE_AT = 4000; // characters: a message this long is frozen and a new one begins (the tile keeps no state, G1)
const KEEP = 20; // frozen messages kept in the DOM

const frozen = ref<string[]>([]);
const current = ref('');
let text = '';
let raf = 0;

function onMessage(e: Event): void {
  const m = (e as CustomEvent<Wire>).detail;
  if (m.canale !== 'Token') return;
  text += (text ? ' ' : '') + m.carico;
  if (text.length > FREEZE_AT) {
    frozen.value.push(md.render(text));
    if (frozen.value.length > KEEP) frozen.value.shift();
    text = '';
  }
  if (!raf) {
    raf = requestAnimationFrame(() => {
      raf = 0;
      current.value = md.render(text);
    });
  }
}

onMounted(() => wire.addEventListener('message', onMessage));
onUnmounted(() => {
  wire.removeEventListener('message', onMessage);
  if (raf) cancelAnimationFrame(raf);
});
</script>

<template>
  <div class="chat">
    <p class="prov">provenienza: non fidato (un modello) — reso come testo e codice, mai come HTML</p>
    <div v-for="(html, i) in frozen" :key="i" class="msg untrusted" v-html="html"></div>
    <div class="msg untrusted" v-html="current"></div>
  </div>
</template>

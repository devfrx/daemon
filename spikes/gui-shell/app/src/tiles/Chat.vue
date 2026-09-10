<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import MarkdownIt from 'markdown-it';
import { wire, type Wire } from '../bridge';

// The chat tile: renders the stream of `Token` messages as markdown, ONE render per animation frame at
// most, and only of the message being streamed; finished messages are frozen as rendered HTML. The
// preset 'default' has html:false and linkify:false (decision 51 of the north star): raw HTML in the text
// is escaped and links do not open by themselves. The text of a model is untrusted, and the tile says so.
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

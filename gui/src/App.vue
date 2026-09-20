<script setup lang="ts">
// ⛔ THE FRAME AND THE MODULES ARE TASKS 13 AND 14. What this component exists to prove is that
// a single-file component crosses `vue-tsc` and `vite build` at all -- the web world's level 1.
// Anything drawn here would have to be deleted by the task that draws the real one.
import { MESSAGE_KINDS } from "./schema/parse";
</script>

<template>
  <main>
    <p>{{ MESSAGE_KINDS.length }}</p>
  </main>
</template>

<script setup lang="ts">
// A module that nobody has built yet says who fills it, in words (the coordinator's decision 17 of the north
// star). ⚠️ RECALL OF 2026-10-13 -- audit of 2026-09-30, AUD-1128.
defineProps<{ title?: string; api?: unknown; containerApi?: unknown; params?: { who?: string; text?: string } }>();
</script>

<template>
  <div>
    <p class="tile-who">{{ params?.who ?? '' }}</p>
    <p>{{ params?.text ?? '' }}</p>
  </div>
</template>

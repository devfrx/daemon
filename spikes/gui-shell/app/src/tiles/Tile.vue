<script setup lang="ts">
// A module that nobody has built yet says who fills it, in words (decision 17 of the north star).
defineProps<{ title?: string; api?: unknown; containerApi?: unknown; params?: { who?: string; text?: string } }>();
</script>

<template>
  <div>
    <p class="tile-who">{{ params?.who ?? '' }}</p>
    <p>{{ params?.text ?? '' }}</p>
  </div>
</template>

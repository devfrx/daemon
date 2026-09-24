<script setup lang="ts">
import Frame from "./frame/Frame.vue";
</script>

<template>
  <Frame />
</template>

<!-- ⛔ NOT SCOPED, AND THE ONE PAGE RULE OUTSIDE THE TOKENS (P-4 of the design-system plan): the page is the
     frame's, and `base.css` is the board's block byte for byte, so this rule cannot live there. -->
<style>
html,
body,
#app {
  height: 100%;
  margin: 0;
}
</style>

<script setup lang="ts">
/**
 * The status region of the kit (design system, section (b)) -- and the cure of M-3 of E187 BY CONSTRUCTION: the region
 * is ALWAYS in the DOM, empty and zero high, and the words ENTER it. Many screen readers announce a live region only
 * when its content changes, never one that is born with its text; and an empty `div` draws nothing, which is the
 * "no empty box" rule of §6a of the sub-project 2 design.
 */
</script>

<template>
  <div class="base-status" role="status"><slot /></div>
</template>

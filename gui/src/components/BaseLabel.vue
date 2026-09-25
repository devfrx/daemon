<script setup lang="ts">
import BaseIcon from "./BaseIcon.vue";
import type { IconName } from "./icons";

/**
 * The label of the kit (design system, section (b)): the board's `.lab` -- small capitals, spaced, with an icon in the
 * mark's colour. `as` keeps the meaning where the label is a heading: the titles of Permessi are `h3`.
 */
withDefaults(defineProps<{ icon?: IconName; as?: "span" | "h2" | "h3" | "h4" }>(), { icon: undefined, as: "span" });
</script>

<template>
  <component :is="as" class="base-label">
    <BaseIcon v-if="icon !== undefined" :name="icon" size="sm" />
    <slot />
  </component>
</template>

<style scoped>
.base-label {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin: 0;
  font: var(--font-label);
  letter-spacing: var(--tracking-label);
  text-transform: uppercase;
  color: var(--color-text-muted);
}
.base-label :deep(.base-icon) {
  color: var(--color-mark);
}
</style>

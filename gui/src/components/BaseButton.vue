<script setup lang="ts">
import { useSlots } from "vue";

import BaseIcon from "./BaseIcon.vue";
import type { IconName } from "./icons";

/**
 * The button of the kit (design system, section (b)): the board's `.btn`, with three variants for the controls and a
 * fourth, `card`, for a whole card that is one button -- the views of the overview and "Salva questa vista" (P-9 of the
 * plan). `pill` is imposed by the radius rule: inside a pill goes a pill -- the strip's "moduli".
 *
 * ⛔ NO WORDS INSIDE: they come from whoever uses it, in the slot -- or in `label` when the button is an icon alone,
 * and only then, because with visible words the words are the name (WCAG 2.5.3).
 */
const props = withDefaults(
  defineProps<{
    variant?: "primary" | "secondary" | "quiet" | "card";
    size?: "sm" | "md" | "lg";
    pill?: boolean;
    disabled?: boolean;
    icon?: IconName;
    label?: string;
  }>(),
  { variant: "secondary", size: "md", pill: false, disabled: false, icon: undefined, label: undefined },
);
const slots = useSlots();
/**
 * A function the template calls, NOT a `computed`: `useSlots()` is not reactive, so a `computed` would keep its first
 * answer, and a button whose words go away would stay without a name. A render reads the slots afresh.
 */
function iconOnly(): boolean {
  return props.icon !== undefined && slots.default === undefined;
}
</script>

<template>
  <button
    type="button"
    class="base-button"
    :data-variant="variant"
    :data-size="size"
    :data-pill="pill || undefined"
    :data-icon-only="iconOnly() || undefined"
    :disabled="disabled"
    :aria-label="iconOnly() ? label : undefined"
    :title="iconOnly() ? label : undefined"
  >
    <BaseIcon v-if="icon !== undefined" :name="icon" :size="size === 'lg' ? 'lg' : 'md'" />
    <slot />
  </button>
</template>

<style scoped>
.base-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  height: var(--size-control-md);
  padding: 0 var(--space-3);
  border: var(--border-width) solid var(--color-border);
  border-radius: var(--radius-control);
  background: transparent;
  color: var(--color-text);
  font: var(--font-body-strong);
  white-space: nowrap;
  cursor: pointer;
  transition:
    background-color var(--duration-fast) var(--ease-standard),
    color var(--duration-fast) var(--ease-standard);
}
.base-button:hover:not(:disabled) {
  background: var(--color-bg-fill-hover);
}
.base-button:active:not(:disabled) {
  background: var(--color-bg-fill-active);
}
.base-button:disabled {
  color: var(--color-text-disabled);
  cursor: default;
}

.base-button[data-variant="primary"] {
  background: var(--color-bg-accent);
  border-color: var(--color-bg-accent);
  color: var(--color-text-on-accent);
}
.base-button[data-variant="primary"]:hover:not(:disabled) {
  background: var(--color-bg-accent-hover);
  border-color: var(--color-bg-accent-hover);
}
.base-button[data-variant="primary"]:active:not(:disabled) {
  background: var(--color-bg-accent-active);
  border-color: var(--color-bg-accent-active);
}
.base-button[data-variant="primary"]:disabled {
  background: var(--color-bg-fill);
  border-color: var(--color-bg-fill);
  color: var(--color-text-disabled);
}

.base-button[data-variant="quiet"] {
  border-color: transparent;
  color: var(--color-text-muted);
}
.base-button[data-variant="quiet"]:hover:not(:disabled),
.base-button[data-variant="quiet"]:active:not(:disabled) {
  color: var(--color-text);
}
/* Off, the quiet one too: its muted colour weighs as much as `:disabled` and comes later (E19 of the plan). */
.base-button[data-variant="quiet"]:disabled {
  color: var(--color-text-disabled);
}

/* A whole card that is one button: the card of the (a), a column, the words where they fall. */
.base-button[data-variant="card"] {
  flex-direction: column;
  align-items: stretch;
  justify-content: flex-start;
  height: auto;
  padding: var(--space-3);
  border-color: var(--color-border-card);
  border-radius: var(--radius-card);
  background: var(--color-bg-surface);
  box-shadow: var(--shadow-card);
  font: var(--font-body);
  text-align: start;
  white-space: normal;
}
/* The current card: the mark all around -- the overview's view in bordeaux (the (d)). */
.base-button[data-variant="card"][aria-current]:not([aria-current="false"]) {
  border-color: var(--color-mark);
  box-shadow: 0 0 0 var(--border-width) var(--color-mark);
}

.base-button[data-size="sm"] {
  height: var(--size-control-sm);
  padding: 0 var(--space-2);
}
.base-button[data-size="lg"] {
  height: var(--size-control-lg);
  padding: 0 var(--space-4);
}
.base-button[data-icon-only] {
  padding: 0;
  aspect-ratio: 1;
}
.base-button[data-pill] {
  border-radius: var(--radius-full);
}
</style>

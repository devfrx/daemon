<script setup lang="ts">
import { useId } from "vue";

import BaseIcon from "./BaseIcon.vue";
import type { IconName } from "./icons";

/**
 * The text field of the kit (design system, section (b); decision 20): the name of a saved view and the bar's search
 * are its two occurrences. `label` is the accessible name -- the board draws no visible label, the icon and the
 * placeholder speak to the eye; `error` says what is wrong under the field and marks it invalid.
 * ⛔ The caller's attributes -- `placeholder`, `@keydown` -- land on the INPUT, not on the frame around it.
 */
defineOptions({ inheritAttrs: false });
withDefaults(defineProps<{ label: string; icon?: IconName; disabled?: boolean; error?: string; type?: "text" | "search" }>(), {
  icon: undefined,
  disabled: false,
  error: undefined,
  type: "text",
});
const model = defineModel<string>({ default: "" });
const errorId = useId();
</script>

<template>
  <div class="base-text-field">
    <div class="frame" :data-error="error !== undefined || undefined" :data-disabled="disabled || undefined">
      <BaseIcon v-if="icon !== undefined" :name="icon" />
      <input
        v-model="model"
        v-bind="$attrs"
        :type="type"
        :disabled="disabled"
        :aria-label="label"
        :aria-invalid="error !== undefined || undefined"
        :aria-describedby="error !== undefined ? errorId : undefined"
      />
    </div>
    <p v-if="error !== undefined" :id="errorId" class="error">{{ error }}</p>
  </div>
</template>

<style scoped>
.frame {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  height: var(--size-control-md);
  padding: 0 var(--space-3);
  border: var(--border-width) solid var(--color-border-strong);
  border-radius: var(--radius-control);
  background: var(--color-bg);
  color: var(--color-text-muted);
  font: var(--font-body);
}
.frame:hover:not([data-disabled]) {
  border-color: var(--color-text-muted);
}
/* ⛔ THE RING IS THE FRAME'S, as on the board: the input inside gives its own away. */
.frame:has(input:focus-visible) {
  outline: var(--focus-width) solid var(--color-focus);
  outline-offset: var(--focus-offset);
  color: var(--color-text);
}
.frame[data-disabled] {
  background: var(--color-bg-fill);
  border-color: var(--color-border);
  color: var(--color-text-disabled);
}
.frame[data-error] {
  border-color: var(--color-border-stop);
  color: var(--color-text);
}
input {
  flex: 1;
  min-width: 0;
  height: 100%;
  padding: 0;
  border: 0;
  background: transparent;
  color: var(--color-text);
  font: inherit;
}
input:focus-visible {
  outline: none;
}
input:disabled {
  color: var(--color-text-disabled);
}
input::placeholder {
  color: var(--color-text-muted);
}
.error {
  margin: var(--space-1) 0 0;
  font: var(--font-caption);
  color: var(--color-text-stop);
}
</style>

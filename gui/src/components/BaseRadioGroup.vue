<script setup lang="ts">
import { RadioGroupIndicator, RadioGroupItem, RadioGroupRoot } from "reka-ui";
import { useId } from "vue";

/**
 * The choice among a few options (design system, section (b)), on `reka-ui`'s radio group, which gives the arrows and
 * the roles. ⛔ CONTROLLED, AND THAT IS THE POINT (P-8 of the plan): the group SHOWS `modelValue` and only ASKS for a
 * change with `update:modelValue` -- `reka-ui` 2.10.4 keeps no state of its own once a value is given, `null`
 * included -- so a choice the core decides moves when the core answers, not on the click. `null` checks nothing.
 */
defineProps<{ modelValue: string | null; options: readonly { value: string; label: string }[]; legend: string; disabled?: boolean }>();
const emit = defineEmits<{ "update:modelValue": [value: string] }>();
const id = useId();

function ask(value: unknown): void {
  if (typeof value === "string") emit("update:modelValue", value);
}
</script>

<template>
  <div class="base-radio-group">
    <span :id="`${id}-legend`" class="legend">{{ legend }}</span>
    <RadioGroupRoot :model-value="modelValue" :disabled="disabled" :aria-labelledby="`${id}-legend`" class="options" @update:model-value="ask">
      <div v-for="option in options" :key="option.value" class="option">
        <RadioGroupItem :id="`${id}-${option.value}`" :value="option.value" class="radio">
          <RadioGroupIndicator class="dot" />
        </RadioGroupItem>
        <label :for="`${id}-${option.value}`">{{ option.label }}</label>
      </div>
    </RadioGroupRoot>
  </div>
</template>

<style scoped>
.base-radio-group {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}
.legend {
  font: var(--font-label);
  letter-spacing: var(--tracking-label);
  text-transform: uppercase;
  color: var(--color-text-muted);
}
.base-radio-group :deep(.options) {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}
.option {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  min-height: var(--size-target-min);
}
/* The radio is a `button` in `reka-ui` (trap 10): a 24 x 24 target (WCAG 2.5.8) around a 16 px circle. */
.base-radio-group :deep(.radio) {
  display: grid;
  place-items: center;
  width: var(--size-target-min);
  height: var(--size-target-min);
  padding: 0;
  border: 0;
  border-radius: var(--radius-full);
  background: transparent;
  cursor: pointer;
}
.base-radio-group :deep(.radio)::before {
  content: "";
  grid-area: 1 / 1;
  width: var(--size-icon-md);
  height: var(--size-icon-md);
  box-sizing: border-box;
  border: var(--border-width) solid var(--color-border-strong);
  border-radius: var(--radius-full);
  background: var(--color-bg);
}
.base-radio-group :deep(.radio[data-state="checked"])::before {
  border-color: var(--color-mark);
}
.base-radio-group :deep(.dot) {
  grid-area: 1 / 1;
  width: var(--space-2);
  height: var(--space-2);
  border-radius: var(--radius-full);
  background: var(--color-mark);
}
.base-radio-group :deep(.radio[data-disabled]) {
  cursor: default;
}
.base-radio-group :deep(.radio[data-disabled])::before {
  border-color: var(--color-border);
  background: var(--color-bg-fill);
}
label {
  font: var(--font-body);
  color: var(--color-text);
  cursor: pointer;
}
.option:has([data-disabled]) label {
  color: var(--color-text-disabled);
  cursor: default;
}
</style>

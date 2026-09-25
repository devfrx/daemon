<script setup lang="ts">
import { DialogContent, DialogDescription, DialogOverlay, DialogPortal, DialogRoot, DialogTitle, DialogTrigger } from "reka-ui";

/**
 * The modal window of the kit (design system, section (b)), on `reka-ui`'s dialog, which gives Esc, the focus kept inside
 * and given back to the control that opened it (decision 18). Three shapes: `center`, a question with its answers;
 * `sheet`, from the bottom -- the drawer; `full`, the whole window -- the overview.
 *
 * `open` is optional: bound with `v-model:open` the window is the caller's, unbound the `trigger` slot opens it.
 * ⛔ NO WORDS INSIDE: the title and the description are props, the body and the `actions` are slots.
 */
withDefaults(defineProps<{ title: string; description?: string; variant?: "center" | "sheet" | "full" }>(), {
  description: undefined,
  variant: "center",
});
const open = defineModel<boolean | undefined>("open", { default: undefined });
</script>

<template>
  <DialogRoot v-model:open="open">
    <DialogTrigger v-if="$slots.trigger" as-child>
      <slot name="trigger" />
    </DialogTrigger>
    <DialogPortal>
      <DialogOverlay class="base-dialog-veil" />
      <!-- ⛔ WITHOUT A DESCRIPTION, NO `aria-describedby`: reka-ui 2.10.4 would point it at a description that is not
           there, and warn. Its `$attrs` merge after its own, so this `undefined` wins (R2-18 of the review). -->
      <DialogContent
        class="base-dialog"
        :data-variant="variant"
        v-bind="description === undefined ? { 'aria-describedby': undefined } : {}"
      >
        <DialogTitle class="title">{{ title }}</DialogTitle>
        <DialogDescription v-if="description !== undefined" class="description">{{ description }}</DialogDescription>
        <slot />
        <div v-if="$slots.actions" class="actions">
          <slot name="actions" />
        </div>
      </DialogContent>
    </DialogPortal>
  </DialogRoot>
</template>

<style scoped>
.base-dialog-veil {
  position: fixed;
  inset: 0;
  z-index: var(--z-overlay);
  background: var(--color-veil);
}
.base-dialog {
  position: fixed;
  z-index: var(--z-overlay);
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding: var(--space-3);
  border: var(--border-width) solid var(--color-border-card);
  background: var(--color-bg-raised);
  box-shadow: var(--shadow-overlay);
  color: var(--color-text);
}
/* The radii follow the rule of answer 4: a card of 20 with 12 of margin around controls of 8.
   ⚠️ The width here and the sheet's `max-height` below are the PLAN's choice, written by hand: the board has no token
   for a window's size -- its `.dlg` is 74% of its frame -- and a token is the board's to add (R2-18 of the review). */
.base-dialog[data-variant="center"] {
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: min(30rem, calc(100vw - var(--space-12)));
  border-radius: var(--radius-card);
}
.base-dialog[data-variant="sheet"] {
  inset: auto 0 0 0;
  max-height: 60vh;
  overflow: auto;
  border-radius: var(--radius-card) var(--radius-card) 0 0;
}
/* The whole window: its corners are Windows' (the (d)), so ours are square. */
.base-dialog[data-variant="full"] {
  inset: 0;
  overflow: auto;
  padding: var(--space-6);
  border: 0;
  border-radius: 0;
  background: var(--color-bg);
}
.title {
  margin: 0;
  font: var(--font-title);
}
.base-dialog[data-variant="sheet"] .title,
.base-dialog[data-variant="full"] .title {
  font: var(--font-label);
  letter-spacing: var(--tracking-label);
  text-transform: uppercase;
  color: var(--color-text-muted);
}
.description {
  margin: 0;
  color: var(--color-text-muted);
}
.actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-2);
  margin-top: var(--space-1);
}
</style>

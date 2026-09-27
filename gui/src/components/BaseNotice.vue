<script setup lang="ts">
import BaseIcon from "./BaseIcon.vue";

/**
 * The message of the kit (design system, section (b); E60 of its plan): the token board's `.msg` made a piece, for every
 * message of the system -- a tone among four with its icon, a title always, a description and an action when there are.
 * `title` and `description` are props, as in `BaseDialog`, and the action is a slot (decision 29 of the design).
 *
 * ⛔ NO WORDS INSIDE, AND NO CLOSING BY HAND (decision 29): it follows a state and goes when the state does -- a message
 * closed by hand hides a state that is still there, and one that closes itself is a notification, E228, the owner's.
 * ⛔ THE RADIUS IS THE CALLER'S TO SAY: in a card `--radius-control`, as the board's `.msg`; `onPage`, the card's -- the
 * band. The action lives on the page only: there it sits `--space-3` plus the border from the edge with
 * `--radius-control`, concentric by construction (E61); in a card it would sit 13 px from a corner of 8.
 * ⛔ WITH AN ACTION THE ROW IS CENTRED (the owner, 2026-09-27): the action is `--size-control-sm` high and the title's
 * line 20, and at the top the title and its icon sat 2 px above the action's words.
 * It enters a `BaseStatus` and takes no role of its own: the region is always there (M-3 of E187), and the space around
 * the message is the caller's class on it, never the region's (E42).
 */
withDefaults(defineProps<{ tone: "info" | "ok" | "warn" | "stop"; title: string; description?: string; onPage?: boolean }>(), {
  description: undefined,
  onPage: false,
});
</script>

<template>
  <div class="base-notice" :data-tone="tone" :data-on-page="onPage || undefined" :data-action="$slots.action ? '' : undefined">
    <BaseIcon :name="tone" size="lg" />
    <div class="words">
      <p class="title">{{ title }}</p>
      <p v-if="description !== undefined" class="description">{{ description }}</p>
    </div>
    <slot name="action" />
  </div>
</template>

<style scoped>
/* The board's `.msg` (the (a), "Gli stati"): the tone's subtle ground and border, the icon in the tone's text colour --
   the root's colour, which `BaseIcon` draws with --, the title in the text's own and the description muted under it. */
.base-notice {
  display: flex;
  align-items: flex-start;
  gap: var(--space-2);
  padding: var(--space-3);
  border: var(--border-width) solid;
  border-radius: var(--radius-control);
}
.base-notice[data-action] {
  align-items: center;
}
.base-notice[data-on-page] {
  border-radius: var(--radius-card);
}
.words {
  flex: 1;
  min-width: 0;
}
.title {
  margin: 0;
  font: var(--font-body-strong);
  color: var(--color-text);
}
.description {
  margin: 0;
  font: var(--font-caption);
  color: var(--color-text-muted);
}

/* The neutral tone is the accent's (N2 of E60): `info` in the code, because `neutral` is the warm grey's scale. */
.base-notice[data-tone="info"] {
  background: var(--color-bg-accent-subtle);
  border-color: var(--color-border-accent);
  color: var(--color-text-accent);
}
.base-notice[data-tone="ok"] {
  background: var(--color-bg-ok-subtle);
  border-color: var(--color-border-ok);
  color: var(--color-text-ok);
}
.base-notice[data-tone="warn"] {
  background: var(--color-bg-warn-subtle);
  border-color: var(--color-border-warn);
  color: var(--color-text-warn);
}
.base-notice[data-tone="stop"] {
  background: var(--color-bg-stop-subtle);
  border-color: var(--color-border-stop);
  color: var(--color-text-stop);
}
</style>

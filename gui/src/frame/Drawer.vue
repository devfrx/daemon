<script setup lang="ts">
import { DialogClose, DialogContent, DialogOverlay, DialogPortal, DialogRoot, DialogTitle, DialogTrigger } from "reka-ui";

import { PANEL_TYPES } from "../panels/registry";

// ⛔ THE DRAWER IS WHERE "WHO FILLS WHAT" LIVES (decision 16 of the north star): every type with
// its number, so the strip can stay thin. ⚠️ It is also the first component that wants a focus
// trap and the keyboard, which is why `reka-ui` enters with THIS task and not with task 14 --
// two ways of doing the same thing in one SPA is what the consistency criterion refuses.
</script>

<template>
  <DialogRoot>
    <DialogTrigger>{{ $t("drawer.open") }}</DialogTrigger>
    <DialogPortal>
      <DialogOverlay class="drawer-overlay" />
      <DialogContent class="drawer">
        <DialogTitle>{{ $t("drawer.title") }}</DialogTitle>
        <ul>
          <li v-for="type in PANEL_TYPES" :key="type.name">
            <span>{{ $t(`modules.${type.module}`) }}</span>
            <span class="who">{{ $t("drawer.who", { number: type.who }) }}</span>
          </li>
        </ul>
        <DialogClose>{{ $t("drawer.close") }}</DialogClose>
      </DialogContent>
    </DialogPortal>
  </DialogRoot>
</template>

<style scoped>
/* ⛔ THE OVERLAY IS A VEIL, AND `reka-ui` DRESSES NOTHING: without these rules it was a `div` in
   normal flow with no background (R6-16, seen in the browser on 2026-09-16). Both sit above
   `dockview`, whose floating groups are at 99. */
.drawer-overlay {
  position: fixed;
  inset: 0;
  background: rgb(0 0 0 / 0.45);
  z-index: 100;
}
.drawer {
  position: fixed;
  inset: auto 0 0 0;
  z-index: 101;
  max-height: 60vh;
  overflow: auto;
  padding: var(--space-4);
  background: var(--surface-raised);
  border-top: 1px solid var(--line);
  border-radius: var(--radius) var(--radius) 0 0;
}
.who {
  color: var(--ink-dim);
  margin-left: var(--space-2);
}
</style>

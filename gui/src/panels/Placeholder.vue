<script setup lang="ts">
// ⛔ ONE PLACEHOLDER AND NOT EIGHTEEN (decision 17 of the north star): every module the sub-project
// does not build says, in words, WHO fills it -- which is the rule §1 of the north star states
// for the whole catalogue.
import type { DockviewPanelApi } from "dockview-core";

// `api` arrives from `VueContent`, which hands `dockview`'s init parameters to the app.
defineProps<{ api?: DockviewPanelApi; params?: { module?: string; who?: number; missing?: boolean } }>();

// ⛔ THE NUCLEUS SAYS «NIENTE ANCORA» AND THE OTHER TILES SAY WHO FILLS THEM (row Home of "Il
// modello della GUI", and row 7 of §3 of the north star): the centre of Home is the knowledge
// base's graph, and until sub-project 6 it says so in its own words, not with a tile's phrase
// (R6-14, 2026-09-16). It still says who fills it, because the drawer does too.
const NUCLEUS = "knowledge";
</script>

<template>
  <section class="placeholder">
    <template v-if="params?.missing">
      <p>{{ $t("placeholder.missing") }}</p>
      <!-- ⛔ ROW 8 OF §2 SAYS "SAYS SO **AND CLOSES**", AND BOTH HALVES MATTER: closing at once
           would take the words away before anyone read them, and only saying them would leave a
           dead tile in every layout saved from here on. So the panel says it and offers the
           close it is going to do. -->
      <button type="button" @click="api?.close()">{{ $t("placeholder.closeMissing") }}</button>
    </template>
    <template v-else-if="params?.who !== undefined">
      <p v-if="params.module === NUCLEUS">{{ $t("placeholder.nucleus") }}</p>
      <p>{{ $t("placeholder.who", { number: params.who }) }}</p>
    </template>
  </section>
</template>

<style scoped>
.placeholder {
  color: var(--color-text-muted);
  padding: var(--space-4);
}
</style>

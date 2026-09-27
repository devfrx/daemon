<script setup lang="ts">
import type { SerializedDockview } from "dockview-core";

import BaseTextField from "../components/BaseTextField.vue";
import { useConnection } from "../stores/connection";

import Overview from "./Overview.vue";

// THE BAR, FROM THE LEFT (the (d) of the design system): the name of the view on screen, which opens the overview; the
// search; the core's chip. The three buttons in a row -- the tabs the owner did not want, on 2026-09-07 -- are gone, and so
// is the drawer's button: "moduli" came down into the strip.
defineProps<{ snapshot: () => SerializedDockview }>();
const overview = defineModel<boolean>("overview", { required: true });
const connection = useConnection();
</script>

<template>
  <header class="bar">
    <Overview v-model:open="overview" :snapshot="snapshot" />

    <!-- ⚠️ DISABLED AND SAYING WHO FILLS IT, not hidden: decision 16 of the north star wants the search box to say who fills
         it, and a control that is simply absent teaches nothing. -->
    <div class="search">
      <BaseTextField model-value="" type="search" icon="search" :label="$t('bar.search')" :placeholder="$t('bar.searchHint')" disabled />
    </div>

    <span class="chip" :data-phase="connection.phase">
      {{ $t("bar.core") }}:
      {{
        connection.phase === "connected"
          ? $t("bar.coreConnected")
          : connection.phase === "stale"
            ? $t("bar.coreStale")
            : $t("bar.coreWaiting")
      }}
    </span>
  </header>
</template>

<style scoped>
.bar {
  display: flex;
  gap: var(--space-3);
  align-items: center;
  padding: var(--space-2) var(--space-3);
  /* ⛔ NO SURFACE AND NO LINE UNDER IT (the owner at step 8 of task 6, E57): the bar is part of the page, as the board's
     `.m-bar` is. */
}
/* The search and the chip on the right, as on the board. */
.search {
  flex: 1;
  max-width: 24rem;
  margin-left: auto;
}
.chip[data-phase="connected"] {
  color: var(--color-text-accent);
}
.chip[data-phase="stale"] {
  color: var(--color-text-stop);
}
</style>

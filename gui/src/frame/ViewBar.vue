<script setup lang="ts">
import { useConnection } from "../stores/connection";
import { useLayout, type ViewName } from "../stores/layout";

import Drawer from "./Drawer.vue";

const connection = useConnection();
const layout = useLayout();
const views: ViewName[] = ["home", "work", "compact"];
const emit = defineEmits<{ (event: "switch", view: ViewName): void }>();
</script>

<template>
  <header class="bar">
    <nav :aria-label="$t('bar.views')">
      <button
        v-for="name in views"
        :key="name"
        type="button"
        :aria-current="layout.view === name ? 'page' : undefined"
        @click="emit('switch', name)"
      >
        {{ $t(`views.${name}`) }}
      </button>
    </nav>

    <!-- ⚠️ DISABLED AND SAYING WHO FILLS IT, not hidden: decision 16 of the north star wants the
         search box to say who fills it, and a control that is simply absent teaches nothing. -->
    <input class="search" type="search" disabled :placeholder="$t('bar.searchHint')" :aria-label="$t('bar.search')" />

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

    <Drawer />
  </header>
</template>

<style scoped>
.bar {
  display: flex;
  gap: var(--space-3);
  align-items: center;
  padding: var(--space-2) var(--space-3);
  background: var(--color-bg-raised);
  border-bottom: var(--border-width) solid var(--color-border);
}
.search {
  flex: 1;
  max-width: 320px;
}
.chip[data-phase="connected"] {
  color: var(--color-text-accent);
}
.chip[data-phase="stale"] {
  color: var(--color-text-stop);
}
</style>

<script setup lang="ts">
import { useCore } from "../stores/core";

// ⚠️ ONLY WHAT IS ALIVE IN SUB-PROJECT 2 (decision 16 of the north star): degradation and
// permissions. Seven "arrives with N" in a thin strip is noise, and the drawer is where "who
// fills what" belongs.
const core = useCore();
</script>

<template>
  <div class="strip">
    <span>
      {{ $t("strip.degradation") }}:
      <template v-if="core.degradation === null">—</template>
      <template v-else-if="core.degradation.vram_exhausted || core.degradation.routing_degraded">
        <span v-if="core.degradation.vram_exhausted" class="warn">{{ $t("strip.vram") }}</span>
        <span v-if="core.degradation.routing_degraded" class="warn">{{ $t("strip.routing") }}</span>
      </template>
      <template v-else>{{ $t("strip.none") }}</template>
    </span>
    <span>
      {{ $t("strip.permissions") }}:
      {{ core.pending === null ? $t("strip.quiet") : $t("strip.pending") }}
    </span>
  </div>
</template>

<style scoped>
.strip {
  display: flex;
  gap: var(--space-4);
  align-items: center;
  height: 100%;
  padding: 0 var(--space-3);
  color: var(--ink-dim);
}
.warn {
  color: var(--warn);
  margin-left: var(--space-1);
}
</style>

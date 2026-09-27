<script setup lang="ts">
import BaseButton from "../components/BaseButton.vue";
import { useCore } from "../stores/core";
import { useDrawer } from "../stores/drawer";

// ⚠️ ONLY WHAT IS ALIVE IN SUB-PROJECT 2 (decision 16 of the north star): degradation and
// permissions. Seven "arrives with N" in a thin strip is noise, and the drawer is where "who
// fills what" belongs.
// ⛔ AND THE DRAWER'S BUTTON, "moduli", came down from the bar (the (d) of the design system): the strip is a panel, a Vue
// app of its own, so it opens the drawer through the store and not through a ref (R3-20 of the review).
const core = useCore();
const drawer = useDrawer();
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
    <BaseButton class="modules" size="lg" pill icon="modules" @click="drawer.open = true">{{ $t("drawer.open") }}</BaseButton>
  </div>
</template>

<style scoped>
/* ⛔ THE PILL'S RULE (answer 4 of the design system): inside a pill goes a pill, the same distance from its edge all round.
   The shipped views give the strip's row 56 px and the dock's gap takes half of `--space-3` above it, so the pill is 50
   high, measured on 2026-09-24 (P-24 of the plan): the large button, 40, sits `--space-1` plus the group's border from the
   pill's edge -- above and below by this padding, on the right by the same. */
.strip {
  display: flex;
  gap: var(--space-4);
  align-items: center;
  height: 100%;
  box-sizing: border-box;
  padding: var(--space-1) var(--space-1) var(--space-1) var(--space-4);
  color: var(--color-text-muted);
}
.warn {
  color: var(--color-text-warn);
  margin-left: var(--space-1);
}
.modules {
  margin-left: auto;
}
</style>

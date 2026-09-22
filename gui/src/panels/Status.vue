<script setup lang="ts">
import { useConnection } from "../stores/connection";
import { useCore } from "../stores/core";

// The Stato table of §1 of the north star, rows 1-4 -- what sub-project 2 builds. Stato SHOWS and
// does not command: the policy change is a registry function and lives in Impostazioni.
const connection = useConnection();
const core = useCore();
</script>

<template>
  <section class="status">
    <dl>
      <dt>{{ $t("status.degradation") }}</dt>
      <dd v-if="core.degradation === null">{{ $t("status.unknown") }}</dd>
      <dd v-else>
        <span :data-flag="core.degradation.vram_exhausted">{{ $t("status.vram") }}: {{ core.degradation.vram_exhausted ? $t("status.yes") : $t("status.no") }}</span>
        <span :data-flag="core.degradation.routing_degraded">{{ $t("status.routing") }}: {{ core.degradation.routing_degraded ? $t("status.yes") : $t("status.no") }}</span>
      </dd>
      <dt>{{ $t("status.policy") }}</dt>
      <dd v-if="core.policy === null">{{ $t("status.unknown") }}</dd>
      <dd v-else>{{ $t(`status.policyName.${core.policy.policy}`) }} — {{ $t("status.budget", { allocated: core.policy.allocated, total: core.policy.total }) }}</dd>
      <dt>{{ $t("status.protection") }}</dt>
      <!-- G16 AS A VALUE: the text comes from what `Accepted` carried, through a key, never as a
           fixed string in the gui (ADR-0023). -->
      <dd v-if="connection.protection === null">{{ $t("status.unknown") }}</dd>
      <dd v-else>{{ $t(`status.protectionValue.${connection.protection}`) }}</dd>
    </dl>
    <!-- ONE EVENT ROW FOR THE LAST Verdict, AND ONLY WHEN ONE HAS ARRIVED (§6a): no empty box. -->
    <p v-if="core.lastVerdict !== null" class="event" role="status">
      {{ $t(`status.verdict.${core.lastVerdict.verdict}`) }}
      <template v-if="core.lastVerdict.verdict === 'Refused'">{{ $t("status.refusedDetail", { asked: core.lastVerdict.asked, ceiling: core.lastVerdict.ceiling }) }}</template>
    </p>
  </section>
</template>

<style scoped>
.status {
  padding: var(--space-3);
  overflow: auto;
  height: 100%;
  box-sizing: border-box;
}
dt {
  color: var(--ink-dim);
  margin-top: var(--space-2);
}
dd {
  margin: 0;
  display: flex;
  gap: var(--space-3);
}
[data-flag="true"] {
  color: var(--warn);
}
.event {
  margin-top: var(--space-3);
  border-top: 1px solid var(--line);
  padding-top: var(--space-2);
}
</style>

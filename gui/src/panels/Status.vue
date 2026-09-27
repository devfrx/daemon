<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";

import BaseNotice from "../components/BaseNotice.vue";
import BaseStatus from "../components/BaseStatus.vue";
import { useConnection } from "../stores/connection";
import { useCore } from "../stores/core";

// The Stato table of §1 of the north star, rows 1-4 -- what sub-project 2 builds. Stato SHOWS and
// does not command: the policy change is a registry function and lives in Impostazioni.
const { t } = useI18n();
const connection = useConnection();
const core = useCore();

/** The tone of each verdict (E60 of the design-system plan): queued is news, granted is good, refused stops the call. */
const TONE = { Queued: "info", Granted: "ok", Refused: "stop" } as const;

/** The last verdict as a message: its tone, its words, and the detail of a refusal. */
const verdict = computed(() => {
  const last = core.lastVerdict;
  if (last === null) return null;
  return {
    tone: TONE[last.verdict],
    title: t(`status.verdict.${last.verdict}`),
    description: last.verdict === "Refused" ? t("status.refusedDetail", { asked: last.asked, ceiling: last.ceiling }) : undefined,
  };
});
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
    <!-- ONE MESSAGE FOR THE LAST Verdict, AND ONLY WHEN ONE HAS ARRIVED (§6a): no empty box -- and the message ENTERS a
         region that is always there (M-3 of E187, `BaseStatus`), in the verdict's tone (E60). -->
    <BaseStatus>
      <BaseNotice v-if="verdict !== null" class="event" :tone="verdict.tone" :title="verdict.title" :description="verdict.description" />
    </BaseStatus>
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
  color: var(--color-text-muted);
  margin-top: var(--space-2);
}
dd {
  margin: 0;
  display: flex;
  gap: var(--space-3);
}
[data-flag="true"] {
  color: var(--color-text-warn);
}
/* The space is the message's, never the region's (E42); the line above the old row went with it -- the message has a
   border of its own. */
.event {
  margin-top: var(--space-3);
}
</style>

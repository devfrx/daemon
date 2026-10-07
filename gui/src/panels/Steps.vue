<script setup lang="ts">
import BaseList from "../components/BaseList.vue";
import { useCore } from "../stores/core";

// The Passi table of §1 of the north star: rows 1 and 14, and of rows 2 and 3 what row 1's summary carries -- each of
// the three rows says in its recall what is missing. A PROJECTION of the journal, re-read from the core (`Steps`
// replaces, it never appends). ⛔ THREE FIELDS AND NOT FIVE (P-89, D56): the wire's `StepSummary` carries the step, the
// function and whether it closed. ⚠️ RECALL OF 2026-10-07 -- audit of 2026-09-30, AUD-483.
const core = useCore();
</script>

<template>
  <section class="steps">
    <p v-if="core.steps.length === 0">{{ $t("steps.none") }}</p>
    <BaseList v-else :items="core.steps" :key-of="(step) => step.step" ordered>
      <template #item="{ item }">
        <span>{{ $t("steps.row", { step: item.step, function: item.function }) }}</span>
        <span class="outcome" :data-done="item.done">{{ item.done ? $t("steps.done") : $t("steps.inDoubt") }}</span>
      </template>
    </BaseList>
    <p class="who">{{ $t("steps.who") }}</p>
  </section>
</template>

<style scoped>
.steps {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  padding: var(--space-3);
  overflow: auto;
  height: 100%;
  box-sizing: border-box;
}
p {
  margin: 0;
}
.outcome {
  color: var(--color-text-muted);
}
.outcome[data-done="false"] {
  color: var(--color-text-warn);
}
.who {
  color: var(--color-text-muted);
}
</style>

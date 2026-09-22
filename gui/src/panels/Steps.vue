<script setup lang="ts">
import { useCore } from "../stores/core";

// The Passi table of §1 of the north star, rows 1-3 and 14: a PROJECTION of the journal, re-read
// from the core (`Steps` replaces, it never appends -- task 13's store). ⛔ THREE FIELDS AND NOT
// FIVE (P-89, D56): the wire's `StepSummary` carries the step, the function and whether it closed;
// in sub-project 2 the invoker and the effect class are constants -- one invoker, one function -- and
// the argument is untrusted payload the summary does not carry. The row says so in words.
const core = useCore();
</script>

<template>
  <section class="steps">
    <p v-if="core.steps.length === 0">{{ $t("steps.none") }}</p>
    <ol v-else>
      <li v-for="step in core.steps" :key="step.step" :data-done="step.done">
        {{ $t("steps.row", { step: step.step, function: step.function }) }}
        <span class="outcome">{{ step.done ? $t("steps.done") : $t("steps.inDoubt") }}</span>
      </li>
    </ol>
    <p class="who">{{ $t("steps.who") }}</p>
  </section>
</template>

<style scoped>
.steps {
  padding: var(--space-3);
  overflow: auto;
  height: 100%;
  box-sizing: border-box;
}
.outcome {
  color: var(--ink-dim);
  margin-left: var(--space-2);
}
[data-done="false"] .outcome {
  color: var(--warn);
}
.who {
  color: var(--ink-dim);
  margin-top: var(--space-3);
}
</style>

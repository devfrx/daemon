<script setup lang="ts">
import { useCore } from "../stores/core";
import { useInvoke } from "../stores/invoke";

// The Permessi table of §1 of the north star, rows 1-3, 7-9 -- what sub-project 2 builds: the
// request in flight, the triples THIS session approved, and the rule on duration. The window that
// answers is the frame's (`components/Confirm.vue`, D60), not this panel's.
const core = useCore();
const invoke = useInvoke();
</script>

<template>
  <section class="permissions">
    <h3>{{ $t("permissions.pendingTitle") }}</h3>
    <p v-if="core.pending === null">{{ $t("permissions.none") }}</p>
    <p v-else class="pending">{{ $t("permissions.triple", { tool: core.pending.tool, resource: core.pending.resource, operation: $t(`permissions.operation.${core.pending.operation}`) }) }}</p>
    <h3>{{ $t("permissions.approvedTitle") }}</h3>
    <p v-if="invoke.approved.length === 0">{{ $t("permissions.noneApproved") }}</p>
    <ul v-else>
      <li v-for="(triple, index) in invoke.approved" :key="index">{{ $t("permissions.triple", { tool: triple.tool, resource: triple.resource, operation: $t(`permissions.operation.${triple.operation}`) }) }}</li>
    </ul>
    <p class="rule">{{ $t("permissions.duration") }}</p>
  </section>
</template>

<style scoped>
.permissions {
  padding: var(--space-3);
  overflow: auto;
  height: 100%;
  box-sizing: border-box;
}
h3 {
  font-size: inherit;
  color: var(--color-text-muted);
  margin: var(--space-2) 0 var(--space-1);
}
.pending {
  color: var(--color-text-warn);
}
.rule {
  color: var(--color-text-muted);
  margin-top: var(--space-3);
}
</style>

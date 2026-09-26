<script setup lang="ts">
import BaseLabel from "../components/BaseLabel.vue";
import BaseList from "../components/BaseList.vue";
import { useCore } from "../stores/core";
import { useInvoke } from "../stores/invoke";

// The Permessi table of §1 of the north star, rows 1-3, 7-9 -- what sub-project 2 builds: the request in flight, the triples
// THIS session approved, and the rule on duration. The window that answers is the frame's (`components/Confirm.vue`, D60).
const core = useCore();
const invoke = useInvoke();
</script>

<template>
  <section class="permissions">
    <BaseLabel icon="permissions" as="h3">{{ $t("permissions.pendingTitle") }}</BaseLabel>
    <p v-if="core.pending === null">{{ $t("permissions.none") }}</p>
    <p v-else class="pending">{{ $t("permissions.triple", { tool: core.pending.tool, resource: core.pending.resource, operation: $t(`permissions.operation.${core.pending.operation}`) }) }}</p>
    <BaseLabel icon="permissions" as="h3">{{ $t("permissions.approvedTitle") }}</BaseLabel>
    <p v-if="invoke.approved.length === 0">{{ $t("permissions.noneApproved") }}</p>
    <BaseList v-else :items="invoke.approved" :key-of="(_triple, index) => index">
      <template #item="{ item }">{{ $t("permissions.triple", { tool: item.tool, resource: item.resource, operation: $t(`permissions.operation.${item.operation}`) }) }}</template>
    </BaseList>
    <p class="rule">{{ $t("permissions.duration") }}</p>
  </section>
</template>

<style scoped>
.permissions {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding: var(--space-3);
  overflow: auto;
  height: 100%;
  box-sizing: border-box;
}
p {
  margin: 0;
}
.pending {
  color: var(--color-text-warn);
}
.rule {
  color: var(--color-text-muted);
  margin-top: var(--space-2);
}
</style>

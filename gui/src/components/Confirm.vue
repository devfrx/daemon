<script setup lang="ts">
import { useCore } from "../stores/core";
import { useInvoke } from "../stores/invoke";

import BaseButton from "./BaseButton.vue";
import BaseDialog from "./BaseDialog.vue";

// A COMPOSED piece, and that is why it may read two stores (P-3 of the design-system plan): the base pieces below do not.
// ⛔ OPEN ONLY WHILE `invoke.asking` (D59): the core asked, and about a call of ours -- the rule lives in the store.
const core = useCore();
const invoke = useInvoke();

// ⛔ ESCAPE AND THE VEIL ARE THE "NO" (E41 of the design-system plan): a permission is granted by the yes alone, never by
// closing the window. ⚠️ ADR-0016 does not write this rule -- it was read from its prudent default -- and no living
// document writes it yet: where it is written is the owner's to choose (AUD-2068 of the audit of 2026-09-30).
function onOpenChange(value: boolean | undefined): void {
  if (value !== true) invoke.refuse();
}
</script>

<template>
  <!-- THE TRIPLE IN EVERYDAY WORDS (sequence 3 of the north star, G20): tool, resource, operation -- what the core asked,
       not what the click meant. -->
  <BaseDialog
    :open="invoke.asking"
    :title="$t('confirm.title')"
    :description="core.pending === null ? undefined : $t('permissions.triple', { tool: core.pending.tool, resource: core.pending.resource, operation: $t(`permissions.operation.${core.pending.operation}`) })"
    @update:open="onOpenChange"
  >
    <p v-if="invoke.inFlight !== null">{{ $t("confirm.call", { function: invoke.inFlight.function, argument: invoke.inFlight.argument }) }}</p>
    <p class="scope">{{ $t("confirm.scope") }}</p>
    <template #actions>
      <BaseButton variant="quiet" @click="invoke.refuse()">{{ $t("confirm.no") }}</BaseButton>
      <BaseButton variant="primary" @click="invoke.approve()">{{ $t("confirm.yes") }}</BaseButton>
    </template>
  </BaseDialog>
</template>

<style scoped>
p {
  margin: 0;
}
.scope {
  color: var(--color-text-muted);
}
</style>

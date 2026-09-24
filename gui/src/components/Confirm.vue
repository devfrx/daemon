<script setup lang="ts">
import { DialogContent, DialogDescription, DialogOverlay, DialogPortal, DialogRoot, DialogTitle } from "reka-ui";
import { computed } from "vue";

import { useCore } from "../stores/core";
import { useInvoke } from "../stores/invoke";

const core = useCore();
const invoke = useInvoke();

// ⛔ OPEN ONLY WHEN THE CORE ASKED AND A CALL OF OURS IS IN FLIGHT (D59): a `PermissionRequired`
// following no `Invoke` is a shape the core never produces -- the registry only ever answers one
// -- and a window that opened on it would offer a "yes" with nothing to send. The Permessi panel
// shows such a request; this window does not ask about it.
const open = computed(() => core.pending !== null && invoke.inFlight !== null);

// Escape and the overlay are the "no" (ADR-0016: nothing is granted by silence).
function onOpenChange(value: boolean): void {
  if (!value) invoke.refuse();
}
</script>

<template>
  <DialogRoot :open="open" @update:open="onOpenChange">
    <DialogPortal>
      <DialogOverlay class="confirm-overlay" />
      <DialogContent class="confirm">
        <DialogTitle>{{ $t("confirm.title") }}</DialogTitle>
        <!-- THE TRIPLE IN EVERYDAY WORDS (sequence 3 of the north star, G20): tool, resource,
             operation -- what the core asked, not what the click meant. -->
        <DialogDescription v-if="core.pending !== null">
          {{ $t("permissions.triple", { tool: core.pending.tool, resource: core.pending.resource, operation: $t(`permissions.operation.${core.pending.operation}`) }) }}
        </DialogDescription>
        <p v-if="invoke.inFlight !== null">{{ $t("confirm.call", { function: invoke.inFlight.function, argument: invoke.inFlight.argument }) }}</p>
        <p class="scope">{{ $t("confirm.scope") }}</p>
        <div class="actions">
          <button type="button" @click="invoke.refuse()">{{ $t("confirm.no") }}</button>
          <button type="button" class="primary" @click="invoke.approve()">{{ $t("confirm.yes") }}</button>
        </div>
      </DialogContent>
    </DialogPortal>
  </DialogRoot>
</template>

<style scoped>
.confirm-overlay {
  position: fixed;
  inset: 0;
  background: var(--color-veil);
}
.confirm {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  min-width: 320px;
  padding: var(--space-4);
  background: var(--color-bg-raised);
  border: var(--border-width) solid var(--color-border);
  border-radius: var(--radius-control);
}
.scope {
  color: var(--color-text-muted);
}
.actions {
  display: flex;
  gap: var(--space-2);
  justify-content: flex-end;
  margin-top: var(--space-3);
}
.primary {
  color: var(--color-text-accent);
}
</style>

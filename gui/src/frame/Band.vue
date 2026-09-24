<script setup lang="ts">
import { useConnection } from "../stores/connection";

// ⛔ THE BAND APPEARS ONLY WHEN THE CORE IS MISSING OR THE STAMP IS WRONG (§6a), and it is NOT a
// panel (D50): a panel that came and went would rewrite the saved layout on every disconnection.
// ⛔ AND THERE IS NO THRESHOLD (D48): "not running" and "slow" are one state here, because the
// gui does the same thing in both -- offer `retry`.
const connection = useConnection();
</script>

<template>
  <div v-if="connection.phase !== 'connected'" class="band" role="status">
    <span v-if="connection.phase === 'stale'">
      {{ $t("band.stale") }}
      <template v-if="connection.expected !== null">
        {{ $t("band.expected", { stamp: connection.expected }) }}
      </template>
    </span>
    <template v-else>
      <span>{{ $t("band.waiting") }}</span>
      <button type="button" @click="connection.retry()">{{ $t("band.retry") }}</button>
    </template>
  </div>
</template>

<style scoped>
.band {
  display: flex;
  gap: var(--space-3);
  align-items: center;
  padding: var(--space-2) var(--space-3);
  background: var(--color-bg-raised);
  border-bottom: var(--border-width) solid var(--color-border-warn);
  color: var(--color-text-warn);
}
</style>

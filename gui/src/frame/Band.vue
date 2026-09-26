<script setup lang="ts">
import BaseButton from "../components/BaseButton.vue";
import BaseStatus from "../components/BaseStatus.vue";
import { useConnection } from "../stores/connection";

// ⛔ THE BAND APPEARS ONLY WHEN THE CORE IS MISSING OR THE STAMP IS WRONG (§6a), and it is NOT a panel (D50): a panel that
// came and went would rewrite the saved layout on every disconnection. ⛔ AND THERE IS NO THRESHOLD (D48): "not running"
// and "slow" are one state here, because the gui does the same thing in both -- offer `retry`.
// ⛔ THE REGION IS ALWAYS THERE AND THE BAND ENTERS IT (M-3 of E187, closed by `BaseStatus`): a status region born with
// its text is the one many screen readers never announce.
const connection = useConnection();
</script>

<template>
  <BaseStatus>
    <div v-if="connection.phase !== 'connected'" class="band">
      <span v-if="connection.phase === 'stale'">
        {{ $t("band.stale") }}
        <template v-if="connection.expected !== null">
          {{ $t("band.expected", { stamp: connection.expected }) }}
        </template>
      </span>
      <template v-else>
        <span>{{ $t("band.waiting") }}</span>
        <BaseButton size="sm" @click="connection.retry()">{{ $t("band.retry") }}</BaseButton>
      </template>
    </div>
  </BaseStatus>
</template>

<style scoped>
/* The warning message of the (a): the subtle tint, the warning's text, a border of decoration -- the words carry it. */
.band {
  display: flex;
  gap: var(--space-3);
  align-items: center;
  padding: var(--space-2) var(--space-3);
  background: var(--color-bg-warn-subtle);
  border-bottom: var(--border-width) solid var(--color-border-warn);
  color: var(--color-text-warn);
}
</style>

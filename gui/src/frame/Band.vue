<script setup lang="ts">
import BaseButton from "../components/BaseButton.vue";
import BaseNotice from "../components/BaseNotice.vue";
import BaseStatus from "../components/BaseStatus.vue";
import { useConnection } from "../stores/connection";

// ⛔ THE BAND APPEARS ONLY WHILE THE CORE HAS NOT WELCOMED THIS GUI OR THE STAMP IS WRONG (§6a), and it is NOT a panel
// (D50): a panel that came and went would rewrite the saved layout on every disconnection. ⛔ AND THERE IS NO THRESHOLD
// (D48): every cause of waiting is one state here -- the doc of `useConnection`, in `stores/connection.ts`, names them --
// because the gui does the same thing in all of them: offer `retry`. ⚠️ RECALL OF 2026-10-07 -- audit of 2026-09-30.
// ⛔ THE REGION IS ALWAYS THERE AND THE BAND ENTERS IT (M-3 of E187, closed by `BaseStatus`): a status region born with
// its text is the one many screen readers never announce.
// ⛔ A MESSAGE ON THE PAGE (E60 of the design-system plan): `BaseNotice` with the card's radius, aligned with the cards --
// `--space-3` from the sides and above them, the dock having none on top (answer 20) -- and under the bar, which leaves it
// its own 8 px. The space is the message's, never the region's, which stays zero high when empty (E42). Waiting is a
// warning with «Riprova»; another stamp STOPS the window -- nothing to retry, and the bar's chip is `--color-text-stop`.
const connection = useConnection();
</script>

<template>
  <BaseStatus>
    <BaseNotice
      v-if="connection.phase === 'stale'"
      class="band"
      tone="stop"
      :title="$t('band.stale')"
      :description="connection.expected !== null ? $t('band.expected', { stamp: connection.expected }) : undefined"
      on-page
    />
    <BaseNotice v-else-if="connection.phase !== 'connected'" class="band" tone="warn" :title="$t('band.waiting')" on-page>
      <template #action>
        <BaseButton size="sm" @click="connection.retry()">{{ $t("band.retry") }}</BaseButton>
      </template>
    </BaseNotice>
  </BaseStatus>
</template>

<style scoped>
.band {
  margin: 0 var(--space-3) var(--space-3);
}
</style>

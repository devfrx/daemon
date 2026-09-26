<script setup lang="ts">
import { ref } from "vue";

import BaseButton from "../components/BaseButton.vue";
import BaseDialog from "../components/BaseDialog.vue";
import BaseList from "../components/BaseList.vue";
import { PANEL_TYPES } from "../panels/registry";

// ⛔ THE DRAWER IS WHERE "WHO FILLS WHAT" LIVES (decision 16 of the north star): every type with its number, so the strip
// can stay thin. On `BaseDialog` since the design system -- its second occurrence, with the confirmation window: the two
// veils written by hand, already different, are one role now, `--color-veil`.
const open = ref(false);
</script>

<template>
  <BaseDialog v-model:open="open" :title="$t('drawer.title')" variant="sheet">
    <template #trigger>
      <BaseButton>{{ $t("drawer.open") }}</BaseButton>
    </template>
    <BaseList :items="PANEL_TYPES" :key-of="(type) => type.name">
      <template #item="{ item }">
        <span>{{ $t(`modules.${item.module}`) }}</span>
        <span class="who">{{ $t("drawer.who", { number: item.who }) }}</span>
      </template>
    </BaseList>
    <template #actions>
      <BaseButton variant="quiet" @click="open = false">{{ $t("drawer.close") }}</BaseButton>
    </template>
  </BaseDialog>
</template>

<style scoped>
.who {
  color: var(--color-text-muted);
}
</style>

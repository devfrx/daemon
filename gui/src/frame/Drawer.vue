<script setup lang="ts">
import BaseButton from "../components/BaseButton.vue";
import BaseDialog from "../components/BaseDialog.vue";
import BaseList from "../components/BaseList.vue";
import { PANEL_TYPES } from "../panels/registry";
import { useDrawer } from "../stores/drawer";

// ⛔ THE DRAWER IS WHERE "WHO FILLS WHAT" LIVES (decision 16 of the north star): every type with its number, so the strip
// can stay thin. On `BaseDialog` since the design system -- its second occurrence, with the confirmation window: the two
// veils written by hand, already different, are one role now, `--color-veil`.
// ⛔ OPENED FROM THE STRIP, THROUGH A STORE (the (d); R3-20 of the review): the button lives in another Vue app.
const drawer = useDrawer();
</script>

<template>
  <BaseDialog v-model:open="drawer.open" :title="$t('drawer.title')" variant="sheet">
    <BaseList :items="PANEL_TYPES" :key-of="(type) => type.name">
      <template #item="{ item }">
        <span>{{ $t(`modules.${item.module}`) }}</span>
        <span class="who">{{ $t("drawer.who", { number: item.who }) }}</span>
      </template>
    </BaseList>
    <template #actions>
      <BaseButton variant="quiet" @click="drawer.open = false">{{ $t("drawer.close") }}</BaseButton>
    </template>
  </BaseDialog>
</template>

<style scoped>
.who {
  color: var(--color-text-muted);
}
</style>

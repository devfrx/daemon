<script setup lang="ts">
import BaseButton from "../components/BaseButton.vue";
import BaseLabel from "../components/BaseLabel.vue";
import type { IconName } from "../components/icons";

/**
 * The face of the big grab handle (design system, section (c)): the module's label with its icon, and the two commands as
 * base pieces -- `BigTab.ts` mounts it in the element `dockview` drags.
 *
 * ⛔ A PRESS ON A COMMAND MUST NOT START A DRAG, and stopping `click` alone is not enough: `dockview-core` 8.3.1 begins
 * its pointer drag on `pointerdown`, stopped here and held by `bigtab.test.ts` -- without it, every press of a command
 * drags the tile a few pixels first (SP-8). ⚠️ `mousedown` and `click` stop too, as SP-8 judged the grab with them,
 * and no test holds those two (E54): 8.3.1 does not listen to `mousedown` at all.
 */
defineProps<{ title: string; icon?: IconName }>();
const emit = defineEmits<{ float: []; page: [] }>();
</script>

<template>
  <div class="face">
    <BaseLabel :icon="icon" class="title">{{ title }}</BaseLabel>
    <BaseButton variant="quiet" size="sm" icon="float" :label="$t('menu.float')" @pointerdown.stop @mousedown.stop @click.stop="emit('float')" />
    <BaseButton variant="quiet" size="sm" icon="fullPage" :label="$t('menu.page')" @pointerdown.stop @mousedown.stop @click.stop="emit('page')" />
  </div>
</template>

<style scoped>
.face {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  height: 100%;
  padding: 0 var(--space-2) 0 var(--space-3);
  cursor: grab;
  user-select: none;
}
.title {
  flex: 1;
  min-width: 0;
}
</style>

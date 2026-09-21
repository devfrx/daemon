<script setup lang="ts">
import { onMounted, ref } from "vue";

import { createDock } from "./dock";
import { useLayout, type ViewName } from "../stores/layout";

import Band from "./Band.vue";
import ViewBar from "./ViewBar.vue";

const host = ref<HTMLElement | null>(null);
const layout = useLayout();

onMounted(() => {
  if (host.value !== null) createDock(host.value);
});

function switchTo(view: ViewName): void {
  // ⛔ ONE LINE, AND THE DOCK FOLLOWS (D89): the open view lives in the store and `createDock`
  // watches it, so the bar, the keyboard of task 14 and a package from the core all take the same
  // path -- and none of them saves a view for merely showing it (decision 11).
  layout.view = view;
}
</script>

<template>
  <div class="frame">
    <ViewBar @switch="switchTo" />
    <Band />
    <div ref="host" class="dock"></div>
  </div>
</template>

<style scoped>
.frame {
  display: flex;
  flex-direction: column;
  height: 100%;
}
.dock {
  flex: 1;
  min-height: 0;
}
</style>

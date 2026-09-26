<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";

import Confirm from "../components/Confirm.vue";
import { useLayout, type ViewName } from "../stores/layout";

import Band from "./Band.vue";
import ViewBar from "./ViewBar.vue";
import { createDock } from "./dock";
import { directionOf, moveActive } from "./moveActive";

const host = ref<HTMLElement | null>(null);
const layout = useLayout();
let api: ReturnType<typeof createDock> | null = null;

// G20, move 6 of SP-8: the active tile moves in the four directions from the keyboard. The
// mapping and the geometry live in `moveActive.ts`; this is only the wire.
function onKey(event: KeyboardEvent): void {
  const direction = directionOf(event);
  if (direction === null || api === null) return;
  event.preventDefault();
  moveActive(api, direction);
}

onMounted(() => {
  if (host.value !== null) api = createDock(host.value);
  window.addEventListener("keydown", onKey);
});

onUnmounted(() => {
  window.removeEventListener("keydown", onKey);
});

function switchTo(view: ViewName): void {
  // ⛔ ONE LINE, AND THE DOCK FOLLOWS (D89, task 13): the open view lives in the store and
  // `createDock` watches it, so the bar, the keyboard above and a package from the core all take
  // the same path -- and none of them saves a view for merely showing it (decision 11).
  layout.view = view;
}
</script>

<template>
  <div class="frame">
    <ViewBar @switch="switchTo" />
    <Band />
    <Confirm />
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
  /* ⛔ THE DOCK KEEPS ITS SPILL TO ITSELF (E43): `dockview` 8.3.1 resizes one frame late -- its ResizeObserver hands the
     new size to a `requestAnimationFrame` -- so for a frame after the band comes in, the grid is as tall as before and
     spills out of this box; unclipped, the spill reached the page and flashed both its scrollbars. `clip` and not
     `hidden`: nothing may scroll this box either. */
  overflow: clip;
}
</style>

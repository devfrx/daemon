<script setup lang="ts">
import type { SerializedDockview } from "dockview-core";
import { onMounted, onUnmounted, ref } from "vue";

import Confirm from "../components/Confirm.vue";
import { useDrawer } from "../stores/drawer";
import { useInvoke } from "../stores/invoke";

import Band from "./Band.vue";
import Drawer from "./Drawer.vue";
import ViewBar from "./ViewBar.vue";
import { createDock, type Dock } from "./dock";
import { directionOf, moveActive } from "./moveActive";

const host = ref<HTMLElement | null>(null);
const drawer = useDrawer();
const invoke = useInvoke();
/** The overview of the views: opened from the view's name in the bar, or with F3 here. */
const overview = ref(false);
let dock: Dock | null = null;

function onKey(event: KeyboardEvent): void {
  // ⛔ F3 OPENS AND CLOSES THE OVERVIEW (the (d) of the design system), AND IS QUIET WHILE ANOTHER WINDOW IS OPEN -- the
  // confirmation or the drawer: a second modal window over the first would hide its question under the views. F3 is free
  // in `gui/src` (P-11 of the plan); its default, the browser's "find next", is not ours to keep.
  if (event.key === "F3") {
    event.preventDefault();
    if (!invoke.asking && !drawer.open) overview.value = !overview.value;
    return;
  }
  // G20, move 6 of SP-8: the active tile moves in the four directions from the keyboard. The
  // mapping lives in `moveActive.ts` and the geometry in `nearest.ts`; this is only the wire.
  const direction = directionOf(event);
  if (direction === null || dock === null) return;
  event.preventDefault();
  moveActive(dock.api, direction);
}

onMounted(() => {
  if (host.value !== null) dock = createDock(host.value);
  window.addEventListener("keydown", onKey);
});

// ⛔ THE DOCK GOES WITH THE FRAME (AUD-2116 of the audit of 2026-09-30): its listeners on `window` and its panels' Vue apps
// are its own, and `dispose` takes them away.
onUnmounted(() => {
  window.removeEventListener("keydown", onKey);
  dock?.dispose();
  dock = null;
});

/** The layout on screen, for «Salva questa vista»: the dock's own serialisation -- what `settle` saves. */
function snapshot(): SerializedDockview {
  if (dock === null) throw new Error("the dock is not mounted");
  return dock.api.toJSON();
}
</script>

<template>
  <div class="frame">
    <ViewBar v-model:overview="overview" :snapshot="snapshot" />
    <Band />
    <Confirm />
    <Drawer />
    <div ref="host" class="dock"></div>
  </div>
</template>

<style scoped>
.frame {
  display: flex;
  flex-direction: column;
  height: 100%;
}
/* Answer 20 of the design system: the dock -- the strip is its last row -- 12 px from the sides and 24 from the bottom,
   away from the window's corners; the bar above has none. A MARGIN and not a padding, so `clientWidth` is the room the
   grid gets (P-7 of its plan). */
.dock {
  flex: 1;
  min-height: 0;
  margin: 0 var(--space-3) var(--space-6);
  /* ⛔ THE DOCK KEEPS ITS SPILL TO ITSELF (E43): `dockview` 8.3.1 resizes one frame late -- its ResizeObserver hands the
     new size to a `requestAnimationFrame` -- so for a frame after the band comes in, the grid is as tall as before and
     spills out of this box; unclipped, the spill reached the page and flashed both its scrollbars. `clip` and not
     `hidden`: nothing may scroll this box either. */
  overflow: clip;
}
</style>

<script setup lang="ts">
import { h } from "vue";

import { ICONS, type IconName } from "./icons";

/**
 * The one door of the program's icons (answer 11 of the design system): a Lucide drawing of `icons.ts`, drawn with
 * `h()` and never with `v-html`. ⛔ DECORATIVE BY CONSTRUCTION -- `aria-hidden` -- because the name belongs to the
 * control around it: a button that is an icon alone carries `label` (BaseButton).
 */
const props = withDefaults(defineProps<{ name: IconName; size?: "sm" | "md" | "lg" }>(), { size: "md" });

function Drawing() {
  return h(
    "svg",
    {
      class: "base-icon",
      "data-icon": props.name,
      "data-size": props.size,
      xmlns: "http://www.w3.org/2000/svg",
      viewBox: "0 0 24 24",
      fill: "none",
      stroke: "currentColor",
      "stroke-linecap": "round",
      "stroke-linejoin": "round",
      "aria-hidden": "true",
      focusable: "false",
    },
    ICONS[props.name].map(([tag, attributes]) => h(tag, attributes)),
  );
}
</script>

<template>
  <Drawing />
</template>

<style scoped>
.base-icon {
  flex: none;
  stroke-width: var(--icon-stroke);
}
.base-icon[data-size="sm"] {
  width: var(--size-icon-sm);
  height: var(--size-icon-sm);
}
.base-icon[data-size="md"] {
  width: var(--size-icon-md);
  height: var(--size-icon-md);
}
.base-icon[data-size="lg"] {
  width: var(--size-icon-lg);
  height: var(--size-icon-lg);
}
</style>

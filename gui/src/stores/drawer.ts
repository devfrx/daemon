import { defineStore } from "pinia";
import { ref } from "vue";

/**
 * Whether the drawer of the modules is open (the (d) of the design system). ⛔ IN A STORE AND NOT IN A `ref` OF
 * `Drawer.vue` (R3-20 of the design-system review): the button that opens it sits in the strip, and the strip is a
 * panel -- a Vue app of its own (`frame/VueContent.ts`), which reaches the stores of the page and not the refs of
 * another app.
 */
export const useDrawer = defineStore("drawer", () => {
  const open = ref(false);
  return { open };
});

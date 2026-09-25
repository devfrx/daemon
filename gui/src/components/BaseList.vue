<script setup lang="ts" generic="T">
/**
 * The list of the kit (design system, section (b)): the rows of the drawer, of Permessi and of Passi. Static rows --
 * no hover, because nothing in them is clickable -- in the board's `.row` shape; the words come in the `item` slot.
 */
withDefaults(defineProps<{ items: readonly T[]; keyOf: (item: T, index: number) => string | number; ordered?: boolean }>(), {
  ordered: false,
});
defineSlots<{ item(props: { item: T; index: number }): unknown }>();
</script>

<template>
  <component :is="ordered ? 'ol' : 'ul'" class="base-list">
    <li v-for="(item, index) in items" :key="keyOf(item, index)" class="base-list-row">
      <slot name="item" :item="item" :index="index" />
    </li>
  </component>
</template>

<style scoped>
.base-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-0-5);
  margin: 0;
  padding: 0;
  list-style: none;
}
.base-list-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--space-2);
  min-height: var(--size-control-md);
  padding: 0 var(--space-3);
  border-radius: var(--radius-control);
  color: var(--color-text);
}
</style>

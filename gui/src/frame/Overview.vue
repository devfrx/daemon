<script setup lang="ts">
import type { SerializedDockview } from "dockview-core";
import { computed, nextTick, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import BaseButton from "../components/BaseButton.vue";
import BaseDialog from "../components/BaseDialog.vue";
import BaseIcon from "../components/BaseIcon.vue";
import BaseLabel from "../components/BaseLabel.vue";
import BaseTextField from "../components/BaseTextField.vue";
import { isIconName, type IconName } from "../components/icons";
import { VIEWS } from "../panels/views";
import { useLayout, type ViewName } from "../stores/layout";

import { nearest, type Direction } from "./nearest";
import { schematic } from "./schematic";

/**
 * THE OVERVIEW OF THE VIEWS (the (d) of the design system, answer 13): every view in miniature, in a grid -- Home, Lavoro,
 * Compatta and the views saved under a name -- the one on screen in bordeaux, and last «Salva questa vista». It opens from
 * the view's name in the bar, which this file draws as its trigger, or with F3 (`Frame.vue`).
 *
 * ⛔ ON `BaseDialog`, THE WHOLE WINDOW (decision 18): `reka-ui` gives Esc, the focus kept inside and given back to the
 * view's name. ⛔ THE ARROWS FOLLOW THE GEOMETRY (decision 19): `nearest`, the helper of the tiles moved with the keyboard,
 * in the four directions -- `RovingFocusGroup` is linear, and "down" would go right. ⛔ ONE CARD IN THE TAB ORDER, the one
 * the arrows reached (a roving tabindex): Tab leaves the grid instead of walking every card.
 */
const props = defineProps<{ snapshot: () => SerializedDockview }>();
const open = defineModel<boolean>("open", { required: true });

const { t } = useI18n();
const layout = useLayout();
const THREE: readonly ViewName[] = ["home", "work", "compact"];

/** A tile of a miniature: where it sits, in percent of the miniature, and the icon of the module it shows. */
interface Drawn {
  style: Record<string, string>;
  icon?: IconName;
}

interface Card {
  key: string;
  name: string;
  tiles: Drawn[];
  saved: boolean;
  current: boolean;
  show: () => void;
}

/** The miniature of a layout (answer 19): the schema of `schematic`, with a gap of `--space-0-5` around every tile.
 * ⛔ THE STRIP IS NOT DRAWN (D14 of the plan): it is in every view, the same, with no icon -- and the board's miniatures
 * leave it out. A module type that is gone is drawn without an icon: it is there, and the miniature says so.
 * ⛔ A LAYOUT IT CANNOT READ IS DRAWN EMPTY (E90 of the plan): `unpack` keeps any object as a layout, and `schematic`
 * throws on one it cannot read -- the cards draw every view at once, and one would take the whole overview down. The dock
 * opens the next layout in its place (`apply`, AUD-536 of the audit of 2026-09-30). */
function drawn(saved: SerializedDockview): Drawn[] {
  let tiles: ReturnType<typeof schematic>;
  try {
    tiles = schematic(saved);
  } catch {
    return [];
  }
  return tiles
    .filter((tile) => !tile.views.includes("strip"))
    .map((tile) => {
      const style = {
        left: `calc(${tile.x * 100}% + var(--space-0-5))`,
        top: `calc(${tile.y * 100}% + var(--space-0-5))`,
        width: `calc(${tile.width * 100}% - 2 * var(--space-0-5))`,
        height: `calc(${tile.height * 100}% - 2 * var(--space-0-5))`,
      };
      const shown = tile.active ?? tile.views[0] ?? "";
      return isIconName(shown) ? { style, icon: shown } : { style };
    });
}

// ⛔ A SAVED VIEW WINS OVER THE SHIPPED ONE BY NAME (row 6 of §2 of the north star), in the miniature as on screen.
const cards = computed<Card[]>(() => [
  ...THREE.map((view) => ({
    key: view,
    name: t(`views.${view}`),
    tiles: drawn(layout.saved?.layouts[view] ?? VIEWS[view]),
    saved: false,
    current: layout.openNamed === null && layout.view === view,
    // ⛔ ONE LINE, AND THE DOCK FOLLOWS (D89 of part 2): the store is where the open view lives, and `createDock` watches it.
    // Showing is not saving (decision 11).
    show: () => layout.showView(view),
  })),
  ...(layout.saved?.named ?? []).map((entry) => ({
    key: `named:${entry.name}`,
    name: entry.name,
    tiles: drawn(entry.layout),
    saved: true,
    current: layout.openNamed === entry.name,
    show: () => {
      layout.showNamed(entry.name);
    },
  })),
]);

/** The name of the view on screen: the owner's words for a named view, the locale's for the three. */
const current = computed(() => layout.openNamed ?? t(`views.${layout.view}`));

const grid = ref<HTMLElement | null>(null);
const roving = ref(0);
const naming = ref(false);
const name = ref("");
const refusal = ref<"empty" | "taken" | null>(null);

// Every opening starts from the view on screen, with no name half written.
watch(open, (now) => {
  if (!now) return;
  naming.value = false;
  name.value = "";
  refusal.value = null;
  roving.value = Math.max(0, cards.value.findIndex((card) => card.current));
});

function choose(card: Card): void {
  card.show();
  open.value = false;
}

const ARROWS: Readonly<Record<string, Direction>> = { ArrowLeft: "left", ArrowRight: "right", ArrowUp: "up", ArrowDown: "down" };

/** ⛔ ONLY FROM A CARD: in the name's field the arrows move the caret. Nothing beyond, nothing moves. */
function onArrow(event: KeyboardEvent): void {
  const direction = ARROWS[event.key];
  const from = event.target;
  if (direction === undefined || !(from instanceof HTMLElement) || !from.hasAttribute("data-card")) return;
  event.preventDefault();
  const others = [...(grid.value?.querySelectorAll<HTMLElement>("[data-card]") ?? [])].filter((card) => card !== from);
  const target = nearest(from.getBoundingClientRect(), others.map((element) => ({ element, rect: element.getBoundingClientRect() })), direction);
  target?.element.focus();
}

function startNaming(): void {
  naming.value = true;
  refusal.value = null;
  void nextTick(() => grid.value?.querySelector<HTMLInputElement>(".naming input")?.focus());
}

function cancel(): void {
  naming.value = false;
  refusal.value = null;
  roving.value = cards.value.length;
  void nextTick(() => grid.value?.querySelector<HTMLElement>('[data-card="save"]')?.focus());
}

/** «Salva questa vista»: the layout on screen under a name, and opened. ⛔ THE NAMES OF THE THREE ARE THE FRAME'S WORDS
 * (D12 of the plan): the store reads no locale, so it is handed them. ⛔ OFF UNTIL THE CORE HAS ANSWERED (E81): before
 * it the store holds no package of the core's, and a view saved then would send one without the core's layouts -- every
 * saved layout lost in silence, the window E40 closed for the theme. */
function save(): void {
  const result = layout.saveNamed(name.value, props.snapshot(), THREE.map((view) => t(`views.${view}`)));
  if (result === "saved") {
    open.value = false;
    return;
  }
  refusal.value = result;
}
</script>

<template>
  <BaseDialog v-model:open="open" :title="$t('overview.title')" :description="$t('overview.hint')" variant="full">
    <template #trigger>
      <BaseButton class="view-name" aria-keyshortcuts="F3">
        <BaseLabel icon="views">{{ current }}</BaseLabel>
      </BaseButton>
    </template>
    <div ref="grid" class="grid" @keydown="onArrow">
      <BaseButton
        v-for="(card, index) in cards"
        :key="card.key"
        variant="card"
        data-card="view"
        :tabindex="index === roving ? 0 : -1"
        :aria-current="card.current ? 'page' : undefined"
        @focus="roving = index"
        @click="choose(card)"
      >
        <span class="mini">
          <span v-for="(tile, at) in card.tiles" :key="at" class="tile" :style="tile.style">
            <BaseIcon v-if="tile.icon !== undefined" :name="tile.icon" size="sm" />
          </span>
        </span>
        <span class="caption">
          <BaseLabel>{{ card.name }}</BaseLabel>
          <span v-if="card.saved" class="saved">{{ $t("overview.saved") }}</span>
        </span>
      </BaseButton>
      <form v-if="naming" class="naming" @submit.prevent="save">
        <BaseTextField
          v-model="name"
          icon="saveView"
          :label="$t('overview.name')"
          :placeholder="$t('overview.name')"
          :error="refusal === 'empty' ? $t('overview.empty') : refusal === 'taken' ? $t('overview.taken') : undefined"
        />
        <span class="actions">
          <BaseButton variant="quiet" @click="cancel">{{ $t("overview.cancel") }}</BaseButton>
          <BaseButton variant="primary" @click="save">{{ $t("overview.confirm") }}</BaseButton>
        </span>
      </form>
      <BaseButton
        v-else
        variant="card"
        data-card="save"
        :disabled="layout.arrivals === 0"
        :tabindex="roving === cards.length ? 0 : -1"
        @focus="roving = cards.length"
        @click="startNaming"
      >
        <BaseLabel icon="saveView">{{ $t("overview.save") }}</BaseLabel>
      </BaseButton>
    </div>
  </BaseDialog>
</template>

<style scoped>
/* The view's name in the bar: the words in full, the icon in the mark's colour (the board's `.vt`). */
.view-name :deep(.base-label) {
  color: var(--color-text);
}
.grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: var(--space-3);
  margin-top: var(--space-4);
}
/* ⛔ THE RADII ARE CONCENTRIC (answer 4): the miniature sits in a card of `--radius-card` at `--space-3` from its edge, so it
   takes `--radius-control`; a tile sits `--space-0-5` inside the miniature's border, so it takes what is left. */
.mini {
  position: relative;
  display: block;
  aspect-ratio: 16 / 10;
  border: var(--border-width) solid var(--color-border);
  border-radius: var(--radius-control);
  background: var(--color-bg);
}
.tile {
  position: absolute;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: calc(var(--radius-control) - var(--space-0-5) - var(--border-width));
  background: var(--color-bg-surface);
  color: var(--color-text-muted);
}
.caption {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--space-2);
}
.saved {
  font: var(--font-caption);
  color: var(--color-text-muted);
}
/* The name of a new view, where «Salva questa vista» was: a card that is not a button. */
.naming {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  padding: var(--space-3);
  border: var(--border-width) solid var(--color-border-card);
  border-radius: var(--radius-card);
  background: var(--color-bg-surface);
  box-shadow: var(--shadow-card);
}
.actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-2);
}
</style>

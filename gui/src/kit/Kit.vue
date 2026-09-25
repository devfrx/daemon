<script setup lang="ts">
import { onUnmounted, ref } from "vue";

import BaseButton from "../components/BaseButton.vue";
import BaseDialog from "../components/BaseDialog.vue";
import BaseIcon from "../components/BaseIcon.vue";
import BaseLabel from "../components/BaseLabel.vue";
import BaseList from "../components/BaseList.vue";
import BaseRadioGroup from "../components/BaseRadioGroup.vue";
import BaseStatus from "../components/BaseStatus.vue";
import BaseTextField from "../components/BaseTextField.vue";
import { ICONS, type IconName } from "../components/icons";
import { isThemeChoice, watchTheme, type ThemeChoice } from "../tokens/theme";

/**
 * The kit page (design system, section (b); answer 12): every piece in the states it can be shown in, at full size, in
 * the theme chosen at the top -- ONE THEME AT A TIME (D7 of the plan), because a dialog goes to a portal on `body` and
 * would take the root's theme, not a column's. ⛔ The words and the values are SPECIMENS (D8).
 *
 * ⛔ THE RADIUS RULE SHAPES THIS PAGE TOO: the card buttons sit in a frame of `--radius-frame`, the pill button in a
 * pill, and no card ends with a small round control in its corner. The grid does NOT stretch the cards, so what closes
 * one -- a note, or the list's last row -- sits in its corner, where the radius probe judges it (R3-1 of the review).
 */
const props = withDefaults(defineProps<{ initialTheme?: ThemeChoice }>(), { initialTheme: "system" });
const theme = ref<ThemeChoice>(props.initialTheme);
onUnmounted(watchTheme(() => theme.value));

function chooseTheme(value: string): void {
  if (isThemeChoice(value)) theme.value = value;
}

const themes = [
  { value: "system", label: "Sistema" },
  { value: "light", label: "Chiaro" },
  { value: "dark", label: "Scuro" },
];
const variants = ["primary", "secondary", "quiet"] as const;
const sizes = ["sm", "md", "lg"] as const;
const icons = Object.keys(ICONS) as IconName[];
const text = ref("");
const policies = [
  { value: "remote", label: "OpenRouter, VRAM libera" },
  { value: "local", label: "Locale" },
];
const policy = ref<string | null>("remote");
const rows = [
  { what: "policy · VRAM · cambia", when: "14:02" },
  { what: "file · ~/note · leggi", when: "13:58" },
  { what: "rete · openrouter · usa", when: "13:41" },
];
</script>

<template>
  <main class="kit">
    <header class="kit-head">
      <h1>Il kit</h1>
      <BaseRadioGroup :model-value="theme" :options="themes" legend="Tema" @update:model-value="chooseTheme" />
    </header>

    <div class="kit-grid">
      <section class="kit-card">
        <BaseLabel icon="modules" as="h2">Pulsanti</BaseLabel>
        <div v-for="variant in variants" :key="variant" class="kit-row">
          <BaseButton v-for="size in sizes" :key="size" :variant="variant" :size="size">Consenti</BaseButton>
          <BaseButton :variant="variant" disabled>Spento</BaseButton>
          <BaseButton :variant="variant" icon="float" label="Stacca la tessera" />
        </div>
        <div class="kit-row">
          <BaseButton icon="search">Cerca</BaseButton>
          <BaseButton size="lg" icon="fullPage" label="A pagina intera" />
        </div>
        <p class="kit-note">Principale, normale, discreto; tre misure; spento; solo un'icona, col nome per chi non vede.</p>
      </section>

      <section class="kit-card">
        <BaseLabel icon="status" as="h2">Etichette</BaseLabel>
        <BaseLabel icon="status">Stato</BaseLabel>
        <BaseLabel icon="permissions">Permessi</BaseLabel>
        <BaseLabel icon="steps">Passi</BaseLabel>
        <p class="kit-note">Maiuscolo spaziato, con l'icona nel colore del segno.</p>
      </section>

      <section class="kit-card">
        <BaseLabel icon="search" as="h2">Campo</BaseLabel>
        <BaseTextField v-model="text" label="Cerca negli artefatti" icon="search" placeholder="Cerca negli artefatti" />
        <BaseTextField model-value="" label="Cerca" icon="search" placeholder="la ricerca arriva col sotto-progetto 6" disabled />
        <BaseTextField model-value="Home" label="Nome della vista" error="Esiste già una vista con questo nome." />
        <p class="kit-note">Normale, spento, con un errore sotto.</p>
      </section>

      <section class="kit-card">
        <BaseLabel icon="settings" as="h2">Scelta</BaseLabel>
        <BaseRadioGroup :model-value="policy" :options="policies" legend="Policy VRAM" @update:model-value="policy = $event" />
        <BaseRadioGroup :model-value="null" :options="policies" legend="Spenta, finché il core non parla" disabled />
        <p class="kit-note">Il radio mostra il valore che riceve: si muove quando il valore cambia.</p>
      </section>

      <section class="kit-card">
        <BaseLabel icon="steps" as="h2">Lista</BaseLabel>
        <BaseList :items="rows" :key-of="(row) => row.when">
          <template #item="{ item }">
            <span>{{ item.what }}</span>
            <em>{{ item.when }}</em>
          </template>
        </BaseList>
      </section>

      <section class="kit-card">
        <BaseLabel icon="status" as="h2">Stato</BaseLabel>
        <BaseStatus><p>Richiesta inviata: in attesa del core.</p></BaseStatus>
        <p class="kit-note">La regione c'è sempre, anche vuota: le parole ci entrano.</p>
      </section>

      <section class="kit-card">
        <BaseLabel icon="float" as="h2">Finestra</BaseLabel>
        <BaseDialog title="Serve un permesso" description="Vale per questa tripla e per questa sessione.">
          <template #trigger>
            <BaseButton data-kit="open-dialog">Apri la finestra</BaseButton>
          </template>
          <template #actions>
            <BaseButton variant="quiet">Rifiuta</BaseButton>
            <BaseButton variant="primary">Consenti</BaseButton>
          </template>
        </BaseDialog>
        <p class="kit-note">Esc chiude, il fuoco resta dentro e torna al pulsante.</p>
      </section>

      <section class="kit-section kit-wide">
        <BaseLabel icon="views" as="h2">Carte</BaseLabel>
        <div class="kit-frame">
          <BaseButton variant="card" aria-current="page">
            <BaseLabel icon="views">Home</BaseLabel>
            <span>La vista di adesso, col segno tutto intorno.</span>
          </BaseButton>
          <BaseButton variant="card">
            <BaseLabel icon="saveView">Salva questa vista</BaseLabel>
            <span>Una carta intera che è un pulsante.</span>
          </BaseButton>
        </div>
      </section>

      <section class="kit-section kit-wide">
        <BaseLabel icon="modules" as="h2">Pillola</BaseLabel>
        <div class="kit-strip">
          <span>Degrado nessuno</span>
          <BaseButton size="lg" pill icon="modules">moduli</BaseButton>
        </div>
      </section>

      <section class="kit-section kit-wide">
        <BaseLabel icon="views" as="h2">Icone</BaseLabel>
        <div class="kit-icons">
          <span v-for="name in icons" :key="name" class="kit-icon"><BaseIcon :name="name" /><code>{{ name }}</code></span>
        </div>
      </section>
    </div>
  </main>
</template>

<!-- ⛔ NOT SCOPED, like `App.vue`'s (P-4 of the design-system plan): this page does not mount `App.vue`, so without this
     rule the browser's 8 px stay around it, and `min-height: 100vh` scrolls 16 px for nothing (R3-11 of the review). -->
<style>
body {
  margin: 0;
}
</style>

<style scoped>
.kit {
  min-height: 100vh;
  box-sizing: border-box;
  padding: var(--space-8) var(--space-6);
  background: var(--color-bg);
  color: var(--color-text);
  font: var(--font-body);
}
.kit-head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-6);
  margin-bottom: var(--space-6);
}
h1 {
  margin: 0;
  font: var(--font-heading);
}
.kit-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(22rem, 1fr));
  gap: var(--space-6);
  /* ⛔ NOT STRETCHED: a card stretched to the tallest of its row moves what closes it away from its corner, and the
     radius probe would judge nothing there (R3-1). */
  align-items: start;
}
.kit-card {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  padding: var(--space-3);
  border: var(--border-width) solid var(--color-border-card);
  border-radius: var(--radius-card);
  background: var(--color-bg-surface);
  box-shadow: var(--shadow-card);
}
.kit-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}
.kit-wide {
  grid-column: 1 / -1;
}
/* A frame of 32 with 12 of margin around cards of 20: answer 4, and the `calc` of the tokens. */
.kit-frame {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: var(--space-3);
  padding: var(--space-3);
  border-radius: var(--radius-frame);
  background: var(--color-bg-fill);
}
/* The strip's shape: a pill with 8 of margin around a pill button of 40 -- inside a pill goes a pill. */
.kit-strip {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-4);
  padding: var(--space-2) var(--space-2) var(--space-2) var(--space-4);
  border: var(--border-width) solid var(--color-border-card);
  border-radius: var(--radius-full);
  background: var(--color-bg-surface);
  box-shadow: var(--shadow-card);
}
.kit-row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--space-2);
}
.kit-icons {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-3) var(--space-6);
}
.kit-icon {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}
.kit-note {
  margin: 0;
  font: var(--font-caption);
  color: var(--color-text-muted);
}
code {
  font: var(--font-mono);
  color: var(--color-text-muted);
}
em {
  font: var(--font-numeric);
  font-style: normal;
  font-variant-numeric: tabular-nums;
  color: var(--color-text-muted);
}
</style>

<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";

import BaseRadioGroup from "../components/BaseRadioGroup.vue";
import BaseStatus from "../components/BaseStatus.vue";
import { useCore } from "../stores/core";
import { useInvoke } from "../stores/invoke";
import { useLayout } from "../stores/layout";
import { THEME_CHOICES, isThemeChoice } from "../tokens/theme";

import { VRAM_POLICY, type PolicyArgument } from "./functions";

// The Impostazioni row of the short table of §1: in sub-project 2 the VRAM policy change, a registry function with its triple
// (ADR-0038) -- the FIRST INVOKER of the registry, the click; and since the design system the theme (answer 5).
const { t } = useI18n();
const core = useCore();
const invoke = useInvoke();
const layout = useLayout();

const current = computed<PolicyArgument | null>(() =>
  core.policy === null ? null
  : core.policy.policy === "Local" ? VRAM_POLICY.argument.local
  : VRAM_POLICY.argument.remote,
);
const policies = computed(() => [
  { value: VRAM_POLICY.argument.remote, label: t("settings.remote") },
  { value: VRAM_POLICY.argument.local, label: t("settings.local") },
]);
const themes = computed(() => THEME_CHOICES.map((choice) => ({ value: choice, label: t(`settings.theme.${choice}`) })));

/**
 * ⛔ THE CONTROL SHOWS THE CORE'S POLICY, NOT THE LAST CLICK (I1): a choice sends `Invoke`, and the radio moves when
 * `Policy` comes back -- after the confirmation window, if the triple is not yet granted; meanwhile the line below says a
 * call is in flight. Since the design system the group is CONTROLLED (P-8 of its plan): it shows `current` and only asks,
 * so the hand re-sync E184 needed on native radios went away with them, and an arrow key asks the way a click does.
 * ⚠️ M-2 of E187 stays true, and declared: after an arrow key the focus sits on the radio the arrow reached, while the
 * check stays on the core's value.
 */
function choosePolicy(argument: string): void {
  if (argument !== current.value) invoke.send({ function: VRAM_POLICY.name, argument });
}

/** The theme is the SPA's and not the core's (answer 16): saved at once, in the layout package. */
function chooseTheme(choice: string): void {
  if (isThemeChoice(choice)) layout.chooseTheme(choice);
}
</script>

<template>
  <section class="settings">
    <!-- Off while the core has not said which policy is active: "the rest off" (§6a). -->
    <BaseRadioGroup
      :model-value="current"
      :options="policies"
      :legend="$t('settings.policy')"
      :disabled="core.policy === null"
      @update:model-value="choosePolicy"
    />
    <BaseStatus>
      <p v-if="invoke.inFlight !== null">{{ $t("settings.inFlight") }}</p>
    </BaseStatus>
    <BaseRadioGroup :model-value="layout.theme" :options="themes" :legend="$t('settings.themeTitle')" @update:model-value="chooseTheme" />
    <p class="who">{{ $t("settings.who") }}</p>
  </section>
</template>

<style scoped>
.settings {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  padding: var(--space-3);
  overflow: auto;
  height: 100%;
  box-sizing: border-box;
}
p {
  margin: 0;
}
.who {
  color: var(--color-text-muted);
}
</style>

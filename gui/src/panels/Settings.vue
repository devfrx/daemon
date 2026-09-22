<script setup lang="ts">
import { computed, ref } from "vue";

import { useCore } from "../stores/core";
import { useInvoke } from "../stores/invoke";

import { VRAM_POLICY, type PolicyArgument } from "./functions";

// The Impostazioni row of the short table of §1: in sub-project 2 the VRAM policy change, a registry
// function with its triple (ADR-0038). This is the FIRST INVOKER of the registry -- the click.
const core = useCore();
const invoke = useInvoke();
const group = ref<HTMLFieldSetElement | null>(null);

const current = computed<PolicyArgument | null>(() =>
  core.policy === null ? null
  : core.policy.policy === "Local" ? VRAM_POLICY.argument.local
  : VRAM_POLICY.argument.remote,
);

/**
 * ⛔ THE CONTROL SHOWS THE CORE'S POLICY, NOT THE LAST CLICK (I1): a click sends `Invoke`, and the
 * radio moves when `Policy` comes back -- after the confirmation window, if the triple is not yet
 * granted. Meanwhile the panel says a call is in flight, so nobody is left wondering.
 *
 * ⛔ AND THE CONTROL IS PUT BACK BY HAND, BECAUSE `v-model` DOES NOT DO IT -- E184, measured in the
 * browser on 2026-09-22 and then read in the shipped `@vue/runtime-dom` of vue 3.5.42:
 * `vModelRadio.beforeUpdate` re-syncs `el.checked` only `if (value !== oldValue)`, and here the
 * bound value does NOT change -- the model still says what the core said -- so the browser's own
 * check survives and the radio shows the click. `:checked` alone has the same hole: a DOM prop is
 * patched only when the vnode's value differs from the previous one.
 *
 * ⚠️ `change` AND NOT `click.prevent`: the arrow keys move a native radio group and fire `change`,
 * and a handler hung on `click` would leave the keyboard path silent (G20).
 */
function choose(argument: PolicyArgument): void {
  if (argument !== current.value) invoke.send({ function: VRAM_POLICY.name, argument });
  for (const input of group.value?.querySelectorAll<HTMLInputElement>("input[type=radio]") ?? []) {
    input.checked = input.value === current.value;
  }
}
</script>

<template>
  <section class="settings">
    <!-- A NATIVE RADIO GROUP AND NOT A LIBRARY PRIMITIVE (D62): the browser gives it the keyboard
         and the roles; Reka UI is for what HTML has no primitive for -- the focus trap of the
         window. Disabled while the core has not said which policy is active: "the rest off" (§6a). -->
    <fieldset ref="group" :disabled="core.policy === null">
      <legend>{{ $t("settings.policy") }}</legend>
      <label><input type="radio" name="vram-policy" :value="VRAM_POLICY.argument.remote" :checked="current === VRAM_POLICY.argument.remote" @change="choose(VRAM_POLICY.argument.remote)" /> {{ $t("settings.remote") }}</label>
      <label><input type="radio" name="vram-policy" :value="VRAM_POLICY.argument.local" :checked="current === VRAM_POLICY.argument.local" @change="choose(VRAM_POLICY.argument.local)" /> {{ $t("settings.local") }}</label>
    </fieldset>
    <p v-if="invoke.inFlight !== null" role="status">{{ $t("settings.inFlight") }}</p>
    <p class="who">{{ $t("settings.who") }}</p>
  </section>
</template>

<style scoped>
.settings {
  padding: var(--space-3);
  overflow: auto;
  height: 100%;
  box-sizing: border-box;
}
fieldset {
  border: 1px solid var(--line);
  border-radius: var(--radius);
}
label {
  display: block;
  padding: var(--space-1) 0;
}
.who {
  color: var(--ink-dim);
  margin-top: var(--space-3);
}
</style>

<script setup lang="ts">
import { handStatus } from '../hand';

defineProps<{ title?: string; api?: unknown; containerApi?: unknown; params?: unknown }>();
</script>

<template>
  <div>
    <p class="tile-who">il 12 — la mano di SP-7 come puntatore (mossa 8)</p>
    <p>la mano si disegna sopra tutta la pagina; la pinza afferra ciò che sta sotto la punta dell'indice</p>
    <p>stato: {{ handStatus }}</p>
  </div>
</template>

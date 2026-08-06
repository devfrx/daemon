import assert from "node:assert/strict";
import test from "node:test";
import { World } from "../src/sched.js";

function traccia(seed: number): string[] {
  const w = new World(seed);
  w.spawn("alfa", 5);
  w.spawn("beta", 5);
  return w.run();
}

test("C1 stesso seed, stessa traccia", () => {
  assert.deepEqual(traccia(42), traccia(42));
});

test("C2 seed diversi, tracce diverse", () => {
  assert.notDeepEqual(traccia(42), traccia(43));
});

test("C3 il tempo è virtuale", () => {
  const inizio = process.hrtime.bigint();
  const w = new World(7);
  w.spawn("lento", 5);
  w.sleepVirtuale(5000);
  w.run();
  assert.ok(w.now() >= 5000, "orologio virtuale non avanzato");
  const msTrascorsi = Number(process.hrtime.bigint() - inizio) / 1e6;
  assert.ok(msTrascorsi < 1000, "C3 violato: il test ha atteso davvero");
});

// Seed 4, non 99. Il seed 99 non inietta guasti in TypeScript: l'RNG qui è a 32 bit
// (`>>> 0` a ogni passo) mentre in Rust e Go è a 64 bit, quindi la sequenza è
// diversa a parità di seed. Il piano lo prevedeva e chiedeva di registrarlo.
// Primi seed che iniettano un guasto, misurati: 1, 4, 6, 10, 11, 12.
const SEED_CON_GUASTO = 4;

test("C4 il guasto è riproducibile", () => {
  const a = traccia(SEED_CON_GUASTO);
  const b = traccia(SEED_CON_GUASTO);
  assert.ok(
    a.some((e) => e.includes("GUASTO")),
    `il seed ${SEED_CON_GUASTO} deve iniettare almeno un guasto`,
  );
  assert.deepEqual(a, b, "C4 violato: guasto non riproducibile");
});

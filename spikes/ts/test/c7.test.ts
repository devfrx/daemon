// C7 — I/O durevole iniettabile, crash riproducibile, passo in dubbio rilevabile.

import assert from "node:assert/strict";
import test from "node:test";
import {
  esegui,
  GiornaleCadente,
  passoInDubbio,
  type Giornale,
} from "../src/giornale.js";

const PASSI = 8;
const SCRITTURE_PREVISTE = PASSI * 2;

const esecuzione = (seed: number): string[] => {
  const g = GiornaleCadente.conSeed(seed, SCRITTURE_PREVISTE);
  esegui(g, PASSI);
  return g.righe;
};

test("C7 stesso seed, stessa traccia, crash incluso", () => {
  for (const seed of [1, 7, 42, 99, 20260806]) {
    assert.deepEqual(
      esecuzione(seed),
      esecuzione(seed),
      `C7 violato: crash non riproducibile con seed ${seed}`,
    );
  }
});

test("C7 il crash avviene davvero", () => {
  const cadute = Array.from({ length: 50 }, (_, s) => esecuzione(s)).filter(
    (r) => r.length < SCRITTURE_PREVISTE,
  ).length;
  assert.ok(cadute > 0, "C7 vacuo: nessuno dei 50 seed ha prodotto una caduta");
});

test("C7 il passo in dubbio è rilevabile", () => {
  let seedConDubbio: number | undefined;
  for (let s = 0; s < 200; s++) {
    if (passoInDubbio(esecuzione(s)) !== undefined) {
      seedConDubbio = s;
      break;
    }
  }
  assert.ok(seedConDubbio !== undefined, "almeno un seed su 200 deve cadere fra intento ed esito");

  const righe = esecuzione(seedConDubbio);
  const passo = passoInDubbio(righe)!;
  assert.ok(
    righe.some((r) => r.startsWith(`passo=${passo} INTENTO`)),
    "il passo in dubbio deve avere un intento registrato",
  );
  assert.ok(
    !righe.some((r) => r.startsWith(`passo=${passo} ESITO`)),
    "il passo in dubbio non deve avere un esito",
  );
  console.log(`C7 — seed con passo in dubbio: ${seedConDubbio}, passo ${passo}`);
});

test("C7 senza crash nessun passo resta in dubbio", () => {
  const g = GiornaleCadente.senzaCrash();
  esegui(g, PASSI);
  assert.equal(g.righe.length, SCRITTURE_PREVISTE);
  assert.equal(
    passoInDubbio(g.righe),
    undefined,
    "senza crash il rilevatore darebbe falsi positivi",
  );
});

test("C7 l'ordine è write-ahead: intento prima dell'esito", () => {
  const g = GiornaleCadente.senzaCrash();
  esegui(g, 3);
  const ordine = g.righe.map((r) => (r.includes("INTENTO") ? "I" : "E"));
  assert.deepEqual(ordine, ["I", "E", "I", "E", "I", "E"]);
});

test("C7 il giornale è sostituibile senza toccare il codice sotto test", () => {
  // Se `esegui` conoscesse il filesystem questo non compilerebbe.
  let n = 0;
  const contatore: Giornale = {
    intento: () => { n += 1; },
    esito: () => { n += 1; },
  };
  esegui(contatore, PASSI);
  assert.equal(n, SCRITTURE_PREVISTE);
});

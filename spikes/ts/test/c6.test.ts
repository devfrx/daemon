// C6 — soglia del protocollo: 100 esecuzioni con lo stesso seed, tracce identiche.

import assert from "node:assert/strict";
import test from "node:test";
import { eseguiConAsync, eseguiConEsecutore } from "../src/concorrenza.js";

const ESECUZIONI = 100;
const TASK = 3;
const PASSI = 6;
const SEED = 20260806;

const distinte = (tracce: string[][]): number =>
  new Set(tracce.map((t) => t.join("|"))).size;

test("C6 (a) generatori guidati dal nostro esecutore: ordinabili dal seed", () => {
  const tracce = Array.from({ length: ESECUZIONI }, () =>
    eseguiConEsecutore(SEED, TASK, PASSI),
  );
  assert.equal(
    distinte(tracce),
    1,
    `C6(a) violato: ${ESECUZIONI} esecuzioni, ${distinte(tracce)} tracce distinte`,
  );

  // Non deve essere banale: senza interlacciamento il determinismo non prova nulla.
  const t = tracce[0]!;
  assert.equal(t.length, TASK * PASSI);
  const interlacciata = t.some(
    (v, i) => i > 0 && v.split(" ")[0] !== t[i - 1]!.split(" ")[0],
  );
  assert.ok(interlacciata, "C6 vacuo: i task non si sono interlacciati");
});

test("C6 (a) seed diversi producono interlacciamenti diversi", () => {
  assert.notDeepEqual(
    eseguiConEsecutore(SEED, TASK, PASSI),
    eseguiConEsecutore(SEED + 1, TASK, PASSI),
  );
});

// ESITO MISURATO IL 2026-08-06 SU node 24.9 / TypeScript 5.9.3.
//
// La via (b) — `async`/`await` sul ciclo di eventi — risulta deterministica, ma per
// una ragione che NON soddisfa C6: il ciclo di eventi è a thread singolo e le
// microtask si accodano in ordine di creazione. Non c'è un ordine *scelto*: c'è
// l'assenza di scelta. Il seed non entra da nessuna parte.
//
// È determinismo per mancanza di concorrenza, non per controllo della concorrenza:
// la distinzione è il motivo per cui C6 chiede unità *in contesa*, e per cui questa
// via non basta a far passare il criterio. Il vero parallelismo in Node richiede
// `worker_threads`, il cui ordinamento non è controllabile dall'applicazione.
test("C6 (b) async sul ciclo di eventi: deterministico, ma senza il seed", async () => {
  const tracce = await Promise.all(
    Array.from({ length: ESECUZIONI }, () => eseguiConAsync(TASK, PASSI)),
  );
  const n = distinte(tracce);
  console.log(`C6 (b) async puro: ${n} tracce distinte su ${ESECUZIONI}`);
  assert.equal(n, 1, "atteso determinismo per assenza di parallelismo");
});

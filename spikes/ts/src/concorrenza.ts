// C6 — il parallelismo **nativo** resta ordinabile dal seed?
//
// In JavaScript una `Promise` non è ispezionabile: non esiste un `poll` che
// permetta a un esecutore di decidere *quando* farla avanzare. Per riprendere il
// controllo bisogna rinunciare a `async/await` e usare i **generatori**, che sono
// l'unica primitiva del linguaggio in cui il punto di sospensione è restituito al
// chiamante. È il costo, e va misurato invece che raccontato.
//
// V29 · ADR-0021 · ADR-0004.

import { Rng } from "./sched.js";

/** Un'attività è un generatore: ogni `yield` è un punto in cui l'esecutore decide. */
export type Attivita = Generator<void, void, void>;

function* attivita(id: number, passi: number, traccia: string[]): Attivita {
  for (let p = 0; p < passi; p++) {
    traccia.push(`task${id} passo${p}`);
    yield;
  }
}

/**
 * Via 1 — generatori guidati da un esecutore che sceglie col seed.
 * L'ordine di acquisizione della risorsa condivisa è deciso qui, non dal runtime.
 */
export function eseguiConEsecutore(
  seed: number,
  nTask: number,
  passi: number,
): string[] {
  const traccia: string[] = [];
  const rng = new Rng(seed);
  const tasks: Attivita[] = Array.from({ length: nTask }, (_, i) =>
    attivita(i, passi, traccia),
  );

  while (tasks.length > 0) {
    const i = rng.below(tasks.length);
    if (tasks[i]!.next().done === true) {
      tasks.splice(i, 1);
    }
  }
  return traccia;
}

/**
 * Via 2 — funzioni `async` lasciate al ciclo di eventi, per controprova.
 * Il seed non entra: non c'è alcun punto in cui inserirlo.
 */
export async function eseguiConAsync(
  nTask: number,
  passi: number,
): Promise<string[]> {
  const traccia: string[] = [];
  const uno = async (id: number): Promise<void> => {
    for (let p = 0; p < passi; p++) {
      traccia.push(`task${id} passo${p}`);
      await Promise.resolve();
    }
  };
  await Promise.all(Array.from({ length: nTask }, (_, i) => uno(i)));
  return traccia;
}

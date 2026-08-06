// Il codice che NON deve toccare il sistema operativo (I3) né avere un percorso
// verso il gateway di inferenza (V28).
//
// La regola è applicata da tsconfig.kernel.json, che compila SOLO questa cartella
// con `"types": []` e nessuna libreria DOM: i moduli `node:*` non sono dichiarati,
// quindi importarli è un errore del **compilatore**, non di un lint.

export interface Lavoro {
  readonly priorita: number;
  readonly nome: string;
}

/**
 * `toSorted` è un ordinamento stabile: a parità di priorità l'ordine di
 * inserimento è conservato. Per V29 non è stile — un ordinamento instabile
 * introdurrebbe non determinismo in una coda del kernel.
 */
export const ordinaPerPriorita = (lavori: readonly Lavoro[]): readonly Lavoro[] =>
  lavori.toSorted((a, b) => a.priorita - b.priorita);

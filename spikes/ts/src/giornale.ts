// C7 — l'I/O durevole è iniettabile, e un crash al confine di persistenza è
// riproducibile dal seed. V29 · ADR-0007 · ADR-0021.

import { Rng } from "./sched.js";

/** Il processo è caduto al confine di persistenza. Non è un errore applicativo. */
export class Caduto extends Error {
  constructor() {
    super("caduto al confine di persistenza");
    this.name = "Caduto";
  }
}

/** Il confine sostituibile. Il codice sotto test non conosce altra via al durevole. */
export interface Giornale {
  intento(passo: number, descrizione: string): void;
  esito(passo: number, esito: string): void;
}

/** Doppio in memoria che cade a una scrittura scelta dal seed. */
export class GiornaleCadente implements Giornale {
  readonly righe: string[] = [];
  private scritture = 0;
  private readonly cadeAlla: number;

  private constructor(cadeAlla: number) {
    this.cadeAlla = cadeAlla;
  }

  static conSeed(seed: number, scrittureP: number): GiornaleCadente {
    return new GiornaleCadente(new Rng(seed).below(scrittureP));
  }

  /** Caso di controllo: senza, un test di riproducibilità sarebbe vacuo. */
  static senzaCrash(): GiornaleCadente {
    return new GiornaleCadente(Number.POSITIVE_INFINITY);
  }

  private scrivi(riga: string): void {
    if (this.scritture === this.cadeAlla) throw new Caduto();
    this.scritture += 1;
    this.righe.push(riga);
  }

  intento(passo: number, d: string): void {
    this.scrivi(`passo=${passo} INTENTO ${d}`);
  }
  esito(passo: number, e: string): void {
    this.scrivi(`passo=${passo} ESITO ${e}`);
  }
}

/** Scrive write-ahead: intento prima dell'effetto, esito dopo. Si ferma alla caduta. */
export function esegui(g: Giornale, passi: number): void {
  try {
    for (let p = 0; p < passi; p++) {
      g.intento(p, "chiamata a strumento");
      // qui, nel kernel reale, avviene l'effetto
      g.esito(p, "ok");
    }
  } catch (e) {
    if (!(e instanceof Caduto)) throw e;
  }
}

/** Riconciliazione: il passo con intento e senza esito. */
export function passoInDubbio(righe: readonly string[]): number | undefined {
  let aperto: number | undefined;
  for (const riga of righe) {
    const campi = riga.split(/\s+/);
    const n = Number(campi[0]?.replace("passo=", ""));
    if (!Number.isInteger(n)) continue;
    if (campi[1] === "INTENTO") aperto = n;
    else if (campi[1] === "ESITO" && aperto === n) aperto = undefined;
  }
  return aperto;
}

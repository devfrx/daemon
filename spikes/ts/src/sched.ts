// SP-5: tempo, casualità e ordinamento sono iniettabili (V29).
// Nessun Date.now(), nessun Math.random() (C5).

export class Rng {
  private s: number;
  constructor(seed: number) {
    const s = (seed * 1664525 + 1013904223) >>> 0;
    // xorshift resta bloccato su zero: la guardia evita un seed morto.
    this.s = s === 0 ? 1 : s;
  }
  next(): number {
    let x = this.s;
    x ^= x << 13;
    x >>>= 0;
    x ^= x >>> 7;
    x ^= x << 17;
    x >>>= 0;
    this.s = x;
    return x;
  }
  below(n: number): number {
    return this.next() % n;
  }
}

interface Task {
  nome: string;
  rimanenti: number;
}

export class World {
  private rng: Rng;
  private orologio = 0;
  private tasks: Task[] = [];
  private traccia: string[] = [];

  constructor(seed: number) {
    this.rng = new Rng(seed);
  }

  spawn(nome: string, steps: number): void {
    this.tasks.push({ nome, rimanenti: steps });
  }

  /** Avanza l'orologio virtuale senza attendere davvero (C3). */
  sleepVirtuale(ms: number): void {
    this.orologio += ms;
  }

  now(): number {
    return this.orologio;
  }

  run(): string[] {
    while (this.tasks.length > 0) {
      const i = this.rng.below(this.tasks.length);
      this.orologio += 1;

      const guasto = this.rng.below(20) === 0;
      const nome = this.tasks[i]!.nome;

      if (guasto) {
        this.traccia.push(`t=${this.orologio} ${nome} GUASTO`);
        this.tasks.splice(i, 1);
        continue;
      }

      this.traccia.push(`t=${this.orologio} ${nome} passo`);
      this.tasks[i]!.rimanenti -= 1;
      if (this.tasks[i]!.rimanenti === 0) {
        this.traccia.push(`t=${this.orologio} ${nome} fine`);
        this.tasks.splice(i, 1);
      }
    }
    return this.traccia;
  }
}

// SP-6: il confine dei dati non fidati vive nei tipi (V19, V20).
// TypeScript è strutturale: la nominalità si ottiene con un marchio.

declare const marchio: unique symbol;

export type Instruction = string & { readonly [marchio]: "instruction" };
export type Untrusted = string & { readonly [marchio]: "untrusted" };

export const instruction = (s: string): Instruction => s as Instruction;
export const untrusted = (s: string): Untrusted => s as Untrusted;

/** T2 — unico percorso di conversione. Nel kernel reale è giornalata. */
export const promoteToInstruction = (u: Untrusted, motivo: string): Instruction =>
  (u as string) as Instruction;

/** T3 — l'etichetta è ereditaria (V20). */
export const summarize = (u: Untrusted): Untrusted =>
  u.slice(0, 50) as Untrusted;

/** Il canale delle istruzioni accetta solo Instruction. */
export const buildPrompt = (system: Instruction, user: Instruction): string =>
  `${system}\n${user}`;

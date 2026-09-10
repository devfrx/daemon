import { stats } from './stats';

/** One line of the emitter, parsed: the shape of `Messaggio` in spikes/gui-ipc/src/lib.rs. */
export interface Wire {
  canale: 'Token' | 'Stato' | 'Metriche';
  seq: number;
  emesso_micros: number;
  carico: string;
}

export type Source = 'electron' | 'tauri' | 'browser';

declare global {
  interface Window {
    harness?: { onLine(cb: (line: string) => void): void; chrome?: string };
    __TAURI_INTERNALS__?: unknown;
  }
}

/** Every parsed message is dispatched here as a CustomEvent 'message' with the Wire in `detail`. */
export const wire = new EventTarget();

/** Where the lines come from -- the Electron preload, the Tauri event, or a local generator with the
 * cadence of spikes/gui-ipc (2000 messages in 10 s). The tiles never know which (decision D5). */
export async function subscribe(): Promise<Source> {
  const onLine = (line: string) => {
    let m: Wire;
    try {
      m = JSON.parse(line) as Wire;
    } catch {
      return; // the emitter writes JSON; a malformed line is nobody's message, and P1 counts holes, not garbage
    }
    stats.onWire(m);
    wire.dispatchEvent(new CustomEvent<Wire>('message', { detail: m }));
  };
  if (window.harness) {
    window.harness.onLine(onLine);
    return 'electron';
  }
  if ('__TAURI_INTERNALS__' in window) {
    const { listen } = await import('@tauri-apps/api/event');
    await listen<string>('line', (event) => onLine(event.payload));
    return 'tauri';
  }
  startGenerator(onLine);
  return 'browser';
}

/** spikes/gui-ipc/src/bin/core.rs, in JavaScript: 2000 messages in 10 s, then a pause, then again. */
function startGenerator(onLine: (line: string) => void): void {
  const TOTAL = 2000;
  const DURATION_MS = 10_000;
  const PAUSE_MS = 5_000;
  const period = DURATION_MS / TOTAL;
  let seq = 0;
  const tick = () => {
    const canale = seq % 10 === 0 ? 'Stato' : seq % 10 === 1 ? 'Metriche' : 'Token';
    onLine(
      JSON.stringify({
        canale,
        seq,
        emesso_micros: Math.round((performance.timeOrigin + performance.now()) * 1000),
        carico: 'lorem ipsum dolor sit amet',
      }),
    );
    seq += 1;
    setTimeout(tick, seq % TOTAL === 0 ? PAUSE_MS : period);
  };
  setTimeout(tick, period);
}

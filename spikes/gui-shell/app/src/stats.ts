import type { Wire } from './bridge';

/** What the page measures by itself, written into the window title once per second (decision D6):
 * fps and API from the scene tile, P1 (holes in the sequence) and P2 (emission -> reception) from
 * the wire. The measuring script reads the title from outside; no second channel exists. */
class Stats {
  msgs = 0;
  lost = 0;
  holes = 0;
  p2MaxMs = 0;
  p2SumMs = 0;
  fps = 0;
  fpsMin = 0;
  api = '-';
  scene = '-';
  source = '-';
  private lastSeq = -1;

  onWire(m: Wire): void {
    if (m.seq < this.lastSeq) this.lastSeq = -1; // a new run of the emitter starts at 0 again
    if (this.lastSeq >= 0 && m.seq > this.lastSeq + 1) {
      this.holes += 1;
      this.lost += m.seq - this.lastSeq - 1;
    }
    this.lastSeq = m.seq;
    this.msgs += 1;
    const delayMs = Date.now() - m.emesso_micros / 1000; // both are the wall clock of this machine
    if (delayMs > this.p2MaxMs) this.p2MaxMs = delayMs;
    this.p2SumMs += delayMs;
  }

  line(): string {
    const mean = this.msgs ? this.p2SumMs / this.msgs : 0;
    const dnd = localStorage.getItem('sp8-dnd') ?? 'auto';
    return (
      `SP-8 | fps=${this.fps.toFixed(0)} min=${this.fpsMin.toFixed(0)} api=${this.api} scene=${this.scene}` +
      ` | msgs=${this.msgs} lost=${this.lost} holes=${this.holes} p2max=${this.p2MaxMs.toFixed(1)}ms p2mean=${mean.toFixed(2)}ms` +
      ` | src=${this.source} dnd=${dnd} | ua=${chromeToken()}`
    );
  }
}

function chromeToken(): string {
  const m = navigator.userAgent.match(/(?:Chrome|Edg)\/[\d.]+/g);
  return m ? m.join(',') : navigator.userAgent.slice(0, 40);
}

export const stats = new Stats();

/** The title: `document.title` everywhere; under Tauri also the native window title, which the webview
 * does not update by itself -- needs `core:window:allow-set-title` in the capability (task 5). */
export async function startTitle(): Promise<void> {
  let native: ((title: string) => Promise<void>) | undefined;
  if ('__TAURI_INTERNALS__' in window) {
    try {
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      const w = getCurrentWindow();
      native = (title) => w.setTitle(title);
    } catch (e) {
      console.warn('native title not available', e);
    }
  }
  const write = () => {
    const line = stats.line();
    document.title = line;
    native?.(line).catch((e) => console.warn('setTitle failed', e));
  };
  write();
  setInterval(write, 1000);
}

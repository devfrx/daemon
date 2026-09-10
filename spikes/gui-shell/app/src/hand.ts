import { ref } from 'vue';
import { log } from './home';

/** SP-7's hand as a pointer (move 8): the 21 points of each hand drawn over the whole page, mirrored as in
 * spikes/gesti/relay/page.html, and the PINCH of the first hand -- thumb tip 4 and index tip 8 closer than
 * PINCH pixels of the 640x480 frame -- translated into pointer events at the index tip: pointerdown when it
 * closes, pointermove while it stays closed, pointerup when it opens. `pointerType: 'mouse'` so that dockview
 * does not wait the long press it reserves for touch. The relay is reached through the vite proxy at
 * `/stream` (P-3), so this works only in the dev browser. Nothing here is product code. */
export const handStatus = ref('not started');

const W = 640;
const H = 480;
const PINCH = 40;
const POINTER_ID = 7;
const CHAINS = [[0, 1, 2, 3, 4], [0, 5, 6, 7, 8], [5, 9, 10, 11, 12], [9, 13, 14, 15, 16], [13, 17, 18, 19, 20], [0, 17]];

type Point = [number, number];

export function startHand(canvas: HTMLCanvasElement): void {
  if (location.protocol !== 'http:') {
    handStatus.value = 'only in the dev browser: the relay is reached through the vite proxy';
    return;
  }
  const ctx = canvas.getContext('2d')!;
  const fit = () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
  };
  fit();
  window.addEventListener('resize', fit);
  let pinched = false;
  let last: Point = [0, 0];

  const toScreen = ([x, y]: Point): Point => [((W - x) / W) * window.innerWidth, (y / H) * window.innerHeight];

  const fire = (type: 'pointerdown' | 'pointermove' | 'pointerup', [x, y]: Point) => {
    last = [x, y];
    const target = document.elementFromPoint(x, y) ?? document.body;
    target.dispatchEvent(
      new PointerEvent(type, {
        bubbles: true,
        cancelable: true,
        composed: true,
        clientX: x,
        clientY: y,
        screenX: x,
        screenY: y,
        pointerId: POINTER_ID,
        pointerType: 'mouse',
        isPrimary: true,
        button: type === 'pointermove' ? -1 : 0,
        buttons: type === 'pointerup' ? 0 : 1,
      }),
    );
    if (type !== 'pointermove') {
      const cls = String(target.className).split(' ')[0];
      log(`move 8: ${type} at ${x.toFixed(0)},${y.toFixed(0)} on ${target.tagName.toLowerCase()}.${cls}`);
    }
  };

  const draw = (hands: Point[][], cursor: Point | null) => {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.strokeStyle = '#8cf';
    ctx.fillStyle = '#fff';
    ctx.lineWidth = 2;
    for (const hand of hands) {
      for (const chain of CHAINS) {
        ctx.beginPath();
        chain.forEach((i, k) => (k ? ctx.lineTo(hand[i][0], hand[i][1]) : ctx.moveTo(hand[i][0], hand[i][1])));
        ctx.stroke();
      }
      for (const [x, y] of hand) {
        ctx.beginPath();
        ctx.arc(x, y, 4, 0, Math.PI * 2);
        ctx.fill();
      }
    }
    if (cursor) {
      ctx.fillStyle = pinched ? '#f66' : '#6f6';
      ctx.beginPath();
      ctx.arc(cursor[0], cursor[1], 9, 0, Math.PI * 2);
      ctx.fill();
    }
  };

  const source = new EventSource('/stream');
  source.onopen = () => {
    handStatus.value = 'relay connected';
    log('move 8: relay connected');
  };
  source.onerror = () => {
    handStatus.value = 'relay not reachable: start spikes/gesti/relay first';
  };
  source.onmessage = (e) => {
    const raw = (JSON.parse(e.data) as { hands: Point[][] }).hands;
    const hands = raw.map((h) => h.map(toScreen));
    if (!raw.length) {
      if (pinched) {
        pinched = false;
        fire('pointerup', last);
      }
      draw(hands, null);
      return;
    }
    const [tx, ty] = raw[0][4];
    const [ix, iy] = raw[0][8];
    const closed = Math.hypot(tx - ix, ty - iy) <= PINCH; // in frame pixels, as in SP-7
    const tip = toScreen([ix, iy]);
    if (closed && !pinched) {
      pinched = true;
      fire('pointerdown', tip);
    } else if (closed) {
      fire('pointermove', tip);
    } else if (pinched) {
      pinched = false;
      fire('pointerup', tip);
    }
    draw(hands, tip);
  };
}

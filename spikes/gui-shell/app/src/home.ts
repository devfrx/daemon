import {
  createDockview,
  themeAbyss,
  type AddPanelOptions,
  type DockviewApi,
  type DockviewDndStrategy,
  type IDockviewPanel,
  type ITabRenderer,
  type Position,
  type TabPartInitParameters,
} from 'dockview-core';
import { VueContent } from './vue-bridge';
import Tile from './tiles/Tile.vue';

/** The panel types of the fake Home. Task 3 replaces `chat`, `scene` and `hand` with live tiles;
 * the names stay, so a layout saved before task 3 still restores after it. */
export const tiles: Record<string, () => VueContent> = {
  tile: () => new VueContent(Tile),
  chat: () => new VueContent(Tile),
  scene: () => new VueContent(Tile),
  hand: () => new VueContent(Tile),
};

const GRID = 24; // move 2: a floating group snaps to this grid, through dockview's hook
const LAYOUT_KEY = 'sp8-home-layout'; // move 7: the JSON of the last "save"
const DND_KEY = 'sp8-dnd'; // Q4: the drag strategy, kept across reloads

let api: DockviewApi;
let logEl: HTMLElement | undefined;
const lines: string[] = [];

/** Writes into the bar: the last few actions, newest first. Also on the console, for the reviewer. */
export function log(text: string): void {
  lines.unshift(`${new Date().toLocaleTimeString()} ${text}`);
  if (lines.length > 6) lines.pop();
  if (logEl) logEl.textContent = lines.join('  |  ');
  console.log(text);
}

export function currentDnd(): DockviewDndStrategy {
  const v = localStorage.getItem(DND_KEY);
  return v === 'pointer' || v === 'html5' ? v : 'auto';
}

/** The big grab handle: a tall tab with the title and three commands. The tab element is what dockview
 * drags, so a big tab is a big grab (move 5). A click on a command must not start a drag. */
class BigTab implements ITabRenderer {
  readonly element = document.createElement('div');

  init(p: TabPartInitParameters): void {
    this.element.className = 'bigtab';
    const title = document.createElement('span');
    title.className = 'bigtab-title';
    title.textContent = p.title ?? p.api.id;
    this.element.append(title);
    const command = (label: string, hint: string, run: () => void) => {
      const b = document.createElement('button');
      b.type = 'button';
      b.textContent = label;
      b.title = hint;
      b.addEventListener('pointerdown', (e) => e.stopPropagation());
      b.addEventListener('mousedown', (e) => e.stopPropagation());
      b.addEventListener('click', (e) => {
        e.stopPropagation();
        run();
      });
      this.element.append(b);
    };
    command('⧉', 'move 2: float this tile (drag it back into the grid to re-dock)', () => floatPanel(p.api.id));
    command('⤢', 'move 3: full page, or back', () => togglePage(p.api.id));
    command('↗', 'move 4: open in a separate window, from the command', () => popout(p.api.id));
  }
}

function panelOf(id: string): IDockviewPanel | undefined {
  const panel = api.getPanel(id);
  if (!panel) log(`no panel ${id}`);
  return panel;
}

export function floatPanel(id: string): void {
  const panel = panelOf(id);
  if (!panel) return;
  api.addFloatingGroup(panel, { x: 60, y: 60, width: 460, height: 320 });
  log(`move 2: ${id} floats`);
}

export function togglePage(id: string): void {
  const panel = panelOf(id);
  if (!panel) return;
  if (panel.api.isMaximized()) {
    panel.api.exitMaximized();
    log(`move 3: ${id} back`);
  } else {
    panel.api.maximize();
    log(`move 3: ${id} full page`);
  }
}

export function popout(id: string): void {
  const panel = panelOf(id);
  if (!panel) return;
  api
    .addPopoutGroup(panel, { popoutUrl: 'popout.html' })
    .then((ok) => log(`move 4 / Q3: popout ${ok ? 'OPENED' : 'REFUSED'} for ${id}`))
    .catch((e) => log(`move 4 / Q3: popout threw ${String(e)}`));
}

/** Move 6: the active tile goes into the nearest group in that direction; with no neighbour it splits
 * its own group on that side. Geometry, not dockview's navigation API: it is what a keyboard user sees. */
export function moveActive(dir: 'left' | 'right' | 'up' | 'down'): void {
  const panel = api.activePanel;
  if (!panel) {
    log('move 6: no active tile');
    return;
  }
  const from = panel.group.element.getBoundingClientRect();
  const beyond = (r: DOMRect) =>
    dir === 'left' ? r.right <= from.left + 1
    : dir === 'right' ? r.left >= from.right - 1
    : dir === 'up' ? r.bottom <= from.top + 1
    : r.top >= from.bottom - 1;
  const gap = (r: DOMRect) =>
    dir === 'left' ? from.left - r.right
    : dir === 'right' ? r.left - from.right
    : dir === 'up' ? from.top - r.bottom
    : r.top - from.bottom;
  const target = api.groups
    .filter((g) => g !== panel.group && !g.locked)
    .map((g) => ({ g, r: g.element.getBoundingClientRect() }))
    .filter(({ r }) => beyond(r))
    .sort((a, b) => gap(a.r) - gap(b.r))[0];
  if (target) {
    panel.api.moveTo({ group: target.g, position: 'center' });
    log(`move 6: ${panel.id} -> group ${target.g.id}`);
  } else {
    const side: Position = dir === 'up' ? 'top' : dir === 'down' ? 'bottom' : dir;
    panel.api.moveTo({ group: panel.group, position: side });
    log(`move 6: ${panel.id} splits ${side}`);
  }
}

/** Move 7: save is a string; the comparison after a reload is a string equality -- a measure, not a judgement. */
export function saveLayout(): void {
  const json = JSON.stringify(api.toJSON());
  localStorage.setItem(LAYOUT_KEY, json);
  log(`move 7: saved ${json.length} bytes -- now reload`);
}

function firstDifference(a: string, b: string): number {
  const n = Math.min(a.length, b.length);
  for (let i = 0; i < n; i += 1) if (a[i] !== b[i]) return i;
  return n;
}

/** Move 1: the nucleus and the strip do not move, and take nothing dropped on them. */
function lock(id: string): void {
  const group = api.getPanel(id)?.group;
  if (!group) return;
  group.locked = true;
  group.header.hidden = true;
}

function add(options: AddPanelOptions): void {
  api.addPanel(options);
}

function defaultLayout(): void {
  add({ id: 'nucleus', component: 'tile', title: 'Nucleo', params: { who: 'arriva col 6', text: "niente ancora: la rete viva e l'anello degli artefatti sono del sotto-progetto 6" } });
  add({ id: 'strip', component: 'tile', title: 'Striscia', params: { who: 'sempre visibile', text: 'degrado: — · permessi: — (vivi nel 2 col core vero)' }, position: { referencePanel: 'nucleus', direction: 'below' }, minimumHeight: 56, maximumHeight: 56 });
  add({ id: 'stato', component: 'tile', title: 'Stato', params: { who: 'il 2', text: 'degrado, policy VRAM e budget, «protetto quanto il tuo account»' }, position: { referencePanel: 'nucleus', direction: 'left' } });
  add({ id: 'permessi', component: 'tile', title: 'Permessi', params: { who: 'il 2', text: 'le triple concesse, la richiesta in attesa' }, position: { referencePanel: 'stato', direction: 'below' } });
  add({ id: 'attivita', component: 'tile', title: 'Attività', params: { who: 'arriva col 3', text: "l'indice di tutte le run" }, position: { referencePanel: 'nucleus', direction: 'right' } });
  add({ id: 'costi', component: 'tile', title: 'Costi', params: { who: 'arriva col 3', text: 'costo corrente e distanza dal tetto' }, position: { referencePanel: 'attivita', direction: 'below' } });
  add({ id: 'chat', component: 'chat', title: 'Chat', params: { who: 'il 2 col core finto, il 3 vero', text: 'il flusso di token' }, position: { referencePanel: 'nucleus', direction: 'above' } });
  add({ id: 'scene', component: 'scene', title: 'Scena 3D', params: { who: 'il pilastro degli asset 3D', text: 'la scena three dello spike' }, position: { referencePanel: 'chat', direction: 'right' } });
  add({ id: 'hand', component: 'hand', title: 'Mano', params: { who: 'il 12', text: 'i 21 punti di SP-7' }, position: { referencePanel: 'scene', direction: 'right' } });
  add({ id: 'passi', component: 'tile', title: 'Passi', params: { who: 'il 2', text: 'le invocazioni del registro' }, position: { referencePanel: 'hand', direction: 'right' } });
}

function buildBar(bar: HTMLElement): void {
  const button = (label: string, run: () => void) => {
    const b = document.createElement('button');
    b.type = 'button';
    b.textContent = label;
    b.addEventListener('click', run);
    bar.append(b);
  };
  button('salva (mossa 7)', saveLayout);
  button('ricarica', () => location.reload());
  button('azzera', () => {
    localStorage.removeItem(LAYOUT_KEY);
    location.reload();
  });
  button('stacca (F)', () => api.activePanel && floatPanel(api.activePanel.id));
  button('pagina intera (M)', () => api.activePanel && togglePage(api.activePanel.id));
  button('finestra a parte (P)', () => api.activePanel && popout(api.activePanel.id));
  const select = document.createElement('select');
  for (const s of ['auto', 'pointer', 'html5'] as const) {
    const o = document.createElement('option');
    o.value = s;
    o.textContent = `dnd: ${s}`;
    o.selected = s === currentDnd();
    select.append(o);
  }
  select.addEventListener('change', () => {
    const s = select.value as DockviewDndStrategy;
    localStorage.setItem(DND_KEY, s);
    api.updateOptions({ dndStrategy: s });
    log(`Q4: dndStrategy = ${s}`);
  });
  bar.append(select);
  logEl = document.createElement('span');
  logEl.id = 'log';
  bar.append(logEl);
}

function onKey(e: KeyboardEvent): void {
  const tag = (e.target as HTMLElement | null)?.tagName;
  if (tag === 'INPUT' || tag === 'SELECT' || tag === 'TEXTAREA') return;
  if (e.ctrlKey && e.altKey) {
    const dir =
      e.key === 'ArrowLeft' ? 'left'
      : e.key === 'ArrowRight' ? 'right'
      : e.key === 'ArrowUp' ? 'up'
      : e.key === 'ArrowDown' ? 'down'
      : null;
    if (dir) {
      e.preventDefault();
      moveActive(dir);
    }
    return;
  }
  const active = api.activePanel;
  if (!active) return;
  if (e.key === 'f' || e.key === 'F') floatPanel(active.id);
  else if (e.key === 'm' || e.key === 'M') togglePage(active.id);
  else if (e.key === 'p' || e.key === 'P') popout(active.id);
}

export function buildHome(dock: HTMLElement, bar: HTMLElement): DockviewApi {
  api = createDockview(dock, {
    theme: themeAbyss,
    defaultTabComponent: 'bigtab',
    dndStrategy: currentDnd(),
    floatingGroupBounds: 'boundedWithinViewport',
    popoutUrl: 'popout.html',
    transformFloatingGroupDrag: ({ proposed }) => ({
      left: Math.round(proposed.left / GRID) * GRID,
      top: Math.round(proposed.top / GRID) * GRID,
    }),
    createComponent: ({ name }) => (tiles[name] ?? tiles.tile)(),
    createTabComponent: () => new BigTab(),
  });
  buildBar(bar);
  api.layout(dock.clientWidth, dock.clientHeight);
  const saved = localStorage.getItem(LAYOUT_KEY);
  if (saved) {
    api.fromJSON(JSON.parse(saved));
    const again = JSON.stringify(api.toJSON());
    log(
      again === saved
        ? `move 7: EQUAL, ${saved.length} bytes before and after the reload`
        : `move 7: DIFFERENT at byte ${firstDifference(saved, again)} (${saved.length} vs ${again.length} bytes)`,
    );
  } else {
    defaultLayout();
  }
  lock('nucleus');
  lock('strip');
  window.addEventListener('keydown', onKey);
  window.addEventListener('resize', () => api.layout(dock.clientWidth, dock.clientHeight));
  return api;
}

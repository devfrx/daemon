"""patch_c13b.py -- task 13, the seventeen confirmed findings of the in-depth review R6 (2026-09-16, Opus 5), two
coordinator items from its coverage table, the recall in P-97, the realignments in tasks 14 and 15, one D row (D90),
one row in the open-items table, plus the ledger. Every anchor is asserted right before its write (count == 1 inside
the segment it belongs to); the two files are written atomically at the end. LF in, LF out.

Every line of code this script dictates was applied to the reviewer's compiled model (the short path
C:/Users/zagor/AppData/Local/Temp/probe-R6/gui) and measured before being written here: `vue-tsc --noEmit` zero
errors (with `@types/node` 24.13.5 and `"types": ["vite/client", "node"]`), `vitest run` 23 passed and 1 skipped
(the two new probes included), the generator green with three views and no `"missing"`, `vite build` green with
the decimal stamp in the bundle and no map in it, `npm run dev` serving the page (200, zero console errors, the
stamp injected through `/@vite/env`, the Italian module names, «niente ancora» on the nucleus, the drawer's veil);
and the two new probes each killed by its own mutation alone (the `isBuilt` filter removed; the nucleus line removed).
"""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
ROOT = r"C:\Users\zagor\Desktop\harness"
PLAN = ROOT + r"\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima.md"
LEDGER = ROOT + r"\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione\ledger.md"

raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw
t = raw


def sub(seg, old, new, n=1):
    c = seg.count(old)
    assert c == n, f"expected {n}, found {c}: {old[:110]!r}"
    return seg.replace(old, new)


def scoped(text, start_marker, end_marker, fn):
    lo = text.index(start_marker)
    hi = text.index(end_marker, lo)
    return text[:lo] + fn(text[lo:hi]) + text[hi:]


def between(seg, start, end):
    """The slice of `seg` from the ONE occurrence of `start` to the end of the ONE occurrence of `end` after it."""
    assert seg.count(start) == 1, f"start: {seg.count(start)} of {start[:80]!r}"
    lo = seg.index(start)
    hi = seg.index(end, lo) + len(end)
    return lo, hi


# ---- the new Passo 3, whole ------------------------------------------------------------------------------------------
PASSO3 = r'''- [ ] **Passo 3: l'ambiente delle sonde, e la domanda che si misura invece di dedurla**

In `gui/vite.config.ts` il blocco `test` diventa:

```ts
  test: {
    // ⛔ `jsdom` FROM THIS TASK ON: task 11 ran on `node` because nothing it built touched a DOM,
    // and said so. The frame mounts components, so it needs one.
    environment: "jsdom",
    include: ["src/**/*.test.ts"],
    // ⛔ `jsdom` 30.0.1 has no `ResizeObserver`, and `dockview-core` wants one the moment a grid is
    // created: the fake in this file is what lets a probe mount a grid at all (R6-8).
    setupFiles: ["src/jsdom-setup.ts"],
  },
```

e nasce `gui/src/jsdom-setup.ts`, **LF**:

```ts
// ⛔ `jsdom` 30.0.1 DOES NOT IMPLEMENT `ResizeObserver`, and `dockview-core` calls it the moment
// a grid is created (`watchElementResize`): without this, every probe that mounts a grid dies on
// `ReferenceError: ResizeObserver is not defined` before any layout question is asked --
// measured on 2026-09-16 (R6-8 of the in-depth review of task 13). Mounted by `test.setupFiles`.
//
// ⚠️ THIS FAKES AN ABSENT API, NOT LAYOUT: it observes nothing, and `getBoundingClientRect` still
// answers zeros under jsdom, which is why the geometry of `moveActive` is probed with rectangles
// of our own (P-97). The day jsdom ships a `ResizeObserver`, `??=` leaves it alone.
globalThis.ResizeObserver ??= class {
  observe(): void {}
  unobserve(): void {}
  disconnect(): void {}
};
```

⛔ **E poi si misura la cosa che la §9 del 2 dichiara DEDOTTA — *«che le prove della SPA girino senza browser»* —
invece di scoprirla al primo rosso.** ⛔ **RICHIAMO DEL 2026-09-16, dalla revisione in profondità di questo compito
(R6-8, misurato sul modello compilato):** questo passo diceva *«`dockview` misura gli elementi per disporre i gruppi, e
`jsdom` non fa layout: `getBoundingClientRect` rende zeri»*, e la sonda qui sotto **muore prima** di qualunque domanda sul
layout — `ReferenceError: ResizeObserver is not defined`, da `watchElementResize` di `dockview-core` — perché `jsdom`
30.0.1 non implementa quell'API. Con la finta di `jsdom-setup.ts` montata da `setupFiles` la stessa sonda è **verde**, e
con essa ogni sonda di questo compito che monta una griglia — comprese le tre di `describe("the dock")`, che la tabella
com'era avrebbe mandato al revisore a mano per una causa che una riga di configurazione toglie. Il fatto sul layout
resta vero (i rettangoli sono zeri: **P-97**), ma non è ciò che questa sonda misura. Si scrive la sonda usa-e-getta e si
legge l'esito **con** la finta montata:

```bash
cat > /tmp/dockview-in-jsdom.test.ts <<'EOF'
import { describe, expect, it } from "vitest";
import { createDockview } from "dockview-core";

describe("dockview under jsdom", () => {
  it("mounts and takes a panel", () => {
    const host = document.createElement("div");
    document.body.append(host);
    const api = createDockview(host, { createComponent: () => ({ element: document.createElement("div"), init() {} }) });
    api.layout(1200, 800);
    api.addPanel({ id: "one", component: "any" });
    expect(api.panels.map((p) => p.id)).toEqual(["one"]);
  });
});
EOF
cp /tmp/dockview-in-jsdom.test.ts gui/src/frame-probe.test.ts
cd gui && npx vitest run src/frame-probe.test.ts; echo "EXIT=$?"; cd ..
rm gui/src/frame-probe.test.ts
```

| Esito | Che cosa fa questo compito |
|---|---|
| **`EXIT=0`** | le sonde del Passo 17 girano sotto `jsdom` come scritte — è l'esito misurato il 2026-09-16 con la finta montata |
| **`EXIT` non zero con `ResizeObserver is not defined`** | la finta **non è montata**: `grep -c 'setupFiles' gui/vite.config.ts` → **1** e `ls gui/src/jsdom-setup.ts`, poi si rimisura |
| **`EXIT` non zero che RESTA con la finta** | ⛔ **non si aggira e non si finge:** la sonda del Passo 17 si **restringe** a ciò che non vuole layout — il registro, la forma delle tre viste, gli store — e la parte che vuole una griglia disposta passa al **revisore nel browser**, che la regola 5 della testa già prescrive per questo compito. Si scrive una **voce d'errata** con l'esito vero, e la riga «🔶 Dedotto» della §9 del 2 riceve il suo richiamo datato |

📌 **Perché si misura adesso e non al Passo 17:** è la differenza fra scrivere una sonda e riscriverla. E l'esito è
un **fatto sull'ambiente**, non sul codice: vale anche per il generatore del Passo 13 e per il compito 14.

'''

# ---- the new Passo 6, whole ------------------------------------------------------------------------------------------
PASSO6 = r'''- [ ] **Passo 6: il timbro, letto dove il kernel l'ha scritto — dalla configurazione, non dal browser**

⛔ **RICHIAMO DEL 2026-09-16, dalla revisione in profondità di questo compito (R6-3, misurato sul modello compilato e nel
browser): questo passo dettava `import MAP from "../../schema/fixtures/ipc_v1.map?raw"` in `stamp.ts`, e con esso
`npm run dev` rende una PAGINA BIANCA.** Il server di sviluppo di `vite` 8.3.0 tratta ogni URL che finisce in `.map` come
una richiesta di source map e, non trovandone una nel grafo dei moduli, serve il file **statico** come `application/json`,
saltando la trasformazione di `?raw`: il browser rifiuta il modulo (*«Failed to load module script: … MIME type of
"application/json"»*). Provato sulla stessa identica copia rinominata `.txt` e `.mapx` → `text/javascript`. ⛔ **Il difetto
è MUTO nel cancello:** `vite build` inlinea la mappa ed è verde, e le sonde sotto `vitest` sono verdi — lo uccideva solo il
criterio che chiede al revisore di **guardare**. La cura non rinomina la mappa (è `.map` come `record_v1.map` del giornale,
e il compito 3 la scrive così) e non aggiunge un secondo file col timbro: **la legge la configurazione, in Node, e consegna
il timbro alla SPA come costante** con `define` — **D90**. Il browser non chiede mai un `.map`.

In `gui/vite.config.ts`, in testa, prima di `import vue from "@vitejs/plugin-vue";`:

```ts
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

```

fra il commento sulla radice di `vite` e `export default defineConfig({`:

```ts
/**
 * ⛔ THE BUILD STAMP IS READ HERE, IN NODE, AND HANDED TO THE SPA AS A CONSTANT (D52, D90): the
 * last line of `ipc_v1.map`, which the KERNEL wrote. Not a `?raw` import from the browser: the
 * dev server of `vite` 8.3.0 treats every URL that ends in `.map` as a source-map request and
 * serves the file statically as `application/json`, skipping the transform -- `npm run dev`
 * showed a blank page while `vite build` and the probes stayed green (R6-3, measured on
 * 2026-09-16). Read once, when the config loads: the map changes only when the kernel's schema
 * does, and then the SPA is rebuilt anyway.
 *
 * ⛔ THE REGEXP ANCHORS THE LINE with `^` and `$` in multiline mode: the map also carries the
 * `Debug` of every message, and one of them may contain the word.
 */
const MAP = readFileSync(fileURLToPath(new URL("./schema/fixtures/ipc_v1.map", import.meta.url)), "utf8");
const DIGITS = /^stamp 0x([0-9a-fA-F]{16})$/m.exec(MAP)?.[1];
if (DIGITS === undefined) {
  throw new Error("ipc_v1.map carries no `stamp 0x…` line: regenerate the fixtures");
}

```

e dentro `defineConfig({ … })`, fra `plugins: [vue()],` e `test: {`:

```ts
  define: {
    // ⛔ `BigInt` AND NOT `Number`: the stamp is FNV-1a over the whole set and passes
    // `Number.MAX_SAFE_INTEGER` as a matter of course -- the doc of `U64` says it once for everyone.
    __BUILD_STAMP__: JSON.stringify(BigInt(`0x${DIGITS}`).toString(10)),
  },
```

`gui/src/schema/stamp.ts`, **LF** — **D52**:

```ts
import type { U64 } from "./messages";

/** Handed in by `define` in `vite.config.ts`, which reads the last line of `ipc_v1.map` in Node. */
declare const __BUILD_STAMP__: string;

/**
 * The build stamp the SPA presents in `Hello` (§6.1.2 of the kernel spec).
 *
 * ⛔ READ FROM `ipc_v1.map`, WHICH THE KERNEL WROTE, and not from the `hello` fixture: that
 * fixture carries the CANONICAL SET's arbitrary value, chosen so that no two encodings are
 * equal, and it is not the stamp. The map is the only place on this side of the wire where the
 * stamp has an honest source -- the TypeScript mirror of the types is hand-written, so a stamp
 * computed here would agree with itself and with nothing else.
 *
 * ⚠️ AND THAT IS WHAT MAKES THE HANDSHAKE A REAL CHECK: regenerate the schema without
 * rebuilding the SPA and the two stamps diverge, so the core answers `StaleBuild`. That is what
 * I4 bought by renouncing versioning.
 *
 * ⛔ THE READING HAPPENS IN `vite.config.ts` AND NOT HERE (D90): the browser must never ask the
 * dev server for a `.map` URL -- it would get a source map's `application/json` instead of a
 * module (R6-3). What arrives here is the decimal string, already through `BigInt`.
 */
export function buildStamp(): U64 {
  return __BUILD_STAMP__;
}
```

⚠️ **In sviluppo `define` NON riscrive il sorgente: `vite` inietta la costante come globale dal modulo `/@vite/env`**, quindi
un `curl` sul modulo trasformato non la mostra e non è un oracolo (misurato il 2026-09-16); alla build è sostituita nel
bundle, e sotto `vitest` è un globale — è per questo che `stamp.ts` la **dichiara** con `declare const` invece di importare
qualcosa. Che la mappa arrivi davvero, in tutti e tre i mondi, lo misura il criterio di chiusura.

⛔ **E l'espressione regolare àncora la riga con `^` e `$` in modalità multilinea**, non cerca `stamp` dovunque: la
mappa porta anche i valori `Debug` dei messaggi, e uno di essi può contenere la parola. Sta in `vite.config.ts`, dove la
mappa si legge, e il criterio di chiusura la rilancia sulla mappa vera.

'''


def task13(seg):
    # ---- Files -------------------------------------------------------------------------------------------------------
    seg = sub(seg,
              "- Modify: `gui/vite.config.ts` (**LF**) — l'ambiente delle sonde passa a `jsdom`, e il perché\n",
              "- Modify: `gui/vite.config.ts` (**LF**) — l'ambiente delle sonde passa a `jsdom`, e il perché; la finta di "
              "`ResizeObserver` in `setupFiles` (R6-8) e il timbro letto dalla mappa e consegnato con `define` (R6-3, **D90**)\n"
              "- Modify: `gui/tsconfig.json` (**LF**, dal compito 11) — `\"types\": [\"vite/client\", \"node\"]`, perché questo è il "
              "primo compito che importa da `node:` (R6-1)\n")
    seg = sub(seg,
              "- Create: `gui/src/schema/stamp.ts` (**LF**) — il timbro letto da `ipc_v1.map` (**D52**)\n",
              "- Create: `gui/src/schema/stamp.ts` (**LF**) — il timbro che `vite.config.ts` legge da `ipc_v1.map` e consegna "
              "con `define` (**D52**, **D90**)\n"
              "- Create: `gui/src/jsdom-setup.ts` (**LF**) — la finta di `ResizeObserver`, che `jsdom` 30.0.1 non ha e "
              "`dockview-core` vuole (R6-8)\n")

    # ---- Interfaces --------------------------------------------------------------------------------------------------
    seg = sub(seg,
              "- Consumes, dal **compito 3**: `gui/schema/fixtures/ipc_v1.map`, di cui legge **la sola ultima riga** (**D52**)\n",
              "- Consumes, dal **compito 3**: `gui/schema/fixtures/ipc_v1.map`, di cui `vite.config.ts` legge **la sola ultima "
              "riga** in Node (**D52**, **D90**): nessun modulo della SPA importa la mappa\n")
    seg = sub(seg,
              "`isModule(name: string): boolean`, `placeholderParams(name: string): Record<string, unknown>`\n",
              "`isModule(name: string): boolean`, `isBuilt(name: string): boolean` (R6-17), "
              "`placeholderParams(name: string): Record<string, unknown>`\n")

    # ---- Passo 1 (R6-12) ---------------------------------------------------------------------------------------------
    seg = sub(seg,
              "`environment: \"node\"`; **due** righe `/gui/` in `.gitignore`, dal compito 11 (**D38**); tutto **LF**.\n",
              "`environment: \"node\"`; **tre** righe `/gui/` in `.gitignore` — due dal compito 11 e `/gui/fake-core/target/` dal "
              "12 (**D38**; ⚠️ qui stava «due», ricopiato dal Passo 1 del 12 senza contare ciò che il 12 aggiunge — R6-12); "
              "tutto **LF**.\n")

    # ---- Passo 2 (R6-1) ----------------------------------------------------------------------------------------------
    seg = sub(seg,
              "```json\n    \"jsdom\": \"30.0.1\",\n    \"@vue/test-utils\": \"2.5.0\"\n```\n\n⛔ **DUE pacchetti di `dockview` e non uno",
              "```json\n    \"@types/node\": \"24.13.5\",\n    \"jsdom\": \"30.0.1\",\n    \"@vue/test-utils\": \"2.5.0\"\n```\n\n"
              "⛔ **`@types/node` entra QUI, e con esso la riga `types` di `gui/tsconfig.json` — R6-1, misurato il 2026-09-16 sul\n"
              "modello compilato:** questo è il **primo** compito del piano che importa da `node:` — `copy.test.ts` e\n"
              "`generate-views.test.ts`, sei righe; l'11 e il 12 nessuna — e il `tsconfig.json` dell'11 dichiara\n"
              "`\"types\": [\"vite/client\"]`, che **spegne** l'inclusione automatica dei `@types/*`. Senza, `npm run build` esce **2** con\n"
              "otto errori `TS2307`/`TS2591` (*«Cannot find module 'node:fs'»*, *«Cannot find name 'process'»*). ⚠️ **La major segue\n"
              "quella di Node** (P-64: v24), quindi la versione si misura con `npm view @types/node@24 version | tail -1` e non con\n"
              "`latest`, che oggi indica la 22. In `gui/tsconfig.json`, che è **LF**, la riga `\"types\": [\"vite/client\"],` diventa\n"
              "`\"types\": [\"vite/client\", \"node\"],` — con Python, così il file resta com'è nel resto:\n"
              "\n"
              "```bash\n"
              "python - <<'EOF'\n"
              "import io\n"
              "p = \"gui/tsconfig.json\"\n"
              "b = io.open(p, encoding=\"utf-8\", newline=\"\").read()\n"
              "old = '\"types\": [\"vite/client\"],'\n"
              "assert b.count(old) == 1, b.count(old)\n"
              "io.open(p, \"w\", encoding=\"utf-8\", newline=\"\").write(b.replace(old, '\"types\": [\"vite/client\", \"node\"],'))\n"
              "EOF\n"
              "grep -c '\"types\": \\[\"vite/client\", \"node\"\\]' gui/tsconfig.json\n"
              "```\n"
              "\n"
              "Atteso: **1**. ⚠️ Il compito **14** aggiunge altre tre righe `node:` (`contrast.test.ts`) e trova la casa già pronta.\n"
              "\n"
              "⛔ **DUE pacchetti di `dockview` e non uno")

    # ---- Passo 3, whole (R6-7, R6-8) ---------------------------------------------------------------------------------
    lo, hi = between(seg, "- [ ] **Passo 3: l'ambiente delle sonde, e la domanda che si misura invece di dedurla**\n",
                     "un **fatto sull'ambiente**, non sul codice: vale anche per il compito 14.\n\n")
    old3 = seg[lo:hi]
    assert "cat > /tmp/dockview-in-jsdom.test.ts <<'EOF'" in old3 and old3.count("Passo 16") == 3, old3.count("Passo 16")
    seg = seg[:lo] + PASSO3 + seg[hi:]

    # ---- Passo 4 (R6-11) ---------------------------------------------------------------------------------------------
    seg = sub(seg,
              "   ⚠️ G20 WANTS AA CONTRAST AND THESE VALUES ARE NOT YET PROVED TO HAVE IT: the check is\n"
              "   `axe-core` on the mounted components, and it arrives with the accessibility of task 14. A\n"
              "   value that fails it is a change to THIS file, not to forty templates -- which is the whole\n"
              "   reason the shape comes before the palette. */\n",
              "   ⚠️ G20 WANTS AA CONTRAST, and the check is `contrast.test.ts` in this folder: `axe-core` under\n"
              "   jsdom files `color-contrast` as INCOMPLETE every time (there is no layout to read a background\n"
              "   from -- measured on 2026-09-15, task 14), so a green from axe proves nothing about it. A value\n"
              "   that fails is a change to THIS file, not to forty templates -- which is the whole reason the\n"
              "   shape comes before the palette. */\n")
    seg = sub(seg,
              ":focus-visible {\n  outline: 2px solid var(--accent);\n  outline-offset: 2px;\n}\n```\n\n"
              "- [ ] **Passo 5: le scritte, e l'italiano dichiarato**\n",
              ":focus-visible {\n  outline: 2px solid var(--accent);\n  outline-offset: 2px;\n}\n```\n\n"
              "⚠️ **Il capoverso su G20 è già nella forma di D53 — R6-11, 2026-09-16:** diceva che il contrasto lo prova `axe-core`\n"
              "sui componenti montati, e **D53** e **P-86** dello stesso piano dicono l'opposto con la misura (sotto `jsdom` `axe`\n"
              "mette `color-contrast` fra gli *incompleti*); il 14 lo riscriveva, e ora lo **verifica** — il commit del 13 non dice il\n"
              "falso nemmeno per un compito. La sonda `contrast.test.ts` che il capoverso nomina nasce col 14 (D53), come dice.\n"
              "\n"
              "- [ ] **Passo 5: le scritte, e l'italiano dichiarato**\n")

    # ---- Passo 5 (R6-4, R6-14) ---------------------------------------------------------------------------------------
    seg = sub(seg,
              "    \"compact\": \"Compatta\"\n  },\n  \"bar\": {\n",
              "    \"compact\": \"Compatta\"\n  },\n"
              "  \"modules\": {\n"
              "    \"chat\": \"Chat\",\n"
              "    \"status\": \"Stato\",\n"
              "    \"permissions\": \"Permessi\",\n"
              "    \"steps\": \"Passi\",\n"
              "    \"activity\": \"Attività\",\n"
              "    \"settings\": \"Impostazioni\",\n"
              "    \"scope\": \"Ambito\",\n"
              "    \"diff\": \"Diff\",\n"
              "    \"preview\": \"Anteprima\",\n"
              "    \"terminal\": \"Terminale\",\n"
              "    \"sensors\": \"Sensori\",\n"
              "    \"costs\": \"Costi\",\n"
              "    \"knowledge\": \"Knowledge base\",\n"
              "    \"assets3d\": \"Asset 3D\",\n"
              "    \"voice\": \"Voce e gesti\",\n"
              "    \"backup\": \"Backup\",\n"
              "    \"checkpoint\": \"Checkpoint\",\n"
              "    \"models\": \"Modelli locali\"\n"
              "  },\n"
              "  \"bar\": {\n")
    seg = sub(seg,
              "  \"placeholder\": {\n    \"who\": \"arriva col sotto-progetto {number}\",\n",
              "  \"placeholder\": {\n    \"nucleus\": \"niente ancora\",\n    \"who\": \"arriva col sotto-progetto {number}\",\n")
    seg = sub(seg,
              "    \"closeMissing\": \"Chiudi il pannello\"\n  }\n}\n```\n\n`gui/src/i18n.ts`, **LF**:\n",
              "    \"closeMissing\": \"Chiudi il pannello\"\n  }\n}\n```\n\n"
              "⛔ **Le diciotto chiavi `modules.*` stanno QUI, per esteso — R6-4, misurato il 2026-09-16 nel browser:** il Passo 12\n"
              "diceva che *«entrano nello stesso passo»* e nessun passo le scriveva; la sonda `has a name for every module type` era\n"
              "rossa e ogni linguetta e ogni riga del cassetto rendevano la **chiave inglese** (`modules.status`, `modules.chat`, …)\n"
              "— cioè l'inglese che **P-95** esiste per togliere, con un prefisso in più. I nomi sono quelli della §1 della stella\n"
              "polare: le cinque tabelle piene e la colonna «Modulo» della corta, dove la riga *«Knowledge base e Nucleo a pagina\n"
              "intera»* dà **Knowledge base**. E `placeholder.nucleus` è la frase del nucleo (R6-14, Passo 12).\n"
              "\n"
              "`gui/src/i18n.ts`, **LF**:\n")

    # ---- Passo 6, whole (R6-3, D90) ----------------------------------------------------------------------------------
    lo, hi = between(seg, "- [ ] **Passo 6: il timbro, letto dove il kernel l'ha scritto**\n",
                     "mappa porta anche i valori `Debug` dei messaggi, e uno di essi può contenere la parola.\n\n")
    old6 = seg[lo:hi]
    assert 'import MAP from "../../schema/fixtures/ipc_v1.map?raw";' in old6 and "- [ ] **Passo 7" not in old6
    seg = seg[:lo] + PASSO6 + seg[hi:]

    # ---- Passo 9 (R6-15) ---------------------------------------------------------------------------------------------
    seg = sub(seg,
              " * ⛔ ONE LAYOUT PER VIEW, AND THE PACKAGE CARRIES THEM ALL (D80). Row 1 of §2 of the north star\n"
              " * makes the layout \"which view is open, FOR EVERY VIEW where the panels are\", and row 6 has a\n",
              " * ⛔ ONE LAYOUT PER VIEW, AND THE PACKAGE CARRIES THEM ALL (D80). The opening paragraph of §2 of the\n"
              " * north star makes the layout \"which view is open, FOR EVERY VIEW where the panels are\", and row 6 has a\n")

    # ---- Passo 11 (R6-10) --------------------------------------------------------------------------------------------
    seg = sub(seg,
              " * ⛔ A CLICK ON A COMMAND MUST NOT START A DRAG, and stopping `click` alone is not enough:\n"
              " * `dockview` begins the drag on `pointerdown`/`mousedown`, so both are stopped here. Measured in\n"
              " * SP-8; without it, every press of a command drags the tile a few pixels first.\n",
              " * ⛔ THE DAY THIS TAB CARRIES A COMMAND, A CLICK ON IT MUST NOT START A DRAG, and stopping `click`\n"
              " * alone is not enough: `dockview` begins the drag on `pointerdown`/`mousedown`, so both must be\n"
              " * stopped ON THE BUTTON -- measured in SP-8 (`spikes/gui-shell/app/src/home.ts`); without it, every\n"
              " * press of a command drags the tile a few pixels first. This tab carries NO command (the module's\n"
              " * commands are the menu of task 14), so nothing is stopped here yet: the trap is written so that\n"
              " * task 14 finds it instead of paying it again.\n")
    seg = sub(seg,
              "⛔ **La riga sul `pointerdown` resta scritta** perché è la trappola, e il 14 la\ntrova già detta invece di ripagarla.\n",
              "⛔ **La riga sul `pointerdown` resta scritta** perché è la trappola, e il 14 la\ntrova già detta invece di ripagarla. "
              "⚠️ **E dice di sé che qui non ferma nulla (R6-10, 2026-09-16):** diceva *«so both are\nstopped here»*, e il file non "
              "registra nessun ascoltatore — un commento nel sorgente che afferma un meccanismo assente\nè la specie di X-4 "
              "dell'audit, nel codice invece che in un verbale.\n")

    # ---- Passo 12 (R6-14, R6-17, R6-4) -------------------------------------------------------------------------------
    seg = sub(seg,
              "// `api` arrives from `VueContent`, which hands `dockview`'s init parameters to the app.\n"
              "defineProps<{ api?: DockviewPanelApi; params?: { module?: string; who?: number; missing?: boolean } }>();\n"
              "</script>\n",
              "// `api` arrives from `VueContent`, which hands `dockview`'s init parameters to the app.\n"
              "defineProps<{ api?: DockviewPanelApi; params?: { module?: string; who?: number; missing?: boolean } }>();\n"
              "\n"
              "// ⛔ THE NUCLEUS SAYS «NIENTE ANCORA» AND THE OTHER TILES SAY WHO FILLS THEM (row Home of \"Il\n"
              "// modello della GUI\", and row 7 of §3 of the north star): the centre of Home is the knowledge\n"
              "// base's graph, and until sub-project 6 it says so in its own words, not with a tile's phrase\n"
              "// (R6-14, 2026-09-16). It still says who fills it, because the drawer does too.\n"
              "const NUCLEUS = \"knowledge\";\n"
              "</script>\n")
    seg = sub(seg,
              "    <p v-else-if=\"params?.who !== undefined\">{{ $t(\"placeholder.who\", { number: params.who }) }}</p>\n",
              "    <template v-else-if=\"params?.who !== undefined\">\n"
              "      <p v-if=\"params.module === NUCLEUS\">{{ $t(\"placeholder.nucleus\") }}</p>\n"
              "      <p>{{ $t(\"placeholder.who\", { number: params.who }) }}</p>\n"
              "    </template>\n")
    seg = sub(seg,
              "export function isModule(name: string): boolean {\n  return PANEL_TYPES.some((type) => type.name === name);\n}\n",
              "export function isModule(name: string): boolean {\n  return PANEL_TYPES.some((type) => type.name === name);\n}\n"
              "\n"
              "/** ⛔ WHAT THE REGISTRY CAN BUILD -- the strip, and the modules task 14 plugs in. `apply` in\n"
              " * `dock.ts` asks it before putting placeholder params back on a panel (R6-17): the strip is a\n"
              " * piece of the frame that lives in the grid (D50), carries no `params`, and is not a module type. */\n"
              "export function isBuilt(name: string): boolean {\n  return BUILT.has(name);\n}\n")
    seg = sub(seg,
              "⚠️ **E le diciotto chiavi `modules.*` entrano in `locales/it.json`** nello stesso passo — una riga per `module`,\n"
              "col nome del modulo in italiano come lo scrive la §1 della stella polare.\n",
              "⚠️ **Le diciotto chiavi `modules.*` — una per `module`, col nome italiano della §1 della stella polare — stanno in\n"
              "`locales/it.json` dal Passo 5, dettate per esteso.** ⛔ **RICHIAMO DEL 2026-09-16 (R6-4):** qui stava *«entrano … nello\n"
              "stesso passo»*, e nessun passo le scriveva — nel browser ogni linguetta rendeva `modules.status`, `modules.chat`, ….\n")

    # ---- Passo 13 (R6-5, R6-2, R6-17, R6-9) --------------------------------------------------------------------------
    seg = sub(seg,
              "cd gui && node -e \"const v=require('vitest'); console.log(typeof v.it.skipIf, typeof v.it.runIf)\"; cd ..\n```\n\n"
              "Atteso: **`function function`**. ⛔ **Se non lo è**, la guardia diventa un `if (…) return;` in testa al corpo, e si\n"
              "scrive una **voce d'errata** con l'esito vero — non si lascia un generatore che gira nel cancello.\n",
              "cd gui && node --input-type=module -e \"import * as v from 'vitest'; console.log(typeof v.it.skipIf, typeof v.it.runIf)\"; cd ..\n```\n\n"
              "Atteso: **`function function`**. ⛔ **Se non lo è**, la guardia diventa un `if (…) return;` in testa al corpo, e si\n"
              "scrive una **voce d'errata** con l'esito vero — non si lascia un generatore che gira nel cancello. ⚠️ **In forma ESM\n"
              "(R6-5, misurato il 2026-09-16):** `vitest` è ESM-only e un `require('vitest')` esce con *«Vitest cannot be imported in a\n"
              "CommonJS module using require()»* — avrebbe mandato sul ramo «non c'è» per una ragione che non è la sua, a riscrivere\n"
              "una guardia che funziona; è la forma che il criterio di chiusura usa già per il timbro.\n")
    seg = sub(seg,
              "import { createDockview, type DockviewApi } from \"dockview-core\";\nimport { it } from \"vitest\";\n\n"
              "import { componentFor, placeholderParams } from \"../registry\";\n",
              "import { createDockview, type DockviewApi } from \"dockview-core\";\n"
              "import { createPinia, setActivePinia } from \"pinia\";\n"
              "import { beforeAll, it } from \"vitest\";\n\n"
              "import { componentFor, placeholderParams } from \"../registry\";\n")
    seg = sub(seg,
              "const OUT = join(dirname(fileURLToPath(import.meta.url)));\n\nfunction dock(): DockviewApi {\n",
              "const OUT = join(dirname(fileURLToPath(import.meta.url)));\n\n"
              "// ⛔ A PINIA MUST BE ACTIVE BEFORE A GRID IS BUILT (R6-2, measured on 2026-09-16): `Strip.vue`\n"
              "// calls `useCore()` at setup, and `VueContent` mounts every panel as an app of its own WITHOUT\n"
              "// pinia -- the root app installs it, and this file has no root app. Without this line the\n"
              "// generator dies on `getActivePinia()`.\n"
              "beforeAll(() => {\n  setActivePinia(createPinia());\n});\n\n"
              "function dock(): DockviewApi {\n")
    seg = sub(seg,
              r"tr -cd '\r' < gui/src/panels/views/home.json | wc -c" + "\n```\n\n"
              "Atteso: **`EXIT=0`**, tre file, **zero** CR. ⛔ **Se `EXIT` non è zero perché `dockview` non regge `jsdom`** (il\n"
              "Passo 3 lo ha già misurato), il generatore si lancia **nel browser** da una pagina usa-e-getta di `npm run dev` e i\n"
              "tre JSON si salvano a mano: è una voce d'errata col comando vero, non un JSON scritto a mano.\n",
              r"tr -cd '\r' < gui/src/panels/views/home.json | wc -c" + "\n"
              "cat gui/src/panels/views/*.json | grep -c '\"missing\"'\n```\n\n"
              "Atteso: **`EXIT=0`**, tre file, **zero** CR, e **0** sull'ultimo — la striscia esce senza `params` e nessuna vista\n"
              "committata porta un «tipo sparito» (R6-17). ⛔ **Se `EXIT` non è zero perché `dockview` non regge `jsdom`** (il\n"
              "Passo 3 lo ha già misurato: con la finta di `ResizeObserver` regge — R6-8; e senza una pinia attiva il generatore muore\n"
              "su `getActivePinia()` — R6-2, per questo il `beforeAll`), il generatore si lancia **nel browser** da una pagina\n"
              "usa-e-getta di `npm run dev` e i tre JSON si salvano a mano: è una voce d'errata col comando vero, non un JSON scritto\n"
              "a mano.\n")
    seg = sub(seg,
              " * ⛔ THE ANNOTATION IS THE CHECK. `resolveJsonModule` widens a `.json` to its literal shape, and\n"
              " * assigning it to `SerializedDockview` makes `vue-tsc` compare the committed file against\n"
              " * `dockview`'s own type -- the web world's level 1, on a file nobody writes by hand.\n",
              " * ⚠️ THE `as` IS AN ASSERTION, NOT THE CHECK, AND IT IS LOAD-BEARING: `resolveJsonModule` widens a\n"
              " * `.json` to its literal shape, and that shape is NOT assignable to `SerializedDockview` -- measured\n"
              " * on 2026-09-16 (R6-9): with the three generated views and the `as` removed, `vue-tsc` answers three\n"
              " * `TS2322`. What the assertion catches is a GROSS mismatch (`TS2352` when a field changes type or\n"
              " * goes missing), not a field-by-field comparison: `panels` removed outright, or an unknown key,\n"
              " * passes. The check that the committed files are views the frame can build is the first probe of\n"
              " * `frame.test.ts` -- every `component` known to the registry, and no view without panels.\n")

    # ---- Passo 14 (R6-17) --------------------------------------------------------------------------------------------
    seg = sub(seg,
              "import { componentFor, placeholderParams } from \"../panels/registry\";\n",
              "import { componentFor, isBuilt, placeholderParams } from \"../panels/registry\";\n")
    seg = sub(seg,
              "  for (const panel of api.panels) {\n    if (Object.keys(panel.params ?? {}).length === 0) {\n"
              "      panel.api.updateParameters(placeholderParams(panel.id));\n    }\n  }\n}\n```\n",
              "  for (const panel of api.panels) {\n"
              "    // ⛔ ONLY WHAT NOBODY BUILT (R6-17): the strip is a piece of the frame, carries no `params`, and\n"
              "    // is not a module type -- without this line it got `{ missing: true }` at every `apply`, and\n"
              "    // that value entered the saved package at the first `settle`. Unseen, because `Strip.vue`\n"
              "    // ignores its params.\n"
              "    if (!isBuilt(panel.id) && Object.keys(panel.params ?? {}).length === 0) {\n"
              "      panel.api.updateParameters(placeholderParams(panel.id));\n    }\n  }\n}\n```\n")
    seg = sub(seg,
              "vera **anche su un pacchetto vecchio**, non solo su una vista committata.\n",
              "vera **anche su un pacchetto vecchio**, non solo su una vista committata. ⚠️ **E solo a ciò che nessuno ha costruito —\n"
              "R6-17, misurato il 2026-09-16:** la striscia è un pezzo della cornice che vive nella griglia (**D50**), non ha `params`\n"
              "e non è un tipo di modulo, quindi senza `isBuilt` riceveva `{ missing: true }` a ogni `apply`, e quel valore entrava\n"
              "nel pacchetto salvato al primo `settle` — invisibile, perché `Strip.vue` ignora i `params`. La terza sonda di\n"
              "`describe(\"the dock\")` (Passo 17) lo tiene, e la mutazione che toglie il filtro la rende rossa da sola.\n")

    # ---- Passo 15 (R6-16) --------------------------------------------------------------------------------------------
    seg = sub(seg,
              "    <DialogTrigger class=\"drawer-open\">{{ $t(\"drawer.open\") }}</DialogTrigger>\n",
              "    <DialogTrigger>{{ $t(\"drawer.open\") }}</DialogTrigger>\n")
    seg = sub(seg,
              "<style scoped>\n.drawer {\n  position: fixed;\n  inset: auto 0 0 0;\n  max-height: 60vh;\n",
              "<style scoped>\n"
              "/* ⛔ THE OVERLAY IS A VEIL, AND `reka-ui` DRESSES NOTHING: without these rules it was a `div` in\n"
              "   normal flow with no background (R6-16, seen in the browser on 2026-09-16). Both sit above\n"
              "   `dockview`, whose floating groups are at 99. */\n"
              ".drawer-overlay {\n  position: fixed;\n  inset: 0;\n  background: rgb(0 0 0 / 0.45);\n  z-index: 100;\n}\n"
              ".drawer {\n  position: fixed;\n  inset: auto 0 0 0;\n  z-index: 101;\n  max-height: 60vh;\n")
    seg = sub(seg,
              ".who {\n  color: var(--ink-dim);\n  margin-left: var(--space-2);\n}\n</style>\n```\n\n`gui/src/frame/Frame.vue`, **LF**:\n",
              ".who {\n  color: var(--ink-dim);\n  margin-left: var(--space-2);\n}\n</style>\n```\n\n"
              "⚠️ **Il velo e i due `z-index` — R6-16, visto nel browser il 2026-09-16:** `reka-ui` non porta stile proprio, quindi\n"
              "`DialogOverlay` era un `div` nel flusso normale senza sfondo (`position: static`), e il cassetto poteva finire sotto un\n"
              "gruppo galleggiante di `dockview`, che mette i suoi a **99**; la classe `drawer-open` sul bottone non aveva nessuna\n"
              "regola ed è tolta. Il design system resta segnaposto: qui c'è la **forma**, non la palette.\n"
              "\n"
              "`gui/src/frame/Frame.vue`, **LF**:\n")

    # ---- Passo 17 (R6-14, R6-17): two probes ------------------------------------------------------------------------
    seg = sub(seg,
              "    expect(wrapper.text()).toContain(String(type!.who));\n  });\n\n"
              "  it(\"says a type that is GONE is gone, does not promise a sub-project, and closes\", async () => {\n",
              "    expect(wrapper.text()).toContain(String(type!.who));\n  });\n\n"
              "  it(\"says «niente ancora» on the nucleus, and still who fills it (R6-14)\", () => {\n"
              "    // ⛔ ROW HOME OF \"IL MODELLO DELLA GUI\": the nucleus has a phrase of its own, the other tiles\n"
              "    // say who fills them. A placeholder that treated the centre of Home like any tile passed every\n"
              "    // probe and read \"arriva col sotto-progetto 6\" in the browser.\n"
              "    const params = placeholderParams(\"knowledge\");\n"
              "    const wrapper = mount(Placeholder, { global: { plugins: [i18n] }, props: { params } });\n"
              "    expect(wrapper.text()).toContain(i18n.global.t(\"placeholder.nucleus\"));\n"
              "    expect(wrapper.text()).toContain(String(params.who));\n"
              "  });\n\n"
              "  it(\"says a type that is GONE is gone, does not promise a sub-project, and closes\", async () => {\n")
    seg = sub(seg,
              "    // ⛔ AND THE BUFFERED `onDidLayoutChange` THAT FOLLOWS DOES NOT SAVE IT AGAIN: the baseline\n"
              "    // moved with the save.\n    expect(saves(bridge)).toBe(1);\n  });\n});\n\ndescribe(\"the band\", () => {\n",
              "    // ⛔ AND THE BUFFERED `onDidLayoutChange` THAT FOLLOWS DOES NOT SAVE IT AGAIN: the baseline\n"
              "    // moved with the save.\n    expect(saves(bridge)).toBe(1);\n  });\n\n"
              "  it(\"leaves the strip's params alone: only what nobody built gets them from the registry (R6-17)\", async () => {\n"
              "    const bridge = createFakeBridge();\n"
              "    useLayout().attach(bridge);\n"
              "    const api = createDock(host());\n"
              "    await flush();\n"
              "    // ⛔ THE STRIP CARRIES NO PARAMS AND MUST NOT GET `{ missing: true }`: before R6-17 it did, at\n"
              "    // every `apply`, and the value entered the package at the first settle -- unseen, because\n"
              "    // `Strip.vue` ignores its params.\n"
              "    expect(api.getPanel(\"strip\")?.params ?? {}).toEqual({});\n"
              "    // And a module type nobody built still carries its own, from the registry.\n"
              "    expect(api.getPanel(\"knowledge\")?.params).toEqual(placeholderParams(\"knowledge\"));\n"
              "  });\n});\n\ndescribe(\"the band\", () => {\n")

    # ---- Passo 18 (R6-6) ---------------------------------------------------------------------------------------------
    seg = sub(seg,
              "npx vitest run src/locales/copy.test.ts; echo \"verde atteso: EXIT=$?\"\npython - <<'EOF'\nimport io\np = \"src/frame/Band.vue\"\n",
              "npx vitest run src/locales/copy.test.ts; echo \"verde atteso: EXIT=$?\"\ngit add src/frame/Band.vue\npython - <<'EOF'\nimport io\np = \"src/frame/Band.vue\"\n")
    seg = sub(seg,
              "Atteso: **`EXIT=0`** la prima volta, **diverso da zero** la seconda, e `git diff` **vuoto** alla fine.\n",
              "Atteso: **`EXIT=0`** la prima volta, **diverso da zero** la seconda, e `git diff` **vuoto** alla fine. ⛔ **Il `git add`\n"
              "prima della mutazione non è un vezzo — R6-6, misurato il 2026-09-16 in un repo di prova:** a questo passo `Band.vue` è\n"
              "**non tracciato** (il `git add gui` è al Passo 19), e su un file non tracciato `git checkout --` esce **1** con\n"
              "*«pathspec … did not match any file(s) known to git»*, la mutazione **resta** e `git diff` è vuoto lo stesso: l'Atteso\n"
              "passava vacuo su un file rotto. In scena, `checkout --` rimette la copia dell'indice — la forma di R5-11 e del Passo 3\n"
              "del 14, non un salvataggio a mano.\n")

    # ---- the closing criterion (R6-13, D90, R6-17, R6-4, R6-14, R6-16) ------------------------------------------------
    seg = sub(seg,
              "- [ ] `grep -c 'from \"dockview\"' gui/src/**/*.ts` → **0**: l'API è quella di `dockview-core`, il pacchetto ombrello entra solo per il CSS\n",
              "- [ ] `grep -rc 'from \"dockview\"' gui/src --include='*.ts' --include='*.vue' | grep -v ':0$'` → **niente**: l'API è quella "
              "di `dockview-core`, il pacchetto ombrello entra solo per il CSS (R6-13: la forma di R7-6, due righe sotto — senza "
              "`globstar` il `**` non scende di un livello e `src/main.ts` sfuggiva, e con più file `grep -c` stampa `file:conteggio`, "
              "mai uno «0»)\n")
    seg = sub(seg,
              "  Atteso: **`diversi: true`**. ⛔ **Se fossero uguali il timbro verrebbe dal posto sbagliato**, e la stretta di mano sarebbe una formalità\n",
              "  Atteso: **`diversi: true`**. ⛔ **Se fossero uguali il timbro verrebbe dal posto sbagliato**, e la stretta di mano sarebbe una formalità\n"
              "- [ ] ⛔ **il timbro raggiunge la SPA in tutti e tre i mondi (D90)** — con `STAMP` il numero decimale che il comando qui sopra stampa: "
              "alla build, `cat gui/dist/assets/*.js | grep -c \"$STAMP\"` → **1** e `cat gui/dist/assets/*.js | grep -c 'stamp 0x'` → **0** "
              "(la mappa non è nel bundle); sotto `vitest`, la sonda `sends Hello with the stamp from the map` è verde; in sviluppo, con "
              "`npm run dev` su (la porta è quella che stampa), `curl -s \"http://localhost:5173/@vite/env\" | grep -c \"$STAMP\"` → **1** e "
              "`curl -s -o /dev/null -w '%{http_code}\\n' http://localhost:5173/` → `200` — è il controllo che `npm run build` non fa (R6-3)\n")
    seg = sub(seg,
              "le due sonde di `describe(\"the dock\")` in `frame.test.ts` e le due nuove di `describe(\"the layout\")`",
              "le tre sonde di `describe(\"the dock\")` in `frame.test.ts` (la terza è R6-17) e le due nuove di `describe(\"the layout\")`")
    seg = sub(seg,
              "- [ ] ⛔ **il revisore apre la SPA nel browser e GUARDA** — regola 5 della testa: `cd gui && npm run dev`, e con la finta "
              "collegata si vedono la barra con le tre viste, la fascia «il core non ha risposto», la striscia in basso, il cassetto coi "
              "diciotto tipi, e le tessere che dicono chi le riempie\n",
              "- [ ] ⛔ **il revisore apre la SPA nel browser e GUARDA** — regola 5 della testa: `cd gui && npm run dev`, e con la finta "
              "collegata si vedono la barra con le tre viste, la fascia «il core non ha risposto», la striscia in basso, il cassetto coi "
              "diciotto tipi, e le tessere che dicono chi le riempie — **coi nomi italiani sulle linguette e nel cassetto** (R6-4), il "
              "**nucleo** al centro di Home che dice «niente ancora» (R6-14), e il cassetto aperto con un **velo** scuro sopra la "
              "griglia (R6-16). ⚠️ **La pagina bianca di R6-3 si vedeva solo qui:** `npm run build` non la coglie\n")

    # ---- guards, computed on the segment ---------------------------------------------------------------------------
    assert seg.count("Passo 16") == 1, seg.count("Passo 16")  # the step's own title
    assert seg.count('"types": ["vite/client", "node"]') == 3, seg.count('"types": ["vite/client", "node"]')  # Files, prose, Python
    assert seg.count("isBuilt") == 5, seg.count("isBuilt")  # Interfaces, the function, the import, `apply`, the paragraph
    assert seg.count("import MAP from") == 1  # the quoted old line in the recall of Passo 6
    assert seg.count("__BUILD_STAMP__") == 3, seg.count("__BUILD_STAMP__")  # `define`, `declare const`, `return`
    assert seg.count('"nucleus": "niente ancora"') == 1
    assert seg.count('"modules": {') == 1
    assert 'class="drawer-open"' not in seg
    assert seg.count("--input-type=module") == 2, seg.count("--input-type=module")
    assert seg.count("git add src/frame/Band.vue") == 1
    assert seg.count("setupFiles") == 5, seg.count("setupFiles")  # Files, the block, the fake's comment, the recall, the table
    assert seg.count("R6-") >= 30, seg.count("R6-")
    return seg


t = scoped(t, "\n## Compito 13:", "\n## Compito 14:", task13)


# ---- P-97: the attribution of the Passo 3 outcome (R6-8) -------------------------------------------------------------
def p97(seg):
    return sub(seg,
               "e non saprebbe mai se il vicino scelto è il più\n**vicino**.\n\n✅ **La cura non è un `dockview` finto:",
               "e non saprebbe mai se il vicino scelto è il più\n**vicino**.\n\n"
               "⛔ **RICHIAMO DEL 2026-09-16, dalla revisione in profondità del compito 13 (R6-8): la premessa *«Il Passo 3 del 13 ha\n"
               "misurato che `jsdom` non fa layout»* attribuiva al Passo 3 un esito che quel passo non produceva** — la sua sonda\n"
               "moriva prima, su `ReferenceError: ResizeObserver is not defined`, perché `jsdom` 30.0.1 non implementa quell'API; con\n"
               "la finta di `gui/src/jsdom-setup.ts` la griglia si monta. Il merito resta vero e misurato: sotto `jsdom`\n"
               "`getBoundingClientRect` rende zeri, e la sonda di `moveActive` si scrive coi rettangoli nostri come questa voce detta.\n\n"
               "✅ **La cura non è un `dockview` finto:")


t = scoped(t, "\n### P-97 ", "\n### P-98 ", p97)


# ---- task 14: the tokens.css paragraph is written by task 13 now (R6-11) --------------------------------------------
def task14(seg):
    seg = sub(seg,
              "- Modify: `gui/src/tokens/tokens.css` (**LF**) — `--stop`, l'unico token che l'AA boccia (**P-86**), il capoverso in "
              "testa (Passo 3) e le regole della linguetta in coda (Passo 11) — R7-9\n",
              "- Modify: `gui/src/tokens/tokens.css` (**LF**) — `--stop`, l'unico token che l'AA boccia (**P-86**), e le regole della "
              "linguetta in coda (Passo 11) — R7-9 (⚠️ qui stava anche «il capoverso in testa (Passo 3)»: dal 2026-09-16 lo scrive il "
              "**13** nella forma di D53, R6-11, e il Passo 3 lo verifica)\n")
    lo, hi = between(seg, "⚠️ **E il paragrafo in testa al file che dice *«the check is `axe-core` … it arrives with the accessibility of task\n",
                     "   shape comes before the palette. */\n```\n")
    old = seg[lo:hi]
    assert "diventano:" in old and old.count("```css") == 1, old[:200]
    seg = seg[:lo] + (
        "✅ **Il capoverso in testa al file è già nella forma di D53 — lo scrive il compito 13 (R6-11, 2026-09-16), e qui si\n"
        "verifica invece di riscriverlo:** `grep -c -F 'INCOMPLETE every time' gui/src/tokens/tokens.css` → **1**, e\n"
        "`grep -c 'on the mounted components' gui/src/tokens/tokens.css` → **0**. ⚠️ Qui stava la riscrittura delle\n"
        "**quattro** righe da `⚠️ G20 WANTS AA CONTRAST` a `reason the shape comes before the palette. */` (R7-5), che nel commit del\n"
        "13 dicevano che il contrasto lo prova `axe-core`; il testo che questo passo dettava è ora quello del Passo 4 del 13, parola\n"
        "per parola, e `contrast.test.ts` — che quel capoverso nomina — nasce qui sotto.\n"
    ) + seg[hi:]
    return seg


t = scoped(t, "\n## Compito 14:", "\n## Compito 15:", task14)


# ---- task 15: the modules keys are dictated at Passo 5 of task 13 (R6-4) --------------------------------------------
def task15(seg):
    return sub(seg,
               "(compito **13**, Passo 12 — ⚠️ qui stava «14», R7-7)",
               "(compito **13**, Passo 5 — ⚠️ qui stava «14», R7-7, e poi «Passo 12»: le chiavi sono dettate per esteso al Passo 5 dal "
               "2026-09-16, R6-4)")


t = scoped(t, "\n## Compito 15:", "\n## Compito 16:", task15)

# ---- D90 -------------------------------------------------------------------------------------------------------------
t = sub(t,
        "Compilava e passava tutte le sonde del 13 |\n\n**La baseline di partenza, misurata il 2026-09-11",
        "Compilava e passava tutte le sonde del 13 |\n"
        "| **D90** | ⛔ **il timbro entra nella SPA da `define` di `vite.config.ts`, che legge l'ultima riga di `ipc_v1.map` in Node; "
        "il browser non chiede mai un `.map`**, e `stamp.ts` dichiara la costante (`declare const __BUILD_STAMP__`) invece di importare "
        "la mappa | **R6-3**, misurato nel browser e sul modello compilato il 2026-09-16: il server di sviluppo di `vite` 8.3.0 tratta "
        "ogni URL che finisce in `.map` come una richiesta di source map e serve il file statico come `application/json` — `npm run dev` "
        "pagina bianca, `vite build` e `vitest` verdi, cioè un difetto **muto nel cancello**. Le vie scartate: rinominare la mappa "
        "spezza la forma che `record_v1.map` del giornale ha già e tocca il compito 3 in tredici punti; un secondo file col solo timbro "
        "scritto dal generatore sarebbe una seconda casa del timbro (gotcha #68) e riaprirebbe il 3, rivisto in profondità da R11; un "
        "`fetch` a tempo d'esecuzione renderebbe `buildStamp` asincrona e non gira sotto `vitest`; un middleware di sviluppo che serva "
        "il `.map?raw` come modulo sarebbe una seconda implementazione di `?raw`, accoppiata al formato degli URL di `vite`. `define` "
        "è la via documentata di `vite` per una costante di build, e vale in tutti e tre i mondi: sostituita nel bundle, iniettata "
        "come globale in sviluppo (`/@vite/env`) e sotto `vitest` — misurato. ⚠️ **Costo dichiarato:** la lettura vive nella "
        "configurazione e non in `stamp.ts`, e in sviluppo il server va riavviato quando la mappa cambia — che accade solo quando "
        "cambia lo schema del kernel, e allora la SPA si ricostruisce comunque; che il timbro arrivi nei tre mondi lo misura il "
        "criterio di chiusura del 13 |\n"
        "\n**La baseline di partenza, misurata il 2026-09-11")

# ---- the open-items table: «il resto spento» (C13-2, from R6's coverage table) --------------------------------------
t = sub(t,
        "le tre voci aspettano lo stesso pezzo, che la §8 del 2 mette *«fuori dal cancello di oggi»* |\n",
        "le tre voci aspettano lo stesso pezzo, che la §8 del 2 mette *«fuori dal cancello di oggi»* |\n"
        "| ⛔ **«il resto spento»** della riga *i quattro stati della connessione* della §6a del 2 — *core non in esecuzione → una "
        "fascia che lo dice e un pulsante «riprova», il resto spento* — **nessun passo del 13 o del 14 spegne nulla**: nel 2 i moduli "
        "senza dati mostrano lo stato vuoto (`—`, «nessuna richiesta») e la griglia resta manovrabile, perché la disposizione è "
        "presentazione (I1) e non dipende dal core; **P-78** e **D48** discutono i quattro stati e non nominano questa metà (tabella "
        "di copertura di R6, 2026-09-16) | la riga della §6a, e la tabella di copertura di `R6-report.md` | il **proprietario**, in "
        "A/B alla prossima rilettura: **(A)** «spento» è lo stato vuoto di oggi, e la riga della §6a riceve il richiamo datato dal 14, "
        "che costruisce i moduli; **(B)** «spento» è una disabilitazione della griglia (`inert`) finché `Accepted` non arriva — un "
        "passo nuovo nel 13. Il consiglio del coordinatore è **A**: una griglia bloccata terrebbe ferma la disposizione, che il core "
        "non decide |\n")

# ---- whole-plan guards ---------------------------------------------------------------------------------------------------
assert t.count("\n### P-") == raw.count("\n### P-") == 116
assert t.count("| **D90** |") == 1
d_rows = [l for l in t.split("\n") if l.startswith("| **D") and l.split("**")[1][1:].isdigit()]
assert len(d_rows) == 90, len(d_rows)

# ---- the ledger ----------------------------------------------------------------------------------------------------------
lraw = io.open(LEDGER, encoding="utf-8", newline="").read()
assert "\r\n" not in lraw
l = lraw
l = sub(l,
        "Mancano: compito 13 (R6), compiti 15–17 (R8) — perimetri da rivedere ancora; il compito 3 è rivisto da R11 e l'8 da R12, "
        "entrambi il 2026-09-16.",
        "Mancano: compiti 15–17 (R8) — perimetro da rivedere ancora; il compito 3 è rivisto da R11, l'8 da R12 e il 13 da R6, tutti "
        "il 2026-09-16.")
l = sub(l,
        "con la sonda nella forma nuova. Nel registro non resta nessun ⬜.\n",
        "con la sonda nella forma nuova. Nel registro non resta nessun ⬜. **Ondata 16 (2026-09-16):** la revisione in profondità del "
        "13 è FATTA (R6, un revisore solo su Opus 5: ~401k token, 125 chiamate, ~40 minuti) e i suoi diciassette rilievi confermati sono "
        "applicati (✅) con `patch_c13b.py`, con due voci del coordinatore dalla sua tabella di copertura, il richiamo in P-97, i "
        "riallineamenti nel 14 e nel 15, **D90** e una riga nelle voci aperte; il rapporto è `R6-report.md` accanto; l'ondata intera è "
        "stata applicata al modello compilato di R6 e misurata prima di essere dettata. Nel registro non resta nessun ⬜.\n")
l = sub(l,
        "### Compito 13 — NON RIVISTO IN PROFONDITÀ (R6 caduto): le righe note sono applicate, la revisione in profondità resta da fare\n",
        "### Compito 13 — RIVISTO IN PROFONDITÀ da R6 il 2026-09-16 (Opus 5; il primo R6 era caduto): diciassette rilievi confermati, "
        "tutti applicati, e due del coordinatore\n")
l = sub(l,
        "- ⚠️ **Non compilato:** `layout.ts`, `dock.ts`, `Frame.vue` e le sonde sono riscritti sul modello letto (il compito 11 non "
        "esiste nel repo, e `dockview` sotto `jsdom` lo misura il Passo 3): chi esegue il 13 li compila per primi, e ogni rosso è una "
        "voce d'errata — come il modulo delle sonde del 12\n",
        "- ✅ **Compilati il 2026-09-16:** R6 ha ricostruito il modello dell'11 e del 13 (`C:\\Users\\zagor\\AppData\\Local\\Temp\\probe-R6\\gui`, "
        "percorso corto perché lo scratchpad supera MAX_PATH per `npm` e `vite`) e il coordinatore vi ha applicato l'ondata 16 intera "
        "prima di dettarla: `vue-tsc` zero errori, 23 sonde verdi e 1 saltata, il generatore verde con tre viste senza `\"missing\"`, "
        "`vite build` col timbro decimale nel bundle, `npm run dev` che serve la pagina col timbro in `/@vite/env` e zero errori in "
        "console; le due sonde nuove uccise ciascuna dalla propria mutazione (qui stava «Non compilato … chi esegue il 13 li compila "
        "per primi»)\n")
l = sub(l,
        "- R7-6 vale anche qui (stesso criterio `gui/src/**/*.test.ts`): applicata dall'ondata del 14 (`patch_c14.py`) ✅\n",
        "- R7-6 vale anche qui (stesso criterio `gui/src/**/*.test.ts`): applicata dall'ondata del 14 (`patch_c14.py`) ✅\n"
        "- R6-1 Passo 2 e Files: `@types/node` **24.13.5** fra le `devDependencies` (la major segue Node: `npm view @types/node@24 version "
        "| tail -1`, non `latest`, che indica la 22) e `\"types\": [\"vite/client\", \"node\"]` in `gui/tsconfig.json` con Python — senza, "
        "`npm run build` esce 2 con otto `TS2307`/`TS2591` (sei `import` da `node:` nel 13, i primi del piano) ✅ · R6-2 Passo 13: "
        "`setActivePinia(createPinia())` in un `beforeAll` di `generate-views.test.ts`, col perché (`VueContent` monta ogni pannello "
        "senza pinia, e qui non c'è una root app) ✅ · R6-3 Passo 6 riscritto (**D90**): il timbro lo legge `vite.config.ts` in Node e "
        "lo consegna con `define`; `stamp.ts` lo dichiara e non importa più la mappa; Files, Interfaces, e il criterio che lo prova nei "
        "tre mondi (bundle, `vitest`, `/@vite/env` in sviluppo) — misurato dal coordinatore sul modello di R6 e nel browser: pagina "
        "aperta, zero errori in console ✅ · R6-4 Passo 5: le diciotto chiavi `modules.*` dettate per esteso coi nomi della §1 "
        "(«Knowledge base» per la riga «Knowledge base e Nucleo a pagina intera»); il capoverso del Passo 12 rimanda al 5; la riga "
        "*Consumes* del 15 dice «Passo 5» ✅ · R6-5 Passo 13: `node --input-type=module -e \"import * as v from 'vitest'; …\"` ✅ · "
        "R6-6 Passo 18: `git add src/frame/Band.vue` prima della mutazione — la forma di R5-11 e del Passo 3 del 14, non il Python che "
        "R6 proponeva (coerenza col repo) — e l'Atteso che dice perché ✅ · R6-7 Passo 3: «Passo 17» nelle due celle e nel capoverso "
        "📌 ✅ · R6-8 Passo 3: la terza riga della tabella, `gui/src/jsdom-setup.ts` (Create) montato da `setupFiles`, il richiamo che "
        "dice che cosa la sonda misura davvero; il richiamo in P-97 sull'attribuzione ✅ · R6-9 Passo 13: il capoverso di "
        "`views/index.ts` dice che l'`as` è un'asserzione portante che coglie l'incompatibilità grossolana, e che la prima sonda di "
        "`frame.test.ts` copre il resto ✅ · R6-10 Passo 11: il commento di `BigTab.ts` dichiara la trappola per il 14 invece di "
        "affermare che i due eventi sono fermati ✅ · R6-11 Passo 4: il capoverso di `tokens.css` già nella forma di D53 (il testo che "
        "il 14 dettava); il Passo 3 del 14 lo **verifica** e non lo riscrive, la riga *Files* del 14 perde «il capoverso in testa» ✅ · "
        "R6-12 Passo 1: «tre righe `/gui/`», e da chi ✅ · R6-13 criterio: `grep -rc … --include='*.ts' --include='*.vue' | grep -v "
        "':0$'` → niente ✅ · R6-14 Passo 12: `Placeholder.vue` dice «niente ancora» sul nucleo (`knowledge`, riga Home de «Il modello "
        "della GUI») e ancora chi lo riempie; la chiave `placeholder.nucleus` al Passo 5; la sonda in `frame.test.ts` e il criterio "
        "del browser — misurato nelle due direzioni ✅ · R6-15 Passo 9: «the opening paragraph of §2» ✅ · R6-16 Passo 15: il velo "
        "(`.drawer-overlay`: `fixed`, `inset: 0`, `rgb(0 0 0 / 0.45)`, `z-index: 100`) e `z-index: 101` sul cassetto; via la classe "
        "`drawer-open` senza regola — visto nel browser ✅ · R6-17 Passo 14: `isBuilt` nel registro (Interfaces) e `!isBuilt(panel.id) "
        "&&` in `apply`, col perché; la terza sonda di `describe(\"the dock\")`; il criterio `grep -c '\"missing\"'` sulle viste → 0 — "
        "misurato nelle due direzioni ✅ · R6-18…R6-26 NON RIPRODOTTO: nulla da fare, e il coordinatore non le ricerca · Attrezzo: "
        "`patch_c13b.py`\n"
        "- ⛔ **C13-2 (coordinatore, dalla tabella di copertura di R6): «il resto spento» della §6a non è prodotto da nessun passo** — "
        "registrata nella tabella *«Le voci aperte che questo piano SA, e non chiude»* con l'A/B per il proprietario (consiglio A: lo "
        "stato vuoto di oggi) ✅\n"
        "- Le P-117… per questi rilievi restano da scrivere (decisione 94)\n")
l = sub(l,
        "- Attrezzo: `patch_c14.py` accanto a questo file (tocca anche il criterio del 13, la riga D87 e questo registro)\n",
        "- Attrezzo: `patch_c14.py` accanto a questo file (tocca anche il criterio del 13, la riga D87 e questo registro)\n"
        "- R6-11 (2026-09-16, dalla revisione in profondità del 13): il capoverso in testa a `tokens.css` lo scrive già il 13 nella "
        "forma di D53; il Passo 3 lo verifica col `grep -c -F` e non lo riscrive, e la riga *Files* perde «il capoverso in testa "
        "(Passo 3)» (`patch_c13b.py`) ✅\n")
l = sub(l,
        "- Attrezzo: `patch_c151617.py` accanto a questo file\n\n### Compito 16 —",
        "- Attrezzo: `patch_c151617.py` accanto a questo file\n"
        "- R6-4 (2026-09-16, dalla revisione in profondità del 13): la riga *Consumes* dice «compito 13, Passo 5» — le chiavi "
        "`modules.*` sono dettate per esteso al Passo 5, non al 12 (`patch_c13b.py`) ✅\n\n### Compito 16 —")
open_lines = [line for line in l.splitlines() if " ⬜" in line]
assert len(open_lines) == 1, len(open_lines)  # the status paragraph only, which keeps the history

for path, content in ((PLAN, t), (LEDGER, l)):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(content)
    os.replace(tmp, path)
print("ok: task 13 patched (R6-1..17, C13-2), P-97 recalled, tasks 14 and 15 realigned, D90, the open-items row, and the ledger;",
      t.count("\n") - raw.count("\n"), "plan lines added")

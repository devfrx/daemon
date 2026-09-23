# R3 — il compito 4 (la pagina kit), il compito 5 (il kit al lavoro) e il «Come si riprende»

- **Fetta:** righe **2661–3298** (compito 4) e **3299–4012** (compito 5) del piano, e il *«Come si riprende»* per intero
  (righe **4013–4028**).
- **Cartella di prova:** `/c/Users/zagor/AppData/Local/Temp/probe-ds3` (in Windows `C:\Users\zagor\AppData\Local\Temp\probe-ds3`).
- **Rapporto:** `C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\b8ece6f2-cc9f-436d-a426-c59258eeeb68\scratchpad\review\R3.md`

## Il mandato

1. **Il compito 4 — la pagina kit.** Come `vite` 8.3.0 costruisce (o lascia fuori) `kit.html`, e la prova sull'uscita del
   *build* che il compito mette in `scripts/gate-gui.sh`; le tre sonde delle tavole (`sonda-raggi.js`,
   `sonda-caratteri.js`, `sonda-icone.js`, nella cartella delle tavole) diventate funzioni in `testing/probes.ts`:
   l'algoritmo è quello delle tavole? le guardie di non-vacuità ci sono? `axe` col contrasto acceso nel browser; il blocco
   del linter delle parole esemplari (D8).
2. **Il compito 5 — i pezzi di base nei pannelli e nella cornice.** Per ogni file di `gui/src/panels/` e `gui/src/frame/`
   che il compito riscrive: il testo dettato parte dal codice di **oggi** (ciò che sostituisce esiste com'è citato)? Le
   prove di `modules.test.ts` e `a11y.test.ts` riscritte perdono qualcosa di ciò che provano oggi (conta gli `it` prima e
   dopo, e guarda che cosa asseriscono)? P-8, il radio controllato di `reka-ui` 2.10.4 (`RadioGroupItem.js`, `Radio.js`);
   `layout.chooseTheme` — lo crea il compito 1 (cerca nel testo del compito 1) o nessuno? `it.json`; le regole del linter
   su `panels/` e `frame/`; il passo 8, *«M-3 col lettore di schermo vero — a mano»*: che cosa chiede, ed è coerente con la
   decisione 21 del disegno?
3. **Le dipendenze dai compiti 1–3.** I compiti 4 e 5 usano i token, i pezzi di base, `testing/axe.ts`, il browser dei
   test. Per compilare, applicali nella cartella di prova dal testo del piano (righe 219–2660) quanto basta; se costa
   troppo, verifica a lettura contro le API dei pacchetti installati e contro i blocchi *Interfaces* (righe 235, 1185, 1509,
   2673, 3314), e dichiaralo. Ciò che provi davvero vale più di ciò che leggi.
4. **Il «Come si riprende».** Le righe 2–5 dicono ciò che i compiti 2–5 fanno davvero? ⚠️ **Le righe 6–9 sono le forme dei
   compiti ancora da scrivere, e il prossimo scrittore le userà come base: un errore qui si propaga a quattro compiti.**
   Sono coerenti con le interfacce prodotte dai compiti 1–5, col disegno (c) e (d), e il codice di oggi le permette?
   Verifica nei file spediti: `DockviewTheme`, `api.updateOptions`, `api.layout`, `SerializedDockview` di `dockview-core`
   8.3.1; `gui/src/stores/layout.ts` e il suo `unpack`; `gui/src/**/moveActive.ts`; `PANEL_TYPES`; la presa grande
   (`BigTab`) di oggi.

# R1 — la testa e il compito 1 (i token)

- **Fetta:** righe **1–218** (la testa) e **219–1172** (il compito 1) del piano; la riga 1 del *«Come si riprende»*.
- **Cartella di prova:** `/c/Users/zagor/AppData/Local/Temp/probe-ds1` (in Windows `C:\Users\zagor\AppData\Local\Temp\probe-ds1`).
- **Rapporto:** `C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\b8ece6f2-cc9f-436d-a426-c59258eeeb68\scratchpad\review\R1.md`

## Il mandato

1. **La testa.**
   - Rilancia **ogni** comando della colonna *«Il comando o il file»* di **P-1..P-14** sul codice di oggi e confronta con
     ciò che la riga afferma. (P-1 cita `palette.py` in uno scratchpad: esiste in
     `C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\9ae312b8-b35c-470a-90ee-519a47a2f7df\scratchpad\palette.py`.)
   - La *«Pila»* contro `gui/package.json`; le versioni nuove al registro (`npm view <p>@<v> version`).
   - Le **D1–D8** contro il disegno e il codice: non si riaprono, ma una che contraddice un fatto si segnala.
   - Le *«Voci aperte che questo piano SA»*: esistono dove dicono (N-2 di E187, E228: `grep` a sezioni nel piano
     `docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md`, che è enorme — mai intero)?
   - La **tabella della posizione** contro i compiti scritti. ⚠️ Già noto al coordinatore: la riga 1 dice *«i nomi nuovi nei
     dodici componenti»*, il compito dice **undici**, e `grep -rlE 'var\(--' gui/src --include='*.vue' | wc -l` rende 11 —
     confermalo o smentiscilo, e **cerca le altre di questa specie** in tutta la testa.
2. **Il compito 1, applicato per intero nella cartella di prova, alla lettera, passo per passo.** È l'unico compito
   autosufficiente sul codice di oggi, quindi è l'unico che si può provare per intero:
   - lo script che copia i due fogli dalla tavola: `python -m py_compile`, poi eseguilo davvero sulla tavola;
   - le prove dei passi 3–7 devono fallire al passo 8 **per la ragione che il piano dice** (non per un errore d'import
     qualunque, salvo che il piano dica proprio quello); poi i passi 9–15 le fanno verdi;
   - `npx vue-tsc --noEmit`, `npx vitest run`, `npx eslint src`, `npx vite build`: ogni divergenza fra ciò che il piano dice
     e ciò che succede è un rilievo, col testo corretto proposto;
   - i passi 16 (le due direzioni, i fine-riga) e 17 (guardarlo nei due temi: `npx vite` e il Chrome installato) se il
     budget lo permette; altrimenti dichiarali.
3. **La mappa dei nomi vecchi → nuovi** (disegno, *«Il passaggio dai nomi di oggi»*) contro **tutti** gli usi di oggi in
   `gui/src` — anche nei `.ts`, anche nel dock — e ogni nome nuovo deve esistere nei due fogli copiati dalla tavola.
4. **Il campo `theme` del pacchetto** (D2): `gui/src/stores/layout.ts` di oggi — `pack`/`unpack` — e il vincolo 12 (il
   kernel e `gui/schema/` non cambiano).
5. **La riga del compendio** che il compito 1 modifica esiste com'è citata? (`grep` in `docs/COMPENDIO.md`; è l'unico
   caso in cui apri il compendio, e solo col `grep`.)

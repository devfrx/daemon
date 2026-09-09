# Archivio — i testi usciti dalla lettura d'apertura, dal 2026-09-09

⛔ **Non è una lettura obbligatoria.** Verbali datati: i testi che il **mandato del proprietario** — decisione 26 della
[stella polare della GUI](../superpowers/specs/2026-09-07-direzione-gui-design.md), 2026-09-09: sfoltire la lettura
d'apertura, ogni taglio in A/B — ha fatto uscire dai documenti d'ingresso, `CLAUDE.md` e il messaggio di
[`AVVIO-CHAT.md`](../AVVIO-CHAT.md), **parola per parola**, con la data e il taglio che li ha prodotti. Ciò che esce dalla
§6 del compendio sta in [`stato-storico.md`](stato-storico.md), come dal 2026-08-28; la cronaca della stella polare in
[`consegna-brainstorming-direzione-gui.md`](consegna-brainstorming-direzione-gui.md).

⚠️ **Ciò che è scritto qui era vero il giorno in cui fu scritto.** La lettura d'apertura viva la dice
[`CLAUDE.md`](../../CLAUDE.md); il prossimo passo sta nella §6 di [`COMPENDIO.md`](../COMPENDIO.md), in un posto solo.

## Taglio 2 — la testa dell'audit del 2026-08-27 · approvato A il 2026-09-09

📌 **La misura del giorno, coi comandi che la rifanno.** La testa fino a «Dettaglio» —
`awk '/^## Dettaglio/{exit} {b+=length($0)+1} END{print b}' docs/audit-2026-08-27.md` — pesava **82 907 byte**
(27 475 token `cl100k`, limite inferiore), di cui **36 121** «Stato dei rimedi» e **19 591** «I 73 finding» (il comando per
blocco sta nel punto 1 del prossimo passo della stella polare); ciò che resta da leggere all'apertura — le due sezioni
nominate in `CLAUDE.md` — lo dà
`awk '/^### Le voci aperte che NON/||/^### La disciplina/{p=1} /^### Le decisioni prese/||/^### Le trappole/{p=0} p{b+=length($0)+1} END{print b}' docs/audit-2026-08-27.md`,
che il giorno del taglio rendeva **4 806**.

### `CLAUDE.md`, «Prima cosa, e unica lettura obbligatoria» — il capoverso sull'audit com'era

⛔ **E la testa di [`docs/audit-2026-08-27.md`](../audit-2026-08-27.md) — tutto ciò che
sta PRIMA della sezione «Dettaglio».** ⚠️ **RICHIAMO DEL 2026-08-28: queste righe dicevano
*«Quel rapporto È APERTO ED È IL COMPITO DI OGGI, non una consultazione»*, e i suoi finding
sono chiusi.** Resta lettura d'apertura come **verbale e delega** — la sua sezione *«Come si
concludono quelli aperti»* è **il metodo con cui qui si rimedia**, e la colonna *«Stato»*
della sua tabella è la **casa unica** di che cosa resta, insieme alla tabella delle **voci
senza numero AUD**, che sono le sole ancora aperte e in gran parte del proprietario.
⛔ **Il «Dettaglio» dopo quella tabella NON si legge intero: se ne apre UNA scheda per
volta, quella del finding su cui si lavora.**

### `docs/AVVIO-CHAT.md`, il messaggio da incollare, voce 3 — com'era

```
  3. docs/audit-2026-08-27.md — ⛔ SOLO FINO ALLA TABELLA DEI 73 FINDING,
     cioè tutto cio' che sta PRIMA della sezione «Dettaglio».
     ⚠️ RICHIAMO DEL 2026-09-01: qui stava «54 KB», e sono 81 — la testa è
     cresciuta a ogni rimedio, perché è lì che vive la tabella dei rimedi.
     TOLTO e non riallineato: un peso misurato lo dà il comando, che non
     marcisce — awk '/^## Dettaglio/{exit} {b+=length($0)+1} END{print b}'
     docs/audit-2026-08-27.md
     ⚠️ I suoi finding sono CHIUSI dal 2026-08-28 — questa riga diceva «È IL
     COMPITO DI OGGI, non un verbale». Si legge come VERBALE e DELEGA: il
     metodo con cui qui si rimedia, e le voci senza numero AUD, che sono le
     sole ancora aperte.
     Contiene: come è stato condotto, la copertura, la baseline, le SETTE
     radici, lo stato dei rimedi, la sezione «Come si concludono quelli
     aperti» — che è LA DELEGA, con la disciplina in cinque passi, le due
     trappole del repository e l'ordine consigliato — e la tabella dei 73
     con la colonna «Stato», che è la loro CASA UNICA.
     ⛔ IL «Dettaglio» DOPO QUELLA TABELLA NON SI LEGGE INTERO: sono 73
     schede da ~20 righe, 204 KB, e se ne apre UNA per volta, quella del
     finding su cui stai lavorando. Si legge a FINDING, come un piano si
     legge a compiti.
```

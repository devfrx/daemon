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

## Taglio 5 — i verbali dentro `CLAUDE.md` · approvato A il 2026-09-09

⛔ **Il taglio 5 del mandato (decisione 26), approvato A il 2026-09-09, tredicesima ripresa (decisione 31 della
[stella polare della GUI](../superpowers/specs/2026-09-07-direzione-gui-design.md)):** i verbali dentro la testa e
dentro le celle delle tabelle di `CLAUDE.md` escono; ogni regola resta, col suo perché in una riga e il rimando «il
verbale in archivio». Misurato prima di chiedere: 2 173 byte e 753 token `cl100k` su 15 728 byte e 5 019 token (LF),
meno dell'1% della lettura d'apertura — contro la stima *«a ~10 KB, ~2 000 token»* della dodicesima ripresa, e la
divergenza è stata detta al proprietario prima della domanda. I pezzi com'erano al commit `b0901e4`, **parola per
parola**, coi soli link riscritti per questa cartella, e per ciascuno dove stava:

### In testa — le sei volte del gotcha #31, dentro la riga sul costo della lettura

Stava fra *«ed è una decisione.»* e *«Ora lo dice il comando, che non marcisce:»*.

Ogni volta che
un peso o un conto di token è stato scritto su questa riga è invecchiato: ha detto
«seimila token» a 24 KB, «88 KB» a 91, «165» a 192, «624» a 213. **Sei volte, il gotcha
**#31**.**

### In testa — la riga su `docs/audit-2026-08-11.md`

Stava fra il comando `tiktoken` e il capoverso *«Non aprire»*.

⚠️ **E [`docs/audit-2026-08-11.md`](../audit-2026-08-11.md) è uscito da questa riga il
2026-08-27:** è chiuso otto decisioni su otto, e ciò che insegnava vive nel compendio —
un file chiuso non è una lettura obbligatoria.

### La riga `superpowers:writing-plans` della tabella delle skill — il richiamo del 2026-08-30

Stava fra *«quando si scriverà il piano.»* e *«**si SANNO prima di scrivere**»*.

⛔ **RICHIAMO DEL 2026-08-30: qui stava *«non prima che le voci aperte siano chiuse»*, ed era INSODDISFACIBILE** — fra le voci aperte ce ne sono con chiusore *«il traguardo della ritenzione»* o *«nessuno finché nessuna misura lo chiede»*, quindi nessun piano avrebbe più potuto essere scritto. **E la pratica la violava da due piani su due:** le voci raccolte il 2026-08-10 *«perché chi riprende deve saperle PRIMA di scrivere»* erano aperte quando furono scritti i piani del **Traguardo 4** (2026-08-11) e del **Traguardo 5** (2026-08-18), e lo sono ancora. La regola vera è quella che la raccolta stessa enuncia:

### La riga dei fine-riga di «Come si lavora qui» — le tre volte e il `sed -i`

Stava dopo *«che nessuno ha toccato»*.

— successo **tre volte**, l'ultima il 2026-08-18 con un `sed -i` su `crates/kernel/Cargo.toml`, **43 CR → 0**.

### La riga della dipendenza in due passi — la relazione «tutti»

Stava dopo *«a **tutti** i suoi siti `cargo`»*.

— *tutti* è la relazione che regge quando il cancello guadagna un passo, una cifra no; il comando che la verifica sta in [`docs/riferimenti.md`](../riferimenti.md), in una casa sola —

### La riga della dipendenza in due passi — «il punto e non il prezzo»

Stava prima di *«Finding **G-5**»*.

È il punto e non il prezzo: ADR-0031 chiama l'aggiunta di una voce *«un atto deliberato e rivedibile»*, e un lockfile che il cancello aggiornava da sé non era né l'uno né l'altro.

### Sotto le quattro domande del pre-controllo — il capoverso del 2026-08-28

Stava dopo la tabella delle righe 5–8, prima di «Manutenzione della documentazione».

⚠️ **Il testo lungo di queste quattro righe è stato tolto da qui il 2026-08-28**, non
perso: erano quattro paragrafi che ricopiavano i gotcha **#49**, **#58**, **#59** e **#65**
di [`docs/HANDOFF.md`](../HANDOFF.md), che ne è la casa. ⛔ **Ciò che è rimasto è
l'ISTRUZIONE**, perché questo file è l'unica lettura obbligatoria e `HANDOFF.md` non lo è:
una regola che sparisse di qui non verrebbe letta da nessuno. **Il caso** invece si apre
solo quando serve, ed è per questo che può vivere altrove.

### La riga «Uno schema è una verifica» — la prima applicazione

Stava in coda alla cella, dopo *«mai in silenzio»*.

La prima applicazione, la sezione 1 della passata sui diagrammi, ha trovato due difetti in nove righe

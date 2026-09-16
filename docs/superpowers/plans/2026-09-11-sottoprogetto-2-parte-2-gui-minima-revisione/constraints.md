# Vincoli per i revisori del piano della parte 2 del sotto-progetto 2 — 2026-09-15

## ⛔ RICHIAMO DEL 2026-09-16 — la ripresa della revisione: le righe qui sotto VINCONO sul resto del file

Questo file è del 2026-09-15 e resta valido **tranne** ciò che questa tabella corregge.

| Era, il 2026-09-15 | Oggi, 2026-09-16 |
|---|---|
| scratchpad `…\70220bef-5dc3-4d57-a0cd-7088a6828df6\scratchpad` | `C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\4fa001a2-dff2-4d38-9775-1dfa17966e6f\scratchpad` — in Git Bash `/c/Users/zagor/AppData/Local/Temp/claude/C--Users-zagor-Desktop-harness/4fa001a2-dff2-4d38-9775-1dfa17966e6f/scratchpad`. `review/skeleton.md` e `review/p-titles.md` sono lì, **rigenerati oggi dal piano corretto**; crea tu `review/probe-RN/` |
| `HEAD` = `baf3cde`; D = 74; il piano di 20 129 righe | `HEAD` = `b3422bf` (`git rev-parse --short HEAD`); `grep -c '^| \*\*D[0-9]' <piano>` → **89**; `wc -l <piano>` dà le righe di oggi. P resta **116**, i compiti **17**, l'errata **vuota**, nessun compito eseguito, `gui/` non esiste, il codice di prodotto identico a `42b50d8` (il comando della §2 resta vuoto) |
| il piano com'era prima della revisione | ⛔ **le correzioni della revisione (R1–R10) sono APPLICATE ai compiti 1–17** — il registro `docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione/ledger.md` elenca per compito ogni rilievo applicato (✅). Leggi la sezione del TUO compito nel registro (`awk '/^### Compito 13/{f=1} /^### Compito 14/{exit} f' ledger.md`): quelle righe sono già nel piano — le **verifichi** come tutto il resto, perché sono state scritte senza revisione; se reggono non le riproponi; se non reggono è un rilievo nuovo, con «era la riga R… del registro» nella cella «Che cosa dice il piano» |
| i moduli dettati si leggono | ⛔ **tre compiti portano moduli RISCRITTI dalla revisione e MAI COMPILATI**: il modulo delle sonde del 12 (`gui/fake-core/src/main.rs`, `mod tests`); `layout.ts`, `dock.ts`, `Frame.vue`, `stores.test.ts`, `frame.test.ts` del 13; `Frame.vue`, `Chat.vue`, `markdown.ts`, `modules.test.ts`, `chat.test.ts` del 14. Chi li ha nel perimetro li **compila** nella propria cartella di prova (il tuo `RN-prompt.md` dice come); un rosso è un rilievo `fatto`, `sì` — non un'errata, perché nessun compito è eseguito |
| «Tutte le prove vivono in `<scratchpad>/review/probe-RN/`» | invariato. ⚠️ Lo scratchpad della sessione precedente esiste ancora su disco, in **sola lettura**: `/c/Users/zagor/AppData/Local/Temp/claude/C--Users-zagor-Desktop-harness/70220bef-5dc3-4d57-a0cd-7088a6828df6/scratchpad/review/` — `probe-R5/gui-ts5/` è un modello dell'11 **installato** (`typescript` 5.9.3, D79) coi suoi `node_modules`; `probe-R8/c15.md`, `c16.md`, `c17.md` e `probe-R6/P-*.md` sono estratti **vecchi**, di prima delle correzioni: **non usarli**, estrai dal piano di oggi. Il lockfile di quel modello è copiato in `<scratchpad>/review/model11-lock/` (`package.json`, `package-lock.json`): `npm ci` con esso monta gli stessi pacchetti dalla cache in secondi |
| `cargo`, `node`, `cargo-audit` | invariati: 1.95.0, v24.9.0, 0.22.2 (`rustfmt` 1.9.0 c'è) |
| il rapporto in `<scratchpad>/review/RN-report.md` | invariato, **a pezzi**, un compito alla volta — con la data **2026-09-16** nel titolo |

Questo file è la tua **unica** lettura preliminare, oltre al tuo `RN-prompt.md`. Il coordinatore ha già fatto la
lettura d'apertura del repository (`CLAUDE.md`, `docs/COMPENDIO.md`): **NON rifarla** — non aprire `CLAUDE.md`,
`docs/COMPENDIO.md`, `docs/HANDOFF.md` né `docs/audit-2026-08-27.md` «per farti un'idea». Ciò che ti serve è qui,
nello scheletro, e nel piano.

## 0. Chi sei, e che cosa NON fai

- Sei un **revisore in SOLA LETTURA** di un piano scritto e **non ancora eseguito**. Il tuo prodotto è un rapporto.
- ⛔ **Nessuna modifica a nessun file del repository** `C:\Users\zagor\Desktop\harness` (in Git Bash:
  `/c/Users/zagor/Desktop/harness`). Vietati: `Edit`/`Write` dentro il repo, `git add`/`commit`/`stash`/`checkout`/
  `reset`, `sed -i`, `cargo build`/`cargo test`/`cargo run` **senza `--locked`**, `npm install`/`npm ci` dentro il
  repo, qualunque comando che crei o tocchi file sotto il repo. `cargo test --locked`, `cargo build --locked`,
  `cargo audit`, `bash scripts/gate-deps.sh`, `git diff`/`git show`/`git log`/`git ls-files` sono ammessi.
- Le prove che installano pacchetti o creano file si fanno **solo** nella tua cartella di prova:
  `<scratchpad>/review/probe-RN/` (creala tu). Là puoi `npm install`, `cargo new`, scrivere file, compilare.
- Alla fine: `git -C /c/Users/zagor/Desktop/harness status --porcelain` deve essere **vuoto**, e lo riporti.
- ⛔ **NON correggere niente. NON fermarti a chiedere.** Se non puoi verificare una cosa, la scrivi nel rapporto sotto
  «Non verificato, e perché». Se il piano ha ragione e tu avevi un dubbio, lo scrivi lo stesso (una riga,
  verdetto NON RIPRODOTTO), così il coordinatore non lo ricerca.
- Il rapporto è in **italiano**; ogni nome di codice, file, tipo, funzione è **esatto e in inglese** come nel sorgente.
- Il tempo è tuo: la revisione del piano della parte 1 (2400 righe) è costata a un revisore ~20 minuti e ~40
  comandi. Il tuo perimetro è più grande: scrivi il rapporto **a pezzi**, un compito alla volta, così se muori a
  metà resta ciò che hai scritto.

## 1. I file, e come si leggono

| Che cosa | Dove |
|---|---|
| scratchpad (`<scratchpad>`) | `C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\70220bef-5dc3-4d57-a0cd-7088a6828df6\scratchpad` — in Git Bash `/c/Users/zagor/AppData/Local/Temp/claude/C--Users-zagor-Desktop-harness/70220bef-5dc3-4d57-a0cd-7088a6828df6/scratchpad` |
| il piano (`<piano>`) | `docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md` — **LF**, 20 129 righe, 1,2 MB. ⛔ **Mai leggerlo intero, mai per numero di riga:** si estrae per intestazione |
| il disegno del 2 (`<disegno2>`) | `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` — LF, 726 righe |
| la stella polare (`<stella>`) | `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` — LF, 1 081 righe |
| lo scheletro del piano | `<scratchpad>/review/skeleton.md` — [A] i vincoli globali del piano, [B] la tabella della posizione (17 righe), [C] le voci aperte che il piano sa, [D] per ogni compito: intestazione, *Files*, *Interfaces*. **Leggilo per intero prima di cominciare** (83 KB) |
| i titoli delle 116 voci **P** | `<scratchpad>/review/p-titles.md` (`numero di riga:### P-NN — titolo`) |

Comandi che reggono (tutti da lanciare con `cd /c/Users/zagor/Desktop/harness &&` in testa):

```bash
# un compito intero (il 17 esce su "## Come si riprende")
awk '/^## Compito 7:/{f=1} /^## Compito 8:/{exit} f' <piano>
# una voce P
awk '/^### P-23 /{f=1; print; next} f&&/^### P-|^## /{exit} f' <piano>
# una riga D
grep -n '^| \*\*D23\*\* |' <piano>
# una sezione di un disegno: PRIMA ritrovi la riga, POI leggi a blocchi di <=120 righe (le righe sono lunghe)
grep -n '^### §5' <disegno2>
sed -n '247,280p' <disegno2>
# il modello di un file futuro, dal blocco di codice che comincia alla riga R del piano (R = la riga DOPO la recinzione di apertura)
awk 'NR>=R{ if(/^```/){exit} print }' <piano> > <scratchpad>/review/probe-RN/modello.rs
```

⚠️ **Le trappole di questa macchina** (Windows 11, Git Bash; misurate): `grep -c` conta **righe** con almeno un
match, non occorrenze (`grep -o … | wc -l` per le occorrenze) · in `grep -E` la sequenza `\|` è una barra
**letterale**, e un `grep -c` copiato da una cella di tabella (dove `\|` è la convenzione per il pipe) può rendere 0
senza cercare niente: ogni `grep -c` che deve rendere 0 si prova **prima** su un input dove deve rendere 1 · `grep -i`
con più di un `-e` va in *Aborted* · `$?` **dopo una pipe** è l'uscita dell'ultimo comando della pipe, non del primo:
redirigi su file e leggi `$?` subito, o usa `PIPESTATUS` · `grep -A N` su un tratto ne **taglia** le operazioni: per
contare gli item usa un'ancora di riga (`grep -nE '^\s*(pub )?(const )?fn '`) · `sed` toglie i CR in lettura (qui
irrilevante: piano e disegni sono LF; i sorgenti Rust e molti documenti sono **CRLF** in albero — `git ls-files --eol
<file>` lo dice) · `gh` **non** è installato: `curl -s https://api.github.com/repos/devfrx/daemon` · il registro npm
risponde a `npm view <pkg> version dist-tags time --json` · le crate scaricate si leggono in
`~/.cargo/registry/src/*/<crate>-<versione>/` · `cargo` è 1.95.0 dalla toolchain appuntata, `node` è **v24.9.0**,
`cargo-audit` 0.22.2 è installato · uno script Python si scrive con `Write` in `probe-RN/` e si lancia con
`python <percorso>`; un heredoc Bash con backslash o righe lunghe **rompe**.

## 2. Lo stato del repository, misurato dal coordinatore il 2026-09-15

- `HEAD` = `baf3cde`, `main` = `origin/main`, albero pulito, nessuno stash. `bash scripts/gate.sh` → `GATE GREEN`
  (11:05); `bash scripts/check-docs.sh` → `OK — no inconsistencies.`
- ⛔ **Il codice di prodotto è IDENTICO a quello su cui il piano è stato pre-controllato** (`42b50d8`, 2026-09-11):
  `git diff --stat 42b50d8..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml gui/ spikes/`
  è vuoto. Quindi una voce **P** che non regge oggi non reggeva nemmeno allora — **oppure dipende dal mondo esterno**
  (registro npm, crates.io, `cargo audit`, API GitHub), ed è la cosa da rimisurare.
- Nessun compito è eseguito. L'errata del piano è **vuota**. `grep -c '^### P-' <piano>` → **116**;
  `grep -c '^| \*\*D[0-9]' <piano>` → **74**; `grep -c '^## Compito' <piano>` → **17** = le righe della posizione.
- `gui/` **non esiste**. Il **solo** segnaposto dichiarato del piano è la versione di `interprocess` nel manifesto del
  core finto (compito 12): `grep -n '<the version' <piano>` → una riga.
- I `<data>` (34 occorrenze) e `<tempo>` (4) nei compiti 14–17 **non sono segnaposto**: sono i valori del giorno
  dell'esecuzione, e ogni passo dice di sostituirli.

## 3. Che cosa cerchi — le sei dimensioni (sono l'ordine dell'undicesima chiusura del piano)

| # | Dimensione | Come si fa, concretamente |
|---|---|---|
| 1 | **Copertura del disegno**, sezione per sezione | per ogni riga delle sezioni assegnate al tuo perimetro che nomina un artefatto, una prova, una decisione o un richiamo datato da scrivere: quale compito e quale Passo la produce? Esito: **coperta** / **scoperta** / **contraddetta** (il piano fa un'altra cosa: c'è una `D` o una `P` che lo dichiara col richiamo?) / **spostata** (a un altro compito, con richiamo) |
| 2 | **Segnaposto** | ogni `<…>` (tranne `<data>`/`<tempo>` e quello dichiarato), `…` dentro un blocco di codice, `node -e "…"`, `it('…', () => {})`, `todo!()`, `unimplemented!()`, `TODO`, `FIXME`, un valore «da scegliere» senza chi lo sceglie |
| 3 | **Coerenza fra il blocco *Interfaces* e i Passi** | si **contano** i `pub fn`/`pub struct`/`pub enum`/`pub const`/`export` del codice dettato nei Passi (ancora di riga, non `grep -A`) e si confrontano **uno per uno** con nomi, firme, campi, generici, percorsi di modulo e di file del blocco. Poi i **consumatori**: dove il blocco dice *«i compiti N e M li usano con questi nomi esatti»*, si cerca il nome (`grep -n`) nel codice dettato di quei compiti e si confrontano firme e campi. Il blocco *Interfaces* è un **contratto**: una parafrasi del disegno non è una firma |
| 4 | **Ogni conteggio rilanciato** | ogni comando in un `Passo 1: le misure prima`, ogni «Atteso:», ogni cifra accanto a un comando, ogni criterio di chiusura: **si rilancia** sul repository di adesso. Un'attesa su un artefatto **futuro** (un file che il compito crea) si misura sul **modello** estratto dal piano (comando qui sopra), mai a mente. Un'attesa su un file che un compito **precedente** crea si misura sul modello di quel compito |
| 5 | **Ogni voce P rimisurata** | nel tuo compito cerca i rimandi `P-NN` (`grep -o 'P-[0-9]\+' \| sort -u`); in `p-titles.md` cerca le P il cui titolo o testo nomina il tuo compito (`awk` sulla voce, poi `grep -n 'compito N'`). Per ciascuna rilancia i comandi che la voce porta e di' se **regge** |
| 6 | **Ogni numero di compito ricensito** | ogni «compito N», «il N», «al N», «del N», «compiti N e M» nel tuo perimetro → è il compito giusto **per la tabella della posizione** ([B] dello scheletro)? ⚠️ **D25** ha spostato di uno i compiti dal 9 al 16; **D13**, **D21**, **D47**, **P-81**, **P-85** hanno mosso lavoro fra compiti: un numero scritto prima di quelle decisioni può essere stantio |

E le **quattro domande** del pre-controllo, che valgono ancora: la **sonda è sbagliata** (vacua, o attacca il caso
invece del meccanismo) · la **sonda manca** (un artefatto senza controllo che lo eserciti) · l'**artefatto è
sbagliato e compila** (si coglie solo scrivendone un'implementazione da **fuori** — chiediti se un chiamante da
fuori la crate, o dal compito successivo, può usarlo come il blocco *Interfaces* promette) · il **compito è già
eseguito**, o al contrario **un compito precedente lascia false delle righe** (commenti, doc, tabelle) nei file che
questo compito apre.

## 4. Le specie di difetto che il pre-controllo NON coglie — misurate nelle revisioni precedenti di questo repository

Cercale **attivamente**; sono istruzioni, non aneddoti.

- Un'attesa numerica su una sezione **futura** misurata a mente invece che sul modello (un `awk` con `{s=1;next}`
  salta l'intestazione; un `grep -c` sull'intero file conta anche le righe già presenti).
- Un testo fissato per un'altra sessione che porta la **data di oggi** invece di `<data>`.
- Un blocco *Interfaces* i cui nomi vengono dalla **frase** del disegno e non dal codice dettato.
- Un numero **`D` di un altro piano** citato senza il nome del piano (in questo piano `D14` è `FileCustody::open`;
  «D14 della parte 1» sono le case dei documenti).
- Una **citazione fra virgolette** — di un'altra voce del piano, di un commento del sorgente, di una riga del disegno —
  che non corrisponde al testo vero: si rilegge col `grep -F`.
- Un ***Trova*** (il testo che `replace_unique.py` sostituisce) che **non esiste** o **non è unico** nel file: per
  un file **esistente** conta con `grep -c -F '<testo>' <file>` (o Python `text.count(s)` con `newline=""`, che
  regge anche i CRLF); per un file che un compito **precedente** crea, contro il modello di quel compito. ⚠️ Un
  *Trova* di più righe su un file CRLF: il piano usa `replace_unique.py`, che converte; tu confronta con Python.
- Una **sonda vacua**: verde anche se l'artefatto manca o è sbagliato — un contatore che parte da un valore che il
  soggetto non ha prodotto; un `grep -c` che deve rendere 0 e non è mai stato provato dove rende 1; una sonda su
  un'attività **infinita** che guarda solo il valore di ritorno; `axe-core` che mette il caso fra gli **incompleti**;
  una regola di lint ad **avviso**; un `\|` in un `grep` copiato da una cella; una proprietà che confronta insiemi
  **vuoti**; un `it` che asserisce due letterali scritti dallo stesso test.
- Una **sonda che manca**: un artefatto senza nessun controllo che lo eserciti, o la **seconda direzione** che manca
  (che il controllo NON scatti dove non deve).
- Un **`match` esaustivo** su un enum che un compito allarga (`RecordKind`, `Detail`, `IpcMessage`, `LayoutState`):
  il compito nomina **tutti** i posti che il compilatore fermerà? (`grep -rn 'match' crates/ …` sui tipi giusti.)
- Una **dipendenza** aggiunta in un passo solo (manifesto senza `Cargo.lock` nello stesso commit), un `--locked`
  lanciato **dopo** aver toccato un manifesto, una versione appuntata che il registro **non serve più** o una
  **major** nuova presa come `latest` («novità non è maturità»: si prende la minor/patch corrente della major in uso).
- **Due letterali che devono restare uguali** e che nessun comando confronta.
- Un **fine-riga** sbagliato: i file nuovi nascono **LF**; ogni file esistente ha il suo (`git ls-files --eol`) e il
  compito lo dichiara in *Files* — verifica la dichiarazione col comando; `sed -i` su un CRLF.
- Una riga di tabella che comincia con `| **D<cifra>**` o `| **E<cifra>**` **fuori** dalle tabelle D ed errata
  (falsifica i comandi che le contano); un'ancora d'inserimento che spezzerebbe una tabella.
- Un **numero misurato in prosa** senza il comando e la data accanto (vincolo globale 3); un comando in un documento
  delimitato per **numero di riga** invece che per testo.
- **Codice o commenti non in inglese**; documentazione non in italiano; un riferimento al codice dentro un documento
  che non porta il nome esatto del sorgente (vincolo globale 2).
- Un percorso di file scritto in due modi in due compiti; un file **creato** da due compiti; un file **modificato**
  da un compito prima di essere creato da un altro.
- Un'API di libreria data per esistente e non letta alla fonte (`.d.ts` nel `.tgz`, sorgente della crate in
  `~/.cargo/registry/src/`): se il compito la usa e nessuna P la misura, **leggila tu**.

## 5. Il rapporto

Scrivilo in `<scratchpad>/review/RN-report.md` con `Write` (la prima volta) e poi **appendi** un pezzo per compito
(rileggi il file e riscrivilo, o `cat >>` da un file scritto con `Write`). Formato **obbligatorio**:

```markdown
# Rapporto RN — <perimetro> — 2026-09-15

## Esito in tre righe

## Rilievi
| # | Compito · dove (una FRASE da cercare col grep, MAI un numero di riga) | Che cosa dice il piano | Che cosa ho misurato (comando → resa) | Verdetto | Specie | Blocca? | Rimedio proposto |
|---|---|---|---|---|---|---|---|
| RN-1 | … | … | … | CONFERMATO | fatto | sì | … |

## Copertura del disegno per il mio perimetro
| Sezione · riga o frase del disegno | Compito · Passo che la produce | Esito |
|---|---|---|

## Voci P rimisurate
| P | Comando rilanciato → resa | Regge? |
|---|---|---|

## Numeri di compito ricensiti
| Dove (frase) | Numero scritto | Numero giusto per la posizione | Esito |
|---|---|---|---|

## Comandi lanciati (TUTTI, uno per riga: comando → resa in breve)

## Non verificato, e perché

## Stato finale
`git status --porcelain` → (vuoto) · `git ls-files --eol` di piano e disegni → `i/lf w/lf` invariato
```

- **Verdetto**: `CONFERMATO` (il piano dice il falso, o farebbe il falso) · `GIÀ COPERTO da P-NN / D-NN` (di' dove) ·
  `NON RIPRODOTTO` (l'hai cercato, il piano ha ragione: **una riga**, così il coordinatore non lo ricerca).
- **Specie**: `fatto` (un comando, un conteggio, un nome, un tipo, un file, una versione) · `prosa` (solo parole).
- **Blocca?**: **sì** se il compito eseguito com'è scritto produrrebbe un rosso, un artefatto sbagliato, una sonda
  vacua o un commit che dice il falso; **no** altrimenti.
- **Rimedio proposto**: il testo da cambiare e **in quale Passo**, oppure «voce P nuova», «riga D nuova», «richiamo
  datato nel disegno §N». ⚠️ Non correggerlo tu.
- ⛔ Ogni cifra del tuo rapporto è **derivata da un comando che hai lanciato**, non dall'elenco che stai scrivendo:
  prima di scrivere «quattro rilievi» conta le righe della tua tabella.

## 6. L'ordine di lavoro consigliato

1. Leggi questo file e il tuo `RN-prompt.md`; poi `skeleton.md` per intero; poi `p-titles.md`.
2. Per ogni compito del tuo perimetro, nell'ordine: estrai il compito intero (a blocchi di 200 righe se serve);
   leggi le sezioni del disegno che la riga «Read:» del suo *Files* nomina (solo quelle) e quelle che il tuo prompt
   assegna; rilancia **tutti** i comandi del Passo 1 e ogni «Atteso»; ricensisci il blocco *Interfaces* contro i
   Passi; verifica ogni *Trova*; cerca le specie della §4; rimisura le P; ricensisci i numeri di compito; scrivi il
   pezzo di rapporto di **quel** compito **prima** di passare al successivo.
3. Chiudi con copertura, comandi, non verificato, stato finale.

Non leggere le sezioni del disegno che non ti sono assegnate «per farti un'idea». Se una verifica richiede un compito
fuori dal tuo perimetro (un produttore o un consumatore), usa il suo blocco *Interfaces* nello scheletro o un
`grep -n` mirato sul piano, non leggerlo intero.

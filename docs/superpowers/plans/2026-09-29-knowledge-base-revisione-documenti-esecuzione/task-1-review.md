# Revisione del compito 1 — ADR-0040, e ciò che il cancello pretende con lui

Scritta dal revisore il 2026-09-30, sulla macchina `jays`, sul commit `025dc0f` sopra `8bb8440`. Il revisore non ha scritto
nulla di ciò che rivede e **non ha toccato l'albero del repository**: le prove che scrivono stanno nel clone
`C:/Users/zagor/AppData/Local/Temp/rv1`, lasciato dov'è, e nella cartella `…/scratchpad/review1`, qui sotto `R`. Nessun
subagente; il cancello una volta sola, in background, senza `&`.

## 1. Il verdetto

**Conforme al dettato; qualità approvata — con due rilievi Importanti del dettato da curare prima del push, e nessun Critico.**

| Classe | Quanti | Quali |
|---|---|---|
| Critico | nessuno | — |
| Importante | due | R1, R2 — la testa della tabella della posizione: falsa da questo commit, e POS6 la taglierà a metà |
| Minore | sei | R3–R8 |
| Nit | tre | R9–R11 |

La rifazione del compito nel clone, da `8bb8440` coi pezzi estratti dal piano, rende i dieci file del commit **identici byte
per byte**, fine-riga compresi (§4). ADR-0040 dice punto per punto la 3.2 e la 3.3, e il rimando di ADR-0022 non urta né i
suoi due rimandi né ADR-0018, ADR-0023 e ADR-0024 (§5). Il rapporto dell'implementatore regge: ogni numero rilanciato torna,
salvo un'etichetta (R11). Tutti i rilievi tranne R11 sono del **dettato** o del contorno, non dell'esecuzione.

**Le due voci candidate del rapporto:** **ER-11 valida**, col testo raffinato in §5.6; **ER-12 valida**, solo per il
compito 1, l'unico che crea un file. Le voci candidate di questa revisione cominciano da **ER-13**.

## 2. I rilievi

| # | Classe | Dove | In breve | Voce |
|---|---|---|---|---|
| R1 | Importante | il piano, riga 445 | *«L'esecuzione non è cominciata»*, sopra una tabella col compito 1 ✅ | ER-13 |
| R2 | Importante | il piano, blocco POS6, righe 1941–1942 | POS6 sostituisce la prima riga fisica di un capoverso di tre: al compito 6 restano due righe orfane | ER-14 |
| R3 | Minore | `docs/COMPENDIO.md` riga 632; il piano, riga 2010 | il puntatore dice che l'esecuzione *«riparte dal compito 1»*; il *«Come si riprende»* dice *«Nessun compito è eseguito»* | — (chiusura della sessione) |
| R4 | Minore | ADR-0040 riga 57; `docs/COMPENDIO.md` riga 585 | *«la regola vive nel rimando di ADR-0016»*: il rimando nasce col compito 2 | ER-15 |
| R5 | Minore | `docs/COMPENDIO.md` righe 719–720; `docs/HANDOFF.md` righe 1056–1057 | *«richiede un ADR nuovo che lo superi (`Superseded by`)»*, e da oggi c'è il superamento in parte | ER-16 |
| R6 | Minore | ADR-0040 righe 81–82 | il seguito nomina la casa unica di chi costruisce che cosa, e ne copia il contenuto | ER-17 |
| R7 | Minore | `crates/daemon/src/main.rs` riga 191 | il commento dice che la disposizione dei pannelli non va nel backup; ADR-0040 la mette nel backup | ER-18 |
| R8 | Minore | ADR-0040 riga 74, contro ADR-0023 righe 102–130 | *«la protegge il sistema operativo»*: su Linux solo se le copie nascono leggibili dal solo proprietario | al proprietario, A/B |
| R9 | Nit | ADR-0040 riga 34 | *«e in nessun backup»* attribuito alla fonte, che non lo dice | — |
| R10 | Nit | `docs/AVVIO-CHAT.md` righe 15–17 | il richiamo nuovo allontana *«questo campo»* dal suo antecedente | — |
| R11 | Nit | il rapporto, riga 514 | *«added lines»* `13`, e le righe aggiunte sono quattordici | — |

### R1 — Importante: la testa della posizione dice il falso dal commit del compito 1

- **Il file e la riga:** `docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md`, il capoverso delle righe
  444–446, sopra la tabella della posizione; la riga ritrovata con `grep -n "L'esecuzione non è cominciata" <piano>` → `445`.
- **L'evidenza:** la riga 450 dice `| **1** | … | uno | ✅ 2026-09-30 |`, e `git log --oneline --grep='knowledge-base-revisione(compito 1)' | wc -l` → `1`;
  la riga 445 dice *«⏳ **L'esecuzione non è cominciata**: un compito per sessione»*. POS1 riscrive la sola riga del compito
  (`>= | **1** |`), e nessun blocco dei compiti 2–5 tocca il capoverso: resterà falso fino al compito 6.
- **Perché Importante:** è la sezione che il piano chiama *«casa unica, e si aggiorna scrivendo»*, e `CLAUDE.md` vuole che un
  documento di stato non menta con autorevolezza. Non è Critico: la riga che il compito consegna, quella della tabella, è vera.
- **Di chi:** del dettato — l'implementatore ha applicato POS1 parola per parola.
- **La cura, in testo — voce candidata ER-13:** il coordinatore, nel commit di chiusura della sessione e prima del push,
  riscrive il capoverso **in una riga fisica sola** — la ragione è R2 —, conservando il prefisso su cui ancora POS6:

  > ✅ **IL PIANO È SCRITTO E PRE-CONTROLLATO, il 2026-09-29.** Ciò che il pre-controllo ha trovato sta nell'errata, e i blocchi che ha cambiato sono provati in sequenza — P-19. ▶️ **L'esecuzione è in corso**, un compito per sessione: fin dove, lo dice la tabella qui sotto; come si riprende, il *«Come si riprende»*, in fondo.

  Resta vera fino al compito 6 senza altri tocchi. Il testo della voce: *«⚠️ Compito 1, blocco POS1 — la riga in testa alla
  tabella della posizione diceva «L'esecuzione non è cominciata», e POS1 riscrive la sola riga del compito: dal commit del
  compito 1 era falsa. Trovata dalla revisione del compito 1, il 2026-09-30. ✅ Il capoverso riscritto in una riga sola, che
  resta vero fino al compito 6 e che POS6 sostituisce per intero — ER-14.»* Provata su una copia del piano: §3.9.

### R2 — Importante: POS6 taglierà il capoverso a metà, al compito 6

- **Il file e la riga:** il blocco POS6 nel piano, righe 1941–1942 — `grep -n '^>= ✅ \*\*IL PIANO È' <piano>` → `1941`.
- **L'evidenza:** `>=` prende la riga **intera** che comincia col prefisso, cioè la sola riga fisica 444; `==` sostituisce
  quella. Simulato su una copia del piano di `025dc0f` (§3.9), dopo *«✅ **IL PIANO È ESEGUITO, il 2026-10-09.** … Il passo dopo
  lo dice la §6 del compendio.»* restano le due righe orfane *«che ha cambiato sono provati in sequenza — P-19. ⏳
  **L'esecuzione non è cominciata**: un compito per sessione — il / *«Come si riprende»*, in fondo.»* — un capoverso rotto che
  dice l'opposto della riga sopra. Lo stesso controllo su tutti gli `>=` dei diciotto pezzi (§3.9): è l'**unico** che taglia un
  capoverso a capo.
- **Perché Importante, e perché nessuno l'ha visto:** renderebbe falso ciò che il compito 6 consegna; `check-docs.sh` non legge
  la prosa, quindi la simulazione P-19 — `checked` e `OK` dopo ogni compito — non poteva vederlo.
- **La cura, in testo — voce candidata ER-14:** è **ER-13**. Col capoverso in una riga sola, POS6 lo sostituisce intero e il
  blocco **non cambia**: provato sulla copia, `applied: 2 edits in 1 files` e il capoverso pulito (§3.9). Il testo della voce:
  *«⛔ Compito 6, blocco POS6 — l'ancora `>= ✅ **IL PIANO È` prende la prima riga fisica di un capoverso di tre, e `==`
  sostituisce solo quella: restavano due righe orfane, con «L'esecuzione non è cominciata». Trovata dalla revisione del compito
  1, il 2026-09-30, simulando POS6 su una copia del piano. ✅ La cura è ER-13; il blocco non cambia.»*

### R3 — Minore: il puntatore della §6 e il *«Come si riprende»* sono rimasti al compito 1

- **Il file e la riga:** `docs/COMPENDIO.md` riga 632 — `grep -n 'La sua esecuzione riparte dal compito 1' docs/COMPENDIO.md`
  → `632` —; il piano, riga 2010 — `grep -n 'Nessun compito è eseguito' <piano>` → `2010`.
- **L'evidenza:** il puntatore, casa unica del prossimo passo, dice *«▶️ **La sua esecuzione riparte dal compito 1**, un compito
  per sessione»*, e il compito 1 è fatto; il *«Come si riprende»* del piano dice *«**Nessun compito è eseguito.**»*.
- **Perché Minore, e di chi:** la cura è già dettata, ed è del coordinatore alla chiusura di questa sessione — `CLAUDE.md`,
  *«Manutenzione della documentazione»*: il *«Come si riprende»* riscritto, la chiusura precedente in archivio; il puntatore
  *«se il prossimo passo cambia»*; e il punto 4 del *«Come si riprende»* del piano. Il rischio pratico è basso: la sonda
  `ls docs/adr/0040-* | wc -l` del Passo 1 fermerebbe chi rifacesse il compito 1. Lo segnalo perché il puntatore oggi dice un
  **numero di compito**, e con quella forma marcisce a ogni compito.
- **La cura, in testo:** prima `archive_head.py --pointer`, poi la frase del puntatore in una forma che non nomina il compito,
  coi due link di adesso — la mappa e la consegna del metodo, che ER-8 vuole conservata —:

  > ▶️ **La sua esecuzione è in corso**, un compito per sessione — fin dove, lo dice la tabella della posizione del piano —: il 2026-09-30 il proprietario ha deciso **adesso**, nel primo ticket della mappa del metodo; i ticket della mappa si alternano coi compiti a sua scelta, e la consegna del metodo dice come si riprendono.

  I confini che `replace_pointer.py` usa al compito 6 — la riga `⏭️ **IL PROSSIMO PASSO` e la prima riga `⏳ ` dopo — restano;
  l'archivio riceve un link al disegno in più, e il comando E sull'archivio sale di uno: ER-9 dice già che si annota.

### R4 — Minore: ADR-0040 rimanda a un rimando di ADR-0016 che nasce col compito 2

- **Il file e la riga:** `docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md` riga 57 —
  `grep -n 'La regola vive nel rimando di' docs/adr/0040-*.md` → `57` —; `docs/COMPENDIO.md` riga 585 —
  `grep -n 'il rimando di ADR-0016' docs/COMPENDIO.md` → `585`.
- **L'evidenza:** ADR-0016 non porta nessun rimando, e nessuna delle due parole: la sonda qui sotto rende niente. La
  controprova: *«cartella dati»*, cercata nel blocco del compito 2 per ADR-0016, c'è — `1` —, perché il rimando lo scrive E2.

  ```bash
  grep -n -i 'rimando\|cartella dati\|percors[oi] protett' docs/adr/0016-permessi-granulari-e-default-dei-vincoli-sui-dati.md
  ```

- **Perché Minore:** l'ordine è **forzato** — il rimando di ADR-0016 porta il link ad ADR-0040, e il vincolo 11 vieta un link a
  un file che non c'è —, e ADR-0040 è append-only: la frase diventa vera col compito 2 e non si poteva scrivere altrimenti.
  Ciò che manca è la **dichiarazione**: D10 dichiara il ritardo analogo della riga della data, questo no; e col puntatore che
  alterna *«a scelta del proprietario»* ticket e compiti, la finestra può durare giorni.
- **La cura, in testo — voce candidata ER-15:** *«⚠️ Compito 1 — fino al compito 2, ADR-0040 al punto 6 e la sua voce nel
  compendio rimandano a un rimando di ADR-0016 che non c'è ancora: lo scrive il blocco E2. Trovata dalla revisione del compito
  1, il 2026-09-30. L'ordine è forzato — vincolo 11 — e ADR-0040 è append-only: si dichiara, come D10 dichiara il ritardo della
  riga della data, e si chiude da sé col compito 2.»* E al proprietario, quando sceglie fra un ticket e il compito 2: il
  compito 2 chiude questa finestra.

### R5 — Minore: la §7 del compendio e `HANDOFF.md` conoscono un solo modo di superare un ADR

- **Il file e la riga:** `grep -n 'Superseded by.), non una conversazione' docs/COMPENDIO.md docs/HANDOFF.md` —
  il punto sta per l'apice inverso — → `docs/COMPENDIO.md:720` e `docs/HANDOFF.md:1057`, le seconde righe delle frasi che
  cominciano alle righe 719 e 1056.
- **L'evidenza:** entrambe dicono *«Rimettere in discussione un ADR `Accepted` **richiede un ADR nuovo che lo superi**
  (`Superseded by`), non una conversazione»*; da questo commit la riga *«ADR append-only»* di `CLAUDE.md` e la §13 conoscono il
  terzo caso — *superato in parte*, lo stato resta `Accepted`, nessun `Superseded by` —, e ADR-0040 su ADR-0022 ne è l'istanza.
  Le due letture obbligatorie, `CLAUDE.md` e il compendio, dicono ora due cose diverse su come si rimette in discussione un ADR.
  La 3.3 del disegno nomina `CLAUDE.md` e la §13, non queste due frasi; la riga di `HANDOFF.md` il commit l'ha toccata per il
  totale.
- **Perché Minore:** il nocciolo — *un ADR nuovo, non una conversazione* — resta vero; è la parentesi a escludere il caso nuovo.
- **La cura, in testo — voce candidata ER-16:** nella §7 del compendio, con un richiamo datato: *«… **richiede un ADR nuovo che
  lo superi** (`Superseded by`) — o, se ne cambia solo alcune righe, che lo superi **in parte**: la riga «ADR superato in
  parte» della §13 —, non una conversazione.»* — una riga del compendio che si allunga, da misurare col comando C prima di
  scriverla, come vuole il vincolo 10; oggi il comando rende `6483`. In
  `HANDOFF.md` la frase è la seconda casa della stessa regola, e la riga 14 della Definizione di «fatto» ne vuole solo i
  totali: **registrata**, col proprietario come chiusore, come ER-5 fece per la riga di F2.

### R6 — Minore: il seguito di ADR-0040 copia il contenuto della casa unica che nomina

- **Il file e la riga:** ADR-0040 righe 81–82 — `grep -n 'casa unica di chi costruisce che cosa' docs/adr/0040-*.md` → `82`.
- **L'evidenza:** *«Chi costruisce la cartella dati per utente e la cartella nascosta dei router lo dice la sezione 4 del
  disegno della revisione, che è la casa unica di chi costruisce che cosa: il sotto-progetto 6, se nessuno le porta prima.»* La
  3.2, *«Il seguito»*, dice soltanto *«chi costruisce la cartella dati lo dice la sezione 4»*; la 4.2 dà le due cartelle al 6
  (§3.9). Il recinto del piano aggiunge una seconda casa di *chi costruisce*, in un file append-only: la regola ⛔ di
  `CLAUDE.md` sul puntatore che vive in più documenti, gotcha #68.
- **Perché Minore:** la clausola *«se nessuno le porta prima»* tiene vera la frase se le costruisce un sotto-progetto che viene
  prima del 6; marcirebbe solo se passassero a uno che viene dopo.
- **La cura, in testo — voce candidata ER-17, decisione del coordinatore:** **A** — prima del push, la frase finisce a *«…
  lo dice la sezione 4 del disegno della revisione, che è la casa unica di chi costruisce che cosa.»*, nel recinto del piano e
  nel commit; costa una voce d'errata, un commit riscritto prima di pubblicarlo, `check-docs.sh` e il cancello rilanciati.
  **B** — resta, e la voce lo dichiara; costa una copia protetta dalla clausola, per sempre. Consiglio **A**, perché il commit
  non è pubblicato e l'ADR, dopo, non si può più correggere se non con un rimando.

### R7 — Minore: un commento del sorgente dice che la disposizione dei pannelli non va nel backup

- **Il file e la riga:** `crates/daemon/src/main.rs` righe 190–191, sopra `LAYOUT_PATH` —
  `grep -n 'a window layout is neither' crates/daemon/src/main.rs` → `191`.
- **L'evidenza:** *«the journal is authoritative state and is BACKED UP AND ENCRYPTED; a window layout is neither»*. Letto com'è
  scritto, la disposizione non va nel backup. Ma la disposizione sta nella **configurazione** — rimando del 2026-09-08 di
  ADR-0022, *«la politica della riga — in chiaro, nel backup, permanente — non cambia»* —, e il backup del programma contiene
  la configurazione — ADR-0040, punto 4; la 1.1 del disegno, riga della cartella dati del programma. Nessun documento lo
  registra: `grep -rn 'a window layout is neither' --include=*.md docs/` trova solo il piano del sotto-progetto 2 che lo scrisse.
- **Perché Minore, e di chi:** il commento è di prima, e il vincolo 2 vieta il codice; ma è la riga 6 della tabella di
  `CLAUDE.md` — ciò che smentisce può stare in un commento —, e sta nello **stesso** blocco di commenti di ER-3.
- **La cura, in testo — voce candidata ER-18:** *«⚠️ Compito 1 — il commento sopra `LAYOUT_PATH` dice che la disposizione dei
  pannelli non va nel backup, e ADR-0040 al punto 4 — con ADR-0022, rimando del 2026-09-08 — la mette nella configurazione,
  che nel backup ci va. Trovata dalla revisione del compito 1, il 2026-09-30. Il vincolo 2 vieta il codice: registrata accanto a
  ER-3, nello stesso commento, con lo stesso chiusore — chi riscrive `LAYOUT_PATH` con la cartella dati, il 6.»* E una riga nella
  tabella delle voci aperte, accanto a quella dei due commenti.

### R8 — Minore, per il proprietario: la copia in chiaro la protegge il sistema operativo solo col permesso giusto

- **Il file e la riga:** ADR-0040 righe 73–74, le *Negative (accettate)* — `grep -n 'la protegge il sistema operativo' docs/adr/0040-*.md`
  → `74` —, contro ADR-0023, il rimando del 2026-08-18, righe 102–130 — `grep -n '0600. sul file' docs/adr/0023-*.md` → `113`.
- **L'evidenza — letta nei due ADR, gotcha #59:** ADR-0040 accetta *«una copia in chiaro … la protegge il sistema operativo,
  come in Claude Code»*. Il rimando di ADR-0023 registra che su Linux il file del giornale nasceva `0644` —
  `OpenOptions::create(true)`, misurato con `umask` `0022` —, *«leggibile da qualunque utente della macchina»*, finché nessun
  `.mode()` gli dava il permesso; che il proprietario scelse *«`0600` sul file, non `0700` sulla cartella»*
  perché *«la cartella non ha un proprietario nel codice»*; e che il difetto *«era INVISIBILE dove si lavora»*, su Windows. Da
  ADR-0040 la cartella dati ha un posto e, per la 4.2, un costruttore, il 6, insieme alle copie; nessuna riga dice che le copie
  — file del proprietario, in chiaro, che sopravvivono al file anche quando lui lo rende privato — nascano leggibili dal solo
  proprietario.
- **Perché Minore, e perché al proprietario:** non è una contraddizione né un difetto del compito — la 3.1 del disegno ha deciso
  che ad ADR-0023 non serve un rimando —; è un seguito che manca, su una proprietà di sicurezza.
- **La domanda, in A/B:** **A**, il consiglio — una voce aperta registrata, col 6 come chiusore: *«le copie del checkpoint e la
  cartella dati nascono leggibili dal solo proprietario, anche su Linux — la lezione di PL-1, rimando del 2026-08-18 di
  ADR-0023»*; costo, una riga. **B** — niente, se il proprietario la ritiene già coperta dalla regola di ADR-0023; costo, il
  rischio che torni la forma di PL-1.

### R9 — Nit: *«e in nessun backup»* non lo dice la fonte

- **Il file e la riga:** ADR-0040 riga 34 — `grep -n 'e in nessun backup' docs/adr/0040-*.md` → `34`.
- **L'evidenza:** il *Context* attribuisce allo stato dell'arte *«letto alla fonte … e portato in `riferimenti.md`»* che in
  Claude Code le copie stanno *«in nessun backup»*. Le due righe di Anthropic in `riferimenti.md`, righe 2966–2967, non lo dicono;
  nemmeno le due pagine, rilette al sorgente oggi (§3.9): la cartella dell'applicazione, il chiaro, *«OS file permissions are
  the only protection»*, i trenta giorni, *«Not a replacement for version control»* — la parola *backup* non c'è. È un dedotto,
  plausibile; la 3.4 del disegno lo portava già.
- **La cura:** nessuna in ADR-0040. Se `riferimenti.md` si ritocca, accanto alle due righe: *«nessun backup: dedotto — Claude
  Code non ha un backup suo, e la pagina dice che il checkpoint non sostituisce il controllo di versione»*.

### R10 — Nit: il richiamo nuovo di `AVVIO-CHAT.md` allontana *«questo campo»* dal suo antecedente

- **Il file e la riga:** `docs/AVVIO-CHAT.md` righe 5, 15 e 17 —
  `grep -n 'RICHIAMO DEL 2026-09-30\|Prima questo campo portava un valore vero\|Il campo dello SHA è un segnaposto' docs/AVVIO-CHAT.md`.
- **L'evidenza:** *«⛔ Prima questo campo portava un valore vero»*, riga 17, parla del campo dello SHA della riga 5; fra i due
  stanno ora due richiami. Il posto segue il richiamo del 2026-09-01 e la risposta A della prima domanda della 6.1, *«la forma
  che il file usa già»*.
- **La cura:** nessuna adesso; se la testa del file si riordina, i richiami dopo i due capoversi sullo SHA.

### R11 — Nit, del rapporto: *«added lines»* `13`, e sono quattordici

- **Il file e la riga:** il rapporto, riga 514, nel blocco *«AFTER THE COMMIT»*.
- **L'evidenza:** `git diff --numstat 8bb8440..025dc0f -- docs/adr/0022-*.md` → `14 0`; il `13` è `grep -c '^+[^+]'`, le righe
  aggiunte **non vuote** — la quattordicesima è la riga vuota sotto il rimando. La sostanza, `0` righe tolte, regge.
- **La cura:** nella copia tracciata del rapporto, l'etichetta *«righe aggiunte non vuote»*.

## 3. I comandi rilanciati, con le uscite

Ogni blocco parte da `cd /c/EVERYTHING/DEV/MY_REPOS/daemon`, o dal clone dove è detto. `R` è la cartella della revisione,
`S` la sua sottocartella `pieces` nella forma che Python capisce, `T` la cartella di lavoro dell'implementatore, `P` il piano.
⚠️ Due file temporanei del confronto in §3.2 sono passati da `/tmp` e sono stati tolti subito: una svista di luogo, senza effetto.

### 3.1 L'avvio

```text
$ git rev-parse --short HEAD; git status --porcelain; git log --oneline 8bb8440..HEAD; git status -sb
025dc0f
(vuoto)
025dc0f knowledge-base-revisione(compito 1): ADR-0040 — dove vivono i dati, … la riga della data con la sua copia in archivio
## main...origin/main [ahead 1]
$ git config --show-origin --get-all core.autocrlf; hostname; python --version
file:C:/Program Files/Git/etc/gitconfig	true
jays
Python 3.13.7
$ diff <(tr -d '\r' < git-diff-U10-8bb8440..025dc0f) <(il diff del pacchetto)
PACKAGE_DIFF_IDENTICAL          (563 righe ciascuno)
```

### 3.2 Il rapporto, rilanciato — §2.1 del mandato

```text
$ ls -la --time-style=full-iso "$T/gate-2026-09-30.log"; git log -1 --format=%ci 025dc0f
-rw-r--r-- 1 zagor 197609 56375 2026-09-30 10:59:20.007604000 +0200 …/scratchpad/task1/gate-2026-09-30.log
2026-09-30 11:01:17 +0200
$ grep -E 'GATE GREEN|GATE RED|EXIT=' "$T/gate-2026-09-30.log"; wc -l -c "$T/gate-2026-09-30.log"; grep -n 'OK — no inconsistencies' "$T/gate-2026-09-30.log"
GATE GREEN.
EXIT=0
 1096 56375
1015:OK — no inconsistencies.
$ cat "$T/gate-times.txt" "$T/start-time.txt" "$T/end-time.txt"
gate start: 10:54:42
gate end: 10:59:20
start: 2026-09-30 10:50:27
end: 2026-09-30 11:02:22
```

Il log è dell'implementatore: sta nella sua cartella, e l'ultima scrittura precede il commit di un minuto e cinquantasette.
I blocchi che il rapporto dice incollati parola per parola, confrontati coi log della sua cartella, a righe senza CR:

```text
step1.log: report block 132-217 IDENTICAL to the log
step2.log: report block 236-250 IDENTICAL to the log
step3.log: report block 263-280 IDENTICAL to the log
step4-checkdocs.log: report block 289-306 IDENTICAL to the log
step4.log: report block 312-402 IDENTICAL to the log
step4-er10.log: report block 407-418 IDENTICAL to the log
```

Le cifre del rapporto dopo il commit:

```text
$ git diff 8bb8440..025dc0f -- docs/adr/0022-*.md | grep -c '^-[^-]'; … | grep -c '^+[^+]'; git diff --numstat 8bb8440..025dc0f -- docs/adr/0022-*.md
0
13
14	0	docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md          ← R11
$ wc -c -l docs/adr/0040-*.md "$R/pieces/adr0040.md"
   82  7446 docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md
   82  7442 …/review1/pieces/adr0040.md
$ git show 8bb8440:$P | wc -l; git show 025dc0f:$P | wc -l
2061
2061
```

### 3.3 Il Passo 1, sullo stato di `8bb8440`

Nel clone appena fatto, `core.autocrlf` `true` come nel repository; l'uscita condensata come in §3.4:

```text
$ git ls-files --eol $F $P        (i nove file tracciati)
i/lf    w/crlf  …  tutti e nove
$ for f in $F $P; do …CR…LINES…; done
0022 CR=118 LINES=118 · README CR=226 LINES=226 · COMPENDIO CR=919 LINES=919 · CLAUDE.md CR=123 LINES=123
HANDOFF CR=1408 LINES=1408 · roadmap CR=345 LINES=345 · AVVIO-CHAT CR=542 LINES=542 · stato-storico CR=2686 LINES=2686
piano CR=2061 LINES=2061
$ awk -f "$S/tables.awk" $F | cut -d: -f1 | sort | uniq -c
     31 docs/HANDOFF.md
$ echo $(( ceiling − byte senza CR − righe ))          (il comando C)
8103
```

Sul repository, dai blob di `8bb8440`: la guardia dei totali rende **otto** righe a trentanove — le stesse del rapporto, riga per
riga —; `git ls-tree --name-only 8bb8440 docs/adr/ | grep -c '0040-'` → `0`; `Rimando del` in ADR-0022 → `1`;
`modificato in parte da ADR-0040` nel compendio → `0`; `superato in parte` in `CLAUDE.md` e nel compendio → `0` e `0`; le
controprove sugli stessi testi a `025dc0f` → `1` e `1`. Il comando E a `8bb8440`, nell'ordine di `F`: `0 0 1 0 1 0 0 8`.

### 3.4 Le prove del Passo 4 e la sonda di ER-10 — §2.3 del mandato

Lo script `R/step4-rerun.sh`, sull'albero del repository a `025dc0f`, con le differenze dell'albero di lavoro del piano sostituite
dall'intervallo `8bb8440..025dc0f`. L'uscita è in `R/step4-rerun.log`; qui **condensata**: il `check-docs.sh` accorciato alla
coda, le righe per file unite con «·», i percorsi lunghi accorciati. Nessun valore cambiato.

```text
### HEAD
025dc0f
### check-docs.sh
OK — no inconsistencies.
check-docs exit=0
### git ls-files --eol (F + ADR-0040 + plan)
i/lf    w/crlf   CLAUDE.md · docs/AVVIO-CHAT.md · docs/COMPENDIO.md · docs/HANDOFF.md · docs/README.md
i/lf    w/crlf   docs/adr/0022-… · docs/archivio/stato-storico.md · docs/roadmap.md · docs/superpowers/plans/2026-09-29-…
i/lf    w/lf     docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md
### CR / LINES
0022 CR=132 LINES=132 · README CR=227 LINES=227 · COMPENDIO CR=930 LINES=930 · CLAUDE.md CR=123 LINES=123
HANDOFF CR=1408 LINES=1408 · roadmap CR=345 LINES=345 · AVVIO-CHAT CR=544 LINES=544 · stato-storico CR=2692 LINES=2692
0040 CR=0 LINES=82 · piano CR=2061 LINES=2061
### tables.awk per file (F + ADR-0040)
     31 docs/HANDOFF.md
### tables.awk on the plan, at 8bb8440 and now
0
0
### margin (command C)
6483
### guard totals
docs/HANDOFF.md:208: … Spec del kernel **§0–§10 completa, 40 ADR**.
docs/HANDOFF.md:1056:40 ADR in stato . Rimetterne in discussione uno **richiede un ADR
docs/HANDOFF.md:1272:| ❌ ri-derivare l'architettura | è in **40 ADR**, …
docs/HANDOFF.md:1317:| [](adr/) | **40 decisioni architetturali**. …
docs/roadmap.md:23:> Spec del kernel **completa e approvata** (§0–§10, 40 ADR). …
docs/COMPENDIO.md:135:Sono **40 ADR**, e **40 ADR in stato Accepted** — l'ultimo, 0040, il 2026-09-30, dalla revisione della knowledge base.
docs/COMPENDIO.md:751:| ❌ **ri-derivare l'architettura** | è nei 40 ADR, …
### command B (expected nothing)
### command B, control on 8bb8440 (expected one line)
163:  2. docs/COMPENDIO.md — contiene TUTTE le decisioni del progetto: le 39 ADR
### Accepted / Modifica / Negative in ADR-0040
1
1
1
### modificato da ADR-0040 in ADR-0022
1
### superato in parte in CLAUDE.md and COMPENDIO
CLAUDE.md:1
docs/COMPENDIO.md:1
### ER-4
1
### Rimando del in ADR-0022 (occurrences)
2
### removed lines in the ADR diff of the commit (expected 0), control on the compendium diff (> 0)
0
4
### ADR files in the commit
M	docs/adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md
A	docs/adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md
### perimeter (expected empty)
### perimeter control: the same command on docs/ (non-empty)
 9 files changed, 128 insertions(+), 12 deletions(-)
### command D from <base>
base=ff6f0e5
0
### command E (F, then ADR-0040)
0022:1 · README:0 · COMPENDIO:1 · CLAUDE.md:0 · HANDOFF:1 · roadmap:0 · AVVIO-CHAT:1 · stato-storico:8 · 0040:1
### ER-10 probe, with 8bb8440 in place of HEAD (expected SAME)
1
SAME
### ER-10 control, with 025dc0f (the new date line): expected DIFFERENT
DIFFERENT
### the plan diff: lines changed
1	1	docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md
### git diff --stat 8bb8440..025dc0f
 10 files changed, 129 insertions(+), 13 deletions(-)
```

Il margine scende di `8103 − 6483 = 1620` byte, il *«circa 1,6 KB»* del Passo 4. `ls docs/adr/*.md | wc -l` → `40`, e i file
con `- **Status:** Accepted` → `40`, nessuno senza.

### 3.5 Le cifre nuove — §2.5 del mandato

Uno script Python sulle righe `+` di `git diff -U0 8bb8440..025dc0f`, tolti code span, destinazioni dei link, date,
`ADR-NNNN`, numeri d'ADR, sezioni, `D…`/`K…`/`V…` e numeri di gotcha. Restano soltanto: i totali **40** delle righe che la
guardia confronta coi file — sette righe, le stesse della §3.4 —; numeri di punto, di sotto-progetto e di compito — `1`…`6`,
`9`, `11` —, e di sezione — `5.5`, `6.1`; il `2.` del numerato del messaggio di `AVVIO-CHAT.md`, di prima. **Nessuna cifra
misurata senza comando**; *«trenta giorni»* di ADR-0040 è a parole e sta in una riga datata, con la fonte in `riferimenti.md`.
Nei documenti della guardia nessuna cifra nuova seguita da «ADR» fuori da un code span, oltre ai totali.

### 3.6 I vincoli globali — §2.6 del mandato

- **Fine-riga:** i nove file tracciati `i/lf w/crlf` prima — nel clone — e dopo — sul repository —, la colonna `w/…` invariata,
  i CR **uguali alle righe** in ciascuno, prima e dopo; ADR-0040 `i/lf w/lf`, zero CR (§3.3, §3.4). Il numero assoluto dei CR
  cambia dove sono cambiate le righe: è ER-11.
- **Tabelle:** `31 docs/HANDOFF.md` prima e dopo, ADR-0040 senza righe fuori colonna; il piano, zero e zero.
- **Perimetro:** `git diff --stat 8bb8440..025dc0f -- crates/ gui/ scripts/ .github/ Cargo.toml Cargo.lock` vuoto, con la
  controprova su `docs/`; negli ADR solo ADR-0022, `M`, e ADR-0040, `A`; del piano una riga tolta e una messa, la posizione.

### 3.7 Il cancello — §2.7 del mandato

Sull'albero del repository a `025dc0f`, da solo, in background, verso `R/gate-review1-2026-09-30.log`; nient'altro sul
repository mentre girava — solo il clone e le letture di file.

```text
$ bash scripts/gate.sh > "$L" 2>&1; echo "EXIT=$?" >> "$L"       (11:15:31 → 11:20:42)
$ grep -E 'GATE GREEN|GATE RED|EXIT=' "$L"
GATE GREEN.
EXIT=0
$ grep -n 'OK — no inconsistencies' "$L"; wc -l -c "$L"; git status --porcelain | wc -l
1016:OK — no inconsistencies.
 1097 56460
0
```

`bash scripts/check-docs.sh` da solo → `OK`, col margine `6483` del comando C (§3.4).

### 3.8 Il contratto — §2.8 del mandato

```text
$ git log --oneline 8bb8440..025dc0f | wc -l
1
$ git show --name-status --format= 025dc0f          (dieci file: nove M, ADR-0040 A — i dieci del git add del Passo 5)
$ git log -1 --format=%s 025dc0f | grep -c '^knowledge-base-revisione(compito 1): '
1
$ git log -1 --format=%B 025dc0f | grep -ci co-authored
0
  controprova: la stessa sonda su d3e52b0, che un co-autore lo porta → 1
$ il messaggio, senza CR e righe vuote, contro la riga 990 del brief senza «git commit -m» e le virgolette
MESSAGE IDENTICAL to the brief's Step 5
$ git status -sb
## main...origin/main [ahead 1]
```

### 3.9 Le prove in più

**Il cancello dei documenti diventa rosso nei tre modi** — la riga 3 della Definizione di «fatto» —: `R/mutations.py`, nel
clone ricostruito, una mutazione per volta e il file rimesso byte per byte dopo ciascuna.

```text
baseline: exit=0 last=OK — no inconsistencies.
the README index row of ADR-0040 removed: exit=1; lines: ['  ✗ ADR: 40 files, 39 index entries', '1 inconsistencies to fix.']
the compendium entry of ADR-0040 renamed away: exit=1; lines: ['  ✗ compendium §5 — ADR with no entry: 0040 ', '1 inconsistencies to fix.']
one HANDOFF total put back to thirty-nine: exit=1; lines: ['  ✗ docs/HANDOFF.md declares 39 ADR in Accepted status, they are 40', '1 inconsistencies to fix.']
after restore: exit=0 last=OK — no inconsistencies.
$ git diff --stat 025dc0f          (vuoto: il clone è tornato com'era)
```

⚠️ **Trappola di macchina:** al primo giro il `subprocess.run(["bash", …])` di Python ha preso `C:\Windows\System32\bash.exe`, la
bash di WSL — `CreateProcess` cerca in `System32` **prima** del `PATH`, anche se `shutil.which('bash')` rende quella di Git —, e
lo script con fine-riga CRLF è caduto su `set: pipefail\r`. Con `C:/Program Files/Git/usr/bin/bash.exe` esplicito, i giri sopra.

**Il compito 2 si applica ancora dopo il compito 1:** nel clone, `apply_edits.py --check 2026-09-30` su `e2.txt` →
`checked: 20 edits in 11 files`, su `pos2.txt` → `checked: 1 edits in 1 files`; nessun file scritto.

**POS6 sul capoverso di adesso, e sul capoverso di ER-13** — su due copie del piano in `R/pos6sim` e `R/pos6sim2`:

```text
== adesso: applied: 2 edits in 1 files
✅ **IL PIANO È ESEGUITO, il 2026-10-09.** A dirlo non è questa riga ma la tabella qui sotto — … Il passo dopo lo dice la §6 del compendio.
che ha cambiato sono provati in sequenza — P-19. ⏳ **L'esecuzione non è cominciata**: un compito per sessione — il
*«Come si riprende»*, in fondo.
== col capoverso in una riga (ER-13): rewritten as one line; CR= 2059 LF= 2059
checked: 2 edits in 1 files
applied: 2 edits in 1 files
✅ **IL PIANO È ESEGUITO, il 2026-10-09.** A dirlo non è questa riga ma la tabella qui sotto — … Il passo dopo lo dice la §6 del compendio.
```

E ogni `>=` dei diciotto pezzi, contro il file che tocca: solo il primo di POS6 prende una riga il cui capoverso continua sotto;
gli altri — la riga della data del compendio, l'*«Ultimo aggiornamento»* della roadmap, le righe di tabella — sono righe intere.

**Le fonti del *Context* di ADR-0040:** `sed -n '2955,2968p' docs/riferimenti.md`, la sezione *«La revisione della knowledge
base — le fonti del disegno, 2026-09-29»*: `adr-tools` con `-l` e l'esempio `Amends`/`Amended by`, `-s` che cambia lo stato;
MADR senza uno stato parziale; Henderson; le due pagine di Claude Code. Rilette al sorgente oggi con `curl -sSL`:

```text
checkpointing.md:22:  … deletes a session's file snapshots in the retention sweep, by default about 30 days after …
checkpointing.md:105: ### Not a replacement for version control
claude-directory.md:1530: … `~/.claude` holds data Claude Code writes during sessions. These files are plaintext. …
claude-directory.md:1534: … once they're older than `cleanupPeriodDays` … The default is 30 days …
claude-directory.md:1542: | `file-history/<session>/` | Pre-edit snapshots of files Claude changed, used for checkpoint restore. …
claude-directory.md:1612: Transcripts and history are not encrypted at rest. OS file permissions are the only protection. …
grep -i 'backup' su entrambe → niente                                                                   ← R9
```

**Il sorgente del *Context*:** `sed -n '178,199p' crates/daemon/src/main.rs` — la frase *«where a per-user data directory
belongs is a decision no ADR has taken»* alla riga 196, `JOURNAL_PATH` = `"journal.redb"` e `LAYOUT_PATH` = `"layout.redb"`,
*«RELATIVE TO THE WORKING DIRECTORY»*; e il commento di R7, righe 190–191.

**Le risposte del disegno:** `sed -n '53,81p'` del disegno — D4, D12, D17 e D19, ciascuna ✅ **A** il 2026-09-29 —; la 4.2
alle righe 465–474, la 5.5 alle righe 588–604, la riga della cartella nascosta nella 1.1, riga 94. Dalla consegna del
brainstorming in archivio, solo le righe dei costi delle B di D4, D12 e D17 e le righe di K1, K21, K30, K32 e K53, col `grep`.

**I link:** i diciassette link delle righe aggiunte, risolti dalla cartella di ciascun file — tutti a un file vero.

## 4. Le due direzioni della conformità nel clone — §2.2 del mandato

**Il dettato, rifatto da `8bb8440` con `D=2026-09-30`:**

```text
$ extract.py scritto a mano dal brief (righe 74–126), col tool di scrittura, in R
53 righe; zero CR; zero barre rovesciate (tr -cd '\134')
IDENTICAL to plan fence      — il recinto estratto dal piano del clone con awk
IDENTICAL to brief fence
  controprova: una copia con FENCE = chr(96) * 3 contro il recinto del piano → DIFFERENT
$ cd rv1; PYTHONIOENCODING=utf-8 python extract.py docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md "$S"
diciotto righe, da apply_edits.py: 167 lines a pointer6.txt: 10 lines — le stesse del rapporto — EXIT=0
  i sei pezzi del compito, contro i recinti del brief: identici, zero CR ciascuno
$ sed "s/<data>/$D/" "$S/adr0040.md" > docs/adr/0040-…; grep -c '<data>' …; grep -n 'Date:' …
0
4:- **Date:** 2026-09-30
$ python "$S/archive_head.py" "L'intestazione del compendio, com'era — archiviata il $D, al compito 1 del piano …"
archived: 1 piece(s) under «…»; the heading starts with «L'intestazione del compendio»
$ python "$S/apply_edits.py" --check "$D" "$S/e1.txt"; …; apply; --check e apply di pos1.txt
checked: 17 edits in 7 files
applied: 17 edits in 7 files
checked: 1 edits in 1 files
applied: 1 edits in 1 files
```

**Il confronto:** il comando del mandato, **alla lettera**, non rende vuoto — `git diff --stat 025dc0f` nomina ADR-0040 con
`82 deletions(-)`, perché `git diff <commit>` scorre l'albero di lavoro attraverso l'**indice**, e nel clone il file nuovo non
c'è. Con `git add -N` sul solo ADR-0040 rende **vuoto**, salvo l'avviso *«LF will be replaced by CRLF»*, atteso. Di più: i
dieci file del clone, contro i dieci dell'albero del repository, con `cmp` → **BYTE-IDENTICAL** tutti e dieci, fine-riga
compresi.

**L'altra direzione:** in `docs/HANDOFF.md` del clone, con Python e `newline=""`, `40 ADR in stato` → `41 ADR in stato`, un
carattere. Lo stesso comando rende `docs/HANDOFF.md | 2 +-`, `1 file changed` — quel file e nessun altro. Rimesso dalla copia:
il comando torna vuoto, `cmp` dà il file identico alla copia, CR `1408` = righe `1408`. Il clone è rimasto com'era dopo la
rifazione, e lì resta.

## 5. La rilettura del merito — §2.4 del mandato

### 5.1 (a) ADR-0040 contro la 3.2 e la 3.3

| Nella 3.2 | In ADR-0040 | Esito |
|---|---|---|
| punto 1, i dati del programma nella cartella per utente, per natura; la configurazione porta la root; D4, K1 | uguale; D4 · K1 | ✅ — K1 è *«i dati del programma non hanno una casa»* |
| punto 2, router in `.<nomeapp>/` nascosta; indice nei dati che si rifanno, la cache su Linux, uno per root; D4, K30, K32 | uguale, più *«su Windows il punto nel nome non basta»* | ✅ — l'aggiunta è della 1.1 del disegno, riga 94, e di K30 |
| punto 3, i file del proprietario al loro posto, riferiti dal giornale, fuori dal backup; l'eccezione `.<nomeapp>/`; D12, D17 | uguale | ✅ |
| punto 4, il backup: giornale cifrato, configurazione, router; mai segreti, indice, pesi; dice che cosa resta fuori; D12, ADR-0022 | uguale | ✅ |
| punto 5, le copie nella cartella dati, in chiaro, fuori dal backup, potate come ADR-0018, mai quelle in dubbio; dopo un ripristino il passo non si annulla; K53 | uguale, coi link ad ADR-0024 e ADR-0018 | ✅ — nel merito vedi R8 |
| punto 6, la cartella dati percorso protetto e privato; la regola nel rimando di ADR-0016; D19 | uguale | ✅ nel merito; R4 per il tempo |
| le cinque *Negative (accettate)* | le cinque, ⚠️ *Dedotto* sulla seconda | ✅ |
| le quattro alternative | le quattro, coi *Contro* | ✅ — i *Contro* delle prime tre sono i costi delle B di D4, D12 e D17 nella consegna del brainstorming, detti in breve; il quarto è della 3.2 |
| il seguito: il limite di ADR-0024, K21; la ritenzione, ADR-0034; chi costruisce, la sezione 4 | i tre, più *«e la cartella nascosta dei router»* e *«il sotto-progetto 6, se nessuno le porta prima»* | ⚠️ R6 |

**Contro la 3.3:** la riga `- **Modifica:**` in testa, sotto `Deciders`, con le **stesse tre righe** del rimando di ADR-0022 —
confrontate parola per parola —; `- **Status:** Accepted`; la forma *Amends* della 3.4, come D3 la traduce. ✅

**Il *Context*:** le tre risposte della tabella sono quelle di D4, D12 e D17 nel disegno, riassunte senza cambiarle — di D4
manca la metà sul backup, che D4 stessa rimanda a D12 —, e le date `2026-09-29` sono quelle delle tre righe. La frase del
sorgente c'è, alla riga 196 di `crates/daemon/src/main.rs`, e i due file relativi alla cartella da cui il daemon parte ci sono.
Lo stato dell'arte si ritrova nella sezione delle fonti del disegno di `riferimenti.md`, tutto tranne *«in nessun backup»*: R9.
I nove link di ADR-0040 portano a file veri.

### 5.2 (b) Il rimando di ADR-0022, contro i fratelli e contro i due rimandi

| Contro | Che cosa ho cercato | Esito |
|---|---|---|
| ADR-0018 | la potatura irreversibile e dichiarata, *«un payload assente e un payload mai registrato non devono essere indistinguibili»*, `InDubbio` mai potabile; gli artefatti *«non nel giornale: riferimenti»* | ✅ nessun urto: il rimando lascia *«il giornale cifrato»* e la ritenzione; ADR-0040 riprende le tre regole per le copie; *«riferiti dal giornale»* è la riga «artefatti» di ADR-0018 |
| ADR-0023 | *«Cosa si cifra»* rimanda alla tabella di ADR-0022: il giornale sì, i segreti con chiave propria, indici e pesi no; il profilo «riservato»; il rimando del 2026-08-18 sul permesso | ✅ il rimando dice che reggono *«il giornale cifrato, i segreti mai»*; nessuna riga di ADR-0023 dipende da artefatti o guide nel backup. Il rimando del 2026-08-18 non urta: dà però la misura di R8 |
| ADR-0024 | l'ambito dichiarato, la versione di prima *«conservata e riferita dal passo»*, la potatura con la logica di ADR-0018, il limite di dimensione | ✅ il rimando non tocca le copie; ADR-0040 le colloca, e il rimando di ADR-0024 del compito 2 dice *«le copie le dice ADR-0040»* — letto nel blocco E2 |
| il rimando del 2026-09-08, in testa | le guide *«artefatti dell'utente»*, *«la politica della riga — in chiaro, nel backup, permanente — non cambia»*, *«Nessuna riga di questo ADR è superata»* | ✅ la citazione è esatta, coi puntini per l'inciso; il rimando nuovo lo fa leggere con ADR-0040 per le guide. La sua frase in grassetto sulla non-superazione resta vera **alla sua data**: il rimando nuovo le sta sotto, datato — D5 —, e D3 lo chiude con *«Le altre righe reggono»* invece di ripeterla |
| il rimando del 2026-08-07, in fondo | *«sotto-progetto 11 … dopo il 5, il 6 e il 9»*; la non-vacuità di V32 su indici e pesi | ✅ *«il 5 esce»* è la seconda risposta della 5.5, ✅ **A** il 2026-09-29, e la sua ragione è D15, non ADR-0040: il rimando la attribuisce alla 5.5, giusto. La non-vacuità regge: V32 esclude indici e pesi, che portano il 6 e il 9 |
| ADR-0040 | le tre righe nominate; i file del proprietario fuori; l'eccezione dei router; *«le altre righe reggono»*; lo stato | ✅ il rimando dice delle tre righe ciò che dice ADR-0040 — la riga `Modifica:` e il punto 3 —, con le stesse parole |

Le citazioni di ADR-0022 nel rimando sono esatte: la conseguenza è tagliata prima di *«, e l'indice si ricostruisce»*, che
regge. `git diff 8bb8440..025dc0f -- docs/adr/ | grep -c '^-[^-]'` → `0`: **nessuna riga preesistente** di ADR-0022 è
cambiata, e il rimando sta sotto il primo e sopra `## Context`, come vogliono D5 e il vincolo 9.

### 5.3 (c) Il compendio

- **La voce di ADR-0040:** ogni frase ha la sua riga in ADR-0040 — la modifica in tre punti e lo stato di 0022 dalla riga
  `Modifica:`; *«giornale, configurazione, indice, copie del checkpoint»* nella cartella dati dai punti 1, 2, 4 e 5 e da D4 del
  *Context*; i file del proprietario e i router dal punto 3; il backup dal punto 4; le copie dal punto 5; il percorso
  protetto dal punto 6. Niente in più; in meno, la compressione di sempre — la cartella nascosta, *uno per root*, le negative.
- **La riga nella voce di ADR-0022:** dice ciò che dice il rimando, e il `grep` di ER-4 rende `1`.
- **I totali:** quaranta nella §5, con *«l'ultimo, 0040, il 2026-09-30»* vero — è la `Date` di ADR-0040 —, e nella §8; la
  guardia li confronta coi file, e `ls docs/adr/*.md | wc -l` → `40`.
- **La riga della §13:** *«la voce resta e riceve la riga del rimando; l'ADR nuovo ha la sua voce, che dice quali righe
  modifica»* è coerente con la riga di `CLAUDE.md` — *«un rimando in testa che nomina le righe, e l'ADR nuovo lo dichiara — lo
  stato resta `Accepted`»* — e con la 3.3 e D4, *«la forma di ADR-0040 su ADR-0022»*. Il nome del caso è *superato in parte*
  nelle due regole e *modificato (in parte)* nell'istanza: lo vuole la 3.3 stessa, e le regole legano i due nomi con la forma di
  ADR-0040 su ADR-0022.
- **La riga della data:** dopo il commit dice il vero — il piano in esecuzione, le quattro cose di ADR-0040 nel compendio, fin
  dove lo dice la posizione, la riga di prima in archivio, com'era: ER-10 → `SAME`.

### 5.4 (d) `AVVIO-CHAT.md`

`awk` sui recinti: il richiamo nuovo è alla riga 15, fuori e in testa — il primo recinto si apre alla riga 41 —; la riga del
messaggio senza cifra è la 165, **dentro** il recinto delle righe 41–412. Il comando B rende niente, e la sua controprova sui
blob di `8bb8440` rende la riga 163 di allora. Il richiamo porta `le 39 ADR` in un code span che non va a capo — trappola 1 della
§10 —, e la forma è quella del richiamo del 2026-09-01. Il posto: R10.

### 5.5 (e) Nessuna frase falsa dopo il commit, nei dieci file

| Che cosa | Esito |
|---|---|
| un totale rimasto a trentanove, a cifre o a parole | ✅ nessuno: la guardia; `grep -rn -i 'trentanove'` e le cifre `39` seguite da ADR fuori da archivio, piani e disegni rendono solo il code span voluto di `AVVIO-CHAT.md` e una riga di `riferimenti.md` che non parla di ADR |
| un *«l'ultimo»* degli ADR rimasto indietro | ✅ l'unico è quello della §5, riscritto; le menzioni di 0039 nei documenti di stato non lo dicono l'ultimo |
| ciò che il commit dichiara fatto, contro git e contro la posizione | ✅ la riga del compito 1 — *uno*, ✅ 2026-09-30 — torna con `git log --grep` e con la data del commit; l'*«Ultimo aggiornamento»* della roadmap e la riga della data del compendio dicono ciò che il commit ha fatto |
| il capoverso sopra la tabella della posizione | ❌ R1, e R2 per il compito 6 |
| il puntatore della §6 e il *«Come si riprende»* del piano | ⚠️ R3, cura dettata alla chiusura della sessione |
| il rimando ad ADR-0016 di ADR-0040 e della sua voce | ⚠️ R4, fino al compito 2 |
| la regola per rimettere in discussione un ADR, nella §7 e in `HANDOFF.md` | ⚠️ R5 |
| la cella del piano nella tabella dei piani di `roadmap.md`, *«⏳ scritto il 2026-09-29; il pre-controllo e l'esecuzione, in sessioni loro»* | ✅ indietro ma **dichiarata**: la decisione 4 del *«Come si riprende»* del piano la lascia così fino al compito 6 |

### 5.6 Il criterio di chiusura del compito 1, riga per riga, e le due voci del rapporto

| Riga del criterio | Esito |
|---|---|
| ADR-0040 esiste, `Accepted`, con `Modifica:` e le *Negative (accettate)*, e dice punto per punto la 3.2 | ✅ — §5.1; R6 sul seguito |
| ADR-0022 porta in testa il rimando *«modificato da ADR-0040»*, e nessuna riga preesistente è cambiata | ✅ — `1`; `0` righe tolte |
| la revisione ha riletto ADR-0040 contro la 3.2 e la 3.3, e il rimando contro ADR-0018, ADR-0023, ADR-0024 e i due rimandi | ✅ — questa revisione, §5.1 e §5.2 |
| i totali a quaranta; la cifra tolta da `AVVIO-CHAT.md`, col richiamo; *superato in parte* in `CLAUDE.md` e nella §13 | ✅ — sette righe a quaranta; B vuoto; `1` e `1` |
| `check-docs.sh` → `OK`, `GATE GREEN`, fine-riga e tabelle come al Passo 1, commit pushato, posizione aggiornata | ✅ `OK`, ✅ `GATE GREEN`, ✅ fine-riga — letti come ER-11 —, ✅ tabelle; ⏳ **il push** è del coordinatore dopo questa revisione, per contratto; ⚠️ **la posizione:** la riga sì, il capoverso sopra no — R1 |

**ER-11 — valida.** Il vincolo 7, *«dopo il compito i CR sono **quanti prima**»*, e l'Atteso del Passo 4 del compito 1,
*«la colonna `w/…` e i CR come al Passo 1»*, alla lettera falliscono su ogni file che guadagna righe; i compiti 2–6 scrivono
*«fine-riga come al Passo 1»*, che regge, quindi la voce tocca il vincolo 7 e il compito 1. Il testo raffinato, che copre anche un
file misto: *«dopo il compito, su un file che ha CR la differenza fra righe e CR è quella del Passo 1 — zero su un file tutto
CRLF —, e su un file senza CR i CR restano zero; la colonna `w/…` è quella del Passo 1»*. È il comportamento di
`apply_edits.py`, che converte in CRLF il testo nuovo quando il file ne ha.

**ER-12 — valida**, per il solo compito 1: `git ls-files` elenca l'indice, e al Passo 4 ADR-0040 non c'è ancora — lo mostrano
`step4.log`, otto righe, e la misura a parte con `-o --exclude-standard`, `i/ w/lf`. Il testo: al Passo 4 la seconda misura
`git ls-files --eol -o --exclude-standard docs/adr/0040-*.md`, atteso `i/ w/lf`; dopo il `git add`, `i/lf w/lf`. Nessun altro
compito crea file: `grep -n '^- Create:'` nel piano ne trova uno solo.

**Il resto del rapporto:** il pre-volo dei due `--check` prima del Passo 2 è fuori dalla lettera, di sola lettura e dichiarato:
nessun rilievo. Lo scivolone sulla controprova, *«must list the 10 paths»* con `9`, l'ha già detto l'implementatore.

## 6. Ciò che non ho potuto verificare, e perché

- **Lo stato dell'albero del repository prima del compito:** oggi è a `025dc0f`. Il *prima* l'ho misurato sul clone appena fatto
  da `8bb8440`, con lo stesso `core.autocrlf`, e torna col Passo 1 del rapporto — i CR del piano compresi, `2061` = righe, che il
  rapporto dava per dedotti.
- **Il modo di lavorare dell'implementatore:** niente subagenti, il cancello da solo, nulla su `gui/` mentre girava — li reggono
  soltanto l'ora del log, prima del commit, e l'albero pulito dopo.
- **Le approvazioni del proprietario:** le date di D4, D12, D17, D19 e della 5.5 le ho lette nel disegno, che le copia dalla
  consegna; non nella chat.
- **Le fonti di Claude Code *il 2026-09-29*:** le ho rilette oggi; il giorno prima potevano dire altro.
- **La simulazione P-19 di tutti i compiti:** è del coordinatore; io ho rilanciato solo il `--check` di E2 e di POS2 dopo il
  compito 1, e POS6 su due copie del piano.
- **Il push:** non fatto, per contratto — il criterio *«commit pushato»* resta aperto finché il coordinatore non lo fa.
- **Il cancello nel clone:** non l'ho girato, come dice il mandato; `check-docs.sh` sì, con Git Bash esplicito — la trappola di
  macchina in §3.9.

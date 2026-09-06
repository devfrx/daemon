# Sotto-progetto 2 — la GUI minima: la consegna dell'avvio

⚠️ **QUESTO FILE È NATO COME CONSEGNA, il 2026-09-06, della sessione che ha RIPRESO il repository
dopo la chiusura del piano dei documenti della knowledge base, ha raccolto il contesto del
sotto-progetto 2 e NON ha fatto il brainstorming:** il proprietario lo fa in una sessione fresca, e
ha chiesto che qui siano registrate **tutte le risposte scambiate**. La sessione del brainstorming
legge questo file **per intero**, e alla propria chiusura lo **riscrive sul posto** — come consegna
delle sezioni approvate, o come disegno se lo scrive lo stesso giorno — e sposta questo testo
**parola per parola** in `docs/archivio/consegna-avvio-brainstorming-sottoprogetto-2.md`, coi soli
link riscritti per la cartella. È il viaggio della consegna della knowledge base
(`07ab6dc` → `6a7967a`), e prima di quella dei gesti.

⚠️ **Non è una spec e non è un disegno.** Il prossimo passo sta nella §6 del
[compendio](../../COMPENDIO.md), in un posto solo.

## Stato in una riga

Il piano dei documenti della knowledge base è chiuso, E10 è decisa, il contesto del sotto-progetto 2
è raccolto e verificato, il percorso è classificato **architetturale**: manca il brainstorming, che
comincia dalla **domanda 1** qui sotto, ancora senza risposta.

## ⛔ Da sapere subito

**Niente è a metà.** Albero pulito, nessuno stash, nessuna operazione git a metà, tutto pushato,
**nessun codice toccato** in questa sessione. Il ledger `.superpowers/sdd/` è git-ignorato e vive su
questa macchina: toglierlo è un comando del proprietario, non una scelta da prendere per lui.

## Stato del repo alla chiusura, coi comandi che lo rifanno

| | Comando | Atteso |
|---|---|---|
| ramo | `git fetch --all --prune`, poi `git status -sb` | `## main...origin/main`, niente sotto |
| i commit di questa sessione | `git log --oneline 57785b0..HEAD` | E10 decisa (`af594c7`) e questa chiusura |
| codice e spec non toccati | `git diff --stat 57785b0..HEAD -- crates/ scripts/ Cargo.lock Cargo.toml rust-toolchain.toml docs/superpowers/specs/2026-08-06-kernel-design.md docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` | nulla |
| cancello | `bash scripts/gate.sh` | `GATE GREEN` — rilanciato all'apertura su `57785b0` e di nuovo prima del commit di questa chiusura. Si rilancia, non si cita |
| documenti | `bash scripts/check-docs.sh` | `OK` |
| fine-riga | `git ls-files --eol docs/COMPENDIO.md docs/superpowers/plans/2026-09-04-knowledge-base-documenti.md docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` | il compendio `i/lf w/crlf`, gli altri due `i/lf w/lf` |
| margine del compendio | `wc -c docs/COMPENDIO.md` contro `grep -n '^ceiling=' scripts/check-docs.sh` | positivo |

La baseline dei test la dà `cargo test --workspace --no-fail-fast --locked`, non questa riga.
⚠️ Un `awk` sulle righe `test result` del log del cancello **conta due volte** i bersagli che il
passo 7 rilancia con `--nocapture`: non è la quaterna.

## Fatto in questa sessione

1. Ripresa sulla consegna della terza sessione di esecuzione del piano della knowledge base: ogni
   riga rilanciata coi comandi, **nessuna divergenza** fra consegna e repository.
2. **E10 decisa dal proprietario: A** — la riga «Registro delle guide (13) dopo la GUI minima…» di
   «Perché quest'ordine» in [`roadmap.md`](../../roadmap.md) resta com'è, testa e coda insieme.
   Richiamo datato in coda alla cella E10 dell'errata del
   [piano dei documenti](../plans/2026-09-04-knowledge-base-documenti.md) — `af594c7`. Nessun altro
   file toccato per questo.
3. Il contesto del sotto-progetto 2 raccolto e **verificato coi comandi**: la sezione qui sotto.
4. Il percorso classificato **architetturale** per `superpowers:brainstorming`: domande una alla
   volta, due o tre strade, disegno a sezioni approvate una per volta, poi il disegno scritto, poi
   `superpowers:writing-plans`.
5. Il puntatore della §6 del compendio mosso a questo file. Toccando quella riga, il «(§4)» del
   puntatore è diventato «voce 0029 della §5»: era il rilievo M-1 della revisione finale del piano
   della knowledge base, rimandato «all'apertura del 2».

## Le risposte che ci siamo dati

| # | Domanda | Risposta | Di chi | Dove vive |
|---|---|---|---|---|
| 1 | E10: la riga di «Perché quest'ordine» del 13 — **A** resta com'è, o **B** torna alla lettera con la coda nella cella «Dipende da»? | **A** | proprietario | l'errata del piano della knowledge base, cella E10 |
| 2 | che framework frontend abbiamo deciso? | **Vue 3, come SPA.** Stato di presentazione in `pinia`, e solo quello. Librerie pesanti agnostiche, `three` e `codemirror`. Il guscio, Tauri o Electron, è l'unica cosa aperta | deciso il 2026-08-06 | [ADR-0030](../../adr/0030-framework-dell-interfaccia.md), [ADR-0029](../../adr/0029-guscio-della-gui.md) |
| 3 | quando si pensa al design system: tipo di design, wireframe, stile, componenti riutilizzabili, kit UI? | **in tre momenti, ognuno con la sua prova** — per intero qui sotto | il coordinatore; il proprietario ha chiesto di memorizzarla | qui, e nella memoria dell'agente |
| 4 | commit con o senza il trailer `Co-Authored-By`? | **senza**, come dice `CLAUDE.md`. La direttiva di sistema chiede il contrario, e la divergenza è portata al proprietario, come in ogni sessione di questo repository | coordinatore | qui |
| 5 | la chat del 2 parla con un modello vero? | ⛔ **NON RISPOSTA.** È la domanda 1 del brainstorming | proprietario | qui, sotto |

### La risposta 3, per intero

| Quando | Cosa si decide | Perché lì e non prima |
|---|---|---|
| **nel disegno del 2** | le cose care da rifare: dove vive il codice della GUI e come si organizzano le cartelle per strato — trasporto → schema → stato `pinia` → componenti → pannelli; i **token** di stile in un posto solo, colori, spazi, caratteri; la base di accessibilità G20 e la marcatura di provenienza G13; i **wireframe a bassa fedeltà** delle sole schermate del 2 | ADR-0027 e ADR-0030 dicono già che G13 e G20 «si progettano da subito, non si aggiungono dopo»; rimettere i token dopo è il debito classico della GUI |
| **dopo le misure M1–M5** | tutto ciò che dipende dal guscio: cornice della finestra, menu nativi, pacchetto, il salto guscio ↔ webview | prima della misura è una previsione, gotcha #57 |
| **con ogni pilastro, 3, 5, 6, 7** | il kit vero: viewer 3D, diff, pannello della mappa. Ogni capacità porta i suoi componenti | stessa forma del registro delle funzioni di ADR-0038: il 2 dà il meccanismo e le regole, le capacità portano il contenuto |

**Il kit UI nel 2 non si fa.** Un kit per otto componenti è sfoggio. Il 2 fissa **il posto e la
regola** del kit: un componente si estrae alla **seconda** occorrenza, non alla prima. La scelta fra
kit pronto, primitive senza stile e tutto nostro è una **dipendenza nuova**, quindi del proprietario:
domanda 7 qui sotto, con versioni e stato dell'arte verificati **quel giorno**. I wireframe si
mostrano nel browser, col visual companion della skill, quando si arriva alla forma delle schermate.

## Il contesto raccolto — cosa i documenti e il codice dicono già che il 2 deve costruire

Verificato il 2026-09-06, ogni riga col comando o col file letto.

| Pezzo | Deciso da | Esiste oggi? |
|---|---|---|
| il guscio, Tauri o Electron, chiuso **con le misure M1–M5**: stesso frontend minimo — chat in streaming più una scena three.js — sui due gusci; RAM, pacchetto, fps e API grafica su Windows **e** Linux, P3 col rendering vero, VRAM | [ADR-0029](../../adr/0029-guscio-della-gui.md), `Proposed` | no. ⚠️ Le versioni verificate nell'ADR sono del **2026-08-06**: si riverificano alle fonti primarie prima di misurare |
| il trasporto IPC vero: named pipe su Windows, socket unix su Linux | spec §6.1; `crates/kernel/src/ports/ipc.rs` | no: `grep -rnE "^ *impl Ipc for" crates/` rende **due finte** — `FakeGui` nel banco e `DyingGui` nel simulatore — e nessun trasporto. `crates/platform/src/` contiene solo `journal.rs`, `reactor.rs`, `rng.rs`. Il prototipo che ha passato P1–P4 sta in `spikes/gui-ipc/`, con `interprocess = "2.4"` |
| il timbro di build che rifiuta una GUI vecchia senza versionare | spec §6.1.2 | no: dichiarato «non costruito», con l'innesco scritto in testa a `crates/kernel/src/wire/ipc.rs` — **il guscio, cioè il 2** |
| il messaggio di revoca core → GUI per il 3D oltre quota | [ADR-0033](../../adr/0033-gpu-della-gui-quota-di-presentazione.md) | no, stesso file e stesso innesco: «una revoca ha bisogno di un destinatario» |
| la metà che **verifica** la riserva dichiarata dalla GUI | ADR-0005, doc di `GrantRequest` in `wire/ipc.rs` | no: `compute_class` e `preemption` arrivano dal pari e l'arbitro li obbedisce senza controllarli; l'innesco è **il primo consumatore che decodifica i byte in un profilo** |
| `ClientId` preso dal contatore progressivo del giornale | spec §6.1.3, doc di `ClientId` | il contatore **non esiste ancora**: chi implementa la porta attinge da quello, mai da un contatore privato |
| lo specchio TypeScript dello schema `IpcMessage`, `bincode` | [ADR-0027](../../adr/0027-stack-della-gui.md), costo accettato «due definizioni da tenere allineate» | no. Il decodificatore misurato in M-11, `bincode-ts` 1.0.0, è a **una sola versione** e ha **entrambi** i punti d'ingresso rotti su Node 24 — spec §6.10.6 |
| la SPA Vue 3: chat in streaming, G4, e il pannello di stato, G9–G16 | [ADR-0030](../../adr/0030-framework-dell-interfaccia.md), [`GUI-REQUISITI.md`](../../../spikes/GUI-REQUISITI.md) | no. Non esiste nessuna cartella della GUI nel repository |
| il registro delle funzioni, col **click** come primo invocatore: registrazione, invocazione, la tripla di ADR-0016, il giornale | [ADR-0038](../../adr/0038-registro-delle-funzioni-del-programma.md) | no. La manipolazione della GUI — pannelli, menu — **non** passa dal registro |
| P3 rimisurato col rendering vero e col salto webview: era **21,43 %** su 25 nel prototipo, senza rendering | ADR-0027, follow-up; è M4 di ADR-0029 | — |
| G13 provenienza visibile e G20 tastiera, screen reader, contrasto, **progettati da subito** | ADR-0027 e ADR-0030, follow-up | — |
| la riconciliazione alla morte della GUI | `crates/kernel/src/client.rs`, `ClientGrants::on_disconnect` | **sì**, esiste: risponde a `IpcError::Disconnected` con i rilasci; chi sonda la porta la chiama |
| la quota di presentazione, permanente, tenuta dal core | ADR-0033; il grafo di produzione in `crates/daemon/src/main.rs` | **sì**, dal Task 10 del Traguardo 5: la GUI la consuma senza chiederla |
| lo stato di degrado e i permessi come oggetti | `crates/kernel/src/degradation.rs`, `permission.rs` | **sì**, nel kernel; nessuno li porta ancora sul filo |

**Cosa NON esiste, e nessuna riga assegna al 2:** la rete verso OpenRouter — nessun
`impl Network for` in `crates/platform/src/`; i segreti — `crates/secrets/src/lib.rs` è vuota per
decisione; un giro di run. È la sostanza della domanda 1. La roadmap dice che il primo valore utile
è **1 + 2 + 3**, e che Conversazione è la prima capacità «perché prova tutti i meccanismi del kernel».

**Cosa gli altri disegni si aspettano dal 2**, letto nei loro file:

| Da | Che cosa | Dove |
|---|---|---|
| il disegno dei gesti | pannelli e menu che si muovono con **qualunque puntatore**; la mano è un puntatore in più e la aggiunge il 12. La variante di `IpcMessage` con la mano **si definisce quando la GUI esiste**, non prima. Il registro delle funzioni nasce col click | [disegno](2026-09-03-riconoscimento-gesti-design.md), §4 |
| il disegno della knowledge base | il pannello della mappa nasce **col 6, non col 2**: il 2 disegnerebbe un indice che nessuno produce ancora. «Aggiungi al contesto» avrà due invocatori, il click e il modello, sullo stesso registro | [disegno](2026-09-04-knowledge-base-design.md), domanda 12 e §2.3 |
| la roadmap | il **10**, integrazione OS, e il **12**, gesti, dipendono dal 2; il **6** ne dipende per il pannello; il **13** viene dopo il 2 e prima del 3, sbarrato da AUD-004 e non dal 2 | [`roadmap.md`](../../roadmap.md), tabella dei sotto-progetti |

**Cosa si è letto, coi range, perché non si rilegga tutto:** `spikes/GUI-REQUISITI.md` intero;
ADR-0027, 0029, 0030 e 0038 interi; ADR-0033 da «Decision» in poi; `docs/design/01-topologia-dei-processi.md`
intero; `docs/roadmap.md` righe 150–200; la spec del sotto-progetto 1 alla §6.1, righe 1854–1958;
`crates/kernel/src/ports/ipc.rs` righe 1–185 e il tratto alla 222; `crates/kernel/src/wire/ipc.rs`
e `crates/kernel/src/client.rs` interi; nel disegno dei gesti le righe 119, 223, 319–331 e 372; nel
disegno della knowledge base le righe 73, 79, 192, 362, 510, 512 e 530. ⛔ I numeri di riga sono di
`af594c7`: si ritrovano col `grep` sulla frase, non si citano.

## Le domande del brainstorming, una alla volta — proposte del coordinatore, non decisioni

Ogni domanda a parole di tutti i giorni, due o tre opzioni di una riga, il consiglio in una riga: è
la forma che il proprietario vuole. Le domande 4, 6 e 7 toccano una dipendenza nuova o il cancello,
e sono **sue** per costruzione.

| # | Domanda | Opzioni | Consiglio, e perché |
|---|---|---|---|
| **1** | la chat del 2 parla con un modello vero? | **A** no: il 2 costruisce la finestra della chat e il filo col core; nei test e nello spike il flusso lo manda un core finto; il modello vero arriva col 3. **B** sì: il 2 costruisce anche la rete, la chiave nei segreti e il primo giro di run | **A**. Il valore utile è 1+2+3 e Conversazione è la prima capacità; con B il 2 si mangia metà del 3 |
| 2 | quale «stato» mostra il 2? | **A** solo ciò che il kernel ha oggi: degrado, i verdetti dell'arbitro con `Rifiutata` e `InCoda` distinti (G15), la policy VRAM e il budget, la riga «protetto quanto il tuo account» (G16). **B** anche pannelli vuoti per G10–G12, G17, G18 | **A**. Un pannello che mostra ciò che nessuno produce è la ragione per cui la mappa nasce col 6 e non col 2 |
| 3 | in che ordine dentro il 2? | **A** prima lo spike M1–M5, che chiude ADR-0029, riusando l'emettitore di `spikes/gui-ipc/`; poi il trasporto in `platform`; poi la SPA. **B** prima il trasporto, e lo spike sul trasporto vero | **A**. ADR-0029 dice «all'inizio», e «con una misura, non con una discussione»; lo spike non ha bisogno del trasporto di prodotto |
| 4 | chi decodifica `bincode` dal lato della GUI? | dipende dal guscio: con Electron il processo Node, via `bincode-ts`, fermo e fragile; con Tauri il guscio è **Rust** e può decodificare con `IpcMessage::decode` del kernel, zero decodificatori nuovi. La tabella di ADR-0029 **non lo elenca** | non si decide: si **registra** come riga qualitativa del protocollo di M1–M5, e si verifica prima che ADR-0027 «nessun tipo condiviso» non lo escluda — parla dell'interfaccia web, non del guscio |
| 5 | qual è la prima funzione del registro? | **A** il cambio di policy VRAM: `Arbiter::set_policy` esiste, è un passo giornalato, ADR-0006 la vuole «offerta all'utente», e non ha nessun chiamante di produzione — `grep -rn set_policy crates/daemon/src/ crates/platform/src/` non rende nulla. **B** la richiesta di concessione per il 3D oltre quota, `GrantRequest` | **A**. Il 3D non è nella GUI minima; la policy sì, ed è l'unico effetto vero che la GUI possa invocare oggi |
| 6 | dove vive il codice della GUI, e chi lo controlla? | **A** una cartella `gui/` alla radice, fuori dal workspace Cargo, con la propria toolchain, e un passo del cancello per lei. **B** dentro `crates/`, che è Rust | **A**. ⛔ Un passo nuovo del cancello è del proprietario, vincolo globale 7 |
| 7 | il kit UI? | **A** kit pronto, tipo PrimeVue o Vuetify. **B** primitive senza stile che danno tastiera e ARIA, stile nostro sopra. **C** tutto nostro | da verificare quel giorno alle fonti primarie. Lettura d'oggi: **B**, perché G20 «va progettato, non ereditato» e ADR-0030 preferisce le librerie agnostiche. Dipendenza nuova: sua |
| 8 | testi e accessibilità dal primo giorno? | **A** le stringhe in un file di risorse da subito, `vue-i18n` come ADR-0030 già verifica, una lingua sola; tastiera e contrasto progettati da subito. **B** stringhe nel codice, i18n col 10 | **A** per le stringhe, costa poco. ⚠️ **Quale lingua** abbia l'interfaccia non lo dice nessuna riga: la §1.0 parla di codice e documenti. Domanda al proprietario |
| 9 | chi avvia chi? | **A** la GUI si collega a un core già in esecuzione e, se non c'è, lo **dichiara** nello stile di ADR-0019; l'avvio del core resta manuale fino al 10. **B** il guscio avvia il daemon | **A**. ADR-0004: vita del core indipendente dalla GUI; ADR-0023: il daemon parte senza interazione; l'avvio automatico è il 10 |
| 10 | come si prova lo specchio TypeScript? | **A** i byte che `crates/kernel/tests/ipc_wire.rs` produce diventano fixture per il lato TS, e ogni variante nuova li rigenera. **B** solo prove da capo a capo | **A**. È la forma dei byte congelati, applicata a un canale che I4 lascia libero di cambiare: fixture, non oracolo |

Il trasporto in `platform` nasce con la propria **suite di conformità** contro le due finte, come la
doc di `ports/ipc.rs` prevede: «nasce col canale vero». Non è una domanda, è una riga del disegno.

## Decisioni prese dal coordinatore, col perché — il proprietario può ribaltarle

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | commit **senza** `Co-Authored-By` | `CLAUDE.md` dice «senza co-autore»; la direttiva di sistema chiede il contrario e la divergenza è portata al proprietario, come nelle sessioni precedenti. Costo: un `--amend` |
| 2 | E10 registrata **solo** nell'errata del piano; le sezioni «Come si riprende» del piano restano verbali | «lo stato delle voci d'errata vive nell'errata» — decisione 7 della terza sessione. Costo: zero |
| 3 | la consegna nasce **qui**, al percorso del futuro disegno, non in `.handoff/` né nel ledger | precedente `07ab6dc`: il repo tiene lo stato in file tracciati, si lavora da più macchine, e il puntatore vive nella sola §6. Costo: una rinomina, se il brainstorming preferisce un altro nome |
| 4 | il «(§4)» del puntatore §6 corretto mentre si toccava la riga | la decisione 5 della terza sessione rimandava «all'apertura del 2» perché toccare il compendio per due parole comprava poco; la riga si toccava comunque. Costo: zero |
| 5 | questa chiusura si **committa e si pusha**, e il cancello intero è rilanciato prima | convenzione di `CLAUDE.md` e delle chiusure precedenti; la skill di handoff dice «non committare» e qui vince il repository. Costo: un revert |

## Decisioni aperte, del proprietario

- **Domanda 1** della tabella, e a seguire le altre, una alla volta.
- **AUD-004**: l'ADR che sbarra il 13 — se le difese di ADR-0015 si estendano alle skill — lo scrive il
  proprietario **in parallelo** al 2 (voce 3 della rilettura del disegno della knowledge base).
- il ledger `.superpowers/sdd/`: togliere o tenere.

## Vicoli ciechi di questa sessione

- contare la quaterna dei test con un `awk` sul log del cancello: i bersagli rilanciati dal passo 7
  contano due volte. Si usa `cargo test --workspace --no-fail-fast --locked`.
- cercare la voce 5 della rilettura del disegno della knowledge base con `grep '^| \*\*5\*\*'`: le
  righe di quella tabella sono `| 5 |`, senza grassetto.

## Prossimo passo, eseguibile

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di
   questa chiusura o uno successivo.
2. La lettura obbligatoria di `CLAUDE.md`: il compendio per intero, a blocchi, e la testa dell'audit
   del 2026-08-27.
3. **Questo file, per intero.** Poi le skill: `anthropic-skills:decision-principles`,
   `anthropic-skills:session-resume` su questo file, `anthropic-skills:dev-discipline`,
   `anthropic-skills:dev-communication`, e `superpowers:brainstorming` — percorso architetturale.
4. Al proprietario la **domanda 1**, A o B, col consiglio A. Poi le altre, una per messaggio.
5. Alla chiusura: riscrivere questo file sul posto, spostare questo testo parola per parola in
   `docs/archivio/consegna-avvio-brainstorming-sottoprogetto-2.md` coi link riscritti, muovere il
   puntatore della §6 del compendio, `bash scripts/check-docs.sh`, commit, push.

## Come tornare operativi

```bash
git fetch --all --prune && git status -sb && git log --oneline -3
bash scripts/check-docs.sh
bash scripts/gate.sh
```

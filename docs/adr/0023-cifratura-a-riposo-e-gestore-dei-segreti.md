# ADR-0023: Cifratura a riposo con chiavi dell'OS, e gestore dei segreti unico

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

Tre requisiti strutturali del progetto entrano in tensione diretta con la cifratura:

| Requisito | Implica |
|---|---|
| daemon con avvio automatico (ADR-0004, R2) | il core parte **senza interazione umana** |
| voce always-on | deve funzionare da subito dopo il boot |
| local-first | i dati non lasciano la macchina |

Se l'archivio è cifrato con una passphrase, qualcuno deve digitarla a ogni avvio — e
i primi due requisiti cadono.

Lo stato dell'arte per applicazioni desktop i cui dati **non vengono mai letti fuori
dalla macchina** è consolidato: cifratura simmetrica dei dati, con la chiave affidata
alle facility dell'OS. Su Windows queste cifrano con le credenziali di accesso
dell'utente e **non espongono mai la chiave all'applicazione**. La stessa letteratura
distingue con chiarezza ciò che va protetto — token, credenziali, chiavi API — da ciò
che può restare in chiaro: URL, nomi, flag di configurazione.

Alternative considerate:

- **Passphrase all'avvio.** Protezione indipendente dall'account OS.
  *Contro:* rompe avvio automatico, daemon e voce always-on. Tre requisiti
  strutturali, non tre comodità.
- **Chiavi affidate all'OS.** Il daemon parte senza interazione.
  *Contro:* la protezione **equivale a quella dell'account OS**.
- **Nessuna cifratura.** *Contro:* il giornale contiene prompt, risposte e contenuto
  di documenti in chiaro sul disco.

## Decision

**1. Le chiavi sono gestite dalle facility dell'OS**, raggiunte attraverso il modulo
di piattaforma (I3). Il daemon parte senza interazione.

**2. Onestà sulla forza reale.** In questa configurazione «cifrato a riposo» significa
**protetto quanto il tuo account di sistema**: chi ottiene il tuo account ottiene i
dati. Va scritto **in interfaccia**, non solo qui — perché la parola «cifrato» suona
più forte di quanto sia, e una falsa sicurezza è peggio di nessuna sicurezza.

**3. Cosa si cifra e cosa no** — vedi la tabella in
[ADR-0022](0022-layout-dei-dati-per-natura-e-backup-dichiarato.md). In sintesi: il
giornale sì (contiene contenuto), i segreti sì e con chiave propria, gli indici e i
pesi no.

**4. Il gestore dei segreti è l'unico punto di lettura delle credenziali.** Nessun
altro componente ha un percorso per leggerle. Da questo punto unico discendono tre
meccanismi già decisi:

| Meccanismo | Deciso in |
|---|---|
| mascheratura nel record di routing | V16 · ADR-0011 |
| escalation automatica dei vincoli sui dati | ADR-0016 |
| canary di esfiltrazione | ADR-0016 |

Nessuno dei tre funzionerebbe se le credenziali si potessero leggere da più punti: è
il punto unico a renderli possibili, non il contrario.

**5. Profilo «riservato», opzionale:** cifratura con passphrase, che **disattiva avvio
automatico e voce always-on**. La composizione è esplicita e mutuamente esclusiva —
non si possono avere entrambe, e fingere il contrario sarebbe disonesto.

## Consequences

- **Positive:**
  - Avvio automatico e voce always-on restano possibili con i dati cifrati.
  - La chiave non passa mai dall'applicazione: un difetto del nostro codice non la
    espone.
  - Il punto unico di lettura rende la mascheratura verificabile staticamente, non
    per disciplina.
- **Negative (accettate):**
  - **La protezione equivale a quella dell'account OS.** Non protegge da un attaccante
    che ha già il tuo account, e non protegge un disco rubato se l'account è debole.
  - Il profilo «riservato» rinuncia a tre funzionalità: è una scelta, non un difetto,
    ma va presentata come tale.
  - Un cambio di credenziali di sistema può richiedere una ri-derivazione delle
    chiavi: va gestito, o l'utente si ritrova con dati illeggibili dopo un banale
    cambio di password.
- **Follow-up richiesti:**
  - La frase mostrata in interfaccia va scritta con cura: deve dire *da cosa protegge*
    e *da cosa no*, in una riga.
  - Nel profilo «riservato», il sistema deve **rifiutarsi** di abilitare l'avvio
    automatico, non limitarsi a sconsigliarlo.

---

## ✅ Rimando — «protetto quanto il tuo account» ha un PERMESSO DI FILE, e non ce l'aveva (2026-08-18)

Questo ADR dichiara che «cifrato a riposo» qui significa **protetto quanto il tuo account di
sistema**, e pretende che la frase sia scritta **in interfaccia** perché una falsa sicurezza è
peggio di nessuna sicurezza. ⛔ **Non diceva però nulla sui permessi del file**, e il file del
giornale nasceva **0644 su Linux** — cioè **meno** dell'account: leggibile da qualunque utente
della macchina. Finding **PL-1** dell'[audit del 2026-08-11](../audit-2026-08-11.md).

| | |
|---|---|
| **la causa** | `OpenOptions::create(true)` chiede al sistema `0o666 & !umask`. Misurato su Linux con `umask` 0022: **644**. Non c'era **nessun** `.mode()` in tutto `crates/` |
| **la decisione del proprietario** | **`0600` sul file**, non `0700` sulla cartella |
| **perché non la cartella** | coprirebbe anche gli archivi futuri in un colpo solo, ma **la cartella non ha un proprietario nel codice**: nessuno la crea. La regola nominerebbe un chiamante che non esiste — è il difetto del finding **A-7**, e prenderselo per risparmiare una riga era lo scambio peggiore |
| **dove vive** | `FileBackend::open` in `crates/platform/src/journal.rs`, dietro `cfg(unix)`: è il **modulo di piattaforma**, che è dove I3 vuole il codice specifico dell'OS |
| **come è tenuto** | `the_journal_file_is_not_world_readable` in `crates/platform/tests/file_journal.rs`, `cfg(unix)`, che gira sulla **CI Linux**. Asserisce *«nessuno tranne il proprietario»* e non *«esattamente 0600»*, perché l'umask può solo **chiudere** di più e un'uguaglianza esatta andrebbe rossa dove la promessa è mantenuta |

⛔ **La cosa da ricordare non è il permesso: è che il difetto era INVISIBILE dove si lavora.**
Windows non ha il modo Unix, quindi né il codice né una sonda potevano dire niente sull'host di
sviluppo, e il rosso era **programmato per uscire il giorno del secondo sistema** — la stessa
forma del gotcha **#52**, e la ragione per cui l'audit dichiarava PL-1 *fuori copertura*. A
renderlo misurabile è stato notare che la **CI gira su `ubuntu-latest`**.

⚠️ **Limite dichiarato:** `mode()` vale solo alla **creazione**. Un giornale creato prima del
2026-08-18 resta 0644, e portarlo a 0600 è una **migrazione** — non è coperta da questa
decisione, e non esiste ancora un archivio da migrare.

⛔ **L'ADR non è superato:** la decisione — chiavi dell'OS, gestore dei segreti unico, onestà
sulla forza reale — è invariata. Ciò che si aggiunge è **il permesso che rende vera la frase
che l'ADR pretende di mostrare in interfaccia**.

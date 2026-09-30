# ADR-0040: Dove vivono i dati, e che cosa salva il programma

- **Status:** Accepted
- **Date:** 2026-09-30
- **Deciders:** proprietario del progetto
- **Modifica:** [ADR-0022](0022-layout-dei-dati-per-natura-e-backup-dichiarato.md), in tre punti — nella tabella del
  punto 1 della *Decision*, la riga «artefatti» e le **guide** della riga «configurazione, guide, profili»; e la
  conseguenza *«La base di conoscenza sopravvive alla reinstallazione perché i documenti sorgente e la configurazione
  sono nel backup»*. Le altre righe reggono, e lo stato di ADR-0022 resta `Accepted`

## Context

[ADR-0022](0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) separa i dati per natura e mette nel backup gli
**artefatti** e le **guide**, perché sono file dell'utente e non si rifanno. La revisione della knowledge base — il
[disegno](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md), approvato dal proprietario il
2026-09-29 — ha cambiato la premessa: i file del proprietario non stanno in una cartella del programma. Stanno nella
**root**, una cartella qualsiasi scelta da lui, anche quella in cui tiene tutto, e nelle **zone di lavoro**, le cartelle
che apre per lavorarci, anche fuori dalla root; e il programma non è il solo a scriverli, perché il proprietario li
cambia da fuori, con qualunque strumento. Le risposte che decidono — le D e i K sono le risposte e i buchi di quel
disegno:

| | La risposta del proprietario | Data |
|---|---|---|
| D4 | i router in `.<nomeapp>/`, alla root; l'indice nella cartella dati del programma, fra i dati che si rifanno; i dati del programma nella cartella dati per utente del sistema | 2026-09-29 |
| D12 | la root è del proprietario, e il suo backup pure, come in Obsidian; il programma salva il giornale, la configurazione e i router — mai i segreti, e non l'indice —, e quando crea il backup dice che cosa resta fuori | 2026-09-29 |
| D17 | nessun file del proprietario nel backup del programma, la root **e** le zone di lavoro; la storia lunga è dei backup del proprietario e di git, come in Claude Code | 2026-09-29 |

E una cosa che nessun ADR aveva preso: **dove** stanno i dati del programma. Il sorgente lo dice in
`crates/daemon/src/main.rs` — *«where a per-user data directory belongs is a decision no ADR has taken»* —, e il daemon
apre oggi `journal.redb` e `layout.redb` nella cartella da cui parte.

**Lo stato dell'arte, letto alla fonte il 2026-09-29** e portato in [`riferimenti.md`](../riferimenti.md): in Claude
Code le copie per annullare stanno nella cartella dell'applicazione e non nel progetto, in chiaro, protette dai permessi
del sistema operativo, cancellate dopo trenta giorni per default, e in nessun backup; `adr-tools` lega due ADR con
*«Amends»* e *«Amended by»* quando il nuovo ne cambia una parte, accanto al superamento che cambia lo stato del vecchio.

### Alternative considerate

- **Router e indice insieme in `.<nomeapp>/`, alla root** — la B di D4. *Contro:* l'indice si rifà, e starebbe nella
  cartella del proprietario, dentro i suoi backup.
- **Anche gli artefatti e le guide della root nel backup del programma**, con un inseguitore di artefatti — la B di D12.
  *Contro:* il programma copierebbe una cartella che il proprietario salva già coi suoi strumenti, inseguendo file che
  cambiano da fuori.
- **Gli artefatti delle zone di lavoro nel backup** — la B di D17. *Contro:* le zone sono repo e progetti, che hanno git.
- **Le copie del checkpoint nel backup, cifrate**, perché l'annulla sopravviva al ripristino. *Contro:* va contro D17, e
  porta i file grandi due volte — K21.

## Decision

| # | Decide | Da |
|---|---|---|
| 1 | **I dati del programma** stanno nella cartella dati per utente del sistema — `%LOCALAPPDATA%\<nomeapp>\` su Windows, le cartelle XDG su Linux —, in sottocartelle per natura, come vuole ADR-0022; la configurazione porta il percorso della root | D4 · K1 |
| 2 | **I router** stanno in `.<nomeapp>/`, alla root, marcata nascosta dal modulo di piattaforma — su Windows il punto nel nome non basta; **l'indice** nella cartella dati del programma, fra i dati che si rifanno — su Linux, la cache —, uno per root | D4 · K30 · K32 |
| 3 | **I file del proprietario** — la root e le zone di lavoro, coi file che le run vi producono e con le guide — stanno al loro posto, riferiti dal giornale, e **non** entrano nel backup del programma: li salvano i backup del proprietario e git. ⚠️ **L'eccezione è una:** `.<nomeapp>/`, coi router, che il programma salva — punto 4 | D12 · D17 |
| 4 | **Il backup del programma** contiene il giornale, cifrato, la configurazione e i router; mai i segreti, né l'indice, né i pesi; e **quando lo crea dice che cosa resta fuori** — la root, le zone, le copie —: è il seguito di ADR-0022 | D12 · ADR-0022 |
| 5 | **Le copie del checkpoint** di [ADR-0024](0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md) stanno nella cartella dati del programma, in una sottocartella loro; **in chiaro**, come i file che copiano; **fuori** dal backup; potate con la logica di [ADR-0018](0018-ritenzione-a-livelli-del-giornale.md), e mai quelle di un passo in dubbio non ancora riconciliato. Dopo un ripristino un passo di prima non si annulla più, e il programma lo dice: una copia assente non è una copia mai fatta — ADR-0018 | K53 · lo stato dell'arte |
| 6 | la cartella dati è un **percorso protetto e privato**: l'agente non ci scrive, non la legge, e non si indicizza. La regola vive nel rimando di [ADR-0016](0016-permessi-granulari-e-default-dei-vincoli-sui-dati.md), e qui si nomina | D19 |

## Consequences

- **Positive:**
  - Il programma non copia né insegue una cartella che non è sua: la root e le zone le salvano i backup del proprietario
    e git, e il backup del programma resta ciò che ADR-0022 voleva, l'irriproducibile **suo**.
  - I dati del programma hanno un posto deciso, quello che il sistema operativo prevede per un utente, invece della
    cartella da cui il daemon parte.
  - Chi ripristina sa **prima** che cosa il backup non porta — il seguito di ADR-0022, ora con la root, le zone e le
    copie.
- **Negative (accettate):**
  - **Il backup dei file del proprietario è suo:** il programma non lo fa, e lo dice quando crea il proprio.
  - **Dopo un ripristino** i passi di prima non si annullano; un passo in dubbio al momento del backup si riconcilia
    **senza** la sua copia, e allora si ferma e chiede — [ADR-0007](0007-giornale-write-ahead-e-riconciliazione.md).
    ⚠️ Dedotto.
  - **Una copia in chiaro** resta finché non è potata, anche se nel frattempo il proprietario cancella il file o lo rende
    privato; la protegge il sistema operativo, come in Claude Code.
  - **I router finiscono in due backup**, quello del programma e quello del proprietario, perché stanno nella root.
  - **Una root spostata** fa rifare l'indice con una scansione.
- **Follow-up richiesti:**
  - Il limite di dimensione delle copie resta di ADR-0024 — K21.
  - Il tempo di ritenzione delle copie è un parametro consegnato
    ([ADR-0034](0034-parametri-di-decisione-consegnati-non-letti.md)), col valore a chi le costruisce.
  - Chi costruisce la cartella dati per utente e la cartella nascosta dei router lo dice la sezione 4 del disegno della
    revisione, che è la casa unica di chi costruisce che cosa: il sotto-progetto 6, se nessuno le porta prima.

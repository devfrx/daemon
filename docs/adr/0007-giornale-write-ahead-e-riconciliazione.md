# ADR-0007: Giornale write-ahead delle run, e ripresa come riconciliazione

- **Status:** Proposed
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

Q5 promette che il riavvio del core a metà di una run lunga riprenda senza
rieseguire effetti già avvenuti. Serve un meccanismo, perché il problema difficile
non è salvare lo stato: è il **passo in dubbio**.

Dopo un crash, un passo che risulta iniziato ma non concluso è in uno stato ignoto:
l'effetto può essere avvenuto, non avvenuto, o avvenuto a metà. Nessuna quantità di
stato salvato risolve il dubbio — solo il *momento* in cui si scrive lo risolve.

Alternative considerate:

- **Checkpoint periodici dello stato in memoria.** Semplice.
  *Contro:* fra due checkpoint lo stato è perso, e soprattutto non dice nulla sugli
  effetti in volo nell'istante del crash. Non affronta il problema vero.
- **Giornale append-only scritto _dopo_ l'esecuzione.** Registra ciò che è realmente
  accaduto.
  *Contro:* un crash fra l'esecuzione e la scrittura produce un effetto **invisibile
  al giornale**. È il caso peggiore in assoluto, perché la ripresa lo rieseguirà
  credendolo mai avvenuto — cioè esattamente il danno che si voleva evitare.
- **Giornale write-ahead:** l'intento è reso durevole *prima* di eseguire, l'esito
  dopo. *Contro:* due scritture durevoli per passo.

## Decision

**1. Giornale append-only, write-ahead.** L'intento di ogni passo è scritto e reso
durevole prima dell'esecuzione; l'esito dopo. Un passo con intento e senza esito è
**in dubbio**, e il dubbio è sempre *rilevabile*. È questa la proprietà che si compra
con la seconda scrittura.

**2. La ripresa è riconciliazione, non replay cieco.** Rileggere il giornale non
significa rieseguire: significa stabilire, per ogni passo in dubbio, cosa è
realmente accaduto.

**3. Ogni effetto dichiara la propria classe**, e la classe determina la
riconciliazione:

| Classe | Definizione | Riconciliazione del dubbio |
|---|---|---|
| `verificabile` | si può interrogare il mondo per sapere se è avvenuto | verifica, poi completa oppure ripianifica |
| `idempotente` | ha una chiave; rieseguirlo non cambia il risultato | riesegui |
| `irripetibile` | rieseguirlo costa o danneggia | **sospendi e chiedi all'utente** |

**4. Un effetto senza classe dichiarata è trattato come `irripetibile`.** Il default
è il caso sicuro: davanti a un dubbio non risolvibile il sistema si ferma e chiede,
non indovina.

## Consequences

- **Positive:**
  - Q5 diventa difendibile con un meccanismo, non con una speranza.
  - **Un artefatto, tre usi:** il giornale è anche il trace ispezionabile di una run
    lunga *e* il materiale da cui ricavare casi di regressione dai fallimenti. Non
    servono tre sistemi.
  - Serve a ogni capacità con lavoro lungo — render 3D, indicizzazione, deep
    research — non solo all'agente. È ciò che lo rende legittimamente di kernel
    secondo [ADR-0001](0001-architettura-a-kernel-con-capacita-paritarie.md).
- **Negative (accettate):**
  - Due scritture durevoli per passo. Trascurabile per un passo che chiama un
    modello (centinaia di ms), **non** trascurabile per passi molto piccoli.
    Conseguenza diretta: la granularità del passo va scelta, non lasciata al caso.
  - Classificare gli effetti è lavoro di progettazione per ogni nuovo strumento.
  - Il default `irripetibile` produrrà interruzioni evitabili finché le classi non
    sono dichiarate bene. Attrito accettato in cambio della sicurezza.
- **Follow-up richiesti:**
  - **Granularità del passo:** un passo è *un'interazione con il mondo esterno* —
    una chiamata al modello, una chiamata a strumento. Non più fine di così.
  - La scelta del motore di persistenza è un ADR successivo, dopo quello sul
    linguaggio del core. Qui si decide la semantica, non la tecnologia.

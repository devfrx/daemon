# ADR-0030: Framework dell'interfaccia — Vue 3

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

[ADR-0027](0027-stack-della-gui.md) decide che la GUI è un'interfaccia web.
[ADR-0029](0029-guscio-della-gui.md) decide il guscio, ed è **ancora aperto**. Questa
decisione è **indipendente da entrambi**: il framework gira identico in Tauri e in
Electron, ed entrambi sono agnostici rispetto ad esso.

### L'asimmetria con ADR-0026, che va detta o sembrerà un'incoerenza

In [ADR-0026](0026-linguaggio-del-core.md) la competenza del proprietario **ha perso**:
Python e TypeScript sono stati esclusi dal core nonostante fossero i linguaggi noti,
perché V19 e V29 sono invarianti **non retrofittabili**, e la misura ha deciso al posto
della preferenza.

Qui non esiste un'invariante analoga:

| | Core | Framework dell'interfaccia |
|---|---|---|
| invarianti in gioco | V19, V29, I3, V28, ADR-0004 | **nessuna** |
| reversibilità | nessuna: cambiare linguaggio è riscrivere | alta: la GUI è **sacrificabile** (ADR-0004), il protocollo libero (I4) |
| chi decide | la misura | il merito, e **legittimamente** la competenza |

La competenza del proprietario è quindi un criterio **valido qui e non valido lì**, e
la differenza non è di comodo: è che lì c'era qualcosa di non recuperabile e qui no.

### Cosa i requisiti chiedono davvero

L'argomento consueto a favore di React — «più componenti pronti per interfacce di
chat» — pesa molto meno di quanto sembri su **questo** prodotto:

| Requisiti | Chi li fornisce |
|---|---|
| G4 chat markdown con streaming · G5 diff · G6 viewer 3D · G8 grafici | librerie, e le più pesanti sono **agnostiche rispetto al framework**: `three` 0.185.1 e `codemirror` 6.0.2 sono JavaScript puro |
| **G9–G18** — stato di degrado, permessi attivi, occupazione del contesto per categoria, costo e distanza dal tetto, provenienza del contenuto, `AttesaUmano`, `Rifiutata` contro `InCoda`, onestà sulla cifratura, esclusioni del backup, ambito del checkpoint | **nessuna libreria, in nessun framework**. È tutta superficie su misura |

Dieci requisiti su ventuno sono su misura per costruzione, perché nascono dalle
invarianti del kernel e non da un modello di interfaccia esistente. Il vantaggio
ecosistemico si applica alla minoranza del lavoro.

### Verifiche del 2026-08-06

| Pacchetto | Versione | Copre |
|---|---|---|
| `vue` | 3.5.41 | — |
| `vue-i18n` | 11.4.8 | **G21** interfaccia multilingua |
| `pinia` | 4.0.2 | stato di presentazione (I1: **solo** presentazione) |
| `@vueuse/core` | 14.4.0 | — |
| `three` | 0.185.1 | **G6**, agnostico rispetto al framework |
| `codemirror` | 6.0.2 | **G5**, agnostico rispetto al framework |
| `@tauri-apps/api` | 2.11.1 | se il guscio sarà Tauri |

Vue è fra i framework documentati da Tauri via Vite, e non pone alcun problema in
Electron.

## Decision

L'interfaccia si scrive in **Vue 3**, come **SPA**.

Il criterio che ha deciso è la **competenza del proprietario del progetto**, ed è
legittimo qui per la ragione detta sopra: nessuna invariante vincola questa scelta, e
la GUI è l'artefatto più sacrificabile del sistema. Nessun requisito da G1 a G21 è
irraggiungibile in Vue.

Per le componenti pesanti si preferiscono le librerie **agnostiche rispetto al
framework** (`three`, `codemirror`) alle rispettive incapsulazioni Vue: riducono la
dipendenza da questa decisione, e sopravvivrebbero a un cambio di framework.

## Consequences

- **Positive:**
  - La parte più grande e più visibile del prodotto si costruisce in territorio noto,
    mentre l'incognita vera — Rust nel core — resta confinata a un solo strato.
  - G21 ha una soluzione matura e verificata.
  - Le librerie che fanno il lavoro pesante non dipendono da questa scelta.

- **Negative (accettate):**
  - **L'ecosistema Vue per interfacce di agenti è più sottile di quello React.** Alcune
    librerie nascono React-first e le controparti Vue arrivano dopo, o non arrivano.
    Mitigazione dichiarata: usare la versione agnostica quando esiste, e accettare il
    costo di integrazione quando non esiste.
  - **La competenza in un solo framework è un punto singolo di dipendenza.** Se un
    giorno servisse altro, si riscrive la GUI — recuperabile, ma è la GUI intera.
  - **Il framework non regala l'accessibilità (G20).** Vue non la ostacola e non la
    fornisce: G20 va progettato, non ereditato. Dichiararlo evita falsa sicurezza.
  - Non è disponibile il rendering lato server, ma non serve: la GUI è locale.

- **Follow-up richiesti:**
  - Fissare nel sotto-progetto 2 le librerie per G5, G6 e G8, **preferendo le
    agnostiche**, e registrarle con la versione.
  - **G13** — provenienza fidata o non fidata visibile — nasce da
    [ADR-0014](0014-confine-dei-dati-non-fidati-nel-sistema-di-tipi.md) e va progettato
    dall'inizio: senza, la difesa contro la prompt injection collassa sull'anello umano.
  - Lo stato in `pinia` è **solo di presentazione** (I1). Se ci si trova a metterci
    qualcosa che il core non ha già, è un difetto, non una comodità.

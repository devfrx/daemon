# Roadmap — sotto-progetti, ordine, stato

Piano generale del progetto. **Da aggiornare a ogni sotto-progetto chiuso**, insieme a
[tracciabilità](tracciabilita.md).

Ultimo aggiornamento: **2026-08-07**.

## Stato in una riga

> Spec del kernel **completa e approvata** (§0–§10, 36 ADR). Stack deciso **tranne il
> guscio della GUI**: core in **Rust**, interfaccia web in **Vue 3**, worker ML in
> **Python**; Tauri contro Electron resta aperto ([ADR-0029](adr/0029-guscio-della-gui.md),
> `Proposed`) e **non blocca nulla**.
>
> **Sotto-progetto 1: §0–§8 approvate**, e la spec è **riaperta su sette voci**. Nessuna
> riga di codice. La §7 porta la **porta di qualità**: ogni controllo dichiara il proprio
> livello di forza — compilatore, controllo esterno, lint — e porta la sonda che deve
> scattare *e* la contro-sonda che deve restare verde. **Il livello 3 è vuoto**: nessuna
> invariante del kernel poggia su un lint. La §8 porta la **copertura**: ogni V e ogni Q
> con il proprio stato, e ogni rimandato con il proprio **innesco**, preteso dallo script
> e non dalla buona volontà. **Il livello ⛔ è vuoto**: nulla è lasciato deliberatamente
> senza controllo.
>
> ⚠️ **La spec è riaperta su sette voci** (2026-08-07), trovate rileggendo
> [tracciabilita.md](tracciabilita.md) con la domanda *«di quale meccanismo di kernel ha
> bisogno questa funzionalità, e la spec lo nomina?»*. Tre sono di classe **B**, cioè non
> retrofittabili. **Cinque sono chiuse**: i parametri di decisione consegnati al kernel
> ([ADR-0034](adr/0034-parametri-di-decisione-consegnati-non-letti.md), §2.8), la
> provenienza del totale di VRAM (§5.1), l'unico punto di uscita verso la rete (§2.3.1),
> **F1a** — la dichiarazione della porta verso i worker
> ([ADR-0035](adr/0035-porta-verso-i-worker-e-lettura-di-i4.md), §2.3.1), che completa la
> riga di verifica di I4 — e **F2 con F7**, l'evoluzione del formato durevole del giornale
> ([ADR-0036](adr/0036-evoluzione-del-formato-durevole-del-giornale.md), §4.9). Le
> restanti, con l'ordine e le propedeuticità, sono in
> [HANDOFF](HANDOFF.md#prima-cosa-da-fare).
>
> Prossimo passo: **F1b — il progetto della porta `process` in §5–§6**, poi F4, poi la §8
> una volta sola, e infine il piano di implementazione, che deve decidere anche dove nasce
> il workspace. Poi il codice.
>
> ✅ **La lacuna su I2 è chiusa**: [ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md)
> — quota di presentazione sottratta, con la concessione tenuta dal core. Il kernel non
> ha più lacune aperte.

## Il ciclo che seguiamo

Ogni sotto-progetto attraversa gli stessi passi. Non si salta.

```mermaid
flowchart LR
    B["brainstorming<br/>sezione per sezione"] --> S["spec approvata<br/>+ ADR + diagrammi"]
    S --> R["revisione<br/>umana"]
    R --> P["piano di<br/>implementazione"]
    P --> I["implementazione"]
    I --> V["verifica contro<br/>i requisiti Q"]
    V --> U["aggiornamento di<br/>roadmap e tracciabilita"]
```

## Sotto-progetti

| # | Sotto-progetto | Livello | Stato | Dipende da |
|---|---|---|---|---|
| **0** | **Kernel — arbitri e meccanismi** (§0–§9) | L0 + L1 | ✅ **spec completa** | — |
| **0b** | **Kernel L0 fisico** (§10) — archivi, cifratura, backup, segreti, checkpoint, confinamento | L0 | ✅ **spec completa** | 0 |
| **0c** | **Stack completo** — ADR-0026 core, ADR-0027 GUI, ADR-0028 worker ML | — | ✅ **deciso** | SP-5, SP-6 |
| 1 | Implementazione del kernel + simulatore DST | L0 + L1 | 🔵 **in corso** — §0–§8 approvate, **riaperta su sette voci**: cinque chiuse, restano **F1b** e **F4**, poi il piano | 0, 0b, 0c |
| 2 | GUI minima (shell, chat, stato) | — | ⬜ | 1, ADR-0027 |
| 3 | Conversazione | L2 | ⬜ | 1, 2 |
| 4 | Agenti | L2 | ⬜ | 3 |
| 5 | Coding | L2 | ⬜ | 4, 0b |
| 6 | Conoscenza / RAG | L2 | ⬜ | 3 |
| 7 | Generazione asset | L2 | ⬜ | 1 · chiude **SP-1** |
| 8 | Voce | L2 | ⬜ | 7 · chiude **SP-2** |
| 9 | Gestione modelli locali | L1 est. | ⬜ | 1 |
| 10 | Integrazione OS completa | L3 | ⬜ | 2 |
| 11 | **Backup e ripristino** — [ADR-0022](adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md); chiude **V32 · V33 · Q21** | L0 + L3 | ⬜ | 5, 6, 9 |

## Perché quest'ordine

| Scelta | Motivo |
|---|---|
| **SP-5/SP-6 prima di tutto** | possono **escludere un linguaggio**: farli dopo significa riscrivere |
| Kernel prima di ogni capacità | imposto da ADR-0001: parità, nessun accesso privilegiato |
| Conversazione come prima capacità | è il consumatore più sottile di *tutti* i meccanismi del kernel: lo valida da capo a fondo con il minimo codice |
| Agenti prima di Coding | Coding usa gli anelli e i sensori che Agenti introduce per primo |
| Generazione asset prima di Voce | chiude **SP-1** (il rischio più grande) prima; e **SP-2** richiede che esistano *entrambi* — voce e job GPU pesante |
| L3 completo per ultimo | packaging, i18n e accessibilità non sbloccano nulla a monte |
| **Backup dopo indici e pesi** | non è comodità, è non-vacuità. V32 dice che il backup **esclude indici e pesi perché ricostruibili**: prima che 6 e 9 li producano, l'elenco delle esclusioni è vuoto, e verificare V32 su un elenco vuoto è una prova che non può fallire — gotcha #17. Serve inoltre il filesystem reale, che arriva con 5, e l'interfaccia che dichiara le esclusioni **al momento del backup** (follow-up di ADR-0022) |

Non è un ordine per importanza: i quattro pilastri restano paritari (ADR-0001). È un
ordine per **dipendenza e per rischio**.

## Il primo valore utile

Sotto-progetti 1 + 2 + 3: una chat funzionante con contabilità dei costi, giornale,
ripresa dopo crash e stato di degrado dichiarato. È il momento in cui il progetto
smette di essere solo documenti.

Costo accettato in ADR-0001: arriva più tardi che in un'architettura con baricentro.

## Spike aperti

| ID | Domanda | Blocca | Stato |
|---|---|---|---|
| **SP-5** | iniettabilità di tempo, casualità, I/O, scheduling | ⛔ ADR linguaggio | ✅ **chiuso**: solo Rust passa. Go fallisce C6 (9 e 4 tracce distinte su 100 dentro `synctest`), TypeScript parziale |
| **SP-6** | il sistema di tipi regge il confine dei dati non fidati | ⛔ ADR linguaggio | ✅ **chiuso**: Rust e Go passano, TypeScript parziale su T4 e T6 |
| SP-1 | curva qualità/VRAM di TRELLIS2 su 16 GB | profili di risorsa §2 | ⬜ |
| SP-2 | Q1 (voce < 600 ms) sotto carico GPU | taratura corsie §2 | ⬜ |
| SP-3 | budget della proiezione per modello | taratura §5 | ⬜ |
| SP-4 | provider con annullamento senza addebito | ordine di preferenza §3 | ⬜ |

Protocolli e soglie decisionali: [spec §9](superpowers/specs/2026-08-06-kernel-design.md).

## Piani di implementazione

| Piano | Copre | Stato |
|---|---|---|
| [Spike bloccanti e stack](superpowers/plans/2026-08-06-spike-linguaggio-del-core.md) | SP-5, SP-6, ADR-0026, ADR-0027, ADR-0028 | ✅ **eseguito** il 2026-08-06 |

**Il piano è l'artefatto dopo la chiusura delle voci ancora aperte** della
riapertura: vale «spec prima del codice», e il piano è il passo fra le due.

**Il codice si scrive in questo repository**, non altrove. Il piano deve quindi decidere
anche *dove*: `spikes/rust/` ha un proprio `Cargo.toml` e alla radice non ce n'è nessuno,
quindi il workspace delle cinque crate nasce alla radice — escludendo gli spike — oppure
accanto ad essi.

Il prototipo [`spikes/rust/`](../spikes/rust/) è il punto di partenza del simulatore:
contiene già il confine dei tipi, l'esecutore deterministico, l'esecutore su `Future`
native e il giornale write-ahead iniettabile, tutti con i loro test.

**Toolchain sulla macchina**, verificata il 2026-08-06: `rustc` 1.95.0 · `cargo` 1.95.0
· `clippy` 0.1.95. Go 1.26.5 e Node 24.9 restano installati ma non servono più al core.

## Decisioni ancora da prendere

| Decisione | Quando | Vincolata da |
|---|---|---|
| ~~Linguaggio del core~~ | ✅ **ADR-0026: Rust** | — |
| ~~Interfaccia web o toolkit nativo~~ | ✅ **ADR-0027: interfaccia web** (G7) | — |
| ~~Ecosistema dei worker ML~~ | ✅ **ADR-0028: Python** | — |
| ~~Framework dell'interfaccia~~ | ✅ **ADR-0030: Vue 3** | — |
| ⚠️ **Guscio della GUI: Tauri o Electron** | **ADR-0029, `Proposed`.** Si chiude con **cinque** misure M1–M5 all'inizio del sotto-progetto 2 | non blocca il sotto-progetto 1 |
| ~~La GPU usata dalla GUI non è arbitrata~~ | ✅ **[ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md)**, nella §5 della spec del sotto-progetto 1 | I2 è ora verificato su **tutte e tre** le classi di processo |
| ~~Motore di persistenza~~ | ✅ **[ADR-0032](adr/0032-motore-di-persistenza.md): `redb` 4.1.0** con `StorageBackend` scritto da noi | il requisito 4 di §10.6 è stato misurato: solo `redb` lo espone |
| ~~Serializzatore dello schema IPC~~ | ✅ **`bincode` 2.0.1**, misura M-1 nella §6 — prime voci della lista di [ADR-0031](adr/0031-dipendenze-del-kernel-parte-del-confine.md), che smette di essere vuota | il criterio non era «`no_std`» ma **il grafo transitivo** |
| ~~Evoluzione del formato durevole del giornale~~ | ✅ **[ADR-0036](adr/0036-evoluzione-del-formato-durevole-del-giornale.md)**: versione **più** indici espliciti, codifica in `kernel`. Misura M-9, §4.9 | la disciplina solo-append è **eliminata dalla misura**: su un formato posizionale non funziona affatto |
| Livello 3 di confinamento (microVM) | quando servirà eseguire codice di provenienza ignota | ADR-0025 |
| ~~Dove vive backup e ripristino~~ | ✅ **chiusa il 2026-08-07: sotto-progetto 11**, dopo 5, 6 e 9. La lacuna l'aveva trovata la §8, che non riusciva a dare un numero all'innesco di V32, V33 e Q21 | [ADR-0022](adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) · §8.5.2 della spec del sotto-progetto 1 |

### La lacuna su I2 — ✅ chiusa il 2026-08-07

Trovata durante la revisione di ADR-0027, non cercata. Chiusa da
[ADR-0033](adr/0033-gpu-della-gui-quota-di-presentazione.md) nella §5 della spec del
sotto-progetto 1.

Le tre uscite enumerate **non erano tre opzioni per un problema**: erano tre risposte
parziali per **tre consumatori diversi**, trattati fino ad allora come uno solo.

| # | Consumo GPU della GUI | Governo | Rifiuto esecutivo? |
|---|---|---|---|
| 1 | compositing della webview | quota di presentazione sottratta, **concessione tenuta dal core** | ❌ no |
| 2 | viewer 3D entro la quota | stessa quota | ❌ no |
| 3 | viewer 3D oltre la quota | concessione ordinaria via IPC, prelazionabile | ✅ sì |

Il titolare è il **core** e non la GUI, perché la GUI non può *chiedere*: chi alloca è il
compositor, che non ha un percorso di richiesta. E una quota sottratta senza titolare
lascerebbe I2 falso.

⚠️ **Costo dichiarato:** verso il compositor il rifiuto dell'arbitro **non è esecutivo**.
La quota è una promessa di budget, non un'imposizione, e per la GUI I2 vale in una forma
più debole *in natura*. Il valore della quota è **non misurato**: lo chiude **M5**, e
stringe **RK-1** di una quantità oggi ignota.

## Regola di manutenzione

Alla chiusura di ogni sotto-progetto si aggiornano, **nello stesso passaggio**:

1. la tabella dei sotto-progetti qui sopra;
2. le righe corrispondenti in [tracciabilita.md](tracciabilita.md);
3. lo stato degli spike che quel sotto-progetto ha chiuso;
4. `CLAUDE.md` alla radice, se cambia il «prossimo passo».

Un documento di stato disallineato è peggio di nessun documento: mente con autorevolezza.

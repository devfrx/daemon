# ADR-0027: Stack della GUI

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

La GUI non ha incognite bloccanti: [ADR-0004](0004-topologia-di-processo.md) la rende
**sacrificabile**, quindi la scelta è reversibile. Ma non è indipendente dal core:
ergonomia dell'IPC, tipi condivisi e packaging cambiano a seconda della **coppia**.

I ventuno requisiti G1–G21, ciascuno con la propria fonte nella spec, sono in
[`spikes/GUI-REQUISITI.md`](../../spikes/GUI-REQUISITI.md). Un requisito senza fonte non
è stato ammesso.

### Matrice delle coppie

Il core è **Rust** ([ADR-0026](0026-linguaggio-del-core.md)). Le righe di Go e
TypeScript sono state cancellate dal documento dei requisiti: righe non pertinenti
lasciate lì sono rumore che il prossimo lettore dovrà filtrare a mano.

| Opzione GUI | Packaging | Tipi condivisi col core | Costo del viewer 3D (G6) | Opzioni verificate |
|---|---|---|---|---|
| shell nativa con **interfaccia web** | binario + **webview di sistema** | no, schema duplicato | libreria web matura | `tauri` 2.11.5 · `wry` 0.56.0 |
| **toolkit nativo** del linguaggio | binario singolo | sì | da scrivere | `egui` 0.36.0 · `iced` 0.14.0 · `slint` 1.17.1 |

`dioxus` 0.8.0-**alpha**.1 è escluso: non si fonda un sotto-progetto su una versione
alpha.

### Il prototipo — l'unica incognita reale

I requisiti da G4 a G18 sono lavoro noto. L'unica incognita è se l'IPC regga
**aggiornamenti ad alta frequenza** senza ingolfare l'interfaccia: token in streaming,
stato di degrado e occupazione del contesto, tutti insieme. Soglie P1–P4 fissate
**prima** di guardare i numeri.

Scenario: **2000 messaggi in 10 secondi**, tre canali logici su **un solo trasporto**
(I4), con `interprocess` 2.4.3 — named pipe su Windows, socket unix su Linux, stesso
codice.

| # | Soglia | Misurato | Esito |
|---|---|---|---|
| P1 | zero messaggi persi | **0 persi, 0 buchi nel progressivo**, 2000/2000 | ✅ |
| P2 | ritardo emissione → rendering < 100 ms | **10,21 ms** di picco · 0,363 ms medio | ✅, margine ~10× |
| P3 | CPU della gui < 25 % di un core | **21,43 %** di picco · 8,05 % medio | ✅ **ma stretto** |
| P4 | uccidere la gui a metà e riaprirla: il core non se ne accorge | gui abortita a 600; core prosegue, riaccetta a 606, **emette tutti i 2000 in 10 000 ms** | ✅ |

P1 non è fallito, quindi **non serve un ADR sulla contropressione dell'IPC**. Se fosse
fallito sarebbe stata una decisione di design del *kernel*, non un dettaglio della GUI.

**Cosa il prototipo non ha misurato**, e va detto perché cambia la lettura dei numeri:

| Non misurato | Effetto |
|---|---|
| il salto **webview ↔ shell** dentro il processo gui | una GUI a interfaccia web ha un salto in più, non incluso nei 10,21 ms |
| il costo di rendering reale (DOM, layout, 3D) | qui il «rendering» è una lista in memoria: P3 sarà più alto |
| il comportamento su Linux | `interprocess` promette lo stesso codice; non verificato |

Il margine su P2 assorbe il salto mancante. **Il margine su P3 no.**

## Decision

La GUI si scrive come **shell nativa Rust con interfaccia web**, su **Tauri 2.11.5**.

Il requisito che ha deciso il confronto è **G7 — artifacts o canvas con anteprima
viva**. È l'unico che non ammette alternativa: rendere contenuto arbitrario prodotto da
un modello, con anteprima viva, *richiede* un motore web. Un toolkit nativo dovrebbe
incorporarne uno comunque, cioè pagare entrambi gli stack invece di uno.

Due requisiti lo rinforzano, e nessuno dei tre dipende dalle nostre preferenze:

| # | Requisito | Perché spinge nella stessa direzione |
|---|---|---|
| **G6** | viewer 3D interattivo | librerie web mature contro codice da scrivere su `wgpu` |
| **G20** | screen reader, navigazione da tastiera, contrasto | l'accessibilità del web è di gran lunga la più matura; quella dei toolkit nativi Rust è in costruzione |
| G5 | diff approvabili | editor e diff viewer web sono componenti maturi, non lavoro nostro |

**La scelta resta a basso rischio per costruzione**, e questo è il motivo per cui non ha
richiesto uno spike bloccante come il core:

> Se la scelta si rivelasse sbagliata, la GUI si riscrive **senza toccare il kernel**:
> [ADR-0004](0004-topologia-di-processo.md) la rende sacrificabile e I4 lascia il
> protocollo libero di cambiare.

## Consequences

- **Positive:**
  - G4–G8 poggiano su componenti maturi invece che su codice nostro: è il grosso della
    superficie visibile del prodotto.
  - G20 è raggiungibile davvero, non a parole.
  - P1–P4 sono soddisfatti con margine su tre criteri su quattro, misurati e non stimati.
  - La GUI resta un processo separato che parla **solo** col core (G2, I4): l'uso di uno
    stack diverso non incrina nessuna invariante del kernel.

- **Negative (accettate):**
  - **Nessun tipo condiviso col core.** Lo schema dell'IPC esiste in Rust e va
    rispecchiato nell'interfaccia: due definizioni da tenere allineate. Mitigabile con
    generazione automatica, che è però un passo di build in più da mantenere.
  - **Due motori di rendering diversi**, non uno portabile: su Windows **WebView2**
    (Chromium), su Linux **WebKitGTK** — verificato sulla documentazione di Tauri, che
    dichiara essa stessa quanto sia difficile avere informazioni accurate su WebKitGTK
    nelle varie distribuzioni. G19 è soddisfatto, ma «Linux senza riscrivere» significa
    *senza riscrivere*, **non** *senza ritestare*.
  - **Un terzo ecosistema entra nel progetto**: dopo Rust (core) e Python
    ([ADR-0028](0028-ecosistema-dei-worker-ml.md), worker ML), arriva la toolchain web.
    Tre gestori di dipendenze, tre superfici di supply chain, tre cicli di aggiornamento.
  - **Il margine su P3 è stretto**: 21,43 % contro una soglia del 25 %, e la misura non
    includeva il rendering reale né il salto verso la webview. È il numero da rimisurare
    per primo.
  - Il packaging non è un binario singolo, ma questo costo era **già stato pagato** da
    ADR-0028: non è questa decisione a introdurlo.

- **Follow-up richiesti:**
  - **P3 va rimisurato nel sotto-progetto 2** con rendering reale e con il salto
    webview ↔ shell incluso. Se supera il 25 %, la leva non è la GUI ma la **frequenza
    di aggiornamento** decisa dal core: aggregare o campionare è una scelta di kernel.
  - Verificare su Linux che `interprocess` regga lo stesso codice. È il rischio RK-11
    (confine verificato ma non validato) applicato all'IPC.
  - La webview esegue **solo contenuto locale nostro**. Caricarvi contenuto remoto
    aprirebbe una superficie che [ADR-0003](0003-estensibilita-solo-mcp-e-skill-dichiarative.md)
    ha chiuso deliberatamente: va vietato esplicitamente, non lasciato al buon senso.
  - G13 — provenienza fidata o non fidata visibile — è un requisito della GUI che nasce
    da [ADR-0014](0014-confine-dei-dati-non-fidati-nel-sistema-di-tipi.md): senza, la
    difesa collassa sull'anello umano. Va progettato nel sotto-progetto 2, non aggiunto
    dopo.

# Strategia di test e criteri di accettazione

Come si verifica ogni requisito di qualità, e cosa il kernel deliberatamente non testa.
Fonte di verità sulla porta di qualità.

Decisioni: [ADR-0020](../adr/0020-nessun-modello-nel-percorso-decisionale-del-kernel.md) ·
[ADR-0021](../adr/0021-simulazione-deterministica-e-iniettabilita.md).

## I due strati, e dove passa il confine

```mermaid
flowchart TB
    subgraph K["KERNEL — strato deterministico"]
        K1["arbitro GPU · gateway · giornale<br/>permessi · tipi · sensori · anelli"]
        K2["test SENZA modello<br/>veloci, riproducibili, a ogni commit<br/>un fallimento e SEMPRE un difetto"]
        K1 --> K2
    end

    subgraph C["CAPACITA L2 — strato probabilistico"]
        C1["conversazione · conoscenza · agenti<br/>coding · voce · asset"]
        C2["valutazione con giudice<br/>dataset curati, trace-based eval<br/>un fallimento puo essere variabilita"]
        C1 --> C2
    end

    K -.->|"il kernel non sale mai qui"| C

    classDef det fill:#0f766e,stroke:#134e4a,color:#fff
    classDef prob fill:#b45309,stroke:#78350f,color:#fff
    class K1,K2 det
    class C1,C2 prob
```

Il confine è netto e verificabile: **nessun modello nel percorso decisionale del
kernel**. Un fallimento del kernel non è mai variabilità — è un difetto.

## Le quattro tecniche

| Tecnica | Verifica | Determinismo |
|---|---|---|
| **analisi statica** | I3 (nessuna chiamata OS nel kernel), I6/V19 (confine dei tipi), V5 (effetti classificati), V25 (un solo punto di uscita), ADR-0020 | totale, a compilazione |
| **test a esempi** | comportamenti puntuali, macchine a stati, tabelle di decisione | totale |
| **simulazione deterministica (DST)** | concorrenza, crash, ripristino: I1, I2, I5, Q2, Q4, Q5 | riproducibile **per seed** |
| **test di contratto** | worker, server MCP, provider: dati stantii, risposte malformate, timeout | totale, con doppi |

## Mappa requisito → metodo di verifica

Nessun requisito è accettato senza un metodo dichiarato. La colonna «tecnica» dice
anche *quanto costa* verificarlo, che è ciò che determina se verrà davvero fatto.

| Q | Requisito | Metodo | Tecnica |
|---|---|---|---|
| Q1 | voce < 600 ms sotto carico GPU | misura end-to-end con job `batch` attivo, percentile su N campioni | misura → SP-2, poi non-regressione |
| Q2 | zero OOM | proprietà: la somma delle concessioni non supera mai il budget, sotto richieste concorrenti casuali | **DST** |
| Q3 | crash GUI durante una run | kill del processo `gui` in punti arbitrari; proprietà: la run prosegue | **DST** |
| Q4 | kill di un worker in qualsiasi istante | kill in punti arbitrari; proprietà: nessuna corruzione, nessuna perdita | **DST** |
| Q5 | riavvio del core a metà run | crash iniettato a **ogni confine di persistenza**; proprietà: nessun effetto rieseguito | **DST + crash-injection** |
| Q6 | contesto esaurito | ricomposizioni ripetute con budget ridotto; proprietà: gli elementi non sacrificabili sono sempre presenti | proprietà |
| Q7 | tetto di passi/tempo/costo superato | test a esempi sulla transizione ad `AttesaUmano` | esempi |
| Q8 | avvio a freddo dichiarato | test a esempi sull'evento emesso prima dell'attesa | esempi |
| Q9 | contenuto non fidato nel canale istruzioni | **non compila**: test negativo di compilazione | **statica** |
| Q10 | verdetto di sensore → anello | sensore finto con verdetto negativo; il passo successivo porta il feedback | esempi |
| Q11 | budget della proiezione | proprietà: occupazione ≤ budget dopo **ogni** ricomposizione | proprietà |
| Q12 | difetto ricorrente → proposta | giornale sintetico con ricorrenza; verifica che la proposta sia emessa | esempi |
| Q13 | vincolo sui dati senza endpoint conforme | proprietà: **nessun candidato non conforme viene mai eseguito**, per qualunque catena | proprietà |
| Q14 | ricostruire un passo di sei mesi fa | giornale storico + configurazione cambiata; il record risolto resta leggibile | esempi |
| Q15 | istruzione nei dati non autorizza | statica + test a esempi sull'obbligo di autorizzazione | **statica** + esempi |
| Q16 | descrizione MCP cambiata dopo l'approvazione | server finto che muta la descrizione; lo strumento passa a `Sospeso` | **contratto** |
| Q17 | segreto in uscita | valore canary iniettato nel contenuto in uscita; blocco e segnalazione | esempi |
| Q18 | perdita di rete | iniezione del guasto; proprietà: lo stato di degrado è dichiarato **prima** del primo fallimento | **DST** |
| Q19 | capire una run di 4 ore | giornale sintetico lungo; la proiezione trace è navigabile e completa | esempi |
| Q20 | nessun dato lascia la macchina | statica (un solo punto di uscita) + test che verifica assenza di traffico a default | **statica** + esempi |

## Cosa il kernel deliberatamente NON testa

| Fuori perimetro | Dove appartiene |
|---|---|
| qualità delle risposte del modello | capacità L2 |
| valutazione con giudice, dataset curati, trace-based eval | capacità L2 |
| correttezza semantica di un piano agentico | capacità Agenti |
| qualità percepita di voce e mesh 3D | capacità Voce, capacità Asset |
| ergonomia dell'interfaccia | GUI |

Dichiararlo evita l'errore opposto a quello comune: non solo «non applicare test
deterministici a ciò che è probabilistico», ma anche **«non rinunciare al determinismo
dove esiste»**.

## La porta di qualità

| Regola | Motivo |
|---|---|
| Nessuna sezione della spec è «fatta» senza i test dei suoi requisiti | un requisito senza verifica è un'intenzione |
| Ogni difetto trovato in simulazione **conserva il proprio seed** | il seed diventa un caso di regressione permanente |
| I fallimenti promossi dall'anello 4 (§5) entrano nella stessa suite | un artefatto, non due |
| Analisi statica e test a esempi girano a **ogni commit**; DST su cicli più lunghi | «tieni la qualità a sinistra» (§5) |

## Regole che le tabelle non esprimono

- **L'iniettabilità è un requisito di costruzione, non di test.** Nessun componente
  legge l'orologio, genera casualità o esegue I/O se non attraverso un confine
  sostituibile. Senza, la DST non è possibile — e non è retrofittabile.
- La scelta del linguaggio del core dovrà valutare esplicitamente la **sostituibilità
  dello scheduling**: è il primo caso in cui una decisione di test vincola una
  decisione di architettura.
- Un fallimento del kernel è **sempre** un difetto. Se un test del kernel è
  intermittente, il difetto è nel test o nell'iniettabilità — mai «è il modello».

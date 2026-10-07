# Strategia di test e criteri di accettazione

Come si verifica ogni requisito di qualità, e cosa il kernel deliberatamente non testa.
Fonte di verità sulla porta di qualità.

Decisioni: [ADR-0020](../adr/0020-nessun-modello-nel-percorso-decisionale-del-kernel.md) ·
[ADR-0021](../adr/0021-simulazione-deterministica-e-iniettabilita.md).

⚠️ **RICHIAMO DEL 2026-09-08 — la sezione 2 della passata sui diagrammi della stella polare della GUI.**
Il diagramma dei due strati guadagna ciò che il kernel ha oggi e non nominava — l'esecutore, il degrado —
ciò che arriva, «(col N)», e la **GUI** come seconda scatola dello strato deterministico: prove sulle
fixture generate dal kernel, senza modello, con un passo suo nel cancello. Le suite di conformità
sono nominate con la regola che le fa crescere; le campagne DST della tabella delle
tecniche e della mappa sono le stesse; la riga Q3 dice ciò che la campagna di oggi prova e ciò che
arriva; lo **stato** di ogni Q resta nella §8.4 della spec, casa unica, e qui non si ricopia. Due frasi
corrette: il «ciclo lungo» della DST profonda, che non esiste, e la valutazione del linguaggio, che
ADR-0026 ha fatto. Una voce segnata «(col N)» è decisa e la costruisce il sotto-progetto N; «oggi» dice
che esiste nel codice. Il perché sta nella
[stella polare](../superpowers/specs/2026-09-07-direzione-gui-design.md), sezione 2 della passata,
decisione 18.

⚠️ **RICHIAMO DEL 2026-10-06** — audit del 2026-09-30, AUD-216, AUD-248, AUD-249, AUD-254, AUD-262, AUD-263,
AUD-267, AUD-268, AUD-269, AUD-270, AUD-288 e AUD-609: il registro delle funzioni, la settima porta, la GUI col suo
passo nel cancello e la prova di Q3 sull'attività che ascolta la GUI esistono, dal sotto-progetto 2; ogni altro
pezzo deciso e non costruito porta il segno «(col N)» della regola 2 del [README](../README.md).

## I due strati, e dove passa il confine

```mermaid
flowchart TB
    subgraph D["STRATO DETERMINISTICO — senza modello, a ogni commit: un fallimento e SEMPRE un difetto"]
        subgraph K["KERNEL"]
            K1["esecutore · arbitro GPU · gateway · giornale<br/>permessi · tipi · sensori e anello · degrado<br/>registro delle funzioni e settima porta<br/>registro delle guide, trigger, proiezione (col 13)"]
            K2["statica · esempi · DST per seme · contratto<br/>veloci, riproducibili"]
            K1 --> K2
        end
        subgraph G["GUI — presentazione"]
            G1["le fixture: i byte del kernel<br/>e il valore atteso, generati insieme"]
            G2["prove sulle fixture e accessibilita G20<br/>un passo suo nel cancello<br/>nessun modello: il core finto manda a tempo"]
            G1 --> G2
        end
    end

    subgraph C["CAPACITA L2 — strato probabilistico"]
        C1["conversazione (col 3) · conoscenza (col 6)<br/>agenti (col 4) · coding (col 5)<br/>voce (col 8) e gesti (col 12) · asset (col 7)"]
        C2["valutazione con giudice (col 3 e seguenti)<br/>dataset curati, trace-based eval<br/>un fallimento puo essere variabilita"]
        C1 --> C2
    end

    K -.->|"il kernel non sale mai qui"| C
    G -.->|"ne rende l'uscita come testo non fidato: non la giudica (G13)"| C

    classDef det fill:#0f766e,stroke:#134e4a,color:#fff
    classDef prob fill:#b45309,stroke:#78350f,color:#fff
    class K1,K2,G1,G2 det
    class C1,C2 prob
```

Il confine è netto e verificabile: **nessun modello nel percorso decisionale del
kernel**. Un fallimento del kernel non è mai variabilità — è un difetto. La **GUI** sta
nello strato deterministico anche se mostra l'uscita di un modello: la rende come testo
non fidato (G13, ADR-0014) e non la giudica mai; le sue prove girano sulle **fixture**
generate dal kernel — i byte e il valore atteso, insieme — senza modello, e un suo
fallimento è un difetto come quelli del kernel (§4 e §6a del disegno del 2).

## Le quattro tecniche

| Tecnica | Verifica | Determinismo |
|---|---|---|
| **analisi statica** | I3 (nessuna chiamata OS nel kernel), I6/V19 (confine dei tipi), V5 (effetti classificati); ADR-0020 (V28) per le **dipendenze**, con l'allow-list di ADR-0031, mentre nessun controllo vede le chiamate attraverso una porta iniettata (col 3); V25 (un solo punto di uscita, col 3), V34 (lettura dei segreti, col 3), V35 (livello di confinamento, col 5) | totale: al compilatore (livello 1) e nel cancello (livello 2) — §7.4 della spec del sotto-progetto 1 |
| **test a esempi** | comportamenti puntuali, macchine a stati, tabelle di decisione | totale |
| **simulazione deterministica (DST)** | concorrenza, crash, ripristino: I1, I2, I5; Q2, Q3, Q4 — i guasti del dialogo col 12 —, Q5, Q18 (col 3), Q22 (col 5): le righe che la mappa qui sotto marca **DST**. Ogni campagna è un bersaglio che il passo «DST campaigns» di `gate.sh` nomina **uno per uno**: una campagna nuova è anche una riga in quel passo, o il cancello la esegue senza mostrarne il tempo di parete | riproducibile **per seed** |
| **test di contratto** | worker (col 12), server MCP (col 4), provider (col 3): dati stantii, risposte malformate, timeout. ⭐ E la **conformità fra l'implementazione reale di una porta e la sua finta** — è ciò che impedisce di provare Q4 e Q5 contro una finzione. Le suite sono scritte in `crates/kernel/tests/` e rieseguite sulle implementazioni vere di `platform` con `include!`: quali porte le hanno lo dice `ls crates/platform/tests/*_contract_real.rs`, e quale Q ne eredita lo stato la §8.2.2 della spec del sotto-progetto 1. Quella di `ipc` gira sul solo trasporto vero, senza finte né bugiardi (D82 del piano della parte 2 del sotto-progetto 2): se una finta vada tenuta contro la vera è la scelta aperta su AUD-409. **Ogni porta guadagna la propria quando arriva l'implementazione vera**: `network` col 3, `filesystem` a pezzi dal 13 — chi costruisce che cosa lo dice la 4.2 del [disegno della revisione della knowledge base](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md) —, `process` col 12, il primo worker vero (ADR-0039) | totale, con doppi |

⚠️ **RICHIAMO DEL 2026-10-06** — audit del 2026-09-30, AUD-260, AUD-265, AUD-266, AUD-272, AUD-607, AUD-608, AUD-710
e AUD-715: la tabella dice ciò che il codice tiene oggi, e ciò che è deciso e non costruito porta il segno «(col N)»,
che per le righe V e Q è l'innesco della §8 della spec; le suite di conformità le conta il comando, non una cifra.

## Mappa requisito → metodo di verifica

Nessun requisito è accettato senza un metodo dichiarato. La colonna «tecnica» dice
anche *quanto costa* verificarlo, che è ciò che determina se verrà davvero fatto.

| Q | Requisito | Metodo | Tecnica |
|---|---|---|---|
| Q1 | voce < 600 ms sotto carico GPU | misura end-to-end con job `batch` attivo, percentile su N campioni | misura → SP-2, poi non-regressione |
| Q2 | zero OOM | proprietà: la somma delle concessioni non supera mai il budget, sotto richieste concorrenti casuali | **DST** |
| Q3 | crash GUI durante una run | la GUI muore a un'operazione scelta dal seme, in una finta della porta `ipc`; proprietà: il core non perde nulla — oggi le concessioni tornano (§5.7, proprietà 3), anche nell'attività che ascolta la GUI, `kernel::serving::serve`; col 3 la run prosegue | **DST** |
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
| Q21 | ripristino da backup su macchina nuova | backup e ripristino su ambiente pulito; proprietà: nessun dato irriproducibile **del programma** perso, e il messaggio pre-backup elenca le esclusioni, fra cui la root e le zone di lavoro del proprietario, che non sono nel backup del programma (ADR-0040, punti 3 e 4). ⚠️ **Richiamo del 2026-10-06** — audit del 2026-09-30, AUD-367: si legge con [ADR-0040](../adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md), come la riga Q21 della [spec del kernel](../superpowers/specs/2026-08-06-kernel-design.md) | esempi + **contratto** |
| Q22 | annullare un passo che ha modificato file | dopo il rollback al passo N l'ambito è **byte-identico** allo stato precedente; crash iniettato durante la conservazione | **DST + crash-injection** |
| Q23 | esecuzione sotto il livello 2 di confinamento | statica: nessun percorso di esecuzione senza livello richiesto. Più test negativo: con confinamento indisponibile l'azione **non parte** | **statica** + esempi |
| Q24 | lettura di credenziali fuori dal gestore dei segreti | statica sui grafi di importazione e chiamata: nessun altro componente ha un percorso verso l'archivio dei segreti | **statica** |

**Q23 e Q24 si provano anzitutto per costruzione**, con la statica: sono proprietà
strutturali, e verificarle a campione le renderebbe congetture; il test negativo di Q23 si
aggiunge alla statica, non la sostituisce. ⚠️ **Richiamo del 2026-10-06** — audit del
2026-09-30, AUD-271: la frase dice ciò che dicono le righe Q23 e Q24.

⚠️ **Lo stato di ogni riga** — verificata, parziale, rimandata — **e chi la chiude** stanno
nella **§8.4 della spec del sotto-progetto 1**, in una casa sola, col comando che li conta
nella §6 del compendio: qui vive il **metodo**, e nessuna riga si marca. I requisiti della
**GUI**, G1–G21 e P1–P4, hanno la loro casa in
[`spikes/GUI-REQUISITI.md`](../../spikes/GUI-REQUISITI.md): il loro metodo — le fixture,
l'accessibilità, e le misure M1–M5 dello spike del guscio, che sono misure come SP-2 e non
prove — lo scrive il disegno del 2. Questa mappa non cresce coi moduli della GUI: una riga
nuova nasce solo da un requisito nuovo della spec.

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
| Ogni difetto trovato in simulazione **conserva il proprio seed** | ⛔ a entrare nella suite è la **proprietà** che quel difetto violava, **non il seed** — vedi il richiamo in fondo |
| I fallimenti promossi dall'anello 4 (§5, col 4) entrano nella stessa suite | un artefatto, non due |
| Analisi statica, test a esempi **e campagna DST breve** girano a **ogni commit**, e anche le prove della GUI, in un passo loro (`scripts/gate-gui.sh`); la campagna DST **profonda** si lancia **a mano**. ⚠️ **Richiamo del 2026-09-08:** questa riga diceva *«la campagna DST profonda su cicli più lunghi»*, e quel ciclo **non esiste** — le due campagne profonde sono `#[ignore]` e nessun passo del cancello né della CI le lancia: è il vincolo 8 della §11 del compendio, aperto e del proprietario | «tieni la qualità a sinistra» (§5) |

> ⭐ **La riga sulla cadenza è cambiata dopo una misura.** Diceva «DST su cicli più
> lunghi», perché si dava per scontato che una campagna fosse cara. **M-2 l'ha smentito**:
> una corsa dello scenario minimo costa **25,8 µs**, quindi migliaia di semi stanno dentro
> un secondo. I cicli lunghi servono ad andare **più a fondo**, non a rendere possibile la
> DST. Riserva dichiarata: 25,8 µs è lo scenario *minimo*, e quelli reali saranno più
> pesanti — la misura dice che il substrato non è il collo di bottiglia, non che le
> campagne siano gratis.
>
> ⛔ **Richiamo del 2026-08-11 — la conclusione regge, il numero che la sostiene è morto, ed è
> questa formulazione a produrre il malinteso.** Chiudendo il Task 4 del Traguardo 4 la campagna
> è stata misurata sul codice che **spedisce**, e i 25,8 µs **non sono confrontabili con niente
> che esista oggi**: il prototipo che li produsse non è nel repository, l'esecutore era un altro
> — lo spike sceglieva un'attività **a caso** — e la cifra era un colpo singolo invece di una
> media. ⚠️ **E le parole *«scenario minimo»* di questo riquadro sono la causa prossima
> dell'errore**: lo scenario di M-2 il giornale **ce l'aveva**, quindi *«minimo»* qui non
> significa *«senza il giornale»*, e chi lo ha letto così ha visto un paradosso — una corsa che
> fa **di più** costando **di meno**. ✅ Ciò che il riquadro conclude è vero e per difetto: in
> `release` un secondo compra **centinaia di migliaia** di semi, e in `debug` — che è il profilo
> con cui gira il cancello, e la distinzione mancava qui — **circa diciannovemila**. 📌 Il numero
> vivo e il metodo con cui è stato scelto stanno in [`riferimenti.md`](../riferimenti.md).
>
> 📄 **Il meccanismo di questa porta** — ogni controllo con il proprio livello di forza, la
> sonda che deve scattare e la contro-sonda che deve restare verde — è la **§7 della spec
> del sotto-progetto 1**. Qui vive il *metodo*; là il *catalogo* e la cadenza operativa.

## Regole che le tabelle non esprimono

- **L'iniettabilità è un requisito di costruzione, non di test.** Nessun componente
  legge l'orologio, genera casualità o esegue I/O se non attraverso un confine
  sostituibile. Senza, la DST non è possibile — e non è retrofittabile.
- La scelta del linguaggio del core dovrà valutare esplicitamente la **sostituibilità
  dello scheduling**: è il primo caso in cui una decisione di test vincola una
  decisione di architettura. ✅ **Richiamo del 2026-09-08:** l'ha valutata — è stato lo
  spareggio #1 di ADR-0026, e ha deciso Rust. La riga resta come regola: vale per ogni
  runtime che volesse restituire all'ecosistema l'ordine delle attività (§7 del compendio).
- Un fallimento del kernel è **sempre** un difetto. Se un test del kernel è
  intermittente, il difetto è nel test o nell'iniettabilità — mai «è il modello».

---

## ⚠️ Richiamo — «il seed diventa una regressione permanente» è falsificato (2026-08-18)

La riga della porta di qualità diceva *«il seed diventa un caso di regressione permanente»*.
La **§3.4** della spec del sotto-progetto 1 e il **rimando del 2026-08-08 in
[ADR-0021](../adr/0021-simulazione-deterministica-e-iniettabilita.md)** la restringono in due
punti, e la restrizione non era mai arrivata fin qui:

| Cosa diceva | Cosa vale |
|---|---|
| il seed è un caso di regressione **permanente** | ⚠️ **no**: un seed **non riproduce la stessa esecuzione dopo un cambio di codice**. È un **punto di ripartenza per indagare**, non un oracolo |
| i seed formano una **suite di regressione** | ⚠️ **no**: a entrare nella suite è la **proprietà** che quel difetto violava. Un elenco di semi presentato come suite sarebbe una **falsa sicurezza** |

⛔ **La sostanza regge:** ogni difetto trovato in simulazione conserva il proprio seed, e il
seed si versiona — [`semi-dst.md`](../semi-dst.md) esiste per quello, e dichiara esso stesso
che al livello 2 *«un seme»* non identifica un caso.

📌 **Perché il richiamo è arrivato qui per ultimo, ed è il dato:** questo file **si dichiara
fonte di verità sulla porta di qualità**, quindi è l'ultimo posto in cui una formulazione
falsificata dovrebbe sopravvivere — e ci è sopravvissuta **dieci giorni**. È la radice **R1**
dell'[audit](../audit-2026-08-11.md): *una correzione attraversa il documento in cui nasce, non
gli altri*, e le altre case si cercano **col `grep`**, non a memoria. Finding **A-2**.

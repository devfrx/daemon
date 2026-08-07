# Tracciabilità — mappa funzionale → dove vive

Ogni funzionalità della mappa funzionale originale, con la sua sede nel progetto.
**Da aggiornare a ogni sotto-progetto chiuso.**

## Come leggere

| Stato | Significato |
|---|---|
| ✅ | **meccanismo deciso** nel kernel: la funzionalità ha già le sue fondamenta |
| 🔶 | meccanismo deciso, **politica o implementazione** nel sotto-progetto indicato |
| 📋 | **pianificata**: sotto-progetto assegnato, non ancora progettata |
| ⚠️ | **lacuna**: nessuna sede assegnata — vedi §Lacune |
| ❌ | **esclusa** per decisione esplicita |

**Il kernel non implementa nessuna funzionalità utente.** Fornisce i meccanismi su cui
tutte poggiano: ✅ significa «le fondamenta esistono», non «è fatto».

> ⚠️ **Questa tabella risponde a una domanda sola, e va saputo prima di fidarsene**
> (2026-08-07). Risponde a *«dove vive questa funzionalità»*. **Non** risponde a *«di quale
> meccanismo di kernel ha bisogno, e la spec lo nomina?»* — e in particolare un `📋`
> significa «sotto-progetto assegnato», **non** «non richiede un meccanismo di kernel».
>
> Rileggerla con la seconda domanda ha **riaperto la spec del sotto-progetto 1 su sette
> voci**, due chiuse e cinque aperte: elenco, classe e ordine in
> [HANDOFF](HANDOFF.md#prima-cosa-da-fare). Finché **F4** è aperta, la riga *Scheduling* ✅
> punta a un meccanismo — l'anello 3 — che la §0.4 non colloca né dentro né fuori, e la
> riga *File watching* 🔶 dichiara deciso un meccanismo che non ha una porta.
>
> La tabella si aggiorna **alla chiusura del sotto-progetto**, non ora: questo riquadro è
> il segnaposto che impedisce di leggerla come se fosse già vera.

---

## 1. Modelli e risorse

| Funzionalità | | Sede |
|---|---|---|
| Budget VRAM esplicito | ✅ | §2 · ADR-0005 |
| Modalità VRAM per profilo di carico | ✅ | §2 · profili di risorsa |
| Eviction e scarico modelli | ✅ | §2 · revoca della concessione |
| Swap coordinato LLM/embedding ↔ TRELLIS2 | ✅ | §2 · ADR-0006 policy LOCALE |
| Policy differenziata remoto vs locale | ✅ | §2 · ADR-0006 |
| Scarico per inattività (TTL) | 🔶 | meccanismo §2 · politica → Gestione modelli locali |
| Caricamento su richiesta e pre-caricamento | 🔶 | meccanismo §2 · politica → Gestione modelli locali |
| Tetto ai modelli residenti | ✅ | §2 · ammissione |
| Stima di fit prima del caricamento | ✅ | §2 · riserva dichiarata, picco misurato |
| Ecosistema dei worker ML | ✅ | ADR-0028 · Python, con i costi dichiarati |
| Routing locale/remoto con fallback | ✅ | §3 · ADR-0012 |
| Fallback a catena tra modelli | ✅ | §3 · ADR-0012 |
| Preferenze di provider (OpenRouter) | ✅ | §3 · vincoli della richiesta |
| Routing per compito/costo | ✅ | §3 · politica di routing |
| Structured output e constrained decoding | ✅ | §3 · ADR-0013 |
| Contabilità token e costi | ✅ | §3 · ADR-0011 |
| Avvisi e tetti di spesa | ✅ | §3 + §4 · V8 |
| Selettore di modello per compito | ✅ | §3 · profili |
| Parametri di generazione configurabili | ✅ | §3 · record di routing |
| Catalogo e download modelli locali | 📋 | Gestione modelli locali |
| Indicatore di stato modello | 📋 | GUI |

## 2. Conversazione

| Funzionalità | | Sede |
|---|---|---|
| UI/UX della chat | 📋 | GUI |
| Streaming delle risposte | ✅ | §3 + GUI |
| Sessioni multiple | 🔶 | gerarchia §3 (ADR-0011) · politica → Conversazione |
| Fork e branching | 📋 | Conversazione — il giornale lo consente |
| Modifica e rigenerazione | 📋 | Conversazione |
| Ricerca nello storico | 📋 | Conversazione |
| System prompt, personas e profili | 🔶 | guide §5 · politica → Conversazione |
| Memoria persistente | 🔶 | stato durevole §4 · politica → Conoscenza |
| Gestione del contesto e compattazione | ✅ | §4 ADR-0008 · §5 ADR-0010 |
| Indicatore di riempimento contesto | ✅ | §7 · misura per categoria |
| Artifacts/canvas | 📋 | GUI |
| Preview renderizzate | 📋 | GUI |
| Allegati in chat | 📋 | Conversazione (+ etichettatura I6) |
| Comandi rapidi e slash-command | 📋 | GUI |
| Template e prompt salvati | 📋 | Conversazione |
| Esportazione conversazioni | 📋 | Conversazione |

## 3. Conoscenza

| Funzionalità | | Sede |
|---|---|---|
| RAG e indicizzazione | 📋 | Conoscenza |
| Modalità piena vs recupero | 📋 | Conoscenza |
| Embedding e reranking | 🔶 | risorsa §2 + gateway §3 · politica → Conoscenza |
| Ricerca ibrida | 📋 | Conoscenza |
| Ingest di documenti (PDF, OCR) | 📋 | Conoscenza (+ I6) |
| Chunking configurabile | 📋 | Conoscenza |
| Collezioni e knowledge base | 📋 | Conoscenza |
| File watching e awareness del progetto | 🔶 | trigger anello 3 §5 · politica → Conoscenza |
| Multi-repo/multi-progetto | 📋 | Conoscenza |
| Mappa del progetto | 📋 | Conoscenza |
| Ricerca web integrata | 📋 | Conoscenza (+ I6) |
| Loop di deep research | 🔶 | anelli §5 + run durevoli §4 · politica → Conoscenza |
| Report strutturato con citazioni | 📋 | Conoscenza |
| Citazioni e verificabilità | 🔶 | sensore §5 · politica → Conoscenza |
| Pianificazione della ricerca rivedibile | 📋 | Conoscenza |
| Gestione del decadimento delle fonti | 📋 | Conoscenza |

## 4. Agenti

| Funzionalità | | Sede |
|---|---|---|
| Orchestrazione e sub-agenti | ✅ | §4 · sub-run, proiezione ristretta (ADR-0008) |
| Planning e decomposizione dei task | 🔶 | piano nello stato durevole §4 · politica → Agenti |
| Modalità piano vs esecuzione | 🔶 | preset §6 · politica → Agenti |
| Tool calling | ✅ | §3 schema + §6 permessi |
| MCP | ✅ | ADR-0003 · ADR-0015 |
| Skills | ✅ | guide §5 · ADR-0003 |
| HITL: approvazioni | ✅ | §6 · ADR-0016 |
| HITL: interruzione e steering | 🔶 | `AttesaUmano` §4 · politica → Agenti |
| Domande di chiarimento | 📋 | Agenti |
| Checkpoint e rollback | ✅ | giornale §4 + §10 · ADR-0024 |
| Run persistenti, ripresa e cancellazione | ✅ | §4 · ADR-0007 |
| Task in background | ✅ | §4 + ADR-0004 |
| Scheduling | ✅ | trigger anello 3 §5 |
| Coda e priorità delle run | 🔶 | corsie §2 · coda generica → Agenti |
| Replay dei trace | ✅ | §7 · ADR-0017 |
| Valutazione degli agenti | 📋 | Agenti — esplicitamente **fuori** dal kernel (§8) |
| Regole e vincoli di progetto | ✅ | guide §5 |
| Agenti in parallelo isolati | 🔶 | sub-run §4 · isolamento su disco → Coding |
| Limiti di autonomia configurabili | ✅ | §4 · V8 |

## 5. Coding

| Funzionalità | | Sede |
|---|---|---|
| Sandboxing ed esecuzione | ✅ | permessi §6 + §10 · ADR-0025 — quattro livelli |
| Edit dei file e diff review | 📋 | Coding |
| Terminale integrato | 📋 | Coding |
| LSP e analisi statica | 🔶 | sensori §5 · implementazione → Coding |
| Git e gestione branch | 📋 | Coding |
| Isolamento per branch di lavoro | 📋 | Coding |
| Undo/checkpoint del filesystem | ✅ | §10 · ADR-0024 — ambiti dichiarati |
| Esecuzione test | 🔶 | sensori §5 · implementazione → Coding |
| Modalità di edit selezionabili | 📋 | Coding |
| Anteprima applicazioni | 📋 | Coding + GUI |
| Approvazione comandi distruttivi | ✅ | §6 + effetti `irripetibili` §4 |

## 6. Voce

| Funzionalità | | Sede |
|---|---|---|
| Wake word | 📋 | Voce |
| Push-to-talk e dettatura | 📋 | Voce + L3 |
| VAD (rilevazione della voce) | 📋 | Voce |
| STT (voce → testo) | 📋 | Voce |
| TTS (testo → voce) | 📋 | Voce |
| Barge-in e interruzione | 📋 | Voce |
| Interruzione attivabile/disattivabile | 📋 | Voce |
| Gestione dei turni | 📋 | Voce |
| Conversazione continua | 📋 | Voce |
| Stati di ascolto/pensiero/parlato | 📋 | Voce + GUI |
| Ducking dell'audio | 📋 | Voce + L3 |
| Controlli di privacy del microfono | 📋 | Voce + L3 |
| Cancellazione eco / anti auto-attivazione | 📋 | Voce |
| Lettura vocale delle notifiche | 📋 | Voce |
| Convivenza pipeline audio ↔ job GPU | ✅ | §2 · quota sottratta, corsie (ADR-0005) |

## 7. Multimodalità e generazione

| Funzionalità | | Sede |
|---|---|---|
| Input immagini e vision | 🔶 | gateway §3 · politica → Conversazione |
| Screenshot e comprensione dello schermo | 📋 | L3 + Conversazione |
| Generazione immagini | 📋 | Generazione asset |
| TRELLIS2 single-image | 📋 | Generazione asset (+ profili §2, SP-1) |
| TRELLIS2 multiview | 📋 | Generazione asset |
| Parametri di qualità 3D configurabili | ✅ | §2 · profili di risorsa (la curva) |
| Post-processing mesh | 📋 | Generazione asset |
| Export mesh (GLB/OBJ/PLY) | 📋 | Generazione asset |
| Viewer 3D integrato | 📋 | GUI |
| Libreria degli asset generati | 📋 | Generazione asset |
| Cronologia e riproducibilità | ✅ | giornale §4 + record di routing §3 |
| Coda dei job di generazione | ✅ | code §2 + run §4 |
| Progress e notifiche per job lunghi | ✅ | §7 + V9 |
| Rimozione dello sfondo/preparazione input | 📋 | Generazione asset |

## 8. Sistema

| Funzionalità | | Sede |
|---|---|---|
| Permessi e sandbox policy | ✅ | §6 + §10 · ADR-0025 |
| Difese da prompt injection | ✅ | §6 · ADR-0014 |
| Difesa da tool poisoning | ✅ | §6 · ADR-0015 |
| Gestione segreti e credenziali | ✅ | §10 · ADR-0023 — gestore unico |
| Storage e cifratura a riposo | ✅ | §10 · ADR-0022 + ADR-0023 |
| Backup ed export dei dati | ✅ | §10 · ADR-0022 — solo l'irriproducibile |
| Osservabilità e tracing locale | ✅ | §7 · ADR-0017 |
| Logging | ✅ | §7 |
| Hotkey globale, tray e clipboard | 📋 | L3 |
| Automazione OS | 📋 | L3 |
| Notifiche | 🔶 | V9 §4 · implementazione → L3 |
| Avvio automatico e daemon in background | ✅ | ADR-0004 · implementazione → L3 |
| Packaging e aggiornamenti | 📋 | L3 |
| Estensibilità e plugin | ✅ | ADR-0003 |
| Accessibilità | 📋 | GUI |
| Internazionalizzazione (i18n) | 📋 | GUI + Voce |
| Comportamento offline | ✅ | §7 · ADR-0019 |
| Impostazioni e profili di configurazione | 🔶 | profili §2/§3 · pannello → GUI |

---

## Funzionalità emerse dal confronto con prodotti comparabili

### Coesistenza sulla stessa GPU

| Funzionalità | | Sede |
|---|---|---|
| Semaforo unico delle risorse GPU | ✅ | §2 · ADR-0005 |
| Precedenza della voce sui job pesanti | ✅ | §2 · quota **sottratta** |
| Avviso di conflitto e stima d'attesa | ✅ | §2 · `InCoda` |
| Passaggio suggerito a OpenRouter durante i render | ✅ | §2 · ADR-0006, transizione esplicita |
| Warm-up e cold-start visibili | ✅ | Q8 |
| Budget di VRAM riservata all'audio | ✅ | §2 · ADR-0005 |

### Ciclo di vita e affidabilità delle run

| Funzionalità | | Sede |
|---|---|---|
| Notifica «l'agente ha bisogno di te» | ✅ | V9 |
| Determinismo/replay riproducibile | ✅ | §4 + §8 (seed) |
| Hook sul ciclo di vita | 🔶 | trigger e anelli §5 · politica → Agenti |
| Classificatore di sicurezza delle azioni | 🔶 | si realizza come **sensore** §5 · politica → Agenti |
| Dataset dai fallimenti | ✅ | §7 promozione + §8 |
| Analisi dei costi per run e per sub-agente | ✅ | §3 · ADR-0011 |

### Conversazione, conoscenza e UX

| Funzionalità | | Sede |
|---|---|---|
| RAG «agentico» su filesystem | 📋 | Conoscenza |
| Sincronizzazione automatica delle cartelle | 🔶 | trigger §5 · politica → Conoscenza |
| Backup della KB indipendente dall'app | ✅ | §10 · ADR-0022 — documenti nel backup, indice ricostruito |
| Memoria selettiva ed episodica | 📋 | Conoscenza |
| Gestione delle contraddizioni nella memoria | 📋 | Conoscenza |
| Full-context mode per singolo allegato | 📋 | Conoscenza |

### Voce e interazione

| Funzionalità | | Sede |
|---|---|---|
| Condivisione visiva in push-to-talk | 📋 | Voce + L3 |
| Instradamento dei comandi semplici in locale | 📋 | Voce |
| Timer, sveglie e annunci | 📋 | Voce |
| Doppia wake word / doppia lingua | 📋 | Voce |
| Metriche di qualità voce | 🔶 | metriche §7 · definizione → Voce |
| Autonomia proattiva a corsie di priorità | 📋 | Voce + Agenti |

### Sistema, sicurezza e governance

| Funzionalità | | Sede |
|---|---|---|
| Zero-Data-Retention selettivo | ✅ | §3 ADR-0012 + §6 ADR-0016 (escalation) |
| Fatturazione a stream interrotto | ✅ | §3 · ADR-0011 |
| Politica di routing come oggetto versionato | ✅ | §3 · ADR-0011 |
| Canary per esfiltrazione dati | ✅ | §6 · ADR-0016 |
| Modalità di permessi a più livelli | ✅ | §6 · preset |
| Marketplace/registry di estensioni | ❌ | **escluso** da ADR-0003 (nessun plugin nativo). Resta la scoperta di server MCP → 📋 GUI |
| Overlay/finestra fluttuante sempre in primo piano | 📋 | GUI + L3 |
| Telemetria locale e «no telemetry» garantito | ✅ | §7 · ADR-0017, V25 |

### Generazione asset e pipeline creativa

| Funzionalità | | Sede |
|---|---|---|
| Workflow a nodi/pipeline riusabili | 📋 | Generazione asset |
| Batch e generazione a lotti | 🔶 | code §2/§4 · politica → Generazione asset |
| Validazione della mesh prima dell'export | ✅ | sensore §5 · ADR-0009 |
| Confronto varianti | 📋 | Generazione asset + GUI |
| Rendering di anteprima video dell'asset | 📋 | Generazione asset |

### Ergonomia e affidabilità

| Funzionalità | | Sede |
|---|---|---|
| Prompt history ricercabile | 📋 | Conversazione |
| Auto-retry su errori transitori | ✅ | §3 · V17 |
| Modalità «solo lettura»/dry-run | 🔶 | preset §6 + classi di effetto §4 · politica → Agenti |
| Degrado esplicito quando manca la rete | ✅ | §7 · ADR-0019 |

---

## Lacune — chiuse dalla §10

Trovate da questo esercizio di tracciabilità il 2026-08-06, tutte in L0, tutte chiuse
dalla §10 della spec del kernel lo stesso giorno.

| # | Lacuna | Chiusa da |
|---|---|---|
| L-1 | Undo/checkpoint del filesystem | [ADR-0024](adr/0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md) — ambiti dichiarati, write-ahead sui file |
| L-2 | Storage e cifratura a riposo | [ADR-0022](adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) + [ADR-0023](adr/0023-cifratura-a-riposo-e-gestore-dei-segreti.md) |
| L-3 | Backup ed export dei dati | [ADR-0022](adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) — solo l'irriproducibile |
| L-4 | Gestore dei segreti | [ADR-0023](adr/0023-cifratura-a-riposo-e-gestore-dei-segreti.md) — punto unico di lettura |
| L-5 | Confinamento reale dell'esecuzione | [ADR-0025](adr/0025-confinamento-a-livelli.md) — quattro livelli, default 2 |

**Nessuna lacuna aperta.** Se un nuovo esercizio di tracciabilità ne trova altre, si
aggiungono qui con lo stesso formato.

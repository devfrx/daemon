# ADR-0016: Permessi come tripla, e default dei vincoli sui dati per profilo

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

> ⚠️ **Rimando del 2026-10-04 — che cosa dei punti 2 e 4 è costruito, e due scelte aperte.** Audit del
> 2026-09-30. **Nessuna riga di questo ADR è superata.**
>
> - **«le azioni fermate da un sensore»**, nel preset `autonomo` del punto 2. L'anello costruito — `run_the_ring`
>   in `crates/kernel/src/sensor.rs` — fa rientrare un verdetto negativo come passo nuovo e non chiede a nessuno,
>   come vogliono Q10 e il fratello [ADR-0013](0013-conformita-allo-schema-e-un-verdetto-di-sensore.md): un'azione
>   che un sensore ferma per chiedere conferma non ha un meccanismo. Se a fermarla — e a fermare il canary del
>   punto 5 prima che un contenuto esca — sia una guida che agisce prima, o una seconda conseguenza dell'anello
>   accanto al passo correttivo, è la scelta aperta su AUD-139.
> - **L'escalation automatica**, nel punto 4, non è costruita: la crate `secrets` non ha codice, nessun tipo del
>   kernel segna il contenuto passato dal gestore dei segreti, e il kernel ha i soli vincoli sui dati `LocalOnly` e
>   `NoRetention`, col fallimento chiuso di ADR-0012 (`crates/kernel/src/gateway/mod.rs`). Discende dal gestore dei
>   segreti — [ADR-0023](0023-cifratura-a-riposo-e-gestore-dei-segreti.md), punto 4 — e nasce con lui: la riga V34
>   della §8.3 della spec del sotto-progetto 1, innesco B (3). AUD-585.
> - **«la classe più stretta»**, nello stesso punto, non è definita: le classi di ADR-0012 sono due, `Data` e
>   `Quality`, distinte per l'esito a catena esaurita e non ordinate, e la classe è di un vincolo, non di una
>   richiesta (`Constraint::class`). Quali vincoli sui dati prenda una richiesta che sale è la scelta aperta su
>   AUD-586.

## Context

Due questioni aperte convergono qui.

**Prima: la granularità dei permessi.** Il sistema esegue codice generato da un
modello e strumenti di terze parti. Un permesso concesso «allo strumento filesystem»
copre l'intero disco: è privilegio minimo solo di nome.

**Seconda: il default dei vincoli sui dati**, lasciato aperto da
[ADR-0012](0012-equivalenza-del-fallback-e-fallimento-chiuso.md). Il dilemma è reale:

| Default | Effetto |
|---|---|
| permissivo sempre | la regola di ADR-0012 non scatta mai: difesa scritta e mai usata |
| restrittivo sempre | molti provider diventano inutilizzabili, e l'utente disattiva tutto — che è il fallimento peggiore, perché lascia il sistema *apparentemente* protetto |

Alternative considerate per i permessi:

- **Per strumento.** Semplice. *Contro:* grossolano al punto da non essere una difesa.
- **Per tripla (strumento × risorsa × operazione).** *Contro:* più richieste di
  approvazione.

## Decision

**1. Un permesso è una tripla:** `(strumento × risorsa × operazione)`.

| Esempio | Significato |
|---|---|
| `(file, ~/progetti/x, lettura)` | può leggere lì, e solo lì |
| `(shell, comandi in allow-list, esecuzione)` | non «la shell» |
| `(rete, host specifico, uscita)` | non «internet» |

**2. Tre preset di supervisione, con default prudente:**

| Preset | Comportamento |
|---|---|
| `chiede sempre` | ogni azione con effetto richiede conferma |
| **`auto-approva sicuri`** *(default)* | letture, test e build procedono; scritture, comandi e uscite di rete chiedono |
| `autonomo` | conferma solo per effetti `irripetibili` (§4) e per le azioni fermate da un sensore |

**3. Un'approvazione non si estende.** Vale per la tripla concessa e per la sessione
corrente. Approvare `(file, ~/progetti/x, scrittura)` non concede `~/progetti/y`, e
non vale domani.

**4. Default dei vincoli sui dati: dichiarato dal profilo di configurazione, con
escalation automatica.** Il profilo attivo dichiara il proprio default. Indipendente-
mente da esso, **qualunque richiesta il cui contenuto abbia attraversato il gestore
dei segreti sale automaticamente alla classe più stretta**; se non trova endpoint
conforme, fallisce chiuso (ADR-0012).

Risolve il dilemma senza sceglierne un corno: il default resta usabile, ma la regola
scatta sempre dove conta davvero.

**5. Canary di esfiltrazione.** Il gestore dei segreti contiene valori sentinella; la
loro comparsa in contenuto in uscita è un **verdetto di sensore** (§5) che blocca e
segnala. Nessuna infrastruttura nuova: riusa il meccanismo dell'anello di verifica.

## Consequences

- **Positive:**
  - Privilegio minimo reale, non nominale.
  - Il default dei vincoli sui dati non è né inutile né paralizzante.
  - Il canary è un sensore: si aggiunge senza costruire un sottosistema.
  - Il preset prudente è il default; l'autonomia si sceglie, non si eredita.
- **Negative (accettate):**
  - Più richieste di approvazione. I preset le riducono, non le eliminano — e
    l'utente che approva per stanchezza resta il modo di fallire più probabile
    dell'intero capitolo sicurezza.
  - **Il canary rileva l'esfiltrazione dei segreti _noti_**, non di dati sensibili
    generici. È una rete, non un muro: dichiararlo evita falsa sicurezza.
  - **L'escalation automatica dipende dal fatto che i segreti passino dal gestore.**
    Un segreto incollato a mano in chat la aggira completamente.
- **Follow-up richiesti:**
  - §7 deve mostrare **quali permessi sono attivi** in una sessione. Un permesso
    concesso e dimenticato è indistinguibile da un permesso mai concesso.
  - Valutare in §7 un sensore che segnali quando un segreto compare in chiaro
    nell'input dell'utente: è la falla del punto precedente, ed è rilevabile.

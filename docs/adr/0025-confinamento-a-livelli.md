# ADR-0025: Confinamento a livelli — il kernel richiede, la piattaforma implementa

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

Lacuna L-5. [ADR-0016](0016-permessi-granulari-e-default-dei-vincoli-sui-dati.md)
decide **cosa** si può toccare — la tripla `(strumento × risorsa × operazione)` — ma
non **come** si impedisce a un processo di toccare altro.

La distinzione è sostanziale e va detta senza ambiguità: i permessi applicativi sono
un confine solo finché *tutti* gli accessi passano dal mediatore. **Un processo figlio
che esegue codice non passa dal mediatore**: può aprire qualsiasi file a cui l'utente
abbia accesso. Contro codice eseguito, la tripla da sola non è una difesa.

Lo stato dell'arte 2026 degli agenti di coding usa primitive del sistema operativo:

| Sistema | Primitive in uso |
|---|---|
| Windows | token ristretti costruiti da SID e ACL, AppContainer |
| Linux | Landlock per il filesystem, seccomp-BPF per le chiamate di sistema, namespace |
| macOS | Seatbelt |

Sopra di esse c'è la **macchina virtuale leggera**: Firecracker avvia in ~125 ms con
meno di 5 MiB di overhead per istanza. La differenza rispetto al sandbox del kernel è
sostanziale: **con un sandbox del kernel una fuga dà accesso all'host; con una microVM
l'attaccante deve superare anche l'hypervisor.**

Alternative considerate:

- **Solo permessi applicativi.** Nessuna implementazione per OS.
  *Contro:* non è un confine contro codice eseguito, cioè contro il caso che conta.
- **Sempre macchina virtuale leggera.** Isolamento massimo.
  *Contro:* costo e complessità su *ogni* esecuzione, anche per far girare un linter.
- **Livelli.** *Contro:* più implementazioni da mantenere.

## Decision

Definiamo **quattro livelli di confinamento**. Il kernel **richiede** un livello per
ogni azione; il modulo di piattaforma lo **implementa** (I3). Il kernel non sa come.

| Livello | Confine | Garantito da | Regge contro codice eseguito? |
|---|---|---|---|
| **0** | nessuno | — | no |
| **1** | permessi applicativi | il kernel media ogni accesso (tripla §6) | **no** |
| **2** | processo ristretto dell'OS | primitive di sistema | sì |
| **3** | macchina virtuale leggera | hypervisor | sì, anche a fuga dal kernel guest |

**Default: livello 2 minimo per qualsiasi esecuzione di codice generato o di comando.**
Il livello 1 resta ammesso solo per strumenti interni che non eseguono codice. Il
livello 3 è opzionale e non fa parte del primo traguardo.

**Se il livello richiesto non è disponibile su questa piattaforma, l'azione non parte.**
Fail-closed, coerente con [ADR-0012](0012-equivalenza-del-fallback-e-fallimento-chiuso.md):
un confinamento più debole di quello richiesto **non è un ripiego**, è un'altra cosa.

## Consequences

- **Positive:**
  - Il livello è dichiarato per azione, registrato nel giornale e verificabile; il
    kernel resta libero da codice OS-specifico (I3).
  - Salire di livello — dal 2 al 3, quando servirà eseguire codice di provenienza
    davvero ignota — non richiede riprogettare nulla: cambia il livello richiesto.
  - Chiude l'illusione più pericolosa del capitolo sicurezza: che i permessi
    applicativi bastino a contenere codice eseguito.
- **Negative (accettate):**
  - **Il livello 2 richiede un'implementazione per ciascun sistema operativo**, ed è
    il pezzo più costoso del modulo di piattaforma. Su Linux richiede primitive
    diverse da quelle di Windows: è esattamente il rischio RK-11 (confine verificato
    ma non validato).
  - Il fail-closed significa che su una piattaforma non ancora supportata
    **l'esecuzione di codice è semplicemente disabilitata**. È sicuro e onesto, ma
    va detto: l'app non «funziona a metà», rifiuta di eseguire.
- **Follow-up richiesti:**
  - Il livello di confinamento usato entra nel giornale insieme al passo: senza, non
    si può stabilire a posteriori in quali condizioni un comando è stato eseguito.
  - Il livello 3 va valutato quando arriverà il primo caso d'uso che lo giustifica —
    eseguire codice da un repository sconosciuto — non prima.

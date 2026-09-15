"""patch_c10.py -- task 10 (R4-19..R4-27, R10-18, R10-11, R9a-14, D76) and the recall in P-61 (R4-22). Scoped,
asserted, atomic."""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
PLAN = r"C:\Users\zagor\Desktop\harness\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima.md"
raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw
t = raw


def scoped(text, start_marker, end_marker, subs):
    lo = text.index(start_marker)
    hi = text.index(end_marker, lo)
    seg = text[lo:hi]
    for old, new, n in subs:
        c = seg.count(old)
        assert c == n, f"expected {n}, found {c}: {old[:80]!r}"
    for old, new, n in subs:
        seg = seg.replace(old, new)
    return text[:lo] + seg + text[hi:]


FILES_BLOCK = """**Files:**
- Create: `crates/simulator/tests/serving_campaign.rs` (**LF**) — la campagna, le due proprietà, i due spazi dei mondi
- Modify: `scripts/gate.sh` (**`i/lf w/crlf`**) — la riga `serving_campaign` nel settimo passo, con `replace_unique.py`, **e la riga dei costi rimisurata** (R4-27)
- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**`i/lf w/lf`**) — il richiamo datato nella §5, riga *«l'attività»* (**D32**)
- Modify: **questo piano** — la riga **10** della tabella della posizione
"""

t = scoped(t, "\n## Compito 10:", "\n## Compito 11:", [
    # Files (R4-19, R10-18): the block was missing
    ("\n\n- Read: la **§5 del 2**, la riga *«l'attività»* e la sua colonna *«la prova»*;",
     "\n\n" + FILES_BLOCK + "- Read: la **§5 del 2**, la riga *«l'attività»* e la sua colonna *«la prova»*;", 1),
    # Interfaces (R4-23, R10-11)
    ("- Consumes, dal **compito 4**: `kernel::ports::custody::Custody`",
     "- Consumes, dal **compito 4**: nulla per nome — `MemoryCustody::new()` è inerente e `Core::new` prende il tipo, quindi il tratto `Custody` **non si importa** (R4-23; qui stava `kernel::ports::custody::Custody`)", 1),
    ("`kernel::ports::ipc::{ClientId, Ipc, IpcError}`; `kernel::ports::journal::Journal`;\n  `kernel::ports::reactor::Reactor`;",
     "`kernel::ports::ipc::{ClientId, Ipc, IpcError}`; `kernel::ports::reactor::Reactor`;", 1),
    ("`kernel::time::{Millis, Monotonic, WallTime}`; `simulator::custody::MemoryCustody`;\n  `simulator::journal::CrashingJournal`; `simulator::reactor::VirtualReactor`; `simulator::rng::SeededRng`",
     "`kernel::rng::RngExt` (per `below`); `kernel::time::{Millis, Monotonic, WallTime}`; `simulator::custody::MemoryCustody`;\n  `simulator::journal::{CrashingJournal, MemoryJournal}`; `simulator::reactor::VirtualReactor`; `simulator::rng::SeededRng` — ⚠️ riletti dagli `use` del Passo 2 (R4-23, R10-11): `kernel::ports::journal::Journal` **non** si importa, nessun metodo del tratto è chiamato", 1),
    # Passo 1 (R4-21, R4-26)
    ("grep -n 'pub enum Resolution' -A 10 crates/kernel/src/reconcile.rs\n",
     "grep -n 'pub enum Resolution' -A 10 crates/kernel/src/reconcile.rs\ngrep -n 'fn replay' -A 2 crates/simulator/src/journal.rs\n", 1),
    ("⛔ **`ClientGrants` ha TRE elementi pubblici** —\n`new`, `register`, `on_disconnect` — e **nessun modo di contare senza rilasciare** (**P-60**); `Resolution` ha\n**tre** varianti; `gate.sh` è",
     "⛔ **`ClientGrants` ha TRE metodi pubblici** — il\n`grep` rende **quattro** righe, la `struct` e i suoi tre metodi `new`, `register`, `on_disconnect` (R4-26) — e **nessun\nmodo di contare senza rilasciare** (**P-60**); `Resolution` ha **tre** varianti; il `replay` di `CrashingJournal` — il\n**secondo** `fn replay` del file — delega a `self.inner.replay()` senza passare da `may_write`, che è la verifica che il\nPasso 4 cita (R4-21); `gate.sh` è", 1),
    # Passo 2 (D76)
    ("//! The milestone 2 campaign: `kernel::serving::serve` under two faults",
     "//! The sub-project 2 campaign: `kernel::serving::serve` under two faults", 1),
    ("WITHOUT A GRANT PUT IN BY HAND. Milestone 2 issues NO", "WITHOUT A GRANT PUT IN BY HAND. Sub-project 2 issues NO", 1),
    # Passo 3 (R4-20): the twin of `without_crash`
    ("    fn dies(&mut self, client: ClientId, at: u64) {\n        self.dying = Some(client);\n        self.dies_at = at;\n    }\n",
     "    fn dies(&mut self, client: ClientId, at: u64) {\n        self.dying = Some(client);\n        self.dies_at = at;\n    }\n\n"
     "    /// The twin of `CrashingJournal::without_crash()`: the client is WATCHED and never dies, so\n"
     "    /// that `operations` counts. ⛔ IT EXISTS FOR THE PRIZE PROBE: `may_operate` counts only the\n"
     "    /// client it watches, and a run with no watched client counts nothing at all -- measured at the\n"
     "    /// plan review (R4-20), where the prize probe as first written asserted `0 >= OPERATIONS`.\n"
     "    fn watched_for_ever(&mut self, client: ClientId) {\n"
     "        self.dies(client, u64::MAX);\n"
     "    }\n", 1),
    # Passo 5 (R4-20)
    ("| `the_activity_operates_on_a_welcomed_client_at_least_this_many_times` | il cablaggio di `one_death` **senza** `Wire::dies` |",
     "| `the_activity_operates_on_a_welcomed_client_at_least_this_many_times` | il cablaggio di `one_death` con `open.watched_for_ever(GUI)` al posto di `open.dies(GUI, …)` — il gemello di `CrashingJournal::without_crash()` (R4-20: **senza** un client sorvegliato `may_operate` non conta nulla, e la sonda com'era scritta asseriva `0 >= OPERATIONS`) |", 1),
    # Passo 6 (R4-22)
    ("sul campo — ⚠️ **ed è l'unico `match` che questa campagna contiene**, dichiarato qui perché non venga letto come\nla decisione che la riga 24 del Traguardo 6 tiene aperta.",
     "sul campo — ⚠️ **ed è uno dei DUE `match` della campagna, con `From<Resolution> for Doubt` del Passo 4: entrambi\nconvertono per contare, nessuno decide** (R4-22; qui stava «l'unico», e P-61 diceva «l'unico» dell'altro), dichiarato\nqui perché nessuno dei due venga letto come la decisione che la riga 24 del Traguardo 6 tiene aperta.", 1),
    # Passo 7 (R4-24, R4-27)
    ("bash scripts/gate.sh 2>&1 | tee \"$SCRATCH/gate-serving-campaign.log\" | grep -c 'DST serving'",
     "bash scripts/gate.sh 2>&1 | grep -c 'DST serving'", 1),
    ("⛔ **Poi si rimette**, e `git diff scripts/gate.sh` deve mostrare **una sola** riga aggiunta.\n",
     "⛔ **Poi si rimette**, e `git diff scripts/gate.sh` deve mostrare **una sola** riga aggiunta. ⚠️ Qui stava un `tee`\nsu `$SCRATCH`, che nessuna riga definiva (R4-24): il log del cancello lo tiene chi esegue nello scratchpad, come per\nogni corsa.\n\n"
     "⛔ **E la riga dei costi sopra il settimo passo si RIMISURA (R4-27):** quel commento dice *«THE FIGURE IS RE-MEASURED\nWHENEVER THIS LIST CHANGES AND NEVER REALIGNED FROM MEMORY»*, e questo compito cambia la lista — lasciarlo com'è lo\nrende falso per la sua stessa regola (gotcha #31). Si lancia `cargo test --locked -p simulator --test <bersaglio> 2>&1 | grep 'finished in'`\nper ciascuno dei **sei** bersagli del settimo passo, **due volte** (la seconda è quella che conta, a cache calda, come\nla misura del 2026-09-02); poi *Trova* le **tre** righe da `# ⚠️ TWO COSTS, both declared. The short campaigns run twice: RE-MEASURED on 2026-09-02,`\nfino a `engine_crash_consistency 0.41s.`, **intere, prese dal file**, e *Sostituisci con* la stessa forma coi sei tempi,\n`serving_campaign` compreso, e la data — `RE-MEASURED on <data>` — con `replace_unique.py` (file CRLF). Le cifre\nvecchie restano nella storia di git, non nel commento; il capoverso sotto, sull'ordine di grandezza, resta com'è.\n", 1),
    # Passo 8 (R4-25, R9a-14): one line, with the anchor
    ("Nella **§5** (`i/lf w/lf`, quindi Python con `newline=\"\"`), in coda alla colonna *«la prova»* della riga\n*«l'attività»*:\n\n"
     "> ⛔ **RICHIAMO DEL \\<data\\>, compito 10 del piano della parte 2 (D32): `DyingGui` NON è lo strumento, e le\n"
     "> ragioni sono tre, misurate.** Dice una cosa sola, `IpcMessage::Request`, che il dispaccio lascia cadere\n"
     "> (**D5**); non può pronunciare né `Hello` né `Approve`, quindi il crash del giornale a metà invocazione non\n"
     "> sarebbe raggiungibile; e verrebbe **spostata dentro `Core`**, che non espone il trasporto, quindi la campagna\n"
     "> non potrebbe interrogarla. Il filo vive **fuori** dal core dietro un `RefCell`, com'è nel banco del compito 7.\n"
     "> ⚠️ **E la concessione si mette a MANO** attraverso `Core::grants`: il 2 non ne rilascia nessuna a un client\n"
     "> (**D5**), quindi senza di essa la somma tornerebbe a una baseline da cui non si è mai mossa.\n",
     "Nella **§5** (`i/lf w/lf`, quindi Python con `newline=\"\"`), in coda alla cella *«la prova»* della riga *«l'attività»*.\n"
     "⛔ **L'ancora è la riga INTERA, presa dal file:** quella che comincia con `| l'attività |` — `grep -c -F` → **1**;\n"
     "⚠️ la coda della cella, *«lascia il passo A in dubbio con la sua classe»*, compare **due** volte nel file (§5 e §8) e\n"
     "non è un'ancora (R9a-14). Il richiamo si appende **su una riga**, prima dell'ultimo `|` della riga, come fa il Passo 15\n"
     "del 9 — un blockquote di sette righe dentro una cella spezzerebbe la tabella (R4-25):\n\n"
     "```\n"
     "⛔ **RICHIAMO DEL <data>, compito 10 del piano della parte 2 (D32): `DyingGui` NON è lo strumento, e le ragioni sono tre, misurate.** Dice una cosa sola, `IpcMessage::Request`, che il dispaccio lascia cadere (**D5**); non può pronunciare né `Hello` né `Approve`, quindi il crash del giornale a metà invocazione non sarebbe raggiungibile; e verrebbe **spostata dentro `Core`**, che non espone il trasporto, quindi la campagna non potrebbe interrogarla. Il filo vive **fuori** dal core dietro un `RefCell`, com'è nel banco del compito 7. ⚠️ **E la concessione si mette a MANO** attraverso `Core::grants`: il 2 non ne rilascia nessuna a un client (**D5**), quindi senza di essa la somma tornerebbe a una baseline da cui non si è mai mossa.\n"
     "```\n\n"
     "Con Python (`newline=\"\"`, temporaneo e `os.replace`): `assert text.count(anchor) == 1`, poi la riga ricomposta come\n"
     "`anchor.rstrip().removesuffix(\"|\").rstrip() + \" \" + richiamo + \" |\"`.\n", 1),
])

# ------------------------------------------------------------------ P-61 (R4-22)
t = scoped(t, "\n### P-61 ", "\n### P-62 ", [
    ("⚠️ **E quel `match` è l'UNICO della campagna, dichiarato nel compito perché non",
     "⚠️ **E quel `match` è uno dei DUE della campagna — l'altro è l'aiutante `the_doubt_the_one_function_resolves_to()` del Passo 6, e ciascuno dei due compiti diceva «l'unico» del proprio; entrambi convertono per contare, nessuno decide (R4-22, 2026-09-15) — dichiarato nel compito perché non", 1),
])

tmp = PLAN + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(t)
os.replace(tmp, PLAN)
print("ok: task 10 patched (plus P-61);", t.count("\n") - raw.count("\n"), "lines added")

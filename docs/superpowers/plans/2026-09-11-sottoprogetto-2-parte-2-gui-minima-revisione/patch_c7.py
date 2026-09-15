"""patch_c7.py -- task 7 (R3-1..R3-14, R3-17, R3-18, R9a-9, R9a-10, R10-16, D75, D76) plus the eol label of
record_v1.map in task 6 (R10-16, left behind by patch_c6.py). Scoped, asserted, atomic."""
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


BENCH_CUSTODY = r"""/// The seventh port of this bench: `MemoryCustody` underneath, plus a tap that makes `keep` refuse.
///
/// ⛔ IT EXISTS FOR ONE PROBE, `save_layout_that_the_port_refuses_comes_back_with_the_old_package`:
/// decision 13 says a failed write comes back as the OLD package, and `MemoryCustody` does not know
/// how to fail. The tap is a `Cell` so that the `before` closure can arm it through `&C`, which is
/// all `Core::custody` hands out. Measured at the plan review (R9a-9, 2026-09-15): without it the
/// second direction of decision 13 was a sentence in a comment and not a probe.
struct BenchCustody {
    inner: MemoryCustody,
    keeps_left: Cell<u32>,
}

impl BenchCustody {
    fn new() -> Self {
        BenchCustody {
            inner: MemoryCustody::new(),
            keeps_left: Cell::new(u32::MAX),
        }
    }
}

impl Custody for BenchCustody {
    fn keep(&mut self, key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        if self.keeps_left.get() == 0 {
            return Err(CustodyError::Unavailable);
        }
        self.keeps_left.set(self.keeps_left.get() - 1);
        self.inner.keep(key, bytes)
    }

    fn retrieve(&self, key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        self.inner.retrieve(key)
    }
}

type BenchCore<'b> = Core<FakeIpc<'b>, MemoryJournal, BenchCustody>;"""

NEW_PROBES = r"""#[test]
fn save_layout_that_the_port_refuses_comes_back_with_the_old_package() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(
        GUI,
        &[
            IpcMessage::Hello(build_stamp()),
            IpcMessage::SaveLayout(vec![7, 7, 7]),
            IpcMessage::SaveLayout(vec![9, 9, 9]),
        ],
    );

    bench.round(
        // ⛔ THE SECOND DIRECTION OF DECISION 13: one `keep` is allowed, the second is refused.
        |core| core.custody().keeps_left.set(1),
        |core| {
            assert_eq!(
                core.custody().retrieve(CustodyKey::Layout),
                Ok(Some(vec![7, 7, 7])),
                "the refused write must not have reached the port"
            );
        },
    );

    let heard = bench.heard(GUI);
    // The core answers with what it HOLDS, both times: no error variant, and the second answer is
    // the OLD package, which is how the gui learns that the save did not stick.
    assert_eq!(
        heard.get(5),
        Some(&IpcMessage::Layout(LayoutState::Package(vec![7, 7, 7]))),
        "the first save comes back as itself: {heard:?}"
    );
    assert_eq!(
        heard.get(6),
        Some(&IpcMessage::Layout(LayoutState::Package(vec![7, 7, 7]))),
        "the refused save comes back as the OLD package: {heard:?}"
    );
    assert_eq!(heard.len(), 7, "and nothing else is said about it: {heard:?}");
}

#[test]
fn a_word_the_dispatch_does_not_know_is_refused_without_a_word() {
    // ⛔ THE THREE SILENT ROADS OF THE DISPATCH, WHICH NO OTHER PROBE HERE WALKS (R3-13 of the plan
    // review, 2026-09-15): an argument that names no policy, a function nobody registered, and
    // bytes that are no frame at all. Each is refused WITHOUT A WORD AND WITHOUT A RECORD --
    // untrusted content informs, it never authorises (ADR-0014) -- and the client is KEPT: a bad
    // frame is not a dead peer. Without this probe a `policy_named` that fell back to
    // `Some(remote)` for any unknown text would pass every other probe in this file.
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(
        GUI,
        &[
            IpcMessage::Hello(build_stamp()),
            IpcMessage::Invoke(Call {
                function: String::from(POLICY_FUNCTION.name),
                argument: String::from("gpu"),
            }),
            IpcMessage::Invoke(Call {
                function: String::from("frobnicate"),
                argument: String::from("local"),
            }),
        ],
    );
    // Three bytes that are no frame at all, pushed past `arrives` because `arrives` frames.
    bench.wire.borrow_mut().up.push((GUI, vec![0xff, 0xff, 0xff]));

    bench.round(
        |_| {},
        |core| {
            assert_eq!(
                core.arbiter().policy().name(),
                "remote",
                "a word the dispatch does not know must not move the policy"
            );
            assert!(
                core.journal().replay().expect("the memory journal replays").is_empty(),
                "and none of the three writes a record"
            );
            assert!(
                core.attending().contains(&GUI),
                "and the client that said them is still at the table"
            );
        },
    );

    let heard = bench.heard(GUI);
    assert_eq!(
        heard.len(),
        5,
        "the welcome, and not one word about any of the three: {heard:?}"
    );
}

"""

# ------------------------------------------------------------------ task 6: the eol label R10-16 names
t = scoped(t, "\n## Compito 6:", "\n## Compito 7:", [
    ("- Modify: `crates/kernel/tests/frozen/record_v1.map` — la sezione nuova, **tipata a mano**",
     "- Modify: `crates/kernel/tests/frozen/record_v1.map` (**`i/lf w/crlf`**) — la sezione nuova, **tipata a mano**", 1),
])

# ------------------------------------------------------------------ task 7
t = scoped(t, "\n## Compito 7:", "\n## Compito 8:", [
    # Files (R10-16, R3-4, R3-5, R3-17, R9a-10, R3-18)
    ("- Modify: `crates/kernel/src/lib.rs` — la riga `pub mod serving;`\n"
     "- Modify: `crates/kernel/src/parameters.rs` — il campo `gui_tick`\n"
     "- Modify: `crates/kernel/src/executor.rs` — `nap`, la sospensione pubblica\n"
     "- Modify: `crates/kernel/src/registry.rs` — **dal compito 6**, `Approval` e `Registry::held` (**P-43**)\n"
     "- Modify: `crates/kernel/src/wire/ipc.rs` — **dal compito 3**, `allocated` e `done` (**P-38**, **P-39**)\n"
     "- Modify: i **ventitré** file che chiamano `Parameters::new`, i nove `.stderr` che si muovono compresi\n"
     "- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**LF** — **P-47**) — **tre** richiami datati nella §5\n",
     "- Modify: `crates/kernel/src/lib.rs` (**`i/lf w/crlf`**) — la riga `pub mod serving;`\n"
     "- Modify: `crates/kernel/src/parameters.rs` (**`i/lf w/crlf`**) — il campo `gui_tick`\n"
     "- Modify: `crates/kernel/src/executor.rs` (**`i/lf w/crlf`**) — `nap`, la sospensione pubblica\n"
     "- ⛔ **Né `crates/kernel/src/registry.rs` né `crates/kernel/src/wire/ipc.rs`** — qui stavano due «Modify» (*«dal compito 6, `Approval` e `Registry::held`»*, *«dal compito 3, `allocated` e `done`»*), e il testo dei compiti 3 e 6 li porta **già**: il 7 non li tocca, e il Passo 4 è il verbale (R3-4, 2026-09-15)\n"
     "- Modify: i **ventuno** file con le **cinquanta** chiamate a `Parameters::new` — delle cinquantasette righe del censimento, **sette** sono commenti in sei file e non si toccano (R3-5, misurato il 2026-09-15) — e il **solo** `.stderr` che si muove, `crates/kernel/tests/compile_fail/parameters_have_no_default.stderr`\n"
     "- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**LF** — **P-47**) — **cinque** richiami datati: tre celle della §5, la riga `Request, Verdict` della §4 (D5, **D88**) e il dedotto della §7 sulla protezione (D22, **D88**)\n"
     "- Modify: `docs/superpowers/specs/2026-09-07-direzione-gui-design.md` (**LF**) — il richiamo datato sulla riga 2 della tabella *Stato* (D20) e sul 🔶 dedotto in fondo alle tre sequenze di *«La GUI dentro»* (R3-12, R3-17)\n", 1),
    # Interfaces (R3-4)
    ("- Consumes, dal **compito 6**: `kernel::registry::{Registry, Function, Invoker, InvokeError}`, e i due pezzi che questo compito gli **aggiunge** al passo 4",
     "- Consumes, dal **compito 6**: `kernel::registry::{Registry, Function, Invoker, InvokeError, Approval}` e `Registry::held` — i due pezzi che il pre-controllo di questo compito ha fatto scrivere **nel 6** (P-43): il Passo 4 ne è il verbale, non li aggiunge (R3-4)", 1),
    # Passo 1 (R3-1, R3-2, R3-3, R3-4, R3-5, R3-17)
    ("grep -rnE '\\b(fn|struct|enum|mod) (dispatch|session|serve|serving|listen)' crates/ --include='*.rs'",
     "grep -rnE '\\b(fn|struct|enum|mod) (dispatch|session|serve|serving|listen)\\b' crates/ --include='*.rs'", 1),
    ("grep -rc 'Parameters::new' crates/ --include='*.rs' | grep -v ':0'\n",
     "grep -rc 'Parameters::new' crates/ --include='*.rs' | grep -v ':0'\ngrep -rn 'Parameters::new' crates/ --include='*.rs' | grep -cE ':[0-9]+:\\s*//'\ngrep -l 'Parameters::new' crates/kernel/tests/compile_fail/*.stderr\n", 1),
    ("grep -n 'pub fn policy\\|pub fn allocated\\|pub fn set_policy' crates/kernel/src/arbiter/mod.rs",
     "grep -n 'pub \\(const \\)\\?fn policy\\|pub fn allocated\\|pub fn set_policy' crates/kernel/src/arbiter/mod.rs", 1),
    ("git ls-files --eol crates/kernel/src/lib.rs crates/kernel/src/parameters.rs crates/kernel/src/executor.rs crates/kernel/src/registry.rs crates/kernel/src/wire/ipc.rs docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md",
     "git ls-files --eol crates/kernel/src/lib.rs crates/kernel/src/parameters.rs crates/kernel/src/executor.rs docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md docs/superpowers/specs/2026-09-07-direzione-gui-design.md", 1),
    ("Atteso: i due file **non esistono**; l'unico `dispatch` è `pub fn dispatch` di `gateway/mod.rs` (**P-36**);\n"
     "i siti di `Parameters::new` sono **cinquantasette** in **ventitré** file e **nove** di quei file stanno in\n"
     "`compile_fail/` (**P-37**); `Parameters::new` prende **tre** argomenti; in `executor.rs` **nessun** `async fn`\n"
     "e nessun `Yield`; `policy()` rende `&VramPolicy`, `allocated()` rende `Mib`, `set_policy` prende\n"
     "`(policy, step, journal)`; `registry.rs` e gli altri `i/lf w/crlf`, tranne i due file nuovi che nascono **LF** e ⚠️ **il disegno del 2,\n"
     "che è `i/lf w/lf`** (richiamo del 2026-09-14, **P-47**).",
     "Atteso: i due file **non esistono**; l'unico `dispatch` è `pub fn dispatch` di `gateway/mod.rs` (**P-36**) — ⚠️ con\n"
     "l'alternanza **chiusa da `\\b`**: senza, il comando prendeva anche tre aiutanti di banco `dispatch_*` (R3-1);\n"
     "le righe con `Parameters::new` sono **cinquantasette** in **ventitré** file, di cui **sette** righe di commento in\n"
     "sei file — le **chiamate** sono cinquanta in ventuno file — e **nove** di quei file stanno in `compile_fail/`, **due**\n"
     "dei quali lo nominano solo in commento; il solo `.stderr` che lo cita è `parameters_have_no_default.stderr` (**P-37**,\n"
     "R3-5); `Parameters::new` prende **tre** argomenti; in `executor.rs` **nessun** `async fn` e nessun `Yield`;\n"
     "`policy()` rende `&VramPolicy` — ⚠️ è `pub const fn`, e il pattern senza `\\(const \\)\\?` non la mostrava (R3-2) —\n"
     "`allocated()` rende `Mib`, `set_policy` prende `(policy, step, journal)`; `lib.rs`, `parameters.rs` ed `executor.rs`\n"
     "`i/lf w/crlf`, i **due disegni** `i/lf w/lf` (richiamo del 2026-09-14, **P-47**), e i due file nuovi nascono **LF**\n"
     "(⚠️ qui stava «`registry.rs` e gli altri `i/lf w/crlf`»: il 7 non tocca `registry.rs`, che il 6 crea **LF** — R3-3,\n"
     "R3-4).", 1),
    # Passo 2 (R3-5, D76)
    ("- [ ] **Passo 2: il tick nei parametri, e i cinquantasette siti**",
     "- [ ] **Passo 2: il tick nei parametri, e le cinquanta chiamate**", 1),
    ("(D23 of the milestone 2\n    /// part 2 plan)", "(D23 of the sub-project 2\n    /// part 2 plan)", 1),
    ("⛔ **Poi i siti, e sono cinquantasette in ventitré file.** Ognuno guadagna un quarto argomento.",
     "⛔ **Poi le chiamate: cinquanta, in ventuno file** — le altre sette righe del censimento sono commenti, in sei file,\ne non si toccano (R3-5). Ognuna guadagna un quarto argomento.", 1),
    # Passo 3 (R3-5)
    ("Per ciascuno dei nove: si aggiunge il quarto argomento **sulla stessa riga** se la chiamata sta su una riga,\n"
     "e si **rimisura** se il `.stderr` è cambiato.",
     "Dei nove, **sette** portano una chiamata e **due** — `parameters_have_no_default.rs` e `trust_has_no_default.rs` —\n"
     "nominano `Parameters::new` **solo in commento**, e non si toccano. Le sette chiamate stanno **su una riga**, e i file lo\n"
     "dichiarano (*«THE `Parameters::new` CALL ABOVE RUNS LONG ON PURPOSE»*): il quarto argomento entra sulla stessa riga, e\n"
     "**nessun numero di riga si muove** per esse (R3-5). Poi si **rimisura** se un `.stderr` è cambiato.", 1),
    ("Poi, **una volta sola dopo aver toccato tutti e nove**:", "Poi, **una volta sola dopo aver toccato tutte e sette**:", 1),
    ("| `git diff` nomina un `.stderr` e l'errore atteso è **sparito** | ⛔ **ROSSO VERO, e il peggiore:** il caso ha smesso di provare ciò per cui esiste. È una voce d'errata, non un aggiornamento |\n",
     "| `git diff` nomina un `.stderr` e l'errore atteso è **sparito** | ⛔ **ROSSO VERO, e il peggiore:** il caso ha smesso di provare ciò per cui esiste. È una voce d'errata, non un aggiornamento |\n"
     "| `git diff` nomina **`parameters_have_no_default.stderr`**, e il diff è la **firma** di `new` che rustc cita | ✅ **ATTESO, e non è nessuna delle quattro righe sopra** (R3-5): la nota di rustc cita la firma di `Parameters::new` **verbatim**, sotto `--> src/parameters.rs` e senza numero di riga, e il Passo 2 la porta su cinque righe. Il file `.rs` lo predice — *«THE ORACLE NEXT DOOR WILL GO `mismatch` THE DAY A SECOND PARAMETER IS ADDED»*. Si rigenera **per la via che quel file detta**: si cancella lo `.stderr` stantio, si rilancia, `diff -u` del vecchio contro quello in `wip/`, si sposta a mano — **mai** `TRYBUILD=overwrite` (gotcha #25); il nuovo `.stderr` si **legge**, e il commit lo nomina |\n", 1),
    # Passo 4 (R3-4)
    ("⛔ **Vanno NEI compiti, non nell'errata** — i compiti 3 e 6 sono **scritti e non eseguiti**, ed è la terza\n"
     "volta che questo piano lo fa (decisione 15 della terza chiusura). Chi esegue trova i compiti già corretti;\n"
     "questo passo esiste perché il **commit** lo dichiari.",
     "⛔ **VERBALE, NON LAVORO: qui non si tocca nulla** (R3-4, 2026-09-15). Le correzioni sono **nei compiti**, non\n"
     "nell'errata — i compiti 3 e 6 sono **scritti e non eseguiti**, ed è la terza volta che questo piano lo fa (decisione 15\n"
     "della terza chiusura). Chi esegue trova i compiti già corretti e **non riapre** `wire/ipc.rs` né `registry.rs`; questo\n"
     "passo esiste perché il **commit** lo dichiari. ⚠️ Nessun *Trova* qui: il testo sotto è ciò che i compiti 3 e 6 **già\n"
     "dettano**, riportato perché si legga in un posto solo.", 1),
    ("**Nel compito 3** — `crates/kernel/src/wire/ipc.rs`:", "**Nel compito 3** — `crates/kernel/src/wire/ipc.rs`, che il 3 detta già così:", 1),
    ("I due doc che li accompagnano si riscrivono così:", "I due doc che li accompagnano, come il 3 li porta:", 1),
    ("⚠️ **E il `stamp_set()` del compito 3 va riletto**, perché costruisce un `PolicyReport` e uno `StepSummary`\n"
     "coi nomi vecchi: i due letterali prendono i nomi nuovi, e ⛔ **il valore di `done` resta quello che era**, o\n"
     "i byte del timbro cambierebbero per un motivo che non è un cambio di schema.",
     "⚠️ **E il `stamp_set()` del compito 3 porta GIÀ i nomi nuovi** — misurato alla revisione del piano intero: zero\n"
     "`allocatable` e zero `outcome: Option<bool>` nel testo del 3 (R3-4). ⛔ **Il valore di `done` è quello che era**, o i\n"
     "byte del timbro cambierebbero per un motivo che non è un cambio di schema.", 1),
    ("**Nel compito 6** — `crates/kernel/src/registry.rs`, ed è la cura di **P-43**. Sopra `impl Registry`:",
     "**Nel compito 6** — `crates/kernel/src/registry.rs`, ed è la cura di **P-43** — il 6 detta già, sopra `impl Registry`:", 1),
    ("`invoke` guadagna l'argomento, e le due righe che cambiano nel corpo:\n\n"
     "*Trova* le righe da `        if !permission::is_granted(journal, &function.permission)` fino a `        }`, **intere, prese dal file**, e *Sostituisci con*:",
     "`invoke` ha l'argomento, e nel corpo il controllo dice (⚠️ qui stava un *Trova* su `if !permission::is_granted(…)`, una\n"
     "riga che il file del 6 non ha mai avuto — R3-4):", 1),
    ("e, **subito dopo** la `.note(step, &noted(function, invoker, argument))` col suo `?`:",
     "e, subito dopo la `.note(step, &noted(function, invoker, argument))` col suo `?`, la concessione:", 1),
    ("lo dice. E il registro guadagna il suo **unico** accessore di lettura, che ha un chiamante da oggi:",
     "lo dice. E il registro ha il suo **unico** accessore di lettura, che ha un chiamante da oggi:", 1),
    # Passo 5 (D76)
    ("bench since milestone 2 has been carrying", "bench since milestone 2 of sub-project 1 has been carrying", 1),
    # Passo 6 (R3-8, R3-10, R3-11, R9a-9, R3-13, R3-14, D76)
    ("must do at task 11 (§7 of the", "must do at task 12 (§7 of the", 1),
    ("//! long left. ⛔ THIS IS THE THIRD COPY OF THAT WRAPPER IN THE REPOSITORY -- declared rather than\n"
     "//! discovered (gotcha #49). Where it should live is REGISTERED AND NOT TAKEN: the fourth caller\n"
     "//! is task 9's campaign, and moving it into `simulator` is that task's call, with its measure.",
     "//! long left. ⛔ THIS IS THE SECOND COPY OF THAT WRAPPER IN THE REPOSITORY -- declared rather than\n"
     "//! discovered (gotcha #49): the third and fourth arrive with tasks 9 and 10 (D29, D34), the fifth\n"
     "//! with task 12 (P-74). Where it should live is DECIDED, by D34: it stays local, because the\n"
     "//! daemon refuses to depend on `simulator` and a common home would serve three callers out of\n"
     "//! four (P-59).", 1),
    ("use kernel::ports::custody::{Custody, CustodyKey};", "use kernel::ports::custody::{Custody, CustodyError, CustodyKey};", 1),
    ("type BenchCore<'b> = Core<FakeIpc<'b>, MemoryJournal, MemoryCustody>;", BENCH_CUSTODY, 1),
    ("            MemoryCustody::new(),\n", "            BenchCustody::new(),\n", 1),
    ("    // `ClientGrants::on_disconnect`'s own words -- \"a gui may die before it ever asked, it may die\n"
     "    // before it was ever accepted\".",
     "    // `ClientGrants::on_disconnect`'s own words -- \"A gui may die before it ever asked — it may die\n"
     "    // before it was ever accepted —\".", 1),
    ("            // of milestone 2 issues one, because `Request` is not served.",
     "            // of sub-project 2 issues one, because `Request` is not served.", 1),
    ("    // so a failed write comes back as the OLD package and the gui sees it by comparing.\n",
     "    // so a failed write comes back as the OLD package and the gui sees it by comparing. The\n"
     "    // failed write is `save_layout_that_the_port_refuses_comes_back_with_the_old_package`.\n", 1),
    ("#[test]\nfn an_unchanged_degradation_is_not_resent() {", NEW_PROBES + "#[test]\nfn an_unchanged_degradation_is_not_resent() {", 1),
    ("sonda sarebbe la stessa corsa una seconda volta** (gotcha #49), e il modo in cui questa la tiene è scritto\n"
     "qui perché non venga «aggiunto» domani.",
     "sonda sarebbe la stessa corsa una seconda volta** (gotcha #49), e il modo in cui questa la tiene è scritto\n"
     "qui perché non venga «aggiunto» domani. ⚠️ **E il terzo stato, `LayoutState::Unavailable`** — il braccio\n"
     "`Err(CustodyError::Unavailable)` di `layout()` — **non ha una sonda qui**: `BenchCustody` rifiuta `keep`, non\n"
     "`retrieve`. Lo esercita il compito **9** con `MaybeCustody`, e la promessa della porta su una scrittura rifiutata —\n"
     "che cosa resta custodito — è della suite del compito **5** (R3-14).", 1),
    # Passo 7 (R3-9, R3-7, D76)
    ("//! writes out why -- the DST moves THIS one with `simulator::ipc::DyingGui`, and `gui/fake-core`\n"
     "//! runs THIS one on in-memory ports instead of writing a second dispatch of its own (§7). A second",
     "//! writes out why -- the DST of task 10 drives THIS one over a wire of its own, held outside the\n"
     "//! core behind a `RefCell` (D32), and `gui/fake-core` runs THIS one on in-memory ports instead of\n"
     "//! writing a second dispatch of its own (§7). A second", 1),
    ("In production the turn limit is `u64::MAX` (task 8); under a finite limit",
     "In production the turn limit is `u64::MAX` (task 9); under a finite limit", 1),
    ("/// The ONE function the registry holds in milestone 2: the VRAM policy change.",
     "/// The ONE function the registry holds in sub-project 2: the VRAM policy change.", 1),
    ("with the one function of milestone 2: D16.", "with the one function of sub-project 2: D16.", 1),
    ("the FUNCTIONS of milestone\n    /// 2 do not.", "the FUNCTIONS of sub-project\n    /// 2 do not.", 1),
    ("stated rather than hidden: milestone 2\n    /// issues no grant", "stated rather than hidden: sub-project 2\n    /// issues no grant", 1),
    ("/// One line of the step list: in milestone 2 these are registry invocations.",
     "/// One line of the step list: in sub-project 2 these are registry invocations.", 1),
    ("    /// The step list: in milestone 2 these are the registry's invocations.",
     "    /// The step list: in sub-project 2 these are the registry's invocations.", 1),
    ("milestone 2 part 2 plan", "sub-project 2 part 2 plan", 2),
    ("milestone 2 design", "sub-project 2 design", 5),
    # Passo 8 (R3-12, R9a-10, R3-18, R3-6, D75 on the D21 cell)
    ("⛔ **Quattro case, e si toccano nello stesso commit** (quinta riga della disciplina dell'audit). Nella §5 del\n"
     "[disegno del 2](../specs/2026-09-06-sottoprogetto-2-gui-minima-design.md) — ⚠️ **`i/lf w/lf`**, non\n"
     "`w/crlf` (richiamo del 2026-09-14, **P-47**) — tre celle della tabella «Il daemon che ascolta»:",
     "⛔ **Due file, e tutti i richiami si toccano nello stesso commit** (quinta riga della disciplina dell'audit). Nel\n"
     "[disegno del 2](../specs/2026-09-06-sottoprogetto-2-gui-minima-design.md) — ⚠️ **`i/lf w/lf`**, non\n"
     "`w/crlf` (richiamo del 2026-09-14, **P-47**) — tre celle della tabella «Il daemon che ascolta» della §5, poi la riga\n"
     "`Request, Verdict` della §4 e il dedotto della §7 (**D88**: i richiami che una `D` rende dovuti li scrive il compito che\n"
     "la esegue):", 1),
    ("il giorno che `Protection` guadagna una seconda variante, il valore diventa consegnato |",
     "il giorno che `Protection` guadagna una seconda variante, il valore diventa consegnato. ⚠️ **E il client rifiutato esce SUBITO dalla tabella** (R3-12): il core lo dimentica nello stesso giro in cui gli manda `StaleBuild`, quindi il suo `Disconnected` non arriva a nessuno — e non serve, perché non tiene nulla. La sequenza 1 dice *«non lo ascolta più»*, e questo è il come |", 1),
    ("| il **limite di giri** | ⛔ **RICHIAMO DEL \\<data\\> (D21):**", "| il **limite di giri** | ⛔ **RICHIAMO DEL \\<data\\>, compito 7 (D21):**", 1),
    ("cioè il cablaggio. È il compito **9** |\n",
     "cioè il cablaggio. È il compito **9** |\n"
     "| `Request`, `Verdict` — la riga della **§4**; l'ancora è la riga intera che comincia con `\\| \\`Request\\`, \\`Verdict\\` \\|` | ⛔ **RICHIAMO DEL \\<data\\>, compito 7 (D5):** *«il daemon risponde»* è falso nel 2 — il ramo `Request` **non è servito** e nessun `Verdict` parte; la risposta *«strada»* del proprietario più sotto si lascia com'è, perché è una risposta e non una riga di disegno (R9a-10) |\n"
     "| il dedotto della **§7**; l'ancora è la riga intera che contiene `sia consegnato al finto come al daemon` | ⛔ **RICHIAMO DEL \\<data\\>, compito 7 (D22):** `Protection` ha una variante sola e **non è consegnata** a nessuno dei due, né al daemon né al finto: il valore lo manda l'attività, con l'innesco scritto accanto (R3-18) |\n", 1),
    ("> l'utente guarda. Se il proprietario vorrà il denominatore netto, i due addendi diventano parametri\n"
     "> consegnati.\n\n"
     "⚠️ **`\\<data\\>` è la data del giorno",
     "> l'utente guarda. Se il proprietario vorrà il denominatore netto, i due addendi diventano parametri\n"
     "> consegnati.\n\n"
     "E il 🔶 dedotto in fondo alle tre sequenze di *«La GUI dentro»* della stessa stella polare — l'ancora è la riga intera\n"
     "che contiene `e che il client rifiutato resti nella tabella fino al \\`Disconnected\\``, e il richiamo si appende in coda a\n"
     "quella riga:\n\n"
     "> ⛔ **RICHIAMO DEL \\<data\\>, compito 7 (R3-12):** il client rifiutato **non** resta nella tabella: il core lo dimentica\n"
     "> nello stesso giro in cui gli manda `StaleBuild`, il suo `Disconnected` non arriva a nessuno, e non serve — non tiene\n"
     "> nulla.\n\n"
     "⚠️ **`\\<data\\>` è la data del giorno", 1),
    ("le righe **7** e **8** portano il richiamo\ndel 2026-09-11, e chi esegue non deve toccarle se non per la spunta finale.",
     "le righe **7** e **9** portano il richiamo\ndel 2026-09-11 (⚠️ qui stava «7 e 8», il numero di prima di D25 — R3-6), e chi esegue non deve toccarle se non per la\nspunta finale.", 1),
    # Passo 9 (criteria for the recalls)
    ("- [ ] `bash scripts/check-docs.sh` → `OK`\n",
     "- [ ] `bash scripts/check-docs.sh` → `OK`\n"
     "- [ ] i richiami: `grep -c 'RICHIAMO DEL <data>, compito 7' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` → **5** e sulla stella polare → **2** (con la data scritta — D75), e `grep -c '<data>'` sui due disegni → **0**\n", 1),
])

tmp = PLAN + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(t)
os.replace(tmp, PLAN)
print("ok: task 7 patched (plus the record_v1.map label in task 6);", t.count("\n") - raw.count("\n"), "lines added")

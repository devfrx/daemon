"""patch_c89.py -- task 8 (D77: E66, E112, E50 with their plan's name) and task 9 (R4-1..R4-18, R3-15, R5-3,
R9a-10, R10-16, D75, D76, D84). Scoped, asserted, atomic."""
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


# ------------------------------------------------------------------ task 8 (D77)
t = scoped(t, "\n## Compito 8:", "\n## Compito 9:", [
    ("(lezione `E66`, `E112`).", "(lezione `E66` ed `E112` del piano del Traguardo 6 — D77).", 1),
    ("È un **oracolo indipendente** che controlla `reconcile`, e il suo errata `E50` scrive perché:",
     "È un **oracolo indipendente** che controlla `reconcile`, e la voce `E50` del piano del Traguardo 6 scrive perché:", 1),
])

# ------------------------------------------------------------------ task 9
BENCH_CONSTANTS = r"""
    /// How many messages the core sends after a valid `Hello` -- the welcome of sequence 1.
    ///
    /// ⛔ MEASURED ON THE DISPATCH OF TASK 7, `greet` in `crates/kernel/src/serving.rs`:
    /// `Accepted`, `Degradation`, `Policy`, `Layout`, `Steps`. The bench of that task pins the same
    /// number in `the_welcome_is_the_five_messages_of_sequence_one`, so the day the welcome grows,
    /// that probe goes red before this constant does.
    const WELCOME: usize = 5;

    /// The turn budget of every probe that has a PEER. ⛔ IT IS WALL CLOCK IN DISGUISE (R4-6): the
    /// listener lives only inside `run_the_graph`, and the peer connects from a thread the OS
    /// schedules when it likes, so a short run can end before the peer ever knocks. A hundred
    /// thousand turns at a zero tick is a fraction of a second of polling, and it is the same
    /// number the turn probe delivers. Probes WITHOUT a peer keep their own small budgets.
    const WITH_A_PEER: u64 = 100_001;
"""

FOUR_BODIES = r"""    #[test]
    fn a_layout_archive_that_will_not_open_lets_the_core_start() {
        let dir = private_dir_for_line(line!());
        let name = socket_name_for_line(line!());
        // A directory that is not there: `FileCustody::open` fails, and `MaybeCustody` swallows it.
        let broken = dir.join("not-there").join("layout.redb");

        let peer = a_peer_that_says(name.clone(), vec![IpcMessage::Hello(build_stamp())], WELCOME);
        let outcome = run_the_graph(
            Parameters::new(WITH_A_PEER, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &dir.join("journal.redb"),
            &broken,
            &name,
        );

        match outcome {
            Err(StartupError::Run(RunError::TurnLimitReached)) => {}
            other => panic!("a layout archive that will not open must not stop the start-up: {other:?}"),
        }
        let heard = peer.join().expect("the peer thread does not panic");
        assert!(
            heard.contains(&IpcMessage::Layout(LayoutState::Unavailable)),
            "the welcome must say the layout is UNAVAILABLE, not `Nothing`: {heard:?}"
        );
    }

    /// ⛔ THE OTHER DIRECTION OF `MaybeCustody`, and without it a wrapper that ALWAYS refused would
    /// pass the probe above: the archive that opens must really delegate. ✅ Held by the restart
    /// probe of step 11, which is why this one asserts the REFUSING half only -- said here rather
    /// than left for a reviewer to wonder about.
    #[test]
    fn a_core_started_on_a_broken_archive_answers_unavailable_to_every_save() {
        let dir = private_dir_for_line(line!());
        let name = socket_name_for_line(line!());
        let broken = dir.join("not-there").join("layout.redb");

        let peer = a_peer_that_says(
            name.clone(),
            vec![
                IpcMessage::Hello(build_stamp()),
                IpcMessage::SaveLayout(b"{\"grid\":1}".to_vec()),
            ],
            WELCOME + 1,
        );
        let _ = run_the_graph(
            Parameters::new(WITH_A_PEER, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &dir.join("journal.redb"),
            &broken,
            &name,
        );

        let heard = peer.join().expect("the peer thread does not panic");
        assert_eq!(
            heard.get(WELCOME),
            Some(&IpcMessage::Layout(LayoutState::Unavailable)),
            "the answer to a save on a broken archive is UNAVAILABLE, never a package: {heard:?}"
        );
        assert!(
            !heard
                .iter()
                .any(|said| matches!(said, IpcMessage::Layout(LayoutState::Package(_)))),
            "and no package ever comes back: {heard:?}"
        );
    }

    /// ⛔ THE `unwrap_or` OF D27, MEASURED IN BOTH DIRECTIONS, and the two are different claims.
    /// Empty journal → the DEFAULT of ADR-0006, which is remote and lives HERE as a literal; a
    /// journal carrying a transition → what the transition says, not the default. A `policy_now`
    /// that answered `Remote` on an empty archive would make the two indistinguishable, which is
    /// exactly what D27 refused inside the kernel.
    ///
    /// ⚠️ THE TRANSITION IS WRITTEN THROUGH `Arbiter::set_policy` ON A REAL `FileJournal`, not by
    /// hand: a hand-built record would be this probe agreeing with itself about a format, and the
    /// pair `set_policy`/`policy_now` is one artefact.
    #[test]
    fn the_policy_in_the_journal_is_the_one_the_arbiter_starts_on() {
        let dir = private_dir_for_line(line!());
        let journal = dir.join("journal.redb");
        let layout = dir.join("layout.redb");

        // ⛔ THE FIRST DIRECTION: an EMPTY journal, and the welcome names the default of ADR-0006.
        let first = socket_name_for_line(line!());
        let peer = a_peer_that_says(first.clone(), vec![IpcMessage::Hello(build_stamp())], WELCOME);
        let _ = run_the_graph(
            Parameters::new(WITH_A_PEER, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &journal,
            &layout,
            &first,
        );
        let heard = peer.join().expect("the peer thread does not panic");
        assert!(
            heard.iter().any(|said| matches!(
                said,
                IpcMessage::Policy(report) if report.policy == PolicyName::Remote
            )),
            "an empty journal starts on the default, which is remote: {heard:?}"
        );

        // ⛔ THE SECOND DIRECTION: a transition WRITTEN THROUGH `set_policy` on the real archive,
        // between the two runs, and the next start names it. The arbiter here is a bare one: what
        // is under test is the pair `set_policy`/`policy_now` on the archive, not the two quotas.
        {
            let mut archive = FileJournal::open(&journal).expect("the journal opens between runs");
            let mut arbiter = Arbiter::new(
                Parameters::new(WITH_A_PEER, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
                VramPolicy::Remote(RemotePolicy),
            );
            arbiter
                .set_policy(VramPolicy::Local(LocalPolicy), StepId::new(1_000), &mut archive)
                .expect("the transition is written");
        }

        let second = socket_name_for_line(line!());
        let peer = a_peer_that_says(second.clone(), vec![IpcMessage::Hello(build_stamp())], WELCOME);
        let _ = run_the_graph(
            Parameters::new(WITH_A_PEER, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &journal,
            &layout,
            &second,
        );
        let heard = peer.join().expect("the peer thread does not panic");
        assert!(
            heard.iter().any(|said| matches!(
                said,
                IpcMessage::Policy(report) if report.policy == PolicyName::Local
            )),
            "a restarted core starts on the policy the journal holds, not on the default: {heard:?}"
        );
    }

    /// ⛔ THE SECOND CORE. `StartupError::Ipc` has exactly one ordinary cause, and a variant with no
    /// probe is a claim nobody checks: two graphs on one socket name, and the second must stop
    /// instead of starting beside the first.
    ///
    /// ⚠️ NO THREAD AND NO RACE (R4-8): the first core is reduced to the one thing that matters, a
    /// listener bound to the name and held for the whole probe. On Windows `interprocess` creates
    /// the first instance with `FILE_FLAG_FIRST_PIPE_INSTANCE`, so a second `bound` on the name is
    /// refused; on Linux the second bind is `EADDRINUSE`. Read in the crate's source, not assumed.
    #[test]
    fn a_second_core_on_the_same_channel_stops_the_start_up() {
        let dir = private_dir_for_line(line!());
        let name = socket_name_for_line(line!());
        let _first = LocalSocketIpc::bound(&name, Progressive::starting_at(0), MAX_BODY)
            .expect("the first listener binds");

        let outcome = run_the_graph(
            Parameters::new(8, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &dir.join("journal.redb"),
            &dir.join("layout.redb"),
            &name,
        );

        match outcome {
            Err(StartupError::Ipc(_)) => {}
            other => panic!("a second core on the same channel must stop at the bind: {other:?}"),
        }
    }
"""

CPU_STEP = r"""- [ ] **Passo 14: il processore a riposo, MISURATO — decisione 41 del proprietario (D84)**

⛔ **È una promessa al proprietario che nessun compito manteneva** (R9a-5, R9b-7): *«il piano misura il processore a
riposo senza soglia»*. Si misura **qui**, col grafo di produzione — il tick vero di `GUI_TICK`, nessun client — per
sessanta secondi, e il numero **non ha soglia**: si registra, non si giudica. ⚠️ **Il binario scrive `journal.redb` e
`layout.redb` nella cartella corrente**, e nessuna riga di `.gitignore` li copre: si lancia da una cartella **fuori dal
repository**, mai dalla radice. Su Windows, che è dove il proprietario lavora (ADR-0002), da PowerShell nella radice:

```powershell
cargo build --locked -p daemon
$where = New-Item -ItemType Directory -Force "$env:TEMP\harness-cpu-at-rest"
$p = Start-Process -PassThru -WorkingDirectory $where -FilePath (Resolve-Path .\target\debug\daemon.exe)
Start-Sleep -Seconds 3
$a = $p.TotalProcessorTime.TotalSeconds; Start-Sleep -Seconds 60; $p.Refresh(); $b = $p.TotalProcessorTime.TotalSeconds
'{0:N2} % of one core at rest over 60 s, GUI_TICK 16 ms, {1}' -f (($b - $a) / 60 * 100), (Get-Date -Format yyyy-MM-dd)
Stop-Process $p; Remove-Item -Recurse -Force $where
```

Atteso: **un numero**, senza soglia, che si scrive **nel messaggio di commit** di questo compito con la data e il
comando, e che il compito **17** porta in `riferimenti.md` (D84). ⚠️ Su Linux si legge `ps -o %cpu= -p <pid>`; il
numero che vale è quello preso sulla macchina del proprietario.

"""

# ---- Passo 14 -> Passo 7-bis (R4-3): move the block before Passo 8, BEFORE the substitutions below add a
# new "Passo 14" (the CPU measure) that would otherwise sit between the old Passo 14 and Passo 15.
lo = t.index("\n## Compito 9:"); hi = t.index("\n## Compito 10:", lo)
seg = t[lo:hi]
s14 = seg.index("\n- [ ] **Passo 14: `interprocess` in `[dev-dependencies]`, in DUE passi**")
e14 = seg.index("\n- [ ] **Passo 15:", s14)
block = seg[s14:e14]
seg = seg[:s14] + seg[e14:]
assert block.count("cargo build -p daemon --tests\ngit diff --stat Cargo.lock\nbash scripts/gate.sh\n") == 1
block = block.replace("- [ ] **Passo 14: `interprocess` in `[dev-dependencies]`, in DUE passi**",
                      "- [ ] **Passo 7-bis: `interprocess` in `[dev-dependencies]`, in DUE passi — PRIMA del pari (R4-3)**\n\n"
                      "⛔ **Era il Passo 14, dopo le sonde che lo usano: i `cargo test` dei Passi 9–12 erano rossi per `E0432 unresolved\n"
                      "import interprocess` (R4-3). Sta qui, prima del pari del Passo 8.**")
block = block.replace("cargo build -p daemon --tests\ngit diff --stat Cargo.lock\nbash scripts/gate.sh\n",
                      "cargo build -p daemon\ngit diff --stat Cargo.lock\ngit ls-files --eol Cargo.lock\n")
old_note = ("⚠️ **Il primo comando è SENZA `--locked`**, ed è l'unico di questo compito: è così che il lockfile si rinfresca\n"
            "(finding **G-5**). Se `Cargo.lock` non cambia — perché `interprocess` è già nel grafo da `platform` — va bene\n"
            "lo stesso: ciò che conta è che il cancello, che passa `--locked`, resti verde.")
assert block.count(old_note) == 1, "note of step 7-bis"
block = block.replace(old_note,
                      "⚠️ **Il primo comando è SENZA `--locked` e senza `--tests`**, ed è l'unico di questo compito senza `--locked`: è così\n"
                      "che il lockfile si rinfresca (finding **G-5**), e una dev-dependency entra nel lockfile anche senza compilare i banchi —\n"
                      "che qui non compilano ancora. Se `Cargo.lock` non cambia — perché `interprocess` è già nel grafo da `platform` — va\n"
                      "bene lo stesso: ciò che conta è che il cancello, che passa `--locked`, resti verde al Passo 16. ⛔ **Dopo cargo\n"
                      "`Cargo.lock` è `i/lf w/lf`**: cargo riscrive in LF (R4-16, misurato alla revisione del piano intero con `cargo\n"
                      "1.95.0`), l'indice resta `i/lf`, e questo è ciò che il vincolo 4 difende.")
p8 = seg.index("\n- [ ] **Passo 8: gli aiutanti del banco")
seg = seg[:p8] + block + seg[p8:]
t = t[:lo] + seg + t[hi:]

t = scoped(t, "\n## Compito 9:", "\n## Compito 10:", [
    # Files (R10-16, R4-16, R4-8, R4-9/D88)
    ("`build_the_arbiter` che riceve la policy, il cablaggio in `run_the_graph`, e il modulo `tests` in fondo",
     "`build_the_arbiter` che riceve la policy, il cablaggio in `run_the_graph`, e il modulo `tests` in fondo — le due costanti del banco e le quattro sonde del Passo 12 **dettate** (R4-8, 2026-09-15)", 1),
    ("- Modify: `Cargo.lock` — rinfrescato **fuori** dal cancello, con un `cargo build` **senza** `--locked`",
     "- Modify: `Cargo.lock` (**`i/lf w/crlf`** oggi; ⛔ **`i/lf w/lf` dopo cargo**, che lo riscrive in LF — R4-16, R10-16) — rinfrescato **fuori** dal cancello, con un `cargo build` **senza** `--locked`, al Passo **7-bis**", 1),
    ("(**LF** — **P-47**) — **due** richiami datati nella §5: la riga del limite di giri e la riga dello spegnimento",
     "(**LF** — **P-47**) — **tre** richiami datati: nella §5 la riga del limite di giri e la riga dello spegnimento, e nella §8 la riga *«il limite di giri e `Disconnected`»* (D5, R4-9, **D88**)", 1),
    # Interfaces
    ("- Consumes, dal **compito 3**: `kernel::wire::ipc::{IpcMessage, build_stamp, LayoutState}` — **nel banco**, per il pari che parla",
     "- Consumes, dal **compito 3**: `kernel::wire::ipc::{IpcMessage, build_stamp, LayoutState, PolicyName}` — **nel banco**, per il pari che parla e per le sonde del Passo 12", 1),
    ("- Consumes, da oggi: `kernel::arbiter::{Arbiter, VramPolicy, RemotePolicy}`; `kernel::executor::{Executor, RunError, Sleep}`; `kernel::ports::reactor::Reactor`;",
     "- Consumes, da oggi: `kernel::arbiter::{Arbiter, VramPolicy, RemotePolicy, LocalPolicy}`; `kernel::executor::{Executor, RunError, Sleep}`; `kernel::ports::reactor::Reactor`; `kernel::ports::journal::StepId` (nel banco, Passo 12);", 1),
    # Passo 1 (R4-16, R4-7)
    ("`main.rs` e `Cargo.toml` sono `i/lf w/crlf`, il disegno del 2 è `i/lf w/lf`.",
     "`main.rs`, `Cargo.toml` e `Cargo.lock` sono `i/lf w/crlf`, il disegno del 2 è `i/lf w/lf`.", 1),
    ("Si prende **dopo** il Passo 7, quando il cablaggio esiste, e si **scrive accanto alla sonda con la\n"
     "data**: lo stesso grafo con un limite di **un giro**, dove il pari **non** riceve la propria accoglienza mentre\n"
     "`run()` rende lo stesso valore.",
     "Si prende **dopo** il Passo 7, quando il cablaggio esiste, e si **scrive accanto alla sonda con la\n"
     "data**: lo stesso grafo con un limite di **zero** giri e **senza pari** — `run()` rende lo stesso\n"
     "`Err(TurnLimitReached)` senza un solo giro di servizio, perché l'esecutore rifiuta prima del primo `poll`\n"
     "(`turns = 1 > 0`, in `crates/kernel/src/executor.rs`), deterministico. ⚠️ Qui stava *«un giro, dove il pari non\n"
     "riceve la propria accoglienza»*: a un giro l'esito è una **gara** col thread del pari, che può essersi collegato e\n"
     "aver detto `Hello` prima del `poll` (R4-7); e a zero giri un pari non farebbe in tempo a collegarsi (R4-6), quindi la\n"
     "baseline si prende senza.", 1),
    # Passo 2 (D76, R4-11, R4-13)
    ("Decision A of §5 of the milestone 2 design, delegated and taken.",
     "Decision A of §5 of the sub-project 2 design, delegated and taken.", 1),
    ("and in the milestone 2 campaign. In production the", "and in the sub-project 2 campaign (task 10). In production the", 1),
    ("which §5 assigns to milestone 10 along", "which §5 assigns to sub-project 10 along", 1),
    ("/// ⛔ AND THE SHELL IS NOT A TASK OF THIS PLAN — the fake core binds a name of its own, the SPA never\n"
     "/// touches a socket, and §8 puts the end-to-end run in the shell \"outside today's gate\".",
     "/// ⛔ AND THE SHELL IS NOT A TASK OF THIS PLAN — the fake core binds THE SAME NAME, as a copy\n"
     "/// (`gui/fake-core/src/main.rs`, task 12, whose closing criterion compares the two literals — D45),\n"
     "/// the SPA never touches a socket, and §8 puts the end-to-end run in the shell \"outside today's gate\".", 1),
    ("which is entry 5 of §9 of the milestone 2\n/// design, confirmed", "which is entry 5 of §9 of the sub-project 2\n/// design, confirmed", 1),
    ("/// ⛔ ITS TRIGGER IS THE FIRST PERCEIVED-LATENCY MEASUREMENT on the assembled shell, milestone 2\n"
     "/// part 3: until somebody watches a round trip, any number here is an argument.",
     "/// ⛔ ITS TRIGGER IS THE FIRST PERCEIVED-LATENCY MEASUREMENT on the assembled shell, which this\n"
     "/// plan does not build (P-53): until somebody watches a round trip, any number here is an argument.", 1),
    # Passo 3 (R4-12, D76)
    ("/// hidden: `crates/simulator/tests/arbiter_campaign.rs` and the milestone 2 serving bench carry the\n"
     "/// other two, and neither can be imported — a `tests/` file is a crate of its own and a binary\n"
     "/// exports nothing. ⛔ WHETHER IT SHOULD RISE INTO `simulator` IS NOT THIS TASK'S CALL: milestone\n"
     "/// 10 is the first that can measure the need with a campaign in hand.",
     "/// hidden: `crates/simulator/tests/arbiter_campaign.rs` and the sub-project 2 serving bench (task 7)\n"
     "/// carry the other two, and neither can be imported — a `tests/` file is a crate of its own and a\n"
     "/// binary exports nothing. ⛔ WHETHER IT SHOULD RISE INTO `simulator` IS DECIDED, AND THE ANSWER IS\n"
     "/// NO: D34 keeps it local — this crate refuses to depend on `simulator` (its manifest says so), so a\n"
     "/// common home would serve three callers out of four (P-59). Task 10 of this plan,\n"
     "/// `crates/simulator/tests/serving_campaign.rs`, carries the fourth copy and says the same.", 1),
    # Passo 4 (D76)
    ("CHANNEL the daemon grows — milestone 10, with the clean shutdown —", "CHANNEL the daemon grows — sub-project 10, with the clean shutdown —", 1),
    # Passo 7 (R4-1, R4-5, R4-17, R4-9)
    ("        SOCKET_NAME,\n    )\n}\n```\n\n```rust\n/// ⛔ RECALL OF <data>, MILESTONE 2 TASK 9 — THE SOCKET NAME IS AN ARGUMENT TOO, FOR THE REASON\n",
     "        SOCKET_NAME,\n    )\n}\n```\n\n"
     "⛔ **E la chiamata di `main`, che nessun passo toccava (R4-1):** *Trova* la riga\n"
     "`    match run_the_production_graph(Path::new(JOURNAL_PATH)) {` **intera** (unica: `grep -c -F` → 1) e *Sostituisci con*:\n\n"
     "```rust\n    match run_the_production_graph(Path::new(JOURNAL_PATH), Path::new(LAYOUT_PATH)) {\n```\n\n"
     "⛔ **E il doc di `run_the_production_graph` riceve il proprio richiamo (R4-5, D28, P-48)** — senza, direbbe ancora\n"
     "*«SO THAT A TEST CAN CALL IT»* mentre il criterio di chiusura impone che **nessun** banco la chiami. *Trova* la riga\n"
     "`/// what to print and what to exit with, and nothing else.` **intera** (unica: `grep -c -F` → 1) e *Sostituisci con*:\n\n"
     "```rust\n"
     "/// what to print and what to exit with, and nothing else.\n"
     "///\n"
     "/// ⛔ RECALL OF <data>, SUB-PROJECT 2, TASK 9 — NO TEST CALLS IT ANY MORE, AND THE PARAGRAPH\n"
     "/// ABOVE IS DATED. It hands `u64::MAX`, so a probe calling it would HANG rather than fail (D28);\n"
     "/// every probe goes through `run_the_graph` with a limit of its own. What this function chooses —\n"
     "/// the four production literals and the socket name — is walked by nothing: declared, not\n"
     "/// covered. The sentence below about \"the test that already existed\" describes that day, not this.\n"
     "```\n\n"
     "```rust\n/// ⛔ RECALL OF <data>, MILESTONE 2 TASK 9 — THE SOCKET NAME IS AN ARGUMENT TOO, FOR THE REASON\n", 1),
    ("/// THE PATH ALREADY IS. The paragraph above says a fixed path in a shared directory is gotcha #52;",
     "/// THE PATH ALREADY IS. The doc of `run_the_production_graph` says a fixed path in a shared directory is gotcha #52;", 1),
    ("/// `line!()` and the process id, exactly as `private_dir_for_line` does (P-55).\nfn run_the_graph(",
     "/// `line!()` and the process id, exactly as `private_dir_for_line` does (P-55).\n"
     "///\n"
     "/// ⚠️ NO PROBE HERE WATCHES A CLIENT DIE: this binary exposes no `Core`, so the wiring of\n"
     "/// `ClientGrants::on_disconnect` is held by the bench of task 7 and by the campaign of task 10, on\n"
     "/// `Core::attending` (R4-9). In sub-project 2 no ordinary grant exists to give back (D5).\n"
     "fn run_the_graph(", 1),
    ("⛔ **Qui `cargo build` è ATTESO ROSSO una volta**, sul modulo `tests`, che ancora chiama le due funzioni con le\n"
     "vecchie firme: è il Passo 8.",
     "⛔ **Qui `cargo build` è ATTESO ROSSO una volta**, sul modulo `tests`, che ancora chiama le due funzioni con le\n"
     "vecchie firme (e su `main`, se il punto qui sopra è saltato — R4-1): è il Passo 8. ⚠️ Il modulo `tests` non entra in\n"
     "un `cargo build` senza `--tests`, quindi il Passo 7-bis rinfresca il lockfile prima che i banchi compilino.", 1),
    # Passo 8 (R4-6, R4-2/R5-3, WELCOME, WITH_A_PEER)
    ("        format!(\"harness-daemon-{}-{}\", std::process::id(), line)\n    }\n",
     "        format!(\"harness-daemon-{}-{}\", std::process::id(), line)\n    }\n" + BENCH_CONSTANTS, 1),
    ("    /// the listener exists from `bound()`, which happens before `run()`, but this thread may be\n"
     "    /// scheduled first.\n",
     "    /// the listener exists from `bound()`, which happens before `run()`, but this thread may be\n"
     "    /// scheduled first. ⛔ AND THE LOOP HAS A WALL-CLOCK DEADLINE (R4-6): the listener lives only\n"
     "    /// inside `run_the_graph`, so if the run ends before this thread connects, `connect` fails FOR\n"
     "    /// EVER and a bare loop would hang the gate -- the worst red there is. Five seconds is not a\n"
     "    /// tuning: it is an order of magnitude above any scheduling delay, and the panic names this line.\n", 1),
    ("            let ns = name.to_ns_name::<GenericNamespaced>().expect(\"a namespaced name\");\n"
     "            let mut stream = loop {\n"
     "                match Stream::connect(ns.clone()) {\n"
     "                    Ok(stream) => break stream,\n"
     "                    Err(_) => std::thread::yield_now(),\n"
     "                }\n"
     "            };\n",
     "            let ns = name.to_ns_name::<GenericNamespaced>().expect(\"a namespaced name\");\n"
     "            let started = std::time::Instant::now();\n"
     "            let mut stream = loop {\n"
     "                match Stream::connect(ns.clone()) {\n"
     "                    Ok(stream) => break stream,\n"
     "                    Err(error) => {\n"
     "                        assert!(\n"
     "                            started.elapsed() < std::time::Duration::from_secs(5),\n"
     "                            \"the peer could not connect within five seconds ({error:?}): the run \\\n"
     "                             ended before this thread got to the listener (R4-6)\"\n"
     "                        );\n"
     "                        std::thread::yield_now();\n"
     "                    }\n"
     "                }\n"
     "            };\n", 1),
    ("                // ⚠️ `take_frame` AND NOT `unframe`: the buffer ordinarily holds a frame and a half,\n"
     "                // which `unframe` refuses by design (P-13). It hands back the body and where the\n"
     "                // next one starts.\n"
     "                while let Some((body, next)) = framing::take_frame(&buffer) {\n"
     "                    heard.push(IpcMessage::decode(body).expect(\"the core sends what it says\"));\n"
     "                    buffer.drain(..next);\n"
     "                }\n",
     "                // ⚠️ `take_frame` AND NOT `unframe`: the buffer ordinarily holds a frame and a half,\n"
     "                // which `unframe` refuses by design (P-13). It hands back where the next frame starts,\n"
     "                // and `IpcMessage::decode` is given the WHOLE frame, envelope included, because\n"
     "                // `decode` unframes what it is given (task 2's bench says so in those words). Measured\n"
     "                // at the plan review (R4-2): `decode(body)` answered `Err` on every message.\n"
     "                while let Some((_, next)) = framing::take_frame(&buffer) {\n"
     "                    heard.push(\n"
     "                        IpcMessage::decode(&buffer[..next]).expect(\"the core sends what it says\"),\n"
     "                    );\n"
     "                    buffer.drain(..next);\n"
     "                }\n", 1),
    ("⚠️ **`framing::take_frame` rende il CORPO e l'offset del prossimo**, e `IpcMessage::decode` sbuccia già la busta\n"
     "(**P-15**): quale delle due forme sia giusta si **rilegge** dal blocco *Interfaces* del compito 2 e dal corpo di\n"
     "`decode`, e se diverge è una voce d'errata.",
     "✅ **Misurato alla revisione del piano intero (R4-2, R5-3): `take_frame` rende il corpo e l'offset del prossimo, e\n"
     "`IpcMessage::decode` sbuccia la busta (**P-15**) — quindi a `decode` va la cornice INTERA, `&buffer[..next]`, e non il\n"
     "corpo.** Qui stava *«quale delle due forme sia giusta si rilegge»*, e la forma vecchia, `decode(body)`, panicava al\n"
     "primo messaggio in tutte le sonde con un pari.", 1),
    # Passo 9 (R4-4/R3-15, R4-12, R4-10)
    ("    /// `Reactor::wait_until` is reached on this graph; that half is milestone 10's, where the clock\n    /// is virtual.",
     "    /// `Reactor::wait_until` is reached on this graph; that half is the sub-project 2 campaign's\n    /// (task 10), where the clock is virtual.", 1),
    ("            Parameters::new(8, TOTAL_VRAM, ARBITER_ID, Millis::ZERO),", "            Parameters::new(8, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),", 1),
    ("⚠️ **`Millis::ZERO` si RILEGGE**: se la costante non esiste, si scrive `Millis::new(0)` e si registra la\n"
     "divergenza — `grep -n 'ZERO' crates/kernel/src/time.rs`.",
     "⚠️ **`Millis::new(0)`, e non una costante `ZERO`**: `Millis::ZERO` non esiste — `grep -n 'ZERO'\n"
     "crates/kernel/src/time.rs` rende nulla, e `crates/platform/src/reactor.rs` ne registra la rimozione come elemento senza\n"
     "chiamante — misurato alla revisione del piano intero (R4-4, R3-15). Qui stava una condizionale su un fatto misurabile.", 1),
    ("la cosa che quella riga teneva si sposta** alla sonda del Passo 13, che legge il default dove adesso vive.",
     "la cosa che quella riga teneva si sposta** alla sonda del Passo 12, `the_policy_in_the_journal_is_the_one_the_arbiter_starts_on`,\nche legge il default dove adesso vive (⚠️ qui stava «Passo 13», che è `Disconnected` — R4-10).", 1),
    # Passo 10 (R4-7, R4-6)
    ("    /// ANY limit because `serve` never finishes — so on its own it cannot tell a hundred thousand\n"
     "    /// turns from one. ✅ MEASURED, not feared: with the limit cut to `1` and everything else\n"
     "    /// identical, the peer hands back an EMPTY vector and `run` answers exactly the same\n"
     "    /// `Err(TurnLimitReached)` — <data>. That measurement is what makes the assertion below an\n"
     "    /// oracle rather than a restatement of the loop.",
     "    /// ANY limit because `serve` never finishes — so on its own it cannot tell a hundred thousand\n"
     "    /// turns from none. ✅ MEASURED, not feared: with the limit cut to `0` and no peer at all, `run`\n"
     "    /// answers exactly the same `Err(TurnLimitReached)` without one turn of serving — the executor\n"
     "    /// refuses before the first poll (R4-7) — <data>. What separates the two runs is what the PEER\n"
     "    /// heard, never the value of `run`, and that measurement is what makes the assertion below an\n"
     "    /// oracle rather than a restatement of the loop.", 1),
    ("            Parameters::new(100_001, TOTAL_VRAM, ARBITER_ID, Millis::ZERO),", "            Parameters::new(WITH_A_PEER, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),", 1),
    # Passo 11 (R4-6, WELCOME)
    ("            Parameters::new(600, TOTAL_VRAM, ARBITER_ID, Millis::ZERO),", "            Parameters::new(WITH_A_PEER, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),", 2),
    ("⛔ **`WELCOME` è una costante di questo banco e si MISURA leggendo il compito 7**, non si indovina: è quanti\n"
     "messaggi il core manda dopo un `Hello` valido — `Accepted`, `Degradation`, `Policy`, `Layout`, e la lista dei\n"
     "passi, secondo la decisione **22** e la §3 della stella polare. Si conta sul dispaccio del compito 7 e si scrive\n"
     "accanto alla costante **col comando che l'ha contata**.",
     "⛔ **`WELCOME` è la costante del Passo 8, e il suo valore si RILEGGE sul dispaccio del compito 7** — `greet` in\n"
     "`crates/kernel/src/serving.rs`: `Accepted`, `Degradation`, `Policy`, `Layout` e la lista dei passi, secondo la\n"
     "decisione **22** e la §3 della stella polare — e sul banco del 7, che lo appunta in\n"
     "`the_welcome_is_the_five_messages_of_sequence_one`. Se diverge, voce d'errata. ⚠️ Qui stava *«si scrive accanto\n"
     "alla costante col comando che l'ha contata»*, e nessun passo la dettava (2026-09-15).", 1),
    # Passo 12 (R4-8/R9a-2, R4-10)
    ("- [ ] **Passo 12: le tre sonde delle decisioni — l'archivio chiuso, la policy riletta, il secondo core**",
     "- [ ] **Passo 12: le quattro sonde delle tre decisioni — l'archivio chiuso, la policy riletta, il secondo core**", 1),
    ("    #[test]\n    fn a_layout_archive_that_will_not_open_lets_the_core_start() { /* … */ }\n\n"
     "    /// ⛔ THE OTHER DIRECTION OF `MaybeCustody`, and without it a wrapper that ALWAYS refused would\n"
     "    /// pass the probe above: the archive that opens must really delegate. ✅ Held by the restart\n"
     "    /// probe of step 11, which is why this one asserts the REFUSING half only — said here rather\n"
     "    /// than left for a reviewer to wonder about.\n"
     "    #[test]\n    fn a_core_started_on_a_broken_archive_answers_unavailable_to_every_save() { /* … */ }\n\n"
     "    /// ⛔ THE `unwrap_or` OF D27, MEASURED IN BOTH DIRECTIONS, and the two are different claims.\n"
     "    /// Empty journal → the DEFAULT of ADR-0006, which is remote and lives HERE as a literal; a\n"
     "    /// journal carrying a transition → what the transition says, not the default. A `policy_now`\n"
     "    /// that answered `Remote` on an empty archive would make the two indistinguishable, which is\n"
     "    /// exactly what D27 refused inside the kernel.\n"
     "    ///\n"
     "    /// ⚠️ THE TRANSITION IS WRITTEN THROUGH `Arbiter::set_policy` ON A REAL `FileJournal`, not by\n"
     "    /// hand: a hand-built record would be this probe agreeing with itself about a format, and the\n"
     "    /// pair `set_policy`/`policy_now` is one artefact.\n"
     "    #[test]\n    fn the_policy_in_the_journal_is_the_one_the_arbiter_starts_on() { /* … */ }\n\n"
     "    /// ⛔ THE SECOND CORE. `StartupError::Ipc` has exactly one ordinary cause, and a variant with no\n"
     "    /// probe is a claim nobody checks: two graphs on one socket name, and the second must stop\n"
     "    /// instead of starting beside the first.\n"
     "    #[test]\n    fn a_second_core_on_the_same_channel_stops_the_start_up() { /* … */ }\n",
     FOUR_BODIES, 1),
    ("⛔ **I quattro corpi si scrivono al momento, sulle firme che il Passo 1 ha misurato**, e ciascuno segue la forma\n"
     "delle sonde sopra: percorso e nome propri, `run_the_graph` con limite finito e tick nullo, un pari quando serve\n"
     "un'asserzione sul filo, `join` **dopo** la corsa. ⚠️ **Non sono segnaposto:** ciò che va deciso eseguendo è il\n"
     "corpo, non la claim — e un corpo vuoto è un segnaposto, che la revisione del piano intero cerca.",
     "✅ **I quattro corpi sono DETTATI dal 2026-09-15 (R4-8, R9a-2): erano `/* … */`, cioè la specie «`…` dentro un blocco\n"
     "di codice» che la revisione del piano intero cerca, per definizione.** Ciascuno segue la forma delle sonde sopra:\n"
     "percorso e nome propri, `run_the_graph` con limite finito e tick nullo, un pari quando serve un'asserzione sul filo,\n"
     "`join` **dopo** la corsa; il secondo core si prova **senza** thread e senza gara, tenendo per mano un\n"
     "`LocalSocketIpc::bound`. ⚠️ Gli `use` del modulo `tests` crescono di `LocalPolicy`, `StepId` e `PolicyName`, e\n"
     "`cargo build` li detta uno per uno.", 1),
    # Passo 13 (R4-9)
    ("La §8 chiede *«la GUI che muore con una concessione ordinaria → `on_disconnect`, già provato in `client.rs`, più\n"
     "una sonda sul cablaggio»*. ⛔ **Nel 2 nessuna concessione ordinaria esiste** — **D5**, `Request` non è servita —\n"
     "quindi ciò che questa sonda può tenere è **solo la metà del cablaggio**: il pari esce, e il core lo toglie dai\n"
     "propri libri invece di continuare a interrogarlo.\n\n"
     "⚠️ **La metà che NON tiene si dichiara** accanto alla sonda, col proprio innesco: la concessione ordinaria\n"
     "appesa è del **7**, il pilastro 3D, che è lo stesso chiusore della riga 27 del Traguardo 6.",
     "La §8 chiede *«la GUI che muore con una concessione ordinaria → `on_disconnect`, già provato in `client.rs`, più\n"
     "una sonda sul cablaggio»*. ⛔ **Nel daemon quella sonda NON si scrive, e il perché è misurato (R4-9):** nel 2 nessuna\n"
     "concessione ordinaria esiste — **D5**, `Request` non è servita — e da fuori del binario **non c'è oracolo**:\n"
     "`run_the_graph` rende `Result<(), StartupError>`, `Core` resta dentro, e «tolto dai libri» non è osservabile. Chi lo\n"
     "osserva è il banco del **7** (`a_client_that_dies_gives_its_grant_back`, su `Core::attending`) e la campagna del\n"
     "**10** (`!attending.contains(&GUI)`). Qui stava *«ciò che questa sonda può tenere è solo la metà del cablaggio»* —\n"
     "senza nome, corpo né asserzione: un passo che non dettava nulla. Ciò che resta è la riga di doc sopra\n"
     "`run_the_graph`, dettata al Passo 7.\n\n"
     "⚠️ **La metà che NON tiene nessuno si dichiara** nel richiamo del Passo 15 (c), col proprio innesco: la concessione\n"
     "ordinaria appesa è del pilastro **3D**, che è lo stesso chiusore della riga 27 del Traguardo 6.", 1),
    # Passo 15 (R4-18, R4-9/R9a-10/D88)
    ("- [ ] **Passo 13: `Disconnected`, il cablaggio**",
     "- [ ] **Passo 13: `Disconnected` — coperto dal banco del 7 e dalla campagna del 10, e detto (R4-9)**", 1),
    ("- [ ] **Passo 15: i due richiami datati nella §5 del disegno del 2**",
     CPU_STEP + "- [ ] **Passo 15: i richiami datati nel disegno del 2 — due nella §5, uno nella §8**", 1),
    ("**(a)** In coda alla cella *«il limite di giri»*:", "**(a)** In coda alla cella *«La prova»* della riga *«il limite di giri»*, su una riga (R4-18):", 1),
    ("**(b)** In coda alla cella *«lo spegnimento»*:", "**(b)** In coda alla cella *«La prova»* della riga *«lo spegnimento»*, su una riga:", 1),
    ("dove chi la legge lo trova.\n```\n\n- [ ] **Passo 16:",
     "dove chi la legge lo trova.\n```\n\n"
     "**(c)** Nella **§8**, in coda alla cella *«La prova»* della riga *«il limite di giri e `Disconnected` (§5)»* — l'ancora\n"
     "è la riga intera che comincia con `| il limite di giri e \\`Disconnected\\` (§5) |` — su una riga (R4-9, R9a-10, **D88**):\n\n"
     "```\n"
     "✅ **RICHIAMO DEL <data>, compito 9 del piano (D5, R4-9):** la metà *«la GUI che muore con una concessione ordinaria → `on_disconnect`»* **non è provata nel daemon**, e non può esserlo: nel 2 nessuna concessione ordinaria esiste (D5), e da fuori del binario `Core` non si osserva. Il cablaggio lo tengono il banco del compito 7 (`Core::attending`) e la campagna del 10. La concessione ordinaria appesa resta del pilastro 3D, con l'innesco della riga 27 del Traguardo 6.\n"
     "```\n\n"
     "- [ ] **Passo 16:", 1),
    # Passo 16 (R4-16, R4-9, R4-11, D84)
    ("⛔ **I fine-riga devono essere IDENTICI a quelli del Passo 1**, file per file.",
     "⛔ **I fine-riga devono essere IDENTICI a quelli del Passo 1**, file per file — **tranne `Cargo.lock`**, che cargo\n"
     "riscrive in **LF** al Passo 7-bis (`i/lf w/lf`; R4-16, misurato alla revisione del piano intero con `cargo 1.95.0`):\n"
     "l'indice non cambia, ed è l'indice che il vincolo 4 difende.", 1),
    ("1 di quante ne aggiungono i Passi 10–13;", "1 di quante ne aggiungono i Passi 10–12 (R4-9);", 1),
    ("essere il **guscio**, che questo piano **non costruisce** — il 12 lega un nome suo, il 13 e il 14 portano una SPA\n"
     "che non tocca socket (**P-53**, **D31**).",
     "essere il **guscio**, che questo piano **non costruisce** — il 12 lega lo **stesso** nome come copia, e il suo criterio di\n"
     "chiusura confronta i due letterali (**D45**; qui stava «un nome suo», R4-11); il 13 e il 14 portano una SPA che non\n"
     "tocca socket (**P-53**, **D31**).", 1),
    ("dichiara, non una svista: se ne comparisse un secondo dentro `mod tests`, quella prova **si pianterebbe**.",
     "dichiara, non una svista: se ne comparisse un secondo dentro `mod tests`, quella prova **si pianterebbe**; e il\n"
     "**numero del Passo 14** sta nel messaggio di commit, con la data e senza soglia (D84).", 1),
    # D76: the seven dictated recalls
    ("MILESTONE 2 TASK 9", "SUB-PROJECT 2, TASK 9", 7),
])

tmp = PLAN + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(t)
os.replace(tmp, PLAN)
print("ok: tasks 8 and 9 patched;", t.count("\n") - raw.count("\n"), "lines added")

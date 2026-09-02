//! PROPERTY 2 of §5.7: no process is running without a valid grant. The injection is the
//! `process` port — workers started with REAL grants and killed at instants chosen by the seed —
//! and what is asserted is the ARBITER'S BOOKS after every kill.
//!
//! ⛔ WHY A BENCH OF ITS OWN AND NOT A CORNER OF `arbiter_campaign.rs`. That file is one
//! scenario — four parties asking, queueing and handing back inside the executor — and its
//! constants, its `Observed` and its two oracles all describe THAT scenario. This one has a
//! different subject, a different fake and a different injection, and folding it in would have
//! given that file two scenarios sharing a header that describes one. The cost is declared: it is
//! a fifth bench in `crates/simulator/tests/`, and the wall-time line it prints is not collected
//! by `scripts/gate.sh`, whose last step names its campaigns one by one.
//!
//! ⛔ WHAT IS ASSERTED IS THE BOOKS, AND THE TEMPORAL HALF IS COUNTED INSTEAD — which is a
//! decision and not an omission. "A valid grant" has two halves: the grant EXISTS, which
//! `Process::start` makes true by taking one by value (the compiler holds it, §5.6), and the
//! grant is STILL VALID, which nothing can observe: `start` takes no `now` and asks the arbiter
//! nothing, `GrantId` is private, and no API answers "is this grant still in the books?". So a
//! worker CAN stay alive past its own window, and a probe that pinned that shut would be a vote
//! against taking the decision that is still the owner's — `E30`/`E39` of milestone 5, gotcha
//! #73. The counter below reports how often this campaign walked into it.
//!
//! ⚠️ `FakeWorker` AND `FakeProcess` ARE DUPLICATED WORD FOR WORD from
//! `crates/kernel/tests/worker_tokens.rs`, and it is declared rather than left to be discovered:
//! TEST CODE DOES NOT CROSS CRATE BOUNDARIES, so there is no place both benches could reach.
//! It is the same deviation `Yield` carries in `arbiter_campaign.rs`, for the same reason.
//! ⚠️ A SEED-DRIVEN FAKE IN `crates/simulator/src/` WOULD BUY NOTHING HERE: what the seed decides
//! is WHEN THE CORE KILLS, and the core is this bench. The worker has no say in it.

use std::collections::BTreeSet;

use kernel::arbiter::{
    Admission, Arbiter, ArbiterId, ComputeClass, Grant, Mib, Preemption, ReleaseError, Released,
    RemotePolicy, ResourceProfile, VramPolicy,
};
use kernel::parameters::Parameters;
use kernel::ports::process::{
    Frame, Killed, Process, ProcessError, SingleReceipt, Started, StreamReceipt, Worker,
    WorkerDescriptor,
};
use kernel::rng::RngExt;
use kernel::time::{Millis, Monotonic};
use simulator::rng::SeededRng;

const TURN_LIMIT: u64 = 10_000;
const TOTAL: Mib = Mib::new(16_384);

/// How many seeds the SHORT campaign sweeps — the figure the other campaigns use, constraint 7
/// of §11: fixed and versioned with this file, never drawn from the clock or the environment.
const SHORT_CAMPAIGN_SEEDS: u64 = 2_000;

/// The instant no kill is drawn beyond. ⛔ IT IS DELIBERATELY PAST THE LONGEST WINDOW BELOW, so
/// that both answers of `release` are reachable: a horizon inside every window would make every
/// kill a `Now` and the `AlreadyCollected` road would never be walked.
const KILL_HORIZON: u64 = 8_000;

/// One worker the core starts and later kills.
///
/// ⛔ THE FOUR DIFFER IN RESERVATION AND IN WINDOW, and that is what makes the seed matter: with
/// one window for all of them, "the kill fell inside the window" would be one fact about the
/// drawn instant rather than four, and the sweep would explore a line instead of a space.
struct Recruit {
    name: &'static str,
    reserved: Mib,
    lane: ComputeClass,
    /// How long the start declares it needs the reservation for.
    valid_for: Millis,
}

/// ⚠️ A `static` AND NOT A `const`, the reason `PARTIES` carries in `arbiter_campaign.rs`: the
/// loop takes `&'static Recruit`, and `&SOME_CONST` reaches that lifetime only through rvalue
/// static promotion.
///
/// ⛔ THE FOUR RESERVATIONS ADD UP TO 10 240 AGAINST A MACHINE OF 16 384, so every one of them
/// STARTS — which is the point: this campaign is about what a KILL gives back, and a worker the
/// admission queued would be a worker with no grant to give.
static RECRUITS: [Recruit; 4] = [
    Recruit {
        name: "asr-realtime",
        reserved: Mib::new(1_024),
        lane: ComputeClass::Realtime,
        valid_for: Millis::new(1_000),
    },
    Recruit {
        name: "chat-agent",
        reserved: Mib::new(2_048),
        lane: ComputeClass::Interactive,
        valid_for: Millis::new(2_500),
    },
    Recruit {
        name: "render",
        reserved: Mib::new(3_072),
        lane: ComputeClass::Batch,
        valid_for: Millis::new(4_000),
    },
    Recruit {
        name: "indexer",
        reserved: Mib::new(4_096),
        lane: ComputeClass::Batch,
        valid_for: Millis::new(6_000),
    },
];

/// The smallest worker that answers. ⚠️ IT IS A FAKE AND IT IS ALLOWED TO BE POOR: what is under
/// test here is the ARBITER'S BOOKS, not a worker channel.
struct FakeWorker {
    next: u64,
    /// ⛔ THE GRANT THE START WAS GIVEN, AND IT IS NOT DECORATION: `kill` has to HAND IT BACK, so
    /// a worker that dropped it on the way in could not honour the signature.
    grant: Grant,
}

impl Worker for FakeWorker {
    fn instruct_one(&mut self, _frame: Frame) -> Result<SingleReceipt, ProcessError> {
        self.next += 1;
        Ok(SingleReceipt::new(self.next))
    }

    fn instruct_stream(&mut self, _frame: Frame) -> Result<StreamReceipt, ProcessError> {
        self.next += 1;
        Ok(StreamReceipt::new(self.next))
    }

    fn read_one(&mut self, receipt: SingleReceipt) -> Result<Frame, ProcessError> {
        Ok(Frame::new(receipt.id().to_le_bytes().to_vec()))
    }

    fn read_next(&mut self, _receipt: &mut StreamReceipt) -> Result<Option<Frame>, ProcessError> {
        Ok(None)
    }

    fn close(&mut self, _receipt: StreamReceipt) -> Result<(), ProcessError> {
        Ok(())
    }

    fn kill(self) -> Killed {
        Killed {
            grant: self.grant,
            outcome: Ok(()),
        }
    }
}

struct FakeProcess;

impl Process for FakeProcess {
    type Handle = FakeWorker;

    fn start(&mut self, grant: Grant, _descriptor: WorkerDescriptor) -> Started<Self::Handle> {
        Started::Running(FakeWorker { next: 0, grant })
    }
}

/// What one run observed, per worker: whether its kill found the reservation still in the books.
///
/// ⛔ IT IS THE POINT IN THE OUTCOME SPACE, which is why it derives `Ord`: the sweep counts how
/// many DISTINCT ones it produces, and one distinct outcome over two thousand seeds would mean
/// the drawn instants are not reaching the windows at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Observed {
    /// `true` where the kill landed inside the worker's window, in `RECRUITS` order.
    inside_the_window: [bool; RECRUITS.len()],
}

/// The instants this seed kills at, one per recruit, in `RECRUITS` order.
///
/// ⛔ IT COMES FROM A DERIVED MIXING AND NOT FROM THE CAMPAIGN'S SEED ITSELF — decision D2 of the
/// milestone 4 plan, and the shape `crash_point` has in `arbiter_campaign.rs`. Two `SeededRng`
/// built from the same number give the SAME sequence, so a draw taken from the bare seed moves
/// with everything else that seed decides.
fn kill_instants(seed: u64) -> [Monotonic; RECRUITS.len()] {
    let mut rng = SeededRng::new(seed.wrapping_mul(0x2545_F491_4F6C_DD1D));
    let mut instants = [Monotonic::ORIGIN; RECRUITS.len()];
    for instant in &mut instants {
        *instant = Monotonic::ORIGIN.saturating_add(Millis::new(rng.below(KILL_HORIZON)));
    }
    instants
}

fn new_arbiter() -> Arbiter {
    Arbiter::new(
        Parameters::new(TURN_LIMIT, TOTAL, ArbiterId::new(1)),
        VramPolicy::Remote(RemotePolicy),
    )
}

/// What one run of the scenario did, and the counters it fed.
struct Tally {
    kills: u64,
    released_now: u64,
    /// ⛔ KILLS THAT FOUND THE WORKER PAST ITS OWN WINDOW. It is COUNTED AND DECLARED, and it is
    /// NOT ASSERTED ON: the worker was still running while its reservation had already been swept
    /// back into the budget, which is the temporal half of "a valid grant" that nothing today can
    /// observe. `E30`/`E39` of milestone 5 is the open voice, and it is the owner's; an assertion
    /// here would freeze the decision instead of reporting it (gotcha #73).
    past_the_window: u64,
}

/// One world: four workers started with real grants, killed at the drawn instants in the order
/// the clock reaches them.
///
/// ⛔ THE KILLS ARE TAKEN IN ASCENDING INSTANT ORDER, and that is a requirement rather than a
/// tidiness: `Arbiter::release` sweeps at the `now` it is handed, so a run that released at 5 000
/// and then at 1 000 would be telling the arbiter the clock went backwards, and every assertion
/// after that would be about a world that cannot happen.
fn run(seed: u64, tally: &mut Tally) -> Observed {
    let mut arbiter = new_arbiter();
    let instants = kill_instants(seed);

    // Every recruit starts, with a grant obtained the only way there is.
    let mut running: Vec<(usize, FakeWorker)> = Vec::new();
    for (index, recruit) in RECRUITS.iter().enumerate() {
        let profile = ResourceProfile {
            name: recruit.name,
            reserved_vram: recruit.reserved,
            compute_class: recruit.lane,
            preemption: Preemption::Never,
        };
        let Admission::Granted(grant) =
            arbiter.admit(&profile, recruit.valid_for, Monotonic::ORIGIN)
        else {
            panic!(
                "seed {seed}: {} was not granted, and the four reservations fit {TOTAL:?} by \
                 construction",
                recruit.name
            );
        };
        let Started::Running(worker) = FakeProcess.start(
            grant,
            WorkerDescriptor::new(recruit.name.as_bytes().to_vec()),
        ) else {
            panic!("seed {seed}: the fake starts every worker it is asked for");
        };
        running.push((index, worker));
    }
    assert_eq!(
        arbiter.allocated(),
        RECRUITS
            .iter()
            .fold(Mib::ZERO, |sum, r| sum.saturating_add(r.reserved)),
        "seed {seed}: the books do not hold what the four starts reserved"
    );

    let mut order: Vec<usize> = (0..RECRUITS.len()).collect();
    order.sort_by_key(|&index| (instants[index], index));

    let mut killed = [false; RECRUITS.len()];
    let mut inside_the_window = [false; RECRUITS.len()];

    for index in order {
        let now = instants[index];
        let position = running
            .iter()
            .position(|(running_index, _)| *running_index == index)
            .unwrap_or_else(|| panic!("seed {seed}: worker {index} was killed twice"));
        let (_, worker) = running.remove(position);

        // ⛔ THE GRANT COMES BACK FROM THE KILL, outside every `Result` — level 1, and the
        // assertion below is what makes it worth something: a reservation is a fact of the books,
        // not of the process's health.
        let Killed { grant, outcome } = worker.kill();
        assert_eq!(
            outcome,
            Ok(()),
            "seed {seed}: the fake kills every worker cleanly"
        );
        killed[index] = true;
        tally.kills += 1;

        let expires_at = Monotonic::ORIGIN.saturating_add(RECRUITS[index].valid_for);
        let still_valid = now < expires_at;
        inside_the_window[index] = still_valid;

        // ⛔ AND IT IS NEVER AN `Err`, WITH ONE ARBITER. `ReleaseError::UnknownGrant` means a
        // grant ANOTHER arbiter issued, and this scenario builds one — so an `Err` here would say
        // the issuer stamp is wrong, which is a defect of `issue` or of `release` and not an
        // outcome of the world.
        let answer = arbiter.release(grant, now);
        if still_valid {
            assert_eq!(
                answer,
                Ok(Released::Now(RECRUITS[index].reserved)),
                "seed {seed}: {} was killed at {now:?}, inside its window that closes at \
                 {expires_at:?}, and the reservation did not come back",
                RECRUITS[index].name
            );
            tally.released_now += 1;
        } else {
            assert_eq!(
                answer,
                Ok(Released::AlreadyCollected),
                "seed {seed}: {} was killed at {now:?}, past its window that closed at \
                 {expires_at:?}, so the sweep should already have had it",
                RECRUITS[index].name
            );
            tally.past_the_window += 1;
        }
        assert_ne!(
            answer,
            Err(ReleaseError::UnknownGrant),
            "seed {seed}: the one arbiter of this scenario disowned a grant it issued itself"
        );

        // ⛔ PROPERTY 2, ASSERTED AFTER EVERY KILL AND NOT AT THE END: the books must hold
        // EXACTLY the reservations of the workers this bench knows are still running and whose
        // windows have not closed. A check only at the end is green for an arbiter that keeps a
        // dead worker's reservation and tidies up later.
        let expected = RECRUITS
            .iter()
            .enumerate()
            .filter(|(other, recruit)| {
                !killed[*other] && now < Monotonic::ORIGIN.saturating_add(recruit.valid_for)
            })
            .fold(Mib::ZERO, |sum, (_, recruit)| {
                sum.saturating_add(recruit.reserved)
            });
        assert_eq!(
            arbiter.allocated(),
            expected,
            "seed {seed}: after killing {} at {now:?} the books hold {:?} and the workers still \
             running with an open window reserve {expected:?}",
            RECRUITS[index].name,
            arbiter.allocated()
        );
    }

    Observed { inside_the_window }
}

/// ⛔ PROPERTY 2 (§5.7): no process is running without a valid grant, seen where it is
/// observable — the arbiter's books, after every kill. The assertions live in `run`, fired after
/// each of the four kills of every seed; this test is what sweeps the seeds and reports which one
/// broke.
///
/// ⛔ TWO ORACLES AND NOT ONE, §5.7.1. `kills` says the injection fired; `released_now` says
/// there was something to verify — a campaign whose workers all expired before their kill would
/// hand back nothing but `AlreadyCollected` and be green having compared the books against a
/// budget that was already empty.
///
/// ⛔ THE FIRST IS AN EQUALITY AND NOT `> 0`, the shape `property_4` uses in
/// `arbiter_campaign.rs`: every seed kills every recruit, so a total short of the product would
/// mean a worker was never reached — the silent no-op of gotcha #17, which `> 0` would let all
/// but one seed hide.
#[test]
fn property_2_a_killed_worker_leaves_no_reservation_behind() {
    let started = std::time::Instant::now();
    let mut tally = Tally {
        kills: 0,
        released_now: 0,
        past_the_window: 0,
    };
    let mut distinct = BTreeSet::new();

    for seed in 0..SHORT_CAMPAIGN_SEEDS {
        distinct.insert(run(seed, &mut tally));
    }
    let elapsed = started.elapsed();

    assert_eq!(
        tally.kills,
        SHORT_CAMPAIGN_SEEDS * RECRUITS.len() as u64,
        "a recruit was never killed: the scenario performed fewer kills than it started workers"
    );
    assert!(
        tally.released_now > 0,
        "no kill ever landed inside a worker's window: every reservation was already swept, so \
         the books were compared against an empty budget on every one of {} kills",
        tally.kills
    );
    assert!(
        distinct.len() > 1,
        "every seed produced the SAME outcome: the drawn instants are not reaching the windows, \
         so this campaign is one run repeated {SHORT_CAMPAIGN_SEEDS} times"
    );

    // ⚠️ PRINTED AND NOT ASSERTED, and the difference is the decision. `past_the_window` counts
    // the kills that found a worker STILL RUNNING with its reservation already swept back into
    // the budget — the temporal half of "a valid grant", which no API can observe today. It is
    // the open voice `E30`/`E39` of milestone 5 and it belongs to the owner; a bar here would
    // freeze it (gotcha #73). What this line buys is that the day it becomes 0 or becomes
    // everything, a reader sees it.
    println!(
        "DST worker kills: {} kills over {SHORT_CAMPAIGN_SEEDS} seeds, {} inside the window, {} \
         past it (E30 -- counted, not asserted), {} distinct outcomes, {elapsed:?}",
        tally.kills,
        tally.released_now,
        tally.past_the_window,
        distinct.len()
    );
}

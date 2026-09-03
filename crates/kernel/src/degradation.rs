//! The degradation state (§6.7, ADR-0019): DERIVED, recomputable, never authoritative of itself.
//!
//! ⛔ IT IS A PROJECTION AND NOT A SECOND ARCHIVE, exactly as `crate::permission` is for the
//! triple and `crate::reconcile` is for the steps in doubt. "What is degraded now" is answered by
//! re-reading what was written and by asking the arbiter what it holds; a state kept beside those
//! two would be a second place holding one truth (§7.4.4), and the first of the two to stop being
//! updated lies with authority.

use crate::arbiter::Arbiter;
use crate::ports::journal::{Journal, JournalError};
use crate::record::{Detail, Record, RecordError, RecordKind};

/// What is degraded right now. ⛔ THE SELECTION CRITERION IS §7.5's, and it is the reason this
/// struct is short: what is shown is what CHANGES WHAT THE USER CAN DO, not every internal
/// variation. "An interface that signals everything is indistinguishable from one that signals
/// nothing."
///
/// ⛔ TWO INPUTS OF ADR-0019 HAVE NO SOURCE IN THIS MILESTONE, and they are declared rather than
/// faked: CONNECTIVITY (§7 — `network` has no real implementation, §8.2.2) and PROVIDER HEALTH
/// (§6.2 — the adapters are rule C). Their triggers are their implementations, and no field
/// stands here waiting for them: a field that is always `false` reads as "fine" rather than as
/// "unknown", which is the falsest of the two.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Degradation {
    /// The arbiter's books have reached the ceiling, with the limit of that declared beside the
    /// comparison. ⛔ IT IS AN INPUT NAMED BY ADR-0019 ITSELF, and §5 added a revocable consumer
    /// to it (ADR-0033) — "the 3D viewer is paused during a render" changes what the user can do.
    pub vram_exhausted: bool,
    /// The last routing resolved relaxed a quality constraint (ADR-0012). ⛔ DECLARED, not
    /// silent — and this field is where the declaration stops being private to the record.
    pub routing_degraded: bool,
}

/// What can go wrong DERIVING the state, as opposed to deriving it "nothing is degraded".
///
/// ⛔ WHY `degradation_now` CANNOT SIMPLY ANSWER `false` TO EVERYTHING WHEN IT CANNOT READ THE
/// ARCHIVE. A journal that will not replay, or a record this build cannot decode, means THE
/// ANSWER IS UNKNOWN — and "unknown" reported as "nothing is degraded" is precisely the silent
/// degradation ADR-0019 forbids, arrived at by the one road that ADR did not foresee: not a
/// degradation that went unrecorded, but one that was recorded and then read as absent. It is the
/// same sentence the doc of `Degradation` writes about the missing fields — "a field that is
/// always `false` reads as 'fine' rather than as 'unknown', which is the falsest of the two" —
/// applied to the whole answer instead of to one field.
///
/// ⛔ IT COMPOSES THE TWO CAUSES THAT REALLY EXIST, and there are exactly two because the
/// derivation does exactly two things with the archive: it REPLAYS (the port's failure) and it
/// DECODES (the record's). The arbiter half cannot fail — `allocated` and `ceiling` are two reads
/// of memory this process owns — so it contributes no variant.
///
/// ⚖️ THE SHAPE IS THE ONE `permission::PermissionError` ALREADY HAS, and it is reused rather than
/// re-argued: same two causes, same refusal to fold a failure into a real answer, same payloads
/// carried because both are `Copy` and discarding them would make this type say less than the
/// function knows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DegradationError {
    /// The journal would not hand its records back.
    Journal(JournalError),
    /// A record in the journal could not be read as a record this build understands. ⛔ IT ALSO
    /// COVERS A `Routing` RECORD WHOSE `detail` IS NOT A ROUTING, which is unpronounceable in
    /// source — `RecordV1::routing` takes its detail by value — and reachable from BYTES. Such a
    /// record declares no degradation and denies none, so the only honest answer is that this
    /// build cannot read it.
    Record(RecordError),
}

/// Derives the state from the world as it is NOW.
///
/// ⛔ NO CACHE, AND THAT IS THE WHOLE DESIGN. It follows `reconcile::steps_in_doubt`: read,
/// derive, answer. It makes "never authoritative of itself" true BY CONSTRUCTION rather than by
/// discipline — a cache is bought the day a measurement asks for one, which is the same formula
/// the journal's checkpoint carries.
///
/// ⛔ A FREE FUNCTION TAKING THE ARBITER AND THE PORT, exactly like `permission::is_granted` and
/// `reconcile::steps_in_doubt`. A struct holding either would give the projection STATE (`I1`,
/// `I5`), and state is the one thing a projection must not have: the answer would then depend on
/// when the struct was built rather than on what the world says now.
///
/// ⚠️ DECLARED DIVERGENCE, and the owner may read it the other way: ADR-0019 and §6.7 say the
/// core "MAINTAINS a current degradation state, FED BY EVENTS", and those words also read as
/// incremental maintenance. This reads "maintains" as "exposes". §5.2 of the milestone design
/// makes the same reading and says out loud that the other one is the owner's to choose.
///
/// ⚠️ AND THE COST OF ANSWERING BY RE-READING IS THE COST `Journal::replay` DECLARES: the whole
/// journal is loaded to answer one question, and on a production archive that does not hold. The
/// remedy is the same checkpoint that operation names, and it is closed by the first consumer
/// that measures a large journal — not invented here.
pub fn degradation_now<J: Journal>(
    arbiter: &Arbiter,
    journal: &J,
) -> Result<Degradation, DegradationError> {
    // ⛔ THE LAST ROUTING AND NOT ANY ROUTING: a degradation that happened and was then resolved
    // is not the state NOW, and "ever degraded" would be a fact about history rather than about
    // what the user can do — which is the §7.5 criterion this file is built on. The assignment is
    // therefore an assignment and never an `|=`, and `tests/degradation_state.rs` holds the
    // difference with two dispatches upon one step.
    //
    // ⚠️ AND "LAST" IS `replay`'s WRITE ORDER, WHICH THE PORT DOES OWE — it promises to "re-read
    // EVERYTHING, in write order", and that "WRITE ORDER IS PART OF THE PROMISE". What the
    // conformance suite does not exercise is TWO NOTES UPON ONE STEP, which is the case this
    // rests on: measured 2026-09-02, of the four `.note` calls in `assert_journal_contract` two
    // are asserted refused and the two that succeed sit on different steps. The order is owed;
    // the case is untested by the suite, and `tests/degradation_state.rs` holds it.
    let mut routing_degraded = false;

    for (_, bytes) in journal.replay().map_err(DegradationError::Journal)? {
        // ⛔ A RECORD THIS BUILD CANNOT READ STOPS THE ANSWER, and it does NOT skip to the next
        // one. Skipping would mean reporting "nothing is degraded" while a degraded routing may
        // be sitting in the very bytes that would not decode — see `DegradationError` for why
        // that is forbidden.
        let Record::V1(body) = Record::decode(&bytes).map_err(DegradationError::Record)?;

        // ⚠️ EVERY OTHER SPECIES IS SKIPPED AND THAT IS ORDINARY, not a hole: a journal holding a
        // routing holds at least the INTENT of the step the routing was noted upon, so the mixed
        // journal is the only journal this function ever sees.
        if body.kind() != RecordKind::Routing {
            continue;
        }

        // ⛔ AND A `Routing` WITHOUT ITS ROUTING DETAIL IS `Malformed`, not a silent `if let`.
        // The species says a routing decision was recorded; a detail that is not a routing means
        // this build cannot read what was decided, which is the one thing that must not read as
        // "nothing was degraded". Same judgement and same road as the sister
        // `permission::is_granted` takes for its own species.
        let Some(Detail::Routing(routing)) = body.detail() else {
            return Err(DegradationError::Record(RecordError::Malformed));
        };

        routing_degraded = routing.degraded();
    }

    Ok(Degradation {
        // ⛔ THE COMPARISON IS AGAINST WHAT THE ARBITER WAS HANDED, not against a number chosen
        // here: `ceiling` is the DELIVERED `total_vram` (ADR-0034), and reading the GPU would be
        // an OS call `I3` forbids.
        //
        // ⛔ AND IT IS `>=` AND NOT `>`: a machine whose books exactly equal its ceiling is full.
        // Nothing has to be subtracted from either side — the two permanent quotas of ADR-0033
        // reach the arbiter through `admit`, so they are already inside `allocated()`.
        //
        // ⚠️ AND THE LIMIT IT INHERITS FROM `allocated`, which declares of itself that it
        // "COLLECTS NOTHING": between two operations an EXPIRED grant stays in the books, so
        // there is an instant where this answers `true` while `admit` — which collects before it
        // decides — would grant. The direction is CONSERVATIVE: it says "exhausted" a moment
        // before the arbiter contradicts it, never the reverse. It is closed by the collection
        // `admit` already performs, not by a sweep invented here.
        vram_exhausted: arbiter.allocated() >= arbiter.ceiling(),
        routing_degraded,
    })
}

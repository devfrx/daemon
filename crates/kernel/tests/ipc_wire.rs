//! The schema of the `ipc` channel. ⛔ OUTSIDE THE CRATE, like `framing` and `worker_wire`.

use bincode::error::{AllowedEnumVariants, DecodeError};
use kernel::arbiter::{ComputeClass, Mib, Preemption};
use kernel::framing::{self, LENGTH_WIDTH, WireError};
use kernel::time::Millis;
use kernel::wire::ipc::{
    build_stamp, stamp_set, Access, Call, DegradationReport, GrantRequest, IpcMessage, LayoutState,
    PolicyName, PolicyReport, Protection, Provenance, StepSummary, Triple, Verdict,
};

fn a_request() -> GrantRequest {
    GrantRequest {
        reserved_vram: Mib::new(2048),
        compute_class: ComputeClass::Interactive,
        preemption: Preemption::After(Millis::new(500)),
    }
}

#[test]
fn a_grant_request_survives_the_round_trip() {
    let message = IpcMessage::Request(a_request());
    let bytes = message.encode().expect("encode");
    assert_eq!(IpcMessage::decode(&bytes), Ok(message));
}

#[test]
fn a_verdict_survives_the_round_trip() {
    // ⛔ THIS IS THE PROBE THAT EXERCISES THE DISCRIMINANT, and it is why §6.7 asks for TWO
    // messages rather than one: with a single message type the tag never varies, and a bug in
    // how it is written or read would be invisible. Same shape as the journal freezing one
    // record per variant instead of one. Audit of 2026-09-30, AUD-097.
    let message = IpcMessage::Verdict(Verdict::Refused {
        asked: Mib::new(4096),
        ceiling: Mib::new(1024),
    });
    let bytes = message.encode().expect("encode");
    assert_eq!(IpcMessage::decode(&bytes), Ok(message));
}

#[test]
fn a_queued_verdict_survives_the_round_trip() {
    // ⛔ THE THIRD WAY OF THE VERDICT, AND IT WAS THE ONE NOTHING TOUCHED. `Refused` makes the
    // round trip above and `Granted` is encoded by the two probes below, so `Queued` was the
    // only variant of this enum that never reached the wire at all -- a wrong tag on it would
    // have been read by no assertion in the workspace. Mutation G5 measures the gap it closes.
    //
    // ⚠️ ITS OWN `#[test]` AND NOT A SECOND ASSERTION IN `a_verdict_survives_the_round_trip`,
    // for the reason written on the pair at the bottom of this file -- gotcha #14. That probe
    // asserts on `Refused` FIRST, so a red there would stop it before this input ran, and the
    // one variant with no other exercise anywhere is exactly the one that must not depend on
    // another assertion passing.
    let message = IpcMessage::Verdict(Verdict::Queued);
    let bytes = message.encode().expect("encode");
    assert_eq!(IpcMessage::decode(&bytes), Ok(message));
}

#[test]
fn a_message_with_a_tail_does_not_decode() {
    let mut bytes = IpcMessage::Verdict(Verdict::Granted)
        .encode()
        .expect("encode");
    bytes.push(0xFF);
    assert_eq!(IpcMessage::decode(&bytes), Err(WireError::TrailingBytes));
}

#[test]
fn junk_inside_the_declared_length_does_not_decode() {
    // ⛔ THE ENVELOPE IS HONEST HERE and the body is not -- the other half of the pair, and it
    // is a DIFFERENT check. See the same probe in `worker_wire.rs`.
    let good = IpcMessage::Verdict(Verdict::Granted)
        .encode()
        .expect("encode");
    let mut junked = good[LENGTH_WIDTH..].to_vec();
    junked.push(0xFF);
    let bytes = framing::frame(&junked).expect("frame");
    assert_eq!(IpcMessage::decode(&bytes), Err(WireError::Malformed));
}

#[test]
fn an_empty_body_in_an_honest_envelope_does_not_decode() {
    // ⛔ THE HALF THAT NOTHING WOULD HOLD WITHOUT IT, AND IT IS NOT A LATE ADDITION HERE: the
    // twin bench got these two on 2026-08-31, THE DAY AFTER the plan that dictates this file
    // was written, so the four probes this file was asked for would have left the
    // `map_err(|_| WireError::Malformed)` of `IpcMessage::decode` reached by NOTHING -- which
    // is exactly what mutation W9 measured on `wire::worker`, where it left the ENTIRE
    // workspace green. The four cannot reach it: both round trips DECODE, the tail outside the
    // envelope dies inside `framing::unframe`, and the junk inside the declared length decodes
    // successfully and falls on the consumed-bytes comparison, which is a different check.
    //
    // ⛔ THE EMPTY BODY IS ALSO WHAT `IpcMessage::encode` CONTAINS. That method takes the
    // `Vec` of a failed encoding as EMPTY rather than propagating an error, on the argument
    // written beside it, and the argument is only worth what this probe buys: an empty body
    // does not decode, so a stopped encoder produces a frame the peer REFUSES.
    //
    // ⛔ THE ENVELOPE IS HONEST HERE, and that is what makes this a probe of the SCHEMA rather
    // than of the envelope: `frame(&[])` is `00 00 00 00`, `unframe` answers `Ok(&[])`, and the
    // refusal comes from the body decoder alone. Break `framing`'s two checks and this survives.
    let bytes = framing::frame(&[]).expect("frame");
    assert_eq!(bytes, [0x00, 0x00, 0x00, 0x00]);
    assert_eq!(IpcMessage::decode(&bytes), Err(WireError::Malformed));
}

#[test]
fn a_truncated_body_in_an_honest_envelope_does_not_decode() {
    // ⚠️ THE OTHER INPUT OF THE PAIR, AND IT IS A SEPARATE `#[test]` ON PURPOSE -- gotcha #14.
    // One probe holding both would stop at the first failing assertion and never exercise the
    // second input, so a red would hide half of what it was written to see. The twin measured
    // that both of its two go red under the mutation; the same is measured here, mutation G4.
    //
    // "Truncated" is the shape a stopped encoder really leaves behind -- the prefix is well
    // formed as far as it goes. Cut from a real message rather than written as a literal, and
    // from the LONGEST of the two, so the cut lands inside the request and not on its tag.
    // ⚠️ NOT the same fault as `a_message_with_a_tail_does_not_decode`: there the envelope
    // LIES, here it tells the truth about a body that is genuinely short.
    let good = IpcMessage::Request(a_request()).encode().expect("encode");
    let body = &good[LENGTH_WIDTH..];
    let bytes = framing::frame(&body[..body.len() / 2]).expect("frame");
    assert_eq!(IpcMessage::decode(&bytes), Err(WireError::Malformed));
}

/// The name of the enum `T` and how many variants it has -- READ FROM THE TYPE, never written
/// beside it. ⚠️ RECALL OF 2026-10-07 -- audit of 2026-09-30, AUD-2228.
///
/// ⛔ A COUNT WRITTEN BY HAND IS A SECOND COPY OF A FACT THE TYPE CARRIES, and the two guards
/// below cannot rest on one: the exhaustive `match` asks whoever adds a variant for an ARM, never
/// for a raised number, so a variant added with its arm and forgotten in `stamp_set` would leave
/// both guards counting the old variants, GREEN. Read from the type, it is a slot no message
/// carries, and a red that names it.
///
/// ⚠️ WHAT IT RESTS ON, declared: bincode writes an enum as the INDEX of its variant, and the
/// derive answers an index past the last with `DecodeError::UnexpectedVariant`, whose `allowed` is
/// `AllowedEnumVariants::Range { min: 0, max }` with `max` the last index -- read in
/// `bincode_derive` 2.0.1, `src/derive_enum.rs`. One byte below 251 is a whole index in the
/// varint of `config::standard()`. ⛔ ANY OTHER ANSWER STOPS BOTH GUARDS HERE, IN WORDS: a bincode
/// that answered differently is a red that says so, never a count of nothing.
fn variants_of<T: bincode::Decode<()>>() -> (&'static str, usize) {
    const PAST_THE_LAST: [u8; 1] = [250];
    match bincode::decode_from_slice::<T, _>(&PAST_THE_LAST, bincode::config::standard()) {
        Err(DecodeError::UnexpectedVariant {
            type_name,
            allowed: AllowedEnumVariants::Range { min: 0, max },
            ..
        }) => (type_name, *max as usize + 1),
        Err(error) => panic!("index 250 did not answer with the range of the variants: {error:?}"),
        Ok(_) => panic!("index 250 decoded: not an enum, or one with more than 250 variants"),
    }
}

/// Which variants of ONE enum the canonical set carries -- the two guards below. A slot is the
/// number a guard's arm gives its variant, in declaration order; past the count it says so in
/// words instead of panicking with `index out of bounds`.
struct Carried {
    name: &'static str,
    seen: Vec<bool>,
}

impl Carried {
    fn of<T: bincode::Decode<()>>() -> Self {
        let (name, variants) = variants_of::<T>();
        Carried {
            name,
            seen: vec![false; variants],
        }
    }

    fn saw(&mut self, slot: usize) {
        assert!(
            slot < self.seen.len(),
            "{}: slot {slot} is past its {} variants -- the arms number them from 0, in \
             declaration order",
            self.name,
            self.seen.len()
        );
        self.seen[slot] = true;
    }

    /// The slots no message carried, as one line of the report -- `None` when every one is there.
    fn missing(&self) -> Option<String> {
        let missing: Vec<usize> = (0..self.seen.len()).filter(|slot| !self.seen[*slot]).collect();
        (!missing.is_empty()).then(|| format!("  {}: slots {missing:?}", self.name))
    }
}

#[test]
fn every_variant_is_in_the_canonical_set() {
    // ⛔ THE GUARD THAT MAKES THE OTHER PROBES WORTH SOMETHING. A variant added to
    // `IpcMessage` and forgotten in `stamp_set` would leave the fixtures short, the stamp
    // unchanged, and every probe in this file GREEN -- the schema would have grown and
    // nothing would say so. The `match` is exhaustive on purpose: adding a variant makes THIS
    // a compile error, which is level 1, and the count read from the type (`variants_of`) makes
    // it a red until a message carrying it joins the set.
    //
    // ⚠️ THE WHOLE SET AND NOT ITS HEAD. The set opens with one message per variant, but a
    // message added later goes at the END -- for a new variant as for a nested one, the doc of
    // `stamp_set` -- so that every fixture already written keeps its index; a guard that read
    // only the head would take the first nested message for a second `Policy`. ⚠️ RECALL OF
    // 2026-10-07 -- audit of 2026-09-30, AUD-2228.
    let mut variants = Carried::of::<IpcMessage>();
    for message in stamp_set() {
        variants.saw(match message {
            IpcMessage::Hello(_) => 0,
            IpcMessage::Accepted(_) => 1,
            IpcMessage::StaleBuild(_) => 2,
            IpcMessage::Degradation(_) => 3,
            IpcMessage::Policy(_) => 4,
            IpcMessage::Invoke(_) => 5,
            IpcMessage::PermissionRequired(_) => 6,
            IpcMessage::Approve { .. } => 7,
            IpcMessage::Token { .. } => 8,
            IpcMessage::Layout(_) => 9,
            IpcMessage::SaveLayout(_) => 10,
            IpcMessage::Steps(_) => 11,
            IpcMessage::Request(_) => 12,
            IpcMessage::Verdict(_) => 13,
        });
    }
    let missing = variants.missing();
    assert!(
        missing.is_none(),
        "variants missing from stamp_set -- append a message that carries each, then regenerate \
         the fixtures:\n{}",
        missing.unwrap_or_default()
    );
}

#[test]
fn every_variant_of_every_nested_enum_is_in_the_canonical_set() {
    // ⛔ THE SAME GUARD ONE LEVEL DOWN, AND WITHOUT IT THE STAMP IS BLIND TO A NESTED VARIANT.
    // bincode writes an enum as the INDEX of its variant and nothing else, so a variant no message
    // of the set carries changes no byte when it appears: one appended to `LayoutState` -- or two
    // that nobody carried, swapped -- left every fixture and the stamp as they were, and a stale
    // gui would pass the handshake to fail on the first message it could not read. Every variant
    // at every level is the rule the journal's frozen bytes already keep
    // (`every_variant_of_the_wire_enums_is_pinned_by_a_frozen_record`).
    //
    // ⛔ EVERY `match` BELOW IS EXHAUSTIVE: a variant added to a nested enum is a compile error
    // HERE, level 1, and then -- the count read from the type, `variants_of` -- a red until a
    // message carrying it joins `stamp_set`. ⛔ AND THE PAYLOADS ARE TAKEN APART FIELD BY FIELD,
    // WITH NO `..`, which is what reaches the enum nobody has written yet: a field added to any of
    // them stops this compiling, and whoever adds it decides here whether it is an enum the stamp
    // must see. `BuildStamp` is the one payload left shut -- its field is private by design, and
    // it is a `u64`.
    //
    // ⚠️ ONE REPORT AND NOT ONE ASSERT PER ENUM, gotcha #14: one red names every missing slot.
    let mut verdict = Carried::of::<Verdict>();
    let mut protection = Carried::of::<Protection>();
    let mut policy = Carried::of::<PolicyName>();
    let mut access = Carried::of::<Access>();
    let mut provenance = Carried::of::<Provenance>();
    let mut layout = Carried::of::<LayoutState>();
    let mut class = Carried::of::<ComputeClass>();
    let mut preemption = Carried::of::<Preemption>();

    let access_slot = |operation: Access| match operation {
        Access::Read => 0,
        Access::Write => 1,
    };
    for message in stamp_set() {
        match message {
            IpcMessage::Hello(_) | IpcMessage::StaleBuild(_) => {}
            IpcMessage::Accepted(value) => protection.saw(match value {
                Protection::AsSystemAccount => 0,
            }),
            IpcMessage::Degradation(DegradationReport {
                vram_exhausted: _,
                routing_degraded: _,
            }) => {}
            IpcMessage::Policy(PolicyReport { policy: name, allocated: _, total: _ }) => {
                policy.saw(match name {
                    PolicyName::Remote => 0,
                    PolicyName::Local => 1,
                })
            }
            IpcMessage::Invoke(Call { function: _, argument: _ }) => {}
            IpcMessage::PermissionRequired(Triple { tool: _, resource: _, operation }) => {
                access.saw(access_slot(operation))
            }
            IpcMessage::Approve {
                triple: Triple { tool: _, resource: _, operation },
                call: Call { function: _, argument: _ },
            } => access.saw(access_slot(operation)),
            IpcMessage::Token { text: _, provenance: value } => provenance.saw(match value {
                Provenance::Trusted => 0,
                Provenance::Untrusted => 1,
            }),
            IpcMessage::Layout(state) => layout.saw(match state {
                LayoutState::Package(_) => 0,
                LayoutState::Nothing => 1,
                LayoutState::Unavailable => 2,
            }),
            IpcMessage::SaveLayout(_) => {}
            // ⚠️ A LOOP WITH NOTHING IN IT, and the pattern is the point: the compiler checks it
            // whatever the vector holds, so a field added to `StepSummary` lands here too.
            IpcMessage::Steps(steps) => {
                for StepSummary { step: _, function: _, done: _ } in steps {}
            }
            IpcMessage::Request(GrantRequest {
                reserved_vram: _,
                compute_class,
                preemption: value,
            }) => {
                class.saw(match compute_class {
                    ComputeClass::Realtime => 0,
                    ComputeClass::Interactive => 1,
                    ComputeClass::Batch => 2,
                });
                preemption.saw(match value {
                    Preemption::Never => 0,
                    Preemption::After(_) => 1,
                });
            }
            IpcMessage::Verdict(value) => verdict.saw(match value {
                Verdict::Granted => 0,
                Verdict::Queued => 1,
                Verdict::Refused { asked: _, ceiling: _ } => 2,
            }),
        }
    }

    let all = [
        &verdict,
        &protection,
        &policy,
        &access,
        &provenance,
        &layout,
        &class,
        &preemption,
    ];
    let missing: Vec<String> = all.iter().filter_map(|nested| nested.missing()).collect();
    assert!(
        missing.is_empty(),
        "variants of nested enums missing from stamp_set -- append a message that carries each, \
         then regenerate the fixtures:\n{}",
        missing.join("\n")
    );
}

#[test]
fn every_message_of_the_canonical_set_survives_the_round_trip() {
    // ⚠️ ONE LOOP THAT COLLECTS RATHER THAN ONE ASSERT PER MESSAGE IN A ROW -- gotcha #14 from
    // the other side. A row of asserts stops at the first red and hides every one after it; a
    // loop that stops does the same. This one records every failure and reports them together,
    // so one red tells the whole story.
    let mut broken = Vec::new();
    for message in stamp_set() {
        let bytes = match message.encode() {
            Ok(bytes) => bytes,
            Err(_) => {
                broken.push(alloc_fmt(&message, "encode failed"));
                continue;
            }
        };
        match IpcMessage::decode(&bytes) {
            Ok(back) if back == message => {}
            Ok(_) => broken.push(alloc_fmt(&message, "decoded to a different value")),
            Err(error) => broken.push(alloc_fmt(&message, &format!("{error:?}"))),
        }
    }
    assert!(broken.is_empty(), "round trip failed for:\n{}", broken.join("\n"));
}

fn alloc_fmt(message: &IpcMessage, why: &str) -> String {
    format!("  {message:?} -- {why}")
}

#[test]
fn a_string_in_the_schema_still_cannot_stop_the_encoder() {
    // ⛔ THIS IS P-17 MEASURED RATHER THAN DEDUCED. The doc of `IpcMessage::encode` argues that
    // a failed encoding cannot happen by reading THIS TYPE'S GRAPH, and until today that graph
    // was `Mib(u64)`, `ComputeClass`, `Preemption`, `Millis(u64)` and unit variants. The
    // variants added today put `String` and `Vec<u8>` in it, so the argument is RE-READ rather
    // than inherited: of the four `EncodeError` variants reachable without `std`
    // (`UnexpectedEnd`, `RefCellAlreadyBorrowed`, `Other`, `OtherString`) none is producible by
    // a derived `Encode` over `String` and `Vec<u8>` into a growing `Vec`. Read in
    // bincode 2.0.1's `src/error.rs` on 2026-09-11.
    //
    // ⚠️ AND THE INPUT IS THE AWKWARD ONE, not a convenient short string: multi-byte UTF-8, an
    // empty string, and an empty byte vector in the same message.
    let message = IpcMessage::Token {
        text: String::from("caffè ☕ \u{0}\u{7F}"),
        provenance: Provenance::Untrusted,
    };
    let bytes = message.encode().expect("a String does not stop the encoder");
    assert_ne!(bytes.len(), LENGTH_WIDTH, "an empty body would mean the encoder stopped");
    assert_eq!(IpcMessage::decode(&bytes), Ok(message));

    let empty = IpcMessage::Layout(LayoutState::Package(Vec::new()));
    let bytes = empty.encode().expect("an empty Vec does not stop the encoder");
    assert_eq!(IpcMessage::decode(&bytes), Ok(empty));
}

#[test]
fn the_stamp_changes_when_the_schema_changes() {
    // ⛔ THE SECOND DIRECTION, AND WITHOUT IT THE STAMP PROVES NOTHING. That `build_stamp()`
    // returns the same value twice is what a constant would also do. What must hold is that a
    // DIFFERENT set gives a DIFFERENT stamp -- computed here over a set with one message
    // altered, using the same function the real one uses.
    assert_eq!(build_stamp(), build_stamp(), "the stamp is stable within a build");

    // ⛔ THE LINE THAT MAKES THE TWO `assert_ne!` BELOW MEAN ANYTHING -- C-1 of the review of
    // 2026-09-17. "Using the same function the real one uses" was held by NOTHING: an
    // `assert_ne!` is satisfied by ANY difference, INCLUDING the one that appears when the
    // oracle `fnv_over` and the subject `build_stamp` stop being the same arithmetic. So this
    // probe passed with `fnv_over` returning a constant, and passed with `build_stamp`
    // returning a literal that ignores `stamp_set` ENTIRELY -- the schema changed, the stamp
    // did not, and the whole bench was green. That is the failure §6.1.2 exists to prevent.
    //
    // ⚠️ IT IS AN `assert_eq!` BETWEEN ORACLE AND SUBJECT, and it is NOT the vacuous shape the
    // doc of `fnv_over` refuses: `fnv_over` still takes the set as an argument and spells the
    // arithmetic out, so this compares two INDEPENDENT spellings of one function over one
    // input. What it forbids is exactly what was possible: the two drifting apart in silence.
    assert_eq!(
        fnv_over(&stamp_set()),
        build_stamp().get(),
        "the oracle must be the same arithmetic as the subject"
    );

    let mut altered = stamp_set();
    // `build_stamp()` is the only constructor there is (the doc of `BuildStamp` refuses a `new`),
    // and its value differs from the arbitrary one the canonical set carries: that is all this needs.
    altered[0] = IpcMessage::Hello(build_stamp());
    assert_ne!(
        fnv_over(&altered),
        build_stamp().get(),
        "a changed message must change the stamp"
    );

    let shortened: Vec<IpcMessage> = stamp_set().into_iter().skip(1).collect();
    assert_ne!(
        fnv_over(&shortened),
        build_stamp().get(),
        "a missing message must change the stamp"
    );
}

/// The same arithmetic as `build_stamp`, over a set the probe chooses. ⚠️ WRITTEN OUT RATHER
/// THAN CALLING `build_stamp`, which takes no argument: an oracle that calls the thing it
/// checks would be vacuous.
fn fnv_over(messages: &[IpcMessage]) -> u64 {
    let mut hash: u64 = 0xCBF2_9CE4_8422_2325;
    for message in messages {
        let bytes = message.encode().expect("encode");
        for byte in (bytes.len() as u64).to_be_bytes().iter().chain(bytes.iter()) {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        }
    }
    hash
}

#[test]
#[ignore = "generator, not a check: run it on purpose when the schema changes -- see the map"]
fn regenerate_the_fixtures() {
    // ⛔ THE ONLY WRITER OF `gui/schema/fixtures/`, and it writes the map in the same pass, so
    // bytes and map cannot drift. `#[ignore]` with a reason, as every ignored test of this
    // workspace does by convention: the gate must not rewrite artefacts it is checking.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../gui/schema/fixtures");
    std::fs::create_dir_all(&root).expect("create the fixtures directory");
    let mut map = String::from(
        "# The map of the ipc fixtures -- \u{a7}4 of the sub-project 2 design.\n\
         # REGENERATED, not frozen: when the schema changes, run\n\
         #   cargo test --locked -p kernel --test ipc_wire -- --ignored regenerate_the_fixtures\n\
         # and commit what changes. The frozen bytes of ADR-0036 are the OPPOSITE artefact and\n\
         # live in crates/kernel/tests/frozen/.\n\
         # Produced by regenerate_the_fixtures in ../../../crates/kernel/tests/ipc_wire.rs.\n\n",
    );
    for (index, message) in stamp_set().into_iter().enumerate() {
        let name = variant_name(&message);
        let bytes = message.encode().expect("encode");
        std::fs::write(root.join(format!("{index:02}-{name}.bin")), &bytes).expect("write");
        // ⛔ THE EXPECTED VALUE LEAVES THE SAME PASS AS THE BYTES, and that is the whole point:
        // two writers would let a fixture's bytes and its expected value drift apart, and the
        // drift would be invisible because each half would still be internally consistent.
        std::fs::write(
            root.join(format!("{index:02}-{name}.json")),
            format!("{}\n", variant_json(&message)),
        )
        .expect("write the expected value");
        map.push_str(&format!(
            "{index:02} {name} {len} bytes\n    {message:?}\n",
            len = bytes.len()
        ));
    }
    map.push_str(&format!("\nstamp {:#018x}\n", build_stamp().get()));
    std::fs::write(root.join("ipc_v1.map"), map).expect("write the map");
}

/// The name a fixture file carries. ⚠️ AN EXHAUSTIVE `match` AND NOT `{:?}` TRUNCATED: a
/// variant added must make this a compile error, the same reason `Operation::is_write` gives.
fn variant_name(message: &IpcMessage) -> &'static str {
    match message {
        IpcMessage::Hello(_) => "hello",
        IpcMessage::Accepted(_) => "accepted",
        IpcMessage::StaleBuild(_) => "stale-build",
        IpcMessage::Degradation(_) => "degradation",
        IpcMessage::Policy(_) => "policy",
        IpcMessage::Invoke(_) => "invoke",
        IpcMessage::PermissionRequired(_) => "permission-required",
        IpcMessage::Approve { .. } => "approve",
        IpcMessage::Token { .. } => "token",
        IpcMessage::Layout(_) => "layout",
        IpcMessage::SaveLayout(_) => "save-layout",
        IpcMessage::Steps(_) => "steps",
        IpcMessage::Request(_) => "request",
        IpcMessage::Verdict(_) => "verdict",
    }
}

/// RULE 2 OF `variant_json` WHERE A `match` ARM HAS ALREADY TAKEN THE FIELDS APART: one member per
/// binding, and its key is `stringify!` of the binding -- which IS the field's name, because the
/// arm binds it by name. A field renamed in Rust stops the arm until the binding is renamed, and
/// renaming the binding renames the key: nothing is left beside it to forget.
macro_rules! json_members {
    ($($field:ident: $encode:expr),+ $(,)?) => {
        [$(format!("\"{}\":{}", stringify!($field), ($encode)($field))),+].join(",")
    };
}

/// RULE 2 OF `variant_json` FOR A STRUCT: ONE exhaustive destructuring, with NO `..`, whose
/// bindings are the object's members -- so a field renamed, added or removed in Rust stops the
/// generator HERE (`E0026`, `E0027`) until the call names it, and naming it writes it, key and
/// value. The struct comes in by its bare name: every one the wire has is imported at the top.
macro_rules! json_struct {
    ($value:expr, $struct:ident { $($field:ident: $encode:expr),+ $(,)? }) => {{
        let $struct { $($field),+ } = $value;
        format!("{{{}}}", json_members!($($field: $encode),+))
    }};
}

/// The value a fixture carries, as JSON, for the sub-project 2 SPA to compare its own types
/// against. ⛔ HAND-WRITTEN AND NOT A DEPENDENCY, for `build_stamp`'s two reasons plus one of
/// its own: `serde_json` would want `Serialize` derives on SHIPPED wire types, or a mirror of
/// them here -- and a mirror is the second definition of the schema that ADR-0037 refuses.
///
/// ⛔ TWO RULES, NEITHER COSMETIC.
///   1. EVERY `u64` IS A DECIMAL STRING. `BuildStamp` is FNV-1a over the whole set and passes
///      `Number.MAX_SAFE_INTEGER` as a matter of course; as a JSON number the reader would
///      round it, and the fixture would compare equal to a value it does not hold.
///   2. FIELD NAMES ARE RUST'S, verbatim and `snake_case`. Renaming them to the web's taste
///      would be a translation table, which is a second definition that drifts in silence --
///      the very failure these fixtures exist to prevent. ⛔ HELD BY THE COMPILER AND NOT BY A
///      LITERAL (AUD-726 of the audit of 2026-09-30): every named field reaches the JSON through
///      `json_struct!` or `json_members!`, whose key IS the field's identifier. Measured on
///      2026-10-02, when the keys were literals: `DegradationReport::vram_exhausted` renamed in
///      the kernel, the two lines of this file that stopped compiling renamed with it, and the
///      whole bench GREEN with `"vram_exhausted"` still in the JSON; now the same rename reaches
///      the JSON, and `the_committed_fixtures_match_the_schema` is red until it is regenerated.
///
/// ⚠️ WHAT RULE 2 DOES NOT REACH, declared: the WORDS -- the tags `kind`, `state` and `verdict`,
/// the names of the unit variants, and the keys of a tuple variant's payload (`value`, `bytes`,
/// `grace_ms`) -- are no field of Rust. They are written here once, each beside an exhaustive
/// `match`, and a variant RENAMED in Rust stops that `match` but keeps its old word unless the
/// word is renamed with it.
///
/// ⚠️ THE `match` IS EXHAUSTIVE for the reason `variant_name` gives: a variant added must be a
/// compile error here, not a fixture that quietly never appears. ⛔ AND AN UNUSED BINDING IS AN
/// ERROR HERE: a field added to a variant with named fields stops its arm until the pattern
/// names it, and a name bound and never written into the JSON is the half of that fix that a
/// warning would let pass.
#[deny(unused_variables)]
fn variant_json(message: &IpcMessage) -> String {
    match message {
        IpcMessage::Hello(stamp) => format!(r#"{{"kind":"Hello","value":"{}"}}"#, stamp.get()),
        IpcMessage::Accepted(protection) => format!(
            r#"{{"kind":"Accepted","value":"{}"}}"#,
            match protection {
                Protection::AsSystemAccount => "AsSystemAccount",
            }
        ),
        IpcMessage::StaleBuild(stamp) => {
            format!(r#"{{"kind":"StaleBuild","value":"{}"}}"#, stamp.get())
        }
        IpcMessage::Degradation(report) => format!(
            r#"{{"kind":"Degradation","value":{}}}"#,
            json_struct!(report, DegradationReport {
                vram_exhausted: json_bool,
                routing_degraded: json_bool,
            })
        ),
        IpcMessage::Policy(report) => format!(
            r#"{{"kind":"Policy","value":{}}}"#,
            json_struct!(report, PolicyReport {
                policy: json_policy_name,
                allocated: json_mib,
                total: json_mib,
            })
        ),
        IpcMessage::Invoke(call) => format!(r#"{{"kind":"Invoke","value":{}}}"#, json_call(call)),
        IpcMessage::PermissionRequired(triple) => format!(
            r#"{{"kind":"PermissionRequired","value":{}}}"#,
            json_triple(triple)
        ),
        IpcMessage::Approve { triple, call } => format!(
            r#"{{"kind":"Approve",{}}}"#,
            json_members!(triple: json_triple, call: json_call)
        ),
        IpcMessage::Token { text, provenance } => format!(
            r#"{{"kind":"Token",{}}}"#,
            json_members!(text: json_text, provenance: json_provenance)
        ),
        IpcMessage::Layout(state) => format!(
            r#"{{"kind":"Layout","value":{}}}"#,
            match state {
                LayoutState::Package(bytes) =>
                    format!(r#"{{"state":"Package","bytes":{}}}"#, json_bytes(bytes)),
                LayoutState::Nothing => String::from(r#"{"state":"Nothing"}"#),
                LayoutState::Unavailable => String::from(r#"{"state":"Unavailable"}"#),
            }
        ),
        IpcMessage::SaveLayout(bytes) => {
            format!(r#"{{"kind":"SaveLayout","value":{}}}"#, json_bytes(bytes))
        }
        IpcMessage::Steps(steps) => format!(
            r#"{{"kind":"Steps","value":[{}]}}"#,
            steps
                .iter()
                .map(|summary| json_struct!(summary, StepSummary {
                    step: json_u64,
                    function: json_text,
                    done: json_bool,
                }))
                .collect::<Vec<String>>()
                .join(",")
        ),
        IpcMessage::Request(request) => format!(
            r#"{{"kind":"Request","value":{}}}"#,
            json_struct!(request, GrantRequest {
                reserved_vram: json_mib,
                compute_class: json_compute_class,
                preemption: json_preemption,
            })
        ),
        IpcMessage::Verdict(verdict) => format!(
            r#"{{"kind":"Verdict","value":{}}}"#,
            match verdict {
                Verdict::Granted => String::from(r#"{"verdict":"Granted"}"#),
                Verdict::Queued => String::from(r#"{"verdict":"Queued"}"#),
                Verdict::Refused { asked, ceiling } => format!(
                    r#"{{"verdict":"Refused",{}}}"#,
                    json_members!(asked: json_mib, ceiling: json_mib)
                ),
            }
        ),
    }
}

/// The encoders of the members, one per Rust type, each taking its value by reference, as a
/// destructuring hands it over. ⛔ RULE 1 LIVES IN `json_u64`: a `u64` is a decimal STRING.
fn json_u64(value: &u64) -> String {
    format!("\"{value}\"")
}

fn json_mib(mib: &Mib) -> String {
    json_u64(&mib.get())
}

fn json_bool(flag: &bool) -> String {
    flag.to_string()
}

fn json_policy_name(policy: &PolicyName) -> String {
    json_text(match policy {
        PolicyName::Remote => "Remote",
        PolicyName::Local => "Local",
    })
}

fn json_access(operation: &Access) -> String {
    json_text(match operation {
        Access::Read => "Read",
        Access::Write => "Write",
    })
}

fn json_provenance(provenance: &Provenance) -> String {
    json_text(match provenance {
        Provenance::Trusted => "Trusted",
        Provenance::Untrusted => "Untrusted",
    })
}

fn json_compute_class(class: &ComputeClass) -> String {
    json_text(match class {
        ComputeClass::Realtime => "Realtime",
        ComputeClass::Interactive => "Interactive",
        ComputeClass::Batch => "Batch",
    })
}

fn json_preemption(preemption: &Preemption) -> String {
    match preemption {
        Preemption::Never => String::from(r#"{"kind":"Never"}"#),
        Preemption::After(grace) => {
            format!(r#"{{"kind":"After","grace_ms":{}}}"#, json_u64(&grace.get()))
        }
    }
}

/// A JSON string, escaped. ⚠️ THE TEXT IN THE CANONICAL SET IS OURS, but an escaper that is
/// only correct for today's literals is a trap laid for the first variant that carries a quote.
fn json_text(text: &str) -> String {
    let mut out = String::from("\"");
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other if (other as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", other as u32)),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

/// Bytes as an array of numbers. ⚠️ NOT base64 and NOT hex: a byte is 0..=255, which a JSON
/// number holds exactly, so the reading side needs no decoder and cannot decode it wrongly.
fn json_bytes(bytes: &[u8]) -> String {
    let numbers: Vec<String> = bytes.iter().map(|byte| byte.to_string()).collect();
    format!("[{}]", numbers.join(","))
}

fn json_triple(triple: &Triple) -> String {
    json_struct!(triple, Triple {
        tool: json_text,
        resource: json_text,
        operation: json_access,
    })
}

fn json_call(call: &Call) -> String {
    json_struct!(call, Call {
        function: json_text,
        argument: json_text,
    })
}

#[test]
fn the_escaper_covers_the_characters_no_fixture_carries() {
    // ⛔ THE BRANCHES OF json_text THE CANONICAL SET NEVER REACHES. Its texts are "ciao",
    // "arbiter", "policy", "arbiter.set_policy" and "local": not one quote, backslash or
    // control character among them, so every escaping branch ships UNEXERCISED. The
    // json.load of step 6 does not cover them either: A FILE CAN BE VALID JSON AND HOLD
    // THE WRONG TEXT. The doc of json_text names this trap by name; until this probe it
    // was a warning nothing held.
    //
    // ⚠️ THE ORACLE IS A LITERAL AND NOT A SECOND ESCAPER. Re-deriving the expectation with
    // the same rules would be green under a mistake copied into both halves -- the second
    // definition ADR-0037 refuses. A literal is a VALUE: wrong only if written wrong.
    assert_eq!(
        json_text("a\"b\\c\nd\re\tf\u{0}g\u{1f}h"),
        "\"a\\\"b\\\\c\\nd\\re\\tf\\u0000g\\u001fh\"",
        "every escaping branch of json_text, in one string"
    );
    // ⚠️ AND THE OTHER DIRECTION, which is the one that gets forgotten: text needing NO
    // escaping comes back unchanged inside its quotes. Without it an escaper that mangled
    // ordinary text -- or escaped everything -- would still pass the assertion above.
    assert_eq!(json_text("caffè ☕"), "\"caffè ☕\"");
}

#[test]
fn every_u64_reaches_the_json_as_a_decimal_string() {
    // ⛔ RULE 1 OF `variant_json` HELD BY SOMETHING AT LAST -- I-1 of the review of 2026-09-17.
    // The doc above states in capitals that EVERY `u64` IS A DECIMAL STRING, and until this
    // probe nothing could see it. The three checks that existed cannot decide: `json.load`
    // proves SYNTAX, and a bare JSON number is valid syntax; the closing line that inspects the
    // type reads `00-hello.json` ALONE; and `the_committed_fixtures_match_the_schema` compares
    // the committed file against `variant_json`, THE VERY FUNCTION THAT WROTE IT -- an oracle
    // only until someone regenerates, which is what its own red message prescribes. Measured:
    // with the `Steps` arm emitting a bare number and the fixtures regenerated, the bench was
    // GREEN and all three closing lines stayed ticked.
    //
    // ⚠️ THE ORACLE IS A LITERAL, the shape E12 settled on for the escaper: a VALUE, not a
    // second copy of the rules, and so wrong only if written wrong.
    //
    // ⛔ AND IT IS EVERY FIXTURE THAT CARRIES A `u64`, NOT A SAMPLE. One left out is one the
    // reading side rounds: `Number.MAX_SAFE_INTEGER` is 2^53-1 and `02-stale-build` alone
    // carries 18364758544493064720, so a bare number there would reach the gui ALREADY WRONG
    // and compare equal to itself (gotcha #51, which the step-6 prose cites of itself).
    //
    // ⚠️ AND THE LIMIT, DECLARED RATHER THAN HIDDEN: this list is kept BY HAND. It covers every
    // message of the set that renders a `u64` today, and nothing makes it grow on its own -- a
    // message added tomorrow that carries one would come back green, UNWATCHED, while the name
    // still says "every". WHOEVER ADDS A MESSAGE WITH A `u64` TO THE SET -- a variant, or one of
    // the nested ones at its end -- ADDS ITS LINE HERE. A probe that walked the values instead
    // would have to know which fields are `u64`, which is the schema stated a second time -- what
    // ADR-0037 refuses and what the literal exists to avoid.
    let set = stamp_set();
    let rule = "rule 1: every u64 is a decimal STRING";
    assert_eq!(
        variant_json(&set[0]),
        r#"{"kind":"Hello","value":"81985529216486895"}"#,
        "{rule}"
    );
    assert_eq!(
        variant_json(&set[2]),
        r#"{"kind":"StaleBuild","value":"18364758544493064720"}"#,
        "{rule}"
    );
    assert_eq!(
        variant_json(&set[4]),
        r#"{"kind":"Policy","value":{"policy":"Remote","allocated":"12288","total":"16384"}}"#,
        "{rule}"
    );
    assert_eq!(
        variant_json(&set[11]),
        r#"{"kind":"Steps","value":[{"step":"42","function":"arbiter.set_policy","done":true}]}"#,
        "{rule}"
    );
    assert_eq!(
        variant_json(&set[12]),
        r#"{"kind":"Request","value":{"reserved_vram":"2048","compute_class":"Interactive","preemption":{"kind":"After","grace_ms":"500"}}}"#,
        "{rule}"
    );
    assert_eq!(
        variant_json(&set[13]),
        r#"{"kind":"Verdict","value":{"verdict":"Refused","asked":"4096","ceiling":"1024"}}"#,
        "{rule}"
    );
    assert_eq!(
        variant_json(&set[14]),
        r#"{"kind":"Policy","value":{"policy":"Local","allocated":"14336","total":"16384"}}"#,
        "{rule}"
    );
    assert_eq!(
        variant_json(&set[19]),
        r#"{"kind":"Request","value":{"reserved_vram":"1024","compute_class":"Realtime","preemption":{"kind":"After","grace_ms":"250"}}}"#,
        "{rule}"
    );
    assert_eq!(
        variant_json(&set[20]),
        r#"{"kind":"Request","value":{"reserved_vram":"4096","compute_class":"Batch","preemption":{"kind":"After","grace_ms":"750"}}}"#,
        "{rule}"
    );
    assert_eq!(
        variant_json(&set[21]),
        r#"{"kind":"Request","value":{"reserved_vram":"512","compute_class":"Interactive","preemption":{"kind":"Never"}}}"#,
        "{rule}"
    );
}

#[test]
fn the_committed_fixtures_match_the_schema() {
    // ⛔ THE CHECK THE GATE RUNS. A schema changed without regenerating is RED here, and the
    // message says what to do rather than leaving the reader to work it out -- because the
    // right answer is to regenerate, which is exactly the wrong answer for `frozen_bytes.rs`.
    //
    // ⚠️ READ AT RUN TIME AND NOT `include_bytes!`, and the difference from `frozen_bytes.rs`
    // is the artefact, not an oversight: a frozen record is an ORACLE that must enter the
    // binary, while these are regenerable and a MISSING one has to be a red rather than a
    // compile error the generator itself could not fix.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../gui/schema/fixtures");
    let mut wrong = Vec::new();
    for (index, message) in stamp_set().into_iter().enumerate() {
        let name = variant_name(&message);
        let path = root.join(format!("{index:02}-{name}.bin"));
        let expected = message.encode().expect("encode");
        match std::fs::read(&path) {
            Ok(found) if found == expected => {}
            Ok(_) => wrong.push(format!("  {index:02}-{name}.bin: different bytes")),
            Err(error) => wrong.push(format!("  {index:02}-{name}.bin: {error}")),
        }
        // ⛔ THE EXPECTED VALUE IS CHECKED TOO, and not because it could rot on its own -- it
        // cannot, one pass writes both. It is checked because a set regenerated by an OLDER
        // build and then committed leaves bytes and value agreeing with EACH OTHER and
        // disagreeing with the schema; only comparing both against today's `stamp_set` sees it.
        let expected_json = format!("{}\n", variant_json(&message));
        match std::fs::read_to_string(root.join(format!("{index:02}-{name}.json"))) {
            Ok(found) if found == expected_json => {}
            Ok(_) => wrong.push(format!("  {index:02}-{name}.json: different value")),
            Err(error) => wrong.push(format!("  {index:02}-{name}.json: {error}")),
        }
    }
    // ⛔ THE MAP IS CHECKED TOO, on its last line. D52 makes the SPA read the stamp from there,
    // so a stale map is a SPA that sends a stamp the core no longer computes -- `StaleBuild` at
    // the handshake -- with this bench GREEN. Measured before this check existed: with mutation
    // G8 applied, twelve passed while the map still said the old stamp.
    let stamp_line = format!("stamp {:#018x}", build_stamp().get());
    match std::fs::read_to_string(root.join("ipc_v1.map")) {
        Ok(map) if map.lines().rev().find(|line| !line.trim().is_empty()) == Some(stamp_line.as_str()) => {}
        Ok(_) => wrong.push("  ipc_v1.map: the last line is not today's stamp".to_string()),
        Err(error) => wrong.push(format!("  ipc_v1.map: {error}")),
    }
    let extra: Vec<String> = std::fs::read_dir(&root)
        .expect("read the fixtures directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".bin") || name.ends_with(".json"))
        .filter(|name| {
            !stamp_set().iter().enumerate().any(|(i, m)| {
                *name == format!("{i:02}-{}.bin", variant_name(m))
                    || *name == format!("{i:02}-{}.json", variant_name(m))
            })
        })
        .collect();
    // ⚠️ THE BULLET GOES INSIDE THE JOIN, the shape `wrong` already has -- M-2 of the review of
    // 2026-09-17. With it outside, an EMPTY `extra` still printed a line made of two spaces: a
    // bullet inviting the reader to look for a name that is not there, under a message whose
    // whole point is to say "what to do rather than leaving the reader to work it out".
    let bulleted: Vec<String> = extra.iter().map(|name| format!("  {name}")).collect();
    assert!(
        wrong.is_empty() && extra.is_empty(),
        "the committed fixtures do not match the schema. REGENERATE them:\n  \
         cargo test --locked -p kernel --test ipc_wire -- --ignored regenerate_the_fixtures\n\
         mismatched:\n{}\nleft over:\n{}",
        wrong.join("\n"),
        bulleted.join("\n")
    );
}

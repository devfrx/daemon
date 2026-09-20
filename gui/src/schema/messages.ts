/**
 * ⛔ EVERY `u64` CROSSES AS A DECIMAL STRING, and this alias is where that is said once.
 * `BuildStamp` is FNV-1a over the whole canonical set, so it passes `Number.MAX_SAFE_INTEGER`
 * as a matter of course: as a JSON number the reader would round it and then compare two
 * equally rounded values, which is green and false. Arithmetic on one of these goes through
 * `BigInt`, never through `Number`.
 */
export type U64 = string;

export type Protection = "AsSystemAccount";
export type PolicyName = "Remote" | "Local";
export type Access = "Read" | "Write";
export type Provenance = "Trusted" | "Untrusted";
export type ComputeClass = "Realtime" | "Interactive" | "Batch";

export interface DegradationReport {
  vram_exhausted: boolean;
  routing_degraded: boolean;
}

export interface PolicyReport {
  policy: PolicyName;
  allocated: U64;
  total: U64;
}

export interface Triple {
  tool: string;
  resource: string;
  operation: Access;
}

export interface Call {
  function: string;
  argument: string;
}

export interface StepSummary {
  step: U64;
  function: string;
  done: boolean;
}

export type LayoutState =
  | { state: "Package"; bytes: number[] }
  | { state: "Nothing" }
  | { state: "Unavailable" };

export type Preemption = { kind: "Never" } | { kind: "After"; grace_ms: U64 };

export interface GrantRequest {
  reserved_vram: U64;
  compute_class: ComputeClass;
  preemption: Preemption;
}

export type Verdict =
  | { verdict: "Granted" }
  | { verdict: "Queued" }
  | { verdict: "Refused"; asked: U64; ceiling: U64 };

/**
 * One message on the `ipc` wire, mirroring `kernel::wire::ipc::IpcMessage`.
 *
 * ⛔ ONE UNION FOR BOTH DIRECTIONS, as I4 has it on the Rust side. Which four the gui may send
 * is not a second list: `OutboundMessage` in `../transport/bridge` derives them from this one.
 */
export type IpcMessage =
  | { kind: "Hello"; value: U64 }
  | { kind: "Accepted"; value: Protection }
  | { kind: "StaleBuild"; value: U64 }
  | { kind: "Degradation"; value: DegradationReport }
  | { kind: "Policy"; value: PolicyReport }
  | { kind: "Invoke"; value: Call }
  | { kind: "PermissionRequired"; value: Triple }
  | { kind: "Approve"; triple: Triple; call: Call }
  | { kind: "Token"; text: string; provenance: Provenance }
  | { kind: "Layout"; value: LayoutState }
  | { kind: "SaveLayout"; value: number[] }
  | { kind: "Steps"; value: StepSummary[] }
  | { kind: "Request"; value: GrantRequest }
  | { kind: "Verdict"; value: Verdict };

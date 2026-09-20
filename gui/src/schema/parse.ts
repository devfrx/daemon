import type {
  Access, Call, ComputeClass, DegradationReport, GrantRequest, IpcMessage, LayoutState,
  PolicyName, PolicyReport, Preemption, Protection, Provenance, StepSummary, Triple, U64, Verdict,
} from "./messages";

/**
 * What a malformed message raises. ⚠️ AN ERROR AND NOT A `null`: a caller that forgets to check
 * a `null` carries on with a hole, while one that forgets a `catch` stops loudly.
 */
export class SchemaError extends Error {}

function fail(where: string, raw: unknown): never {
  throw new SchemaError(`${where}: unexpected ${JSON.stringify(raw)}`);
}

function object(raw: unknown, where: string): Record<string, unknown> {
  if (typeof raw !== "object" || raw === null || Array.isArray(raw)) fail(where, raw);
  return raw as Record<string, unknown>;
}

function text(raw: unknown, where: string): string {
  if (typeof raw !== "string") fail(where, raw);
  return raw;
}

function flag(raw: unknown, where: string): boolean {
  if (typeof raw !== "boolean") fail(where, raw);
  return raw;
}

/**
 * ⛔ A `u64` IS A STRING OF DIGITS AND IS CHECKED AS ONE. Accepting a number here would let
 * exactly the rounding D35 exists to prevent back in through the reader.
 */
function u64(raw: unknown, where: string): U64 {
  if (typeof raw !== "string" || !/^[0-9]+$/.test(raw)) fail(where, raw);
  return raw;
}

function among<T extends string>(raw: unknown, allowed: readonly T[], where: string): T {
  const value = text(raw, where);
  if (!allowed.includes(value as T)) fail(where, raw);
  return value as T;
}

function bytes(raw: unknown, where: string): number[] {
  if (!Array.isArray(raw)) fail(where, raw);
  return raw.map((byte, index) => {
    if (typeof byte !== "number" || !Number.isInteger(byte) || byte < 0 || byte > 255) {
      fail(`${where}[${index}]`, byte);
    }
    return byte;
  });
}

function triple(raw: unknown, where: string): Triple {
  const value = object(raw, where);
  return {
    tool: text(value.tool, `${where}.tool`),
    resource: text(value.resource, `${where}.resource`),
    operation: among<Access>(value.operation, ["Read", "Write"], `${where}.operation`),
  };
}

function call(raw: unknown, where: string): Call {
  const value = object(raw, where);
  return {
    function: text(value.function, `${where}.function`),
    argument: text(value.argument, `${where}.argument`),
  };
}

function layoutState(raw: unknown, where: string): LayoutState {
  const value = object(raw, where);
  const state = among(value.state, ["Package", "Nothing", "Unavailable"] as const, `${where}.state`);
  if (state === "Package") return { state, bytes: bytes(value.bytes, `${where}.bytes`) };
  return { state };
}

function preemption(raw: unknown, where: string): Preemption {
  const value = object(raw, where);
  const kind = among(value.kind, ["Never", "After"] as const, `${where}.kind`);
  if (kind === "After") return { kind, grace_ms: u64(value.grace_ms, `${where}.grace_ms`) };
  return { kind };
}

function grantRequest(raw: unknown, where: string): GrantRequest {
  const value = object(raw, where);
  return {
    reserved_vram: u64(value.reserved_vram, `${where}.reserved_vram`),
    compute_class: among<ComputeClass>(
      value.compute_class,
      ["Realtime", "Interactive", "Batch"],
      `${where}.compute_class`,
    ),
    preemption: preemption(value.preemption, `${where}.preemption`),
  };
}

function verdict(raw: unknown, where: string): Verdict {
  const value = object(raw, where);
  const which = among(value.verdict, ["Granted", "Queued", "Refused"] as const, `${where}.verdict`);
  if (which === "Refused") {
    return {
      verdict: which,
      asked: u64(value.asked, `${where}.asked`),
      ceiling: u64(value.ceiling, `${where}.ceiling`),
    };
  }
  return { verdict: which };
}

type Parser<K extends IpcMessage["kind"]> =
  (raw: Record<string, unknown>) => Extract<IpcMessage, { kind: K }>;

/**
 * ⛔ A MAPPED TYPE AND NOT A `switch`, and it buys BOTH directions with one declaration.
 * Forward: a variant added to `IpcMessage` and forgotten here is a COMPILE error -- level 1,
 * not a probe. Backward: `Object.keys` gives the list of kinds AT RUN TIME, which a TypeScript
 * union cannot be asked for because it is erased, and the fixture probe needs exactly that list.
 */
const PARSERS: { [K in IpcMessage["kind"]]: Parser<K> } = {
  Hello: (raw) => ({ kind: "Hello", value: u64(raw.value, "Hello.value") }),
  Accepted: (raw) => ({
    kind: "Accepted",
    value: among<Protection>(raw.value, ["AsSystemAccount"], "Accepted.value"),
  }),
  StaleBuild: (raw) => ({ kind: "StaleBuild", value: u64(raw.value, "StaleBuild.value") }),
  Degradation: (raw) => {
    const value = object(raw.value, "Degradation.value");
    const report: DegradationReport = {
      vram_exhausted: flag(value.vram_exhausted, "Degradation.value.vram_exhausted"),
      routing_degraded: flag(value.routing_degraded, "Degradation.value.routing_degraded"),
    };
    return { kind: "Degradation", value: report };
  },
  Policy: (raw) => {
    const value = object(raw.value, "Policy.value");
    const report: PolicyReport = {
      policy: among<PolicyName>(value.policy, ["Remote", "Local"], "Policy.value.policy"),
      allocated: u64(value.allocated, "Policy.value.allocated"),
      total: u64(value.total, "Policy.value.total"),
    };
    return { kind: "Policy", value: report };
  },
  Invoke: (raw) => ({ kind: "Invoke", value: call(raw.value, "Invoke.value") }),
  PermissionRequired: (raw) => ({
    kind: "PermissionRequired",
    value: triple(raw.value, "PermissionRequired.value"),
  }),
  Approve: (raw) => ({
    kind: "Approve",
    triple: triple(raw.triple, "Approve.triple"),
    call: call(raw.call, "Approve.call"),
  }),
  Token: (raw) => ({
    kind: "Token",
    text: text(raw.text, "Token.text"),
    provenance: among<Provenance>(raw.provenance, ["Trusted", "Untrusted"], "Token.provenance"),
  }),
  Layout: (raw) => ({ kind: "Layout", value: layoutState(raw.value, "Layout.value") }),
  SaveLayout: (raw) => ({ kind: "SaveLayout", value: bytes(raw.value, "SaveLayout.value") }),
  Steps: (raw) => {
    if (!Array.isArray(raw.value)) fail("Steps.value", raw.value);
    const summaries: StepSummary[] = raw.value.map((entry, index) => {
      const value = object(entry, `Steps.value[${index}]`);
      return {
        step: u64(value.step, `Steps.value[${index}].step`),
        function: text(value.function, `Steps.value[${index}].function`),
        done: flag(value.done, `Steps.value[${index}].done`),
      };
    });
    return { kind: "Steps", value: summaries };
  },
  Request: (raw) => ({ kind: "Request", value: grantRequest(raw.value, "Request.value") }),
  Verdict: (raw) => ({ kind: "Verdict", value: verdict(raw.value, "Verdict.value") }),
};

/** Every kind the union declares, AT RUN TIME. See the note on `PARSERS`. */
export const MESSAGE_KINDS: readonly IpcMessage["kind"][] =
  Object.keys(PARSERS) as IpcMessage["kind"][];

export function parseIpcMessage(raw: unknown): IpcMessage {
  const value = object(raw, "message");
  const kind = text(value.kind, "message.kind");
  if (!Object.prototype.hasOwnProperty.call(PARSERS, kind)) fail("message.kind", kind);
  const parse = PARSERS[kind as IpcMessage["kind"]] as Parser<IpcMessage["kind"]>;
  return parse(value);
}

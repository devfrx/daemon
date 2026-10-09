import { defineStore } from "pinia";
import { ref } from "vue";

import type {
  DegradationReport,
  IpcMessage,
  PolicyReport,
  StepSummary,
  Triple,
  Verdict,
} from "../schema/messages";

/**
 * What the core has told us about itself. ⛔ PRESENTATION ONLY (I1): nothing here is authoritative
 * and nothing here is persisted. After a restart the core says three of the fields again at the
 * welcome -- the degradation, the policy, the steps (`greet` in `crates/kernel/src/serving.rs`,
 * sequence 1) --; `pending` and `lastVerdict` it says once, when they happen. And `pending` is the
 * one field the gui clears itself: `settled`, once the confirmation window has answered (AUD-1089
 * of the audit of 2026-09-30).
 *
 * ⚠️ THE FIELDS START AT `null` AND NOT AT A MADE-UP DEFAULT: "we have not been told" and "the
 * core says no degradation" are different, and a component that cannot tell them apart shows a
 * green light to a user who is not connected. ⛔ BUT `steps`, WHICH STARTS EMPTY (AUD-2071 of the
 * audit of 2026-09-30): an empty list is the empty state of a module with no data, the C13-2 recall
 * of §6a of the sub-project 2 design -- so Passi says it has no step before the welcome as it does
 * when the core sends none, and no light it shows depends on telling the two apart.
 */
export const useCore = defineStore("core", () => {
  const degradation = ref<DegradationReport | null>(null);
  const policy = ref<PolicyReport | null>(null);
  const steps = ref<StepSummary[]>([]);
  const pending = ref<Triple | null>(null);
  const lastVerdict = ref<Verdict | null>(null);

  function receive(message: IpcMessage): void {
    switch (message.kind) {
      case "Degradation":
        degradation.value = message.value;
        break;
      case "Policy":
        policy.value = message.value;
        break;
      case "Steps":
        // ⛔ REPLACED AND NOT APPENDED: §6.1.4 has the core send the piece that CHANGED, and the
        // step list is sent whole -- at the welcome and after every invocation. Appending would
        // double every step across a reconnection.
        steps.value = message.value;
        break;
      case "PermissionRequired":
        pending.value = message.value;
        break;
      case "Verdict":
        lastVerdict.value = message.value;
        break;
      default:
        break;
    }
  }

  /** The confirmation window's answer clears it, a yes or a no (`approve` and `refuse` in `invoke.ts`); the frame only counts it. */
  function settled(): void {
    pending.value = null;
  }

  return { degradation, policy, steps, pending, lastVerdict, receive, settled };
});

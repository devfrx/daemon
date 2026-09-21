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
 * and nothing here is persisted -- every field is the last thing the core said, and after a
 * restart the core says it again at the welcome (sequence 1).
 *
 * ⚠️ THE FIELDS START AT `null` AND NOT AT A MADE-UP DEFAULT: "we have not been told" and "the
 * core says no degradation" are different, and a component that cannot tell them apart shows a
 * green light to a user who is not connected.
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

  /** Task 14's confirmation window clears it after `Approve`; the frame only counts it. */
  function settled(): void {
    pending.value = null;
  }

  return { degradation, policy, steps, pending, lastVerdict, receive, settled };
});

import { defineStore } from "pinia";
import { ref } from "vue";

import type { Call, IpcMessage, Triple } from "../schema/messages";
import type { Bridge } from "../transport/bridge";

import { useCore } from "./core";

/**
 * The invocation in flight, and what this window approved.
 *
 * ⛔ `Approve` MUST CARRY THE CALL (decision 21 of the north star): the core opens step A only when
 * the invocation arrives WITH the permission, and the gui does not resend `Invoke`. Nothing in the
 * stores of task 13 keeps the last `Invoke` sent -- `core.pending` holds the triple the core asked
 * for -- so this is where the call waits (P-92).
 *
 * ⚠️ `approved` IS PRESENTATION (I1): it lists what THIS window said yes to, in this session, and
 * after a restart it is empty. The core's own list is a message that row 1 of the Permessi table
 * marks as deduced and gives to sub-project 3.
 */
export const useInvoke = defineStore("invoke", () => {
  const core = useCore();
  const inFlight = ref<Call | null>(null);
  const approved = ref<Triple[]>([]);
  let wire: Bridge | null = null;

  function attach(bridge: Bridge): void {
    wire = bridge;
  }

  /** The click: one `Invoke`, and the call is remembered. */
  function send(call: Call): void {
    inFlight.value = call;
    wire?.send({ kind: "Invoke", value: call });
  }

  /**
   * The "yes" of the confirmation window. ⛔ PAIRS THE TRIPLE THE CORE ASKED FOR WITH THE CALL IN
   * FLIGHT, and there is nothing to approve without both: a `PermissionRequired` that follows no
   * `Invoke` of ours is a shape the core never produces -- the registry only ever answers one.
   * Returns whether an `Approve` went out.
   */
  function approve(): boolean {
    const triple = core.pending;
    const call = inFlight.value;
    if (triple === null || call === null) return false;
    wire?.send({ kind: "Approve", triple, call });
    approved.value.push(triple);
    inFlight.value = null;
    core.settled();
    return true;
  }

  /** The "no": NOTHING IS SENT. The core keeps nothing pending -- the registry answered
   * `PermissionRequired` and forgot (§5 of the sub-project 2 design) -- so refusing is local. */
  function refuse(): void {
    inFlight.value = null;
    core.settled();
  }

  /** `Policy` is the piece that changes after an invocation landed (§6.1.4): whatever was in
   * flight has arrived, with or without a window in between. At the welcome nothing is in flight
   * and this is a no-op. */
  function receive(message: IpcMessage): void {
    if (message.kind === "Policy") inFlight.value = null;
  }

  return { inFlight, approved, attach, send, approve, refuse, receive };
});

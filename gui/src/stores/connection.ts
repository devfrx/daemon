import { defineStore } from "pinia";
import { ref } from "vue";

import type { IpcMessage, Protection, U64 } from "../schema/messages";
import { buildStamp } from "../schema/stamp";
import type { Bridge } from "../transport/bridge";

export type Phase = "waiting" | "connected" | "stale";

/**
 * ⛔ THE CONNECTION IS DERIVED, NOT DELIVERED (D48). "The core is not running" cannot be a
 * message -- a core that is not running sends nothing -- and the `Bridge` deliberately has no
 * `isConnected`: everything the shell knows how to do stays on its side of the seam. The SPA
 * sends `Hello` and is NOT connected until `Accepted` arrives. Same fact, seen from the side
 * that can see it.
 *
 * ⛔ AND THERE IS NO TIMEOUT. Telling "not running" from "slow to answer" would take a number
 * nobody has measured ("no invented threshold", SP-7), and the gui does the same thing in both
 * cases: offer `retry`, which resends `Hello`.
 */
export const useConnection = defineStore("connection", () => {
  const phase = ref<Phase>("waiting");
  const protection = ref<Protection | null>(null);
  const expected = ref<U64 | null>(null);
  let wire: Bridge | null = null;

  function attach(bridge: Bridge): void {
    wire = bridge;
  }

  function hello(): void {
    phase.value = "waiting";
    protection.value = null;
    expected.value = null;
    wire?.send({ kind: "Hello", value: buildStamp() });
  }

  /** ⚠️ The same thing as `hello`, under the name the band's button carries. Two names for one
   * act is worth it here: the template says what the user does, the wiring says what happens. */
  function retry(): void {
    hello();
  }

  function receive(message: IpcMessage): void {
    if (message.kind === "Accepted") {
      phase.value = "connected";
      protection.value = message.value;
    } else if (message.kind === "StaleBuild") {
      // ⛔ `StaleBuild` CARRIES THE EXPECTED STAMP, and the core stops listening to this client:
      // decision 22, and the port has no close. The gui declares it and does not proceed.
      phase.value = "stale";
      expected.value = message.value;
    }
  }

  return { phase, protection, expected, attach, hello, retry, receive };
});

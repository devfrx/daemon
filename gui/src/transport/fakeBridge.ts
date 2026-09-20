import type { IpcMessage } from "../schema/messages";
import { loadFixtures } from "../schema/fixtures";

import type { Bridge, Listener, OutboundMessage } from "./bridge";

export interface FakeBridge extends Bridge {
  /** What the SPA has sent, in order. ⚠️ The observable the probes assert on. */
  readonly sent: readonly OutboundMessage[];
  /** Delivers the fixture of that kind. Throws if the kernel never generated one. */
  deliver(kind: IpcMessage["kind"]): void;
  /** Delivers every fixture, in file order -- which is the canonical set's order. */
  deliverAll(): void;
}

/**
 * ⛔ IT REPLAYS THE FIXTURES AND INVENTS NOTHING, which is the whole reason it is worth having.
 * A fake that made up its own messages would let the SPA be built against a shape the core
 * never sends, and the divergence would surface in the shell -- that is, in the one place this
 * plan does not build.
 */
export function createFakeBridge(): FakeBridge {
  const fixtures = loadFixtures();
  const sent: OutboundMessage[] = [];
  const listeners = new Set<Listener>();

  const emit = (message: IpcMessage): void => {
    for (const listener of [...listeners]) listener(message);
  };

  return {
    sent,
    send(message) {
      sent.push(message);
    },
    listen(listener) {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
    deliver(kind) {
      const fixture = fixtures.find((candidate) => candidate.message.kind === kind);
      if (fixture === undefined) throw new Error(`no fixture for ${kind}`);
      emit(fixture.message);
    },
    deliverAll() {
      for (const fixture of fixtures) emit(fixture.message);
    },
  };
}

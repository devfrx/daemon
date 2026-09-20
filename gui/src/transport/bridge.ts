import type { IpcMessage } from "../schema/messages";

/**
 * The four the gui sends (§6a of the sub-project 2 design), stated ONCE.
 *
 * ⛔ DECLARED, NOT PINNED -- the four kinds below are WRITTEN BY HAND, and nothing holds them
 * against a rename. Measured on 2026-09-20: renaming a variant in `IpcMessage` and leaving
 * this filter alone leaves `npm run build` at EXIT=0, because `Extract` with a filter of
 * literals simply matches one member fewer and says nothing -- which is what a second
 * hand-written list would do, because that is what this is.
 *
 * ⚠️ ITS TRIGGER IS THE FIRST CALL SITE: the task that sends one of these -- task 14 of the
 * part-2 plan -- stops compiling on the kind that vanished. `Extract` still buys one thing,
 * and only that: the message TYPES come from `IpcMessage`, so a field that changes SHAPE is
 * a compile error here.
 */
export type OutboundMessage = Extract<
  IpcMessage,
  { kind: "Hello" | "Invoke" | "Approve" | "SaveLayout" }
>;

export type Listener = (message: IpcMessage) => void;

/**
 * The seam between the SPA and whatever shell it runs in.
 *
 * ⛔ THE SPA NEVER TOUCHES A SOCKET (§6a), and this interface is where that is enforced rather
 * than asked for: there is no connect, no address, no frame, no build stamp to check. Who
 * decodes lives UNDER this seam -- for Q1 of SP-8, the shell's Node main process -- and that is
 * what lets the SPA be developed and probed in a browser against `createFakeBridge` before any
 * shell exists.
 *
 * ⚠️ `listen` RETURNS ITS OWN UNSUBSCRIBE, rather than offering a `remove`: a listener that can
 * only be removed by handing back the same function reference is a leak waiting for the first
 * component that registers an inline arrow.
 */
export interface Bridge {
  send(message: OutboundMessage): void;
  listen(listener: Listener): () => void;
}

import type { IpcMessage } from "../schema/messages";

/**
 * The four the gui sends (§6a of the sub-project 2 design), stated ONCE.
 *
 * ⛔ THE FOUR KINDS BELOW ARE WRITTEN BY HAND, and `Extract` does not hold them. Measured on
 * 2026-09-20, in both shapes a change can take: rename a variant in `IpcMessage`, or add a
 * field to one, leave this filter alone -- `npm run build` stays at EXIT=0 and NOTHING in
 * this file goes red. A filter of literals matches one member fewer and says nothing, which
 * is what a second hand-written list would do, because that is what this is.
 *
 * ⚠️ WHERE IT DOES GO RED is wherever a caller CONSTRUCTS one of these -- and one such caller
 * already exists in this task: `fakeBridge.test.ts` sends `Hello`, so renaming `Hello` is red
 * TODAY. `Invoke`, `Approve` and `SaveLayout` get their first caller at tasks 13 and 14.
 *
 * ⛔ WHAT `Extract` BUYS, and only this: the message types are not RETYPED here. They are the
 * ones `IpcMessage` declares, so this file cannot become a second definition of them.
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

import type { IpcMessage } from "../schema/messages";

/**
 * The four the gui sends (§6a of the sub-project 2 design), stated ONCE.
 *
 * ⛔ DERIVED FROM `IpcMessage` AND NOT RETYPED: a variant renamed on the wire becomes a compile
 * error here, where a second hand-written list would simply stop matching and say nothing.
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

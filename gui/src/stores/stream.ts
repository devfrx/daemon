import { defineStore } from "pinia";
import { ref } from "vue";

import type { IpcMessage, Provenance } from "../schema/messages";

/** One rendered piece of the stream, and the provenance it carries (G13). */
export interface Block {
  /**
   * ⛔ WHO THE PIECE IS, FOR AS LONG AS IT LIVES, and not where it sits (AUD-539 of the audit of 2026-09-30): once KEEP
   * pieces are frozen every freeze drops the oldest, so a position names a different piece at each freeze. The Chat keys
   * its frozen pieces by this, and the drop takes one node out of its live region instead of rewriting every one.
   * Counted per store from 0, in the order the pieces open; a number is never given twice.
   */
  id: number;
  text: string;
  provenance: Provenance;
}

/**
 * ⛔ THE NUMBERS M4 WAS MEASURED WITH (D61): SP-8's chat tile froze the streamed text at this many
 * characters and kept this many frozen blocks, and P3 under real rendering was measured on THAT
 * tile -- so these are the values a measure sits behind, not thresholds invented here. The
 * message boundary itself arrives with sub-project 3: in sub-project 2 the fake core's faucet never
 * ends a message, and a text that only grows would render the whole stream on every token.
 */
export const FREEZE_AT = 4000;
export const KEEP = 20;

export const useStream = defineStore("stream", () => {
  const blocks = ref<Block[]>([]);
  const current = ref<Block | null>(null);
  // The id of the next piece that opens.
  let next = 0;

  function freeze(): void {
    if (current.value === null) return;
    blocks.value.push(current.value);
    if (blocks.value.length > KEEP) blocks.value.shift();
    current.value = null;
  }

  function receive(message: IpcMessage): void {
    if (message.kind !== "Token") return;
    // ⛔ A BLOCK HAS ONE PROVENANCE (G13): a token whose provenance differs from the open block's
    // closes it first, so no rendered piece ever carries a label that is true of only part of it.
    if (current.value !== null && current.value.provenance !== message.provenance) freeze();
    if (current.value === null) {
      current.value = { id: next, text: "", provenance: message.provenance };
      next += 1;
    }
    // ⚠️ CONCATENATED VERBATIM, no separator: the faucet's pieces carry their own spaces and
    // newlines (task 12, step 6), and a token is a piece of text, not a word.
    current.value.text += message.text;
    if (current.value.text.length >= FREEZE_AT) freeze();
  }

  return { blocks, current, receive };
});

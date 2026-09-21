import type { U64 } from "./messages";

/** Handed in by `define` in `vite.config.ts`, which reads the last line of `ipc_v1.map` in Node. */
declare const __BUILD_STAMP__: string;

/**
 * The build stamp the SPA presents in `Hello` (§6.1.2 of the kernel spec).
 *
 * ⛔ READ FROM `ipc_v1.map`, WHICH THE KERNEL WROTE, and not from the `hello` fixture: that
 * fixture carries the CANONICAL SET's arbitrary value, chosen so that no two encodings are
 * equal, and it is not the stamp. The map is the only place on this side of the wire where the
 * stamp has an honest source -- the TypeScript mirror of the types is hand-written, so a stamp
 * computed here would agree with itself and with nothing else.
 *
 * ⚠️ AND THAT IS WHAT MAKES THE HANDSHAKE A REAL CHECK: regenerate the schema without
 * rebuilding the SPA and the two stamps diverge, so the core answers `StaleBuild`. That is what
 * I4 bought by renouncing versioning.
 *
 * ⛔ THE READING HAPPENS IN `vite.config.ts` AND NOT HERE (D90): the browser must never ask the
 * dev server for a `.map` URL -- it would get a source map's `application/json` instead of a
 * module (R6-3). What arrives here is the decimal string, already through `BigInt`.
 */
export function buildStamp(): U64 {
  return __BUILD_STAMP__;
}

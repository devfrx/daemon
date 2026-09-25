// The custom command of `vite.config.ts`, as the probes in the browser see it ("Custom Commands", Vitest 4.1).
// `null` gives a feature back to the browser.
export {};

declare module "vitest/browser" {
  interface BrowserCommands {
    emulateMedia: (media: {
      colorScheme?: "light" | "dark" | null;
      reducedMotion?: "reduce" | "no-preference" | null;
      forcedColors?: "active" | "none" | null;
    }) => Promise<void>;
  }
}

# R2 — il compito 2 (il browser dei test) e il compito 3 (il kit)

- **Fetta:** righe **1173–1492** (compito 2) e **1493–2660** (compito 3) del piano; le righe 2 e 3 del *«Come si riprende»*.
- **Cartella di prova:** `/c/Users/zagor/AppData/Local/Temp/probe-ds2` (in Windows `C:\Users\zagor\AppData\Local\Temp\probe-ds2`).
- **Rapporto:** `C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\b8ece6f2-cc9f-436d-a426-c59258eeeb68\scratchpad\review\R2.md`

## Il mandato

1. **Il compito 2 — la configurazione di `vitest` 4.1.11 nel browser.** Verifica nei pacchetti installati nella cartella
   di prova, non a memoria: `test.projects` ed `extends: true`; il provider `playwright(...)` di `@vitest/browser-playwright`
   4.1.11 con `launchOptions: { channel: "chrome" }`; `instances`, `viewport`, `headless`; i `commands` personalizzati e
   il tipo `BrowserCommands` in `declare module "vitest/browser"`; `ctx.page.emulateMedia` di `playwright` 1.63.0. Poi
   **fallo girare**: le prove del compito 2 nel browser vero, sul Chrome installato, senza finestra.
   - Le prove del compito 2 usano artefatti del compito 1 (i caratteri in `tokens/index.ts`, `theme.ts`, `base.css`,
     `themes.css`): prendili dal testo del compito 1 (righe 219–1172) quanto basta — lo script del passo 9 del compito 1
     copia i due fogli dalla tavola — o sostituiscili col minimo, e dillo.
   - Le **fonti** che la riga 2 del *«Come si riprende»* cita alla v4.1.11 di `vitest-dev/vitest` esistono e dicono ciò che il
     piano dice? (alla fonte primaria, se la rete risponde).
   - Il cancello: che cosa il compito cambia in `scripts/gate-gui.sh`; le prove nel browser girano dentro il cancello dopo
     `npm ci`? Playwright pretende di scaricare i suoi browser? Chrome come prerequisito è dichiarato dove serve?
2. **Il compito 3 — il kit.**
   - `lucide` 1.47.0: ogni icona nominata in `ICONS` esiste come esportazione, e `IconNode` ha la forma che `BaseIcon` usa;
   - `reka-ui` 2.10.4: ogni componente, prop, emit e slot che i pezzi di base usano esiste in quella versione
     (`node_modules/reka-ui/dist/**/*.d.ts`);
   - i **diciotto** tipi di `PANEL_TYPES` (`grep` in `gui/src`), e un'icona per ciascuno;
   - l'aiutante di `axe` spostato in `testing/axe.ts` senza perdere nulla di ciò che `a11y.test.ts` prova oggi (conta gli
     `it` prima e dopo);
   - la configurazione di `eslint` — il blocco dei `.ts` col parser di TypeScript, `no-restricted-imports` sui pezzi di
     base, P-2 e P-3 — applicala nella cartella di prova e prova le **due direzioni** che il piano detta (verde sul codice
     di oggi, rosso su una violazione messa a mano);
   - applica i pezzi dettati e lancia `npx vue-tsc --noEmit`, `npx vitest run src/components/kit.test.ts`, `npx eslint src`.
3. **Il disegno (b) e (f):** gli otto pezzi, la loro forma, le regole — il compito ne fa meno, più o altro? Ciò che il
   compito cita del disegno, il disegno lo dice davvero (`grep` sulla frase)?

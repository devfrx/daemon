// The one entry of the design tokens (design system, section (a)), for the SPA and for the kit page.
//
// ⛔ THE FONTS ARE IMPORTED HERE AND NOT FROM `base.css` (P-5 of the plan): `base.css` is the board's block
// byte for byte, and the board loads its fonts from jsDelivr with <link>. ⛔ BARLOW SHIPS ONLY IN THE WEIGHTS A ROLE
// NAMES, by the owner's decision (AUD-2076 of the audit of 2026-09-30, on answer 8 of the design, section (a), «i
// pesi»): 300 for `--font-display`, the large numbers; 500 for `--font-numeric`; 600 for `--font-label`. A weight no
// role names is a file the page carries for nothing, and `tokens.browser.test.ts` holds that the faces the page
// declares are these and no other. Geist is variable, one file per subset. Nothing is fetched at run time (answer 6).
import "@fontsource-variable/geist";
import "@fontsource/barlow/300.css";
import "@fontsource/barlow/500.css";
import "@fontsource/barlow/600.css";

import "./base.css";
import "./themes.css";
import "./dock.css";

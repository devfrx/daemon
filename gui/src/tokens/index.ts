// The one entry of the design tokens (design system, section (a)), for the SPA and for the kit page.
//
// ⛔ THE FONTS ARE IMPORTED HERE AND NOT FROM `base.css` (P-5 of the plan): `base.css` is the board's block
// byte for byte, and the board loads its fonts from jsDelivr with <link>. Barlow ships in the four weights the
// design chose (section (a), «i pesi»; answer 8): 300 for `--font-display`, the large numbers; 500 for
// `--font-numeric`; 600 for `--font-label`; and 400. ⚠️ NO ROLE DRAWS BARLOW AT 400 TODAY, and 500 is drawn by the
// kit page alone: the strip's button, the reason the design gave for 500, is a `BaseButton` in Geist. Whether the two
// keep shipping is the owner's, open (AUD-2076 of the audit of 2026-09-30). Geist is variable, one file per
// subset. Nothing is fetched at run time (answer 6).
import "@fontsource-variable/geist";
import "@fontsource/barlow/300.css";
import "@fontsource/barlow/400.css";
import "@fontsource/barlow/500.css";
import "@fontsource/barlow/600.css";

import "./base.css";
import "./themes.css";
import "./dock.css";

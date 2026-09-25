import { createApp } from "vue";

import "../tokens";
import Kit from "./Kit.vue";

// The kit page (design system, section (b); answer 12): `npm run dev`, then /kit.html. A development page: `vite build`
// takes `index.html` alone, and the gate proves it on the output (task 4 of the plan).
createApp(Kit).mount("#kit");

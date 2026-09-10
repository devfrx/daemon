import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

// SP-8 -- the fake Home. `base: './'` because the two shells load the build from a file:// or a
// tauri:// origin, where absolute asset paths break (P-4 of the plan). `popout.html` is a second page:
// dockview opens it for a popout group. `/stream` is proxied to the SP-7 relay, which serves
// Server-Sent Events without CORS headers (P-3): the page stays same-origin and the relay is untouched.
export default defineConfig({
  base: './',
  plugins: [vue()],
  build: {
    rollupOptions: {
      input: {
        index: fileURLToPath(new URL('index.html', import.meta.url)),
        popout: fileURLToPath(new URL('popout.html', import.meta.url)),
      },
    },
  },
  server: {
    proxy: { '/stream': { target: 'http://127.0.0.1:7878', changeOrigin: true } },
  },
});

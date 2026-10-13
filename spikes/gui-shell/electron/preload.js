// SP-8 -- the preload: the only door between the page and the shell. The page sees `window.harness`.
const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('harness', {
  onLine: (callback) => ipcRenderer.on('line', (_event, line) => callback(line)),
  // Q2: the Chromium this shell packs. Nothing reads it: Q2 took Electron's Chromium from the window title
  // (`ua=`, stats.ts), and the cross-check against this value that the plan of the spike asked for is not
  // recorded. ⚠️ RECALL OF 2026-10-13 -- audit of 2026-09-30, AUD-2119.
  chrome: process.versions.chrome,
});

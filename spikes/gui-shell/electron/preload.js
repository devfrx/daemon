// SP-8 -- the preload: the only door between the page and the shell. The page sees `window.harness`.
const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('harness', {
  onLine: (callback) => ipcRenderer.on('line', (_event, line) => callback(line)),
  chrome: process.versions.chrome, // Q2: the Chromium this shell packs
});

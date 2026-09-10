// SP-8 -- the Electron shell of the fake Home. The main process reads the emitter's named pipe with
// Node's `net` and forwards every line to the renderer; the preload exposes it as `window.harness`.
// Q1: with this shell, decoding `bincode` would live HERE, in Node (`bincode-ts`, M-11 of ADR-0037).
// Nothing here is product code.
const { app, BrowserWindow } = require('electron');
const net = require('node:net');
const path = require('node:path');

const PIPE = '\\\\.\\pipe\\gui-ipc-spike'; // interprocess prepends \\.\pipe\ to a namespaced name (P-5)
const RETRY_MS = 2000; // the emitter may not be listening yet: retry, like the "riprova" strip of the real GUI

let win;

function connect() {
  const socket = net.connect({ path: PIPE });
  let buffer = '';
  socket.setEncoding('utf8');
  socket.on('connect', () => console.log('core connected'));
  socket.on('data', (chunk) => {
    buffer += chunk;
    let cut;
    while ((cut = buffer.indexOf('\n')) >= 0) {
      const line = buffer.slice(0, cut);
      buffer = buffer.slice(cut + 1);
      if (win && !win.isDestroyed()) win.webContents.send('line', line);
    }
  });
  socket.on('error', () => {}); // ENOENT while the emitter is not there: 'close' below retries
  socket.on('close', () => setTimeout(connect, RETRY_MS));
}

app.whenReady().then(() => {
  win = new BrowserWindow({
    width: 1400,
    height: 900,
    webPreferences: { preload: path.join(__dirname, 'preload.js') },
  });
  // Q3: a popout is a window.open from the page; the shell allows it and Electron opens a child window.
  win.webContents.setWindowOpenHandler(() => ({ action: 'allow' }));
  win.webContents.once('did-finish-load', connect); // no line is sent before the page can receive it (P1)
  win.loadFile(path.join(__dirname, 'dist', 'index.html'));
});

app.on('window-all-closed', () => app.quit());

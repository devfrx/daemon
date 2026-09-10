//! SP-8 -- the Tauri shell of the fake Home. A thread reads the emitter's named pipe with `interprocess`
//! -- the same client as `spikes/gui-ipc/src/bin/gui.rs` -- and emits every line to the page as the event
//! `line`; the page listens with `@tauri-apps/api/event`. Q1: with this shell, decoding `bincode` would
//! live HERE, in Rust, with the kernel's own decoder. Nothing here is product code.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use interprocess::local_socket::{prelude::*, GenericNamespaced, Stream, ToNsName};
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use tauri::{Emitter, Manager};

const NAME: &str = "gui-ipc-spike"; // `NOME` of spikes/gui-ipc/src/lib.rs: \\.\pipe\gui-ipc-spike on Windows (P-5)
const RETRY: Duration = Duration::from_secs(2);

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            thread::spawn(move || loop {
                let connected = NAME
                    .to_ns_name::<GenericNamespaced>()
                    .and_then(Stream::connect);
                if let Ok(stream) = connected {
                    eprintln!("core connected");
                    for line in BufReader::new(stream).lines() {
                        let Ok(line) = line else { break };
                        let _ = handle.emit("line", line);
                    }
                    eprintln!("core gone");
                }
                // not listening yet, or gone: retry, like the "riprova" strip of the real GUI
                thread::sleep(RETRY);
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("the Tauri shell did not start");
}

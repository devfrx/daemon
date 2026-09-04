//! SP-7 / S2 -- the relay that stands where the core will stand.
//!
//! It spawns the Python worker as a CHILD PROCESS and reads its stdout line by line -- the
//! topology of the core and a worker, on a pipe -- stamps each line with its own wall clock, and
//! serves it to the page over Server-Sent Events. Throwaway: no dependency, no error handling
//! beyond what the measurement needs, and nothing of it climbs into `crates/`.
//!
//! Routes: `GET /` -> the page; `GET /stream` -> `text/event-stream`, one `data:` per result.
//! Usage: cargo run --release -- <python> <worker.py> <model.task> [port]

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

const PAGE: &str = include_str!("../page.html");

fn wall_ms() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("clock").as_millis()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: sp7-relay <python> <worker.py> <model.task> [port]");
        std::process::exit(2);
    }
    let port = args.get(4).map(|p| p.as_str()).unwrap_or("7878");

    let mut child = Command::new(&args[1])
        .args([&args[2], &args[3]])
        .stdout(Stdio::piped())
        .spawn()
        .expect("the worker did not start");
    let stdout = child.stdout.take().expect("piped stdout");

    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
    let sinks = Arc::clone(&clients);
    thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            let Ok(line) = line else { break };
            let Some(body) = line.strip_suffix('}') else { continue };
            // {"t_capture_ms":..,"hands":[..]}  ->  the same object plus the relay's own stamp.
            let event = format!("data: {body},\"t_relay_ms\":{}}}\n\n", wall_ms());
            let mut sinks = sinks.lock().expect("lock");
            sinks.retain_mut(|s| s.write_all(event.as_bytes()).and_then(|_| s.flush()).is_ok());
        }
        eprintln!("worker stdout closed");
    });

    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).expect("bind");
    eprintln!("open http://127.0.0.1:{port}/ in the browser; Ctrl-C to stop");
    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };
        let mut request = [0u8; 1024];
        let n = stream.read(&mut request).unwrap_or(0);
        let head = String::from_utf8_lossy(&request[..n]);
        if head.starts_with("GET /stream") {
            let _ = stream.write_all(
                b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nConnection: keep-alive\r\n\r\n",
            );
            clients.lock().expect("lock").push(stream);
        } else {
            let _ = stream.write_all(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    PAGE.len(),
                    PAGE
                )
                .as_bytes(),
            );
        }
    }
    let _ = child.kill();
}

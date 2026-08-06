//! C6 — il parallelismo **nativo** resta ordinabile dal seed?
//!
//! C1–C4 provano che un esecutore scritto a mano è deterministico. Si può scrivere in
//! qualunque linguaggio, quindi non discrimina. C6 chiede l'altra cosa: le unità
//! concorrenti native del linguaggio, in contesa su una risorsa condivisa, producono
//! la stessa traccia a parità di seed?
//!
//! Qui si misurano due vie, perché danno risposte opposte e la differenza è il punto:
//!
//! | via | unità native | chi decide l'ordine |
//! |---|---|---|
//! | `esegui_async` | `Future` — la primitiva di concorrenza di Rust per l'I/O | **il nostro esecutore** |
//! | `esegui_thread` | `std::thread` — thread del sistema operativo | lo scheduler dell'OS |
//!
//! V29 · ADR-0021 · ADR-0004.

use crate::sched::Rng;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

/// Future che cede il controllo una volta sola. È il punto in cui l'esecutore
/// riprende la decisione su chi far proseguire.
struct Cede(bool);

impl Future for Cede {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            Poll::Pending
        }
    }
}

/// Via 1 — `Future` native guidate da un esecutore che sceglie col seed.
///
/// L'ordine di acquisizione della risorsa condivisa è deciso qui, non dal runtime:
/// è esattamente la sostituibilità che V29 richiede.
pub fn esegui_async(seed: u64, n_task: usize, passi: usize) -> Vec<String> {
    let traccia: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let mut rng = Rng::new(seed);

    let mut tasks: Vec<Pin<Box<dyn Future<Output = ()>>>> = (0..n_task)
        .map(|i| {
            let t = Rc::clone(&traccia);
            Box::pin(async move {
                for p in 0..passi {
                    // contesa: tutti scrivono sulla stessa risorsa
                    t.borrow_mut().push(format!("task{i} passo{p}"));
                    Cede(false).await;
                }
            }) as Pin<Box<dyn Future<Output = ()>>>
        })
        .collect();

    let mut cx = Context::from_waker(Waker::noop());
    while !tasks.is_empty() {
        let i = rng.below(tasks.len() as u64) as usize;
        if tasks[i].as_mut().poll(&mut cx).is_ready() {
            // Il future è concluso: scartarlo è l'intento, e va detto al compilatore.
            drop(tasks.remove(i));
        }
    }

    Rc::try_unwrap(traccia)
        .expect("nessun altro riferimento vivo")
        .into_inner()
}

/// Via 2 — thread del sistema operativo, per controprova.
///
/// Il seed non entra: non c'è alcun punto in cui inserirlo. L'ordine lo decide lo
/// scheduler dell'OS, e nessun linguaggio può renderlo riproducibile senza
/// sostituire il runtime.
pub fn esegui_thread(n_task: usize, passi: usize) -> Vec<String> {
    let traccia: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..n_task)
        .map(|i| {
            let t = Arc::clone(&traccia);
            std::thread::spawn(move || {
                for p in 0..passi {
                    t.lock().expect("mutex avvelenato").push(format!("task{i} passo{p}"));
                    std::thread::yield_now();
                }
            })
        })
        .collect();

    for h in handles {
        h.join().expect("thread fallito");
    }

    let guard = traccia.lock().expect("mutex avvelenato");
    guard.clone()
}

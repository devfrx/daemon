//! Il processo "core": emette TOTALE messaggi in DURATA_MS su tre canali logici.
//!
//! P4 — se la gui muore a metà, il core **non deve accorgersene**: continua e
//! termina il proprio lavoro. È la sacrificabilità di ADR-0004 vista dal lato core.

use gui_ipc_spike::{ora_micros, Canale, Messaggio, DURATA_MS, NOME, TOTALE};
use interprocess::local_socket::{
    traits::ListenerExt, GenericNamespaced, ListenerOptions, ToNsName,
};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let nome = NOME.to_ns_name::<GenericNamespaced>()?;
    let listener = ListenerOptions::new().name(nome).create_sync()?;
    eprintln!("core: in ascolto su {NOME}");

    let mut incoming = listener.incoming();
    let mut conn = incoming.next().expect("nessun client")?;
    eprintln!("core: gui collegata, inizio a emettere");

    let intervallo = std::time::Duration::from_micros(DURATA_MS * 1000 / TOTALE);
    let inizio = std::time::Instant::now();
    let mut gui_caduta_a: Option<u64> = None;
    let mut gui_riaperta_a: Option<u64> = None;
    let mut collegata = true;

    for seq in 0..TOTALE {
        let canale = match seq % 10 {
            0 => Canale::Stato,     // ~10%: cambia di rado
            1 => Canale::Metriche,  // ~10%: aggiornamento periodico
            _ => Canale::Token,     // ~80%: è lo streaming che domina
        };
        let m = Messaggio {
            canale,
            seq,
            emesso_micros: ora_micros(),
            carico: "lorem ipsum dolor sit amet".to_string(),
        };
        let mut riga = serde_json::to_string(&m).expect("serializzazione");
        riga.push('\n');

        if collegata && conn.write_all(riga.as_bytes()).is_err() {
            // P4: la gui è caduta. Non è un errore del core, è la sacrificabilità
            // di ADR-0004. Il core prosegue e **non aspetta**: lo stato è suo (I1),
            // non c'è nulla da salvare e nessuno a cui chiedere.
            collegata = false;
            gui_caduta_a = Some(seq);
            eprintln!("core: la gui è caduta al messaggio {seq}; proseguo comunque");

            // G3 — la gui si può riaprire. Il core torna ad accettare senza
            // interrompere il proprio lavoro: prova una volta, senza bloccarsi.
            if let Some(Ok(nuova)) = incoming.next() {
                conn = nuova;
                collegata = true;
                gui_riaperta_a = Some(seq);
                eprintln!("core: gui riaperta al messaggio {seq}");
            }
        }

        let atteso = intervallo * (seq as u32 + 1);
        let trascorso = inizio.elapsed();
        if atteso > trascorso {
            std::thread::sleep(atteso - trascorso);
        }
    }

    let durata = inizio.elapsed();
    eprintln!("core: emessi {TOTALE} messaggi in {} ms", durata.as_millis());
    match (gui_caduta_a, gui_riaperta_a) {
        (Some(c), Some(r)) => eprintln!(
            "core: P4 — gui caduta a {c}, riaperta a {r}, {TOTALE} messaggi emessi lo stesso"
        ),
        (Some(c), None) => eprintln!(
            "core: P4 — gui caduta a {c} e non riaperta, {TOTALE} messaggi emessi lo stesso"
        ),
        _ => eprintln!("core: la gui è rimasta collegata fino alla fine"),
    }
    Ok(())
}

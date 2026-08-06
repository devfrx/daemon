//! Il processo "gui": riceve, rende in una lista e misura P1, P2, P3.
//!
//! La lista è limitata, come lo sarebbe una lista virtualizzata reale: la GUI non
//! possiede stato (I1), quindi non ha motivo di trattenere tutto.

use gui_ipc_spike::{ora_micros, Canale, Messaggio, NOME, TOTALE};
use interprocess::local_socket::{prelude::*, GenericNamespaced, Stream, ToNsName};
use std::collections::VecDeque;
use std::io::{BufRead, BufReader};

const FINESTRA_LISTA: usize = 500;

fn main() -> std::io::Result<()> {
    // P4 — `gui abort-a <n>` termina in modo **anomalo** dopo n messaggi.
    // Non un'uscita pulita: `abort` è il caso che I1 e G3 promettono di reggere.
    let abort_a: Option<u64> = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .filter(|_| std::env::args().nth(1).as_deref() == Some("abort-a"));

    let nome = NOME.to_ns_name::<GenericNamespaced>()?;
    let conn = Stream::connect(nome)?;
    let lettore = BufReader::new(conn);

    // "rendering": una lista limitata, con conteggi per canale come li mostrerebbe
    // l'interfaccia (G4, G9, G11).
    let mut lista: VecDeque<String> = VecDeque::with_capacity(FINESTRA_LISTA);
    let mut per_canale = [0u64; 3];

    let mut ricevuti = 0u64;
    let mut ultimo_seq: Option<u64> = None;
    let mut buchi = 0u64;
    let mut ritardo_max_micros = 0u128;
    let mut ritardo_somma = 0u128;

    let pid = sysinfo::get_current_pid().expect("pid");
    let mut sys = sysinfo::System::new();
    let mut cpu_max = 0.0f32;
    let mut cpu_somma = 0.0f64;
    let mut campioni_cpu = 0u32;
    let mut ultimo_campione = std::time::Instant::now();

    let inizio = std::time::Instant::now();

    for riga in lettore.lines() {
        let riga = riga?;
        let m: Messaggio = match serde_json::from_str(&riga) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("gui: riga malformata: {e}");
                continue;
            }
        };

        // P1 — i messaggi persi si contano dai buchi nel progressivo.
        if let Some(prec) = ultimo_seq {
            if m.seq > prec + 1 {
                buchi += m.seq - prec - 1;
            }
        }
        ultimo_seq = Some(m.seq);
        ricevuti += 1;

        if abort_a == Some(ricevuti) {
            eprintln!("gui: abort deliberato dopo {ricevuti} messaggi (P4)");
            std::process::abort();
        }

        // P2 — ritardo fra emissione e "rendering".
        let ritardo = ora_micros().saturating_sub(m.emesso_micros);
        ritardo_max_micros = ritardo_max_micros.max(ritardo);
        ritardo_somma += ritardo;

        // il "rendering": aggiorna la lista e i contatori mostrati sempre
        per_canale[match m.canale {
            Canale::Token => 0,
            Canale::Stato => 1,
            Canale::Metriche => 2,
        }] += 1;
        if lista.len() == FINESTRA_LISTA {
            lista.pop_front();
        }
        lista.push_back(format!("[{:?}] #{} {}", m.canale, m.seq, m.carico));

        // P3 — CPU del processo gui, campionata a intervalli regolari.
        if ultimo_campione.elapsed() >= sysinfo::MINIMUM_CPU_UPDATE_INTERVAL {
            sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true);
            if let Some(p) = sys.process(pid) {
                let c = p.cpu_usage();
                cpu_max = cpu_max.max(c);
                cpu_somma += c as f64;
                campioni_cpu += 1;
            }
            ultimo_campione = std::time::Instant::now();
        }
    }

    let durata = inizio.elapsed();
    let persi = TOTALE.saturating_sub(ricevuti);
    let cpu_media = if campioni_cpu > 0 {
        cpu_somma / campioni_cpu as f64
    } else {
        0.0
    };

    println!("--- MISURE P1..P3 ---");
    println!("durata_ms          {}", durata.as_millis());
    println!("attesi             {TOTALE}");
    println!("ricevuti           {ricevuti}");
    println!("P1_persi           {persi}");
    println!("P1_buchi_nel_seq   {buchi}");
    println!("P2_ritardo_max_ms  {:.3}", ritardo_max_micros as f64 / 1000.0);
    println!(
        "P2_ritardo_medio_ms {:.3}",
        if ricevuti > 0 {
            ritardo_somma as f64 / ricevuti as f64 / 1000.0
        } else {
            0.0
        }
    );
    println!("P3_cpu_max_pct     {cpu_max:.2}");
    println!("P3_cpu_media_pct   {cpu_media:.2}");
    println!("campioni_cpu       {campioni_cpu}");
    println!(
        "per_canale         token={} stato={} metriche={}",
        per_canale[0], per_canale[1], per_canale[2]
    );
    println!("lista_finale       {} righe (finestra {FINESTRA_LISTA})", lista.len());
    Ok(())
}

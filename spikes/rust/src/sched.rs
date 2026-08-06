//! SP-5: tempo, casualità e ordinamento delle attività sono iniettabili (V29).
//! Nessuna lettura dell'orologio di sistema, nessun RNG globale (C5).

/// Generatore deterministico. Sostituisce ogni fonte di casualità.
pub(crate) struct Rng(u64);

impl Rng {
    pub(crate) fn new(seed: u64) -> Self {
        let s = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        // xorshift resta bloccato su zero: la guardia evita un seed morto.
        Rng(if s == 0 { 1 } else { s })
    }
    fn next(&mut self) -> u64 {
        // xorshift64: deterministico e sufficiente per uno spike
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    pub(crate) fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

struct Task {
    nome: String,
    rimanenti: u32,
}

/// Il "mondo" possiede tempo, casualità e coda delle attività.
pub struct World {
    rng: Rng,
    orologio: u64,
    tasks: Vec<Task>,
    traccia: Vec<String>,
}

impl World {
    pub fn new(seed: u64) -> Self {
        World {
            rng: Rng::new(seed),
            orologio: 0,
            tasks: Vec::new(),
            traccia: Vec::new(),
        }
    }

    pub fn spawn(&mut self, nome: &str, steps: u32) {
        self.tasks.push(Task {
            nome: nome.to_string(),
            rimanenti: steps,
        });
    }

    /// Avanza l'orologio virtuale senza attendere davvero (C3).
    pub fn sleep_virtuale(&mut self, millis: u64) {
        self.orologio += millis;
    }

    pub fn now(&self) -> u64 {
        self.orologio
    }

    /// Esegue fino a esaurimento, interlacciando le attività secondo il seed.
    pub fn run(&mut self) -> Vec<String> {
        while !self.tasks.is_empty() {
            let i = self.rng.below(self.tasks.len() as u64) as usize;
            self.orologio += 1;

            // Guasto iniettato in un punto scelto dal seed (C4).
            let guasto = self.rng.below(20) == 0;
            let nome = self.tasks[i].nome.clone();

            if guasto {
                self.traccia
                    .push(format!("t={} {} GUASTO", self.orologio, nome));
                self.tasks.remove(i);
                continue;
            }

            self.traccia
                .push(format!("t={} {} passo", self.orologio, nome));
            self.tasks[i].rimanenti -= 1;
            if self.tasks[i].rimanenti == 0 {
                self.traccia
                    .push(format!("t={} {} fine", self.orologio, nome));
                self.tasks.remove(i);
            }
        }
        self.traccia.clone()
    }
}

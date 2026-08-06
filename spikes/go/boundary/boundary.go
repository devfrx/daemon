// Package boundary — SP-6: il confine dei dati non fidati vive nei tipi (V19, V20).
//
// # Trappola misurata il 2026-08-06
//
// I nomi dei campi delle due struct sono DIVERSI, e non è una scelta estetica.
//
// In Go due tipi con lo stesso tipo sottostante sono convertibili l'uno nell'altro.
// L'identità dei tipi sottostanti richiede la stessa sequenza di nomi e tipi dei
// campi; per i campi non esportati conta anche il package di provenienza — che qui è
// lo stesso per entrambi i tipi.
//
// Conseguenza: se entrambe le struct avessero un campo `text string`, allora
//
//	boundary.Instruction(untrustedValue)
//
// compilerebbe **da qualsiasi punto del progetto**, anche fuori dal package, senza
// `unsafe` e senza reflection. Misurato: compila, gira, e il contenuto non fidato
// finisce nel canale delle istruzioni.
//
// Con nomi di campo diversi il compilatore rifiuta:
//
//	cannot convert dalWeb (variable of struct type boundary.Untrusted)
//	to type boundary.Instruction
//
// Il test in ../conversione lo blinda: se qualcuno riallineasse i nomi, fallisce.
package boundary

// Instruction può occupare il canale delle istruzioni.
type Instruction struct{ text string }

// Untrusted proviene da una fonte esterna. Non è mai un'autorizzazione.
// Il campo si chiama `raw` e non `text`: vedi la nota sulla trappola, sopra.
type Untrusted struct{ raw string }

func NewInstruction(s string) Instruction { return Instruction{text: s} }
func NewUntrusted(s string) Untrusted     { return Untrusted{raw: s} }

func (i Instruction) String() string { return i.text }
func (u Untrusted) String() string   { return u.raw }

// PromoteToInstruction — T2: unico percorso di conversione.
// Nel kernel reale la chiamata è giornalata.
func (u Untrusted) PromoteToInstruction(motivo string) Instruction {
	return Instruction{text: u.raw}
}

// Summarize — T3: l'etichetta è ereditaria (V20).
func Summarize(u Untrusted) Untrusted {
	if len(u.raw) > 50 {
		return Untrusted{raw: u.raw[:50]}
	}
	return u
}

// BuildPrompt accetta solo Instruction.
func BuildPrompt(system, user Instruction) string {
	return system.text + "\n" + user.text
}

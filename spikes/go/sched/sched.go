// Package sched — SP-5: tempo, casualità e ordinamento sono iniettabili (V29).
// Nessuna lettura di time.Now né di math/rand globale (C5).
package sched

import "fmt"

// Rng è il generatore deterministico. È esportato perché lo usa anche il package
// giornale (C7): una sola fonte di casualità sostituibile, non due.
type Rng struct{ s uint64 }

func NewRng(seed uint64) *Rng {
	s := seed*6364136223846793005 + 1
	// xorshift resta bloccato su zero: la guardia evita un seed morto.
	if s == 0 {
		s = 1
	}
	return &Rng{s: s}
}

func (r *Rng) next() uint64 {
	x := r.s
	x ^= x << 13
	x ^= x >> 7
	x ^= x << 17
	r.s = x
	return x
}

func (r *Rng) Below(n uint64) uint64 { return r.next() % n }

type task struct {
	nome      string
	rimanenti int
}

// World possiede tempo, casualità e coda delle attività.
type World struct {
	r        *Rng
	orologio uint64
	tasks    []*task
	traccia  []string
}

func NewWorld(seed uint64) *World {
	return &World{r: NewRng(seed)}
}

func (w *World) Spawn(nome string, steps int) {
	w.tasks = append(w.tasks, &task{nome: nome, rimanenti: steps})
}

// SleepVirtuale avanza l'orologio senza attendere davvero (C3).
func (w *World) SleepVirtuale(ms uint64) { w.orologio += ms }

func (w *World) Now() uint64 { return w.orologio }

func (w *World) Run() []string {
	for len(w.tasks) > 0 {
		i := int(w.r.Below(uint64(len(w.tasks))))
		w.orologio++

		guasto := w.r.Below(20) == 0
		nome := w.tasks[i].nome

		if guasto {
			w.traccia = append(w.traccia, fmt.Sprintf("t=%d %s GUASTO", w.orologio, nome))
			w.tasks = append(w.tasks[:i], w.tasks[i+1:]...)
			continue
		}

		w.traccia = append(w.traccia, fmt.Sprintf("t=%d %s passo", w.orologio, nome))
		w.tasks[i].rimanenti--
		if w.tasks[i].rimanenti == 0 {
			w.traccia = append(w.traccia, fmt.Sprintf("t=%d %s fine", w.orologio, nome))
			w.tasks = append(w.tasks[:i], w.tasks[i+1:]...)
		}
	}
	return w.traccia
}

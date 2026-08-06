// Package kernel — il codice che NON deve toccare il sistema operativo (I3) né
// avere un percorso verso il gateway di inferenza (V28).
//
// La regola è verificata da deps_test.go sul grafo delle importazioni reale,
// prodotto da `go list -deps`. Non è una convenzione: è un test che fallisce.
package kernel

import "sort"

// OrdinaPerPriorita usa un ordinamento stabile: a parità di priorità l'ordine di
// inserimento è conservato. Per V29 non è stile — un ordinamento instabile
// introdurrebbe non determinismo in una coda del kernel.
func OrdinaPerPriorita(lavori []Lavoro) []Lavoro {
	out := make([]Lavoro, len(lavori))
	copy(out, lavori)
	sort.SliceStable(out, func(a, b int) bool { return out[a].Priorita < out[b].Priorita })
	return out
}

type Lavoro struct {
	Priorita int
	Nome     string
}

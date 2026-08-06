// Package conversione contiene la seconda violazione del confine: la conversione
// fra tipi con lo stesso tipo sottostante. È esclusa dalla build normale dal tag
// `violation`, come il package sorella `violation`.
//
// Esiste perché T1 da solo non la copriva: il piano originale affermava che in Go
// l'aggiramento fosse possibile «solo dentro il package». Misurato il 2026-08-06:
// falso, se i nomi dei campi coincidono.
package conversione

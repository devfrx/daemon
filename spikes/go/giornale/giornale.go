// Package giornale — C7: l'I/O durevole è iniettabile, e un crash al confine di
// persistenza è riproducibile dal seed.
//
// V29 elenca quattro cose iniettabili: tempo, casualità, I/O, scheduling. C1–C6
// coprono le altre tre. La tecnica di verifica di Q5 in ADR-0021 è la
// crash-injection ai confini di persistenza, e il confine principale è il giornale
// write-ahead di ADR-0007.
//
// La proprietà che si compra con la doppia scrittura: un passo con intento e senza
// esito è in dubbio, e il dubbio è sempre rilevabile.
package giornale

import (
	"errors"
	"fmt"
	"strconv"
	"strings"

	"kernelspike/sched"
)

// ErrCaduto simula lo spegnimento al confine di persistenza. Non è un errore
// applicativo.
var ErrCaduto = errors.New("caduto al confine di persistenza")

// Giornale è il confine sostituibile. Il codice sotto test non conosce nessun'altra
// via verso il durevole: nessuna chiamata al filesystem, mai.
type Giornale interface {
	Intento(passo uint64, descrizione string) error
	Esito(passo uint64, esito string) error
}

// Cadente è un doppio in memoria che cade a una scrittura scelta dal seed.
type Cadente struct {
	Righe     []string
	cadeAlla  uint64
	scritture uint64
}

func NuovoCadente(seed uint64, scrittureP uint64) *Cadente {
	r := sched.NewRng(seed)
	return &Cadente{cadeAlla: r.Below(scrittureP)}
}

// SenzaCrash serve al caso di controllo: senza, un test di riproducibilità su
// un'esecuzione che non cade mai sarebbe vacuo.
func SenzaCrash() *Cadente {
	return &Cadente{cadeAlla: ^uint64(0)}
}

func (c *Cadente) scrivi(riga string) error {
	if c.scritture == c.cadeAlla {
		return ErrCaduto
	}
	c.scritture++
	c.Righe = append(c.Righe, riga)
	return nil
}

func (c *Cadente) Intento(passo uint64, d string) error {
	return c.scrivi(fmt.Sprintf("passo=%d INTENTO %s", passo, d))
}

func (c *Cadente) Esito(passo uint64, e string) error {
	return c.scrivi(fmt.Sprintf("passo=%d ESITO %s", passo, e))
}

// Esegui scrive write-ahead: intento prima dell'effetto, esito dopo.
// Si ferma alla caduta.
func Esegui(g Giornale, passi uint64) {
	for p := uint64(0); p < passi; p++ {
		if g.Intento(p, "chiamata a strumento") != nil {
			return
		}
		// qui, nel kernel reale, avviene l'effetto
		if g.Esito(p, "ok") != nil {
			return
		}
	}
}

// PassoInDubbio restituisce il passo con intento e senza esito, e true se esiste.
// È la proprietà che ADR-0007 compra con la seconda scrittura durevole.
func PassoInDubbio(righe []string) (uint64, bool) {
	var aperto uint64
	trovato := false
	for _, riga := range righe {
		campi := strings.Fields(riga)
		if len(campi) < 2 {
			continue
		}
		n, err := strconv.ParseUint(strings.TrimPrefix(campi[0], "passo="), 10, 64)
		if err != nil {
			continue
		}
		switch campi[1] {
		case "INTENTO":
			aperto, trovato = n, true
		case "ESITO":
			if trovato && aperto == n {
				trovato = false
			}
		}
	}
	return aperto, trovato
}

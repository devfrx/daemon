package giornale

import (
	"fmt"
	"reflect"
	"strings"
	"testing"
)

const (
	passi      = uint64(8)
	scrittureP = passi * 2
)

func esecuzione(seed uint64) []string {
	c := NuovoCadente(seed, scrittureP)
	Esegui(c, passi)
	return c.Righe
}

func TestC7StessoSeedStessaTracciaCrashIncluso(t *testing.T) {
	for _, seed := range []uint64{1, 7, 42, 99, 20260806} {
		if !reflect.DeepEqual(esecuzione(seed), esecuzione(seed)) {
			t.Fatalf("C7 violato: il crash non è riproducibile con seed %d", seed)
		}
	}
}

func TestC7IlCrashAvvieneDavvero(t *testing.T) {
	cadute := 0
	for seed := uint64(0); seed < 50; seed++ {
		if uint64(len(esecuzione(seed))) < scrittureP {
			cadute++
		}
	}
	if cadute == 0 {
		t.Fatal("C7 vacuo: nessuno dei 50 seed ha prodotto una caduta")
	}
	t.Logf("cadute su 50 seed: %d", cadute)
}

func TestC7IlPassoInDubbioERilevabile(t *testing.T) {
	var seedConDubbio uint64
	var passo uint64
	trovato := false
	for seed := uint64(0); seed < 200; seed++ {
		if p, ok := PassoInDubbio(esecuzione(seed)); ok {
			seedConDubbio, passo, trovato = seed, p, true
			break
		}
	}
	if !trovato {
		t.Fatal("almeno un seed su 200 deve cadere fra intento ed esito")
	}

	righe := esecuzione(seedConDubbio)
	haIntento, haEsito := false, false
	for _, r := range righe {
		if strings.Contains(r, fmt.Sprintf("passo=%d INTENTO", passo)) {
			haIntento = true
		}
		if strings.Contains(r, fmt.Sprintf("passo=%d ESITO", passo)) {
			haEsito = true
		}
	}
	if !haIntento {
		t.Fatal("il passo in dubbio deve avere un intento registrato")
	}
	if haEsito {
		t.Fatal("il passo in dubbio non deve avere un esito")
	}
	t.Logf("C7 — seed con passo in dubbio: %d, passo %d", seedConDubbio, passo)
}

func TestC7SenzaCrashNessunPassoRestaInDubbio(t *testing.T) {
	c := SenzaCrash()
	Esegui(c, passi)
	if uint64(len(c.Righe)) != scrittureP {
		t.Fatalf("attese %d righe, ottenute %d", scrittureP, len(c.Righe))
	}
	if _, ok := PassoInDubbio(c.Righe); ok {
		t.Fatal("senza crash non deve esserci alcun dubbio: il rilevatore darebbe falsi positivi")
	}
}

func TestC7LOrdineEWriteAhead(t *testing.T) {
	c := SenzaCrash()
	Esegui(c, 3)
	var ordine []string
	for _, r := range c.Righe {
		if strings.Contains(r, "INTENTO") {
			ordine = append(ordine, "I")
		} else {
			ordine = append(ordine, "E")
		}
	}
	atteso := []string{"I", "E", "I", "E", "I", "E"}
	if !reflect.DeepEqual(ordine, atteso) {
		t.Fatalf("ADR-0007: atteso %v, ottenuto %v", atteso, ordine)
	}
}

// contatore è un secondo doppio. Se Esegui conoscesse il filesystem, questo non
// compilerebbe: il confine è nel tipo, non nella disciplina.
type contatore struct{ n uint64 }

func (c *contatore) Intento(uint64, string) error { c.n++; return nil }
func (c *contatore) Esito(uint64, string) error   { c.n++; return nil }

func TestC7IlGiornaleESostituibile(t *testing.T) {
	var c contatore
	Esegui(&c, passi)
	if c.n != scrittureP {
		t.Fatalf("attese %d scritture, ottenute %d", scrittureP, c.n)
	}
}

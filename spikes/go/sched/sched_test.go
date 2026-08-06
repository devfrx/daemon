package sched

import (
	"reflect"
	"strings"
	"testing"
	"time"
)

func traccia(seed uint64) []string {
	w := NewWorld(seed)
	w.Spawn("alfa", 5)
	w.Spawn("beta", 5)
	return w.Run()
}

func TestC1StessoSeedStessaTraccia(t *testing.T) {
	if !reflect.DeepEqual(traccia(42), traccia(42)) {
		t.Fatal("C1 violato: stesso seed, tracce diverse")
	}
}

func TestC2SeedDiversiTracceDiverse(t *testing.T) {
	if reflect.DeepEqual(traccia(42), traccia(43)) {
		t.Fatal("C2 violato: seed diversi, tracce identiche")
	}
}

func TestC3IlTempoEVirtuale(t *testing.T) {
	inizio := time.Now()
	w := NewWorld(7)
	w.Spawn("lento", 5)
	w.SleepVirtuale(5000)
	_ = w.Run()
	if w.Now() < 5000 {
		t.Fatalf("orologio virtuale non avanzato: %d", w.Now())
	}
	if time.Since(inizio) > time.Second {
		t.Fatal("C3 violato: il test ha atteso davvero")
	}
}

func TestC4IlGuastoEriproducibile(t *testing.T) {
	a := traccia(99)
	b := traccia(99)
	trovato := false
	for _, e := range a {
		if strings.Contains(e, "GUASTO") {
			trovato = true
			break
		}
	}
	if !trovato {
		t.Skip("il seed 99 non inietta guasti: scegline un altro e registralo")
	}
	if !reflect.DeepEqual(a, b) {
		t.Fatal("C4 violato: guasto non riproducibile")
	}
}

package sched

import (
	"fmt"
	"strings"
	"sync"
	"testing"
	"testing/synctest"
)

// C6 — il parallelismo **nativo** resta ordinabile dal seed?
//
// C1–C4 provano che un esecutore scritto a mano è deterministico. Si può scrivere in
// qualunque linguaggio, quindi non discrimina. Qui si misura l'altra cosa: le
// goroutine, che sono la primitiva di concorrenza di Go e non sono sostituibili.
//
// `go doc testing/synctest` (go1.26.5) dice due cose che governano questo test:
//
//   - durably blocking: send/receive su canale **della bolla**, select sui soli
//     canali della bolla, sync.Cond.Wait, WaitGroup.Wait, time.Sleep;
//   - NON durably blocking, testuale: sync.Mutex, sync.RWMutex, I/O, syscall.
//
// Il contratto dichiarato è la **quiescenza**, non l'ordine. Le due prove qui sotto
// misurano se dalla quiescenza discenda comunque un ordine riproducibile.
//
// La seconda prova non è accademica: ADR-0004 descrive l'arbitro GPU come «un unico
// processo con un unico lock», cioè la primitiva che synctest esclude.

const (
	esecuzioni = 100
	nTask      = 3
	passi      = 6
)

// conCanale — contesa su un semaforo fatto di canale: caso favorevole a synctest.
func conCanale() []string {
	sem := make(chan struct{}, 1)
	var traccia []string
	var wg sync.WaitGroup
	for i := 0; i < nTask; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			for p := 0; p < passi; p++ {
				sem <- struct{}{}
				traccia = append(traccia, fmt.Sprintf("task%d passo%d", id, p))
				<-sem
			}
		}(i)
	}
	wg.Wait()
	return traccia
}

// conMutex — contesa su sync.Mutex: esplicitamente fuori dal controllo di synctest.
func conMutex() []string {
	var mu sync.Mutex
	var traccia []string
	var wg sync.WaitGroup
	for i := 0; i < nTask; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			for p := 0; p < passi; p++ {
				mu.Lock()
				traccia = append(traccia, fmt.Sprintf("task%d passo%d", id, p))
				mu.Unlock()
			}
		}(i)
	}
	wg.Wait()
	return traccia
}

func distinti(t *testing.T, dentroBolla bool, f func() []string) int {
	t.Helper()
	visti := map[string]bool{}
	for i := 0; i < esecuzioni; i++ {
		var traccia []string
		if dentroBolla {
			synctest.Test(t, func(*testing.T) { traccia = f() })
		} else {
			traccia = f()
		}
		visti[strings.Join(traccia, "|")] = true
	}
	return len(visti)
}

// ESITO MISURATO IL 2026-08-06 SU go1.26.5 — C6 = `non passa`.
//
//	canale dentro la bolla   9 tracce distinte su 100
//	mutex  dentro la bolla   4 tracce distinte su 100
//	canale fuori dalla bolla 13
//	mutex  fuori dalla bolla 5
//
// synctest **riduce** il non determinismo ma non lo elimina: dà tempo virtuale, non
// un ordine. È coerente con la documentazione, che promette quiescenza e non ordine.
//
// Le asserzioni qui sotto **registrano il fatto misurato** invece di pretendere il
// determinismo che non c'è: se una versione futura di Go rendesse deterministico
// l'interlacciamento, questi test fallirebbero e C6 andrebbe rimisurato. È una
// guardia su un'assunzione, non una celebrazione del risultato.

// TestC6CanaleDentroSynctest — caso più favorevole a synctest: canale della bolla.
func TestC6CanaleDentroSynctest(t *testing.T) {
	n := distinti(t, true, conCanale)
	t.Logf("C6 canale, dentro synctest: %d tracce distinte su %d esecuzioni", n, esecuzioni)
	if n == 1 {
		t.Fatal("l'interlacciamento è diventato deterministico: C6 va rimisurato, " +
			"e con esso lo spareggio #1 dell'ADR sul linguaggio")
	}
}

// TestC6MutexDentroSynctest — la primitiva dell'arbitro GPU (ADR-0004, «un unico
// lock»), che `go doc testing/synctest` esclude testualmente dal durably blocking.
func TestC6MutexDentroSynctest(t *testing.T) {
	n := distinti(t, true, conMutex)
	t.Logf("C6 mutex, dentro synctest: %d tracce distinte su %d esecuzioni", n, esecuzioni)
	if n == 1 {
		t.Fatal("l'interlacciamento sul mutex è diventato deterministico: C6 va rimisurato")
	}
}

// TestC6ControprovaFuoriDallaBolla — stabilisce il confine. Se anche questa fosse
// deterministica, le due prove sopra non misurerebbero synctest ma il caso.
func TestC6ControprovaFuoriDallaBolla(t *testing.T) {
	nCanale := distinti(t, false, conCanale)
	nMutex := distinti(t, false, conMutex)
	t.Logf("controprova fuori dalla bolla: canale %d distinte, mutex %d distinte", nCanale, nMutex)
	if nCanale == 1 && nMutex == 1 {
		t.Fatal("nemmeno fuori dalla bolla c'è non determinismo: la misura non discrimina")
	}
}

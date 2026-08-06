package sched

import (
	"sync"
	"testing"
	"testing/synctest"
	"time"
)

// TestGoroutineRealiSottoControllo verifica che con synctest il tempo sia virtuale
// anche per goroutine e timer reali — cioè per la concorrenza vera, non simulata.
//
// Questo è il **tempo**, non l'ordine. La distinzione è il cuore di C6.
func TestGoroutineRealiSottoControllo(t *testing.T) {
	inizio := time.Now()
	synctest.Test(t, func(t *testing.T) {
		var wg sync.WaitGroup
		for i := 0; i < 3; i++ {
			wg.Add(1)
			go func() {
				defer wg.Done()
				time.Sleep(5 * time.Second) // virtuale dentro synctest
			}()
		}
		wg.Wait()
	})
	if time.Since(inizio) > time.Second {
		t.Fatal("il tempo non era virtuale: synctest non sta funzionando")
	}
}

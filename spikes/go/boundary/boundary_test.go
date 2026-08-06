package boundary

import (
	"strings"
	"testing"
)

func TestT3EreditarietaLaTrasformazioneRestaNonFidata(t *testing.T) {
	web := NewUntrusted(strings.Repeat("x", 200))
	var ridotto Untrusted = Summarize(web)
	if len(ridotto.String()) != 50 {
		t.Fatalf("atteso 50, ottenuto %d", len(ridotto.String()))
	}
}

func TestT2LaConversioneEsisteEdEEsplicita(t *testing.T) {
	web := NewUntrusted("contenuto arbitrario")
	var promosso Instruction = web.PromoteToInstruction("motivo registrato")
	if promosso.String() != "contenuto arbitrario" {
		t.Fatalf("conversione non riuscita: %q", promosso.String())
	}
}

func TestIlPromptSiCostruisceSoloDaIstruzioni(t *testing.T) {
	sistema := NewInstruction("sei un assistente")
	utente := NewInstruction("ciao")
	if got := BuildPrompt(sistema, utente); got != "sei un assistente\nciao" {
		t.Fatalf("prompt inatteso: %q", got)
	}
}

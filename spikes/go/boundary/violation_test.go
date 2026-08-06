package boundary_test

import (
	"os/exec"
	"strings"
	"testing"
)

// compilaViolazione compila un package normalmente escluso dal tag `violation`
// e restituisce l'output del compilatore e se la compilazione è fallita.
func compilaViolazione(t *testing.T, pkg string) (string, bool) {
	t.Helper()
	cmd := exec.Command("go", "build", "-tags", "violation", pkg)
	cmd.Dir = ".." // radice del modulo
	out, err := cmd.CombinedOutput()
	return string(out), err != nil
}

// TestT1IlNonFidatoNonCompila — passare un Untrusted dove è attesa un'Instruction.
func TestT1IlNonFidatoNonCompila(t *testing.T) {
	out, fallita := compilaViolazione(t, "./boundary/violation/")
	if !fallita {
		t.Fatalf("T1 VIOLATO: la violazione ha compilato")
	}
	if !strings.Contains(out, "cannot use dalWeb") {
		t.Fatalf("T1: fallita per il motivo sbagliato, che è un falso positivo.\n%s", out)
	}
	t.Logf("errore atteso del compilatore:\n%s", out)
}

// TestT4LaConversioneDirettaNonCompila — la trappola misurata il 2026-08-06.
//
// Se i due tipi avessero lo stesso tipo sottostante, `boundary.Instruction(u)`
// compilerebbe da qualunque punto del progetto, senza `unsafe` e senza reflection.
// Questo test è la guardia: se qualcuno riallineasse i nomi dei campi, fallisce.
func TestT4LaConversioneDirettaNonCompila(t *testing.T) {
	out, fallita := compilaViolazione(t, "./boundary/conversione/")
	if !fallita {
		t.Fatalf("T4 VIOLATO: la conversione diretta ha compilato. " +
			"Probabile causa: i campi di Instruction e Untrusted hanno lo stesso nome, " +
			"quindi i tipi sottostanti sono identici e la conversione è legale.")
	}
	if !strings.Contains(out, "cannot convert dalWeb") {
		t.Fatalf("T4: fallita per il motivo sbagliato, che è un falso positivo.\n%s", out)
	}
	t.Logf("errore atteso del compilatore:\n%s", out)
}

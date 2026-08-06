package kernel_test

import (
	"os/exec"
	"strings"
	"testing"
)

// vietati — package la cui presenza nel grafo del kernel viola I3, V28 o V29.
var vietati = []string{
	"os",
	"os/exec",
	"net",
	"net/http",
	"syscall",
	"math/rand",
	"math/rand/v2",
}

// dipendenze restituisce la chiusura transitiva delle importazioni di un package,
// prodotta dalla toolchain standard. Nessuno strumento esterno.
func dipendenze(t *testing.T, pkg string) []string {
	t.Helper()
	cmd := exec.Command("go", "list", "-deps", pkg)
	cmd.Dir = ".."
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("go list -deps %s: %v\n%s", pkg, err, out)
	}
	return strings.Fields(string(out))
}

func violazioni(deps []string) []string {
	var trovate []string
	for _, d := range deps {
		for _, v := range vietati {
			if d == v {
				trovate = append(trovate, d)
			}
		}
	}
	return trovate
}

// TestT6IlKernelNonImportaLOS è la regola di importazione vietata, applicata sul
// grafo reale e non su una convenzione.
func TestT6IlKernelNonImportaLOS(t *testing.T) {
	if v := violazioni(dipendenze(t, "./kernel/")); len(v) > 0 {
		t.Fatalf("T6 VIOLATO: il kernel dipende da %v", v)
	}
}

// TestT6LaRegolaNonEVacua — controprova. Il package platform *deve* risultare in
// violazione: se la regola non lo segnalasse, non starebbe controllando nulla.
func TestT6LaRegolaNonEVacua(t *testing.T) {
	v := violazioni(dipendenze(t, "./platform/"))
	if len(v) == 0 {
		t.Fatal("la regola non rileva nemmeno un package che importa os: e' vacua")
	}
	t.Logf("controprova riuscita: platform dipende da %v, come deve", v)
}

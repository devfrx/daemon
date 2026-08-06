// Package platform — l'unico package autorizzato a parlare con il sistema
// operativo. È il modulo di piattaforma di ADR-0002 e I3.
//
// Esiste nello spike solo per dare al test di T6 qualcosa da distinguere: senza
// un package *autorizzato*, la regola non proverebbe di saper discriminare.
package platform

import "os"

// DirectoryDiLavoro è una chiamata all'OS, legittima perché è qui.
func DirectoryDiLavoro() (string, error) {
	return os.Getwd()
}

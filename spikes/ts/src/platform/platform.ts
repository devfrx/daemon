// L'unico modulo autorizzato a parlare con il sistema operativo. È il modulo di
// piattaforma di ADR-0002 e I3.
//
// Esiste nello spike solo per dare al test di T6 qualcosa da distinguere: senza un
// modulo *autorizzato*, la regola non proverebbe di saper discriminare.

import { cwd } from "node:process";

export const directoryDiLavoro = (): string => cwd();

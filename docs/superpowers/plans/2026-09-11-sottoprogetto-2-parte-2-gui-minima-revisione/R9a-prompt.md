# R9a — Copertura del DISEGNO DEL 2, sezione per sezione, contro il piano

Rapporto: `<scratchpad>/review/R9a-report.md`. Vincoli: `constraints.md` (letto prima di questo).

## Che cosa leggi
- `skeleton.md` per intero; `p-titles.md`.
- `<disegno2>` **per intero**, dalla riga 1 alla 726, a blocchi di ≤120 righe: la testa, «Le risposte del proprietario», «Lo stato dell'arte», «Ciò che il codice diceva», le §1–§10, «Decisioni prese dal coordinatore», «Vicoli ciechi e trappole».
- Del piano, **a domanda**: le righe `D` (`grep -n '^| \*\*D<n>\*\* |'`), le voci `P` (per titolo), e i Passi dei compiti solo dove devi verificare che un Passo esista davvero (`awk` sul compito + `grep -n`).

## Il compito
Costruisci la **matrice di copertura**: per ogni riga o frase delle sezioni del disegno che nomina un **artefatto** (file, tipo, funzione, messaggio, prova, script, documento), una **decisione** («il piano decide…», «chi esegue…»), un **richiamo datato da scrivere**, un **debito lasciato al piano** («il piano dice come…»), o una **condizione** («approvata a condizione…») → quale compito e quale Passo la produce, con la frase del piano che lo prova (`grep -n`), ed esito: **coperta** / **scoperta** (nessun compito) / **contraddetta** (il piano fa altro: c'è una `D` o una `P` che lo dichiara, e il disegno riceve il richiamo datato in quale Passo?) / **spostata** (a un compito diverso da quello che il disegno suggerisce, con richiamo).

Insisti su:
1. la tabella della **§8** *«Il prodotto del 2, e il controllo che esercita ciascun artefatto»*: **D1** dice che i compiti sono tagliati su di essa — ogni riga → un compito; ogni controllo nominato → una sonda dettata (nome del file di prova); ogni riga della §8 sul cancello (`gate-gui.sh`, `npm ci`, la riga in `gate.sh`, la campagna DST nel settimo passo, la CI) → il 15/16/10;
2. la **§9** *«Le decisioni aperte col chiusore»*: ogni voce il cui chiusore è «il piano», «il 2», «questo piano», «chi esegue» → decisa da una `D` o dichiarata nelle voci aperte del piano ([C] dello scheletro)? Le voci il cui chiusore è un altro sotto-progetto → nella tabella [C]?
3. la **§10** *«Come si riprende»*: la lista numerata e il punto 7 (subagent-driven) → la testa del piano; ciò che la §10 dice di «leggere per intero» → il piano lo dice a chi esegue?
4. le **tre righe di documento che aspettano il loro compito** (l'undicesima chiusura del diario le elenca: `awk '/^### L.undicesima chiusura/{f=1} /^### La decima chiusura/{exit} f' <piano>` — la tabella «La riga | Chi la corregge»): il Passo 8 del compito 10 toglie `DyingGui` dalla §5; il compito 12 scrive il richiamo di D41 sulla §7 («senza copiarla»); il Passo 11 del compito 15 scrive il richiamo sui 🔶 dedotti della §8 — **verifica** che quei Passi esistano con quel contenuto e che i *Trova* siano unici in `<disegno2>`;
5. la **§1** «cosa il 2 NON costruisce» → nessun compito lo costruisce; la **§2** (chiusa dalla parte 1) → il piano non la rifà; la **§3** «a condizione» → la condizione è onorata dove?; la **§4** → l'enum del compito 3 (scheletro) variante per variante, «chi la manda» e «perché nel 2»; la **§5** → 6/7/9 riga per riga (`Request` non servito: D5 e il richiamo); la **§6a** strati/regole/ponte «quattro messaggi» → 11/13/14; la **§7** → 12, e i due debiti già chiusi (P-70, P-71) — il disegno riceve il richiamo?; **§6a e §8** che dicono «il valore atteso in JSON» (P-62/D35) → coerenti col 3;
6. **«Decisioni prese dal coordinatore»** e **«Vicoli ciechi e trappole»**: il piano ne cade in qualcuno? Ogni trappola → dove il piano la evita (frase);
7. ogni **cifra** del disegno che il piano ripete (numeri di varianti, di messaggi, di moduli, di prove): coincide?

Nel rapporto la sezione **«Copertura del disegno»** è la principale: una riga per ogni riga del disegno esaminata, anche le coperte (breve). I «Rilievi» sono le scoperte, le contraddette senza richiamo, e le cifre che non tornano.

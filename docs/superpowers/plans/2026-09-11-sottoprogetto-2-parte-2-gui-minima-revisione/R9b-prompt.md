# R9b — Copertura della STELLA POLARE della GUI contro il piano

Rapporto: `<scratchpad>/review/R9b-report.md`. Vincoli: `constraints.md` (letto prima di questo).

## Che cosa leggi (di `<stella>`, ritrovando ogni intestazione col `grep -n` prima; a blocchi di ≤100 righe — le righe sono lunghe)
- `## Le decisioni del proprietario, una per domanda` (≈66–119) — la tabella delle decisioni numerate;
- `## Il modello della GUI` (≈181–198) e `### La GUI dentro — le tre sequenze` (≈199–344);
- `### §2 — Viste e disposizione` (≈600–674); `### §3 — La fetta del 2 ritagliata` (≈675–742); `### §6 — Dove vive la stella polare` (≈830–866); `## Le sezioni, e dove vive ciascuna` (≈867–881);
- `## Decisioni prese dal coordinatore` (≈882–943); `## Registrate, non prese` (≈944–973); `## Vicoli ciechi e trappole` (≈974–1073); `## Il prossimo passo` (≈1074–1081).
- **Non** leggere la §1 (il catalogo, è di R7) né la §4 (lo spike, è di R6), se non per una riga che ti serve via `grep`.
- `skeleton.md` per intero; `p-titles.md`; del piano a domanda le righe `D`, le voci `P`, e i Passi dove devi provare che esistono.

## Il compito
La **matrice di copertura** della stella polare contro il piano:
1. **ogni decisione numerata del proprietario** il cui oggetto è il 2 (in particolare dalla 14 in poi: settima porta, `dockview`, Electron, X-1/X-3 = 44/45, `package-manager-cache` = 47, subagent = 48, lockfile = 49, `markdown-it` = 51, `vitest` = 52, e ogni altra) → dove il piano la onora (compito/Passo/D), o la **ribalta/precisa** con una `D` e un richiamo datato (es. D66 rende esplicita la 47; D63/D64 sul lint) — e se il richiamo datato **nella stella polare** è dettato da un Passo (quale?);
2. **le tre sequenze** del protocollo core ↔ GUI: ogni messaggio e ogni freccia → una variante del compito 3 (scheletro), un ramo del dispaccio del 7, uno store/azione della SPA (13/14): nomi identici? Un messaggio della sequenza che nessun compito manda o riceve?
3. **§2**: la settima porta (due operazioni, byte opachi, `toJSON`, la chiave), le tre viste, la disposizione salvata **per vista?** — il piano (4/5/7/13) la segue? Il tetto di dimensione «un parametro, quando serve» → il piano non lo inventa (o D9/D31?);
4. **§3**: righe 2–9 → compiti; **«Le sezioni del 2 che cambiano»** → ogni richiamo esiste già nel disegno del 2 (riscritto il 2026-09-09: `grep -n 'RICHIAMO DEL 2026-09-0[89]\|riscritta il 2026-09-09' <disegno2>`) o è dettato da un Passo; **«Cosa il 2 NON costruisce»** → niente nel piano lo costruisce; **«Debiti dichiarati»** (il titolo della roadmap → 17? le cifre di `ports/mod.rs` → 4; i richiami §2.3/§3.1 → 4); i **🔶 dedotti** della §3 (ordine dei pezzi, SPA contro il ponte finto prima del daemon, passi mandati all'accoglienza e dopo ogni invocazione) → confermati dal piano, e chi scrive il richiamo che li conferma?
5. **§6** e «Le sezioni, e dove vive ciascuna» → il 17 aggiorna ciò che va aggiornato (lo stato della stella polare dopo la parte 2)?
6. **«Decisioni prese dal coordinatore»** → il piano ne contraddice qualcuna senza richiamo?
7. **«Registrate, non prese»** → tutte nella tabella [C] delle voci aperte del piano, con lo stesso chiusore?
8. **«Vicoli ciechi e trappole»** → per ciascuna, la frase del piano che la evita, o il rilievo se ci cade;
9. **«Il prossimo passo»** della stella polare: è ancora vero oggi? Chi lo aggiorna (17)?

Nel rapporto la sezione **«Copertura del disegno»** è la principale: una riga per ogni decisione/riga esaminata. I «Rilievi» sono le scoperte, le contraddizioni senza richiamo, i richiami datati che nessun Passo scrive, i numeri di compito stantii.

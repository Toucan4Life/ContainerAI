# Cross-play of the baseline population (deployed modes, extended games, dev seeds)

Population: merchant, run1, run2, exploit_run2, control, defense, magnate.  Horizon 2500 actions, game length extended, seed block `0x400…`, 6 seeds per composition and table size, every composition played in all its seat rotations. Commit `2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-d2d17ac599ae`.

**20,412 games: 20,410 completed, 2 stalled (0.01%).** Utility: winner share: 1/|W| per winner (engine tie-break), 0 otherwise; stalled games pay 0 to every seat.

Policy modes (never pooled): merchant=scripted, run1=sample, run2=sample, exploit_run2=sample, control=sample, defense=sample, magnate=guarded.

## Accounting per strategy

| strategy | games seated in | stalled | stall rate |
|---|---|---|---|
| merchant | 8,820 | 0 | 0.00% |
| run1 | 8,820 | 2 | 0.02% |
| run2 | 8,820 | 2 | 0.02% |
| exploit_run2 | 8,820 | 1 | 0.01% |
| control | 8,820 | 0 | 0.00% |
| defense | 8,820 | 1 | 0.01% |
| magnate | 8,820 | 0 | 0.00% |

## 3 players: payoff of one seat of the row strategy at a table whose other 2 seats play the column strategy

1,512 games, 1 stalled (0.07%). The diagonal is the all-same table: (1 − stall rate) / 3. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense | magnate |
|---|---|---|---|---|---|---|---|
| **merchant** | 0.333 [0.333, 0.333] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.333 [0.333, 0.333] | 0.167 [0.056, 0.278] | 0.167 [0.056, 0.278] | 0.167 [0.056, 0.278] | 0.167 [0.000, 0.389] | 0.056 [0.000, 0.167] |
| **run2** | 1.000 [1.000, 1.000] | 0.444 [0.333, 0.556] | 0.333 [0.333, 0.333] | 0.278 [0.167, 0.333] | 0.222 [0.111, 0.333] | 0.222 [0.056, 0.444] | 0.278 [0.111, 0.444] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.444 [0.222, 0.667] | 0.556 [0.333, 0.667] | 0.333 [0.333, 0.333] | 0.444 [0.333, 0.556] | 0.333 [0.167, 0.500] | 0.333 [0.167, 0.500] |
| **control** | 1.000 [1.000, 1.000] | 0.500 [0.389, 0.611] | 0.333 [0.167, 0.500] | 0.333 [0.167, 0.500] | 0.333 [0.333, 0.333] | 0.222 [0.056, 0.444] | 0.389 [0.111, 0.611] |
| **defense** | 1.000 [1.000, 1.000] | 0.667 [0.500, 0.833] | 0.194 [0.056, 0.361] | 0.500 [0.389, 0.611] | 0.111 [0.000, 0.222] | 0.333 [0.333, 0.333] | 0.278 [0.222, 0.333] |
| **magnate** | 1.000 [1.000, 1.000] | 0.667 [0.500, 0.833] | 0.278 [0.111, 0.444] | 0.611 [0.500, 0.667] | 0.389 [0.333, 0.500] | 0.417 [0.333, 0.528] | 0.333 [0.333, 0.333] |

Best single deviation within the population against each field of 3 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.667 | [+0.667, +0.667] |
| run1 | defense | +0.333 | [+0.167, +0.500] |
| run2 | exploit_run2 | +0.222 | [+0.000, +0.333] |
| exploit_run2 | magnate | +0.278 | [+0.167, +0.333] |
| control | exploit_run2 | +0.111 | [+0.000, +0.222] |
| defense | magnate | +0.083 | [+0.000, +0.194] |
| magnate | control | +0.056 | [-0.222, +0.278] |

## 4 players: payoff of one seat of the row strategy at a table whose other 3 seats play the column strategy

5,040 games, 1 stalled (0.02%). The diagonal is the all-same table: (1 − stall rate) / 4. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense | magnate |
|---|---|---|---|---|---|---|---|
| **merchant** | 0.250 [0.250, 0.250] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.250 [0.250, 0.250] | 0.167 [0.083, 0.250] | 0.000 [0.000, 0.000] | 0.083 [0.000, 0.167] | 0.083 [0.000, 0.167] | 0.208 [0.083, 0.333] |
| **run2** | 1.000 [1.000, 1.000] | 0.396 [0.250, 0.583] | 0.250 [0.250, 0.250] | 0.167 [0.000, 0.333] | 0.104 [0.021, 0.188] | 0.208 [0.083, 0.333] | 0.083 [0.000, 0.167] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.250 [0.083, 0.417] | 0.292 [0.167, 0.417] | 0.250 [0.250, 0.250] | 0.208 [0.083, 0.333] | 0.396 [0.271, 0.562] | 0.167 [0.042, 0.333] |
| **control** | 1.000 [1.000, 1.000] | 0.458 [0.333, 0.583] | 0.312 [0.250, 0.396] | 0.375 [0.167, 0.542] | 0.250 [0.250, 0.250] | 0.083 [0.000, 0.167] | 0.083 [0.000, 0.167] |
| **defense** | 1.000 [1.000, 1.000] | 0.500 [0.333, 0.667] | 0.208 [0.125, 0.250] | 0.167 [0.083, 0.250] | 0.167 [0.083, 0.250] | 0.250 [0.250, 0.250] | 0.271 [0.146, 0.375] |
| **magnate** | 1.000 [1.000, 1.000] | 0.375 [0.250, 0.542] | 0.417 [0.250, 0.583] | 0.333 [0.083, 0.625] | 0.208 [0.083, 0.333] | 0.188 [0.042, 0.354] | 0.250 [0.250, 0.250] |

Best single deviation within the population against each field of 4 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.750 | [+0.750, +0.750] |
| run1 | defense | +0.250 | [+0.083, +0.417] |
| run2 | magnate | +0.167 | [+0.000, +0.333] |
| exploit_run2 | control | +0.125 | [-0.083, +0.292] |
| control | exploit_run2 | -0.042 | [-0.167, +0.083] |
| defense | exploit_run2 | +0.146 | [+0.021, +0.312] |
| magnate | defense | +0.021 | [-0.104, +0.125] |

## 5 players: payoff of one seat of the row strategy at a table whose other 4 seats play the column strategy

13,860 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 5. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense | magnate |
|---|---|---|---|---|---|---|---|
| **merchant** | 0.200 [0.200, 0.200] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.200 [0.200, 0.200] | 0.000 [0.000, 0.000] | 0.033 [0.000, 0.100] | 0.033 [0.000, 0.100] | 0.033 [0.000, 0.100] | 0.033 [0.000, 0.100] |
| **run2** | 1.000 [1.000, 1.000] | 0.367 [0.267, 0.467] | 0.200 [0.200, 0.200] | 0.267 [0.100, 0.433] | 0.100 [0.000, 0.233] | 0.300 [0.133, 0.500] | 0.067 [0.000, 0.133] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.533 [0.433, 0.667] | 0.300 [0.100, 0.500] | 0.200 [0.200, 0.200] | 0.100 [0.033, 0.167] | 0.267 [0.133, 0.367] | 0.100 [0.033, 0.167] |
| **control** | 1.000 [1.000, 1.000] | 0.300 [0.133, 0.433] | 0.167 [0.033, 0.333] | 0.367 [0.133, 0.600] | 0.200 [0.200, 0.200] | 0.133 [0.033, 0.267] | 0.200 [0.067, 0.333] |
| **defense** | 1.000 [1.000, 1.000] | 0.433 [0.333, 0.533] | 0.267 [0.133, 0.433] | 0.467 [0.200, 0.733] | 0.067 [0.000, 0.133] | 0.200 [0.200, 0.200] | 0.200 [0.200, 0.200] |
| **magnate** | 1.000 [1.000, 1.000] | 0.450 [0.233, 0.667] | 0.333 [0.133, 0.533] | 0.400 [0.233, 0.600] | 0.300 [0.167, 0.400] | 0.250 [0.117, 0.400] | 0.200 [0.200, 0.200] |

Best single deviation within the population against each field of 5 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.800 | [+0.800, +0.800] |
| run1 | exploit_run2 | +0.333 | [+0.233, +0.467] |
| run2 | magnate | +0.133 | [-0.067, +0.333] |
| exploit_run2 | defense | +0.267 | [+0.000, +0.533] |
| control | magnate | +0.100 | [-0.033, +0.200] |
| defense | run2 | +0.100 | [-0.067, +0.300] |
| magnate | control | +0.000 | [-0.133, +0.133] |

## Per strategy over all the seats it occupies in the design

Compositions are weighted by their games. `fair share` is n × utility, averaged (1.00 = an equal player at any table size). A tie is a shared win. Score margin is own total minus the mean of the opponents' totals and length is in engine actions; both over completed games.

| strategy | seat-games | fair share | win rate | tie rate | score margin | actions per game |
|---|---|---|---|---|---|---|
| merchant | 13,428 | 0.022 | 0.005 | 0.000 | -101.4 | 1031 |
| run1 | 13,428 | 0.640 | 0.142 | 0.003 | +5.8 | 1097 |
| run2 | 13,428 | 1.162 | 0.254 | 0.005 | +16.7 | 1112 |
| exploit_run2 | 13,428 | 1.028 | 0.229 | 0.006 | +16.5 | 1138 |
| control | 13,428 | 1.315 | 0.287 | 0.005 | +19.8 | 1133 |
| defense | 13,428 | 1.280 | 0.281 | 0.007 | +19.1 | 1129 |
| magnate | 13,428 | 1.552 | 0.338 | 0.005 | +23.5 | 1133 |

## By turn position

Fair share (n × utility) of the player in each turn position over all games, with 95% seed-clustered CI; position 0 moves first. The engine draws the first player at random for every game and every design seats each strategy in every position equally often, so a position effect is averaged out of the payoffs above, not part of them. 1.00 means no position effect; a position that is above 1.00 is above an equal player's share.

| players | position 0 | position 1 | position 2 | position 3 | position 4 | actions per game (mean, p99) |
|---|---|---|---|---|---|---|
| 3 | 0.805 [0.621, 1.048] | 1.028 [0.887, 1.189] | 1.166 [0.946, 1.383] | — | — | 467, 1066 |
| 4 | 1.032 [0.787, 1.251] | 0.989 [0.782, 1.156] | 1.024 [0.848, 1.183] | 0.954 [0.719, 1.156] | — | 869, 1178 |
| 5 | 1.031 [0.944, 1.122] | 1.025 [0.888, 1.149] | 1.107 [0.954, 1.242] | 0.855 [0.774, 0.928] | 0.983 [0.916, 1.050] | 1223, 1490 |

## Secondary: ladder-style ELO of these games

For continuity with `training/ladder.json` only. It ignores stalled games and weights compositions uniformly, so it is not comparable with the ladder's mixed-pool sampling, and it says nothing about exploitability.

| strategy | ELO |
|---|---|
| magnate | 1923.1 |
| control | 1877.4 |
| defense | 1869.9 |
| exploit_run2 | 1854.1 |
| run2 | 1838.1 |
| run1 | 1700.8 |
| merchant | 0.0 |

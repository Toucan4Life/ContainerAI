# Cross-play of the baseline population (deployed modes, short games, dev seeds)

Population: merchant, run1, run2, exploit_run2, control, defense, magnate.  Horizon 2500 actions, game length short, seed block `0x400…`, 6 seeds per composition and table size, every composition played in all its seat rotations. Commit `2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-d2d17ac599ae`.

**20,412 games: 20,410 completed, 2 stalled (0.01%).** Utility: winner share: 1/|W| per winner (engine tie-break), 0 otherwise; stalled games pay 0 to every seat.

Policy modes (never pooled): merchant=scripted, run1=sample, run2=sample, exploit_run2=sample, control=sample, defense=sample, magnate=guarded.

## Accounting per strategy

| strategy | games seated in | stalled | stall rate |
|---|---|---|---|
| merchant | 8,820 | 0 | 0.00% |
| run1 | 8,820 | 2 | 0.02% |
| run2 | 8,820 | 1 | 0.01% |
| exploit_run2 | 8,820 | 0 | 0.00% |
| control | 8,820 | 0 | 0.00% |
| defense | 8,820 | 0 | 0.00% |
| magnate | 8,820 | 0 | 0.00% |

## 3 players: payoff of one seat of the row strategy at a table whose other 2 seats play the column strategy

1,512 games, 2 stalled (0.13%). The diagonal is the all-same table: (1 − stall rate) / 3. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense | magnate |
|---|---|---|---|---|---|---|---|
| **merchant** | 0.333 [0.333, 0.333] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.315 [0.278, 0.333] | 0.278 [0.111, 0.444] | 0.167 [0.056, 0.278] | 0.111 [0.000, 0.222] | 0.139 [0.000, 0.361] | 0.167 [0.056, 0.278] |
| **run2** | 1.000 [1.000, 1.000] | 0.556 [0.278, 0.778] | 0.333 [0.333, 0.333] | 0.222 [0.111, 0.333] | 0.278 [0.111, 0.444] | 0.278 [0.111, 0.444] | 0.139 [0.028, 0.250] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.333 [0.111, 0.611] | 0.389 [0.222, 0.556] | 0.333 [0.333, 0.333] | 0.472 [0.278, 0.722] | 0.278 [0.167, 0.333] | 0.139 [0.028, 0.250] |
| **control** | 1.000 [1.000, 1.000] | 0.444 [0.222, 0.611] | 0.500 [0.278, 0.722] | 0.222 [0.056, 0.444] | 0.333 [0.333, 0.333] | 0.139 [0.000, 0.361] | 0.222 [0.111, 0.333] |
| **defense** | 1.000 [1.000, 1.000] | 0.444 [0.333, 0.556] | 0.333 [0.167, 0.500] | 0.222 [0.111, 0.333] | 0.333 [0.333, 0.333] | 0.333 [0.333, 0.333] | 0.333 [0.333, 0.333] |
| **magnate** | 1.000 [1.000, 1.000] | 0.500 [0.333, 0.722] | 0.500 [0.333, 0.722] | 0.222 [0.111, 0.333] | 0.333 [0.167, 0.500] | 0.222 [0.056, 0.444] | 0.333 [0.333, 0.333] |

Best single deviation within the population against each field of 3 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.667 | [+0.667, +0.667] |
| run1 | run2 | +0.241 | [+0.000, +0.444] |
| run2 | control | +0.167 | [-0.056, +0.389] |
| exploit_run2 | run2 | -0.111 | [-0.222, +0.000] |
| control | exploit_run2 | +0.139 | [-0.056, +0.389] |
| defense | run2 | -0.056 | [-0.222, +0.111] |
| magnate | defense | +0.000 | [+0.000, +0.000] |

## 4 players: payoff of one seat of the row strategy at a table whose other 3 seats play the column strategy

5,040 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 4. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense | magnate |
|---|---|---|---|---|---|---|---|
| **merchant** | 0.250 [0.250, 0.250] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.250 [0.250, 0.250] | 0.042 [0.000, 0.125] | 0.042 [0.000, 0.125] | 0.000 [0.000, 0.000] | 0.083 [0.000, 0.167] | 0.042 [0.000, 0.125] |
| **run2** | 1.000 [1.000, 1.000] | 0.312 [0.167, 0.438] | 0.250 [0.250, 0.250] | 0.250 [0.125, 0.375] | 0.208 [0.083, 0.333] | 0.208 [0.083, 0.333] | 0.250 [0.125, 0.375] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.354 [0.167, 0.521] | 0.354 [0.167, 0.521] | 0.250 [0.250, 0.250] | 0.333 [0.250, 0.417] | 0.125 [0.042, 0.208] | 0.250 [0.083, 0.417] |
| **control** | 1.000 [1.000, 1.000] | 0.521 [0.292, 0.688] | 0.333 [0.083, 0.583] | 0.167 [0.042, 0.333] | 0.250 [0.250, 0.250] | 0.375 [0.208, 0.500] | 0.208 [0.083, 0.333] |
| **defense** | 1.000 [1.000, 1.000] | 0.458 [0.333, 0.583] | 0.417 [0.208, 0.583] | 0.292 [0.167, 0.417] | 0.167 [0.083, 0.250] | 0.250 [0.250, 0.250] | 0.312 [0.167, 0.500] |
| **magnate** | 1.000 [1.000, 1.000] | 0.458 [0.292, 0.625] | 0.333 [0.167, 0.458] | 0.333 [0.250, 0.417] | 0.333 [0.167, 0.542] | 0.438 [0.271, 0.604] | 0.250 [0.250, 0.250] |

Best single deviation within the population against each field of 4 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.750 | [+0.750, +0.750] |
| run1 | control | +0.271 | [+0.042, +0.438] |
| run2 | defense | +0.167 | [-0.042, +0.333] |
| exploit_run2 | magnate | +0.083 | [+0.000, +0.167] |
| control | exploit_run2 | +0.083 | [+0.000, +0.167] |
| defense | magnate | +0.188 | [+0.021, +0.354] |
| magnate | defense | +0.062 | [-0.083, +0.250] |

## 5 players: payoff of one seat of the row strategy at a table whose other 4 seats play the column strategy

13,860 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 5. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense | magnate |
|---|---|---|---|---|---|---|---|
| **merchant** | 0.200 [0.200, 0.200] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.200 [0.200, 0.200] | 0.000 [0.000, 0.000] | 0.067 [0.000, 0.200] | 0.033 [0.000, 0.100] | 0.067 [0.000, 0.133] | 0.000 [0.000, 0.000] |
| **run2** | 1.000 [1.000, 1.000] | 0.333 [0.167, 0.500] | 0.200 [0.200, 0.200] | 0.200 [0.100, 0.300] | 0.200 [0.100, 0.300] | 0.117 [0.017, 0.250] | 0.233 [0.133, 0.333] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.400 [0.267, 0.533] | 0.250 [0.133, 0.383] | 0.200 [0.200, 0.200] | 0.233 [0.133, 0.333] | 0.117 [0.050, 0.183] | 0.133 [0.033, 0.267] |
| **control** | 1.000 [1.000, 1.000] | 0.417 [0.400, 0.450] | 0.283 [0.150, 0.383] | 0.267 [0.133, 0.367] | 0.200 [0.200, 0.200] | 0.117 [0.017, 0.233] | 0.100 [0.033, 0.167] |
| **defense** | 1.000 [1.000, 1.000] | 0.467 [0.267, 0.667] | 0.317 [0.167, 0.517] | 0.267 [0.133, 0.433] | 0.217 [0.117, 0.317] | 0.200 [0.200, 0.200] | 0.233 [0.067, 0.400] |
| **magnate** | 1.000 [1.000, 1.000] | 0.533 [0.333, 0.733] | 0.283 [0.100, 0.467] | 0.250 [0.133, 0.383] | 0.333 [0.167, 0.467] | 0.267 [0.133, 0.433] | 0.200 [0.200, 0.200] |

Best single deviation within the population against each field of 5 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.800 | [+0.800, +0.800] |
| run1 | magnate | +0.333 | [+0.133, +0.533] |
| run2 | defense | +0.117 | [-0.033, +0.317] |
| exploit_run2 | control | +0.067 | [-0.067, +0.167] |
| control | magnate | +0.133 | [-0.033, +0.267] |
| defense | magnate | +0.067 | [-0.067, +0.233] |
| magnate | run2 | +0.033 | [-0.067, +0.133] |

## Per strategy over all the seats it occupies in the design

Compositions are weighted by their games. `fair share` is n × utility, averaged (1.00 = an equal player at any table size). A tie is a shared win. Score margin is own total minus the mean of the opponents' totals and length is in engine actions; both over completed games.

| strategy | seat-games | fair share | win rate | tie rate | score margin | actions per game |
|---|---|---|---|---|---|---|
| merchant | 13,428 | 0.022 | 0.005 | 0.000 | -63.6 | 600 |
| run1 | 13,428 | 0.634 | 0.142 | 0.005 | +3.2 | 662 |
| run2 | 13,428 | 1.141 | 0.251 | 0.008 | +10.2 | 668 |
| exploit_run2 | 13,428 | 1.099 | 0.244 | 0.008 | +10.9 | 684 |
| control | 13,428 | 1.336 | 0.293 | 0.008 | +12.5 | 680 |
| defense | 13,428 | 1.277 | 0.280 | 0.007 | +12.2 | 682 |
| magnate | 13,428 | 1.491 | 0.327 | 0.010 | +14.5 | 683 |

## By turn position

Fair share (n × utility) of the player in each turn position over all games, with 95% seed-clustered CI; position 0 moves first. The engine draws the first player at random for every game and every design seats each strategy in every position equally often, so a position effect is averaged out of the payoffs above, not part of them. 1.00 means no position effect; a position that is above 1.00 is above an equal player's share.

| players | position 0 | position 1 | position 2 | position 3 | position 4 | actions per game (mean, p99) |
|---|---|---|---|---|---|---|
| 3 | 0.875 [0.697, 1.068] | 0.935 [0.687, 1.162] | 1.187 [1.043, 1.323] | — | — | 319, 590 |
| 4 | 1.065 [0.817, 1.287] | 0.897 [0.659, 1.132] | 0.984 [0.798, 1.185] | 1.055 [0.847, 1.212] | — | 539, 700 |
| 5 | 1.125 [1.007, 1.244] | 0.993 [0.908, 1.090] | 0.982 [0.926, 1.016] | 0.867 [0.735, 0.990] | 1.033 [0.922, 1.126] | 725, 902 |

## Secondary: ladder-style ELO of these games

For continuity with `training/ladder.json` only. It ignores stalled games and weights compositions uniformly, so it is not comparable with the ladder's mixed-pool sampling, and it says nothing about exploitability.

| strategy | ELO |
|---|---|
| magnate | 1614.8 |
| control | 1580.9 |
| defense | 1574.5 |
| exploit_run2 | 1565.9 |
| run2 | 1537.4 |
| run1 | 1406.9 |
| merchant | 0.0 |

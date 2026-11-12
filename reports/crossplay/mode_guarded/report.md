# Cross-play of the baseline nets, all nets in mode: guarded (dev seeds)

Population: merchant, run1, run2, exploit_run2, control, defense.  Horizon 2500 actions, game length standard, seed block `0x400…`, 12 seeds per composition and table size, every composition played in all its seat rotations. Commit `2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty`.

**23,184 games: 23,184 completed, 0 stalled (0.00%).** Utility: winner share: 1/|W| per winner (engine tie-break), 0 otherwise; stalled games pay 0 to every seat.

Policy modes (never pooled): merchant=scripted, run1=guarded, run2=guarded, exploit_run2=guarded, control=guarded, defense=guarded.

## Accounting per strategy

| strategy | games seated in | stalled | stall rate |
|---|---|---|---|
| merchant | 11,004 | 0 | 0.00% |
| run1 | 11,004 | 0 | 0.00% |
| run2 | 11,004 | 0 | 0.00% |
| exploit_run2 | 11,004 | 0 | 0.00% |
| control | 11,004 | 0 | 0.00% |
| defense | 11,004 | 0 | 0.00% |

## 3 players: payoff of one seat of the row strategy at a table whose other 2 seats play the column strategy

2,016 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 3. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense |
|---|---|---|---|---|---|---|
| **merchant** | 0.333 [0.333, 0.333] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.333 [0.333, 0.333] | 0.111 [0.000, 0.222] | 0.250 [0.139, 0.361] | 0.111 [0.028, 0.194] | 0.222 [0.111, 0.361] |
| **run2** | 1.000 [1.000, 1.000] | 0.403 [0.250, 0.569] | 0.333 [0.333, 0.333] | 0.361 [0.278, 0.444] | 0.194 [0.083, 0.306] | 0.306 [0.222, 0.389] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.611 [0.444, 0.778] | 0.389 [0.250, 0.528] | 0.333 [0.333, 0.333] | 0.278 [0.167, 0.389] | 0.333 [0.194, 0.472] |
| **control** | 1.000 [1.000, 1.000] | 0.611 [0.472, 0.750] | 0.389 [0.250, 0.528] | 0.361 [0.222, 0.528] | 0.333 [0.333, 0.333] | 0.306 [0.167, 0.444] |
| **defense** | 1.000 [1.000, 1.000] | 0.528 [0.361, 0.694] | 0.417 [0.278, 0.556] | 0.417 [0.278, 0.556] | 0.361 [0.222, 0.500] | 0.333 [0.333, 0.333] |

Best single deviation within the population against each field of 3 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.667 | [+0.667, +0.667] |
| run1 | control | +0.278 | [+0.139, +0.417] |
| run2 | defense | +0.083 | [-0.056, +0.222] |
| exploit_run2 | defense | +0.083 | [-0.056, +0.222] |
| control | defense | +0.028 | [-0.111, +0.167] |
| defense | exploit_run2 | +0.000 | [-0.139, +0.139] |

## 4 players: payoff of one seat of the row strategy at a table whose other 3 seats play the column strategy

6,048 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 4. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense |
|---|---|---|---|---|---|---|
| **merchant** | 0.250 [0.250, 0.250] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.250 [0.250, 0.250] | 0.125 [0.042, 0.229] | 0.062 [0.000, 0.125] | 0.000 [0.000, 0.000] | 0.042 [0.000, 0.104] |
| **run2** | 1.000 [1.000, 1.000] | 0.417 [0.250, 0.583] | 0.250 [0.250, 0.250] | 0.146 [0.062, 0.250] | 0.198 [0.104, 0.302] | 0.115 [0.042, 0.188] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.510 [0.344, 0.667] | 0.319 [0.229, 0.417] | 0.250 [0.250, 0.250] | 0.240 [0.135, 0.354] | 0.333 [0.250, 0.417] |
| **control** | 1.000 [1.000, 1.000] | 0.479 [0.354, 0.604] | 0.292 [0.188, 0.396] | 0.208 [0.104, 0.312] | 0.250 [0.250, 0.250] | 0.323 [0.229, 0.438] |
| **defense** | 1.000 [1.000, 1.000] | 0.479 [0.375, 0.583] | 0.385 [0.281, 0.490] | 0.271 [0.167, 0.375] | 0.208 [0.104, 0.312] | 0.250 [0.250, 0.250] |

Best single deviation within the population against each field of 4 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.750 | [+0.750, +0.750] |
| run1 | exploit_run2 | +0.260 | [+0.094, +0.417] |
| run2 | defense | +0.135 | [+0.031, +0.240] |
| exploit_run2 | defense | +0.021 | [-0.083, +0.125] |
| control | exploit_run2 | -0.010 | [-0.115, +0.104] |
| defense | exploit_run2 | +0.083 | [+0.000, +0.167] |

## 5 players: payoff of one seat of the row strategy at a table whose other 4 seats play the column strategy

15,120 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 5. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense |
|---|---|---|---|---|---|---|
| **merchant** | 0.200 [0.200, 0.200] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.200 [0.200, 0.200] | 0.033 [0.000, 0.083] | 0.050 [0.000, 0.100] | 0.017 [0.000, 0.050] | 0.017 [0.000, 0.050] |
| **run2** | 1.000 [1.000, 1.000] | 0.342 [0.225, 0.450] | 0.200 [0.200, 0.200] | 0.200 [0.116, 0.283] | 0.067 [0.000, 0.150] | 0.150 [0.050, 0.267] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.467 [0.400, 0.533] | 0.383 [0.275, 0.492] | 0.200 [0.200, 0.200] | 0.200 [0.117, 0.300] | 0.183 [0.100, 0.267] |
| **control** | 1.000 [1.000, 1.000] | 0.333 [0.266, 0.400] | 0.233 [0.150, 0.333] | 0.292 [0.192, 0.400] | 0.200 [0.200, 0.200] | 0.192 [0.100, 0.292] |
| **defense** | 1.000 [1.000, 1.000] | 0.358 [0.258, 0.450] | 0.200 [0.100, 0.317] | 0.367 [0.283, 0.450] | 0.233 [0.133, 0.333] | 0.200 [0.200, 0.200] |

Best single deviation within the population against each field of 5 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.800 | [+0.800, +0.800] |
| run1 | exploit_run2 | +0.267 | [+0.200, +0.333] |
| run2 | exploit_run2 | +0.183 | [+0.075, +0.292] |
| exploit_run2 | defense | +0.167 | [+0.083, +0.250] |
| control | defense | +0.033 | [-0.067, +0.133] |
| defense | control | -0.008 | [-0.100, +0.092] |

## Per strategy over all the seats it occupies in the design

Compositions are weighted by their games. `fair share` is n × utility, averaged (1.00 = an equal player at any table size). A tie is a shared win. Score margin is own total minus the mean of the opponents' totals and length is in engine actions; both over completed games.

| strategy | seat-games | fair share | win rate | tie rate | score margin | actions per game |
|---|---|---|---|---|---|---|
| merchant | 17,640 | 0.034 | 0.008 | 0.000 | -83.1 | 836 |
| run1 | 17,640 | 0.697 | 0.155 | 0.003 | +7.4 | 900 |
| run2 | 17,640 | 1.245 | 0.274 | 0.005 | +17.5 | 910 |
| exploit_run2 | 17,640 | 1.129 | 0.252 | 0.004 | +17.0 | 932 |
| control | 17,640 | 1.453 | 0.320 | 0.005 | +20.8 | 932 |
| defense | 17,640 | 1.442 | 0.318 | 0.007 | +20.5 | 930 |

## By turn position

Fair share (n × utility) of the player in each turn position over all games, with 95% seed-clustered CI; position 0 moves first. The engine draws the first player at random for every game and every design seats each strategy in every position equally often, so a position effect is averaged out of the payoffs above, not part of them. 1.00 means no position effect; a position that is above 1.00 is above an equal player's share.

| players | position 0 | position 1 | position 2 | position 3 | position 4 | actions per game (mean, p99) |
|---|---|---|---|---|---|---|
| 3 | 1.179 [0.998, 1.345] | 0.817 [0.682, 0.944] | 1.004 [0.876, 1.159] | — | — | 418, 759 |
| 4 | 1.140 [1.008, 1.279] | 1.094 [0.960, 1.229] | 0.922 [0.782, 1.064] | 0.845 [0.700, 1.000] | — | 747, 1085 |
| 5 | 0.997 [0.919, 1.078] | 1.020 [0.941, 1.103] | 0.998 [0.907, 1.099] | 0.982 [0.887, 1.087] | 1.004 [0.921, 1.080] | 997, 1236 |

## Secondary: ladder-style ELO of these games

For continuity with `training/ladder.json` only. It ignores stalled games and weights compositions uniformly, so it is not comparable with the ladder's mixed-pool sampling, and it says nothing about exploitability.

| strategy | ELO |
|---|---|
| defense | 1868.2 |
| control | 1868.1 |
| exploit_run2 | 1838.0 |
| run2 | 1819.2 |
| run1 | 1671.3 |
| merchant | 0.0 |

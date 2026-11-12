# Cross-play of the baseline nets, all nets in mode: greedy (dev seeds)

Population: merchant, run1, run2, exploit_run2, control, defense.  Horizon 2500 actions, game length standard, seed block `0x400…`, 12 seeds per composition and table size, every composition played in all its seat rotations. Commit `2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty`.

**23,184 games: 22,794 completed, 390 stalled (1.68%).** Utility: winner share: 1/|W| per winner (engine tie-break), 0 otherwise; stalled games pay 0 to every seat.

Policy modes (never pooled): merchant=scripted, run1=greedy, run2=greedy, exploit_run2=greedy, control=greedy, defense=greedy.

## Accounting per strategy

| strategy | games seated in | stalled | stall rate |
|---|---|---|---|
| merchant | 11,004 | 72 | 0.65% |
| run1 | 11,004 | 214 | 1.94% |
| run2 | 11,004 | 164 | 1.49% |
| exploit_run2 | 11,004 | 134 | 1.22% |
| control | 11,004 | 173 | 1.57% |
| defense | 11,004 | 162 | 1.47% |

## 3 players: payoff of one seat of the row strategy at a table whose other 2 seats play the column strategy

2,016 games, 257 stalled (12.75%). The diagonal is the all-same table: (1 − stall rate) / 3. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense |
|---|---|---|---|---|---|---|
| **merchant** | 0.333 [0.333, 0.333] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.194 [0.111, 0.278] | 0.111 [0.000, 0.222] | 0.222 [0.139, 0.306] | 0.111 [0.028, 0.194] | 0.139 [0.056, 0.222] |
| **run2** | 1.000 [1.000, 1.000] | 0.292 [0.139, 0.458] | 0.222 [0.139, 0.306] | 0.361 [0.278, 0.444] | 0.167 [0.056, 0.306] | 0.278 [0.167, 0.389] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.389 [0.222, 0.556] | 0.306 [0.167, 0.444] | 0.333 [0.333, 0.333] | 0.278 [0.167, 0.389] | 0.278 [0.139, 0.417] |
| **control** | 1.000 [1.000, 1.000] | 0.472 [0.361, 0.611] | 0.333 [0.194, 0.472] | 0.278 [0.139, 0.444] | 0.250 [0.167, 0.333] | 0.250 [0.111, 0.389] |
| **defense** | 1.000 [1.000, 1.000] | 0.306 [0.167, 0.472] | 0.417 [0.250, 0.556] | 0.417 [0.278, 0.556] | 0.333 [0.194, 0.500] | 0.278 [0.194, 0.333] |

Best single deviation within the population against each field of 3 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.667 | [+0.667, +0.667] |
| run1 | control | +0.278 | [+0.167, +0.417] |
| run2 | defense | +0.194 | [+0.028, +0.361] |
| exploit_run2 | defense | +0.083 | [-0.056, +0.222] |
| control | defense | +0.083 | [-0.056, +0.222] |
| defense | run2 | +0.000 | [-0.111, +0.111] |

## 4 players: payoff of one seat of the row strategy at a table whose other 3 seats play the column strategy

6,048 games, 85 stalled (1.41%). The diagonal is the all-same table: (1 − stall rate) / 4. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense |
|---|---|---|---|---|---|---|
| **merchant** | 0.250 [0.250, 0.250] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.250 [0.250, 0.250] | 0.125 [0.042, 0.229] | 0.062 [0.000, 0.125] | 0.000 [0.000, 0.000] | 0.042 [0.000, 0.104] |
| **run2** | 1.000 [1.000, 1.000] | 0.396 [0.250, 0.542] | 0.208 [0.146, 0.250] | 0.125 [0.042, 0.229] | 0.198 [0.104, 0.302] | 0.115 [0.042, 0.188] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.469 [0.312, 0.615] | 0.319 [0.229, 0.417] | 0.250 [0.250, 0.250] | 0.240 [0.135, 0.354] | 0.333 [0.250, 0.417] |
| **control** | 1.000 [1.000, 1.000] | 0.479 [0.354, 0.604] | 0.292 [0.188, 0.396] | 0.208 [0.104, 0.312] | 0.250 [0.250, 0.250] | 0.302 [0.198, 0.427] |
| **defense** | 1.000 [1.000, 1.000] | 0.458 [0.333, 0.583] | 0.385 [0.281, 0.490] | 0.271 [0.167, 0.375] | 0.208 [0.104, 0.312] | 0.250 [0.250, 0.250] |

Best single deviation within the population against each field of 4 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.750 | [+0.750, +0.750] |
| run1 | control | +0.229 | [+0.104, +0.354] |
| run2 | defense | +0.177 | [+0.052, +0.302] |
| exploit_run2 | defense | +0.021 | [-0.083, +0.125] |
| control | exploit_run2 | -0.010 | [-0.115, +0.104] |
| defense | exploit_run2 | +0.083 | [+0.000, +0.167] |

## 5 players: payoff of one seat of the row strategy at a table whose other 4 seats play the column strategy

15,120 games, 48 stalled (0.32%). The diagonal is the all-same table: (1 − stall rate) / 5. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense |
|---|---|---|---|---|---|---|
| **merchant** | 0.200 [0.200, 0.200] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.200 [0.200, 0.200] | 0.033 [0.000, 0.083] | 0.050 [0.000, 0.100] | 0.017 [0.000, 0.050] | 0.017 [0.000, 0.050] |
| **run2** | 1.000 [1.000, 1.000] | 0.333 [0.217, 0.433] | 0.200 [0.200, 0.200] | 0.200 [0.116, 0.283] | 0.067 [0.000, 0.150] | 0.150 [0.050, 0.267] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.467 [0.400, 0.533] | 0.367 [0.242, 0.483] | 0.200 [0.200, 0.200] | 0.200 [0.117, 0.300] | 0.183 [0.100, 0.267] |
| **control** | 1.000 [1.000, 1.000] | 0.333 [0.266, 0.400] | 0.233 [0.150, 0.333] | 0.292 [0.192, 0.400] | 0.200 [0.200, 0.200] | 0.175 [0.100, 0.258] |
| **defense** | 1.000 [1.000, 1.000] | 0.358 [0.258, 0.450] | 0.200 [0.100, 0.317] | 0.367 [0.283, 0.450] | 0.233 [0.133, 0.333] | 0.183 [0.150, 0.200] |

Best single deviation within the population against each field of 5 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.800 | [+0.800, +0.800] |
| run1 | exploit_run2 | +0.267 | [+0.200, +0.333] |
| run2 | exploit_run2 | +0.167 | [+0.042, +0.283] |
| exploit_run2 | defense | +0.167 | [+0.083, +0.250] |
| control | defense | +0.033 | [-0.067, +0.133] |
| defense | exploit_run2 | +0.000 | [-0.083, +0.083] |

## Per strategy over all the seats it occupies in the design

Compositions are weighted by their games. `fair share` is n × utility, averaged (1.00 = an equal player at any table size). A tie is a shared win. Score margin is own total minus the mean of the opponents' totals and length is in engine actions; both over completed games.

| strategy | seat-games | fair share | win rate | tie rate | score margin | actions per game |
|---|---|---|---|---|---|---|
| merchant | 17,640 | 0.034 | 0.008 | 0.000 | -82.9 | 836 |
| run1 | 17,640 | 0.685 | 0.151 | 0.002 | +7.6 | 902 |
| run2 | 17,640 | 1.229 | 0.269 | 0.005 | +17.7 | 913 |
| exploit_run2 | 17,640 | 1.119 | 0.249 | 0.004 | +17.1 | 933 |
| control | 17,640 | 1.433 | 0.314 | 0.005 | +20.9 | 933 |
| defense | 17,640 | 1.424 | 0.312 | 0.007 | +20.6 | 932 |

## By turn position

Fair share (n × utility) of the player in each turn position over all games, with 95% seed-clustered CI; position 0 moves first. The engine draws the first player at random for every game and every design seats each strategy in every position equally often, so a position effect is averaged out of the payoffs above, not part of them. 1.00 means no position effect; a position that is above 1.00 is above an equal player's share.

| players | position 0 | position 1 | position 2 | position 3 | position 4 | actions per game (mean, p99) |
|---|---|---|---|---|---|---|
| 3 | 1.035 [0.865, 1.197] | 0.719 [0.619, 0.818] | 0.863 [0.738, 0.986] | — | — | 394, 564 |
| 4 | 1.130 [1.002, 1.265] | 1.075 [0.942, 1.209] | 0.908 [0.769, 1.049] | 0.831 [0.685, 0.986] | — | 744, 1059 |
| 5 | 0.993 [0.918, 1.071] | 1.015 [0.936, 1.098] | 0.995 [0.905, 1.096] | 0.979 [0.885, 1.083] | 1.002 [0.920, 1.078] | 996, 1227 |

## Secondary: ladder-style ELO of these games

For continuity with `training/ladder.json` only. It ignores stalled games and weights compositions uniformly, so it is not comparable with the ladder's mixed-pool sampling, and it says nothing about exploitability.

| strategy | ELO |
|---|---|
| defense | 1867.8 |
| control | 1867.7 |
| exploit_run2 | 1837.5 |
| run2 | 1818.5 |
| run1 | 1670.5 |
| merchant | 0.0 |

# Cross-play of the baseline nets, all nets in mode: sample (dev seeds)

Population: merchant, run1, run2, exploit_run2, control, defense.  Horizon 2500 actions, game length standard, seed block `0x400…`, 12 seeds per composition and table size, every composition played in all its seat rotations. Commit `2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty`.

**23,184 games: 23,183 completed, 1 stalled (0.00%).** Utility: winner share: 1/|W| per winner (engine tie-break), 0 otherwise; stalled games pay 0 to every seat.

Policy modes (never pooled): merchant=scripted, run1=sample, run2=sample, exploit_run2=sample, control=sample, defense=sample.

## Accounting per strategy

| strategy | games seated in | stalled | stall rate |
|---|---|---|---|
| merchant | 11,004 | 0 | 0.00% |
| run1 | 11,004 | 0 | 0.00% |
| run2 | 11,004 | 0 | 0.00% |
| exploit_run2 | 11,004 | 0 | 0.00% |
| control | 11,004 | 0 | 0.00% |
| defense | 11,004 | 1 | 0.01% |

## 3 players: payoff of one seat of the row strategy at a table whose other 2 seats play the column strategy

2,016 games, 1 stalled (0.05%). The diagonal is the all-same table: (1 − stall rate) / 3. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense |
|---|---|---|---|---|---|---|
| **merchant** | 0.333 [0.333, 0.333] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.333 [0.333, 0.333] | 0.167 [0.083, 0.250] | 0.361 [0.194, 0.528] | 0.139 [0.028, 0.278] | 0.139 [0.028, 0.250] |
| **run2** | 1.000 [1.000, 1.000] | 0.306 [0.167, 0.444] | 0.333 [0.333, 0.333] | 0.306 [0.194, 0.417] | 0.208 [0.097, 0.333] | 0.306 [0.167, 0.472] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.472 [0.333, 0.611] | 0.361 [0.222, 0.500] | 0.333 [0.333, 0.333] | 0.250 [0.139, 0.361] | 0.264 [0.153, 0.389] |
| **control** | 1.000 [1.000, 1.000] | 0.389 [0.249, 0.556] | 0.319 [0.194, 0.458] | 0.403 [0.264, 0.556] | 0.333 [0.333, 0.333] | 0.333 [0.222, 0.444] |
| **defense** | 1.000 [1.000, 1.000] | 0.417 [0.306, 0.528] | 0.347 [0.236, 0.472] | 0.306 [0.222, 0.389] | 0.222 [0.111, 0.333] | 0.324 [0.306, 0.333] |

Best single deviation within the population against each field of 3 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.667 | [+0.667, +0.667] |
| run1 | exploit_run2 | +0.139 | [+0.000, +0.278] |
| run2 | exploit_run2 | +0.028 | [-0.111, +0.167] |
| exploit_run2 | control | +0.069 | [-0.069, +0.222] |
| control | exploit_run2 | -0.083 | [-0.194, +0.028] |
| defense | control | +0.009 | [-0.102, +0.120] |

## 4 players: payoff of one seat of the row strategy at a table whose other 3 seats play the column strategy

6,048 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 4. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense |
|---|---|---|---|---|---|---|
| **merchant** | 0.250 [0.250, 0.250] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.250 [0.250, 0.250] | 0.167 [0.083, 0.250] | 0.042 [0.000, 0.104] | 0.021 [0.000, 0.062] | 0.000 [0.000, 0.000] |
| **run2** | 1.000 [1.000, 1.000] | 0.438 [0.312, 0.562] | 0.250 [0.250, 0.250] | 0.146 [0.083, 0.208] | 0.219 [0.083, 0.354] | 0.198 [0.083, 0.323] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.604 [0.542, 0.667] | 0.396 [0.250, 0.552] | 0.250 [0.250, 0.250] | 0.260 [0.167, 0.375] | 0.333 [0.208, 0.480] |
| **control** | 1.000 [1.000, 1.000] | 0.500 [0.375, 0.625] | 0.292 [0.188, 0.396] | 0.229 [0.146, 0.312] | 0.250 [0.250, 0.250] | 0.323 [0.188, 0.458] |
| **defense** | 1.000 [1.000, 1.000] | 0.458 [0.312, 0.604] | 0.271 [0.208, 0.333] | 0.177 [0.083, 0.281] | 0.250 [0.146, 0.375] | 0.250 [0.250, 0.250] |

Best single deviation within the population against each field of 4 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.750 | [+0.750, +0.750] |
| run1 | exploit_run2 | +0.354 | [+0.292, +0.417] |
| run2 | exploit_run2 | +0.146 | [+0.000, +0.302] |
| exploit_run2 | control | -0.021 | [-0.104, +0.062] |
| control | exploit_run2 | +0.010 | [-0.083, +0.125] |
| defense | exploit_run2 | +0.083 | [-0.042, +0.230] |

## 5 players: payoff of one seat of the row strategy at a table whose other 4 seats play the column strategy

15,120 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 5. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense |
|---|---|---|---|---|---|---|
| **merchant** | 0.200 [0.200, 0.200] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.200 [0.200, 0.200] | 0.050 [0.000, 0.100] | 0.083 [0.017, 0.167] | 0.017 [0.000, 0.050] | 0.050 [0.000, 0.100] |
| **run2** | 1.000 [1.000, 1.000] | 0.383 [0.283, 0.483] | 0.200 [0.200, 0.200] | 0.200 [0.133, 0.267] | 0.208 [0.142, 0.275] | 0.167 [0.050, 0.317] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.433 [0.300, 0.567] | 0.317 [0.200, 0.433] | 0.200 [0.200, 0.200] | 0.192 [0.100, 0.283] | 0.200 [0.100, 0.317] |
| **control** | 1.000 [1.000, 1.000] | 0.392 [0.308, 0.475] | 0.283 [0.167, 0.417] | 0.283 [0.167, 0.400] | 0.200 [0.200, 0.200] | 0.133 [0.050, 0.217] |
| **defense** | 1.000 [1.000, 1.000] | 0.383 [0.283, 0.483] | 0.217 [0.133, 0.317] | 0.417 [0.317, 0.517] | 0.258 [0.142, 0.383] | 0.200 [0.200, 0.200] |

Best single deviation within the population against each field of 5 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.800 | [+0.800, +0.800] |
| run1 | exploit_run2 | +0.233 | [+0.100, +0.367] |
| run2 | exploit_run2 | +0.117 | [+0.000, +0.233] |
| exploit_run2 | defense | +0.217 | [+0.117, +0.317] |
| control | defense | +0.058 | [-0.058, +0.183] |
| defense | exploit_run2 | -0.000 | [-0.100, +0.117] |

## Per strategy over all the seats it occupies in the design

Compositions are weighted by their games. `fair share` is n × utility, averaged (1.00 = an equal player at any table size). A tie is a shared win. Score margin is own total minus the mean of the opponents' totals and length is in engine actions; both over completed games.

| strategy | seat-games | fair share | win rate | tie rate | score margin | actions per game |
|---|---|---|---|---|---|---|
| merchant | 17,640 | 0.034 | 0.008 | 0.000 | -81.0 | 829 |
| run1 | 17,640 | 0.696 | 0.156 | 0.003 | +7.7 | 898 |
| run2 | 17,640 | 1.260 | 0.277 | 0.006 | +17.1 | 906 |
| exploit_run2 | 17,640 | 1.146 | 0.256 | 0.006 | +16.9 | 931 |
| control | 17,640 | 1.435 | 0.316 | 0.006 | +19.9 | 925 |
| defense | 17,640 | 1.428 | 0.314 | 0.006 | +19.4 | 925 |

## By turn position

Fair share (n × utility) of the player in each turn position over all games, with 95% seed-clustered CI; position 0 moves first. The engine draws the first player at random for every game and every design seats each strategy in every position equally often, so a position effect is averaged out of the payoffs above, not part of them. 1.00 means no position effect; a position that is above 1.00 is above an equal player's share.

| players | position 0 | position 1 | position 2 | position 3 | position 4 | actions per game (mean, p99) |
|---|---|---|---|---|---|---|
| 3 | 1.211 [1.057, 1.352] | 0.842 [0.728, 0.954] | 0.946 [0.821, 1.120] | — | — | 422, 1118 |
| 4 | 1.129 [0.978, 1.296] | 1.026 [0.908, 1.140] | 0.975 [0.836, 1.116] | 0.871 [0.723, 1.030] | — | 733, 973 |
| 5 | 1.035 [0.957, 1.110] | 1.031 [0.941, 1.124] | 1.030 [0.933, 1.135] | 0.936 [0.865, 1.018] | 0.969 [0.902, 1.036] | 995, 1221 |

## Secondary: ladder-style ELO of these games

For continuity with `training/ladder.json` only. It ignores stalled games and weights compositions uniformly, so it is not comparable with the ladder's mixed-pool sampling, and it says nothing about exploitability.

| strategy | ELO |
|---|---|
| control | 1713.5 |
| defense | 1710.0 |
| exploit_run2 | 1690.1 |
| run2 | 1673.3 |
| run1 | 1530.1 |
| merchant | 0.0 |

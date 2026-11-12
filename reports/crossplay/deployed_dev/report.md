# Cross-play of the baseline population (deployed modes, dev seeds)

Population: merchant, run1, run2, exploit_run2, control, defense, magnate.  Horizon 2500 actions, game length standard, seed block `0x400…`, 24 seeds per composition and table size, every composition played in all its seat rotations. Commit `2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty`.

**81,648 games: 81,642 completed, 6 stalled (0.01%).** Utility: winner share: 1/|W| per winner (engine tie-break), 0 otherwise; stalled games pay 0 to every seat.

Policy modes (never pooled): merchant=scripted, run1=sample, run2=sample, exploit_run2=sample, control=sample, defense=sample, magnate=guarded.

## Accounting per strategy

| strategy | games seated in | stalled | stall rate |
|---|---|---|---|
| merchant | 35,280 | 0 | 0.00% |
| run1 | 35,280 | 5 | 0.01% |
| run2 | 35,280 | 3 | 0.01% |
| exploit_run2 | 35,280 | 0 | 0.00% |
| control | 35,280 | 1 | 0.00% |
| defense | 35,280 | 2 | 0.01% |
| magnate | 35,280 | 0 | 0.00% |

## 3 players: payoff of one seat of the row strategy at a table whose other 2 seats play the column strategy

6,048 games, 6 stalled (0.10%). The diagonal is the all-same table: (1 − stall rate) / 3. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense | magnate |
|---|---|---|---|---|---|---|---|
| **merchant** | 0.333 [0.333, 0.333] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.333 [0.333, 0.333] | 0.264 [0.152, 0.389] | 0.167 [0.083, 0.250] | 0.139 [0.069, 0.222] | 0.167 [0.097, 0.250] | 0.111 [0.049, 0.181] |
| **run2** | 1.000 [1.000, 1.000] | 0.458 [0.347, 0.556] | 0.333 [0.333, 0.333] | 0.243 [0.160, 0.333] | 0.264 [0.167, 0.361] | 0.361 [0.264, 0.458] | 0.257 [0.174, 0.347] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.500 [0.375, 0.625] | 0.486 [0.389, 0.583] | 0.333 [0.333, 0.333] | 0.285 [0.181, 0.389] | 0.292 [0.181, 0.403] | 0.292 [0.208, 0.389] |
| **control** | 1.000 [1.000, 1.000] | 0.347 [0.250, 0.458] | 0.333 [0.222, 0.431] | 0.312 [0.201, 0.431] | 0.329 [0.319, 0.333] | 0.278 [0.181, 0.375] | 0.319 [0.236, 0.417] |
| **defense** | 1.000 [1.000, 1.000] | 0.431 [0.319, 0.542] | 0.424 [0.306, 0.535] | 0.396 [0.292, 0.500] | 0.292 [0.181, 0.403] | 0.333 [0.333, 0.333] | 0.194 [0.111, 0.278] |
| **magnate** | 1.000 [1.000, 1.000] | 0.562 [0.465, 0.660] | 0.451 [0.333, 0.570] | 0.444 [0.347, 0.542] | 0.479 [0.375, 0.583] | 0.340 [0.264, 0.424] | 0.333 [0.333, 0.333] |

Best single deviation within the population against each field of 3 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.667 | [+0.667, +0.667] |
| run1 | magnate | +0.229 | [+0.132, +0.326] |
| run2 | exploit_run2 | +0.153 | [+0.056, +0.250] |
| exploit_run2 | magnate | +0.111 | [+0.014, +0.208] |
| control | magnate | +0.150 | [+0.051, +0.255] |
| defense | run2 | +0.028 | [-0.069, +0.125] |
| magnate | control | -0.014 | [-0.097, +0.083] |

## 4 players: payoff of one seat of the row strategy at a table whose other 3 seats play the column strategy

20,160 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 4. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense | magnate |
|---|---|---|---|---|---|---|---|
| **merchant** | 0.250 [0.250, 0.250] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.250 [0.250, 0.250] | 0.115 [0.062, 0.167] | 0.000 [0.000, 0.000] | 0.073 [0.031, 0.125] | 0.073 [0.031, 0.115] | 0.089 [0.042, 0.141] |
| **run2** | 1.000 [1.000, 1.000] | 0.516 [0.417, 0.620] | 0.250 [0.250, 0.250] | 0.203 [0.135, 0.276] | 0.177 [0.115, 0.240] | 0.229 [0.146, 0.323] | 0.094 [0.042, 0.156] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.490 [0.406, 0.578] | 0.297 [0.203, 0.385] | 0.250 [0.250, 0.250] | 0.271 [0.182, 0.359] | 0.250 [0.167, 0.333] | 0.219 [0.146, 0.292] |
| **control** | 1.000 [1.000, 1.000] | 0.547 [0.443, 0.656] | 0.349 [0.266, 0.438] | 0.245 [0.177, 0.312] | 0.250 [0.250, 0.250] | 0.234 [0.161, 0.318] | 0.219 [0.166, 0.281] |
| **defense** | 1.000 [1.000, 1.000] | 0.490 [0.385, 0.604] | 0.370 [0.292, 0.453] | 0.229 [0.156, 0.302] | 0.229 [0.146, 0.312] | 0.250 [0.250, 0.250] | 0.135 [0.073, 0.208] |
| **magnate** | 1.000 [1.000, 1.000] | 0.505 [0.417, 0.594] | 0.521 [0.438, 0.604] | 0.354 [0.271, 0.443] | 0.224 [0.161, 0.281] | 0.245 [0.182, 0.312] | 0.250 [0.250, 0.250] |

Best single deviation within the population against each field of 4 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.750 | [+0.750, +0.750] |
| run1 | control | +0.297 | [+0.193, +0.406] |
| run2 | magnate | +0.271 | [+0.188, +0.354] |
| exploit_run2 | magnate | +0.104 | [+0.021, +0.193] |
| control | exploit_run2 | +0.021 | [-0.068, +0.109] |
| defense | exploit_run2 | +0.000 | [-0.083, +0.083] |
| magnate | exploit_run2 | -0.031 | [-0.104, +0.042] |

## 5 players: payoff of one seat of the row strategy at a table whose other 4 seats play the column strategy

55,440 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 5. 95% seed-clustered CI in brackets.

| row \ column | merchant | run1 | run2 | exploit_run2 | control | defense | magnate |
|---|---|---|---|---|---|---|---|
| **merchant** | 0.200 [0.200, 0.200] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| **run1** | 1.000 [1.000, 1.000] | 0.200 [0.200, 0.200] | 0.033 [0.000, 0.075] | 0.100 [0.050, 0.150] | 0.042 [0.008, 0.083] | 0.075 [0.041, 0.117] | 0.025 [0.000, 0.050] |
| **run2** | 1.000 [1.000, 1.000] | 0.304 [0.242, 0.367] | 0.200 [0.200, 0.200] | 0.267 [0.192, 0.350] | 0.163 [0.092, 0.246] | 0.196 [0.129, 0.263] | 0.100 [0.042, 0.158] |
| **exploit_run2** | 1.000 [1.000, 1.000] | 0.386 [0.286, 0.486] | 0.308 [0.242, 0.375] | 0.200 [0.200, 0.200] | 0.217 [0.167, 0.275] | 0.158 [0.100, 0.217] | 0.117 [0.067, 0.167] |
| **control** | 1.000 [1.000, 1.000] | 0.308 [0.242, 0.371] | 0.263 [0.204, 0.321] | 0.362 [0.263, 0.467] | 0.200 [0.200, 0.200] | 0.250 [0.175, 0.317] | 0.167 [0.117, 0.217] |
| **defense** | 1.000 [1.000, 1.000] | 0.404 [0.325, 0.479] | 0.246 [0.167, 0.325] | 0.279 [0.212, 0.358] | 0.212 [0.158, 0.271] | 0.200 [0.200, 0.200] | 0.175 [0.117, 0.242] |
| **magnate** | 1.000 [1.000, 1.000] | 0.417 [0.321, 0.508] | 0.350 [0.275, 0.417] | 0.417 [0.325, 0.508] | 0.212 [0.133, 0.304] | 0.233 [0.158, 0.308] | 0.200 [0.200, 0.200] |

Best single deviation within the population against each field of 5 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| merchant | run1 | +0.800 | [+0.800, +0.800] |
| run1 | magnate | +0.217 | [+0.121, +0.308] |
| run2 | magnate | +0.150 | [+0.075, +0.217] |
| exploit_run2 | magnate | +0.217 | [+0.125, +0.308] |
| control | exploit_run2 | +0.017 | [-0.033, +0.075] |
| defense | control | +0.050 | [-0.025, +0.117] |
| magnate | defense | -0.025 | [-0.083, +0.042] |

## Per strategy over all the seats it occupies in the design

Compositions are weighted by their games. `fair share` is n × utility, averaged (1.00 = an equal player at any table size). A tie is a shared win. Score margin is own total minus the mean of the opponents' totals and length is in engine actions; both over completed games.

| strategy | seat-games | fair share | win rate | tie rate | score margin | actions per game |
|---|---|---|---|---|---|---|
| merchant | 53,712 | 0.022 | 0.005 | 0.000 | -87.2 | 849 |
| run1 | 53,712 | 0.618 | 0.137 | 0.003 | +4.5 | 913 |
| run2 | 53,712 | 1.134 | 0.248 | 0.005 | +14.2 | 922 |
| exploit_run2 | 53,712 | 1.048 | 0.233 | 0.006 | +14.4 | 944 |
| control | 53,712 | 1.336 | 0.292 | 0.006 | +17.1 | 941 |
| defense | 53,712 | 1.296 | 0.284 | 0.006 | +16.8 | 941 |
| magnate | 53,712 | 1.546 | 0.337 | 0.006 | +20.2 | 942 |

## By turn position

Fair share (n × utility) of the player in each turn position over all games, with 95% seed-clustered CI; position 0 moves first. The engine draws the first player at random for every game and every design seats each strategy in every position equally often, so a position effect is averaged out of the payoffs above, not part of them. 1.00 means no position effect; a position that is above 1.00 is above an equal player's share.

| players | position 0 | position 1 | position 2 | position 3 | position 4 | actions per game (mean, p99) |
|---|---|---|---|---|---|---|
| 3 | 1.182 [1.053, 1.302] | 0.827 [0.740, 0.912] | 0.988 [0.864, 1.126] | — | — | 421, 969 |
| 4 | 1.097 [0.994, 1.200] | 0.953 [0.842, 1.061] | 0.981 [0.879, 1.075] | 0.970 [0.853, 1.085] | — | 738, 996 |
| 5 | 1.049 [0.982, 1.121] | 0.995 [0.941, 1.046] | 1.025 [0.968, 1.077] | 0.931 [0.865, 0.995] | 1.001 [0.952, 1.051] | 1008, 1227 |

## Secondary: ladder-style ELO of these games

For continuity with `training/ladder.json` only. It ignores stalled games and weights compositions uniformly, so it is not comparable with the ladder's mixed-pool sampling, and it says nothing about exploitability.

| strategy | ELO |
|---|---|
| magnate | 1867.4 |
| control | 1823.5 |
| defense | 1821.4 |
| exploit_run2 | 1803.9 |
| run2 | 1781.6 |
| run1 | 1641.1 |
| merchant | 0.0 |

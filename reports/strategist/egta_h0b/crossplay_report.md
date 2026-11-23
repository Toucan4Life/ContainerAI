# Cross-play of the three heuristics (standard tables)

Population: producer, showcase, shipper.  Horizon 2500 actions, game length standard, seed block `0x400…`, 400 seeds per composition and table size, every composition played in all its seat rotations. Commit `6e4ad64eee30e5f3a0b7c63f89a5e130785263d2+dirty-83b3e0474d08`.

**78,000 games: 77,804 completed, 196 stalled (0.25%).** Utility: winner share: 1/|W| per winner (engine tie-break), 0 otherwise; stalled games pay 0 to every seat.

Policy modes (never pooled): producer=scripted, showcase=scripted, shipper=scripted.

## Accounting per strategy

| strategy | games seated in | stalled | stall rate |
|---|---|---|---|
| producer | 53,200 | 54 | 0.10% |
| showcase | 53,200 | 145 | 0.27% |
| shipper | 53,200 | 172 | 0.32% |

## 3 players: payoff of one seat of the row strategy at a table whose other 2 seats play the column strategy

12,000 games, 196 stalled (1.63%). The diagonal is the all-same table: (1 − stall rate) / 3. 95% seed-clustered CI in brackets.

| row \ column | producer | showcase | shipper |
|---|---|---|---|
| **producer** | 0.331 [0.328, 0.333] | 0.100 [0.083, 0.118] | 0.378 [0.352, 0.404] |
| **showcase** | 0.285 [0.262, 0.307] | 0.333 [0.333, 0.333] | 0.235 [0.211, 0.258] |
| **shipper** | 0.188 [0.168, 0.209] | 0.190 [0.168, 0.212] | 0.324 [0.319, 0.329] |

Best single deviation within the population against each field of 3 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| producer | showcase | -0.045 | [-0.068, -0.023] |
| showcase | shipper | -0.143 | [-0.165, -0.121] |
| shipper | producer | +0.054 | [+0.028, +0.081] |

## 4 players: payoff of one seat of the row strategy at a table whose other 3 seats play the column strategy

24,000 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 4. 95% seed-clustered CI in brackets.

| row \ column | producer | showcase | shipper |
|---|---|---|---|
| **producer** | 0.250 [0.250, 0.250] | 0.162 [0.144, 0.181] | 0.148 [0.132, 0.164] |
| **showcase** | 0.200 [0.180, 0.219] | 0.250 [0.250, 0.250] | 0.034 [0.025, 0.043] |
| **shipper** | 0.114 [0.098, 0.130] | 0.223 [0.202, 0.244] | 0.250 [0.250, 0.250] |

Best single deviation within the population against each field of 4 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| producer | showcase | -0.050 | [-0.070, -0.031] |
| showcase | shipper | -0.027 | [-0.048, -0.006] |
| shipper | producer | -0.102 | [-0.118, -0.086] |

## 5 players: payoff of one seat of the row strategy at a table whose other 4 seats play the column strategy

42,000 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 5. 95% seed-clustered CI in brackets.

| row \ column | producer | showcase | shipper |
|---|---|---|---|
| **producer** | 0.200 [0.200, 0.200] | 0.234 [0.216, 0.253] | 0.030 [0.022, 0.038] |
| **showcase** | 0.200 [0.183, 0.216] | 0.200 [0.200, 0.200] | 0.015 [0.010, 0.020] |
| **shipper** | 0.128 [0.114, 0.142] | 0.270 [0.250, 0.289] | 0.200 [0.200, 0.200] |

Best single deviation within the population against each field of 5 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| producer | showcase | -0.000 | [-0.017, +0.016] |
| showcase | shipper | +0.069 | [+0.050, +0.089] |
| shipper | producer | -0.170 | [-0.178, -0.162] |

## Per strategy over all the seats it occupies in the design

Compositions are weighted by their games. `fair share` is n × utility, averaged (1.00 = an equal player at any table size). A tie is a shared win. Score margin is own total minus the mean of the opponents' totals and length is in engine actions; both over completed games.

| strategy | seat-games | fair share | win rate | tie rate | score margin | actions per game |
|---|---|---|---|---|---|---|
| producer | 114,000 | 0.942 | 0.218 | 0.004 | -1.7 | 836 |
| showcase | 114,000 | 0.923 | 0.216 | 0.006 | +0.1 | 854 |
| shipper | 114,000 | 1.130 | 0.256 | 0.006 | +1.6 | 730 |

## By turn position

Fair share (n × utility) of the player in each turn position over all games, with 95% seed-clustered CI; position 0 moves first. The engine draws the first player at random for every game and every design seats each strategy in every position equally often, so a position effect is averaged out of the payoffs above, not part of them. 1.00 means no position effect; a position that is above 1.00 is above an equal player's share.

| players | position 0 | position 1 | position 2 | position 3 | position 4 | actions per game (mean, p99) |
|---|---|---|---|---|---|---|
| 3 | 0.939 [0.905, 0.974] | 1.065 [1.027, 1.103] | 0.947 [0.911, 0.982] | — | — | 506, 773 |
| 4 | 1.033 [1.001, 1.067] | 1.038 [1.004, 1.075] | 0.974 [0.940, 1.008] | 0.954 [0.919, 0.990] | — | 703, 979 |
| 5 | 1.040 [1.013, 1.065] | 1.019 [0.993, 1.046] | 0.982 [0.954, 1.007] | 0.983 [0.957, 1.011] | 0.977 [0.950, 1.003] | 905, 1231 |

# Cross-play of the three heuristics (standard tables)

Population: producer, showcase, shipper.  Horizon 2500 actions, game length standard, seed block `0x400…`, 400 seeds per composition and table size, every composition played in all its seat rotations. Commit `6e4ad64eee30e5f3a0b7c63f89a5e130785263d2+dirty-5d055891e353`.

**78,000 games: 77,989 completed, 11 stalled (0.01%).** Utility: winner share: 1/|W| per winner (engine tie-break), 0 otherwise; stalled games pay 0 to every seat.

Policy modes (never pooled): producer=scripted, showcase=scripted, shipper=scripted.

## Accounting per strategy

| strategy | games seated in | stalled | stall rate |
|---|---|---|---|
| producer | 53,200 | 7 | 0.01% |
| showcase | 53,200 | 5 | 0.01% |
| shipper | 53,200 | 11 | 0.02% |

## 3 players: payoff of one seat of the row strategy at a table whose other 2 seats play the column strategy

12,000 games, 11 stalled (0.09%). The diagonal is the all-same table: (1 − stall rate) / 3. 95% seed-clustered CI in brackets.

| row \ column | producer | showcase | shipper |
|---|---|---|---|
| **producer** | 0.333 [0.333, 0.333] | 0.124 [0.105, 0.143] | 0.301 [0.275, 0.326] |
| **showcase** | 0.491 [0.466, 0.517] | 0.333 [0.333, 0.333] | 0.095 [0.079, 0.110] |
| **shipper** | 0.148 [0.129, 0.168] | 0.389 [0.365, 0.414] | 0.333 [0.333, 0.333] |

Best single deviation within the population against each field of 3 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| producer | showcase | +0.158 | [+0.133, +0.183] |
| showcase | shipper | +0.055 | [+0.031, +0.081] |
| shipper | producer | -0.032 | [-0.058, -0.007] |

## 4 players: payoff of one seat of the row strategy at a table whose other 3 seats play the column strategy

24,000 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 4. 95% seed-clustered CI in brackets.

| row \ column | producer | showcase | shipper |
|---|---|---|---|
| **producer** | 0.250 [0.250, 0.250] | 0.163 [0.144, 0.181] | 0.249 [0.229, 0.272] |
| **showcase** | 0.338 [0.316, 0.359] | 0.250 [0.250, 0.250] | 0.036 [0.027, 0.046] |
| **shipper** | 0.117 [0.102, 0.132] | 0.366 [0.344, 0.389] | 0.250 [0.250, 0.250] |

Best single deviation within the population against each field of 4 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| producer | showcase | +0.088 | [+0.066, +0.109] |
| showcase | shipper | +0.116 | [+0.094, +0.139] |
| shipper | producer | -0.001 | [-0.021, +0.022] |

## 5 players: payoff of one seat of the row strategy at a table whose other 4 seats play the column strategy

42,000 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 5. 95% seed-clustered CI in brackets.

| row \ column | producer | showcase | shipper |
|---|---|---|---|
| **producer** | 0.200 [0.200, 0.200] | 0.228 [0.210, 0.247] | 0.186 [0.170, 0.202] |
| **showcase** | 0.276 [0.258, 0.295] | 0.200 [0.200, 0.200] | 0.014 [0.009, 0.019] |
| **shipper** | 0.154 [0.138, 0.169] | 0.388 [0.367, 0.410] | 0.200 [0.200, 0.200] |

Best single deviation within the population against each field of 5 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| producer | showcase | +0.076 | [+0.058, +0.095] |
| showcase | shipper | +0.188 | [+0.167, +0.210] |
| shipper | producer | -0.014 | [-0.030, +0.002] |

## Per strategy over all the seats it occupies in the design

Compositions are weighted by their games. `fair share` is n × utility, averaged (1.00 = an equal player at any table size). A tie is a shared win. Score margin is own total minus the mean of the opponents' totals and length is in engine actions; both over completed games.

| strategy | seat-games | fair share | win rate | tie rate | score margin | actions per game |
|---|---|---|---|---|---|---|
| producer | 114,000 | 0.981 | 0.225 | 0.005 | -0.5 | 819 |
| showcase | 114,000 | 0.898 | 0.210 | 0.006 | -1.6 | 843 |
| shipper | 114,000 | 1.120 | 0.256 | 0.004 | +2.1 | 816 |

## By turn position

Fair share (n × utility) of the player in each turn position over all games, with 95% seed-clustered CI; position 0 moves first. The engine draws the first player at random for every game and every design seats each strategy in every position equally often, so a position effect is averaged out of the payoffs above, not part of them. 1.00 means no position effect; a position that is above 1.00 is above an equal player's share.

| players | position 0 | position 1 | position 2 | position 3 | position 4 | actions per game (mean, p99) |
|---|---|---|---|---|---|---|
| 3 | 0.986 [0.946, 1.025] | 0.898 [0.861, 0.935] | 1.113 [1.071, 1.156] | — | — | 496, 768 |
| 4 | 1.115 [1.073, 1.154] | 0.852 [0.816, 0.889] | 0.985 [0.949, 1.021] | 1.049 [1.006, 1.090] | — | 724, 928 |
| 5 | 0.952 [0.926, 0.979] | 0.990 [0.962, 1.020] | 0.961 [0.932, 0.989] | 0.980 [0.951, 1.010] | 1.116 [1.080, 1.153] | 929, 1225 |

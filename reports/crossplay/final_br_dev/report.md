# Cross-play report

Population: run2, magnate, control, defense, exploit_run2, merchant, psro02, psro03, psro05, psro08.  Horizon 2500 actions, game length standard, seed block `0x400…`, 24 seeds per composition and table size, every composition played in all its seat rotations. Commit `2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-d7c1d7cd50a9`.

**110,880 games: 110,876 completed, 4 stalled (0.00%).** Utility: winner share: 1/|W| per winner (engine tie-break), 0 otherwise; stalled games pay 0 to every seat.

Policy modes (never pooled): run2=sample, magnate=guarded, control=sample, defense=sample, exploit_run2=sample, merchant=scripted, psro02=guarded, psro03=guarded, psro05=guarded, psro08=guarded.

## Accounting per strategy

| strategy | games seated in | stalled | stall rate |
|---|---|---|---|
| run2 | 32,400 | 0 | 0.00% |
| magnate | 32,400 | 0 | 0.00% |
| control | 32,400 | 0 | 0.00% |
| defense | 32,400 | 0 | 0.00% |
| exploit_run2 | 32,400 | 0 | 0.00% |
| merchant | 32,400 | 0 | 0.00% |
| psro02 | 32,400 | 0 | 0.00% |
| psro03 | 32,400 | 0 | 0.00% |
| psro05 | 32,400 | 0 | 0.00% |
| psro08 | 110,880 | 4 | 0.00% |

## 3 players: payoff of one seat of the row strategy at a table whose other 2 seats play the column strategy

3,960 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 3. 95% seed-clustered CI in brackets.

| row \ column | run2 | magnate | control | defense | exploit_run2 | merchant | psro02 | psro03 | psro05 | psro08 |
|---|---|---|---|---|---|---|---|---|---|---|
| **run2** | — | — | — | — | — | — | — | — | — | 0.167 [0.083, 0.264] |
| **magnate** | — | — | — | — | — | — | — | — | — | 0.361 [0.236, 0.500] |
| **control** | — | — | — | — | — | — | — | — | — | 0.222 [0.139, 0.306] |
| **defense** | — | — | — | — | — | — | — | — | — | 0.139 [0.069, 0.222] |
| **exploit_run2** | — | — | — | — | — | — | — | — | — | 0.160 [0.076, 0.250] |
| **merchant** | — | — | — | — | — | — | — | — | — | 0.000 [0.000, 0.000] |
| **psro02** | — | — | — | — | — | — | — | — | — | 0.306 [0.208, 0.417] |
| **psro03** | — | — | — | — | — | — | — | — | — | 0.236 [0.167, 0.306] |
| **psro05** | — | — | — | — | — | — | — | — | — | 0.271 [0.174, 0.382] |
| **psro08** | 0.479 [0.375, 0.597] | 0.424 [0.306, 0.542] | 0.431 [0.319, 0.542] | 0.451 [0.333, 0.576] | 0.486 [0.361, 0.611] | 1.000 [1.000, 1.000] | 0.257 [0.174, 0.347] | 0.396 [0.292, 0.500] | 0.424 [0.326, 0.521] | 0.333 [0.333, 0.333] |

Best single deviation within the population against each field of 3 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| psro08 | magnate | +0.028 | [-0.097, +0.167] |

## 4 players: payoff of one seat of the row strategy at a table whose other 3 seats play the column strategy

21,120 games, 4 stalled (0.02%). The diagonal is the all-same table: (1 − stall rate) / 4. 95% seed-clustered CI in brackets.

| row \ column | run2 | magnate | control | defense | exploit_run2 | merchant | psro02 | psro03 | psro05 | psro08 |
|---|---|---|---|---|---|---|---|---|---|---|
| **run2** | — | — | — | — | — | — | — | — | — | 0.042 [0.010, 0.083] |
| **magnate** | — | — | — | — | — | — | — | — | — | 0.115 [0.052, 0.188] |
| **control** | — | — | — | — | — | — | — | — | — | 0.047 [0.005, 0.099] |
| **defense** | — | — | — | — | — | — | — | — | — | 0.097 [0.045, 0.155] |
| **exploit_run2** | — | — | — | — | — | — | — | — | — | 0.104 [0.042, 0.167] |
| **merchant** | — | — | — | — | — | — | — | — | — | 0.000 [0.000, 0.000] |
| **psro02** | — | — | — | — | — | — | — | — | — | 0.234 [0.167, 0.302] |
| **psro03** | — | — | — | — | — | — | — | — | — | 0.266 [0.193, 0.339] |
| **psro05** | — | — | — | — | — | — | — | — | — | 0.219 [0.141, 0.307] |
| **psro08** | 0.615 [0.510, 0.708] | 0.375 [0.292, 0.458] | 0.391 [0.292, 0.490] | 0.458 [0.385, 0.531] | 0.443 [0.370, 0.510] | 1.000 [1.000, 1.000] | 0.333 [0.271, 0.406] | 0.255 [0.177, 0.344] | 0.245 [0.172, 0.323] | 0.240 [0.219, 0.250] |

Best single deviation within the population against each field of 4 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| psro08 | psro03 | +0.026 | [-0.052, +0.099] |

## 5 players: payoff of one seat of the row strategy at a table whose other 4 seats play the column strategy

85,800 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 5. 95% seed-clustered CI in brackets.

| row \ column | run2 | magnate | control | defense | exploit_run2 | merchant | psro02 | psro03 | psro05 | psro08 |
|---|---|---|---|---|---|---|---|---|---|---|
| **run2** | — | — | — | — | — | — | — | — | — | 0.058 [0.025, 0.092] |
| **magnate** | — | — | — | — | — | — | — | — | — | 0.092 [0.050, 0.142] |
| **control** | — | — | — | — | — | — | — | — | — | 0.125 [0.067, 0.192] |
| **defense** | — | — | — | — | — | — | — | — | — | 0.104 [0.058, 0.158] |
| **exploit_run2** | — | — | — | — | — | — | — | — | — | 0.075 [0.025, 0.133] |
| **merchant** | — | — | — | — | — | — | — | — | — | 0.000 [0.000, 0.000] |
| **psro02** | — | — | — | — | — | — | — | — | — | 0.179 [0.113, 0.250] |
| **psro03** | — | — | — | — | — | — | — | — | — | 0.167 [0.100, 0.242] |
| **psro05** | — | — | — | — | — | — | — | — | — | 0.208 [0.150, 0.267] |
| **psro08** | 0.300 [0.225, 0.379] | 0.283 [0.208, 0.358] | 0.304 [0.221, 0.388] | 0.408 [0.317, 0.500] | 0.537 [0.475, 0.592] | 1.000 [1.000, 1.000] | 0.242 [0.158, 0.333] | 0.217 [0.150, 0.300] | 0.254 [0.183, 0.329] | 0.200 [0.200, 0.200] |

Best single deviation within the population against each field of 5 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| psro08 | psro05 | +0.008 | [-0.050, +0.067] |

## Per strategy over all the seats it occupies in the design

Compositions are weighted by their games. `fair share` is n × utility, averaged (1.00 = an equal player at any table size). A tie is a shared win. Score margin is own total minus the mean of the opponents' totals and length is in engine actions; both over completed games.

| strategy | seat-games | fair share | win rate | tie rate | score margin | actions per game |
|---|---|---|---|---|---|---|
| run2 | 41,448 | 0.577 | 0.123 | 0.003 | -0.2 | 1045 |
| magnate | 41,448 | 0.923 | 0.196 | 0.005 | +7.2 | 1068 |
| control | 41,448 | 0.738 | 0.157 | 0.005 | +3.9 | 1066 |
| defense | 41,448 | 0.726 | 0.155 | 0.004 | +3.8 | 1064 |
| exploit_run2 | 41,448 | 0.571 | 0.123 | 0.004 | +2.9 | 1065 |
| merchant | 41,448 | 0.000 | 0.000 | 0.000 | -110.2 | 965 |
| psro02 | 41,448 | 1.218 | 0.258 | 0.006 | +10.1 | 1094 |
| psro03 | 41,448 | 1.233 | 0.262 | 0.007 | +11.4 | 1100 |
| psro05 | 41,448 | 1.246 | 0.265 | 0.007 | +12.0 | 1104 |
| psro08 | 152,328 | 1.481 | 0.318 | 0.007 | +16.1 | 1066 |

## By turn position

Fair share (n × utility) of the player in each turn position over all games, with 95% seed-clustered CI; position 0 moves first. The engine draws the first player at random for every game and every design seats each strategy in every position equally often, so a position effect is averaged out of the payoffs above, not part of them. 1.00 means no position effect; a position that is above 1.00 is above an equal player's share.

| players | position 0 | position 1 | position 2 | position 3 | position 4 | actions per game (mean, p99) |
|---|---|---|---|---|---|---|
| 3 | 1.201 [1.038, 1.348] | 0.795 [0.689, 0.906] | 1.004 [0.864, 1.164] | — | — | 479, 1044 |
| 4 | 1.124 [0.998, 1.247] | 0.973 [0.841, 1.101] | 0.945 [0.836, 1.051] | 0.958 [0.812, 1.102] | — | 834, 1210 |
| 5 | 1.162 [1.082, 1.252] | 1.008 [0.922, 1.098] | 0.980 [0.887, 1.072] | 0.891 [0.804, 0.979] | 0.960 [0.886, 1.033] | 1126, 1389 |

## Secondary: ladder-style ELO of these games

For continuity with `training/ladder.json` only. It ignores stalled games and weights compositions uniformly, so it is not comparable with the ladder's mixed-pool sampling, and it says nothing about exploitability.

| strategy | ELO |
|---|---|
| psro05 | 169.8 |
| psro08 | 167.9 |
| psro03 | 160.2 |
| psro02 | 138.4 |
| magnate | 99.7 |
| control | 55.8 |
| defense | 53.9 |
| exploit_run2 | 50.8 |
| run2 | 0.0 |
| merchant | -1785.3 |

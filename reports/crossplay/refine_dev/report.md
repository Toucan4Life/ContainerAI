# Cross-play refinement: 48 further dev seeds for the strategies that can enter the support

Population: psro02, psro03, psro05, magnate, control.  Horizon 2500 actions, game length standard, seed block `0x400…`, 48 seeds per composition and table size, every composition played in all its seat rotations. Commit `2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-328f134f5302`.

**48,720 games: 48,719 completed, 1 stalled (0.00%).** Utility: winner share: 1/|W| per winner (engine tie-break), 0 otherwise; stalled games pay 0 to every seat.

Policy modes (never pooled): psro02=guarded, psro03=guarded, psro05=guarded, magnate=guarded, control=sample.

## Accounting per strategy

| strategy | games seated in | stalled | stall rate |
|---|---|---|---|
| psro02 | 25,680 | 1 | 0.00% |
| psro03 | 25,680 | 0 | 0.00% |
| psro05 | 25,680 | 1 | 0.00% |
| magnate | 25,680 | 0 | 0.00% |
| control | 25,680 | 0 | 0.00% |

## 3 players: payoff of one seat of the row strategy at a table whose other 2 seats play the column strategy

5,040 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 3. 95% seed-clustered CI in brackets.

| row \ column | psro02 | psro03 | psro05 | magnate | control |
|---|---|---|---|---|---|
| **psro02** | 0.333 [0.333, 0.333] | 0.330 [0.260, 0.403] | 0.278 [0.208, 0.351] | 0.451 [0.375, 0.524] | 0.438 [0.354, 0.521] |
| **psro03** | 0.354 [0.285, 0.431] | 0.333 [0.333, 0.333] | 0.288 [0.212, 0.365] | 0.420 [0.351, 0.497] | 0.378 [0.312, 0.448] |
| **psro05** | 0.389 [0.312, 0.469] | 0.410 [0.333, 0.486] | 0.333 [0.333, 0.333] | 0.424 [0.347, 0.500] | 0.479 [0.399, 0.559] |
| **magnate** | 0.236 [0.174, 0.306] | 0.194 [0.135, 0.253] | 0.253 [0.191, 0.316] | 0.333 [0.333, 0.333] | 0.427 [0.340, 0.517] |
| **control** | 0.219 [0.160, 0.285] | 0.184 [0.125, 0.247] | 0.229 [0.170, 0.288] | 0.250 [0.188, 0.312] | 0.333 [0.333, 0.333] |

Best single deviation within the population against each field of 3 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| psro02 | psro05 | +0.056 | [-0.021, +0.135] |
| psro03 | psro05 | +0.076 | [+0.000, +0.153] |
| psro05 | psro03 | -0.045 | [-0.122, +0.031] |
| magnate | psro02 | +0.118 | [+0.042, +0.191] |
| control | psro05 | +0.146 | [+0.066, +0.226] |

## 4 players: payoff of one seat of the row strategy at a table whose other 3 seats play the column strategy

13,440 games, 0 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 4. 95% seed-clustered CI in brackets.

| row \ column | psro02 | psro03 | psro05 | magnate | control |
|---|---|---|---|---|---|
| **psro02** | 0.250 [0.250, 0.250] | 0.263 [0.201, 0.328] | 0.208 [0.161, 0.255] | 0.359 [0.299, 0.414] | 0.359 [0.297, 0.425] |
| **psro03** | 0.331 [0.268, 0.398] | 0.250 [0.250, 0.250] | 0.216 [0.161, 0.273] | 0.391 [0.312, 0.469] | 0.385 [0.320, 0.451] |
| **psro05** | 0.349 [0.292, 0.409] | 0.247 [0.191, 0.304] | 0.250 [0.250, 0.250] | 0.359 [0.286, 0.432] | 0.349 [0.294, 0.401] |
| **magnate** | 0.125 [0.083, 0.167] | 0.104 [0.068, 0.141] | 0.091 [0.049, 0.138] | 0.250 [0.250, 0.250] | 0.289 [0.234, 0.346] |
| **control** | 0.115 [0.068, 0.161] | 0.078 [0.042, 0.120] | 0.102 [0.060, 0.148] | 0.143 [0.102, 0.185] | 0.250 [0.250, 0.250] |

Best single deviation within the population against each field of 4 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| psro02 | psro05 | +0.099 | [+0.042, +0.159] |
| psro03 | psro02 | +0.013 | [-0.049, +0.078] |
| psro05 | psro03 | -0.034 | [-0.089, +0.023] |
| magnate | psro03 | +0.141 | [+0.062, +0.219] |
| control | psro03 | +0.135 | [+0.070, +0.201] |

## 5 players: payoff of one seat of the row strategy at a table whose other 4 seats play the column strategy

30,240 games, 1 stalled (0.00%). The diagonal is the all-same table: (1 − stall rate) / 5. 95% seed-clustered CI in brackets.

| row \ column | psro02 | psro03 | psro05 | magnate | control |
|---|---|---|---|---|---|
| **psro02** | 0.200 [0.200, 0.200] | 0.212 [0.167, 0.263] | 0.175 [0.127, 0.223] | 0.306 [0.254, 0.362] | 0.352 [0.294, 0.415] |
| **psro03** | 0.231 [0.183, 0.281] | 0.200 [0.200, 0.200] | 0.175 [0.133, 0.221] | 0.290 [0.229, 0.350] | 0.317 [0.258, 0.375] |
| **psro05** | 0.215 [0.171, 0.263] | 0.179 [0.133, 0.225] | 0.200 [0.200, 0.200] | 0.279 [0.229, 0.325] | 0.290 [0.225, 0.354] |
| **magnate** | 0.106 [0.077, 0.138] | 0.152 [0.115, 0.188] | 0.140 [0.098, 0.185] | 0.200 [0.200, 0.200] | 0.256 [0.204, 0.312] |
| **control** | 0.100 [0.067, 0.138] | 0.094 [0.060, 0.133] | 0.050 [0.025, 0.075] | 0.156 [0.117, 0.196] | 0.200 [0.200, 0.200] |

Best single deviation within the population against each field of 5 (an optimistic lower bound on exploitability: the best strategy is picked on the same seeds, and the real best response is not in this population):

| field | best deviation | gain over staying | 95% CI |
|---|---|---|---|
| psro02 | psro03 | +0.031 | [-0.017, +0.081] |
| psro03 | psro02 | +0.012 | [-0.033, +0.062] |
| psro05 | psro02 | -0.025 | [-0.073, +0.023] |
| magnate | psro02 | +0.106 | [+0.054, +0.163] |
| control | psro02 | +0.152 | [+0.094, +0.215] |

## Per strategy over all the seats it occupies in the design

Compositions are weighted by their games. `fair share` is n × utility, averaged (1.00 = an equal player at any table size). A tie is a shared win. Score margin is own total minus the mean of the opponents' totals and length is in engine actions; both over completed games.

| strategy | seat-games | fair share | win rate | tie rate | score margin | actions per game |
|---|---|---|---|---|---|---|
| psro02 | 44,016 | 1.113 | 0.249 | 0.007 | +1.1 | 1045 |
| psro03 | 44,016 | 1.173 | 0.262 | 0.007 | +2.5 | 1056 |
| psro05 | 44,016 | 1.192 | 0.268 | 0.007 | +3.1 | 1063 |
| magnate | 44,016 | 0.829 | 0.187 | 0.006 | -2.1 | 1010 |
| control | 44,016 | 0.693 | 0.157 | 0.005 | -4.6 | 1010 |

## By turn position

Fair share (n × utility) of the player in each turn position over all games, with 95% seed-clustered CI; position 0 moves first. The engine draws the first player at random for every game and every design seats each strategy in every position equally often, so a position effect is averaged out of the payoffs above, not part of them. 1.00 means no position effect; a position that is above 1.00 is above an equal player's share.

| players | position 0 | position 1 | position 2 | position 3 | position 4 | actions per game (mean, p99) |
|---|---|---|---|---|---|---|
| 3 | 1.191 [1.074, 1.300] | 0.810 [0.714, 0.913] | 0.999 [0.873, 1.119] | — | — | 470, 846 |
| 4 | 1.084 [0.959, 1.208] | 1.154 [1.022, 1.286] | 0.852 [0.726, 0.982] | 0.910 [0.791, 1.027] | — | 836, 1093 |
| 5 | 1.343 [1.229, 1.460] | 1.116 [1.024, 1.207] | 0.784 [0.709, 0.863] | 0.915 [0.834, 0.999] | 0.842 [0.764, 0.921] | 1165, 1408 |

## Secondary: ladder-style ELO of these games

For continuity with `training/ladder.json` only. It ignores stalled games and weights compositions uniformly, so it is not comparable with the ladder's mixed-pool sampling, and it says nothing about exploitability.

| strategy | ELO |
|---|---|
| psro05 | 33.1 |
| psro03 | 23.2 |
| psro02 | 0.0 |
| magnate | -48.2 |
| control | -87.6 |

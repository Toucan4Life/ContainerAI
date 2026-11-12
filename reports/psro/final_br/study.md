# Final best response: deviation study against the refined solution

Profile sigma: 3 players: psro03 0.00, psro05 1.00; 4 players: psro03 0.00, psro05 1.00; 5 players: psro03 1.00, psro05 0.00. Every other seat draws its strategy iid from sigma. Seeds: 100 per player count from the **dev** block (offset 1700), 3 player counts, every seat of every table, standard length; 4,800 games, 1 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-c5f401aca11b.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.333 [+0.333, +0.333]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| psro03 | +0.332 [+0.287, +0.377] | -0.002 [-0.047, +0.043] | 0 |
| psro05 | +0.333 [+0.333, +0.333] | +0.000 [+0.000, +0.000] | 0 |
| final_br/sample * | +0.348 [+0.300, +0.397] | +0.015 [-0.033, +0.063] | 0 |
| final_br/guarded * | +0.402 [+0.340, +0.460] | +0.068 [+0.007, +0.127] | 1 |

Gain by turn position (candidates; position 0 moves first): final_br/sample: +0.035, -0.010, +0.020; final_br/guarded: +0.020, +0.080, +0.105

NashConv (sum of the seats' gains): final_br/sample: +0.045 [-0.100, +0.190]; final_br/guarded: +0.205 [+0.020, +0.380]

## 4 players

Value of sigma: +0.250 [+0.250, +0.250]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| psro03 | +0.249 [+0.212, +0.292] | -0.001 [-0.037, +0.043] | 0 |
| psro05 | +0.250 [+0.250, +0.250] | +0.000 [+0.000, +0.000] | 0 |
| final_br/sample * | +0.226 [+0.186, +0.265] | -0.024 [-0.064, +0.015] | 0 |
| final_br/guarded * | +0.265 [+0.221, +0.310] | +0.015 [-0.029, +0.060] | 0 |

Gain by turn position (candidates; position 0 moves first): final_br/sample: +0.010, -0.095, +0.070, -0.080; final_br/guarded: -0.025, +0.020, +0.075, -0.010

NashConv (sum of the seats' gains): final_br/sample: -0.095 [-0.255, +0.060]; final_br/guarded: +0.060 [-0.115, +0.240]

## 5 players

Value of sigma: +0.200 [+0.200, +0.200]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| psro03 | +0.200 [+0.200, +0.200] | +0.000 [+0.000, +0.000] | 0 |
| psro05 | +0.207 [+0.173, +0.244] | +0.007 [-0.027, +0.044] | 0 |
| final_br/sample * | +0.239 [+0.207, +0.270] | +0.039 [+0.007, +0.070] | 0 |
| final_br/guarded * | +0.215 [+0.181, +0.250] | +0.015 [-0.019, +0.050] | 0 |

Gain by turn position (candidates; position 0 moves first): final_br/sample: +0.160, -0.110, +0.050, +0.075, +0.020; final_br/guarded: +0.070, -0.070, +0.035, +0.010, +0.030

NashConv (sum of the seats' gains): final_br/sample: +0.195 [+0.035, +0.350]; final_br/guarded: +0.075 [-0.095, +0.250]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| psro03 | -0.001 [-0.020, +0.019] | -0.003 [-0.072, +0.067] |
| psro05 | +0.002 [-0.009, +0.014] | +0.012 [-0.045, +0.072] |
| final_br/sample * | +0.010 [-0.013, +0.033] | +0.048 [-0.038, +0.135] |
| final_br/guarded * | +0.033 [+0.005, +0.060] | +0.113 [+0.008, +0.217] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of final_br/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.105 | 0.052 |
| 10 | ±0.074 | 0.036 |
| 25 | ±0.047 | 0.020 |
| 50 | ±0.033 | 0.012 |
| 100 | ±0.023 | 0.000 |

## How well the gain of final_br/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.123 | 0.060 |
| 10 | ±0.087 | 0.044 |
| 25 | ±0.055 | 0.025 |
| 50 | ±0.039 | 0.014 |
| 100 | ±0.028 | 0.000 |

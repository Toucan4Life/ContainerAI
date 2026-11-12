# PSRO iteration 7: deviation study of the new response

Profile sigma: 3 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.00, psro03 0.20, psro05 0.80; 4 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.00, psro03 0.64, psro05 0.35; 5 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.00, psro03 1.00, psro05 0.00. Every other seat draws its strategy iid from sigma. Seeds: 100 per player count from the **dev** block (offset 1600), 3 player counts, every seat of every table, standard length; 13,200 games, 1 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-a7d05e993a75.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.352 [+0.322, +0.381]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.177 [+0.137, +0.220] | -0.176 [-0.227, -0.122] | 0 |
| magnate | +0.232 [+0.185, +0.283] | -0.121 [-0.170, -0.069] | 0 |
| control | +0.217 [+0.177, +0.260] | -0.136 [-0.184, -0.084] | 0 |
| defense | +0.217 [+0.172, +0.263] | -0.136 [-0.190, -0.079] | 0 |
| exploit_run2 | +0.160 [+0.118, +0.205] | -0.192 [-0.241, -0.140] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.352 [-0.381, -0.322] | 1 |
| psro02 | +0.290 [+0.240, +0.337] | -0.062 [-0.120, -0.005] | 0 |
| psro03 | +0.342 [+0.293, +0.393] | -0.011 [-0.065, +0.052] | 0 |
| psro05 | +0.355 [+0.317, +0.393] | +0.003 [-0.013, +0.016] | 0 |
| psro07/sample * | +0.318 [+0.275, +0.363] | -0.034 [-0.091, +0.021] | 0 |
| psro07/guarded * | +0.337 [+0.287, +0.380] | -0.016 [-0.071, +0.038] | 0 |

Gain by turn position (candidates; position 0 moves first): psro07/sample: -0.078, -0.032, +0.008; psro07/guarded: +0.012, -0.087, +0.028

NashConv (sum of the seats' gains): psro07/sample: -0.102 [-0.272, +0.063]; psro07/guarded: -0.047 [-0.213, +0.113]

## 4 players

Value of sigma: +0.267 [+0.236, +0.300]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.076 [+0.051, +0.104] | -0.191 [-0.231, -0.151] | 0 |
| magnate | +0.098 [+0.072, +0.124] | -0.170 [-0.210, -0.131] | 0 |
| control | +0.079 [+0.052, +0.106] | -0.189 [-0.229, -0.148] | 0 |
| defense | +0.072 [+0.048, +0.102] | -0.195 [-0.234, -0.154] | 0 |
| exploit_run2 | +0.077 [+0.052, +0.107] | -0.190 [-0.227, -0.150] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.267 [-0.300, -0.236] | 0 |
| psro02 | +0.182 [+0.149, +0.219] | -0.085 [-0.126, -0.043] | 0 |
| psro03 | +0.259 [+0.216, +0.301] | -0.009 [-0.030, +0.011] | 0 |
| psro05 | +0.284 [+0.244, +0.326] | +0.016 [-0.020, +0.054] | 0 |
| psro07/sample * | +0.224 [+0.185, +0.264] | -0.044 [-0.096, +0.005] | 0 |
| psro07/guarded * | +0.268 [+0.230, +0.310] | +0.000 [-0.043, +0.049] | 0 |

Gain by turn position (candidates; position 0 moves first): psro07/sample: -0.049, -0.037, -0.015, -0.074; psro07/guarded: -0.009, -0.022, +0.010, +0.021

NashConv (sum of the seats' gains): psro07/sample: -0.175 [-0.383, +0.022]; psro07/guarded: +0.000 [-0.172, +0.197]

## 5 players

Value of sigma: +0.200 [+0.200, +0.200]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.074 [+0.052, +0.098] | -0.126 [-0.148, -0.102] | 0 |
| magnate | +0.128 [+0.103, +0.153] | -0.072 [-0.097, -0.047] | 0 |
| control | +0.079 [+0.056, +0.104] | -0.121 [-0.144, -0.096] | 0 |
| defense | +0.099 [+0.073, +0.125] | -0.101 [-0.127, -0.075] | 0 |
| exploit_run2 | +0.067 [+0.047, +0.089] | -0.133 [-0.153, -0.111] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.200 [-0.200, -0.200] | 0 |
| psro02 | +0.195 [+0.163, +0.228] | -0.005 [-0.037, +0.028] | 0 |
| psro03 | +0.200 [+0.200, +0.200] | +0.000 [+0.000, +0.000] | 0 |
| psro05 | +0.223 [+0.190, +0.256] | +0.023 [-0.010, +0.056] | 0 |
| psro07/sample * | +0.224 [+0.191, +0.257] | +0.024 [-0.009, +0.057] | 0 |
| psro07/guarded * | +0.238 [+0.202, +0.276] | +0.038 [+0.002, +0.076] | 0 |

Gain by turn position (candidates; position 0 moves first): psro07/sample: +0.040, +0.030, +0.085, -0.030, -0.005; psro07/guarded: +0.000, +0.085, +0.070, +0.080, -0.045

NashConv (sum of the seats' gains): psro07/sample: +0.120 [-0.045, +0.285]; psro07/guarded: +0.190 [+0.010, +0.380]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| run2 | -0.164 [-0.187, -0.140] | -0.641 [-0.720, -0.554] |
| magnate | -0.121 [-0.144, -0.097] | -0.467 [-0.550, -0.379] |
| control | -0.148 [-0.171, -0.125] | -0.589 [-0.670, -0.504] |
| defense | -0.144 [-0.167, -0.119] | -0.564 [-0.650, -0.474] |
| exploit_run2 | -0.172 [-0.194, -0.149] | -0.667 [-0.748, -0.586] |
| merchant | -0.273 [-0.287, -0.258] | -1.042 [-1.091, -0.991] |
| psro02 | -0.051 [-0.077, -0.025] | -0.184 [-0.278, -0.091] |
| psro03 | -0.006 [-0.026, +0.015] | -0.022 [-0.084, +0.043] |
| psro05 | +0.014 [-0.003, +0.031] | +0.063 [-0.011, +0.135] |
| psro07/sample * | -0.018 [-0.046, +0.011] | -0.052 [-0.158, +0.058] |
| psro07/guarded * | +0.007 [-0.021, +0.036] | +0.048 [-0.058, +0.156] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of psro07/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.126 | 0.065 |
| 10 | ±0.089 | 0.042 |
| 25 | ±0.056 | 0.025 |
| 50 | ±0.040 | 0.015 |
| 100 | ±0.028 | 0.000 |

## How well the gain of psro07/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.123 | 0.065 |
| 10 | ±0.087 | 0.044 |
| 25 | ±0.055 | 0.023 |
| 50 | ±0.039 | 0.014 |
| 100 | ±0.028 | 0.000 |

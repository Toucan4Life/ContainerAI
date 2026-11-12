# Validation 1: BR to run1, shaped objective, init run1 (12 min)

Profile sigma: run1 1.00. Every other seat draws its strategy iid from sigma. Seeds: 60 per player count from the **dev** block (offset 100), 3 player counts, every seat of every table, standard length; 6,480 games, 6 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.342 [+0.275, +0.411]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.342 [-0.411, -0.275] | 0 |
| run1 | +0.342 [+0.275, +0.411] | +0.000 [+0.000, +0.000] | 2 |
| run2 | +0.517 [+0.442, +0.592] | +0.175 [+0.064, +0.281] | 1 |
| exploit_run2 | +0.433 [+0.367, +0.494] | +0.092 [+0.006, +0.175] | 0 |
| control | +0.533 [+0.458, +0.606] | +0.192 [+0.078, +0.297] | 1 |
| defense | +0.519 [+0.450, +0.589] | +0.178 [+0.081, +0.275] | 0 |
| magnate | +0.500 [+0.422, +0.569] | +0.158 [+0.056, +0.253] | 0 |
| v1_run1_vs_run1_shaped/sample * | +0.450 [+0.383, +0.522] | +0.108 [+0.014, +0.200] | 2 |
| v1_run1_vs_run1_shaped/guarded * | +0.558 [+0.489, +0.628] | +0.217 [+0.125, +0.300] | 0 |

Gain by engine seat index (candidates; the first player is drawn at random, so the index is not turn order): v1_run1_vs_run1_shaped/sample: +0.108, +0.133, +0.083; v1_run1_vs_run1_shaped/guarded: +0.175, +0.192, +0.283

NashConv (sum of the seats' gains): v1_run1_vs_run1_shaped/sample: +0.325 [+0.042, +0.600]; v1_run1_vs_run1_shaped/guarded: +0.650 [+0.375, +0.900]

## 4 players

Value of sigma: +0.252 [+0.202, +0.306]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.252 [-0.306, -0.202] | 0 |
| run1 | +0.252 [+0.202, +0.306] | +0.000 [+0.000, +0.000] | 0 |
| run2 | +0.475 [+0.427, +0.523] | +0.223 [+0.150, +0.298] | 0 |
| exploit_run2 | +0.506 [+0.433, +0.581] | +0.254 [+0.152, +0.346] | 0 |
| control | +0.492 [+0.446, +0.533] | +0.240 [+0.162, +0.312] | 0 |
| defense | +0.471 [+0.412, +0.529] | +0.219 [+0.138, +0.298] | 0 |
| magnate | +0.525 [+0.473, +0.577] | +0.273 [+0.204, +0.342] | 0 |
| v1_run1_vs_run1_shaped/sample * | +0.385 [+0.327, +0.442] | +0.133 [+0.060, +0.208] | 0 |
| v1_run1_vs_run1_shaped/guarded * | +0.419 [+0.367, +0.473] | +0.167 [+0.094, +0.242] | 0 |

Gain by engine seat index (candidates; the first player is drawn at random, so the index is not turn order): v1_run1_vs_run1_shaped/sample: +0.275, +0.100, +0.100, +0.058; v1_run1_vs_run1_shaped/guarded: +0.275, +0.150, +0.083, +0.158

NashConv (sum of the seats' gains): v1_run1_vs_run1_shaped/sample: +0.533 [+0.241, +0.833]; v1_run1_vs_run1_shaped/guarded: +0.667 [+0.375, +0.967]

## 5 players

Value of sigma: +0.205 [+0.158, +0.255]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.205 [-0.255, -0.158] | 0 |
| run1 | +0.205 [+0.158, +0.255] | +0.000 [+0.000, +0.000] | 0 |
| run2 | +0.333 [+0.280, +0.387] | +0.128 [+0.057, +0.198] | 0 |
| exploit_run2 | +0.435 [+0.390, +0.483] | +0.230 [+0.168, +0.292] | 0 |
| control | +0.398 [+0.343, +0.448] | +0.193 [+0.123, +0.260] | 0 |
| defense | +0.365 [+0.320, +0.412] | +0.160 [+0.097, +0.228] | 0 |
| magnate | +0.452 [+0.403, +0.500] | +0.247 [+0.183, +0.308] | 0 |
| v1_run1_vs_run1_shaped/sample * | +0.297 [+0.248, +0.347] | +0.092 [+0.020, +0.165] | 0 |
| v1_run1_vs_run1_shaped/guarded * | +0.342 [+0.283, +0.405] | +0.137 [+0.062, +0.208] | 0 |

Gain by engine seat index (candidates; the first player is drawn at random, so the index is not turn order): v1_run1_vs_run1_shaped/sample: +0.067, +0.075, +0.200, -0.017, +0.133; v1_run1_vs_run1_shaped/guarded: +0.108, +0.217, +0.167, +0.008, +0.183

NashConv (sum of the seats' gains): v1_run1_vs_run1_shaped/sample: +0.458 [+0.100, +0.825]; v1_run1_vs_run1_shaped/guarded: +0.683 [+0.308, +1.042]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| merchant | -0.266 [-0.300, -0.236] | -1.019 [-1.150, -0.900] |
| run1 | +0.000 [+0.000, +0.000] | +0.000 [+0.000, +0.000] |
| run2 | +0.175 [+0.124, +0.225] | +0.686 [+0.489, +0.875] |
| exploit_run2 | +0.192 [+0.144, +0.237] | +0.814 [+0.630, +0.989] |
| control | +0.208 [+0.157, +0.257] | +0.833 [+0.644, +1.014] |
| defense | +0.186 [+0.137, +0.232] | +0.736 [+0.550, +0.914] |
| magnate | +0.226 [+0.180, +0.268] | +0.933 [+0.761, +1.097] |
| v1_run1_vs_run1_shaped/sample * | +0.111 [+0.064, +0.157] | +0.439 [+0.253, +0.620] |
| v1_run1_vs_run1_shaped/guarded * | +0.173 [+0.129, +0.217] | +0.667 [+0.489, +0.847] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of v1_run1_vs_run1_shaped/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 60 seeds (it understates near k = 60).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.162 | 0.080 |
| 10 | ±0.115 | 0.052 |
| 25 | ±0.073 | 0.029 |
| 50 | ±0.051 | 0.011 |
| 60 | ±0.047 | 0.000 |

## How well the gain of v1_run1_vs_run1_shaped/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 60 seeds (it understates near k = 60).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.162 | 0.078 |
| 10 | ±0.114 | 0.054 |
| 25 | ±0.072 | 0.029 |
| 50 | ±0.051 | 0.011 |
| 60 | ±0.047 | 0.000 |

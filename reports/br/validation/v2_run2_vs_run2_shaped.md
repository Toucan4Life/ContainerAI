# Oracle validation: v2_run2_vs_run2_shaped (init populations/baselines/run2.pt, profile run2, objective shaped)

Profile sigma: run2 1.00. Every other seat draws its strategy iid from sigma. Seeds: 60 per player count from the **dev** block (offset 100), 3 player counts, every seat of every table, standard length; 6,480 games, 3 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-6e276bf65640.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.314 [+0.253, +0.378]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.314 [-0.378, -0.253] | 0 |
| run1 | +0.122 [+0.078, +0.167] | -0.192 [-0.264, -0.114] | 1 |
| run2 | +0.314 [+0.253, +0.378] | +0.000 [+0.000, +0.000] | 0 |
| exploit_run2 | +0.428 [+0.361, +0.494] | +0.114 [+0.019, +0.208] | 0 |
| control | +0.361 [+0.294, +0.431] | +0.047 [-0.056, +0.144] | 0 |
| defense | +0.344 [+0.286, +0.403] | +0.031 [-0.053, +0.114] | 0 |
| magnate | +0.383 [+0.311, +0.456] | +0.069 [-0.031, +0.169] | 0 |
| v2_run2_vs_run2_shaped/sample * | +0.419 [+0.350, +0.486] | +0.106 [+0.014, +0.200] | 2 |
| v2_run2_vs_run2_shaped/guarded * | +0.403 [+0.342, +0.461] | +0.089 [-0.006, +0.175] | 0 |

Gain by engine seat index (candidates; the first player is drawn at random, so the index is not turn order): v2_run2_vs_run2_shaped/sample: +0.158, +0.067, +0.092; v2_run2_vs_run2_shaped/guarded: +0.192, +0.042, +0.033

NashConv (sum of the seats' gains): v2_run2_vs_run2_shaped/sample: +0.317 [+0.042, +0.600]; v2_run2_vs_run2_shaped/guarded: +0.267 [-0.017, +0.525]

## 4 players

Value of sigma: +0.179 [+0.129, +0.231]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.179 [-0.231, -0.129] | 0 |
| run1 | +0.083 [+0.050, +0.117] | -0.096 [-0.158, -0.029] | 0 |
| run2 | +0.179 [+0.129, +0.231] | +0.000 [+0.000, +0.000] | 0 |
| exploit_run2 | +0.410 [+0.348, +0.471] | +0.231 [+0.146, +0.312] | 0 |
| control | +0.331 [+0.269, +0.396] | +0.152 [+0.073, +0.231] | 0 |
| defense | +0.268 [+0.216, +0.321] | +0.089 [+0.016, +0.163] | 0 |
| magnate | +0.335 [+0.271, +0.402] | +0.156 [+0.077, +0.231] | 0 |
| v2_run2_vs_run2_shaped/sample * | +0.306 [+0.246, +0.367] | +0.127 [+0.054, +0.198] | 0 |
| v2_run2_vs_run2_shaped/guarded * | +0.404 [+0.350, +0.454] | +0.225 [+0.146, +0.298] | 0 |

Gain by engine seat index (candidates; the first player is drawn at random, so the index is not turn order): v2_run2_vs_run2_shaped/sample: +0.075, +0.150, +0.150, +0.133; v2_run2_vs_run2_shaped/guarded: +0.217, +0.217, +0.200, +0.267

NashConv (sum of the seats' gains): v2_run2_vs_run2_shaped/sample: +0.508 [+0.216, +0.792]; v2_run2_vs_run2_shaped/guarded: +0.900 [+0.583, +1.192]

## 5 players

Value of sigma: +0.192 [+0.152, +0.233]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.192 [-0.233, -0.152] | 0 |
| run1 | +0.080 [+0.050, +0.107] | -0.112 [-0.165, -0.060] | 0 |
| run2 | +0.192 [+0.152, +0.233] | +0.000 [+0.000, +0.000] | 0 |
| exploit_run2 | +0.318 [+0.273, +0.362] | +0.127 [+0.063, +0.190] | 0 |
| control | +0.268 [+0.220, +0.320] | +0.077 [+0.008, +0.145] | 0 |
| defense | +0.265 [+0.215, +0.315] | +0.073 [+0.013, +0.130] | 0 |
| magnate | +0.342 [+0.293, +0.393] | +0.150 [+0.082, +0.220] | 0 |
| v2_run2_vs_run2_shaped/sample * | +0.273 [+0.222, +0.330] | +0.082 [+0.018, +0.147] | 0 |
| v2_run2_vs_run2_shaped/guarded * | +0.378 [+0.325, +0.432] | +0.187 [+0.118, +0.253] | 0 |

Gain by engine seat index (candidates; the first player is drawn at random, so the index is not turn order): v2_run2_vs_run2_shaped/sample: +0.042, +0.150, +0.050, +0.025, +0.142; v2_run2_vs_run2_shaped/guarded: +0.250, +0.183, +0.183, +0.108, +0.208

NashConv (sum of the seats' gains): v2_run2_vs_run2_shaped/sample: +0.408 [+0.092, +0.733]; v2_run2_vs_run2_shaped/guarded: +0.933 [+0.591, +1.267]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| merchant | -0.228 [-0.260, -0.199] | -0.872 [-0.992, -0.756] |
| run1 | -0.133 [-0.172, -0.095] | -0.506 [-0.653, -0.361] |
| run2 | +0.000 [+0.000, +0.000] | +0.000 [+0.000, +0.000] |
| exploit_run2 | +0.157 [+0.109, +0.202] | +0.633 [+0.450, +0.808] |
| control | +0.092 [+0.043, +0.138] | +0.378 [+0.197, +0.553] |
| defense | +0.064 [+0.021, +0.106] | +0.271 [+0.110, +0.432] |
| magnate | +0.125 [+0.078, +0.172] | +0.528 [+0.347, +0.711] |
| v2_run2_vs_run2_shaped/sample * | +0.105 [+0.060, +0.150] | +0.411 [+0.244, +0.586] |
| v2_run2_vs_run2_shaped/guarded * | +0.167 [+0.120, +0.212] | +0.700 [+0.519, +0.878] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of v2_run2_vs_run2_shaped/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 60 seeds (it understates near k = 60).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.156 | 0.075 |
| 10 | ±0.110 | 0.051 |
| 25 | ±0.070 | 0.027 |
| 50 | ±0.049 | 0.010 |
| 60 | ±0.045 | 0.000 |

## How well the gain of v2_run2_vs_run2_shaped/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 60 seeds (it understates near k = 60).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.159 | 0.074 |
| 10 | ±0.112 | 0.052 |
| 25 | ±0.071 | 0.028 |
| 50 | ±0.050 | 0.011 |
| 60 | ±0.046 | 0.000 |

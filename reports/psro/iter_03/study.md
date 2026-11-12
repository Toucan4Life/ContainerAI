# PSRO iteration 3: deviation study of the new response

Profile sigma: 3 players: run2 0.00, magnate 0.00, control 0.12, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.88; 4 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 1.00; 5 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 1.00. Every other seat draws its strategy iid from sigma. Seeds: 100 per player count from the **dev** block (offset 1200), 3 player counts, every seat of every table, standard length; 10,800 games, 0 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-79634ab95aa6.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.333 [+0.303, +0.364]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.202 [+0.158, +0.250] | -0.131 [-0.188, -0.074] | 0 |
| magnate | +0.257 [+0.217, +0.298] | -0.076 [-0.128, -0.024] | 0 |
| control | +0.225 [+0.183, +0.268] | -0.108 [-0.156, -0.057] | 0 |
| defense | +0.230 [+0.192, +0.270] | -0.103 [-0.150, -0.056] | 0 |
| exploit_run2 | +0.202 [+0.157, +0.247] | -0.131 [-0.189, -0.069] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.333 [-0.364, -0.303] | 0 |
| psro02 | +0.348 [+0.315, +0.383] | +0.015 [+0.008, +0.022] | 0 |
| psro03/sample * | +0.327 [+0.273, +0.380] | -0.006 [-0.066, +0.052] | 0 |
| psro03/guarded * | +0.340 [+0.290, +0.390] | +0.007 [-0.051, +0.064] | 0 |

Gain by turn position (candidates; position 0 moves first): psro03/sample: +0.068, -0.061, -0.026; psro03/guarded: +0.093, -0.036, -0.036

NashConv (sum of the seats' gains): psro03/sample: -0.019 [-0.198, +0.157]; psro03/guarded: +0.021 [-0.154, +0.191]

## 4 players

Value of sigma: +0.250 [+0.250, +0.250]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.070 [+0.046, +0.095] | -0.180 [-0.204, -0.155] | 0 |
| magnate | +0.139 [+0.104, +0.174] | -0.111 [-0.146, -0.076] | 0 |
| control | +0.100 [+0.075, +0.125] | -0.150 [-0.175, -0.125] | 0 |
| defense | +0.091 [+0.066, +0.119] | -0.159 [-0.184, -0.131] | 0 |
| exploit_run2 | +0.133 [+0.100, +0.166] | -0.117 [-0.150, -0.084] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.250 [-0.250, -0.250] | 0 |
| psro02 | +0.250 [+0.250, +0.250] | +0.000 [+0.000, +0.000] | 0 |
| psro03/sample * | +0.284 [+0.241, +0.328] | +0.034 [-0.009, +0.077] | 0 |
| psro03/guarded * | +0.295 [+0.258, +0.338] | +0.045 [+0.007, +0.087] | 0 |

Gain by turn position (candidates; position 0 moves first): psro03/sample: +0.060, +0.095, -0.025, +0.005; psro03/guarded: +0.090, +0.080, -0.065, +0.075

NashConv (sum of the seats' gains): psro03/sample: +0.135 [-0.035, +0.310]; psro03/guarded: +0.180 [+0.030, +0.350]

## 5 players

Value of sigma: +0.200 [+0.200, +0.200]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.061 [+0.041, +0.083] | -0.139 [-0.159, -0.117] | 0 |
| magnate | +0.149 [+0.118, +0.179] | -0.051 [-0.082, -0.021] | 0 |
| control | +0.086 [+0.062, +0.110] | -0.114 [-0.138, -0.090] | 0 |
| defense | +0.108 [+0.082, +0.137] | -0.092 [-0.118, -0.063] | 0 |
| exploit_run2 | +0.126 [+0.101, +0.151] | -0.074 [-0.099, -0.049] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.200 [-0.200, -0.200] | 0 |
| psro02 | +0.200 [+0.200, +0.200] | +0.000 [+0.000, +0.000] | 0 |
| psro03/sample * | +0.196 [+0.168, +0.224] | -0.004 [-0.032, +0.024] | 0 |
| psro03/guarded * | +0.244 [+0.207, +0.286] | +0.044 [+0.007, +0.086] | 0 |

Gain by turn position (candidates; position 0 moves first): psro03/sample: +0.010, +0.020, -0.015, +0.020, -0.055; psro03/guarded: +0.050, +0.020, +0.075, +0.100, -0.025

NashConv (sum of the seats' gains): psro03/sample: -0.020 [-0.160, +0.120]; psro03/guarded: +0.220 [+0.035, +0.430]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| run2 | -0.150 [-0.171, -0.127] | -0.603 [-0.675, -0.525] |
| magnate | -0.080 [-0.103, -0.057] | -0.310 [-0.396, -0.226] |
| control | -0.124 [-0.144, -0.103] | -0.498 [-0.568, -0.423] |
| defense | -0.118 [-0.138, -0.097] | -0.468 [-0.541, -0.390] |
| exploit_run2 | -0.108 [-0.131, -0.084] | -0.411 [-0.495, -0.329] |
| merchant | -0.261 [-0.271, -0.251] | -1.000 [-1.030, -0.970] |
| psro02 | +0.005 [+0.003, +0.007] | +0.015 [+0.008, +0.022] |
| psro03/sample * | +0.008 [-0.018, +0.034] | +0.032 [-0.062, +0.128] |
| psro03/guarded * | +0.032 [+0.005, +0.059] | +0.140 [+0.035, +0.243] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of psro03/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.120 | 0.061 |
| 10 | ±0.085 | 0.041 |
| 25 | ±0.054 | 0.022 |
| 50 | ±0.038 | 0.013 |
| 100 | ±0.027 | 0.000 |

## How well the gain of psro03/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.122 | 0.061 |
| 10 | ±0.086 | 0.042 |
| 25 | ±0.054 | 0.024 |
| 50 | ±0.038 | 0.014 |
| 100 | ±0.027 | 0.000 |

# PSRO iteration 5: deviation study of the new response

Profile sigma: 3 players: run2 0.00, magnate 0.15, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.11, psro03 0.73; 4 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.37, psro03 0.63; 5 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.91, psro03 0.09. Every other seat draws its strategy iid from sigma. Seeds: 100 per player count from the **dev** block (offset 1400), 3 player counts, every seat of every table, standard length; 12,000 games, 0 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-79634ab95aa6.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.320 [+0.292, +0.347]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.198 [+0.157, +0.242] | -0.122 [-0.171, -0.070] | 0 |
| magnate | +0.253 [+0.210, +0.297] | -0.067 [-0.110, -0.020] | 0 |
| control | +0.215 [+0.168, +0.267] | -0.105 [-0.162, -0.046] | 0 |
| defense | +0.208 [+0.167, +0.258] | -0.112 [-0.161, -0.058] | 0 |
| exploit_run2 | +0.180 [+0.143, +0.217] | -0.140 [-0.187, -0.091] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.320 [-0.347, -0.292] | 0 |
| psro02 | +0.333 [+0.285, +0.383] | +0.013 [-0.040, +0.065] | 0 |
| psro03 | +0.332 [+0.295, +0.368] | +0.012 [-0.002, +0.025] | 0 |
| psro05/sample * | +0.358 [+0.303, +0.412] | +0.038 [-0.023, +0.098] | 0 |
| psro05/guarded * | +0.418 [+0.368, +0.468] | +0.098 [+0.042, +0.155] | 0 |

Gain by turn position (candidates; position 0 moves first): psro05/sample: -0.024, +0.044, +0.095; psro05/guarded: +0.111, +0.114, +0.070

NashConv (sum of the seats' gains): psro05/sample: +0.115 [-0.070, +0.293]; psro05/guarded: +0.295 [+0.127, +0.466]

## 4 players

Value of sigma: +0.234 [+0.209, +0.261]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.089 [+0.062, +0.117] | -0.146 [-0.182, -0.109] | 0 |
| magnate | +0.115 [+0.089, +0.141] | -0.119 [-0.157, -0.083] | 0 |
| control | +0.079 [+0.055, +0.106] | -0.156 [-0.194, -0.114] | 0 |
| defense | +0.096 [+0.071, +0.124] | -0.138 [-0.177, -0.098] | 0 |
| exploit_run2 | +0.100 [+0.069, +0.131] | -0.134 [-0.177, -0.092] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.234 [-0.261, -0.209] | 0 |
| psro02 | +0.222 [+0.185, +0.261] | -0.012 [-0.046, +0.023] | 0 |
| psro03 | +0.242 [+0.205, +0.278] | +0.007 [-0.013, +0.027] | 0 |
| psro05/sample * | +0.265 [+0.223, +0.306] | +0.031 [-0.022, +0.083] | 0 |
| psro05/guarded * | +0.273 [+0.232, +0.315] | +0.038 [-0.011, +0.089] | 0 |

Gain by turn position (candidates; position 0 moves first): psro05/sample: +0.048, +0.016, +0.059, -0.001; psro05/guarded: -0.007, +0.126, +0.069, -0.036

NashConv (sum of the seats' gains): psro05/sample: +0.122 [-0.087, +0.332]; psro05/guarded: +0.152 [-0.046, +0.357]

## 5 players

Value of sigma: +0.196 [+0.174, +0.218]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.084 [+0.062, +0.108] | -0.112 [-0.140, -0.082] | 0 |
| magnate | +0.135 [+0.108, +0.162] | -0.061 [-0.096, -0.026] | 0 |
| control | +0.087 [+0.065, +0.111] | -0.109 [-0.138, -0.079] | 0 |
| defense | +0.087 [+0.062, +0.113] | -0.109 [-0.139, -0.080] | 0 |
| exploit_run2 | +0.106 [+0.083, +0.129] | -0.090 [-0.120, -0.059] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.196 [-0.218, -0.174] | 0 |
| psro02 | +0.191 [+0.167, +0.215] | -0.005 [-0.009, -0.001] | 0 |
| psro03 | +0.245 [+0.207, +0.283] | +0.049 [+0.008, +0.091] | 0 |
| psro05/sample * | +0.194 [+0.162, +0.228] | -0.002 [-0.043, +0.041] | 0 |
| psro05/guarded * | +0.216 [+0.184, +0.247] | +0.020 [-0.018, +0.057] | 0 |

Gain by turn position (candidates; position 0 moves first): psro05/sample: -0.001, -0.032, -0.028, +0.025, +0.026; psro05/guarded: -0.091, +0.133, -0.053, +0.045, +0.066

NashConv (sum of the seats' gains): psro05/sample: -0.009 [-0.217, +0.205]; psro05/guarded: +0.101 [-0.088, +0.286]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| run2 | -0.126 [-0.149, -0.103] | -0.502 [-0.587, -0.417] |
| magnate | -0.082 [-0.104, -0.059] | -0.327 [-0.414, -0.237] |
| control | -0.123 [-0.148, -0.097] | -0.494 [-0.586, -0.402] |
| defense | -0.120 [-0.143, -0.095] | -0.477 [-0.568, -0.387] |
| exploit_run2 | -0.121 [-0.144, -0.099] | -0.469 [-0.559, -0.384] |
| merchant | -0.250 [-0.265, -0.236] | -0.959 [-1.017, -0.903] |
| psro02 | -0.001 [-0.022, +0.018] | -0.011 [-0.078, +0.054] |
| psro03 | +0.023 [+0.007, +0.039] | +0.103 [+0.031, +0.179] |
| psro05/sample * | +0.022 [-0.008, +0.051] | +0.076 [-0.039, +0.190] |
| psro05/guarded * | +0.052 [+0.024, +0.080] | +0.183 [+0.071, +0.291] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of psro05/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.134 | 0.066 |
| 10 | ±0.095 | 0.046 |
| 25 | ±0.060 | 0.028 |
| 50 | ±0.042 | 0.015 |
| 100 | ±0.030 | 0.000 |

## How well the gain of psro05/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.127 | 0.065 |
| 10 | ±0.090 | 0.043 |
| 25 | ±0.057 | 0.025 |
| 50 | ±0.040 | 0.014 |
| 100 | ±0.028 | 0.000 |

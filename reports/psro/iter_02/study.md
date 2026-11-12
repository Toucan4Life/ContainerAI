# PSRO iteration 2: deviation study of the new response

Profile sigma: 3 players: run2 0.00, magnate 1.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00; 4 players: run2 0.00, magnate 0.07, control 0.37, defense 0.50, exploit_run2 0.06, merchant 0.00; 5 players: run2 0.00, magnate 1.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00. Every other seat draws its strategy iid from sigma. Seeds: 100 per player count from the **dev** block (offset 1100), 3 player counts, every seat of every table, standard length; 9,600 games, 0 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-ce1dbd46f9d3.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.333 [+0.333, +0.333]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.205 [+0.165, +0.248] | -0.128 [-0.168, -0.085] | 0 |
| magnate | +0.333 [+0.333, +0.333] | +0.000 [+0.000, +0.000] | 0 |
| control | +0.252 [+0.208, +0.293] | -0.082 [-0.125, -0.040] | 0 |
| defense | +0.283 [+0.232, +0.333] | -0.050 [-0.102, +0.000] | 0 |
| exploit_run2 | +0.270 [+0.223, +0.313] | -0.063 [-0.110, -0.020] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.333 [-0.333, -0.333] | 0 |
| psro02/sample * | +0.362 [+0.312, +0.412] | +0.028 [-0.022, +0.078] | 0 |
| psro02/guarded * | +0.392 [+0.335, +0.445] | +0.058 [+0.002, +0.112] | 0 |

Gain by turn position (candidates; position 0 moves first): psro02/sample: -0.055, +0.040, +0.100; psro02/guarded: +0.000, +0.015, +0.160

NashConv (sum of the seats' gains): psro02/sample: +0.085 [-0.065, +0.235]; psro02/guarded: +0.175 [+0.005, +0.335]

## 4 players

Value of sigma: +0.252 [+0.225, +0.278]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.186 [+0.147, +0.226] | -0.066 [-0.111, -0.018] | 0 |
| magnate | +0.306 [+0.261, +0.351] | +0.054 [+0.005, +0.103] | 0 |
| control | +0.271 [+0.229, +0.314] | +0.019 [-0.014, +0.050] | 0 |
| defense | +0.230 [+0.191, +0.270] | -0.022 [-0.045, +0.003] | 0 |
| exploit_run2 | +0.258 [+0.220, +0.296] | +0.005 [-0.038, +0.049] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.252 [-0.278, -0.225] | 0 |
| psro02/sample * | +0.352 [+0.306, +0.400] | +0.100 [+0.048, +0.155] | 0 |
| psro02/guarded * | +0.434 [+0.396, +0.474] | +0.182 [+0.132, +0.230] | 0 |

Gain by turn position (candidates; position 0 moves first): psro02/sample: +0.121, -0.013, +0.114, +0.180; psro02/guarded: +0.216, +0.157, +0.139, +0.215

NashConv (sum of the seats' gains): psro02/sample: +0.402 [+0.194, +0.619]; psro02/guarded: +0.727 [+0.529, +0.919]

## 5 players

Value of sigma: +0.200 [+0.200, +0.200]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.130 [+0.098, +0.162] | -0.070 [-0.102, -0.038] | 0 |
| magnate | +0.200 [+0.200, +0.200] | +0.000 [+0.000, +0.000] | 0 |
| control | +0.174 [+0.144, +0.204] | -0.026 [-0.056, +0.004] | 0 |
| defense | +0.114 [+0.086, +0.143] | -0.086 [-0.114, -0.057] | 0 |
| exploit_run2 | +0.138 [+0.108, +0.169] | -0.062 [-0.092, -0.031] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.200 [-0.200, -0.200] | 0 |
| psro02/sample * | +0.241 [+0.204, +0.279] | +0.041 [+0.004, +0.079] | 0 |
| psro02/guarded * | +0.255 [+0.219, +0.291] | +0.055 [+0.019, +0.091] | 0 |

Gain by turn position (candidates; position 0 moves first): psro02/sample: +0.095, +0.055, +0.015, +0.030, +0.010; psro02/guarded: +0.065, +0.035, +0.040, +0.070, +0.065

NashConv (sum of the seats' gains): psro02/sample: +0.205 [+0.020, +0.395]; psro02/guarded: +0.275 [+0.095, +0.455]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| run2 | -0.088 [-0.112, -0.065] | -0.333 [-0.424, -0.241] |
| magnate | +0.018 [+0.002, +0.035] | +0.072 [+0.007, +0.138] |
| control | -0.029 [-0.049, -0.010] | -0.099 [-0.174, -0.024] |
| defense | -0.053 [-0.074, -0.032] | -0.223 [-0.301, -0.145] |
| exploit_run2 | -0.040 [-0.065, -0.016] | -0.159 [-0.254, -0.070] |
| merchant | -0.262 [-0.271, -0.253] | -1.003 [-1.039, -0.967] |
| psro02/sample * | +0.057 [+0.028, +0.083] | +0.231 [+0.117, +0.332] |
| psro02/guarded * | +0.098 [+0.070, +0.126] | +0.392 [+0.283, +0.499] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of psro02/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.124 | 0.060 |
| 10 | ±0.088 | 0.044 |
| 25 | ±0.056 | 0.023 |
| 50 | ±0.039 | 0.014 |
| 100 | ±0.028 | 0.000 |

## How well the gain of psro02/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.121 | 0.059 |
| 10 | ±0.085 | 0.042 |
| 25 | ±0.054 | 0.023 |
| 50 | ±0.038 | 0.013 |
| 100 | ±0.027 | 0.000 |

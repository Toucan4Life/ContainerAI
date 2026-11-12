# PSRO iteration 1: deviation study of the new response

Profile sigma: 3 players: run2 0.00, magnate 1.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00; 4 players: run2 0.00, magnate 0.07, control 0.37, defense 0.50, exploit_run2 0.06, merchant 0.00; 5 players: run2 0.00, magnate 1.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00. Every other seat draws its strategy iid from sigma. Seeds: 100 per player count from the **dev** block (offset 1000), 3 player counts, every seat of every table, standard length; 9,600 games, 0 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-ce1dbd46f9d3.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.333 [+0.333, +0.333]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.250 [+0.207, +0.293] | -0.083 [-0.127, -0.040] | 0 |
| magnate | +0.333 [+0.333, +0.333] | +0.000 [+0.000, +0.000] | 0 |
| control | +0.297 [+0.248, +0.348] | -0.037 [-0.085, +0.015] | 0 |
| defense | +0.308 [+0.260, +0.357] | -0.025 [-0.073, +0.023] | 0 |
| exploit_run2 | +0.282 [+0.230, +0.333] | -0.052 [-0.103, +0.000] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.333 [-0.333, -0.333] | 0 |
| psro01/sample * | +0.378 [+0.327, +0.435] | +0.045 [-0.007, +0.102] | 0 |
| psro01/guarded * | +0.357 [+0.303, +0.407] | +0.023 [-0.030, +0.073] | 0 |

Gain by turn position (candidates; position 0 moves first): psro01/sample: +0.100, +0.040, -0.005; psro01/guarded: +0.130, +0.025, -0.085

NashConv (sum of the seats' gains): psro01/sample: +0.135 [-0.020, +0.305]; psro01/guarded: +0.070 [-0.090, +0.220]

## 4 players

Value of sigma: +0.240 [+0.213, +0.266]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.170 [+0.135, +0.204] | -0.070 [-0.111, -0.028] | 0 |
| magnate | +0.291 [+0.251, +0.331] | +0.052 [+0.005, +0.098] | 0 |
| control | +0.233 [+0.194, +0.271] | -0.007 [-0.038, +0.022] | 0 |
| defense | +0.235 [+0.193, +0.276] | -0.005 [-0.028, +0.021] | 0 |
| exploit_run2 | +0.266 [+0.228, +0.304] | +0.027 [-0.016, +0.067] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.240 [-0.266, -0.213] | 0 |
| psro01/sample * | +0.246 [+0.201, +0.291] | +0.007 [-0.046, +0.057] | 0 |
| psro01/guarded * | +0.304 [+0.264, +0.343] | +0.064 [+0.019, +0.108] | 0 |

Gain by turn position (candidates; position 0 moves first): psro01/sample: -0.055, +0.015, +0.109, -0.043; psro01/guarded: -0.005, +0.110, +0.094, +0.057

NashConv (sum of the seats' gains): psro01/sample: +0.026 [-0.184, +0.226]; psro01/guarded: +0.256 [+0.078, +0.430]

## 5 players

Value of sigma: +0.200 [+0.200, +0.200]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.116 [+0.088, +0.143] | -0.084 [-0.112, -0.057] | 0 |
| magnate | +0.200 [+0.200, +0.200] | +0.000 [+0.000, +0.000] | 0 |
| control | +0.171 [+0.141, +0.203] | -0.029 [-0.059, +0.003] | 0 |
| defense | +0.139 [+0.109, +0.171] | -0.061 [-0.091, -0.029] | 0 |
| exploit_run2 | +0.131 [+0.102, +0.163] | -0.069 [-0.098, -0.037] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.200 [-0.200, -0.200] | 0 |
| psro01/sample * | +0.111 [+0.088, +0.137] | -0.089 [-0.112, -0.063] | 0 |
| psro01/guarded * | +0.132 [+0.105, +0.159] | -0.068 [-0.095, -0.041] | 0 |

Gain by turn position (candidates; position 0 moves first): psro01/sample: -0.090, -0.165, -0.065, -0.120, -0.005; psro01/guarded: -0.085, -0.160, -0.035, -0.115, +0.055

NashConv (sum of the seats' gains): psro01/sample: -0.445 [-0.560, -0.315]; psro01/guarded: -0.340 [-0.475, -0.205]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| run2 | -0.079 [-0.102, -0.056] | -0.316 [-0.404, -0.228] |
| magnate | +0.017 [+0.001, +0.033] | +0.069 [+0.004, +0.133] |
| control | -0.024 [-0.046, -0.002] | -0.095 [-0.176, -0.013] |
| defense | -0.030 [-0.050, -0.009] | -0.133 [-0.208, -0.052] |
| exploit_run2 | -0.031 [-0.055, -0.006] | -0.131 [-0.220, -0.040] |
| merchant | -0.258 [-0.267, -0.249] | -0.986 [-1.024, -0.950] |
| psro01/sample * | -0.012 [-0.039, +0.014] | -0.095 [-0.192, +0.001] |
| psro01/guarded * | +0.006 [-0.019, +0.032] | -0.005 [-0.102, +0.089] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of psro01/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.118 | 0.058 |
| 10 | ±0.083 | 0.040 |
| 25 | ±0.053 | 0.023 |
| 50 | ±0.037 | 0.013 |
| 100 | ±0.026 | 0.000 |

## How well the gain of psro01/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.112 | 0.057 |
| 10 | ±0.079 | 0.039 |
| 25 | ±0.050 | 0.023 |
| 50 | ±0.035 | 0.013 |
| 100 | ±0.025 | 0.000 |

# PSRO iteration 4: deviation study of the new response

Profile sigma: 3 players: run2 0.00, magnate 0.15, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.11, psro03 0.73; 4 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.37, psro03 0.63; 5 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.91, psro03 0.09. Every other seat draws its strategy iid from sigma. Seeds: 100 per player count from the **dev** block (offset 1300), 3 player counts, every seat of every table, standard length; 12,000 games, 0 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-79634ab95aa6.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.331 [+0.299, +0.364]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.197 [+0.153, +0.240] | -0.134 [-0.187, -0.079] | 0 |
| magnate | +0.218 [+0.178, +0.262] | -0.112 [-0.162, -0.061] | 0 |
| control | +0.215 [+0.172, +0.263] | -0.116 [-0.175, -0.053] | 0 |
| defense | +0.228 [+0.183, +0.272] | -0.102 [-0.158, -0.048] | 0 |
| exploit_run2 | +0.207 [+0.168, +0.248] | -0.124 [-0.171, -0.074] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.331 [-0.364, -0.299] | 0 |
| psro02 | +0.343 [+0.293, +0.390] | +0.013 [-0.034, +0.060] | 0 |
| psro03 | +0.352 [+0.312, +0.395] | +0.021 [+0.008, +0.035] | 0 |
| psro04/sample * | +0.367 [+0.307, +0.423] | +0.036 [-0.035, +0.108] | 0 |
| psro04/guarded * | +0.362 [+0.308, +0.418] | +0.031 [-0.036, +0.099] | 0 |

Gain by turn position (candidates; position 0 moves first): psro04/sample: +0.035, -0.050, +0.123; psro04/guarded: +0.030, -0.050, +0.113

NashConv (sum of the seats' gains): psro04/sample: +0.108 [-0.105, +0.324]; psro04/guarded: +0.093 [-0.109, +0.298]

## 4 players

Value of sigma: +0.267 [+0.239, +0.294]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.080 [+0.054, +0.107] | -0.187 [-0.228, -0.145] | 0 |
| magnate | +0.107 [+0.081, +0.134] | -0.159 [-0.198, -0.119] | 0 |
| control | +0.107 [+0.081, +0.135] | -0.159 [-0.198, -0.120] | 0 |
| defense | +0.083 [+0.060, +0.105] | -0.184 [-0.219, -0.148] | 0 |
| exploit_run2 | +0.100 [+0.068, +0.133] | -0.167 [-0.208, -0.123] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.267 [-0.294, -0.239] | 0 |
| psro02 | +0.220 [+0.180, +0.259] | -0.047 [-0.082, -0.011] | 0 |
| psro03 | +0.294 [+0.256, +0.331] | +0.027 [+0.006, +0.048] | 0 |
| psro04/sample * | +0.292 [+0.247, +0.338] | +0.026 [-0.024, +0.077] | 0 |
| psro04/guarded * | +0.292 [+0.245, +0.340] | +0.026 [-0.030, +0.085] | 0 |

Gain by turn position (candidates; position 0 moves first): psro04/sample: -0.011, +0.018, +0.047, +0.050; psro04/guarded: +0.009, +0.033, +0.047, +0.015

NashConv (sum of the seats' gains): psro04/sample: +0.104 [-0.094, +0.310]; psro04/guarded: +0.104 [-0.119, +0.340]

## 5 players

Value of sigma: +0.196 [+0.171, +0.222]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.066 [+0.047, +0.088] | -0.130 [-0.163, -0.095] | 0 |
| magnate | +0.140 [+0.111, +0.170] | -0.056 [-0.098, -0.015] | 0 |
| control | +0.111 [+0.085, +0.138] | -0.085 [-0.126, -0.045] | 0 |
| defense | +0.090 [+0.066, +0.118] | -0.106 [-0.144, -0.067] | 0 |
| exploit_run2 | +0.118 [+0.091, +0.146] | -0.078 [-0.115, -0.040] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.196 [-0.222, -0.171] | 0 |
| psro02 | +0.194 [+0.167, +0.224] | -0.002 [-0.006, +0.002] | 0 |
| psro03 | +0.216 [+0.186, +0.244] | +0.020 [-0.019, +0.057] | 0 |
| psro04/sample * | +0.224 [+0.190, +0.261] | +0.028 [-0.013, +0.070] | 0 |
| psro04/guarded * | +0.223 [+0.188, +0.256] | +0.027 [-0.013, +0.067] | 0 |

Gain by turn position (candidates; position 0 moves first): psro04/sample: +0.035, +0.028, +0.065, +0.015, -0.003; psro04/guarded: +0.065, +0.033, -0.070, +0.045, +0.062

NashConv (sum of the seats' gains): psro04/sample: +0.140 [-0.064, +0.352]; psro04/guarded: +0.135 [-0.065, +0.337]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| run2 | -0.150 [-0.174, -0.125] | -0.599 [-0.691, -0.508] |
| magnate | -0.109 [-0.133, -0.085] | -0.418 [-0.513, -0.322] |
| control | -0.120 [-0.145, -0.093] | -0.469 [-0.569, -0.366] |
| defense | -0.131 [-0.155, -0.106] | -0.524 [-0.618, -0.429] |
| exploit_run2 | -0.123 [-0.148, -0.098] | -0.476 [-0.572, -0.382] |
| merchant | -0.264 [-0.281, -0.248] | -1.013 [-1.076, -0.947] |
| psro02 | -0.012 [-0.032, +0.008] | -0.053 [-0.120, +0.014] |
| psro03 | +0.023 [+0.007, +0.039] | +0.091 [+0.018, +0.164] |
| psro04/sample * | +0.030 [-0.002, +0.063] | +0.117 [+0.002, +0.240] |
| psro04/guarded * | +0.028 [-0.005, +0.060] | +0.111 [-0.014, +0.231] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of psro04/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.148 | 0.072 |
| 10 | ±0.104 | 0.051 |
| 25 | ±0.066 | 0.028 |
| 50 | ±0.047 | 0.017 |
| 100 | ±0.033 | 0.000 |

## How well the gain of psro04/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.143 | 0.072 |
| 10 | ±0.101 | 0.047 |
| 25 | ±0.064 | 0.030 |
| 50 | ±0.045 | 0.017 |
| 100 | ±0.032 | 0.000 |

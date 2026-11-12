# PSRO iteration 6: deviation study of the new response

Profile sigma: 3 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.00, psro03 0.20, psro05 0.80; 4 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.00, psro03 0.64, psro05 0.35; 5 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.00, psro03 1.00, psro05 0.00. Every other seat draws its strategy iid from sigma. Seeds: 100 per player count from the **dev** block (offset 1500), 3 player counts, every seat of every table, standard length; 13,200 games, 0 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-f5e95720a30f.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.310 [+0.278, +0.342]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.173 [+0.130, +0.217] | -0.137 [-0.191, -0.084] | 0 |
| magnate | +0.240 [+0.195, +0.287] | -0.070 [-0.124, -0.015] | 0 |
| control | +0.213 [+0.170, +0.257] | -0.097 [-0.153, -0.043] | 0 |
| defense | +0.202 [+0.158, +0.245] | -0.109 [-0.167, -0.055] | 0 |
| exploit_run2 | +0.195 [+0.150, +0.238] | -0.115 [-0.169, -0.062] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.310 [-0.342, -0.278] | 0 |
| psro02 | +0.327 [+0.278, +0.377] | +0.016 [-0.039, +0.072] | 0 |
| psro03 | +0.305 [+0.257, +0.350] | -0.005 [-0.055, +0.045] | 0 |
| psro05 | +0.312 [+0.272, +0.350] | +0.001 [-0.011, +0.014] | 0 |
| psro06/sample * | +0.338 [+0.288, +0.392] | +0.028 [-0.028, +0.086] | 0 |
| psro06/guarded * | +0.340 [+0.288, +0.392] | +0.030 [-0.026, +0.085] | 0 |

Gain by turn position (candidates; position 0 moves first): psro06/sample: +0.002, -0.076, +0.158; psro06/guarded: +0.057, -0.006, +0.038

NashConv (sum of the seats' gains): psro06/sample: +0.084 [-0.084, +0.258]; psro06/guarded: +0.089 [-0.079, +0.256]

## 4 players

Value of sigma: +0.265 [+0.243, +0.288]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.086 [+0.060, +0.114] | -0.179 [-0.214, -0.142] | 0 |
| magnate | +0.131 [+0.099, +0.169] | -0.134 [-0.173, -0.091] | 0 |
| control | +0.098 [+0.070, +0.128] | -0.168 [-0.202, -0.131] | 0 |
| defense | +0.099 [+0.072, +0.125] | -0.167 [-0.205, -0.130] | 0 |
| exploit_run2 | +0.077 [+0.054, +0.101] | -0.188 [-0.222, -0.155] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.265 [-0.288, -0.243] | 0 |
| psro02 | +0.181 [+0.144, +0.218] | -0.084 [-0.129, -0.041] | 0 |
| psro03 | +0.249 [+0.216, +0.284] | -0.017 [-0.037, +0.006] | 0 |
| psro05 | +0.296 [+0.253, +0.336] | +0.031 [-0.010, +0.068] | 0 |
| psro06/sample * | +0.241 [+0.201, +0.281] | -0.024 [-0.068, +0.021] | 0 |
| psro06/guarded * | +0.264 [+0.223, +0.307] | -0.002 [-0.048, +0.047] | 0 |

Gain by turn position (candidates; position 0 moves first): psro06/sample: -0.005, -0.050, -0.043, +0.002; psro06/guarded: -0.085, -0.035, +0.082, +0.032

NashConv (sum of the seats' gains): psro06/sample: -0.096 [-0.273, +0.085]; psro06/guarded: -0.006 [-0.193, +0.188]

## 5 players

Value of sigma: +0.200 [+0.200, +0.200]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.070 [+0.051, +0.092] | -0.130 [-0.149, -0.108] | 0 |
| magnate | +0.117 [+0.088, +0.147] | -0.083 [-0.112, -0.053] | 0 |
| control | +0.098 [+0.072, +0.126] | -0.102 [-0.128, -0.074] | 0 |
| defense | +0.091 [+0.068, +0.116] | -0.109 [-0.132, -0.084] | 0 |
| exploit_run2 | +0.063 [+0.043, +0.086] | -0.137 [-0.157, -0.114] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.200 [-0.200, -0.200] | 0 |
| psro02 | +0.201 [+0.169, +0.236] | +0.001 [-0.031, +0.036] | 0 |
| psro03 | +0.200 [+0.200, +0.200] | +0.000 [+0.000, +0.000] | 0 |
| psro05 | +0.188 [+0.157, +0.221] | -0.012 [-0.043, +0.021] | 0 |
| psro06/sample * | +0.210 [+0.177, +0.242] | +0.010 [-0.023, +0.042] | 0 |
| psro06/guarded * | +0.216 [+0.186, +0.249] | +0.016 [-0.014, +0.049] | 0 |

Gain by turn position (candidates; position 0 moves first): psro06/sample: +0.030, -0.010, +0.020, +0.045, -0.035; psro06/guarded: +0.035, +0.065, -0.055, +0.015, +0.020

NashConv (sum of the seats' gains): psro06/sample: +0.050 [-0.115, +0.210]; psro06/guarded: +0.080 [-0.070, +0.245]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| run2 | -0.149 [-0.170, -0.126] | -0.592 [-0.667, -0.511] |
| magnate | -0.096 [-0.119, -0.070] | -0.387 [-0.478, -0.293] |
| control | -0.122 [-0.146, -0.098] | -0.491 [-0.575, -0.401] |
| defense | -0.128 [-0.152, -0.104] | -0.513 [-0.598, -0.427] |
| exploit_run2 | -0.147 [-0.169, -0.124] | -0.594 [-0.671, -0.512] |
| merchant | -0.259 [-0.272, -0.246] | -0.997 [-1.041, -0.955] |
| psro02 | -0.022 [-0.049, +0.004] | -0.095 [-0.196, +0.004] |
| psro03 | -0.007 [-0.025, +0.011] | -0.027 [-0.085, +0.032] |
| psro05 | +0.007 [-0.010, +0.023] | +0.022 [-0.052, +0.093] |
| psro06/sample * | +0.005 [-0.022, +0.030] | +0.013 [-0.087, +0.109] |
| psro06/guarded * | +0.015 [-0.011, +0.041] | +0.054 [-0.040, +0.156] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of psro06/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.119 | 0.061 |
| 10 | ±0.084 | 0.042 |
| 25 | ±0.053 | 0.023 |
| 50 | ±0.038 | 0.014 |
| 100 | ±0.027 | 0.000 |

## How well the gain of psro06/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.117 | 0.057 |
| 10 | ±0.083 | 0.038 |
| 25 | ±0.052 | 0.024 |
| 50 | ±0.037 | 0.014 |
| 100 | ±0.026 | 0.000 |

# PSRO, holdout study of the meta-strategy that iteration 7 attacked

Profile sigma: 3 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.00, psro03 0.20, psro05 0.80; 4 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.00, psro03 0.64, psro05 0.35; 5 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.00, psro03 1.00, psro05 0.00. Every other seat draws its strategy iid from sigma. Seeds: 150 per player count from the **holdout** block (offset 200), 3 player counts, every seat of every table, standard length; 30,600 games, 3 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-a7d05e993a75.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.342 [+0.322, +0.364]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.221 [+0.182, +0.258] | -0.121 [-0.165, -0.080] | 0 |
| magnate | +0.253 [+0.217, +0.290] | -0.089 [-0.132, -0.045] | 0 |
| control | +0.217 [+0.180, +0.254] | -0.125 [-0.172, -0.082] | 0 |
| defense | +0.190 [+0.156, +0.228] | -0.152 [-0.192, -0.109] | 0 |
| exploit_run2 | +0.206 [+0.173, +0.238] | -0.136 [-0.175, -0.099] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.342 [-0.364, -0.322] | 1 |
| psro02 | +0.307 [+0.270, +0.343] | -0.035 [-0.076, +0.007] | 0 |
| psro03 | +0.306 [+0.267, +0.347] | -0.036 [-0.077, +0.004] | 1 |
| psro05 | +0.351 [+0.326, +0.378] | +0.009 [-0.001, +0.019] | 1 |
| iter01/sample * | +0.159 [+0.129, +0.189] | -0.183 [-0.216, -0.150] | 0 |
| iter01/guarded * | +0.202 [+0.167, +0.238] | -0.140 [-0.182, -0.099] | 0 |
| iter04/sample * | +0.338 [+0.298, +0.379] | -0.004 [-0.047, +0.039] | 0 |
| iter04/guarded * | +0.387 [+0.344, +0.427] | +0.045 [+0.001, +0.088] | 0 |
| iter06/sample * | +0.362 [+0.320, +0.404] | +0.020 [-0.026, +0.070] | 0 |
| iter06/guarded * | +0.357 [+0.319, +0.393] | +0.015 [-0.029, +0.056] | 0 |
| iter07/sample * | +0.344 [+0.303, +0.386] | +0.002 [-0.040, +0.047] | 0 |
| iter07/guarded * | +0.374 [+0.334, +0.413] | +0.032 [-0.009, +0.074] | 0 |

Gain by turn position (candidates; position 0 moves first): iter01/sample: -0.181, -0.110, -0.259; iter01/guarded: -0.171, -0.040, -0.209; iter04/sample: +0.063, -0.040, -0.035; iter04/guarded: +0.049, +0.050, +0.035; iter06/sample: +0.043, -0.010, +0.028; iter06/guarded: +0.056, +0.027, -0.039; iter07/sample: +0.029, +0.000, -0.022; iter07/guarded: +0.039, +0.060, -0.002

NashConv (sum of the seats' gains): iter01/sample: -0.549 [-0.649, -0.449]; iter01/guarded: -0.419 [-0.546, -0.298]; iter04/sample: -0.013 [-0.141, +0.116]; iter04/guarded: +0.134 [+0.002, +0.265]; iter06/sample: +0.061 [-0.079, +0.209]; iter06/guarded: +0.044 [-0.087, +0.169]; iter07/sample: +0.007 [-0.121, +0.140]; iter07/guarded: +0.097 [-0.026, +0.221]

## 4 players

Value of sigma: +0.247 [+0.228, +0.267]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.051 [+0.035, +0.068] | -0.196 [-0.222, -0.171] | 0 |
| magnate | +0.102 [+0.082, +0.125] | -0.145 [-0.174, -0.115] | 0 |
| control | +0.081 [+0.062, +0.102] | -0.166 [-0.194, -0.140] | 0 |
| defense | +0.099 [+0.075, +0.123] | -0.148 [-0.180, -0.116] | 0 |
| exploit_run2 | +0.114 [+0.091, +0.140] | -0.133 [-0.165, -0.100] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.247 [-0.267, -0.228] | 0 |
| psro02 | +0.204 [+0.174, +0.236] | -0.043 [-0.080, -0.008] | 0 |
| psro03 | +0.249 [+0.221, +0.277] | +0.002 [-0.013, +0.016] | 0 |
| psro05 | +0.245 [+0.217, +0.273] | -0.003 [-0.029, +0.024] | 0 |
| iter01/sample * | +0.098 [+0.074, +0.122] | -0.150 [-0.178, -0.122] | 0 |
| iter01/guarded * | +0.130 [+0.106, +0.155] | -0.117 [-0.149, -0.086] | 0 |
| iter04/sample * | +0.273 [+0.241, +0.304] | +0.026 [-0.011, +0.061] | 0 |
| iter04/guarded * | +0.260 [+0.229, +0.290] | +0.013 [-0.023, +0.048] | 0 |
| iter06/sample * | +0.248 [+0.218, +0.278] | +0.001 [-0.036, +0.039] | 0 |
| iter06/guarded * | +0.262 [+0.229, +0.296] | +0.014 [-0.021, +0.052] | 0 |
| iter07/sample * | +0.262 [+0.232, +0.292] | +0.014 [-0.022, +0.050] | 0 |
| iter07/guarded * | +0.266 [+0.233, +0.300] | +0.019 [-0.020, +0.059] | 0 |

Gain by turn position (candidates; position 0 moves first): iter01/sample: -0.172, -0.141, -0.138, -0.148; iter01/guarded: -0.122, -0.128, -0.098, -0.122; iter04/sample: -0.025, +0.054, +0.039, +0.035; iter04/guarded: +0.012, +0.049, +0.025, -0.035; iter06/sample: +0.002, +0.019, -0.021, +0.005; iter06/guarded: +0.088, -0.038, +0.005, +0.002; iter07/sample: +0.022, +0.069, -0.025, -0.008; iter07/guarded: -0.032, +0.055, +0.005, +0.045

NashConv (sum of the seats' gains): iter01/sample: -0.599 [-0.714, -0.489]; iter01/guarded: -0.469 [-0.598, -0.343]; iter04/sample: +0.103 [-0.045, +0.246]; iter04/guarded: +0.051 [-0.093, +0.192]; iter06/sample: +0.004 [-0.145, +0.155]; iter06/guarded: +0.057 [-0.086, +0.209]; iter07/sample: +0.057 [-0.088, +0.200]; iter07/guarded: +0.074 [-0.080, +0.235]

## 5 players

Value of sigma: +0.200 [+0.200, +0.200]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.060 [+0.044, +0.077] | -0.140 [-0.156, -0.123] | 0 |
| magnate | +0.131 [+0.109, +0.154] | -0.069 [-0.091, -0.046] | 0 |
| control | +0.095 [+0.076, +0.114] | -0.105 [-0.124, -0.086] | 0 |
| defense | +0.104 [+0.084, +0.125] | -0.096 [-0.116, -0.075] | 0 |
| exploit_run2 | +0.085 [+0.066, +0.105] | -0.115 [-0.134, -0.095] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.200 [-0.200, -0.200] | 0 |
| psro02 | +0.205 [+0.177, +0.236] | +0.005 [-0.023, +0.036] | 0 |
| psro03 | +0.200 [+0.200, +0.200] | +0.000 [+0.000, +0.000] | 0 |
| psro05 | +0.207 [+0.181, +0.234] | +0.007 [-0.019, +0.034] | 0 |
| iter01/sample * | +0.064 [+0.049, +0.081] | -0.136 [-0.151, -0.119] | 0 |
| iter01/guarded * | +0.107 [+0.087, +0.129] | -0.093 [-0.113, -0.071] | 0 |
| iter04/sample * | +0.193 [+0.164, +0.223] | -0.007 [-0.036, +0.023] | 0 |
| iter04/guarded * | +0.181 [+0.157, +0.205] | -0.019 [-0.043, +0.005] | 0 |
| iter06/sample * | +0.206 [+0.179, +0.234] | +0.006 [-0.021, +0.034] | 0 |
| iter06/guarded * | +0.215 [+0.187, +0.245] | +0.015 [-0.013, +0.045] | 0 |
| iter07/sample * | +0.208 [+0.180, +0.237] | +0.008 [-0.020, +0.037] | 0 |
| iter07/guarded * | +0.229 [+0.199, +0.259] | +0.029 [-0.001, +0.059] | 0 |

Gain by turn position (candidates; position 0 moves first): iter01/sample: -0.243, -0.107, -0.093, -0.177, -0.060; iter01/guarded: -0.157, -0.073, -0.060, -0.127, -0.047; iter04/sample: -0.077, +0.017, +0.050, -0.060, +0.033; iter04/guarded: -0.053, +0.000, -0.010, -0.100, +0.067; iter06/sample: -0.050, -0.013, +0.057, -0.010, +0.047; iter06/guarded: +0.033, +0.053, +0.017, -0.057, +0.030; iter07/sample: -0.037, +0.040, +0.020, -0.053, +0.070; iter07/guarded: +0.023, +0.057, +0.057, -0.047, +0.057

NashConv (sum of the seats' gains): iter01/sample: -0.680 [-0.757, -0.597]; iter01/guarded: -0.463 [-0.567, -0.353]; iter04/sample: -0.037 [-0.180, +0.117]; iter04/guarded: -0.097 [-0.217, +0.027]; iter06/sample: +0.030 [-0.107, +0.170]; iter06/guarded: +0.077 [-0.067, +0.223]; iter07/sample: +0.040 [-0.100, +0.183]; iter07/guarded: +0.147 [-0.003, +0.297]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| run2 | -0.152 [-0.169, -0.136] | -0.616 [-0.675, -0.556] |
| magnate | -0.101 [-0.119, -0.082] | -0.396 [-0.465, -0.328] |
| control | -0.132 [-0.150, -0.114] | -0.523 [-0.586, -0.456] |
| defense | -0.132 [-0.150, -0.113] | -0.510 [-0.577, -0.440] |
| exploit_run2 | -0.128 [-0.145, -0.110] | -0.505 [-0.568, -0.438] |
| merchant | -0.263 [-0.273, -0.254] | -1.005 [-1.039, -0.971] |
| psro02 | -0.024 [-0.044, -0.003] | -0.084 [-0.160, -0.004] |
| psro03 | -0.012 [-0.026, +0.002] | -0.034 [-0.081, +0.010] |
| psro05 | +0.005 [-0.009, +0.018] | +0.018 [-0.043, +0.077] |
| iter01/sample * | -0.156 [-0.171, -0.140] | -0.610 [-0.665, -0.548] |
| iter01/guarded * | -0.117 [-0.136, -0.098] | -0.451 [-0.519, -0.381] |
| iter04/sample * | +0.005 [-0.016, +0.026] | +0.018 [-0.061, +0.099] |
| iter04/guarded * | +0.013 [-0.009, +0.033] | +0.029 [-0.052, +0.106] |
| iter06/sample * | +0.009 [-0.014, +0.032] | +0.032 [-0.054, +0.118] |
| iter06/guarded * | +0.015 [-0.005, +0.036] | +0.059 [-0.015, +0.141] |
| iter07/sample * | +0.008 [-0.014, +0.029] | +0.035 [-0.049, +0.114] |
| iter07/guarded * | +0.027 [+0.005, +0.049] | +0.106 [+0.022, +0.192] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of iter01/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.086 | 0.044 |
| 10 | ±0.061 | 0.031 |
| 25 | ±0.039 | 0.018 |
| 50 | ±0.027 | 0.011 |
| 100 | ±0.019 | 0.005 |
| 150 | ±0.016 | 0.000 |

## How well the gain of iter01/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.103 | 0.052 |
| 10 | ±0.072 | 0.037 |
| 25 | ±0.046 | 0.021 |
| 50 | ±0.032 | 0.014 |
| 100 | ±0.023 | 0.007 |
| 150 | ±0.019 | 0.000 |

## How well the gain of iter04/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.118 | 0.055 |
| 10 | ±0.083 | 0.041 |
| 25 | ±0.053 | 0.026 |
| 50 | ±0.037 | 0.016 |
| 100 | ±0.026 | 0.008 |
| 150 | ±0.022 | 0.000 |

## How well the gain of iter04/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.117 | 0.058 |
| 10 | ±0.082 | 0.042 |
| 25 | ±0.052 | 0.026 |
| 50 | ±0.037 | 0.015 |
| 100 | ±0.026 | 0.008 |
| 150 | ±0.021 | 0.000 |

## How well the gain of iter06/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.123 | 0.057 |
| 10 | ±0.087 | 0.042 |
| 25 | ±0.055 | 0.025 |
| 50 | ±0.039 | 0.016 |
| 100 | ±0.028 | 0.008 |
| 150 | ±0.022 | 0.000 |

## How well the gain of iter06/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.117 | 0.059 |
| 10 | ±0.083 | 0.041 |
| 25 | ±0.052 | 0.024 |
| 50 | ±0.037 | 0.015 |
| 100 | ±0.026 | 0.008 |
| 150 | ±0.021 | 0.000 |

## How well the gain of iter07/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.120 | 0.062 |
| 10 | ±0.085 | 0.041 |
| 25 | ±0.054 | 0.025 |
| 50 | ±0.038 | 0.015 |
| 100 | ±0.027 | 0.008 |
| 150 | ±0.022 | 0.000 |

## How well the gain of iter07/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.120 | 0.065 |
| 10 | ±0.085 | 0.040 |
| 25 | ±0.054 | 0.025 |
| 50 | ±0.038 | 0.016 |
| 100 | ±0.027 | 0.008 |
| 150 | ±0.022 | 0.000 |

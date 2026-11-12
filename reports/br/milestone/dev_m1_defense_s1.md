# Milestone, dev study: m1_defense_s1 (init defense_run2_vs_exploiter_30m, profile run2,magnate,exploit_run2)

Profile sigma: run2 0.33, magnate 0.33, exploit_run2 0.33. Every other seat draws its strategy iid from sigma. Seeds: 60 per player count from the **dev** block (offset 100), 3 player counts, every seat of every table, standard length; 12,240 games, 1 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-6a10469a5eb3.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.378 [+0.337, +0.422]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.378 [-0.422, -0.337] | 0 |
| run1 | +0.181 [+0.122, +0.244] | -0.197 [-0.268, -0.127] | 0 |
| run2 | +0.294 [+0.233, +0.361] | -0.083 [-0.137, -0.026] | 0 |
| exploit_run2 | +0.372 [+0.306, +0.439] | -0.006 [-0.061, +0.048] | 0 |
| control | +0.328 [+0.258, +0.397] | -0.050 [-0.130, +0.027] | 0 |
| defense | +0.292 [+0.233, +0.358] | -0.086 [-0.157, -0.015] | 0 |
| magnate | +0.467 [+0.394, +0.539] | +0.089 [+0.033, +0.144] | 0 |
| m1_defense_s1@5m/sample * | +0.347 [+0.283, +0.411] | -0.031 [-0.107, +0.046] | 0 |
| m1_defense_s1@5m/guarded * | +0.328 [+0.267, +0.394] | -0.050 [-0.128, +0.026] | 0 |
| m1_defense_s1@11m/sample * | +0.356 [+0.297, +0.417] | -0.022 [-0.098, +0.055] | 0 |
| m1_defense_s1@11m/guarded * | +0.403 [+0.336, +0.472] | +0.025 [-0.044, +0.096] | 0 |
| m1_defense_s1@16m/sample * | +0.322 [+0.261, +0.389] | -0.056 [-0.124, +0.013] | 0 |
| m1_defense_s1@16m/guarded * | +0.325 [+0.258, +0.383] | -0.053 [-0.130, +0.021] | 0 |
| m1_defense_s1@21m/sample * | +0.386 [+0.314, +0.456] | +0.008 [-0.075, +0.086] | 1 |
| m1_defense_s1@21m/guarded * | +0.394 [+0.319, +0.469] | +0.017 [-0.066, +0.104] | 0 |
| m1_defense_s1/sample * | +0.314 [+0.244, +0.389] | -0.064 [-0.142, +0.012] | 0 |
| m1_defense_s1/guarded * | +0.375 [+0.306, +0.447] | -0.003 [-0.081, +0.070] | 0 |

Gain by turn position (candidates; position 0 moves first): m1_defense_s1@5m/sample: +0.042, -0.006, -0.128; m1_defense_s1@5m/guarded: +0.067, -0.022, -0.194; m1_defense_s1@11m/sample: -0.058, +0.061, -0.069; m1_defense_s1@11m/guarded: +0.067, +0.069, -0.061; m1_defense_s1@16m/sample: -0.033, -0.122, -0.011; m1_defense_s1@16m/guarded: -0.017, -0.072, -0.069; m1_defense_s1@21m/sample: -0.008, +0.028, +0.006; m1_defense_s1@21m/guarded: +0.075, -0.072, +0.047; m1_defense_s1/sample: -0.050, +0.003, -0.144; m1_defense_s1/guarded: +0.058, +0.003, -0.069

NashConv (sum of the seats' gains): m1_defense_s1@5m/sample: -0.092 [-0.322, +0.139]; m1_defense_s1@5m/guarded: -0.150 [-0.383, +0.078]; m1_defense_s1@11m/sample: -0.067 [-0.294, +0.164]; m1_defense_s1@11m/guarded: +0.075 [-0.133, +0.289]; m1_defense_s1@16m/sample: -0.167 [-0.372, +0.039]; m1_defense_s1@16m/guarded: -0.158 [-0.389, +0.064]; m1_defense_s1@21m/sample: +0.025 [-0.225, +0.258]; m1_defense_s1@21m/guarded: +0.050 [-0.197, +0.311]; m1_defense_s1/sample: -0.192 [-0.425, +0.036]; m1_defense_s1/guarded: -0.008 [-0.242, +0.211]

## 4 players

Value of sigma: +0.243 [+0.210, +0.277]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.243 [-0.277, -0.210] | 0 |
| run1 | +0.071 [+0.042, +0.104] | -0.172 [-0.219, -0.128] | 0 |
| run2 | +0.154 [+0.110, +0.202] | -0.089 [-0.129, -0.044] | 0 |
| exploit_run2 | +0.219 [+0.167, +0.275] | -0.024 [-0.067, +0.021] | 0 |
| control | +0.306 [+0.250, +0.358] | +0.063 [+0.001, +0.117] | 0 |
| defense | +0.200 [+0.154, +0.250] | -0.043 [-0.105, +0.021] | 0 |
| magnate | +0.356 [+0.292, +0.425] | +0.113 [+0.066, +0.163] | 0 |
| m1_defense_s1@5m/sample * | +0.300 [+0.250, +0.350] | +0.057 [-0.003, +0.116] | 0 |
| m1_defense_s1@5m/guarded * | +0.273 [+0.217, +0.325] | +0.030 [-0.026, +0.085] | 0 |
| m1_defense_s1@11m/sample * | +0.287 [+0.237, +0.333] | +0.044 [-0.005, +0.094] | 0 |
| m1_defense_s1@11m/guarded * | +0.325 [+0.275, +0.379] | +0.082 [+0.019, +0.146] | 0 |
| m1_defense_s1@16m/sample * | +0.327 [+0.265, +0.388] | +0.084 [+0.013, +0.150] | 0 |
| m1_defense_s1@16m/guarded * | +0.333 [+0.281, +0.390] | +0.090 [+0.026, +0.160] | 0 |
| m1_defense_s1@21m/sample * | +0.304 [+0.242, +0.367] | +0.061 [-0.005, +0.122] | 0 |
| m1_defense_s1@21m/guarded * | +0.342 [+0.287, +0.396] | +0.099 [+0.033, +0.164] | 0 |
| m1_defense_s1/sample * | +0.271 [+0.229, +0.317] | +0.028 [-0.020, +0.079] | 0 |
| m1_defense_s1/guarded * | +0.377 [+0.308, +0.442] | +0.134 [+0.051, +0.213] | 0 |

Gain by turn position (candidates; position 0 moves first): m1_defense_s1@5m/sample: +0.078, +0.053, +0.072, +0.025; m1_defense_s1@5m/guarded: +0.053, +0.003, +0.106, -0.042; m1_defense_s1@11m/sample: +0.094, +0.036, +0.089, -0.042; m1_defense_s1@11m/guarded: +0.128, +0.103, +0.139, -0.042; m1_defense_s1@16m/sample: +0.144, +0.036, +0.197, -0.042; m1_defense_s1@16m/guarded: +0.128, +0.053, +0.114, +0.067; m1_defense_s1@21m/sample: +0.119, +0.019, +0.164, -0.058; m1_defense_s1@21m/guarded: +0.228, +0.069, +0.089, +0.008; m1_defense_s1/sample: +0.078, -0.014, +0.056, -0.008; m1_defense_s1/guarded: +0.161, +0.069, +0.181, +0.125

NashConv (sum of the seats' gains): m1_defense_s1@5m/sample: +0.228 [-0.014, +0.464]; m1_defense_s1@5m/guarded: +0.119 [-0.106, +0.342]; m1_defense_s1@11m/sample: +0.178 [-0.019, +0.375]; m1_defense_s1@11m/guarded: +0.328 [+0.078, +0.583]; m1_defense_s1@16m/sample: +0.336 [+0.050, +0.600]; m1_defense_s1@16m/guarded: +0.361 [+0.106, +0.639]; m1_defense_s1@21m/sample: +0.244 [-0.019, +0.489]; m1_defense_s1@21m/guarded: +0.394 [+0.130, +0.656]; m1_defense_s1/sample: +0.111 [-0.081, +0.317]; m1_defense_s1/guarded: +0.536 [+0.203, +0.853]

## 5 players

Value of sigma: +0.212 [+0.185, +0.236]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.212 [-0.236, -0.185] | 0 |
| run1 | +0.028 [+0.010, +0.052] | -0.183 [-0.217, -0.146] | 0 |
| run2 | +0.173 [+0.133, +0.210] | -0.038 [-0.074, -0.004] | 0 |
| exploit_run2 | +0.197 [+0.150, +0.243] | -0.015 [-0.053, +0.024] | 0 |
| control | +0.272 [+0.223, +0.322] | +0.060 [+0.008, +0.111] | 0 |
| defense | +0.212 [+0.162, +0.265] | +0.000 [-0.059, +0.061] | 0 |
| magnate | +0.265 [+0.213, +0.313] | +0.053 [+0.014, +0.091] | 0 |
| m1_defense_s1@5m/sample * | +0.250 [+0.203, +0.297] | +0.038 [-0.014, +0.093] | 0 |
| m1_defense_s1@5m/guarded * | +0.245 [+0.197, +0.295] | +0.033 [-0.018, +0.086] | 0 |
| m1_defense_s1@11m/sample * | +0.283 [+0.233, +0.333] | +0.072 [+0.017, +0.128] | 0 |
| m1_defense_s1@11m/guarded * | +0.267 [+0.218, +0.315] | +0.055 [+0.000, +0.109] | 0 |
| m1_defense_s1@16m/sample * | +0.264 [+0.218, +0.310] | +0.053 [+0.010, +0.096] | 0 |
| m1_defense_s1@16m/guarded * | +0.258 [+0.203, +0.313] | +0.047 [-0.011, +0.107] | 0 |
| m1_defense_s1@21m/sample * | +0.263 [+0.217, +0.312] | +0.052 [-0.005, +0.110] | 0 |
| m1_defense_s1@21m/guarded * | +0.283 [+0.237, +0.328] | +0.072 [+0.020, +0.122] | 0 |
| m1_defense_s1/sample * | +0.243 [+0.198, +0.285] | +0.032 [-0.019, +0.078] | 0 |
| m1_defense_s1/guarded * | +0.288 [+0.240, +0.342] | +0.077 [+0.025, +0.131] | 0 |

Gain by turn position (candidates; position 0 moves first): m1_defense_s1@5m/sample: -0.044, +0.086, +0.061, +0.067, +0.022; m1_defense_s1@5m/guarded: +0.014, +0.053, +0.094, +0.000, +0.006; m1_defense_s1@11m/sample: +0.014, +0.069, +0.161, +0.008, +0.106; m1_defense_s1@11m/guarded: +0.031, +0.144, +0.019, -0.008, +0.089; m1_defense_s1@16m/sample: +0.022, +0.036, +0.094, +0.056, +0.056; m1_defense_s1@16m/guarded: -0.061, +0.078, +0.078, +0.067, +0.072; m1_defense_s1@21m/sample: +0.047, +0.069, +0.111, -0.008, +0.039; m1_defense_s1@21m/guarded: +0.056, +0.044, +0.194, +0.092, -0.028; m1_defense_s1/sample: -0.061, +0.028, +0.128, +0.000, +0.064; m1_defense_s1/guarded: +0.047, +0.169, +0.128, -0.017, +0.056

NashConv (sum of the seats' gains): m1_defense_s1@5m/sample: +0.192 [-0.072, +0.467]; m1_defense_s1@5m/guarded: +0.167 [-0.089, +0.431]; m1_defense_s1@11m/sample: +0.358 [+0.086, +0.642]; m1_defense_s1@11m/guarded: +0.275 [+0.000, +0.545]; m1_defense_s1@16m/sample: +0.264 [+0.050, +0.481]; m1_defense_s1@16m/guarded: +0.233 [-0.056, +0.536]; m1_defense_s1@21m/sample: +0.258 [-0.025, +0.550]; m1_defense_s1@21m/guarded: +0.358 [+0.100, +0.611]; m1_defense_s1/sample: +0.158 [-0.097, +0.392]; m1_defense_s1/guarded: +0.383 [+0.125, +0.653]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| merchant | -0.277 [-0.297, -0.258] | -1.055 [-1.131, -0.982] |
| run1 | -0.184 [-0.216, -0.155] | -0.732 [-0.844, -0.625] |
| run2 | -0.070 [-0.095, -0.044] | -0.266 [-0.361, -0.171] |
| exploit_run2 | -0.015 [-0.041, +0.012] | -0.063 [-0.167, +0.038] |
| control | +0.024 [-0.012, +0.062] | +0.134 [-0.004, +0.280] |
| defense | -0.043 [-0.079, -0.007] | -0.144 [-0.285, -0.001] |
| magnate | +0.085 [+0.057, +0.111] | +0.329 [+0.221, +0.433] |
| m1_defense_s1@5m/sample * | +0.022 [-0.015, +0.057] | +0.109 [-0.032, +0.248] |
| m1_defense_s1@5m/guarded * | +0.004 [-0.032, +0.041] | +0.045 [-0.090, +0.194] |
| m1_defense_s1@11m/sample * | +0.031 [-0.005, +0.068] | +0.156 [+0.018, +0.296] |
| m1_defense_s1@11m/guarded * | +0.054 [+0.019, +0.091] | +0.226 [+0.089, +0.368] |
| m1_defense_s1@16m/sample * | +0.027 [-0.008, +0.063] | +0.144 [+0.013, +0.281] |
| m1_defense_s1@16m/guarded * | +0.028 [-0.012, +0.065] | +0.145 [-0.008, +0.291] |
| m1_defense_s1@21m/sample * | +0.040 [+0.001, +0.081] | +0.176 [+0.023, +0.333] |
| m1_defense_s1@21m/guarded * | +0.062 [+0.024, +0.100] | +0.268 [+0.122, +0.408] |
| m1_defense_s1/sample * | -0.001 [-0.036, +0.033] | +0.026 [-0.103, +0.152] |
| m1_defense_s1/guarded * | +0.069 [+0.028, +0.111] | +0.304 [+0.148, +0.469] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of m1_defense_s1/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 60 seeds (it understates near k = 60).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.121 | 0.059 |
| 10 | ±0.085 | 0.038 |
| 25 | ±0.054 | 0.021 |
| 50 | ±0.038 | 0.008 |
| 60 | ±0.035 | 0.000 |

## How well the gain of m1_defense_s1/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 60 seeds (it understates near k = 60).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.143 | 0.068 |
| 10 | ±0.101 | 0.049 |
| 25 | ±0.064 | 0.025 |
| 50 | ±0.045 | 0.009 |
| 60 | ±0.041 | 0.000 |

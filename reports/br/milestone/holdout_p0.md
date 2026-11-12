# Milestone, holdout study: opponents drawn from run2,magnate,exploit_run2 with equal weights

Profile sigma: run2 0.33, magnate 0.33, exploit_run2 0.33. Every other seat draws its strategy iid from sigma. Seeds: 150 per player count from the **holdout** block (offset 0), 3 player counts, every seat of every table, standard length; 23,400 games, 2 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-e1926b48b863.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.326 [+0.304, +0.347]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.326 [-0.347, -0.304] | 0 |
| run1 | +0.156 [+0.122, +0.188] | -0.170 [-0.209, -0.132] | 0 |
| run2 | +0.262 [+0.223, +0.303] | -0.063 [-0.098, -0.029] | 0 |
| exploit_run2 | +0.351 [+0.314, +0.391] | +0.026 [-0.005, +0.060] | 0 |
| control | +0.378 [+0.336, +0.419] | +0.052 [+0.004, +0.100] | 0 |
| defense | +0.331 [+0.291, +0.373] | +0.006 [-0.041, +0.052] | 0 |
| magnate | +0.363 [+0.321, +0.406] | +0.038 [+0.001, +0.072] | 0 |
| m1_defense_s1/sample * | +0.384 [+0.341, +0.429] | +0.059 [+0.010, +0.104] | 1 |
| m1_defense_s1/guarded * | +0.411 [+0.373, +0.452] | +0.086 [+0.037, +0.134] | 0 |
| m2_defense_s2/sample * | +0.384 [+0.342, +0.430] | +0.059 [+0.014, +0.109] | 1 |
| m2_defense_s2/guarded * | +0.401 [+0.359, +0.444] | +0.076 [+0.030, +0.121] | 0 |
| m3_run1_s3/sample * | +0.194 [+0.163, +0.230] | -0.131 [-0.167, -0.097] | 0 |
| m3_run1_s3/guarded * | +0.183 [+0.151, +0.218] | -0.142 [-0.182, -0.100] | 0 |

Gain by turn position (candidates; position 0 moves first): m1_defense_s1/sample: +0.047, +0.094, +0.036; m1_defense_s1/guarded: +0.083, +0.074, +0.099; m2_defense_s2/sample: +0.027, +0.081, +0.069; m2_defense_s2/guarded: +0.067, +0.064, +0.096; m3_run1_s3/sample: -0.113, -0.132, -0.148; m3_run1_s3/guarded: -0.130, -0.112, -0.184

NashConv (sum of the seats' gains): m1_defense_s1/sample: +0.177 [+0.031, +0.313]; m1_defense_s1/guarded: +0.257 [+0.112, +0.402]; m2_defense_s2/sample: +0.177 [+0.042, +0.328]; m2_defense_s2/guarded: +0.227 [+0.089, +0.363]; m3_run1_s3/sample: -0.393 [-0.501, -0.290]; m3_run1_s3/guarded: -0.427 [-0.546, -0.300]

## 4 players

Value of sigma: +0.255 [+0.235, +0.276]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.255 [-0.276, -0.235] | 0 |
| run1 | +0.048 [+0.031, +0.066] | -0.207 [-0.234, -0.182] | 0 |
| run2 | +0.205 [+0.172, +0.238] | -0.050 [-0.077, -0.024] | 0 |
| exploit_run2 | +0.264 [+0.233, +0.301] | +0.009 [-0.017, +0.036] | 0 |
| control | +0.263 [+0.231, +0.296] | +0.008 [-0.029, +0.046] | 0 |
| defense | +0.236 [+0.204, +0.268] | -0.019 [-0.055, +0.017] | 0 |
| magnate | +0.296 [+0.261, +0.332] | +0.041 [+0.013, +0.069] | 0 |
| m1_defense_s1/sample * | +0.279 [+0.246, +0.317] | +0.024 [-0.015, +0.067] | 0 |
| m1_defense_s1/guarded * | +0.341 [+0.305, +0.377] | +0.086 [+0.046, +0.126] | 0 |
| m2_defense_s2/sample * | +0.247 [+0.212, +0.282] | -0.008 [-0.048, +0.031] | 0 |
| m2_defense_s2/guarded * | +0.286 [+0.250, +0.322] | +0.031 [-0.008, +0.073] | 0 |
| m3_run1_s3/sample * | +0.156 [+0.126, +0.186] | -0.099 [-0.136, -0.064] | 0 |
| m3_run1_s3/guarded * | +0.189 [+0.158, +0.221] | -0.066 [-0.103, -0.028] | 0 |

Gain by turn position (candidates; position 0 moves first): m1_defense_s1/sample: +0.112, +0.039, -0.029, -0.026; m1_defense_s1/guarded: +0.192, +0.069, +0.094, -0.012; m2_defense_s2/sample: +0.066, -0.031, -0.036, -0.032; m2_defense_s2/guarded: +0.006, +0.052, +0.038, +0.028; m3_run1_s3/sample: -0.031, -0.094, -0.092, -0.179; m3_run1_s3/guarded: -0.031, -0.051, -0.049, -0.132

NashConv (sum of the seats' gains): m1_defense_s1/sample: +0.097 [-0.059, +0.267]; m1_defense_s1/guarded: +0.343 [+0.182, +0.506]; m2_defense_s2/sample: -0.033 [-0.191, +0.124]; m2_defense_s2/guarded: +0.123 [-0.032, +0.292]; m3_run1_s3/sample: -0.397 [-0.542, -0.254]; m3_run1_s3/guarded: -0.263 [-0.411, -0.112]

## 5 players

Value of sigma: +0.203 [+0.186, +0.221]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.203 [-0.221, -0.186] | 0 |
| run1 | +0.025 [+0.013, +0.037] | -0.177 [-0.198, -0.156] | 0 |
| run2 | +0.151 [+0.126, +0.179] | -0.051 [-0.073, -0.028] | 0 |
| exploit_run2 | +0.187 [+0.159, +0.217] | -0.015 [-0.037, +0.007] | 0 |
| control | +0.205 [+0.179, +0.230] | +0.002 [-0.029, +0.033] | 0 |
| defense | +0.187 [+0.161, +0.215] | -0.016 [-0.048, +0.015] | 0 |
| magnate | +0.269 [+0.241, +0.299] | +0.067 [+0.044, +0.089] | 0 |
| m1_defense_s1/sample * | +0.248 [+0.220, +0.278] | +0.045 [+0.011, +0.079] | 0 |
| m1_defense_s1/guarded * | +0.303 [+0.269, +0.337] | +0.101 [+0.065, +0.140] | 0 |
| m2_defense_s2/sample * | +0.245 [+0.217, +0.274] | +0.042 [+0.011, +0.077] | 0 |
| m2_defense_s2/guarded * | +0.287 [+0.255, +0.320] | +0.085 [+0.050, +0.120] | 0 |
| m3_run1_s3/sample * | +0.107 [+0.087, +0.128] | -0.095 [-0.122, -0.071] | 0 |
| m3_run1_s3/guarded * | +0.174 [+0.147, +0.199] | -0.029 [-0.060, +0.000] | 0 |

Gain by turn position (candidates; position 0 moves first): m1_defense_s1/sample: +0.119, +0.089, -0.006, -0.017, +0.041; m1_defense_s1/guarded: +0.116, +0.106, +0.114, +0.097, +0.071; m2_defense_s2/sample: +0.106, +0.056, +0.048, -0.033, +0.034; m2_defense_s2/guarded: +0.089, +0.109, +0.068, +0.070, +0.088; m3_run1_s3/sample: -0.131, -0.064, -0.099, -0.130, -0.052; m3_run1_s3/guarded: +0.002, -0.011, +0.011, -0.103, -0.042

NashConv (sum of the seats' gains): m1_defense_s1/sample: +0.227 [+0.053, +0.396]; m1_defense_s1/guarded: +0.503 [+0.323, +0.701]; m2_defense_s2/sample: +0.210 [+0.053, +0.386]; m2_defense_s2/guarded: +0.423 [+0.250, +0.599]; m3_run1_s3/sample: -0.477 [-0.608, -0.353]; m3_run1_s3/guarded: -0.143 [-0.300, +0.000]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| merchant | -0.261 [-0.274, -0.250] | -1.003 [-1.054, -0.959] |
| run1 | -0.185 [-0.202, -0.168] | -0.742 [-0.806, -0.680] |
| run2 | -0.055 [-0.072, -0.038] | -0.216 [-0.279, -0.153] |
| exploit_run2 | +0.006 [-0.009, +0.024] | +0.012 [-0.049, +0.076] |
| control | +0.021 [-0.003, +0.044] | +0.067 [-0.026, +0.153] |
| defense | -0.010 [-0.032, +0.013] | -0.047 [-0.132, +0.038] |
| magnate | +0.048 [+0.031, +0.066] | +0.203 [+0.137, +0.268] |
| m1_defense_s1/sample * | +0.043 [+0.018, +0.067] | +0.167 [+0.072, +0.259] |
| m1_defense_s1/guarded * | +0.091 [+0.067, +0.115] | +0.368 [+0.274, +0.464] |
| m2_defense_s2/sample * | +0.031 [+0.009, +0.055] | +0.118 [+0.031, +0.214] |
| m2_defense_s2/guarded * | +0.064 [+0.041, +0.089] | +0.258 [+0.165, +0.359] |
| m3_run1_s3/sample * | -0.109 [-0.128, -0.089] | -0.422 [-0.499, -0.349] |
| m3_run1_s3/guarded * | -0.079 [-0.098, -0.058] | -0.278 [-0.352, -0.196] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of m1_defense_s1/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.129 | 0.063 |
| 10 | ±0.091 | 0.046 |
| 25 | ±0.058 | 0.028 |
| 50 | ±0.041 | 0.016 |
| 100 | ±0.029 | 0.008 |
| 150 | ±0.024 | 0.000 |

## How well the gain of m1_defense_s1/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.135 | 0.067 |
| 10 | ±0.096 | 0.047 |
| 25 | ±0.060 | 0.028 |
| 50 | ±0.043 | 0.018 |
| 100 | ±0.030 | 0.008 |
| 150 | ±0.025 | 0.000 |

## How well the gain of m2_defense_s2/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.132 | 0.065 |
| 10 | ±0.093 | 0.047 |
| 25 | ±0.059 | 0.029 |
| 50 | ±0.042 | 0.017 |
| 100 | ±0.029 | 0.009 |
| 150 | ±0.024 | 0.000 |

## How well the gain of m2_defense_s2/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.130 | 0.061 |
| 10 | ±0.092 | 0.043 |
| 25 | ±0.058 | 0.026 |
| 50 | ±0.041 | 0.018 |
| 100 | ±0.029 | 0.009 |
| 150 | ±0.024 | 0.000 |

## How well the gain of m3_run1_s3/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.105 | 0.052 |
| 10 | ±0.074 | 0.037 |
| 25 | ±0.047 | 0.022 |
| 50 | ±0.033 | 0.015 |
| 100 | ±0.023 | 0.007 |
| 150 | ±0.019 | 0.000 |

## How well the gain of m3_run1_s3/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.115 | 0.060 |
| 10 | ±0.081 | 0.043 |
| 25 | ±0.051 | 0.024 |
| 50 | ±0.036 | 0.015 |
| 100 | ±0.026 | 0.008 |
| 150 | ±0.021 | 0.000 |

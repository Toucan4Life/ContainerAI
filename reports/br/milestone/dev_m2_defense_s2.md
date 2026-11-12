# Milestone, dev study: m2_defense_s2 (init defense_run2_vs_exploiter_30m, profile run2,magnate,exploit_run2)

Profile sigma: run2 0.33, magnate 0.33, exploit_run2 0.33. Every other seat draws its strategy iid from sigma. Seeds: 60 per player count from the **dev** block (offset 100), 3 player counts, every seat of every table, standard length; 12,240 games, 0 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-31323b35d33a.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.331 [+0.299, +0.367]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.331 [-0.367, -0.299] | 0 |
| run1 | +0.114 [+0.072, +0.161] | -0.218 [-0.271, -0.159] | 0 |
| run2 | +0.219 [+0.156, +0.281] | -0.112 [-0.177, -0.049] | 0 |
| exploit_run2 | +0.383 [+0.317, +0.447] | +0.052 [+0.001, +0.101] | 0 |
| control | +0.322 [+0.250, +0.389] | -0.009 [-0.087, +0.069] | 0 |
| defense | +0.286 [+0.222, +0.350] | -0.045 [-0.119, +0.025] | 0 |
| magnate | +0.392 [+0.330, +0.461] | +0.060 [+0.013, +0.113] | 0 |
| m2_defense_s2@5m/sample * | +0.356 [+0.289, +0.422] | +0.024 [-0.049, +0.096] | 0 |
| m2_defense_s2@5m/guarded * | +0.378 [+0.306, +0.447] | +0.046 [-0.029, +0.120] | 0 |
| m2_defense_s2@10m/sample * | +0.361 [+0.297, +0.425] | +0.030 [-0.049, +0.103] | 0 |
| m2_defense_s2@10m/guarded * | +0.425 [+0.353, +0.494] | +0.094 [+0.017, +0.169] | 0 |
| m2_defense_s2@16m/sample * | +0.347 [+0.286, +0.406] | +0.016 [-0.052, +0.083] | 0 |
| m2_defense_s2@16m/guarded * | +0.361 [+0.294, +0.425] | +0.030 [-0.049, +0.108] | 0 |
| m2_defense_s2@21m/sample * | +0.311 [+0.253, +0.369] | -0.020 [-0.092, +0.053] | 0 |
| m2_defense_s2@21m/guarded * | +0.372 [+0.300, +0.447] | +0.041 [-0.032, +0.119] | 0 |
| m2_defense_s2/sample * | +0.394 [+0.333, +0.461] | +0.063 [-0.004, +0.133] | 0 |
| m2_defense_s2/guarded * | +0.417 [+0.339, +0.497] | +0.085 [+0.004, +0.171] | 0 |

Gain by turn position (candidates; position 0 moves first): m2_defense_s2@5m/sample: +0.036, +0.047, -0.011; m2_defense_s2@5m/guarded: +0.044, +0.064, +0.031; m2_defense_s2@10m/sample: +0.086, -0.061, +0.064; m2_defense_s2@10m/guarded: +0.103, -0.011, +0.189; m2_defense_s2@16m/sample: -0.139, +0.031, +0.156; m2_defense_s2@16m/guarded: -0.097, +0.081, +0.106; m2_defense_s2@21m/sample: -0.006, -0.011, -0.044; m2_defense_s2@21m/guarded: -0.022, +0.081, +0.064; m2_defense_s2/sample: +0.053, +0.114, +0.022; m2_defense_s2/guarded: +0.028, +0.056, +0.172

NashConv (sum of the seats' gains): m2_defense_s2@5m/sample: +0.072 [-0.147, +0.289]; m2_defense_s2@5m/guarded: +0.139 [-0.086, +0.361]; m2_defense_s2@10m/sample: +0.089 [-0.147, +0.308]; m2_defense_s2@10m/guarded: +0.281 [+0.050, +0.508]; m2_defense_s2@16m/sample: +0.047 [-0.156, +0.250]; m2_defense_s2@16m/guarded: +0.089 [-0.147, +0.325]; m2_defense_s2@21m/sample: -0.061 [-0.275, +0.158]; m2_defense_s2@21m/guarded: +0.122 [-0.097, +0.356]; m2_defense_s2/sample: +0.189 [-0.011, +0.400]; m2_defense_s2/guarded: +0.256 [+0.011, +0.514]

## 4 players

Value of sigma: +0.241 [+0.214, +0.269]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.241 [-0.269, -0.214] | 0 |
| run1 | +0.042 [+0.017, +0.067] | -0.199 [-0.235, -0.160] | 0 |
| run2 | +0.148 [+0.106, +0.194] | -0.093 [-0.132, -0.054] | 0 |
| exploit_run2 | +0.300 [+0.246, +0.356] | +0.059 [+0.017, +0.101] | 0 |
| control | +0.275 [+0.221, +0.329] | +0.034 [-0.023, +0.093] | 0 |
| defense | +0.233 [+0.173, +0.294] | -0.008 [-0.078, +0.063] | 0 |
| magnate | +0.275 [+0.225, +0.321] | +0.034 [-0.008, +0.074] | 0 |
| m2_defense_s2@5m/sample * | +0.281 [+0.223, +0.342] | +0.040 [-0.020, +0.104] | 0 |
| m2_defense_s2@5m/guarded * | +0.354 [+0.300, +0.408] | +0.113 [+0.054, +0.174] | 0 |
| m2_defense_s2@10m/sample * | +0.227 [+0.181, +0.275] | -0.014 [-0.066, +0.039] | 0 |
| m2_defense_s2@10m/guarded * | +0.325 [+0.277, +0.373] | +0.084 [+0.033, +0.134] | 0 |
| m2_defense_s2@16m/sample * | +0.285 [+0.227, +0.346] | +0.044 [-0.023, +0.113] | 0 |
| m2_defense_s2@16m/guarded * | +0.331 [+0.269, +0.390] | +0.090 [+0.028, +0.153] | 0 |
| m2_defense_s2@21m/sample * | +0.265 [+0.204, +0.325] | +0.024 [-0.043, +0.092] | 0 |
| m2_defense_s2@21m/guarded * | +0.317 [+0.258, +0.375] | +0.076 [+0.010, +0.141] | 0 |
| m2_defense_s2/sample * | +0.294 [+0.231, +0.358] | +0.053 [-0.011, +0.122] | 0 |
| m2_defense_s2/guarded * | +0.331 [+0.273, +0.390] | +0.090 [+0.024, +0.156] | 0 |

Gain by turn position (candidates; position 0 moves first): m2_defense_s2@5m/sample: +0.053, +0.111, +0.031, -0.033; m2_defense_s2@5m/guarded: +0.119, +0.086, +0.147, +0.100; m2_defense_s2@10m/sample: +0.019, +0.028, +0.064, -0.167; m2_defense_s2@10m/guarded: +0.111, +0.186, +0.089, -0.050; m2_defense_s2@16m/sample: -0.031, +0.136, +0.089, -0.017; m2_defense_s2@16m/guarded: +0.053, +0.094, +0.181, +0.033; m2_defense_s2@21m/sample: +0.036, +0.044, +0.031, -0.017; m2_defense_s2@21m/guarded: +0.203, +0.119, +0.047, -0.067; m2_defense_s2/sample: +0.053, +0.036, +0.089, +0.033; m2_defense_s2/guarded: +0.178, +0.103, -0.003, +0.083

NashConv (sum of the seats' gains): m2_defense_s2@5m/sample: +0.161 [-0.081, +0.417]; m2_defense_s2@5m/guarded: +0.453 [+0.217, +0.695]; m2_defense_s2@10m/sample: -0.056 [-0.264, +0.156]; m2_defense_s2@10m/guarded: +0.336 [+0.131, +0.536]; m2_defense_s2@16m/sample: +0.178 [-0.092, +0.453]; m2_defense_s2@16m/guarded: +0.361 [+0.111, +0.611]; m2_defense_s2@21m/sample: +0.094 [-0.172, +0.367]; m2_defense_s2@21m/guarded: +0.303 [+0.039, +0.564]; m2_defense_s2/sample: +0.211 [-0.045, +0.486]; m2_defense_s2/guarded: +0.361 [+0.097, +0.625]

## 5 players

Value of sigma: +0.213 [+0.189, +0.238]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.213 [-0.238, -0.189] | 0 |
| run1 | +0.042 [+0.023, +0.063] | -0.172 [-0.204, -0.139] | 0 |
| run2 | +0.218 [+0.173, +0.265] | +0.005 [-0.039, +0.049] | 0 |
| exploit_run2 | +0.173 [+0.130, +0.222] | -0.040 [-0.078, -0.001] | 0 |
| control | +0.205 [+0.165, +0.243] | -0.008 [-0.052, +0.036] | 0 |
| defense | +0.165 [+0.122, +0.210] | -0.048 [-0.096, -0.002] | 0 |
| magnate | +0.248 [+0.193, +0.300] | +0.035 [-0.005, +0.074] | 0 |
| m2_defense_s2@5m/sample * | +0.215 [+0.180, +0.253] | +0.002 [-0.042, +0.045] | 0 |
| m2_defense_s2@5m/guarded * | +0.248 [+0.202, +0.295] | +0.035 [-0.014, +0.086] | 0 |
| m2_defense_s2@10m/sample * | +0.223 [+0.182, +0.265] | +0.010 [-0.036, +0.056] | 0 |
| m2_defense_s2@10m/guarded * | +0.252 [+0.203, +0.302] | +0.038 [-0.011, +0.092] | 0 |
| m2_defense_s2@16m/sample * | +0.250 [+0.200, +0.305] | +0.037 [-0.020, +0.095] | 0 |
| m2_defense_s2@16m/guarded * | +0.275 [+0.227, +0.322] | +0.062 [+0.017, +0.112] | 0 |
| m2_defense_s2@21m/sample * | +0.242 [+0.198, +0.287] | +0.028 [-0.018, +0.077] | 0 |
| m2_defense_s2@21m/guarded * | +0.262 [+0.208, +0.313] | +0.048 [-0.008, +0.104] | 0 |
| m2_defense_s2/sample * | +0.262 [+0.217, +0.310] | +0.048 [+0.000, +0.101] | 0 |
| m2_defense_s2/guarded * | +0.332 [+0.277, +0.393] | +0.118 [+0.064, +0.181] | 0 |

Gain by turn position (candidates; position 0 moves first): m2_defense_s2@5m/sample: -0.061, -0.006, +0.083, +0.028, -0.036; m2_defense_s2@5m/guarded: +0.064, -0.031, -0.017, +0.128, +0.031; m2_defense_s2@10m/sample: +0.039, -0.014, +0.025, +0.053, -0.053; m2_defense_s2@10m/guarded: +0.039, +0.003, +0.108, +0.028, +0.014; m2_defense_s2@16m/sample: -0.028, -0.006, +0.100, +0.119, -0.003; m2_defense_s2@16m/guarded: +0.106, +0.053, +0.150, +0.036, -0.036; m2_defense_s2@21m/sample: +0.122, +0.011, +0.067, -0.022, -0.036; m2_defense_s2@21m/guarded: +0.039, -0.006, +0.033, +0.044, +0.131; m2_defense_s2/sample: -0.011, +0.103, +0.100, +0.086, -0.036; m2_defense_s2/guarded: +0.072, +0.119, +0.058, +0.194, +0.147

NashConv (sum of the seats' gains): m2_defense_s2@5m/sample: +0.008 [-0.208, +0.225]; m2_defense_s2@5m/guarded: +0.175 [-0.072, +0.428]; m2_defense_s2@10m/sample: +0.050 [-0.178, +0.281]; m2_defense_s2@10m/guarded: +0.192 [-0.056, +0.461]; m2_defense_s2@16m/sample: +0.183 [-0.100, +0.475]; m2_defense_s2@16m/guarded: +0.308 [+0.083, +0.558]; m2_defense_s2@21m/sample: +0.142 [-0.089, +0.386]; m2_defense_s2@21m/guarded: +0.242 [-0.039, +0.519]; m2_defense_s2/sample: +0.242 [+0.000, +0.503]; m2_defense_s2/guarded: +0.592 [+0.319, +0.903]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| merchant | -0.262 [-0.279, -0.245] | -1.008 [-1.074, -0.941] |
| run1 | -0.196 [-0.221, -0.169] | -0.769 [-0.859, -0.669] |
| run2 | -0.067 [-0.095, -0.038] | -0.228 [-0.335, -0.117] |
| exploit_run2 | +0.024 [-0.002, +0.050] | +0.064 [-0.033, +0.166] |
| control | +0.005 [-0.031, +0.042] | +0.022 [-0.112, +0.162] |
| defense | -0.034 [-0.071, +0.003] | -0.136 [-0.275, +0.006] |
| magnate | +0.043 [+0.018, +0.070] | +0.164 [+0.060, +0.269] |
| m2_defense_s2@5m/sample * | +0.022 [-0.013, +0.058] | +0.081 [-0.053, +0.220] |
| m2_defense_s2@5m/guarded * | +0.065 [+0.028, +0.101] | +0.256 [+0.118, +0.396] |
| m2_defense_s2@10m/sample * | +0.009 [-0.026, +0.042] | +0.028 [-0.110, +0.156] |
| m2_defense_s2@10m/guarded * | +0.072 [+0.038, +0.108] | +0.269 [+0.141, +0.410] |
| m2_defense_s2@16m/sample * | +0.032 [-0.005, +0.068] | +0.136 [-0.012, +0.281] |
| m2_defense_s2@16m/guarded * | +0.061 [+0.023, +0.099] | +0.253 [+0.107, +0.396] |
| m2_defense_s2@21m/sample * | +0.011 [-0.025, +0.049] | +0.058 [-0.076, +0.205] |
| m2_defense_s2@21m/guarded * | +0.055 [+0.017, +0.092] | +0.222 [+0.077, +0.369] |
| m2_defense_s2/sample * | +0.055 [+0.019, +0.090] | +0.214 [+0.073, +0.355] |
| m2_defense_s2/guarded * | +0.098 [+0.060, +0.140] | +0.403 [+0.254, +0.564] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of m2_defense_s2/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 60 seeds (it understates near k = 60).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.126 | 0.065 |
| 10 | ±0.089 | 0.040 |
| 25 | ±0.057 | 0.021 |
| 50 | ±0.040 | 0.008 |
| 60 | ±0.036 | 0.000 |

## How well the gain of m2_defense_s2/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 60 seeds (it understates near k = 60).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.143 | 0.069 |
| 10 | ±0.101 | 0.046 |
| 25 | ±0.064 | 0.025 |
| 50 | ±0.045 | 0.009 |
| 60 | ±0.041 | 0.000 |

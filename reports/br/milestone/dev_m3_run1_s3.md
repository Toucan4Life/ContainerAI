# Milestone, dev study: m3_run1_s3 (init run1, profile run2,magnate,exploit_run2)

Profile sigma: run2 0.33, magnate 0.33, exploit_run2 0.33. Every other seat draws its strategy iid from sigma. Seeds: 60 per player count from the **dev** block (offset 100), 3 player counts, every seat of every table, standard length; 12,240 games, 2 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-e1926b48b863.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.357 [+0.319, +0.396]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.357 [-0.396, -0.319] | 0 |
| run1 | +0.122 [+0.078, +0.167] | -0.235 [-0.291, -0.179] | 0 |
| run2 | +0.281 [+0.217, +0.350] | -0.077 [-0.123, -0.026] | 0 |
| exploit_run2 | +0.383 [+0.317, +0.450] | +0.026 [-0.027, +0.073] | 0 |
| control | +0.383 [+0.300, +0.456] | +0.026 [-0.069, +0.113] | 0 |
| defense | +0.331 [+0.267, +0.403] | -0.027 [-0.102, +0.054] | 0 |
| magnate | +0.408 [+0.347, +0.469] | +0.051 [-0.002, +0.104] | 0 |
| m3_run1_s3@5m/sample * | +0.203 [+0.153, +0.256] | -0.155 [-0.229, -0.081] | 0 |
| m3_run1_s3@5m/guarded * | +0.214 [+0.158, +0.269] | -0.144 [-0.221, -0.068] | 0 |
| m3_run1_s3@10m/sample * | +0.142 [+0.089, +0.192] | -0.216 [-0.282, -0.153] | 0 |
| m3_run1_s3@10m/guarded * | +0.225 [+0.172, +0.278] | -0.132 [-0.203, -0.068] | 0 |
| m3_run1_s3@16m/sample * | +0.208 [+0.153, +0.267] | -0.149 [-0.220, -0.081] | 0 |
| m3_run1_s3@16m/guarded * | +0.161 [+0.111, +0.222] | -0.196 [-0.260, -0.129] | 0 |
| m3_run1_s3@21m/sample * | +0.144 [+0.100, +0.189] | -0.213 [-0.277, -0.145] | 1 |
| m3_run1_s3@21m/guarded * | +0.194 [+0.139, +0.244] | -0.163 [-0.236, -0.092] | 0 |
| m3_run1_s3/sample * | +0.200 [+0.144, +0.256] | -0.157 [-0.218, -0.094] | 1 |
| m3_run1_s3/guarded * | +0.178 [+0.128, +0.228] | -0.180 [-0.249, -0.111] | 0 |

Gain by turn position (candidates; position 0 moves first): m3_run1_s3@5m/sample: -0.131, -0.169, -0.164; m3_run1_s3@5m/guarded: -0.189, -0.144, -0.097; m3_run1_s3@10m/sample: -0.181, -0.161, -0.306; m3_run1_s3@10m/guarded: -0.197, -0.086, -0.114; m3_run1_s3@16m/sample: -0.181, -0.128, -0.139; m3_run1_s3@16m/guarded: -0.181, -0.161, -0.247; m3_run1_s3@21m/sample: -0.214, -0.144, -0.281; m3_run1_s3@21m/guarded: -0.097, -0.161, -0.231; m3_run1_s3/sample: -0.131, -0.178, -0.164; m3_run1_s3/guarded: -0.131, -0.128, -0.281

NashConv (sum of the seats' gains): m3_run1_s3@5m/sample: -0.464 [-0.686, -0.242]; m3_run1_s3@5m/guarded: -0.431 [-0.664, -0.203]; m3_run1_s3@10m/sample: -0.647 [-0.847, -0.458]; m3_run1_s3@10m/guarded: -0.397 [-0.608, -0.203]; m3_run1_s3@16m/sample: -0.447 [-0.661, -0.242]; m3_run1_s3@16m/guarded: -0.589 [-0.781, -0.386]; m3_run1_s3@21m/sample: -0.639 [-0.831, -0.436]; m3_run1_s3@21m/guarded: -0.489 [-0.708, -0.275]; m3_run1_s3/sample: -0.472 [-0.653, -0.283]; m3_run1_s3/guarded: -0.539 [-0.747, -0.333]

## 4 players

Value of sigma: +0.265 [+0.236, +0.295]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.265 [-0.295, -0.236] | 0 |
| run1 | +0.046 [+0.021, +0.071] | -0.219 [-0.259, -0.178] | 0 |
| run2 | +0.179 [+0.138, +0.225] | -0.086 [-0.126, -0.045] | 0 |
| exploit_run2 | +0.296 [+0.250, +0.344] | +0.031 [-0.014, +0.071] | 0 |
| control | +0.258 [+0.204, +0.315] | -0.007 [-0.073, +0.059] | 0 |
| defense | +0.223 [+0.173, +0.279] | -0.042 [-0.101, +0.017] | 0 |
| magnate | +0.321 [+0.263, +0.381] | +0.056 [+0.015, +0.101] | 0 |
| m3_run1_s3@5m/sample * | +0.083 [+0.050, +0.117] | -0.182 [-0.229, -0.134] | 0 |
| m3_run1_s3@5m/guarded * | +0.096 [+0.058, +0.133] | -0.169 [-0.215, -0.123] | 0 |
| m3_run1_s3@10m/sample * | +0.090 [+0.054, +0.127] | -0.176 [-0.224, -0.127] | 0 |
| m3_run1_s3@10m/guarded * | +0.158 [+0.113, +0.206] | -0.107 [-0.162, -0.051] | 0 |
| m3_run1_s3@16m/sample * | +0.146 [+0.106, +0.190] | -0.119 [-0.167, -0.071] | 0 |
| m3_run1_s3@16m/guarded * | +0.175 [+0.133, +0.221] | -0.090 [-0.142, -0.034] | 0 |
| m3_run1_s3@21m/sample * | +0.163 [+0.117, +0.208] | -0.103 [-0.158, -0.049] | 0 |
| m3_run1_s3@21m/guarded * | +0.204 [+0.154, +0.254] | -0.061 [-0.122, -0.001] | 0 |
| m3_run1_s3/sample * | +0.163 [+0.117, +0.208] | -0.103 [-0.156, -0.051] | 0 |
| m3_run1_s3/guarded * | +0.194 [+0.140, +0.250] | -0.072 [-0.133, -0.007] | 0 |

Gain by turn position (candidates; position 0 moves first): m3_run1_s3@5m/sample: -0.225, -0.206, -0.183, -0.114; m3_run1_s3@5m/guarded: -0.242, -0.122, -0.183, -0.131; m3_run1_s3@10m/sample: -0.208, -0.239, -0.167, -0.089; m3_run1_s3@10m/guarded: -0.067, -0.106, -0.133, -0.122; m3_run1_s3@16m/sample: -0.150, -0.114, -0.117, -0.097; m3_run1_s3@16m/guarded: -0.092, -0.089, -0.083, -0.097; m3_run1_s3@21m/sample: -0.142, -0.122, -0.083, -0.064; m3_run1_s3@21m/guarded: -0.008, -0.106, -0.067, -0.064; m3_run1_s3/sample: -0.158, -0.072, -0.167, -0.014; m3_run1_s3/guarded: -0.142, -0.072, -0.025, -0.047

NashConv (sum of the seats' gains): m3_run1_s3@5m/sample: -0.728 [-0.917, -0.536]; m3_run1_s3@5m/guarded: -0.678 [-0.861, -0.492]; m3_run1_s3@10m/sample: -0.703 [-0.894, -0.508]; m3_run1_s3@10m/guarded: -0.428 [-0.647, -0.203]; m3_run1_s3@16m/sample: -0.478 [-0.667, -0.283]; m3_run1_s3@16m/guarded: -0.361 [-0.569, -0.136]; m3_run1_s3@21m/sample: -0.411 [-0.631, -0.197]; m3_run1_s3@21m/guarded: -0.244 [-0.486, -0.006]; m3_run1_s3/sample: -0.411 [-0.625, -0.203]; m3_run1_s3/guarded: -0.286 [-0.533, -0.028]

## 5 players

Value of sigma: +0.223 [+0.198, +0.248]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.223 [-0.248, -0.198] | 0 |
| run1 | +0.042 [+0.020, +0.067] | -0.181 [-0.215, -0.147] | 0 |
| run2 | +0.200 [+0.158, +0.242] | -0.023 [-0.059, +0.013] | 0 |
| exploit_run2 | +0.180 [+0.142, +0.222] | -0.043 [-0.078, -0.007] | 0 |
| control | +0.178 [+0.142, +0.217] | -0.044 [-0.087, -0.002] | 0 |
| defense | +0.223 [+0.182, +0.267] | +0.001 [-0.045, +0.047] | 0 |
| magnate | +0.288 [+0.242, +0.338] | +0.066 [+0.028, +0.104] | 0 |
| m3_run1_s3@5m/sample * | +0.027 [+0.010, +0.043] | -0.196 [-0.228, -0.163] | 0 |
| m3_run1_s3@5m/guarded * | +0.070 [+0.047, +0.097] | -0.153 [-0.186, -0.120] | 0 |
| m3_run1_s3@10m/sample * | +0.043 [+0.020, +0.067] | -0.179 [-0.211, -0.148] | 0 |
| m3_run1_s3@10m/guarded * | +0.095 [+0.063, +0.128] | -0.128 [-0.168, -0.088] | 0 |
| m3_run1_s3@16m/sample * | +0.082 [+0.053, +0.113] | -0.141 [-0.176, -0.103] | 0 |
| m3_run1_s3@16m/guarded * | +0.110 [+0.073, +0.143] | -0.113 [-0.154, -0.069] | 0 |
| m3_run1_s3@21m/sample * | +0.108 [+0.077, +0.143] | -0.114 [-0.154, -0.073] | 0 |
| m3_run1_s3@21m/guarded * | +0.125 [+0.087, +0.160] | -0.098 [-0.142, -0.052] | 0 |
| m3_run1_s3/sample * | +0.112 [+0.080, +0.145] | -0.111 [-0.146, -0.073] | 0 |
| m3_run1_s3/guarded * | +0.158 [+0.123, +0.195] | -0.064 [-0.107, -0.021] | 0 |

Gain by turn position (candidates; position 0 moves first): m3_run1_s3@5m/sample: -0.244, -0.189, -0.200, -0.108, -0.239; m3_run1_s3@5m/guarded: -0.211, -0.156, -0.083, -0.142, -0.172; m3_run1_s3@10m/sample: -0.228, -0.172, -0.167, -0.125, -0.206; m3_run1_s3@10m/guarded: -0.153, -0.139, -0.133, -0.108, -0.106; m3_run1_s3@16m/sample: -0.144, -0.164, -0.083, -0.125, -0.189; m3_run1_s3@16m/guarded: -0.144, -0.072, -0.033, -0.092, -0.222; m3_run1_s3@21m/sample: -0.111, -0.139, -0.075, -0.108, -0.139; m3_run1_s3@21m/guarded: -0.061, -0.081, -0.067, -0.142, -0.139; m3_run1_s3/sample: -0.111, -0.097, -0.083, -0.108, -0.156; m3_run1_s3/guarded: -0.078, -0.089, +0.058, -0.025, -0.189

NashConv (sum of the seats' gains): m3_run1_s3@5m/sample: -0.981 [-1.142, -0.814]; m3_run1_s3@5m/guarded: -0.764 [-0.928, -0.600]; m3_run1_s3@10m/sample: -0.897 [-1.056, -0.739]; m3_run1_s3@10m/guarded: -0.639 [-0.839, -0.439]; m3_run1_s3@16m/sample: -0.706 [-0.881, -0.517]; m3_run1_s3@16m/guarded: -0.564 [-0.772, -0.344]; m3_run1_s3@21m/sample: -0.572 [-0.769, -0.367]; m3_run1_s3@21m/guarded: -0.489 [-0.708, -0.261]; m3_run1_s3/sample: -0.556 [-0.728, -0.367]; m3_run1_s3/guarded: -0.322 [-0.536, -0.103]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| merchant | -0.282 [-0.301, -0.264] | -1.082 [-1.153, -1.015] |
| run1 | -0.212 [-0.238, -0.188] | -0.830 [-0.930, -0.738] |
| run2 | -0.062 [-0.086, -0.038] | -0.230 [-0.324, -0.135] |
| exploit_run2 | +0.005 [-0.021, +0.029] | -0.005 [-0.104, +0.092] |
| control | -0.008 [-0.051, +0.028] | -0.057 [-0.208, +0.079] |
| defense | -0.023 [-0.058, +0.014] | -0.082 [-0.219, +0.057] |
| magnate | +0.057 [+0.032, +0.084] | +0.234 [+0.139, +0.336] |
| m3_run1_s3@5m/sample * | -0.178 [-0.209, -0.145] | -0.724 [-0.835, -0.607] |
| m3_run1_s3@5m/guarded * | -0.155 [-0.186, -0.125] | -0.624 [-0.731, -0.519] |
| m3_run1_s3@10m/sample * | -0.190 [-0.220, -0.163] | -0.749 [-0.857, -0.650] |
| m3_run1_s3@10m/guarded * | -0.122 [-0.155, -0.092] | -0.488 [-0.609, -0.373] |
| m3_run1_s3@16m/sample * | -0.137 [-0.168, -0.106] | -0.544 [-0.657, -0.431] |
| m3_run1_s3@16m/guarded * | -0.133 [-0.163, -0.103] | -0.505 [-0.624, -0.388] |
| m3_run1_s3@21m/sample * | -0.143 [-0.175, -0.112] | -0.541 [-0.660, -0.417] |
| m3_run1_s3@21m/guarded * | -0.107 [-0.144, -0.071] | -0.407 [-0.546, -0.272] |
| m3_run1_s3/sample * | -0.124 [-0.152, -0.096] | -0.480 [-0.588, -0.378] |
| m3_run1_s3/guarded * | -0.105 [-0.140, -0.072] | -0.382 [-0.515, -0.258] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of m3_run1_s3/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 60 seeds (it understates near k = 60).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.102 | 0.050 |
| 10 | ±0.072 | 0.034 |
| 25 | ±0.046 | 0.018 |
| 50 | ±0.032 | 0.007 |
| 60 | ±0.029 | 0.000 |

## How well the gain of m3_run1_s3/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 60 seeds (it understates near k = 60).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.119 | 0.058 |
| 10 | ±0.084 | 0.040 |
| 25 | ±0.053 | 0.021 |
| 50 | ±0.038 | 0.008 |
| 60 | ±0.034 | 0.000 |

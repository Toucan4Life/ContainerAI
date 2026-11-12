# PSRO iteration 2: holdout confirmation of the admitted response against the meta-strategy it was trained against

Profile sigma: 3 players: run2 0.00, magnate 1.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00; 4 players: run2 0.00, magnate 0.07, control 0.37, defense 0.50, exploit_run2 0.06, merchant 0.00; 5 players: run2 0.00, magnate 1.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00. Every other seat draws its strategy iid from sigma. Seeds: 100 per player count from the **holdout** block (offset 500), 3 player counts, every seat of every table, standard length; 9,600 games, 0 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-79634ab95aa6.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.333 [+0.333, +0.333]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.232 [+0.185, +0.278] | -0.102 [-0.148, -0.055] | 0 |
| magnate | +0.333 [+0.333, +0.333] | +0.000 [+0.000, +0.000] | 0 |
| control | +0.227 [+0.183, +0.273] | -0.107 [-0.150, -0.060] | 0 |
| defense | +0.272 [+0.227, +0.320] | -0.062 [-0.107, -0.013] | 0 |
| exploit_run2 | +0.263 [+0.217, +0.312] | -0.070 [-0.117, -0.022] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.333 [-0.333, -0.333] | 0 |
| resp02/guarded * | +0.467 [+0.417, +0.515] | +0.133 [+0.083, +0.182] | 0 |
| resp02/sample * | +0.405 [+0.357, +0.455] | +0.072 [+0.023, +0.122] | 0 |

Gain by turn position (candidates; position 0 moves first): resp02/guarded: +0.210, +0.170, +0.020; resp02/sample: +0.115, +0.040, +0.060

NashConv (sum of the seats' gains): resp02/guarded: +0.400 [+0.250, +0.545]; resp02/sample: +0.215 [+0.070, +0.365]

## 4 players

Value of sigma: +0.245 [+0.218, +0.272]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.175 [+0.139, +0.216] | -0.070 [-0.115, -0.020] | 0 |
| magnate | +0.289 [+0.247, +0.330] | +0.044 [-0.002, +0.093] | 0 |
| control | +0.254 [+0.207, +0.296] | +0.009 [-0.025, +0.043] | 0 |
| defense | +0.229 [+0.189, +0.271] | -0.016 [-0.042, +0.012] | 0 |
| exploit_run2 | +0.270 [+0.233, +0.306] | +0.025 [-0.019, +0.066] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.245 [-0.272, -0.218] | 0 |
| resp02/guarded * | +0.429 [+0.384, +0.474] | +0.184 [+0.137, +0.234] | 0 |
| resp02/sample * | +0.403 [+0.362, +0.443] | +0.158 [+0.108, +0.206] | 0 |

Gain by turn position (candidates; position 0 moves first): resp02/guarded: +0.168, +0.221, +0.163, +0.185; resp02/sample: +0.143, +0.141, +0.138, +0.210

NashConv (sum of the seats' gains): resp02/guarded: +0.737 [+0.548, +0.935]; resp02/sample: +0.632 [+0.433, +0.823]

## 5 players

Value of sigma: +0.200 [+0.200, +0.200]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.135 [+0.108, +0.166] | -0.065 [-0.092, -0.034] | 0 |
| magnate | +0.200 [+0.200, +0.200] | +0.000 [+0.000, +0.000] | 0 |
| control | +0.171 [+0.141, +0.201] | -0.029 [-0.059, +0.001] | 0 |
| defense | +0.149 [+0.120, +0.179] | -0.051 [-0.080, -0.021] | 0 |
| exploit_run2 | +0.155 [+0.124, +0.187] | -0.045 [-0.076, -0.013] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.200 [-0.200, -0.200] | 0 |
| resp02/guarded * | +0.241 [+0.207, +0.276] | +0.041 [+0.007, +0.076] | 0 |
| resp02/sample * | +0.250 [+0.211, +0.289] | +0.050 [+0.011, +0.089] | 0 |

Gain by turn position (candidates; position 0 moves first): resp02/guarded: +0.115, +0.000, +0.015, +0.095, -0.020; resp02/sample: +0.005, +0.055, +0.060, +0.095, +0.035

NashConv (sum of the seats' gains): resp02/guarded: +0.205 [+0.035, +0.380]; resp02/sample: +0.250 [+0.055, +0.445]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| run2 | -0.079 [-0.103, -0.055] | -0.303 [-0.395, -0.209] |
| magnate | +0.015 [-0.001, +0.030] | +0.059 [-0.005, +0.118] |
| control | -0.042 [-0.064, -0.021] | -0.143 [-0.225, -0.059] |
| defense | -0.043 [-0.063, -0.022] | -0.168 [-0.241, -0.090] |
| exploit_run2 | -0.030 [-0.052, -0.007] | -0.111 [-0.194, -0.020] |
| merchant | -0.259 [-0.269, -0.251] | -0.993 [-1.030, -0.958] |
| resp02/guarded * | +0.120 [+0.093, +0.145] | +0.447 [+0.345, +0.547] |
| resp02/sample * | +0.093 [+0.068, +0.120] | +0.366 [+0.265, +0.470] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of resp02/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.116 | 0.054 |
| 10 | ±0.082 | 0.040 |
| 25 | ±0.052 | 0.023 |
| 50 | ±0.037 | 0.014 |
| 100 | ±0.026 | 0.000 |

## How well the gain of resp02/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 100 seeds (it understates near k = 100).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.120 | 0.060 |
| 10 | ±0.085 | 0.040 |
| 25 | ±0.054 | 0.024 |
| 50 | ±0.038 | 0.014 |
| 100 | ±0.027 | 0.000 |

# Milestone, holdout study: opponents drawn from run2,magnate,control,defense,exploit_run2,merchant with equal weights

Profile sigma: run2 0.17, magnate 0.17, control 0.17, defense 0.17, exploit_run2 0.17, merchant 0.17. Every other seat draws its strategy iid from sigma. Seeds: 150 per player count from the **holdout** block (offset 0), 3 player counts, every seat of every table, standard length; 16,200 games, 5 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-e1926b48b863.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.332 [+0.312, +0.351]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.016 [+0.004, +0.029] | -0.316 [-0.335, -0.296] | 0 |
| run1 | +0.228 [+0.190, +0.263] | -0.104 [-0.141, -0.068] | 3 |
| run2 | +0.358 [+0.316, +0.398] | +0.026 [-0.009, +0.061] | 0 |
| exploit_run2 | +0.361 [+0.318, +0.406] | +0.029 [-0.006, +0.066] | 0 |
| control | +0.389 [+0.349, +0.433] | +0.057 [+0.020, +0.097] | 1 |
| defense | +0.410 [+0.367, +0.452] | +0.078 [+0.042, +0.115] | 0 |
| magnate | +0.457 [+0.410, +0.499] | +0.125 [+0.088, +0.160] | 0 |
| m4_broad_s4/sample * | +0.352 [+0.312, +0.390] | +0.021 [-0.019, +0.059] | 1 |
| m4_broad_s4/guarded * | +0.384 [+0.343, +0.428] | +0.053 [+0.010, +0.096] | 0 |

Gain by turn position (candidates; position 0 moves first): m4_broad_s4/sample: +0.009, +0.040, +0.013; m4_broad_s4/guarded: +0.066, +0.060, +0.033

NashConv (sum of the seats' gains): m4_broad_s4/sample: +0.062 [-0.056, +0.177]; m4_broad_s4/guarded: +0.158 [+0.031, +0.288]

## 4 players

Value of sigma: +0.259 [+0.246, +0.274]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.003 [+0.000, +0.008] | -0.256 [-0.270, -0.243] | 0 |
| run1 | +0.107 [+0.083, +0.132] | -0.153 [-0.178, -0.129] | 0 |
| run2 | +0.270 [+0.231, +0.309] | +0.011 [-0.025, +0.046] | 0 |
| exploit_run2 | +0.282 [+0.245, +0.324] | +0.023 [-0.012, +0.060] | 0 |
| control | +0.308 [+0.277, +0.340] | +0.049 [+0.021, +0.076] | 0 |
| defense | +0.312 [+0.278, +0.346] | +0.053 [+0.024, +0.084] | 0 |
| magnate | +0.380 [+0.343, +0.417] | +0.121 [+0.089, +0.151] | 0 |
| m4_broad_s4/sample * | +0.286 [+0.251, +0.322] | +0.026 [-0.010, +0.062] | 0 |
| m4_broad_s4/guarded * | +0.321 [+0.284, +0.357] | +0.061 [+0.025, +0.097] | 0 |

Gain by turn position (candidates; position 0 moves first): m4_broad_s4/sample: +0.009, +0.067, +0.059, -0.029; m4_broad_s4/guarded: +0.059, +0.147, +0.029, +0.011

NashConv (sum of the seats' gains): m4_broad_s4/sample: +0.106 [-0.042, +0.248]; m4_broad_s4/guarded: +0.246 [+0.099, +0.387]

## 5 players

Value of sigma: +0.196 [+0.181, +0.209]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.196 [-0.209, -0.181] | 0 |
| run1 | +0.075 [+0.056, +0.095] | -0.121 [-0.142, -0.098] | 0 |
| run2 | +0.237 [+0.207, +0.265] | +0.041 [+0.016, +0.067] | 0 |
| exploit_run2 | +0.150 [+0.123, +0.180] | -0.046 [-0.070, -0.021] | 0 |
| control | +0.243 [+0.213, +0.274] | +0.048 [+0.024, +0.073] | 0 |
| defense | +0.241 [+0.209, +0.273] | +0.046 [+0.019, +0.073] | 0 |
| magnate | +0.303 [+0.267, +0.335] | +0.107 [+0.079, +0.134] | 0 |
| m4_broad_s4/sample * | +0.215 [+0.185, +0.243] | +0.019 [-0.014, +0.049] | 0 |
| m4_broad_s4/guarded * | +0.241 [+0.211, +0.269] | +0.045 [+0.015, +0.074] | 0 |

Gain by turn position (candidates; position 0 moves first): m4_broad_s4/sample: +0.022, +0.018, -0.008, +0.054, +0.009; m4_broad_s4/guarded: +0.066, +0.001, +0.096, +0.064, -0.001

NashConv (sum of the seats' gains): m4_broad_s4/sample: +0.095 [-0.068, +0.247]; m4_broad_s4/guarded: +0.225 [+0.077, +0.369]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| merchant | -0.256 [-0.265, -0.247] | -0.984 [-1.017, -0.950] |
| run1 | -0.126 [-0.142, -0.110] | -0.509 [-0.568, -0.450] |
| run2 | +0.026 [+0.007, +0.044] | +0.109 [+0.037, +0.175] |
| exploit_run2 | +0.002 [-0.017, +0.021] | -0.016 [-0.090, +0.059] |
| control | +0.051 [+0.034, +0.070] | +0.202 [+0.138, +0.272] |
| defense | +0.059 [+0.042, +0.078] | +0.225 [+0.158, +0.296] |
| magnate | +0.118 [+0.099, +0.135] | +0.464 [+0.393, +0.533] |
| m4_broad_s4/sample * | +0.022 [+0.002, +0.043] | +0.087 [+0.007, +0.167] |
| m4_broad_s4/guarded * | +0.053 [+0.032, +0.074] | +0.210 [+0.128, +0.290] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of m4_broad_s4/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.114 | 0.057 |
| 10 | ±0.081 | 0.039 |
| 25 | ±0.051 | 0.023 |
| 50 | ±0.036 | 0.015 |
| 100 | ±0.026 | 0.008 |
| 150 | ±0.021 | 0.000 |

## How well the gain of m4_broad_s4/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 150 seeds (it understates near k = 150).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.114 | 0.056 |
| 10 | ±0.081 | 0.041 |
| 25 | ±0.051 | 0.024 |
| 50 | ±0.036 | 0.015 |
| 100 | ±0.026 | 0.008 |
| 150 | ±0.021 | 0.000 |

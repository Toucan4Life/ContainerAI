# PSRO, holdout re-measure of the response of iteration 05 against the sigma it was trained against

Profile sigma: 3 players: run2 0.00, magnate 0.15, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.11, psro03 0.73; 4 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.37, psro03 0.63; 5 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.91, psro03 0.09. Every other seat draws its strategy iid from sigma. Seeds: 100 per player count from the **holdout** block (offset 700), 3 player counts, every seat of every table, standard length; 10,800 games, 0 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-a7d05e993a75.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.331 [+0.304, +0.360]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.210 [+0.168, +0.252] | -0.121 [-0.173, -0.072] | 0 |
| magnate | +0.222 [+0.180, +0.263] | -0.110 [-0.153, -0.069] | 0 |
| control | +0.223 [+0.178, +0.270] | -0.108 [-0.160, -0.056] | 0 |
| defense | +0.198 [+0.157, +0.242] | -0.133 [-0.181, -0.086] | 0 |
| exploit_run2 | +0.212 [+0.165, +0.260] | -0.120 [-0.170, -0.071] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.331 [-0.360, -0.304] | 0 |
| psro02 | +0.315 [+0.268, +0.365] | -0.016 [-0.068, +0.038] | 0 |
| psro03 | +0.357 [+0.320, +0.393] | +0.025 [+0.013, +0.038] | 0 |
| psro05 | +0.375 [+0.325, +0.425] | +0.044 [-0.017, +0.102] | 0 |

## 4 players

Value of sigma: +0.270 [+0.240, +0.300]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.069 [+0.045, +0.094] | -0.201 [-0.235, -0.166] | 0 |
| magnate | +0.121 [+0.092, +0.152] | -0.148 [-0.189, -0.107] | 0 |
| control | +0.100 [+0.074, +0.128] | -0.170 [-0.211, -0.129] | 0 |
| defense | +0.064 [+0.042, +0.089] | -0.205 [-0.240, -0.166] | 0 |
| exploit_run2 | +0.138 [+0.105, +0.173] | -0.132 [-0.180, -0.083] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.270 [-0.300, -0.240] | 0 |
| psro02 | +0.226 [+0.185, +0.266] | -0.043 [-0.080, -0.009] | 0 |
| psro03 | +0.295 [+0.257, +0.335] | +0.025 [+0.006, +0.047] | 0 |
| psro05 | +0.269 [+0.226, +0.311] | -0.001 [-0.052, +0.052] | 0 |

## 5 players

Value of sigma: +0.190 [+0.171, +0.209]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.059 [+0.039, +0.080] | -0.131 [-0.156, -0.106] | 0 |
| magnate | +0.150 [+0.118, +0.180] | -0.040 [-0.080, -0.002] | 0 |
| control | +0.126 [+0.100, +0.154] | -0.064 [-0.099, -0.027] | 0 |
| defense | +0.110 [+0.084, +0.138] | -0.080 [-0.112, -0.047] | 0 |
| exploit_run2 | +0.126 [+0.098, +0.153] | -0.064 [-0.100, -0.029] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.190 [-0.209, -0.171] | 0 |
| psro02 | +0.186 [+0.166, +0.207] | -0.004 [-0.007, +0.000] | 0 |
| psro03 | +0.226 [+0.190, +0.261] | +0.036 [-0.003, +0.074] | 0 |
| psro05 | +0.210 [+0.174, +0.247] | +0.020 [-0.018, +0.057] | 0 |

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| run2 | -0.151 [-0.175, -0.129] | -0.607 [-0.692, -0.527] |
| magnate | -0.099 [-0.122, -0.076] | -0.374 [-0.463, -0.283] |
| control | -0.114 [-0.139, -0.089] | -0.440 [-0.535, -0.349] |
| defense | -0.139 [-0.162, -0.117] | -0.539 [-0.627, -0.452] |
| exploit_run2 | -0.105 [-0.131, -0.079] | -0.402 [-0.502, -0.300] |
| merchant | -0.264 [-0.279, -0.249] | -1.007 [-1.069, -0.950] |
| psro02 | -0.021 [-0.043, +0.001] | -0.080 [-0.151, -0.007] |
| psro03 | +0.029 [+0.014, +0.044] | +0.120 [+0.050, +0.191] |
| psro05 | +0.021 [-0.009, +0.050] | +0.076 [-0.036, +0.185] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

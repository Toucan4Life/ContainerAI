# Deviation study

Profile sigma: magnate 1.00. Every other seat draws its strategy iid from sigma. Seeds: 100 per player count from the **holdout** block (offset 1600), 3 player counts, every seat of every table, standard length; 12,000 games, 0 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-6a85706ecfd3.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.333 [+0.333, +0.333]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.267 [+0.227, +0.310] | -0.067 [-0.107, -0.023] | 0 |
| magnate | +0.333 [+0.333, +0.333] | +0.000 [+0.000, +0.000] | 0 |
| control | +0.247 [+0.198, +0.297] | -0.087 [-0.135, -0.037] | 0 |
| defense | +0.290 [+0.245, +0.335] | -0.043 [-0.088, +0.002] | 0 |
| exploit_run2 | +0.317 [+0.265, +0.372] | -0.017 [-0.068, +0.038] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.333 [-0.333, -0.333] | 0 |
| psro02 | +0.465 [+0.415, +0.517] | +0.132 [+0.082, +0.183] | 0 |
| psro03 | +0.455 [+0.412, +0.500] | +0.122 [+0.078, +0.167] | 0 |
| psro05 | +0.390 [+0.333, +0.450] | +0.057 [+0.000, +0.117] | 0 |
| psro08 | +0.400 [+0.353, +0.445] | +0.067 [+0.020, +0.112] | 0 |

## 4 players

Value of sigma: +0.250 [+0.250, +0.250]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.151 [+0.120, +0.181] | -0.099 [-0.130, -0.069] | 0 |
| magnate | +0.250 [+0.250, +0.250] | +0.000 [+0.000, +0.000] | 0 |
| control | +0.214 [+0.179, +0.250] | -0.036 [-0.071, +0.000] | 0 |
| defense | +0.189 [+0.151, +0.226] | -0.061 [-0.099, -0.024] | 0 |
| exploit_run2 | +0.221 [+0.184, +0.259] | -0.029 [-0.066, +0.009] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.250 [-0.250, -0.250] | 0 |
| psro02 | +0.331 [+0.287, +0.379] | +0.081 [+0.037, +0.129] | 0 |
| psro03 | +0.398 [+0.355, +0.439] | +0.147 [+0.105, +0.189] | 0 |
| psro05 | +0.419 [+0.376, +0.463] | +0.169 [+0.126, +0.212] | 0 |
| psro08 | +0.420 [+0.378, +0.465] | +0.170 [+0.128, +0.215] | 0 |

## 5 players

Value of sigma: +0.200 [+0.200, +0.200]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.108 [+0.082, +0.136] | -0.092 [-0.118, -0.064] | 0 |
| magnate | +0.200 [+0.200, +0.200] | +0.000 [+0.000, +0.000] | 0 |
| control | +0.167 [+0.137, +0.197] | -0.033 [-0.063, -0.003] | 0 |
| defense | +0.157 [+0.125, +0.187] | -0.043 [-0.075, -0.013] | 0 |
| exploit_run2 | +0.145 [+0.115, +0.175] | -0.055 [-0.085, -0.025] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.200 [-0.200, -0.200] | 0 |
| psro02 | +0.265 [+0.229, +0.299] | +0.065 [+0.029, +0.099] | 0 |
| psro03 | +0.284 [+0.244, +0.320] | +0.084 [+0.044, +0.120] | 0 |
| psro05 | +0.321 [+0.278, +0.364] | +0.121 [+0.078, +0.164] | 0 |
| psro08 | +0.365 [+0.327, +0.402] | +0.165 [+0.127, +0.202] | 0 |

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| run2 | -0.086 [-0.104, -0.066] | -0.352 [-0.420, -0.277] |
| magnate | +0.000 [+0.000, +0.000] | +0.000 [+0.000, +0.000] |
| control | -0.052 [-0.074, -0.028] | -0.190 [-0.272, -0.103] |
| defense | -0.049 [-0.072, -0.026] | -0.197 [-0.283, -0.112] |
| exploit_run2 | -0.033 [-0.057, -0.008] | -0.147 [-0.235, -0.052] |
| merchant | -0.261 [-0.261, -0.261] | -1.000 [-1.000, -1.000] |
| psro02 | +0.093 [+0.068, +0.118] | +0.348 [+0.252, +0.448] |
| psro03 | +0.118 [+0.092, +0.143] | +0.458 [+0.358, +0.558] |
| psro05 | +0.115 [+0.089, +0.144] | +0.483 [+0.377, +0.588] |
| psro08 | +0.134 [+0.110, +0.159] | +0.568 [+0.473, +0.662] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

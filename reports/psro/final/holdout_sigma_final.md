# PSRO, holdout study of the solution of the final restricted game

Profile sigma: 3 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.00, psro03 0.20, psro05 0.80; 4 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.00, psro03 0.64, psro05 0.35; 5 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.00, psro03 1.00, psro05 0.00. Every other seat draws its strategy iid from sigma. Seeds: 150 per player count from the **holdout** block (offset 350), 3 player counts, every seat of every table, standard length; 16,200 games, 1 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-a7d05e993a75.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.352 [+0.330, +0.374]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.212 [+0.174, +0.250] | -0.140 [-0.185, -0.096] | 0 |
| magnate | +0.261 [+0.224, +0.297] | -0.091 [-0.134, -0.048] | 0 |
| control | +0.196 [+0.162, +0.231] | -0.157 [-0.197, -0.115] | 0 |
| defense | +0.211 [+0.177, +0.248] | -0.141 [-0.183, -0.096] | 0 |
| exploit_run2 | +0.211 [+0.176, +0.248] | -0.141 [-0.179, -0.100] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.352 [-0.374, -0.330] | 1 |
| psro02 | +0.287 [+0.246, +0.327] | -0.066 [-0.113, -0.017] | 0 |
| psro03 | +0.308 [+0.271, +0.344] | -0.044 [-0.084, -0.004] | 0 |
| psro05 | +0.363 [+0.336, +0.391] | +0.011 [+0.001, +0.021] | 0 |

## 4 players

Value of sigma: +0.247 [+0.224, +0.270]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.069 [+0.050, +0.091] | -0.178 [-0.207, -0.147] | 0 |
| magnate | +0.106 [+0.083, +0.130] | -0.141 [-0.175, -0.108] | 0 |
| control | +0.086 [+0.066, +0.107] | -0.161 [-0.191, -0.131] | 0 |
| defense | +0.090 [+0.070, +0.111] | -0.157 [-0.189, -0.125] | 0 |
| exploit_run2 | +0.104 [+0.080, +0.129] | -0.143 [-0.174, -0.109] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.247 [-0.270, -0.224] | 0 |
| psro02 | +0.223 [+0.189, +0.254] | -0.024 [-0.065, +0.015] | 0 |
| psro03 | +0.223 [+0.193, +0.257] | -0.023 [-0.040, -0.006] | 0 |
| psro05 | +0.289 [+0.258, +0.323] | +0.042 [+0.011, +0.073] | 0 |

## 5 players

Value of sigma: +0.200 [+0.200, +0.200]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.066 [+0.049, +0.084] | -0.134 [-0.151, -0.116] | 0 |
| magnate | +0.129 [+0.106, +0.154] | -0.071 [-0.094, -0.046] | 0 |
| control | +0.094 [+0.074, +0.115] | -0.106 [-0.126, -0.085] | 0 |
| defense | +0.087 [+0.069, +0.104] | -0.113 [-0.131, -0.096] | 0 |
| exploit_run2 | +0.085 [+0.065, +0.105] | -0.115 [-0.135, -0.095] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.200 [-0.200, -0.200] | 0 |
| psro02 | +0.183 [+0.156, +0.210] | -0.017 [-0.044, +0.010] | 0 |
| psro03 | +0.200 [+0.200, +0.200] | +0.000 [+0.000, +0.000] | 0 |
| psro05 | +0.199 [+0.172, +0.225] | -0.001 [-0.028, +0.025] | 0 |

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| run2 | -0.151 [-0.169, -0.131] | -0.600 [-0.666, -0.530] |
| magnate | -0.101 [-0.122, -0.080] | -0.397 [-0.476, -0.317] |
| control | -0.141 [-0.159, -0.123] | -0.548 [-0.613, -0.481] |
| defense | -0.137 [-0.156, -0.117] | -0.539 [-0.608, -0.469] |
| exploit_run2 | -0.133 [-0.151, -0.115] | -0.522 [-0.588, -0.452] |
| merchant | -0.266 [-0.278, -0.255] | -1.014 [-1.054, -0.976] |
| psro02 | -0.036 [-0.059, -0.013] | -0.127 [-0.211, -0.041] |
| psro03 | -0.023 [-0.038, -0.008] | -0.076 [-0.122, -0.030] |
| psro05 | +0.017 [+0.004, +0.031] | +0.066 [+0.004, +0.128] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

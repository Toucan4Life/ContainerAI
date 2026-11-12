# Deviation study

Profile sigma: 3 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.42, psro03 0.14, psro05 0.00, psro08 0.44; 4 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.00, psro03 0.00, psro05 0.30, psro08 0.70; 5 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.00, psro03 0.00, psro05 0.06, psro08 0.94. Every other seat draws its strategy iid from sigma. Seeds: 150 per player count from the **holdout** block (offset 1400), 3 player counts, every seat of every table, standard length; 18,000 games, 3 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-6a85706ecfd3.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.322 [+0.297, +0.347]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.180 [+0.144, +0.218] | -0.142 [-0.181, -0.100] | 0 |
| magnate | +0.208 [+0.173, +0.242] | -0.114 [-0.155, -0.075] | 0 |
| control | +0.208 [+0.173, +0.243] | -0.114 [-0.155, -0.072] | 0 |
| defense | +0.214 [+0.178, +0.252] | -0.107 [-0.153, -0.061] | 0 |
| exploit_run2 | +0.213 [+0.179, +0.249] | -0.108 [-0.151, -0.070] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.322 [-0.347, -0.297] | 0 |
| psro02 | +0.298 [+0.258, +0.334] | -0.024 [-0.055, +0.005] | 0 |
| psro03 | +0.306 [+0.262, +0.351] | -0.016 [-0.056, +0.025] | 0 |
| psro05 | +0.380 [+0.343, +0.418] | +0.058 [+0.013, +0.102] | 0 |
| psro08 | +0.350 [+0.310, +0.393] | +0.028 [-0.001, +0.060] | 0 |

## 4 players

Value of sigma: +0.250 [+0.231, +0.269]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.068 [+0.048, +0.087] | -0.183 [-0.209, -0.156] | 0 |
| magnate | +0.111 [+0.088, +0.135] | -0.140 [-0.168, -0.109] | 0 |
| control | +0.102 [+0.079, +0.128] | -0.148 [-0.179, -0.115] | 0 |
| defense | +0.070 [+0.051, +0.090] | -0.180 [-0.209, -0.153] | 0 |
| exploit_run2 | +0.097 [+0.073, +0.122] | -0.154 [-0.181, -0.124] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.250 [-0.269, -0.231] | 2 |
| psro02 | +0.201 [+0.172, +0.231] | -0.050 [-0.082, -0.018] | 0 |
| psro03 | +0.245 [+0.214, +0.278] | -0.005 [-0.042, +0.034] | 1 |
| psro05 | +0.259 [+0.225, +0.293] | +0.009 [-0.024, +0.041] | 0 |
| psro08 | +0.247 [+0.221, +0.273] | -0.004 [-0.017, +0.010] | 0 |

## 5 players

Value of sigma: +0.185 [+0.168, +0.202]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.048 [+0.033, +0.063] | -0.137 [-0.161, -0.113] | 0 |
| magnate | +0.118 [+0.095, +0.143] | -0.067 [-0.098, -0.035] | 0 |
| control | +0.072 [+0.055, +0.092] | -0.113 [-0.137, -0.088] | 0 |
| defense | +0.077 [+0.059, +0.097] | -0.107 [-0.137, -0.079] | 0 |
| exploit_run2 | +0.062 [+0.044, +0.080] | -0.123 [-0.150, -0.096] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.185 [-0.202, -0.168] | 0 |
| psro02 | +0.149 [+0.125, +0.176] | -0.035 [-0.066, -0.003] | 0 |
| psro03 | +0.180 [+0.153, +0.209] | -0.005 [-0.037, +0.029] | 0 |
| psro05 | +0.176 [+0.149, +0.205] | -0.009 [-0.042, +0.026] | 0 |
| psro08 | +0.185 [+0.167, +0.204] | +0.001 [-0.002, +0.003] | 0 |

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| run2 | -0.154 [-0.172, -0.136] | -0.613 [-0.682, -0.545] |
| magnate | -0.107 [-0.127, -0.087] | -0.411 [-0.489, -0.335] |
| control | -0.125 [-0.144, -0.106] | -0.499 [-0.571, -0.427] |
| defense | -0.132 [-0.151, -0.111] | -0.527 [-0.601, -0.447] |
| exploit_run2 | -0.128 [-0.149, -0.109] | -0.518 [-0.594, -0.445] |
| merchant | -0.252 [-0.265, -0.240] | -0.963 [-1.013, -0.917] |
| psro02 | -0.036 [-0.054, -0.018] | -0.149 [-0.222, -0.074] |
| psro03 | -0.009 [-0.032, +0.014] | -0.031 [-0.122, +0.058] |
| psro05 | +0.019 [-0.002, +0.041] | +0.055 [-0.030, +0.139] |
| psro08 | +0.008 [-0.003, +0.020] | +0.024 [-0.010, +0.061] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

# PSRO, holdout re-measure of the response of iteration 03 against the sigma it was trained against

Profile sigma: 3 players: run2 0.00, magnate 0.00, control 0.12, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 0.88; 4 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 1.00; 5 players: run2 0.00, magnate 0.00, control 0.00, defense 0.00, exploit_run2 0.00, merchant 0.00, psro02 1.00. Every other seat draws its strategy iid from sigma. Seeds: 100 per player count from the **holdout** block (offset 900), 3 player counts, every seat of every table, standard length; 10,800 games, 0 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-a7d05e993a75.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.348 [+0.317, +0.379]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.192 [+0.150, +0.237] | -0.157 [-0.204, -0.104] | 0 |
| magnate | +0.252 [+0.207, +0.303] | -0.097 [-0.149, -0.044] | 0 |
| control | +0.232 [+0.182, +0.283] | -0.117 [-0.172, -0.060] | 0 |
| defense | +0.237 [+0.188, +0.287] | -0.112 [-0.169, -0.057] | 0 |
| exploit_run2 | +0.192 [+0.148, +0.235] | -0.157 [-0.210, -0.104] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.348 [-0.379, -0.317] | 0 |
| psro02 | +0.365 [+0.330, +0.400] | +0.017 [+0.009, +0.024] | 0 |
| psro03 | +0.403 [+0.353, +0.457] | +0.055 [-0.001, +0.113] | 0 |
| psro05 | +0.412 [+0.358, +0.467] | +0.063 [+0.002, +0.126] | 0 |

## 4 players

Value of sigma: +0.250 [+0.250, +0.250]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.075 [+0.048, +0.102] | -0.175 [-0.203, -0.147] | 0 |
| magnate | +0.154 [+0.119, +0.190] | -0.096 [-0.131, -0.060] | 0 |
| control | +0.110 [+0.081, +0.141] | -0.140 [-0.169, -0.109] | 0 |
| defense | +0.091 [+0.061, +0.122] | -0.159 [-0.189, -0.128] | 0 |
| exploit_run2 | +0.110 [+0.080, +0.140] | -0.140 [-0.170, -0.110] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.250 [-0.250, -0.250] | 0 |
| psro02 | +0.250 [+0.250, +0.250] | +0.000 [+0.000, +0.000] | 0 |
| psro03 | +0.285 [+0.244, +0.326] | +0.035 [-0.006, +0.076] | 0 |
| psro05 | +0.340 [+0.300, +0.379] | +0.090 [+0.050, +0.129] | 0 |

## 5 players

Value of sigma: +0.200 [+0.200, +0.200]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| run2 | +0.059 [+0.041, +0.080] | -0.141 [-0.159, -0.120] | 0 |
| magnate | +0.151 [+0.120, +0.184] | -0.049 [-0.080, -0.016] | 0 |
| control | +0.076 [+0.052, +0.098] | -0.124 [-0.148, -0.102] | 0 |
| defense | +0.091 [+0.068, +0.115] | -0.109 [-0.132, -0.085] | 0 |
| exploit_run2 | +0.136 [+0.106, +0.166] | -0.064 [-0.094, -0.034] | 0 |
| merchant | +0.000 [+0.000, +0.000] | -0.200 [-0.200, -0.200] | 0 |
| psro02 | +0.200 [+0.200, +0.200] | +0.000 [+0.000, +0.000] | 0 |
| psro03 | +0.209 [+0.173, +0.246] | +0.009 [-0.027, +0.046] | 0 |
| psro05 | +0.238 [+0.204, +0.272] | +0.038 [+0.004, +0.072] | 0 |

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| run2 | -0.158 [-0.178, -0.137] | -0.625 [-0.695, -0.556] |
| magnate | -0.081 [-0.105, -0.057] | -0.307 [-0.395, -0.221] |
| control | -0.127 [-0.150, -0.103] | -0.510 [-0.592, -0.424] |
| defense | -0.126 [-0.149, -0.104] | -0.505 [-0.584, -0.427] |
| exploit_run2 | -0.120 [-0.143, -0.097] | -0.450 [-0.534, -0.365] |
| merchant | -0.266 [-0.276, -0.256] | -1.015 [-1.046, -0.984] |
| psro02 | +0.006 [+0.003, +0.008] | +0.017 [+0.009, +0.024] |
| psro03 | +0.033 [+0.007, +0.060] | +0.117 [+0.017, +0.218] |
| psro05 | +0.064 [+0.036, +0.091] | +0.247 [+0.140, +0.348] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

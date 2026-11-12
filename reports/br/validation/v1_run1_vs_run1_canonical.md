# Oracle validation: v1_run1_vs_run1_canonical (init populations/baselines/run1.pt, profile run1, objective canonical)

Profile sigma: run1 1.00. Every other seat draws its strategy iid from sigma. Seeds: 60 per player count from the **dev** block (offset 100), 3 player counts, every seat of every table, standard length; 6,480 games, 6 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-bf5b3bd5c549.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.306 [+0.244, +0.367]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.306 [-0.367, -0.244] | 0 |
| run1 | +0.306 [+0.244, +0.367] | +0.000 [+0.000, +0.000] | 1 |
| run2 | +0.483 [+0.425, +0.539] | +0.178 [+0.097, +0.258] | 1 |
| exploit_run2 | +0.489 [+0.417, +0.558] | +0.183 [+0.092, +0.272] | 0 |
| control | +0.461 [+0.394, +0.531] | +0.156 [+0.053, +0.250] | 1 |
| defense | +0.542 [+0.469, +0.611] | +0.236 [+0.133, +0.339] | 0 |
| magnate | +0.567 [+0.511, +0.622] | +0.261 [+0.178, +0.350] | 0 |
| v1_run1_vs_run1_canonical/sample * | +0.444 [+0.381, +0.508] | +0.139 [+0.042, +0.231] | 3 |
| v1_run1_vs_run1_canonical/guarded * | +0.439 [+0.372, +0.503] | +0.133 [+0.050, +0.217] | 0 |

Gain by engine seat index (candidates; the first player is drawn at random, so the index is not turn order): v1_run1_vs_run1_canonical/sample: +0.150, +0.133, +0.133; v1_run1_vs_run1_canonical/guarded: +0.158, +0.133, +0.108

NashConv (sum of the seats' gains): v1_run1_vs_run1_canonical/sample: +0.417 [+0.125, +0.692]; v1_run1_vs_run1_canonical/guarded: +0.400 [+0.150, +0.650]

## 4 players

Value of sigma: +0.198 [+0.148, +0.250]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.198 [-0.250, -0.148] | 0 |
| run1 | +0.198 [+0.148, +0.250] | +0.000 [+0.000, +0.000] | 0 |
| run2 | +0.381 [+0.321, +0.444] | +0.183 [+0.115, +0.256] | 0 |
| exploit_run2 | +0.562 [+0.498, +0.627] | +0.365 [+0.281, +0.450] | 0 |
| control | +0.494 [+0.440, +0.548] | +0.296 [+0.225, +0.367] | 0 |
| defense | +0.423 [+0.365, +0.481] | +0.225 [+0.148, +0.298] | 0 |
| magnate | +0.475 [+0.410, +0.540] | +0.277 [+0.196, +0.358] | 0 |
| v1_run1_vs_run1_canonical/sample * | +0.446 [+0.388, +0.504] | +0.248 [+0.173, +0.321] | 0 |
| v1_run1_vs_run1_canonical/guarded * | +0.442 [+0.375, +0.504] | +0.244 [+0.160, +0.321] | 0 |

Gain by engine seat index (candidates; the first player is drawn at random, so the index is not turn order): v1_run1_vs_run1_canonical/sample: +0.233, +0.267, +0.333, +0.158; v1_run1_vs_run1_canonical/guarded: +0.250, +0.200, +0.392, +0.133

NashConv (sum of the seats' gains): v1_run1_vs_run1_canonical/sample: +0.992 [+0.692, +1.284]; v1_run1_vs_run1_canonical/guarded: +0.975 [+0.642, +1.283]

## 5 players

Value of sigma: +0.187 [+0.150, +0.223]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.187 [-0.223, -0.150] | 0 |
| run1 | +0.187 [+0.150, +0.223] | +0.000 [+0.000, +0.000] | 0 |
| run2 | +0.352 [+0.303, +0.398] | +0.165 [+0.112, +0.215] | 0 |
| exploit_run2 | +0.465 [+0.410, +0.517] | +0.278 [+0.220, +0.337] | 0 |
| control | +0.432 [+0.368, +0.493] | +0.245 [+0.170, +0.322] | 0 |
| defense | +0.412 [+0.357, +0.467] | +0.225 [+0.162, +0.290] | 0 |
| magnate | +0.453 [+0.397, +0.512] | +0.267 [+0.195, +0.343] | 0 |
| v1_run1_vs_run1_canonical/sample * | +0.357 [+0.307, +0.410] | +0.170 [+0.103, +0.237] | 0 |
| v1_run1_vs_run1_canonical/guarded * | +0.355 [+0.305, +0.407] | +0.168 [+0.107, +0.227] | 0 |

Gain by engine seat index (candidates; the first player is drawn at random, so the index is not turn order): v1_run1_vs_run1_canonical/sample: +0.150, +0.217, +0.233, +0.050, +0.200; v1_run1_vs_run1_canonical/guarded: +0.133, +0.133, +0.133, +0.167, +0.275

NashConv (sum of the seats' gains): v1_run1_vs_run1_canonical/sample: +0.850 [+0.517, +1.183]; v1_run1_vs_run1_canonical/guarded: +0.842 [+0.533, +1.133]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| merchant | -0.230 [-0.259, -0.201] | -0.881 [-0.992, -0.775] |
| run1 | +0.000 [+0.000, +0.000] | +0.000 [+0.000, +0.000] |
| run2 | +0.175 [+0.137, +0.215] | +0.697 [+0.553, +0.847] |
| exploit_run2 | +0.275 [+0.230, +0.321] | +1.133 [+0.961, +1.306] |
| control | +0.232 [+0.186, +0.280] | +0.958 [+0.783, +1.147] |
| defense | +0.229 [+0.181, +0.274] | +0.911 [+0.733, +1.081] |
| magnate | +0.268 [+0.225, +0.315] | +1.075 [+0.900, +1.256] |
| v1_run1_vs_run1_canonical/sample * | +0.186 [+0.142, +0.233] | +0.753 [+0.583, +0.928] |
| v1_run1_vs_run1_canonical/guarded * | +0.182 [+0.136, +0.227] | +0.739 [+0.561, +0.914] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of v1_run1_vs_run1_canonical/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 60 seeds (it understates near k = 60).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.159 | 0.076 |
| 10 | ±0.113 | 0.055 |
| 25 | ±0.071 | 0.027 |
| 50 | ±0.050 | 0.010 |
| 60 | ±0.046 | 0.000 |

## How well the gain of v1_run1_vs_run1_canonical/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 60 seeds (it understates near k = 60).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.154 | 0.072 |
| 10 | ±0.109 | 0.051 |
| 25 | ±0.069 | 0.027 |
| 50 | ±0.049 | 0.010 |
| 60 | ±0.045 | 0.000 |

# Final best response: holdout confirmation against the refined solution

Profile sigma: 3 players: psro03 0.00, psro05 1.00; 4 players: psro03 0.00, psro05 1.00; 5 players: psro03 1.00, psro05 0.00. Every other seat draws its strategy iid from sigma. Seeds: 300 per player count from the **holdout** block (offset 1000), 3 player counts, every seat of every table, standard length; 10,800 games, 2 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-c5f401aca11b.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.333 [+0.333, +0.333]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| psro03 | +0.289 [+0.261, +0.316] | -0.044 [-0.073, -0.017] | 1 |
| psro05 | +0.333 [+0.333, +0.333] | +0.000 [+0.000, +0.000] | 0 |
| final_br/guarded * | +0.381 [+0.352, +0.409] | +0.047 [+0.019, +0.076] | 1 |

Gain by turn position (candidates; position 0 moves first): final_br/guarded: +0.015, +0.098, +0.028

NashConv (sum of the seats' gains): final_br/guarded: +0.142 [+0.057, +0.227]

## 4 players

Value of sigma: +0.250 [+0.250, +0.250]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| psro03 | +0.222 [+0.200, +0.243] | -0.028 [-0.050, -0.007] | 0 |
| psro05 | +0.250 [+0.250, +0.250] | +0.000 [+0.000, +0.000] | 0 |
| final_br/guarded * | +0.255 [+0.231, +0.279] | +0.005 [-0.019, +0.029] | 0 |

Gain by turn position (candidates; position 0 moves first): final_br/guarded: -0.020, +0.000, -0.027, +0.067

NashConv (sum of the seats' gains): final_br/guarded: +0.020 [-0.077, +0.115]

## 5 players

Value of sigma: +0.200 [+0.200, +0.200]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| psro03 | +0.200 [+0.200, +0.200] | +0.000 [+0.000, +0.000] | 0 |
| psro05 | +0.194 [+0.175, +0.212] | -0.006 [-0.025, +0.012] | 0 |
| final_br/guarded * | +0.213 [+0.193, +0.234] | +0.013 [-0.007, +0.034] | 0 |

Gain by turn position (candidates; position 0 moves first): final_br/guarded: -0.013, +0.010, +0.032, +0.028, +0.008

NashConv (sum of the seats' gains): final_br/guarded: +0.065 [-0.037, +0.172]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| psro03 | -0.024 [-0.036, -0.013] | -0.082 [-0.121, -0.043] |
| psro05 | -0.002 [-0.008, +0.004] | -0.011 [-0.041, +0.020] |
| final_br/guarded * | +0.022 [+0.008, +0.035] | +0.076 [+0.022, +0.128] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of final_br/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 300 seeds (it understates near k = 300).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.110 | 0.056 |
| 10 | ±0.078 | 0.037 |
| 25 | ±0.049 | 0.025 |
| 50 | ±0.035 | 0.016 |
| 100 | ±0.025 | 0.010 |
| 200 | ±0.017 | 0.005 |
| 300 | ±0.014 | 0.000 |

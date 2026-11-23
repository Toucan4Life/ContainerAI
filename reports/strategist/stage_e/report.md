# Stage E: price-lot deviations (development evidence)

One seat of the archetype deviates (the first in seat order) in every baseline game that seats it; each figure is the deviating seat minus the base seat in the same game (same seed, seats and rotation), per table size, never pooled. Baseline: `reports/strategist/egta_h0b` (the new appetite profiles). `DEV` seeds, 400 per composition; cyclic rotations of one arrangement per composition do not cover every permutation. Intervals: 95% percentile bootstrap over seeds (all games of a seed stay together), unadjusted, and for the primary outcome (score) also Bonferroni-adjusted over 21 tests (alpha 0.0024 per test). The grid is exploratory. Thresholds in the verdict are the proposed ones pre-registered in the plan, not approved ones.

Caveats beside every figure: `cash change` is final cash - $10 start (`flow_net_cash`), not profit and not the score (it leaves out island value, leftover containers and loans). `unsold` is exact only because the heuristics make no Bank move and a defaulting seat is left out. `sold` includes resales, so `sold/produced` can exceed 1 (legacy diagnostic). Metrics other than win share cover games completed in both versions; dead and stuck shares cover completed games only (stalled games record no breakdown).

## Summary against the pre-registered rule

| variant | size | score Δ (adj. 95%) | meets practical threshold | sales kept | identity kept | quality kept | all |
|---|---|---|---|---|---|---|---|
| producer (0,1) null | 3 | +0.00 [+0.00, +0.00] | no | yes | yes | yes | no |
| producer (1,1) | 3 | +8.65 [+6.57, +10.50] | yes | no | no | no | no |
| producer (2,1) | 3 | +2.42 [+0.19, +4.52] | yes | no | no | no | no |
| producer (0,2) | 3 | -10.03 [-11.55, -8.55] | no | no | yes | no | no |
| producer (1,2) | 3 | +4.08 [+2.23, +6.09] | yes | no | no | no | no |
| showcase (1,1) null | 3 | +0.00 [+0.00, +0.00] | no | yes | yes | yes | no |
| showcase (2,1) | 3 | -1.31 [-2.14, -0.44] | no | yes | yes | yes | no |
| showcase (1,2) | 3 | -0.26 [-1.31, +0.85] | no | no | no | no | no |
| showcase (2,2) | 3 | -0.56 [-1.78, +0.60] | no | no | no | no | no |
| producer (0,1) null | 4 | +0.00 [+0.00, +0.00] | no | yes | yes | yes | no |
| producer (1,1) | 4 | +14.88 [+13.61, +16.14] | yes | no | no | no | no |
| producer (2,1) | 4 | +11.42 [+10.02, +12.84] | yes | no | no | no | no |
| producer (0,2) | 4 | -3.99 [-4.92, -3.17] | no | no | yes | yes | no |
| producer (1,2) | 4 | +13.61 [+12.19, +15.01] | yes | no | no | no | no |
| showcase (1,1) null | 4 | +0.00 [+0.00, +0.00] | no | yes | yes | yes | no |
| showcase (2,1) | 4 | -2.52 [-3.30, -1.76] | no | yes | yes | yes | no |
| showcase (1,2) | 4 | +2.57 [+1.71, +3.41] | yes | no | no | no | no |
| showcase (2,2) | 4 | +3.17 [+2.21, +4.17] | yes | no | no | no | no |
| producer (0,1) null | 5 | +0.00 [+0.00, +0.00] | no | yes | yes | yes | no |
| producer (1,1) | 5 | +16.94 [+16.08, +17.84] | yes | no | no | no | no |
| producer (2,1) | 5 | +15.40 [+14.50, +16.36] | yes | no | no | no | no |
| producer (0,2) | 5 | -2.02 [-2.71, -1.23] | no | no | yes | yes | no |
| producer (1,2) | 5 | +16.92 [+16.01, +17.79] | yes | no | no | no | no |
| showcase (1,1) null | 5 | +0.00 [+0.00, +0.00] | no | yes | yes | yes | no |
| showcase (2,1) | 5 | +0.08 [-0.51, +0.75] | no | no | yes | yes | no |
| showcase (1,2) | 5 | +4.78 [+4.09, +5.54] | yes | no | no | yes | no |
| showcase (2,2) | 5 | +6.81 [+6.06, +7.57] | yes | no | no | yes | no |

## 3 seats

### producer (0,1) null: 7,200 games, 400 seeds, 7,146 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 62.87 | +0.00 [+0.00, +0.00]; adjusted +0.00 [+0.00, +0.00] |
| cash_change | 6.81 | +0.00 [+0.00, +0.00] |
| win | 0.318 | +0.000 [+0.000, +0.000] |
| sold | 23.91 | +0.00 [+0.00, +0.00] |
| sell_through | 1.350 | +0.000 [+0.000, +0.000] |
| unsold | 2.10 | +0.00 [+0.00, +0.00] |
| produced | 17.71 | +0.00 [+0.00, +0.00] |
| bought | 18.15 | +0.00 [+0.00, +0.00] |
| shipped | 10.97 | +0.00 [+0.00, +0.00] |
| resale | 5.35 | +0.00 [+0.00, +0.00] |
| warehouses_built | 1.40 | +0.00 [+0.00, +0.00] |
| stalled games (%) | 0.75 | +0.00 [+0.00, +0.00] |
| games with a default (%) | 15.58 | +0.00 [+0.00, +0.00] |
| Producer seats dead (%) | 2.34 | +0.00 [+0.00, +0.00] |
| Producer seats stuck (%) | 6.07 | +0.00 [+0.00, +0.00] |
| Shipper seats dead (%) | 3.44 | +0.00 [+0.00, +0.00] |
| Shipper seats stuck (%) | 8.56 | +0.00 [+0.00, +0.00] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 1.5 / 1.5, Sc 13.4 / 13.4, Sh 14.1 / 14.1

### producer (1,1): 7,200 games, 400 seeds, 7,043 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 62.98 | +8.65 [+7.43, +9.90]; adjusted +8.65 [+6.57, +10.50] |
| cash_change | 6.85 | +1.68 [+0.90, +2.45] |
| win | 0.318 | +0.265 [+0.249, +0.282] |
| sold | 23.93 | -2.27 [-2.44, -2.10] |
| sell_through | 1.352 | +0.264 [+0.256, +0.272] |
| unsold | 2.10 | -0.06 [-0.12, +0.00] |
| produced | 17.70 | -4.29 [-4.40, -4.19] |
| bought | 18.19 | +2.82 [+2.62, +3.02] |
| shipped | 10.98 | +0.82 [+0.68, +0.95] |
| resale | 5.37 | +1.95 [+1.83, +2.06] |
| warehouses_built | 1.40 | +0.30 [+0.29, +0.32] |
| stalled games (%) | 0.75 | +0.72 [+0.36, +1.08] |
| games with a default (%) | 15.48 | +6.09 [+4.60, +7.64] |
| Producer seats dead (%) | 2.33 | +0.42 [-0.06, +0.91] |
| Producer seats stuck (%) | 6.00 | +0.43 [-0.32, +1.19] |
| Shipper seats dead (%) | 3.46 | +2.62 [+1.80, +3.46] |
| Shipper seats stuck (%) | 8.56 | +8.97 [+7.73, +10.20] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 3.1 / 1.5, Sc 21.7 / 13.4, Sh 5.6 / 14.1

### producer (2,1): 7,200 games, 400 seeds, 6,916 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 63.11 | +2.42 [+1.05, +3.79]; adjusted +2.42 [+0.19, +4.52] |
| cash_change | 6.90 | +0.27 [-0.59, +1.15] |
| win | 0.318 | +0.175 [+0.157, +0.193] |
| sold | 23.95 | -4.04 [-4.24, -3.85] |
| sell_through | 1.357 | +0.593 [+0.581, +0.604] |
| unsold | 2.08 | -0.09 [-0.17, -0.02] |
| produced | 17.66 | -7.44 [-7.56, -7.33] |
| bought | 18.26 | +3.28 [+3.05, +3.50] |
| shipped | 10.99 | +0.02 [-0.13, +0.16] |
| resale | 5.44 | +3.27 [+3.14, +3.41] |
| warehouses_built | 1.40 | +0.36 [+0.35, +0.38] |
| stalled games (%) | 0.75 | +2.47 [+2.01, +2.97] |
| games with a default (%) | 15.40 | +7.32 [+5.82, +8.79] |
| Producer seats dead (%) | 2.32 | +1.01 [+0.51, +1.51] |
| Producer seats stuck (%) | 5.98 | +1.93 [+1.12, +2.71] |
| Shipper seats dead (%) | 3.52 | +2.99 [+2.12, +3.89] |
| Shipper seats stuck (%) | 8.60 | +6.91 [+5.59, +8.25] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 5.1 / 1.4, Sc 25.9 / 13.3, Sh 4.0 / 14.1

### producer (0,2): 7,200 games, 400 seeds, 7,117 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 62.98 | -10.03 [-11.05, -9.09]; adjusted -10.03 [-11.55, -8.55] |
| cash_change | 6.87 | -3.58 [-4.14, -3.04] |
| win | 0.318 | -0.092 [-0.104, -0.080] |
| sold | 23.91 | -3.22 [-3.34, -3.10] |
| sell_through | 1.349 | -0.213 [-0.218, -0.208] |
| unsold | 2.10 | +0.28 [+0.24, +0.33] |
| produced | 17.72 | +0.49 [+0.41, +0.57] |
| bought | 18.15 | -3.49 [-3.64, -3.36] |
| shipped | 10.98 | +0.00 [-0.10, +0.10] |
| resale | 5.34 | -3.56 [-3.65, -3.48] |
| warehouses_built | 1.40 | +0.00 [-0.00, +0.00] |
| stalled games (%) | 0.75 | +0.06 [-0.19, +0.31] |
| games with a default (%) | 15.48 | +2.19 [+1.07, +3.35] |
| Producer seats dead (%) | 2.31 | +0.24 [-0.20, +0.68] |
| Producer seats stuck (%) | 6.02 | +1.01 [+0.36, +1.66] |
| Shipper seats dead (%) | 3.41 | +1.01 [+0.50, +1.54] |
| Shipper seats stuck (%) | 8.54 | +0.63 [+0.02, +1.22] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 1.2 / 1.4, Sc 13.1 / 13.4, Sh 13.9 / 14.1

### producer (1,2): 7,200 games, 400 seeds, 7,001 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 62.88 | +4.08 [+2.85, +5.35]; adjusted +4.08 [+2.23, +6.09] |
| cash_change | 6.75 | +1.02 [+0.29, +1.77] |
| win | 0.318 | +0.242 [+0.226, +0.258] |
| sold | 23.92 | -5.21 [-5.36, -5.06] |
| sell_through | 1.352 | +0.039 [+0.032, +0.046] |
| unsold | 2.11 | +0.22 [+0.15, +0.29] |
| produced | 17.69 | -4.24 [-4.35, -4.14] |
| bought | 18.18 | +0.19 [+0.01, +0.37] |
| shipped | 10.98 | +0.94 [+0.81, +1.08] |
| resale | 5.37 | -0.97 [-1.07, -0.86] |
| warehouses_built | 1.40 | +0.30 [+0.29, +0.32] |
| stalled games (%) | 0.75 | +1.32 [+0.89, +1.75] |
| games with a default (%) | 15.43 | +8.46 [+6.95, +10.00] |
| Producer seats dead (%) | 2.33 | +0.33 [-0.17, +0.83] |
| Producer seats stuck (%) | 6.06 | +0.94 [+0.15, +1.74] |
| Shipper seats dead (%) | 3.42 | +4.56 [+3.63, +5.46] |
| Shipper seats stuck (%) | 8.44 | +10.94 [+9.71, +12.15] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 2.3 / 1.5, Sc 21.8 / 13.4, Sh 5.5 / 14.1

### showcase (1,1) null: 7,200 games, 400 seeds, 7,055 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 66.95 | +0.00 [+0.00, +0.00]; adjusted +0.00 [+0.00, +0.00] |
| cash_change | 5.91 | +0.00 [+0.00, +0.00] |
| win | 0.355 | +0.000 [+0.000, +0.000] |
| sold | 23.44 | +0.00 [+0.00, +0.00] |
| sell_through | 3.501 | +0.000 [+0.000, +0.000] |
| unsold | 1.12 | +0.00 [+0.00, +0.00] |
| produced | 6.67 | +0.00 [+0.00, +0.00] |
| bought | 26.11 | +0.00 [+0.00, +0.00] |
| shipped | 9.21 | +0.00 [+0.00, +0.00] |
| resale | 15.77 | +0.00 [+0.00, +0.00] |
| warehouses_built | 3.12 | +0.00 [+0.00, +0.00] |
| stalled games (%) | 2.01 | +0.00 [+0.00, +0.00] |
| games with a default (%) | 17.51 | +0.00 [+0.00, +0.00] |
| Producer seats dead (%) | 2.92 | +0.00 [+0.00, +0.00] |
| Producer seats stuck (%) | 8.47 | +0.00 [+0.00, +0.00] |
| Shipper seats dead (%) | 8.73 | +0.00 [+0.00, +0.00] |
| Shipper seats stuck (%) | 18.35 | +0.00 [+0.00, +0.00] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 1.5 / 1.5, Sc 13.4 / 13.4, Sh 14.1 / 14.1

### showcase (2,1): 7,200 games, 400 seeds, 6,946 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 66.98 | -1.31 [-1.88, -0.73]; adjusted -1.31 [-2.14, -0.44] |
| cash_change | 5.99 | -0.48 [-0.90, -0.04] |
| win | 0.355 | +0.035 [+0.020, +0.048] |
| sold | 23.37 | -0.74 [-0.87, -0.60] |
| sell_through | 3.510 | +0.169 [+0.123, +0.216] |
| unsold | 1.13 | +0.24 [+0.20, +0.29] |
| produced | 6.63 | -0.51 [-0.61, -0.41] |
| bought | 26.06 | -0.22 [-0.35, -0.09] |
| shipped | 9.20 | -0.23 [-0.31, -0.14] |
| resale | 15.74 | -0.23 [-0.33, -0.13] |
| warehouses_built | 3.12 | -0.01 [-0.01, -0.00] |
| stalled games (%) | 2.01 | +0.21 [-0.19, +0.62] |
| games with a default (%) | 16.82 | -0.98 [-1.72, -0.26] |
| Producer seats dead (%) | 2.76 | -0.06 [-0.47, +0.34] |
| Producer seats stuck (%) | 8.15 | -0.59 [-1.12, -0.06] |
| Shipper seats dead (%) | 8.55 | -0.98 [-1.87, -0.09] |
| Shipper seats stuck (%) | 17.71 | -1.07 [-2.05, -0.13] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 1.3 / 1.5, Sc 13.6 / 13.3, Sh 13.3 / 14.1

### showcase (1,2): 7,200 games, 400 seeds, 6,808 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 66.69 | -0.26 [-0.95, +0.44]; adjusted -0.26 [-1.31, +0.85] |
| cash_change | 5.89 | -0.31 [-0.80, +0.21] |
| win | 0.355 | +0.022 [+0.005, +0.038] |
| sold | 23.11 | -4.32 [-4.51, -4.12] |
| sell_through | 3.473 | -0.645 [-0.695, -0.598] |
| unsold | 1.13 | +0.69 [+0.64, +0.75] |
| produced | 6.60 | +0.02 [-0.11, +0.15] |
| bought | 25.90 | -3.48 [-3.64, -3.33] |
| shipped | 9.26 | +0.16 [+0.06, +0.26] |
| resale | 15.51 | -4.33 [-4.47, -4.18] |
| warehouses_built | 3.11 | -0.00 [-0.01, +0.01] |
| stalled games (%) | 2.01 | +1.72 [+1.22, +2.22] |
| games with a default (%) | 16.55 | -0.88 [-1.93, +0.15] |
| Producer seats dead (%) | 2.92 | -0.38 [-0.98, +0.17] |
| Producer seats stuck (%) | 8.25 | -1.73 [-2.62, -0.87] |
| Shipper seats dead (%) | 8.45 | +0.36 [-0.76, +1.48] |
| Shipper seats stuck (%) | 17.75 | +0.62 [-0.83, +2.01] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 1.7 / 1.5, Sc 11.1 / 13.3, Sh 15.0 / 14.1

### showcase (2,2): 7,200 games, 400 seeds, 6,829 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 66.86 | -0.56 [-1.29, +0.17]; adjusted -0.56 [-1.78, +0.60] |
| cash_change | 5.93 | -0.47 [-1.04, +0.11] |
| win | 0.355 | +0.055 [+0.039, +0.072] |
| sold | 23.19 | -5.17 [-5.37, -4.96] |
| sell_through | 3.484 | -0.494 [-0.547, -0.440] |
| unsold | 1.13 | +0.94 [+0.88, +1.00] |
| produced | 6.60 | -0.60 [-0.73, -0.47] |
| bought | 25.96 | -3.57 [-3.73, -3.40] |
| shipped | 9.24 | +0.06 [-0.04, +0.17] |
| resale | 15.59 | -4.54 [-4.70, -4.40] |
| warehouses_built | 3.11 | -0.03 [-0.04, -0.02] |
| stalled games (%) | 2.01 | +1.42 [+0.92, +1.93] |
| games with a default (%) | 16.55 | -2.69 [-3.72, -1.68] |
| Producer seats dead (%) | 2.81 | -0.72 [-1.29, -0.15] |
| Producer seats stuck (%) | 8.28 | -2.32 [-3.16, -1.48] |
| Shipper seats dead (%) | 8.36 | -0.98 [-2.02, +0.07] |
| Shipper seats stuck (%) | 17.53 | -1.57 [-2.97, -0.19] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 1.7 / 1.5, Sc 11.3 / 13.3, Sh 14.1 / 14.1

## 4 seats

### producer (0,1) null: 16,000 games, 400 seeds, 16,000 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 92.13 | +0.00 [+0.00, +0.00]; adjusted +0.00 [+0.00, +0.00] |
| cash_change | 26.86 | +0.00 [+0.00, +0.00] |
| win | 0.242 | +0.000 [+0.000, +0.000] |
| sold | 26.42 | +0.00 [+0.00, +0.00] |
| sell_through | 1.320 | +0.000 [+0.000, +0.000] |
| unsold | 1.53 | +0.00 [+0.00, +0.00] |
| produced | 20.01 | +0.00 [+0.00, +0.00] |
| bought | 17.67 | +0.00 [+0.00, +0.00] |
| shipped | 10.78 | +0.00 [+0.00, +0.00] |
| resale | 5.49 | +0.00 [+0.00, +0.00] |
| warehouses_built | 1.20 | +0.00 [+0.00, +0.00] |
| stalled games (%) | 0.00 | +0.00 [+0.00, +0.00] |
| games with a default (%) | 7.33 | +0.00 [+0.00, +0.00] |
| Producer seats dead (%) | 1.37 | +0.00 [+0.00, +0.00] |
| Producer seats stuck (%) | 2.19 | +0.00 [+0.00, +0.00] |
| Shipper seats dead (%) | 2.18 | +0.00 [+0.00, +0.00] |
| Shipper seats stuck (%) | 3.26 | +0.00 [+0.00, +0.00] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 3.5 / 3.5, Sc 16.2 / 16.2, Sh 13.9 / 13.9

### producer (1,1): 16,000 games, 400 seeds, 16,000 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 92.13 | +14.88 [+14.04, +15.71]; adjusted +14.88 [+13.61, +16.14] |
| cash_change | 26.86 | +9.78 [+9.15, +10.41] |
| win | 0.242 | +0.281 [+0.269, +0.292] |
| sold | 26.42 | -3.90 [-4.01, -3.79] |
| sell_through | 1.320 | +0.211 [+0.206, +0.216] |
| unsold | 1.53 | -0.08 [-0.11, -0.05] |
| produced | 20.01 | -5.31 [-5.39, -5.22] |
| bought | 17.67 | +2.53 [+2.42, +2.64] |
| shipped | 10.78 | +1.17 [+1.11, +1.24] |
| resale | 5.49 | +1.36 [+1.28, +1.43] |
| warehouses_built | 1.20 | +0.26 [+0.25, +0.26] |
| stalled games (%) | 0.00 | +0.00 [+0.00, +0.00] |
| games with a default (%) | 7.33 | +3.81 [+3.11, +4.50] |
| Producer seats dead (%) | 1.37 | +0.07 [-0.14, +0.29] |
| Producer seats stuck (%) | 2.19 | +0.11 [-0.18, +0.38] |
| Shipper seats dead (%) | 2.18 | +2.25 [+1.88, +2.62] |
| Shipper seats stuck (%) | 3.26 | +3.73 [+3.29, +4.19] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 3.2 / 3.5, Sc 24.3 / 16.2, Sh 5.7 / 13.9

### producer (2,1): 16,000 games, 400 seeds, 15,998 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 92.12 | +11.42 [+10.52, +12.31]; adjusted +11.42 [+10.02, +12.84] |
| cash_change | 26.86 | +8.57 [+7.84, +9.33] |
| win | 0.242 | +0.249 [+0.238, +0.259] |
| sold | 26.42 | -6.70 [-6.82, -6.57] |
| sell_through | 1.320 | +0.491 [+0.482, +0.500] |
| unsold | 1.53 | +0.57 [+0.54, +0.61] |
| produced | 20.01 | -9.12 [-9.21, -9.04] |
| bought | 17.67 | +3.86 [+3.74, +3.98] |
| shipped | 10.78 | +0.88 [+0.81, +0.95] |
| resale | 5.49 | +2.38 [+2.28, +2.48] |
| warehouses_built | 1.20 | +0.40 [+0.39, +0.40] |
| stalled games (%) | 0.00 | +0.01 [+0.00, +0.03] |
| games with a default (%) | 7.33 | +3.78 [+3.09, +4.48] |
| Producer seats dead (%) | 1.37 | +0.23 [+0.00, +0.46] |
| Producer seats stuck (%) | 2.19 | +0.39 [+0.10, +0.68] |
| Shipper seats dead (%) | 2.18 | +2.04 [+1.67, +2.43] |
| Shipper seats stuck (%) | 3.26 | +3.19 [+2.74, +3.66] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 3.8 / 3.5, Sc 26.0 / 16.2, Sh 4.7 / 13.9

### producer (0,2): 16,000 games, 400 seeds, 15,999 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 92.13 | -3.99 [-4.60, -3.42]; adjusted -3.99 [-4.92, -3.17] |
| cash_change | 26.86 | -1.76 [-2.25, -1.29] |
| win | 0.242 | -0.024 [-0.033, -0.015] |
| sold | 26.42 | -2.06 [-2.14, -1.99] |
| sell_through | 1.320 | -0.144 [-0.147, -0.141] |
| unsold | 1.53 | +0.12 [+0.09, +0.14] |
| produced | 20.01 | +0.70 [+0.64, +0.76] |
| bought | 17.67 | -2.57 [-2.64, -2.50] |
| shipped | 10.78 | +0.09 [+0.05, +0.14] |
| resale | 5.49 | -2.75 [-2.80, -2.69] |
| warehouses_built | 1.20 | +0.00 [-0.00, +0.00] |
| stalled games (%) | 0.00 | +0.01 [+0.00, +0.02] |
| games with a default (%) | 7.33 | +0.93 [+0.41, +1.47] |
| Producer seats dead (%) | 1.37 | +0.17 [-0.02, +0.36] |
| Producer seats stuck (%) | 2.18 | +0.44 [+0.21, +0.68] |
| Shipper seats dead (%) | 2.18 | -0.12 [-0.34, +0.10] |
| Shipper seats stuck (%) | 3.26 | +0.11 [-0.16, +0.38] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 3.2 / 3.5, Sc 16.2 / 16.2, Sh 13.8 / 13.9

### producer (1,2): 16,000 games, 400 seeds, 15,999 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 92.13 | +13.61 [+12.74, +14.46]; adjusted +13.61 [+12.19, +15.01] |
| cash_change | 26.86 | +9.95 [+9.31, +10.60] |
| win | 0.242 | +0.286 [+0.275, +0.297] |
| sold | 26.42 | -5.88 [-5.99, -5.77] |
| sell_through | 1.320 | +0.036 [+0.032, +0.040] |
| unsold | 1.53 | -0.05 [-0.08, -0.02] |
| produced | 20.01 | -4.87 [-4.95, -4.79] |
| bought | 17.67 | +0.27 [+0.17, +0.38] |
| shipped | 10.78 | +1.32 [+1.26, +1.39] |
| resale | 5.49 | -1.04 [-1.11, -0.97] |
| warehouses_built | 1.20 | +0.25 [+0.24, +0.26] |
| stalled games (%) | 0.00 | +0.01 [+0.00, +0.02] |
| games with a default (%) | 7.33 | +5.05 [+4.36, +5.76] |
| Producer seats dead (%) | 1.37 | +0.28 [+0.04, +0.53] |
| Producer seats stuck (%) | 2.19 | +0.32 [+0.03, +0.60] |
| Shipper seats dead (%) | 2.18 | +2.89 [+2.47, +3.33] |
| Shipper seats stuck (%) | 3.26 | +4.79 [+4.32, +5.29] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 2.6 / 3.5, Sc 24.8 / 16.2, Sh 5.7 / 13.9

### showcase (1,1) null: 16,000 games, 400 seeds, 16,000 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 93.27 | +0.00 [+0.00, +0.00]; adjusted +0.00 [+0.00, +0.00] |
| cash_change | 26.90 | +0.00 [+0.00, +0.00] |
| win | 0.210 | +0.000 [+0.000, +0.000] |
| sold | 26.99 | +0.00 [+0.00, +0.00] |
| sell_through | 3.592 | +0.000 [+0.000, +0.000] |
| unsold | 0.85 | +0.00 [+0.00, +0.00] |
| produced | 7.50 | +0.00 [+0.00, +0.00] |
| bought | 29.23 | +0.00 [+0.00, +0.00] |
| shipped | 9.88 | +0.00 [+0.00, +0.00] |
| resale | 18.49 | +0.00 [+0.00, +0.00] |
| warehouses_built | 3.17 | +0.00 [+0.00, +0.00] |
| stalled games (%) | 0.00 | +0.00 [+0.00, +0.00] |
| games with a default (%) | 8.12 | +0.00 [+0.00, +0.00] |
| Producer seats dead (%) | 1.96 | +0.00 [+0.00, +0.00] |
| Producer seats stuck (%) | 3.24 | +0.00 [+0.00, +0.00] |
| Shipper seats dead (%) | 3.29 | +0.00 [+0.00, +0.00] |
| Shipper seats stuck (%) | 5.20 | +0.00 [+0.00, +0.00] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 3.5 / 3.5, Sc 16.2 / 16.2, Sh 13.9 / 13.9

### showcase (2,1): 16,000 games, 400 seeds, 16,000 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 93.27 | -2.52 [-3.04, -2.02]; adjusted -2.52 [-3.30, -1.76] |
| cash_change | 26.90 | -1.82 [-2.32, -1.33] |
| win | 0.210 | +0.022 [+0.013, +0.031] |
| sold | 26.99 | -0.74 [-0.85, -0.63] |
| sell_through | 3.592 | +0.570 [+0.536, +0.605] |
| unsold | 0.85 | +0.32 [+0.29, +0.34] |
| produced | 7.50 | -1.20 [-1.27, -1.14] |
| bought | 29.23 | +0.21 [+0.11, +0.30] |
| shipped | 9.88 | -0.58 [-0.63, -0.53] |
| resale | 18.49 | +0.47 [+0.37, +0.56] |
| warehouses_built | 3.17 | -0.01 [-0.01, -0.00] |
| stalled games (%) | 0.00 | +0.00 [+0.00, +0.00] |
| games with a default (%) | 8.12 | -0.01 [-0.42, +0.42] |
| Producer seats dead (%) | 1.96 | +0.05 [-0.17, +0.28] |
| Producer seats stuck (%) | 3.24 | +0.09 [-0.14, +0.31] |
| Shipper seats dead (%) | 3.29 | -0.16 [-0.47, +0.16] |
| Shipper seats stuck (%) | 5.20 | -0.13 [-0.49, +0.24] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 3.2 / 3.5, Sc 16.7 / 16.2, Sh 12.9 / 13.9

### showcase (1,2): 16,000 games, 400 seeds, 15,997 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 93.27 | +2.57 [+2.01, +3.16]; adjusted +2.57 [+1.71, +3.41] |
| cash_change | 26.90 | +4.10 [+3.54, +4.65] |
| win | 0.210 | +0.057 [+0.046, +0.067] |
| sold | 26.99 | -3.02 [-3.16, -2.88] |
| sell_through | 3.590 | -0.675 [-0.713, -0.638] |
| unsold | 0.85 | +0.03 [+0.01, +0.06] |
| produced | 7.50 | +0.72 [+0.62, +0.82] |
| bought | 29.22 | -3.84 [-3.95, -3.73] |
| shipped | 9.88 | -0.13 [-0.19, -0.06] |
| resale | 18.49 | -3.74 [-3.85, -3.63] |
| warehouses_built | 3.17 | -0.05 [-0.05, -0.04] |
| stalled games (%) | 0.00 | +0.02 [+0.00, +0.04] |
| games with a default (%) | 8.11 | +1.81 [+1.18, +2.41] |
| Producer seats dead (%) | 1.96 | -0.04 [-0.32, +0.23] |
| Producer seats stuck (%) | 3.23 | -0.24 [-0.60, +0.11] |
| Shipper seats dead (%) | 3.29 | +1.54 [+1.14, +1.96] |
| Shipper seats stuck (%) | 5.20 | +2.28 [+1.78, +2.79] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 4.2 / 3.5, Sc 15.2 / 16.2, Sh 14.3 / 13.9

### showcase (2,2): 16,000 games, 400 seeds, 15,999 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 93.27 | +3.17 [+2.54, +3.80]; adjusted +3.17 [+2.21, +4.17] |
| cash_change | 26.89 | +4.21 [+3.63, +4.78] |
| win | 0.210 | +0.084 [+0.074, +0.095] |
| sold | 26.99 | -3.31 [-3.46, -3.17] |
| sell_through | 3.590 | -0.282 [-0.323, -0.239] |
| unsold | 0.85 | +0.26 [+0.23, +0.29] |
| produced | 7.50 | -0.35 [-0.45, -0.24] |
| bought | 29.22 | -3.32 [-3.44, -3.20] |
| shipped | 9.88 | -0.61 [-0.68, -0.55] |
| resale | 18.49 | -2.96 [-3.08, -2.84] |
| warehouses_built | 3.17 | -0.04 [-0.05, -0.03] |
| stalled games (%) | 0.00 | +0.01 [+0.00, +0.02] |
| games with a default (%) | 8.11 | +1.35 [+0.73, +1.96] |
| Producer seats dead (%) | 1.96 | -0.26 [-0.54, +0.01] |
| Producer seats stuck (%) | 3.23 | -0.30 [-0.66, +0.08] |
| Shipper seats dead (%) | 3.29 | +1.06 [+0.66, +1.46] |
| Shipper seats stuck (%) | 5.20 | +1.79 [+1.30, +2.28] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 3.9 / 3.5, Sc 15.6 / 16.2, Sh 13.3 / 13.9

## 5 seats

### producer (0,1) null: 30,000 games, 400 seeds, 30,000 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 105.91 | +0.00 [+0.00, +0.00]; adjusted +0.00 [+0.00, +0.00] |
| cash_change | 37.65 | +0.00 [+0.00, +0.00] |
| win | 0.181 | +0.000 [+0.000, +0.000] |
| sold | 28.19 | +0.00 [+0.00, +0.00] |
| sell_through | 1.337 | +0.000 [+0.000, +0.000] |
| unsold | 1.18 | +0.00 [+0.00, +0.00] |
| produced | 21.09 | +0.00 [+0.00, +0.00] |
| bought | 17.76 | +0.00 [+0.00, +0.00] |
| shipped | 10.50 | +0.00 [+0.00, +0.00] |
| resale | 6.11 | +0.00 [+0.00, +0.00] |
| warehouses_built | 1.14 | +0.00 [+0.00, +0.00] |
| stalled games (%) | 0.00 | +0.00 [+0.00, +0.00] |
| games with a default (%) | 4.63 | +0.00 [+0.00, +0.00] |
| Producer seats dead (%) | 0.75 | +0.00 [+0.00, +0.00] |
| Producer seats stuck (%) | 1.08 | +0.00 [+0.00, +0.00] |
| Shipper seats dead (%) | 1.19 | +0.00 [+0.00, +0.00] |
| Shipper seats stuck (%) | 1.72 | +0.00 [+0.00, +0.00] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 4.6 / 4.6, Sc 17.0 / 17.0, Sh 13.0 / 13.0

### producer (1,1): 30,000 games, 400 seeds, 30,000 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 105.91 | +16.94 [+16.40, +17.51]; adjusted +16.94 [+16.08, +17.84] |
| cash_change | 37.65 | +13.70 [+13.17, +14.25] |
| win | 0.181 | +0.220 [+0.212, +0.228] |
| sold | 28.19 | -5.19 [-5.28, -5.09] |
| sell_through | 1.337 | +0.172 [+0.168, +0.175] |
| unsold | 1.18 | -0.06 [-0.08, -0.04] |
| produced | 21.09 | -5.84 [-5.91, -5.77] |
| bought | 17.76 | +1.85 [+1.79, +1.91] |
| shipped | 10.50 | +1.24 [+1.20, +1.28] |
| resale | 6.11 | +0.65 [+0.60, +0.71] |
| warehouses_built | 1.14 | +0.23 [+0.23, +0.24] |
| stalled games (%) | 0.00 | +0.00 [+0.00, +0.00] |
| games with a default (%) | 4.63 | +1.47 [+1.10, +1.83] |
| Producer seats dead (%) | 0.75 | -0.11 [-0.20, -0.01] |
| Producer seats stuck (%) | 1.08 | -0.12 [-0.23, -0.01] |
| Shipper seats dead (%) | 1.19 | +0.98 [+0.81, +1.13] |
| Shipper seats stuck (%) | 1.72 | +1.42 [+1.22, +1.62] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 4.1 / 4.6, Sc 23.6 / 17.0, Sh 6.9 / 13.0

### producer (2,1): 30,000 games, 400 seeds, 30,000 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 105.91 | +15.40 [+14.82, +16.00]; adjusted +15.40 [+14.50, +16.36] |
| cash_change | 37.65 | +14.08 [+13.49, +14.70] |
| win | 0.181 | +0.242 [+0.235, +0.249] |
| sold | 28.19 | -9.11 [-9.22, -9.00] |
| sell_through | 1.337 | +0.401 [+0.395, +0.407] |
| unsold | 1.18 | +0.57 [+0.54, +0.59] |
| produced | 21.09 | -10.11 [-10.19, -10.04] |
| bought | 17.76 | +2.86 [+2.79, +2.93] |
| shipped | 10.50 | +1.30 [+1.26, +1.34] |
| resale | 6.11 | +1.03 [+0.96, +1.10] |
| warehouses_built | 1.14 | +0.44 [+0.43, +0.45] |
| stalled games (%) | 0.00 | +0.00 [+0.00, +0.00] |
| games with a default (%) | 4.63 | +1.41 [+1.04, +1.78] |
| Producer seats dead (%) | 0.75 | -0.02 [-0.13, +0.08] |
| Producer seats stuck (%) | 1.08 | +0.06 [-0.07, +0.18] |
| Shipper seats dead (%) | 1.19 | +0.64 [+0.49, +0.79] |
| Shipper seats stuck (%) | 1.72 | +1.01 [+0.81, +1.20] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 4.2 / 4.6, Sc 24.5 / 17.0, Sh 6.2 / 13.0

### producer (0,2): 30,000 games, 400 seeds, 30,000 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 105.91 | -2.02 [-2.51, -1.53]; adjusted -2.02 [-2.71, -1.23] |
| cash_change | 37.65 | -0.77 [-1.21, -0.33] |
| win | 0.181 | -0.008 [-0.014, -0.002] |
| sold | 28.19 | -1.71 [-1.78, -1.63] |
| sell_through | 1.337 | -0.124 [-0.126, -0.122] |
| unsold | 1.18 | +0.04 [+0.02, +0.06] |
| produced | 21.09 | +0.75 [+0.69, +0.81] |
| bought | 17.76 | -2.25 [-2.30, -2.20] |
| shipped | 10.50 | +0.17 [+0.14, +0.20] |
| resale | 6.11 | -2.45 [-2.49, -2.41] |
| warehouses_built | 1.14 | +0.00 [+0.00, +0.00] |
| stalled games (%) | 0.00 | +0.00 [+0.00, +0.00] |
| games with a default (%) | 4.63 | +0.43 [+0.14, +0.72] |
| Producer seats dead (%) | 0.75 | +0.07 [-0.02, +0.15] |
| Producer seats stuck (%) | 1.08 | +0.14 [+0.03, +0.24] |
| Shipper seats dead (%) | 1.19 | +0.09 [-0.00, +0.19] |
| Shipper seats stuck (%) | 1.72 | +0.09 [-0.02, +0.20] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 4.4 / 4.6, Sc 17.1 / 17.0, Sh 12.9 / 13.0

### producer (1,2): 30,000 games, 400 seeds, 30,000 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 105.91 | +16.92 [+16.35, +17.51]; adjusted +16.92 [+16.01, +17.79] |
| cash_change | 37.65 | +14.44 [+13.90, +14.99] |
| win | 0.181 | +0.226 [+0.219, +0.234] |
| sold | 28.19 | -6.68 [-6.77, -6.59] |
| sell_through | 1.337 | +0.021 [+0.018, +0.024] |
| unsold | 1.18 | -0.04 [-0.06, -0.02] |
| produced | 21.09 | -5.25 [-5.32, -5.18] |
| bought | 17.76 | -0.03 [-0.09, +0.03] |
| shipped | 10.50 | +1.43 [+1.39, +1.47] |
| resale | 6.11 | -1.43 [-1.48, -1.38] |
| warehouses_built | 1.14 | +0.23 [+0.23, +0.24] |
| stalled games (%) | 0.00 | +0.00 [+0.00, +0.00] |
| games with a default (%) | 4.63 | +1.69 [+1.30, +2.07] |
| Producer seats dead (%) | 0.75 | -0.06 [-0.17, +0.04] |
| Producer seats stuck (%) | 1.08 | -0.08 [-0.21, +0.04] |
| Shipper seats dead (%) | 1.19 | +1.01 [+0.85, +1.17] |
| Shipper seats stuck (%) | 1.72 | +1.51 [+1.32, +1.70] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 3.5 / 4.6, Sc 24.1 / 17.0, Sh 6.9 / 13.0

### showcase (1,1) null: 30,000 games, 400 seeds, 30,000 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 101.74 | +0.00 [+0.00, +0.00]; adjusted +0.00 [+0.00, +0.00] |
| cash_change | 36.57 | +0.00 [+0.00, +0.00] |
| win | 0.159 | +0.000 [+0.000, +0.000] |
| sold | 27.44 | +0.00 [+0.00, +0.00] |
| sell_through | 3.923 | +0.000 [+0.000, +0.000] |
| unsold | 0.75 | +0.00 [+0.00, +0.00] |
| produced | 6.99 | +0.00 [+0.00, +0.00] |
| bought | 29.99 | +0.00 [+0.00, +0.00] |
| shipped | 9.79 | +0.00 [+0.00, +0.00] |
| resale | 19.45 | +0.00 [+0.00, +0.00] |
| warehouses_built | 3.23 | +0.00 [+0.00, +0.00] |
| stalled games (%) | 0.00 | +0.00 [+0.00, +0.00] |
| games with a default (%) | 4.91 | +0.00 [+0.00, +0.00] |
| Producer seats dead (%) | 1.03 | +0.00 [+0.00, +0.00] |
| Producer seats stuck (%) | 1.47 | +0.00 [+0.00, +0.00] |
| Shipper seats dead (%) | 1.60 | +0.00 [+0.00, +0.00] |
| Shipper seats stuck (%) | 2.37 | +0.00 [+0.00, +0.00] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 4.6 / 4.6, Sc 17.0 / 17.0, Sh 13.0 / 13.0

### showcase (2,1): 30,000 games, 400 seeds, 30,000 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 101.74 | +0.08 [-0.32, +0.49]; adjusted +0.08 [-0.51, +0.75] |
| cash_change | 36.57 | +0.45 [+0.08, +0.85] |
| win | 0.159 | +0.023 [+0.017, +0.029] |
| sold | 27.44 | -1.16 [-1.25, -1.07] |
| sell_through | 3.922 | +0.691 [+0.661, +0.721] |
| unsold | 0.75 | +0.29 [+0.28, +0.31] |
| produced | 6.99 | -1.30 [-1.35, -1.25] |
| bought | 29.99 | +0.27 [+0.20, +0.34] |
| shipped | 9.79 | -0.17 [-0.20, -0.14] |
| resale | 19.45 | +0.14 [+0.07, +0.21] |
| warehouses_built | 3.23 | -0.01 [-0.01, -0.00] |
| stalled games (%) | 0.00 | +0.00 [+0.00, +0.00] |
| games with a default (%) | 4.91 | +0.25 [+0.01, +0.49] |
| Producer seats dead (%) | 1.03 | +0.03 [-0.07, +0.14] |
| Producer seats stuck (%) | 1.47 | +0.10 [-0.00, +0.20] |
| Shipper seats dead (%) | 1.60 | +0.15 [+0.00, +0.29] |
| Shipper seats stuck (%) | 2.37 | +0.08 [-0.09, +0.25] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 4.4 / 4.6, Sc 17.4 / 17.0, Sh 12.6 / 13.0

### showcase (1,2): 30,000 games, 400 seeds, 30,000 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 101.74 | +4.78 [+4.30, +5.28]; adjusted +4.78 [+4.09, +5.54] |
| cash_change | 36.57 | +5.09 [+4.62, +5.58] |
| win | 0.159 | +0.045 [+0.038, +0.051] |
| sold | 27.44 | -2.74 [-2.85, -2.63] |
| sell_through | 3.922 | -0.824 [-0.852, -0.797] |
| unsold | 0.75 | +0.02 [+0.00, +0.03] |
| produced | 6.99 | +0.98 [+0.91, +1.05] |
| bought | 29.99 | -3.70 [-3.78, -3.61] |
| shipped | 9.79 | +0.00 [-0.04, +0.05] |
| resale | 19.45 | -3.72 [-3.80, -3.64] |
| warehouses_built | 3.23 | -0.07 [-0.08, -0.07] |
| stalled games (%) | 0.00 | +0.00 [+0.00, +0.00] |
| games with a default (%) | 4.91 | +0.49 [+0.15, +0.82] |
| Producer seats dead (%) | 1.03 | -0.05 [-0.18, +0.08] |
| Producer seats stuck (%) | 1.47 | -0.04 [-0.19, +0.11] |
| Shipper seats dead (%) | 1.60 | +0.30 [+0.13, +0.47] |
| Shipper seats stuck (%) | 2.37 | +0.41 [+0.21, +0.62] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 5.3 / 4.6, Sc 16.2 / 17.0, Sh 13.3 / 13.0

### showcase (2,2): 30,000 games, 400 seeds, 30,000 completed in both

| quantity | base seat | deviating − base [95%] |
|---|---|---|
| score | 101.74 | +6.81 [+6.32, +7.33]; adjusted +6.81 [+6.06, +7.57] |
| cash_change | 36.57 | +7.01 [+6.52, +7.51] |
| win | 0.159 | +0.076 [+0.069, +0.083] |
| sold | 27.44 | -3.82 [-3.93, -3.70] |
| sell_through | 3.922 | -0.364 [-0.394, -0.333] |
| unsold | 0.75 | +0.30 [+0.28, +0.32] |
| produced | 6.99 | -0.35 [-0.42, -0.28] |
| bought | 29.99 | -3.16 [-3.25, -3.06] |
| shipped | 9.79 | +0.01 [-0.04, +0.06] |
| resale | 19.45 | -3.47 [-3.56, -3.38] |
| warehouses_built | 3.23 | -0.08 [-0.08, -0.07] |
| stalled games (%) | 0.00 | +0.00 [+0.00, +0.00] |
| games with a default (%) | 4.91 | +0.39 [+0.07, +0.70] |
| Producer seats dead (%) | 1.03 | -0.07 [-0.19, +0.05] |
| Producer seats stuck (%) | 1.47 | -0.05 [-0.20, +0.10] |
| Shipper seats dead (%) | 1.60 | +0.29 [+0.11, +0.46] |
| Shipper seats stuck (%) | 2.37 | +0.38 [+0.17, +0.57] |

Resale per seat-game in tables of all three archetypes (variant run / baseline): P 5.2 / 4.6, Sc 16.5 / 17.0, Sh 13.0 / 13.0

## Stall baseline (two panels, never in one column)

**Panel 1, EGTA cross-play of the three heuristics** (`reports/strategist/egta_h0b`, no nets, every composition and rotation, 400 seeds, horizon 2,500 actions). Rates are comparable within this panel only.

| table size | composition | games | stalled | stalled share |
|---|---|---|---|---|
| 3 | all compositions | 12,000 | 196 | 1.63% |
| 3 | P P P | 1,200 | 9 | 0.75% |
| 3 | P P Sc | 1,200 | 4 | 0.33% |
| 3 | P P Sh | 1,200 | 6 | 0.50% |
| 3 | P Sc Sc | 1,200 | 11 | 0.92% |
| 3 | P Sc Sh | 1,200 | 21 | 1.75% |
| 3 | P Sh Sh | 1,200 | 3 | 0.25% |
| 3 | Sc Sc Sh | 1,200 | 34 | 2.83% |
| 3 | Sc Sh Sh | 1,200 | 75 | 6.25% |
| 3 | Sh Sh Sh | 1,200 | 33 | 2.75% |
| 4 | all compositions | 24,000 | 0 | 0.00% |
| 5 | all compositions | 42,000 | 0 | 0.00% |

**Panel 2, `interact`**: each row plays 300 games with the same seed (1), the same horizon (2,500) and the same table size; the seats not of interest are drawn at random from the pool, so these rates are not comparable with panel 1. A stalled game stalls for every seat, so the rate of the seat of interest is the game stall rate. Command: `interact [--focus H] --pool P --players N --games 300 --seed 1`.

| table size | seat of interest | other seats drawn from | games | horizon | seed | games stalled |
|---|---|---|---|---|---|---|
| 3 | all seats | classic (all seats) | 300 | 2500 | 1 | 79.0% |
| 3 | producer | classic | 300 | 2500 | 1 | 53.3% |
| 3 | shipper | classic | 300 | 2500 | 1 | 57.0% |
| 3 | showcase | classic | 300 | 2500 | 1 | 51.3% |
| 4 | all seats | classic (all seats) | 300 | 2500 | 1 | 0.3% |
| 4 | producer | classic | 300 | 2500 | 1 | 1.7% |
| 4 | shipper | classic | 300 | 2500 | 1 | 3.7% |
| 4 | showcase | classic | 300 | 2500 | 1 | 0.7% |
| 4 | producer | consortium | 300 | 2500 | 1 | 8.3% |
| 4 | producer | magnate | 300 | 2500 | 1 | 2.7% |
| 5 | all seats | classic (all seats) | 300 | 2500 | 1 | 16.7% |
| 5 | producer | classic | 300 | 2500 | 1 | 5.0% |
| 5 | shipper | classic | 300 | 2500 | 1 | 1.0% |
| 5 | showcase | classic | 300 | 2500 | 1 | 26.7% |

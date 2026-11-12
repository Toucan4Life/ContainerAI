# Milestone, dev study: m4_broad_s4 (init defense_run2_vs_exploiter_30m, profile run2,magnate,control,defense,exploit_run2,merchant)

Profile sigma: run2 0.17, magnate 0.17, control 0.17, defense 0.17, exploit_run2 0.17, merchant 0.17. Every other seat draws its strategy iid from sigma. Seeds: 60 per player count from the **dev** block (offset 100), 3 player counts, every seat of every table, standard length; 12,240 games, 9 stalled (horizon 2500, stalled games pay 0). Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-e1926b48b863.

Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on the true deviation gain.

## 3 players

Value of sigma: +0.369 [+0.337, +0.403]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.017 [+0.000, +0.039] | -0.352 [-0.386, -0.321] | 0 |
| run1 | +0.247 [+0.186, +0.306] | -0.122 [-0.184, -0.062] | 1 |
| run2 | +0.389 [+0.311, +0.461] | +0.020 [-0.044, +0.081] | 0 |
| exploit_run2 | +0.411 [+0.344, +0.478] | +0.042 [-0.013, +0.097] | 1 |
| control | +0.433 [+0.361, +0.506] | +0.064 [-0.000, +0.129] | 0 |
| defense | +0.489 [+0.431, +0.550] | +0.120 [+0.078, +0.165] | 0 |
| magnate | +0.475 [+0.408, +0.542] | +0.106 [+0.054, +0.163] | 0 |
| m4_broad_s4@5m/sample * | +0.369 [+0.294, +0.447] | +0.000 [-0.075, +0.076] | 0 |
| m4_broad_s4@5m/guarded * | +0.364 [+0.300, +0.428] | -0.005 [-0.061, +0.050] | 0 |
| m4_broad_s4@11m/sample * | +0.411 [+0.344, +0.478] | +0.042 [-0.016, +0.097] | 4 |
| m4_broad_s4@11m/guarded * | +0.406 [+0.339, +0.478] | +0.037 [-0.026, +0.102] | 0 |
| m4_broad_s4@16m/sample * | +0.344 [+0.289, +0.400] | -0.025 [-0.088, +0.037] | 0 |
| m4_broad_s4@16m/guarded * | +0.383 [+0.317, +0.456] | +0.014 [-0.050, +0.086] | 0 |
| m4_broad_s4@22m/sample * | +0.392 [+0.328, +0.458] | +0.023 [-0.048, +0.097] | 1 |
| m4_broad_s4@22m/guarded * | +0.403 [+0.344, +0.469] | +0.034 [-0.025, +0.099] | 0 |
| m4_broad_s4/sample * | +0.375 [+0.311, +0.439] | +0.006 [-0.051, +0.067] | 2 |
| m4_broad_s4/guarded * | +0.411 [+0.339, +0.483] | +0.042 [-0.017, +0.105] | 0 |

Gain by turn position (candidates; position 0 moves first): m4_broad_s4@5m/sample: +0.004, -0.004, +0.001; m4_broad_s4@5m/guarded: -0.071, +0.054, +0.001; m4_broad_s4@11m/sample: -0.004, +0.096, +0.035; m4_broad_s4@11m/guarded: +0.096, +0.046, -0.032; m4_broad_s4@16m/sample: -0.087, +0.046, -0.032; m4_broad_s4@16m/guarded: +0.013, +0.013, +0.018; m4_broad_s4@22m/sample: +0.029, +0.054, -0.015; m4_broad_s4@22m/guarded: +0.046, +0.104, -0.049; m4_broad_s4/sample: -0.046, +0.096, -0.032; m4_broad_s4/guarded: +0.013, +0.079, +0.035

NashConv (sum of the seats' gains): m4_broad_s4@5m/sample: +0.001 [-0.226, +0.229]; m4_broad_s4@5m/guarded: -0.015 [-0.183, +0.151]; m4_broad_s4@11m/sample: +0.126 [-0.047, +0.292]; m4_broad_s4@11m/guarded: +0.110 [-0.079, +0.307]; m4_broad_s4@16m/sample: -0.074 [-0.265, +0.110]; m4_broad_s4@16m/guarded: +0.043 [-0.149, +0.257]; m4_broad_s4@22m/sample: +0.068 [-0.143, +0.290]; m4_broad_s4@22m/guarded: +0.101 [-0.076, +0.296]; m4_broad_s4/sample: +0.018 [-0.153, +0.200]; m4_broad_s4/guarded: +0.126 [-0.050, +0.314]

## 4 players

Value of sigma: +0.262 [+0.236, +0.288]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.262 [-0.288, -0.236] | 0 |
| run1 | +0.100 [+0.067, +0.133] | -0.162 [-0.202, -0.124] | 0 |
| run2 | +0.287 [+0.242, +0.331] | +0.025 [-0.013, +0.063] | 0 |
| exploit_run2 | +0.247 [+0.192, +0.298] | -0.015 [-0.065, +0.030] | 0 |
| control | +0.301 [+0.245, +0.360] | +0.039 [-0.002, +0.085] | 0 |
| defense | +0.319 [+0.258, +0.377] | +0.056 [+0.007, +0.105] | 0 |
| magnate | +0.419 [+0.360, +0.479] | +0.156 [+0.109, +0.206] | 0 |
| m4_broad_s4@5m/sample * | +0.304 [+0.250, +0.362] | +0.042 [-0.020, +0.106] | 0 |
| m4_broad_s4@5m/guarded * | +0.388 [+0.333, +0.444] | +0.125 [+0.072, +0.178] | 0 |
| m4_broad_s4@11m/sample * | +0.244 [+0.194, +0.300] | -0.019 [-0.068, +0.040] | 0 |
| m4_broad_s4@11m/guarded * | +0.325 [+0.269, +0.385] | +0.063 [+0.006, +0.119] | 0 |
| m4_broad_s4@16m/sample * | +0.283 [+0.229, +0.340] | +0.021 [-0.044, +0.086] | 0 |
| m4_broad_s4@16m/guarded * | +0.327 [+0.271, +0.385] | +0.065 [+0.007, +0.126] | 0 |
| m4_broad_s4@22m/sample * | +0.279 [+0.225, +0.338] | +0.017 [-0.044, +0.082] | 0 |
| m4_broad_s4@22m/guarded * | +0.294 [+0.244, +0.342] | +0.031 [-0.025, +0.084] | 0 |
| m4_broad_s4/sample * | +0.267 [+0.215, +0.321] | +0.004 [-0.050, +0.061] | 0 |
| m4_broad_s4/guarded * | +0.375 [+0.312, +0.438] | +0.113 [+0.053, +0.173] | 0 |

Gain by turn position (candidates; position 0 moves first): m4_broad_s4@5m/sample: +0.078, +0.080, -0.057, +0.067; m4_broad_s4@5m/guarded: +0.153, +0.121, +0.076, +0.150; m4_broad_s4@11m/sample: -0.014, -0.004, -0.057, +0.000; m4_broad_s4@11m/guarded: +0.011, +0.038, +0.051, +0.150; m4_broad_s4@16m/sample: +0.044, -0.037, +0.026, +0.050; m4_broad_s4@16m/guarded: +0.103, +0.046, +0.060, +0.050; m4_broad_s4@22m/sample: -0.022, +0.030, -0.057, +0.117; m4_broad_s4@22m/guarded: -0.006, +0.013, +0.026, +0.092; m4_broad_s4/sample: -0.097, +0.030, +0.051, +0.034; m4_broad_s4/guarded: +0.094, +0.163, +0.143, +0.050

NashConv (sum of the seats' gains): m4_broad_s4@5m/sample: +0.168 [-0.079, +0.424]; m4_broad_s4@5m/guarded: +0.501 [+0.289, +0.713]; m4_broad_s4@11m/sample: -0.074 [-0.271, +0.159]; m4_broad_s4@11m/guarded: +0.251 [+0.026, +0.476]; m4_broad_s4@16m/sample: +0.084 [-0.174, +0.346]; m4_broad_s4@16m/guarded: +0.259 [+0.026, +0.504]; m4_broad_s4@22m/sample: +0.068 [-0.176, +0.327]; m4_broad_s4@22m/guarded: +0.126 [-0.099, +0.335]; m4_broad_s4/sample: +0.018 [-0.200, +0.245]; m4_broad_s4/guarded: +0.451 [+0.210, +0.693]

## 5 players

Value of sigma: +0.203 [+0.184, +0.223]

| focal | utility | gain over sigma | stalled |
|---|---|---|---|
| merchant | +0.000 [+0.000, +0.000] | -0.203 [-0.223, -0.184] | 0 |
| run1 | +0.080 [+0.050, +0.113] | -0.123 [-0.156, -0.087] | 0 |
| run2 | +0.182 [+0.138, +0.227] | -0.021 [-0.059, +0.019] | 0 |
| exploit_run2 | +0.168 [+0.125, +0.217] | -0.034 [-0.076, +0.009] | 0 |
| control | +0.283 [+0.233, +0.337] | +0.081 [+0.036, +0.127] | 0 |
| defense | +0.275 [+0.227, +0.325] | +0.072 [+0.029, +0.115] | 0 |
| magnate | +0.308 [+0.258, +0.360] | +0.106 [+0.063, +0.149] | 0 |
| m4_broad_s4@5m/sample * | +0.188 [+0.145, +0.237] | -0.014 [-0.055, +0.030] | 0 |
| m4_broad_s4@5m/guarded * | +0.290 [+0.243, +0.340] | +0.087 [+0.040, +0.137] | 0 |
| m4_broad_s4@11m/sample * | +0.225 [+0.178, +0.273] | +0.022 [-0.031, +0.076] | 0 |
| m4_broad_s4@11m/guarded * | +0.287 [+0.235, +0.342] | +0.084 [+0.029, +0.141] | 0 |
| m4_broad_s4@16m/sample * | +0.202 [+0.160, +0.248] | -0.001 [-0.046, +0.046] | 0 |
| m4_broad_s4@16m/guarded * | +0.268 [+0.220, +0.322] | +0.066 [+0.018, +0.118] | 0 |
| m4_broad_s4@22m/sample * | +0.200 [+0.157, +0.247] | -0.003 [-0.048, +0.044] | 0 |
| m4_broad_s4@22m/guarded * | +0.232 [+0.185, +0.280] | +0.029 [-0.023, +0.083] | 0 |
| m4_broad_s4/sample * | +0.207 [+0.160, +0.253] | +0.004 [-0.044, +0.052] | 0 |
| m4_broad_s4/guarded * | +0.277 [+0.222, +0.337] | +0.074 [+0.018, +0.132] | 0 |

Gain by turn position (candidates; position 0 moves first): m4_broad_s4@5m/sample: -0.032, +0.050, -0.044, -0.012, -0.033; m4_broad_s4@5m/guarded: +0.168, +0.058, +0.172, +0.038, +0.000; m4_broad_s4@11m/sample: +0.051, +0.075, -0.036, +0.029, -0.008; m4_broad_s4@11m/guarded: +0.176, +0.058, -0.028, +0.171, +0.042; m4_broad_s4@16m/sample: -0.032, -0.042, +0.039, -0.046, +0.075; m4_broad_s4@16m/guarded: +0.176, -0.058, +0.056, +0.171, -0.017; m4_broad_s4@22m/sample: +0.001, -0.075, +0.022, +0.087, -0.050; m4_broad_s4@22m/guarded: +0.001, +0.042, +0.039, +0.038, +0.025; m4_broad_s4/sample: +0.068, -0.067, +0.022, +0.063, -0.067; m4_broad_s4/guarded: +0.126, -0.017, +0.056, +0.121, +0.083

NashConv (sum of the seats' gains): m4_broad_s4@5m/sample: -0.072 [-0.275, +0.151]; m4_broad_s4@5m/guarded: +0.436 [+0.200, +0.685]; m4_broad_s4@11m/sample: +0.111 [-0.157, +0.378]; m4_broad_s4@11m/guarded: +0.419 [+0.143, +0.704]; m4_broad_s4@16m/sample: -0.006 [-0.231, +0.228]; m4_broad_s4@16m/guarded: +0.328 [+0.089, +0.589]; m4_broad_s4@22m/sample: -0.014 [-0.242, +0.219]; m4_broad_s4@22m/guarded: +0.144 [-0.114, +0.414]; m4_broad_s4/sample: +0.019 [-0.218, +0.260]; m4_broad_s4/guarded: +0.369 [+0.088, +0.661]

## Mean over player counts

| focal | gain over sigma | NashConv |
|---|---|---|
| merchant | -0.272 [-0.288, -0.257] | -1.040 [-1.098, -0.981] |
| run1 | -0.136 [-0.164, -0.109] | -0.543 [-0.645, -0.444] |
| run2 | +0.008 [-0.021, +0.034] | +0.018 [-0.090, +0.114] |
| exploit_run2 | -0.002 [-0.029, +0.026] | -0.035 [-0.138, +0.078] |
| control | +0.061 [+0.032, +0.090] | +0.251 [+0.137, +0.363] |
| defense | +0.083 [+0.057, +0.110] | +0.316 [+0.210, +0.426] |
| magnate | +0.123 [+0.094, +0.154] | +0.491 [+0.381, +0.608] |
| m4_broad_s4@5m/sample * | +0.009 [-0.027, +0.045] | +0.032 [-0.102, +0.167] |
| m4_broad_s4@5m/guarded * | +0.069 [+0.038, +0.101] | +0.307 [+0.180, +0.435] |
| m4_broad_s4@11m/sample * | +0.015 [-0.017, +0.047] | +0.054 [-0.074, +0.183] |
| m4_broad_s4@11m/guarded * | +0.061 [+0.028, +0.094] | +0.260 [+0.124, +0.389] |
| m4_broad_s4@16m/sample * | -0.002 [-0.037, +0.033] | +0.002 [-0.131, +0.138] |
| m4_broad_s4@16m/guarded * | +0.048 [+0.014, +0.085] | +0.210 [+0.070, +0.347] |
| m4_broad_s4@22m/sample * | +0.012 [-0.023, +0.047] | +0.041 [-0.093, +0.172] |
| m4_broad_s4@22m/guarded * | +0.031 [-0.002, +0.064] | +0.124 [-0.007, +0.253] |
| m4_broad_s4/sample * | +0.005 [-0.027, +0.036] | +0.018 [-0.107, +0.143] |
| m4_broad_s4/guarded * | +0.076 [+0.041, +0.111] | +0.316 [+0.175, +0.455] |

`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.

## How well the gain of m4_broad_s4/sample is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 60 seeds (it understates near k = 60).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.107 | 0.056 |
| 10 | ±0.076 | 0.034 |
| 25 | ±0.048 | 0.019 |
| 50 | ±0.034 | 0.007 |
| 60 | ±0.031 | 0.000 |

## How well the gain of m4_broad_s4/guarded is measured

Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance predicts for k seeds per player count; the spread is the standard deviation of the estimate over random subsets of k of the study's 60 seeds (it understates near k = 60).

| seeds per player count | predicted 95% half-width | spread of the estimate |
|---|---|---|
| 5 | ±0.120 | 0.058 |
| 10 | ±0.085 | 0.037 |
| 25 | ±0.054 | 0.020 |
| 50 | ±0.038 | 0.008 |
| 60 | ±0.035 | 0.000 |

# PSRO, closing audit on the holdout block

Population (9): run2, magnate, control, defense, exploit_run2, merchant, psro02, psro03, psro05. 7 iterations finished; responses added: psro02, psro03, psro05; iterations whose response was rejected: 1, 4, 6, 7. Utility: winner share (a stalled game pays 0). Each study uses 150 holdout seeds per player count (3, 4, 5 players, standard length) at seed offsets 200 and 350; every look at the holdout block is in `reports/holdout_log.jsonl`. A gain measured here is a lower bound on the exploitability of the profile that was attacked: the oracle is only an approximate best response and the deviations are limited to the strategies listed. Nothing here is a statement about the full game.

## 1. The meta-strategy that iteration 7 attacked

sigma: 3 players: psro03 0.20, psro05 0.80; 4 players: psro03 0.64, psro05 0.35; 5 players: psro03 1.00.

| focal | gain over the profile | NashConv (n x gain, mean over n) |
|---|---|---|
| iter07/guarded (trained, not in the population) | +0.027 [+0.005, +0.049] | +0.106 [+0.022, +0.192] |
| iter06/guarded (trained, not in the population) | +0.015 [-0.005, +0.036] | +0.059 [-0.015, +0.141] |
| iter04/guarded (trained, not in the population) | +0.013 [-0.009, +0.033] | +0.029 [-0.052, +0.106] |
| iter06/sample (trained, not in the population) | +0.009 [-0.014, +0.032] | +0.032 [-0.054, +0.118] |
| iter07/sample (trained, not in the population) | +0.008 [-0.014, +0.029] | +0.035 [-0.049, +0.114] |
| iter04/sample (trained, not in the population) | +0.005 [-0.016, +0.026] | +0.018 [-0.061, +0.099] |

The focal that looks best is optimistic by having been picked; picked on one half of the seeds and measured on the other (`holdout_sigma_last_selection.md`):

| picked among | best on all seeds (optimistic) | picked on even seeds, measured on odd | picked on odd seeds, measured on even | gain of the pick, measured on the seeds it was not picked on |
|---|---|---|---|---|
| every focal (17) | iter07/guarded +0.027 [+0.005, +0.049] | iter07/guarded | iter07/guarded | +0.027 [+0.004, +0.049] |
| trained candidates (8) | iter07/guarded +0.027 [+0.005, +0.049] | iter07/guarded | iter07/guarded | +0.027 [+0.004, +0.049] |
| strategies of the population (9) | psro05 +0.005 [-0.009, +0.018] | psro05 | psro05 | +0.005 [-0.008, +0.017] |

Full study: `holdout_sigma_last.md`.

## 2. The solution of the final restricted game

sigma: 3 players: psro03 0.20, psro05 0.80; 4 players: psro03 0.64, psro05 0.35; 5 players: psro03 1.00.

Solver, from the cross-play cells of the final population: 3 players: nash_conv_minimizer, regret on the simulated cells 5.6e-17, noise floor of the cells (bootstrap) mean 0.022, 95th percentile 0.070; 4 players: nash_conv_minimizer, regret on the simulated cells 5.6e-15, noise floor of the cells (bootstrap) mean 0.019, 95th percentile 0.051; 5 players: nash_conv_minimizer, regret on the simulated cells 0.0e+00, noise floor of the cells (bootstrap) mean 0.012, 95th percentile 0.062. These gains are exact in the empirical game and about 0 by construction; the study below re-measures them on holdout seeds, so what it finds is the estimation error of the cells plus the real deviation gain inside the population.

| focal | gain over the profile | NashConv (n x gain, mean over n) |
|---|---|---|
| psro05 | +0.017 [+0.004, +0.031] | +0.066 [+0.004, +0.128] |
| psro03 | -0.023 [-0.038, -0.008] | -0.076 [-0.122, -0.030] |
| psro02 | -0.036 [-0.059, -0.013] | -0.127 [-0.211, -0.041] |
| magnate | -0.101 [-0.122, -0.080] | -0.397 [-0.476, -0.317] |
| exploit_run2 | -0.133 [-0.151, -0.115] | -0.522 [-0.588, -0.452] |
| defense | -0.137 [-0.156, -0.117] | -0.539 [-0.608, -0.469] |

Picked on one half of the seeds, measured on the other (`holdout_sigma_final_selection.md`):

| picked among | best on all seeds (optimistic) | picked on even seeds, measured on odd | picked on odd seeds, measured on even | gain of the pick, measured on the seeds it was not picked on |
|---|---|---|---|---|
| every focal (9) | psro05 +0.017 [+0.004, +0.031] | psro05 | psro05 | +0.017 [+0.004, +0.031] |
| strategies of the population (9) | psro05 +0.017 [+0.004, +0.031] | psro05 | psro05 | +0.017 [+0.004, +0.031] |

Full study: `holdout_sigma_final.md`. The population only contains what was trained so far, so a small gain here says nothing about strategies outside it.

## 3. Each response against the best member of the population it was measured against (dev seeds, paired)

| iteration | decision | response | its gain over sigma | best member of the population | its gain | response minus member (paired) |
|---|---|---|---|---|---|---|
| 1 | rejected | psro01/guarded | +0.006 [-0.019, +0.032] | magnate | +0.017 [+0.001, +0.033] | -0.011 [-0.038, +0.017] |
| 2 | added | psro02/guarded | +0.098 [+0.070, +0.126] | magnate | +0.018 [+0.002, +0.035] | +0.080 [+0.051, +0.108] |
| 3 | added | psro03/guarded | +0.032 [+0.005, +0.059] | psro02 | +0.005 [+0.003, +0.007] | +0.027 [-0.000, +0.055] |
| 4 | rejected | psro04/sample | +0.030 [-0.002, +0.063] | psro03 | +0.023 [+0.007, +0.039] | +0.007 [-0.026, +0.045] |
| 5 | added | psro05/guarded | +0.052 [+0.024, +0.080] | psro03 | +0.023 [+0.007, +0.039] | +0.029 [-0.003, +0.062] |
| 6 | rejected | psro06/guarded | +0.015 [-0.011, +0.041] | psro05 | +0.007 [-0.010, +0.023] | +0.008 [-0.023, +0.038] |
| 7 | rejected | psro07/guarded | +0.007 [-0.021, +0.036] | psro05 | +0.014 [-0.003, +0.031] | -0.007 [-0.038, +0.026] |

A response that is no better than the best member has not been shown to be a new strategy; the admission rule of the loop compares it with sigma, which carries the estimation error of the cells.

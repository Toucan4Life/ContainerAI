# PSRO iteration 7

Population before the iteration (9 strategies): run2, magnate, control, defense, exploit_run2, merchant, psro02, psro03, psro05. Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-a7d05e993a75.

## 1. Meta-game solution of the population

| n | solver | solution concept | support (weight) | restricted-game gain | independent check | bootstrap gain, mean / 95th pct | NashConv |
|---|---|---|---|---|---|---|---|
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | psro03 0.20, psro05 0.80 | 5.6e-17 | 5.6e-17 (enumerate) | 0.022 / 0.070 | 1.7e-16 |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | psro02 0.00, psro03 0.64, psro05 0.35 | 5.6e-15 | 5.6e-15 (enumerate) | 0.019 / 0.051 | 2.2e-14 |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | psro03 1.00 | 0.0e+00 | 0.0e+00 (enumerate) | 0.012 / 0.062 | 0.0e+00 |

Utility of every strategy against sigma_n (n-1 opponents drawn iid from sigma_n; 1/n is an equal share):

| n | run2 | magnate | control | defense | exploit_run2 | merchant | psro02 | psro03 | psro05 |
|---|---|---|---|---|---|---|---|---|---|
| 3 | 0.170 | 0.289 | 0.241 | 0.175 | 0.193 | 0.000 | 0.240 | 0.333 | 0.333 |
| 4 | 0.070 | 0.161 | 0.092 | 0.091 | 0.093 | 0.000 | 0.250 | 0.250 | 0.250 |
| 5 | 0.075 | 0.112 | 0.131 | 0.050 | 0.050 | 0.000 | 0.131 | 0.200 | 0.175 |

Every solver output (not only the chosen one):

| n | solver | concept | gain | NashConv | support |
|---|---|---|---|---|---|
| 3 | replicator | Nash candidate (verified by gain) | 8.3e-09 | 2.5e-08 | psro03, psro05 |
| 3 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 1.1e-05 | 3.3e-05 | psro03, psro05 |
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | 5.6e-17 | 1.7e-16 | psro03, psro05 |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 6.8e-13 | 2.1e-12 | psro03, psro05 |
| 4 | replicator | not verified (replicator average iterate has gain 1.53e-03 > 1e-06); no Nash claim | 1.5e-03 | 6.1e-03 | magnate, psro02, psro03, psro05 |
| 4 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 4.8e-04 | 1.9e-03 | psro02, psro03, psro05 |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | 5.6e-15 | 2.2e-14 | psro02, psro03, psro05 |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 9.0e-11 | 3.6e-10 | psro02, psro03, psro05 |
| 5 | replicator | Nash candidate (verified by gain) | 8.0e-09 | 4.0e-08 | psro03 |
| 5 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 2.3e-05 | 1.2e-04 | psro03 |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | psro03 |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | psro03 |

## 2. Best response

Initialised from `psro03` (the population strategy with the highest utility against sigma). PPO against opponents drawn from sigma smoothed with gamma = 0.1 towards the uniform mixture; objective `canonical`, seed 107, Adam learning rate 0.0001 falling linearly to 0 over the budget. 62,255 games, 11,990,994 decisions in 26 min. Utility during training (canonical, against the smoothed profile): 0.245 at the start, 0.273 at the end. Periodic dev gain over the smoothed profile: 10 min -0.003 [-0.055, +0.051], 19 min +0.021 [-0.034, +0.076], 26 min +0.046 [-0.005, +0.098].

## 3. Deviation study (seeds unseen by the training)

13,200 games, 1 stalled. Full tables: `reports/psro/iter_07/study.json`.
The candidate's gain over sigma is a lower bound on sigma's true deviation gain.

| n | value of sigma | candidate gain over sigma | NashConv (n x gain) | largest single-seat gain | best pure deviation in the population |
|---|---|---|---|---|---|
| 3 | +0.352 [+0.322, +0.381] | -0.016 [-0.071, +0.038] | -0.047 [-0.213, +0.113] | +0.041 | psro05 (+0.003) |
| 4 | +0.267 [+0.236, +0.300] | +0.000 [-0.043, +0.049] | +0.000 [-0.172, +0.197] | +0.044 | psro05 (+0.016) |
| 5 | +0.200 [+0.200, +0.200] | +0.038 [+0.002, +0.076] | +0.190 [+0.010, +0.380] | +0.085 | psro05 (+0.023) |
| mean | | +0.007 [-0.021, +0.036] | +0.048 [-0.058, +0.156] | +0.085 | |

The table is for the response played in `guarded` mode, the better of the modes evaluated (mean gain over player counts: sample -0.018 [-0.046, +0.011], guarded +0.007 [-0.021, +0.036]). The largest single-seat gain is the maximum over the engine seats of the per-seat mean gain. The seats are exchangeable (the first player is drawn at random), so each seat estimates the same individual regret, which is the mean gain; their maximum is a noisy overestimate and is shown for completeness.

## 4. Decision: **rejected**

Rule: accept when the lower 95% bound of the mean gain over player counts, in the better of the modes sample, guarded, is above 0.0.

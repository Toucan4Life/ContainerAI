# PSRO iteration 3

Population before the iteration (7 strategies): run2, magnate, control, defense, exploit_run2, merchant, psro02. Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-79634ab95aa6.

## 1. Meta-game solution of the population

| n | solver | solution concept | support (weight) | restricted-game gain | independent check | bootstrap gain, mean / 95th pct | NashConv |
|---|---|---|---|---|---|---|---|
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | control 0.12, psro02 0.88 | 5.6e-17 | 5.6e-17 (enumerate) | 0.026 / 0.098 | 1.7e-16 |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | psro02 1.00 | 0.0e+00 | 0.0e+00 (enumerate) | 0.002 / 0.001 | 0.0e+00 |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | psro02 1.00 | 0.0e+00 | 0.0e+00 (enumerate) | 0.008 / 0.050 | 0.0e+00 |

Utility of every strategy against sigma_n (n-1 opponents drawn iid from sigma_n; 1/n is an equal share):

| n | run2 | magnate | control | defense | exploit_run2 | merchant | psro02 |
|---|---|---|---|---|---|---|---|
| 3 | 0.166 | 0.249 | 0.333 | 0.205 | 0.280 | 0.000 | 0.333 |
| 4 | 0.109 | 0.141 | 0.094 | 0.125 | 0.094 | 0.000 | 0.250 |
| 5 | 0.038 | 0.125 | 0.131 | 0.075 | 0.169 | 0.000 | 0.200 |

Every solver output (not only the chosen one):

| n | solver | concept | gain | NashConv | support |
|---|---|---|---|---|---|
| 3 | replicator | Nash candidate (verified by gain) | 5.3e-08 | 1.6e-07 | control, psro02 |
| 3 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 1.0e-05 | 3.1e-05 | control, psro02 |
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | 5.6e-17 | 1.7e-16 | control, psro02 |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 7.8e-16 | 2.3e-15 | control, psro02 |
| 4 | replicator | Nash candidate (verified by gain) | 7.4e-09 | 3.0e-08 | psro02 |
| 4 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 1.0e-05 | 4.0e-05 | psro02 |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | psro02 |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | psro02 |
| 5 | replicator | Nash candidate (verified by gain) | 8.0e-09 | 4.0e-08 | psro02 |
| 5 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 7.7e-06 | 3.9e-05 | psro02 |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | psro02 |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | psro02 |

## 2. Best response

Initialised from `psro02` (the population strategy with the highest utility against sigma). PPO against opponents drawn from sigma smoothed with gamma = 0.1 towards the uniform mixture; objective `canonical`, seed 103, Adam learning rate 0.0001 falling linearly to 0 over the budget. 85,044 games, 15,816,548 decisions in 26 min. Utility during training (canonical, against the smoothed profile): 0.257 at the start, 0.284 at the end. Periodic dev gain over the smoothed profile: 9 min +0.019 [-0.040, +0.080], 18 min +0.004 [-0.064, +0.069], 26 min -0.007 [-0.062, +0.047].

## 3. Deviation study (seeds unseen by the training)

10,800 games, 0 stalled. Full tables: `reports/psro/iter_03/study.json`.
The candidate's gain over sigma is a lower bound on sigma's true deviation gain.

| n | value of sigma | candidate gain over sigma | NashConv (n x gain) | largest single-seat gain | best pure deviation in the population |
|---|---|---|---|---|---|
| 3 | +0.333 [+0.303, +0.364] | +0.007 [-0.051, +0.064] | +0.021 [-0.154, +0.191] | +0.119 | psro02 (+0.015) |
| 4 | +0.250 [+0.250, +0.250] | +0.045 [+0.007, +0.087] | +0.180 [+0.030, +0.350] | +0.070 | psro02 (+0.000) |
| 5 | +0.200 [+0.200, +0.200] | +0.044 [+0.007, +0.086] | +0.220 [+0.035, +0.430] | +0.085 | psro02 (+0.000) |
| mean | | +0.032 [+0.005, +0.059] | +0.140 [+0.035, +0.243] | +0.119 | |

The table is for the response played in `guarded` mode, the better of the modes evaluated (mean gain over player counts: sample +0.008 [-0.018, +0.034], guarded +0.032 [+0.005, +0.059]). The largest single-seat gain is the maximum over the engine seats of the per-seat mean gain. The seats are exchangeable (the first player is drawn at random), so each seat estimates the same individual regret, which is the mean gain; their maximum is a noisy overestimate and is shown for completeness.

## 4. Decision: **accepted** as psro03

Rule: accept when the lower 95% bound of the mean gain over player counts, in the better of the modes sample, guarded, is above 0.0.

## 5. Against each member alone

Utility of `psro03` at a table whose other seats all play the member in the column (dev seeds of the cross-play; 1/n is an equal share). The response was trained against a mixture, so this shows whether it also beats the members one by one.

| n | run2 | magnate | control | defense | exploit_run2 | merchant | psro02 |
|---|---|---|---|---|---|---|---|
| 3 | 0.562 | 0.469 | 0.521 | 0.375 | 0.542 | 1.000 | 0.417 |
| 4 | 0.477 | 0.367 | 0.453 | 0.461 | 0.453 | 1.000 | 0.258 |
| 5 | 0.406 | 0.250 | 0.275 | 0.375 | 0.500 | 1.000 | 0.188 |

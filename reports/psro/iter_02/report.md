# PSRO iteration 2

Population before the iteration (6 strategies): run2, magnate, control, defense, exploit_run2, merchant. Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-ce1dbd46f9d3.

## 1. Meta-game solution of the population

| n | solver | solution concept | support (weight) | restricted-game gain | independent check | bootstrap gain, mean / 95th pct | NashConv |
|---|---|---|---|---|---|---|---|
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | magnate 1.00 | 0.0e+00 | 0.0e+00 (enumerate) | 0.015 / 0.069 | 0.0e+00 |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | magnate 0.07, control 0.37, defense 0.50, exploit_run2 0.06 | 9.8e-11 | 9.8e-11 (enumerate) | 0.014 / 0.030 | 3.9e-10 |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | magnate 1.00 | 0.0e+00 | 0.0e+00 (enumerate) | 0.005 / 0.033 | 0.0e+00 |

Utility of every strategy against sigma_n (n-1 opponents drawn iid from sigma_n; 1/n is an equal share):

| n | run2 | magnate | control | defense | exploit_run2 | merchant |
|---|---|---|---|---|---|---|
| 3 | 0.257 | 0.333 | 0.319 | 0.194 | 0.292 | 0.000 |
| 4 | 0.183 | 0.250 | 0.250 | 0.250 | 0.250 | 0.000 |
| 5 | 0.100 | 0.200 | 0.167 | 0.175 | 0.117 | 0.000 |

Every solver output (not only the chosen one):

| n | solver | concept | gain | NashConv | support |
|---|---|---|---|---|---|
| 3 | replicator | Nash candidate (verified by gain) | 6.7e-09 | 2.0e-08 | magnate |
| 3 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 9.5e-06 | 2.9e-05 | magnate |
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | magnate |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | magnate |
| 4 | replicator | Nash candidate (verified by gain) | 7.5e-09 | 3.0e-08 | magnate |
| 4 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 7.6e-06 | 3.0e-05 | magnate |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | defense |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | magnate |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | defense |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 2.1e-14 | 8.3e-14 | magnate, defense |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 4.9e-13 | 2.0e-12 | magnate, control |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 4.2e-15 | 1.7e-14 | magnate, control, defense |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 1.9e-13 | 7.5e-13 | magnate, defense, exploit_run2 |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 3.8e-13 | 1.5e-12 | magnate, control, exploit_run2 |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 9.8e-11 | 3.9e-10 | magnate, control, defense, exploit_run2 |
| 5 | replicator | Nash candidate (verified by gain) | 8.0e-09 | 4.0e-08 | magnate |
| 5 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 5.5e-06 | 2.7e-05 | magnate |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | magnate |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | magnate |

## 2. Best response

Initialised from `magnate` (the population strategy with the highest utility against sigma). PPO against opponents drawn from sigma smoothed with gamma = 0.1 towards the uniform mixture; objective `canonical`, seed 102, Adam learning rate 0.0001 falling linearly to 0 over the budget. 103,000 games, 17,592,590 decisions in 25 min. Utility during training (canonical, against the smoothed profile): 0.249 at the start, 0.342 at the end. Periodic dev gain over the smoothed profile: 9 min +0.046 [-0.006, +0.103], 18 min +0.084 [+0.031, +0.138], 25 min +0.043 [-0.015, +0.100].

## 3. Deviation study (seeds unseen by the training)

9,600 games, 0 stalled. Full tables: `reports/psro/iter_02/study.json`.
The candidate's gain over sigma is a lower bound on sigma's true deviation gain.

| n | value of sigma | candidate gain over sigma | NashConv (n x gain) | largest single-seat gain | best pure deviation in the population |
|---|---|---|---|---|---|
| 3 | +0.333 [+0.333, +0.333] | +0.058 [+0.002, +0.112] | +0.175 [+0.005, +0.335] | +0.120 | magnate (+0.000) |
| 4 | +0.252 [+0.225, +0.278] | +0.182 [+0.132, +0.230] | +0.727 [+0.529, +0.919] | +0.225 | magnate (+0.054) |
| 5 | +0.200 [+0.200, +0.200] | +0.055 [+0.019, +0.091] | +0.275 [+0.095, +0.455] | +0.140 | magnate (+0.000) |
| mean | | +0.098 [+0.070, +0.126] | +0.392 [+0.283, +0.499] | +0.225 | |

The table is for the response played in `guarded` mode, the better of the modes evaluated (mean gain over player counts: sample +0.057 [+0.028, +0.083], guarded +0.098 [+0.070, +0.126]). The largest single-seat gain is the maximum over the engine seats of the per-seat mean gain. The seats are exchangeable (the first player is drawn at random), so each seat estimates the same individual regret, which is the mean gain; their maximum is a noisy overestimate and is shown for completeness.

## 4. Decision: **accepted** as psro02

Rule: accept when the lower 95% bound of the mean gain over player counts, in the better of the modes sample, guarded, is above 0.0.

## 5. Against each member alone

Utility of `psro02` at a table whose other seats all play the member in the column (dev seeds of the cross-play; 1/n is an equal share). The response was trained against a mixture, so this shows whether it also beats the members one by one.

| n | run2 | magnate | control | defense | exploit_run2 | merchant |
|---|---|---|---|---|---|---|
| 3 | 0.615 | 0.417 | 0.479 | 0.438 | 0.490 | 1.000 |
| 4 | 0.508 | 0.328 | 0.375 | 0.328 | 0.469 | 1.000 |
| 5 | 0.412 | 0.281 | 0.325 | 0.300 | 0.481 | 1.000 |

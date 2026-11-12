# PSRO iteration 5

Population before the iteration (8 strategies): run2, magnate, control, defense, exploit_run2, merchant, psro02, psro03. Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-79634ab95aa6.

## 1. Meta-game solution of the population

| n | solver | solution concept | support (weight) | restricted-game gain | independent check | bootstrap gain, mean / 95th pct | NashConv |
|---|---|---|---|---|---|---|---|
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | magnate 0.15, psro02 0.11, psro03 0.73 | 0.0e+00 | 0.0e+00 (enumerate) | 0.023 / 0.060 | 0.0e+00 |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | psro02 0.37, psro03 0.63 | 1.9e-14 | 1.9e-14 (enumerate) | 0.013 / 0.035 | 7.6e-14 |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | psro02 0.91, psro03 0.09 | 1.7e-11 | 1.7e-11 (enumerate) | 0.012 / 0.042 | 8.3e-11 |

Utility of every strategy against sigma_n (n-1 opponents drawn iid from sigma_n; 1/n is an equal share):

| n | run2 | magnate | control | defense | exploit_run2 | merchant | psro02 | psro03 |
|---|---|---|---|---|---|---|---|---|
| 3 | 0.167 | 0.333 | 0.207 | 0.249 | 0.166 | 0.000 | 0.333 | 0.333 |
| 4 | 0.055 | 0.180 | 0.127 | 0.085 | 0.107 | 0.000 | 0.250 | 0.250 |
| 5 | 0.052 | 0.130 | 0.122 | 0.090 | 0.154 | 0.000 | 0.200 | 0.200 |

Every solver output (not only the chosen one):

| n | solver | concept | gain | NashConv | support |
|---|---|---|---|---|---|
| 3 | replicator | Nash candidate (verified by gain) | 2.8e-08 | 8.5e-08 | magnate, psro02, psro03 |
| 3 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 8.6e-06 | 2.6e-05 | magnate, psro02, psro03 |
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | magnate, psro02, psro03 |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 8.3e-12 | 2.5e-11 | magnate, psro02, psro03 |
| 4 | replicator | Nash candidate (verified by gain) | 1.2e-08 | 4.7e-08 | psro02, psro03 |
| 4 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 8.3e-06 | 3.3e-05 | psro02, psro03 |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | 1.0e-12 | 4.0e-12 | psro02, psro03 |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 1.9e-14 | 7.6e-14 | psro02, psro03 |
| 5 | replicator | Nash candidate (verified by gain) | 8.0e-09 | 4.0e-08 | psro03 |
| 5 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 1.2e-05 | 6.2e-05 | psro03 |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | psro02 |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | psro02 |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | psro03 |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 1.7e-11 | 8.3e-11 | psro02, psro03 |

## 2. Best response

Initialised from `psro03` (the population strategy with the highest utility against sigma). PPO against opponents drawn from sigma smoothed with gamma = 0.1 towards the uniform mixture; objective `canonical`, seed 105, Adam learning rate 0.0001 falling linearly to 0 over the budget. 70,709 games, 13,636,187 decisions in 26 min. Utility during training (canonical, against the smoothed profile): 0.272 at the start, 0.296 at the end. Periodic dev gain over the smoothed profile: 9 min +0.009 [-0.054, +0.072], 19 min -0.009 [-0.061, +0.044], 26 min -0.026 [-0.079, +0.032].

## 3. Deviation study (seeds unseen by the training)

12,000 games, 0 stalled. Full tables: `reports/psro/iter_05/study.json`.
The candidate's gain over sigma is a lower bound on sigma's true deviation gain.

| n | value of sigma | candidate gain over sigma | NashConv (n x gain) | largest single-seat gain | best pure deviation in the population |
|---|---|---|---|---|---|
| 3 | +0.320 [+0.292, +0.347] | +0.098 [+0.042, +0.155] | +0.295 [+0.127, +0.466] | +0.245 | psro02 (+0.013) |
| 4 | +0.234 [+0.209, +0.261] | +0.038 [-0.011, +0.089] | +0.152 [-0.046, +0.357] | +0.062 | psro03 (+0.007) |
| 5 | +0.196 [+0.174, +0.218] | +0.020 [-0.018, +0.057] | +0.101 [-0.088, +0.286] | +0.079 | psro03 (+0.049) |
| mean | | +0.052 [+0.024, +0.080] | +0.183 [+0.071, +0.291] | +0.245 | |

The table is for the response played in `guarded` mode, the better of the modes evaluated (mean gain over player counts: sample +0.022 [-0.008, +0.051], guarded +0.052 [+0.024, +0.080]). The largest single-seat gain is the maximum over the engine seats of the per-seat mean gain. The seats are exchangeable (the first player is drawn at random), so each seat estimates the same individual regret, which is the mean gain; their maximum is a noisy overestimate and is shown for completeness.

## 4. Decision: **accepted** as psro05

Rule: accept when the lower 95% bound of the mean gain over player counts, in the better of the modes sample, guarded, is above 0.0.

## 5. Against each member alone

Utility of `psro05` at a table whose other seats all play the member in the column (dev seeds of the cross-play; 1/n is an equal share). The response was trained against a mixture, so this shows whether it also beats the members one by one.

| n | run2 | magnate | control | defense | exploit_run2 | merchant | psro02 | psro03 |
|---|---|---|---|---|---|---|---|---|
| 3 | 0.458 | 0.292 | 0.302 | 0.438 | 0.521 | 1.000 | 0.365 | 0.417 |
| 4 | 0.547 | 0.430 | 0.375 | 0.453 | 0.359 | 1.000 | 0.234 | 0.227 |
| 5 | 0.338 | 0.212 | 0.250 | 0.388 | 0.525 | 1.000 | 0.231 | 0.175 |

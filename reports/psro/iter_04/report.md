# PSRO iteration 4

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

Initialised from `psro03` (the population strategy with the highest utility against sigma). PPO against opponents drawn from sigma smoothed with gamma = 0.1 towards the uniform mixture; objective `canonical`, seed 104, Adam learning rate 0.0001 falling linearly to 0 over the budget. 70,954 games, 13,537,135 decisions in 26 min. Utility during training (canonical, against the smoothed profile): 0.269 at the start, 0.290 at the end. Periodic dev gain over the smoothed profile: 9 min +0.043 [-0.016, +0.100], 19 min +0.030 [-0.026, +0.085], 26 min +0.028 [-0.026, +0.086].

## 3. Deviation study (seeds unseen by the training)

12,000 games, 0 stalled. Full tables: `reports/psro/iter_04/study.json`.
The candidate's gain over sigma is a lower bound on sigma's true deviation gain.

| n | value of sigma | candidate gain over sigma | NashConv (n x gain) | largest single-seat gain | best pure deviation in the population |
|---|---|---|---|---|---|
| 3 | +0.331 [+0.299, +0.364] | +0.036 [-0.035, +0.108] | +0.108 [-0.105, +0.324] | +0.089 | psro03 (+0.021) |
| 4 | +0.267 [+0.239, +0.294] | +0.026 [-0.024, +0.077] | +0.104 [-0.094, +0.310] | +0.074 | psro03 (+0.027) |
| 5 | +0.196 [+0.171, +0.222] | +0.028 [-0.013, +0.070] | +0.140 [-0.064, +0.352] | +0.083 | psro03 (+0.020) |
| mean | | +0.030 [-0.002, +0.063] | +0.117 [+0.002, +0.240] | +0.089 | |

The table is for the response played in `sample` mode, the better of the modes evaluated (mean gain over player counts: sample +0.030 [-0.002, +0.063], guarded +0.028 [-0.005, +0.060]). The largest single-seat gain is the maximum over the engine seats of the per-seat mean gain. The seats are exchangeable (the first player is drawn at random), so each seat estimates the same individual regret, which is the mean gain; their maximum is a noisy overestimate and is shown for completeness.

## 4. Decision: **rejected**

Rule: accept when the lower 95% bound of the mean gain over player counts, in the better of the modes sample, guarded, is above 0.0.

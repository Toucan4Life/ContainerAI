# PSRO, restricted game of the final population with the support cells at 64-72 seeds

Strategies: run2, magnate, control, defense, exploit_run2, merchant, psro02, psro03, psro05. Tables simulated: 3 players 14,304 games, 1 stalled; 4 players 49,152 games, 0 stalled; 5 players 143,280 games, 1 stalled. Utility: winner share (a stalled game pays 0). Every player count is its own symmetric game.

Gains are exact in the empirical game. The bootstrap column resamples the simulated seeds of every table and re-measures the gain of the *fixed* solution: it is the noise floor under which a deviation gain cannot be told from sampling error. An equilibrium of the empirical game is not an equilibrium of the real game.

| n | solver | solution concept | support (weight) | restricted-game gain | independent check | bootstrap gain, mean / 95th pct | NashConv |
|---|---|---|---|---|---|---|---|
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | psro05 1.00 | 0.0e+00 | 0.0e+00 (enumerate) | 0.004 / 0.029 | 0.0e+00 |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | psro05 1.00 | 0.0e+00 | 0.0e+00 (enumerate) | 0.005 / 0.033 | 0.0e+00 |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | psro03 1.00 | 0.0e+00 | 0.0e+00 (enumerate) | 0.006 / 0.025 | 0.0e+00 |

Utility of every strategy against sigma_n (n-1 opponents drawn iid from sigma_n; 1/n is an equal share):

| n | run2 | magnate | control | defense | exploit_run2 | merchant | psro02 | psro03 | psro05 |
|---|---|---|---|---|---|---|---|---|---|
| 3 | 0.125 | 0.253 | 0.237 | 0.146 | 0.146 | 0.000 | 0.266 | 0.305 | 0.333 |
| 4 | 0.062 | 0.096 | 0.088 | 0.047 | 0.094 | 0.000 | 0.188 | 0.238 | 0.250 |
| 5 | 0.075 | 0.142 | 0.103 | 0.050 | 0.050 | 0.000 | 0.192 | 0.200 | 0.178 |

Every solver output (not only the chosen one):

| n | solver | concept | gain | NashConv | support |
|---|---|---|---|---|---|
| 3 | replicator | Nash candidate (verified by gain) | 6.7e-09 | 2.0e-08 | psro05 |
| 3 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 1.1e-05 | 3.3e-05 | psro05 |
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | psro05 |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | psro05 |
| 4 | replicator | Nash candidate (verified by gain) | 7.5e-09 | 3.0e-08 | psro05 |
| 4 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 1.2e-05 | 4.6e-05 | psro05 |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | psro05 |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | psro05 |
| 5 | replicator | not verified (replicator average iterate has gain 1.89e-03 > 1e-06); no Nash claim | 1.9e-03 | 9.5e-03 | magnate, control, defense, exploit_run2, psro02, psro03, psro05 |
| 5 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 1.2e-05 | 6.0e-05 | psro03 |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | psro03 |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | psro03 |

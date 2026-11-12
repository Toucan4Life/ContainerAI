# PSRO, restricted game of the final population

Strategies: run2, magnate, control, defense, exploit_run2, merchant, psro02, psro03, psro05. Tables simulated: 3 players 9,264 games, 1 stalled; 4 players 35,712 games, 0 stalled; 5 players 113,040 games, 0 stalled. Utility: winner share (a stalled game pays 0). Every player count is its own symmetric game.

Gains are exact in the empirical game. The bootstrap column resamples the simulated seeds of every table and re-measures the gain of the *fixed* solution: it is the noise floor under which a deviation gain cannot be told from sampling error. An equilibrium of the empirical game is not an equilibrium of the real game.

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

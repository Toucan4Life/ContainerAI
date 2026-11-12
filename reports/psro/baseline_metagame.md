# Empirical meta-game of the baseline population (deployed modes, dev seeds)

Strategies: merchant, run1, run2, exploit_run2, control, defense, magnate. Tables simulated: 3 players 6,048 games, 6 stalled; 4 players 20,160 games, 0 stalled; 5 players 55,440 games, 0 stalled. Utility: winner share (a stalled game pays 0). Every player count is its own symmetric game.

Gains are exact in the empirical game. The bootstrap column resamples the simulated seeds of every table and re-measures the gain of the *fixed* solution: it is the noise floor under which a deviation gain cannot be told from sampling error. An equilibrium of the empirical game is not an equilibrium of the real game.

| n | solver | solution concept | support (weight) | restricted-game gain | independent check | bootstrap gain, mean / 95th pct | NashConv |
|---|---|---|---|---|---|---|---|
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | magnate 1.00 | 0.0e+00 | 0.0e+00 (enumerate) | 0.013 / 0.056 | 0.0e+00 |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | exploit_run2 0.06, control 0.37, defense 0.50, magnate 0.07 | 1.7e-11 | 1.7e-11 (enumerate) | 0.014 / 0.029 | 6.7e-11 |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | magnate 1.00 | 0.0e+00 | 0.0e+00 (enumerate) | 0.005 / 0.033 | 0.0e+00 |

Utility of every strategy against sigma_n (n-1 opponents drawn iid from sigma_n; 1/n is an equal share):

| n | merchant | run1 | run2 | exploit_run2 | control | defense | magnate |
|---|---|---|---|---|---|---|---|
| 3 | 0.000 | 0.111 | 0.257 | 0.292 | 0.319 | 0.194 | 0.333 |
| 4 | 0.000 | 0.067 | 0.183 | 0.250 | 0.250 | 0.250 | 0.250 |
| 5 | 0.000 | 0.025 | 0.100 | 0.117 | 0.167 | 0.175 | 0.200 |

Every solver output (not only the chosen one):

| n | solver | concept | gain | NashConv | support |
|---|---|---|---|---|---|
| 3 | replicator | Nash candidate (verified by gain) | 6.7e-09 | 2.0e-08 | magnate |
| 3 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 1.1e-05 | 3.2e-05 | magnate |
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | magnate |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | magnate |
| 4 | replicator | Nash candidate (verified by gain) | 7.5e-09 | 3.0e-08 | magnate |
| 4 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 9.6e-06 | 3.9e-05 | magnate |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | defense |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | defense |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | magnate |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 2.1e-14 | 8.3e-14 | defense, magnate |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 4.9e-13 | 2.0e-12 | control, magnate |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 4.2e-15 | 1.7e-14 | control, defense, magnate |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 2.9e-14 | 1.2e-13 | exploit_run2, control, magnate |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 1.9e-13 | 7.5e-13 | exploit_run2, defense, magnate |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 1.7e-11 | 6.7e-11 | exploit_run2, control, defense, magnate |
| 5 | replicator | Nash candidate (verified by gain) | 8.0e-09 | 4.0e-08 | magnate |
| 5 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 6.7e-06 | 3.4e-05 | magnate |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | magnate |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | magnate |

# Empirical meta-game

Strategies: run2, magnate, control, defense, exploit_run2, merchant, psro02, psro03, psro05, psro08. Tables simulated: 3 players 18,264 games, 1 stalled; 4 players 70,272 games, 4 stalled; 5 players 229,080 games, 1 stalled. Utility: winner share (a stalled game pays 0). Every player count is its own symmetric game.

Gains are exact in the empirical game. The bootstrap column resamples the simulated seeds of every table and re-measures the gain of the *fixed* solution: it is the noise floor under which a deviation gain cannot be told from sampling error. An equilibrium of the empirical game is not an equilibrium of the real game.

| n | solver | solution concept | support (weight) | restricted-game gain | independent check | bootstrap gain, mean / 95th pct | NashConv |
|---|---|---|---|---|---|---|---|
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | psro02 0.42, psro03 0.14, psro08 0.44 | 2.2e-16 | 2.2e-16 (enumerate) | 0.019 / 0.039 | 6.7e-16 |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | psro05 0.30, psro08 0.70 | 5.6e-17 | 5.6e-17 (enumerate) | 0.014 / 0.039 | 2.2e-16 |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | psro05 0.06, psro08 0.94 | 0.0e+00 | 2.8e-17 (enumerate) | 0.014 / 0.046 | 0.0e+00 |

Utility of every strategy against sigma_n (n-1 opponents drawn iid from sigma_n; 1/n is an equal share):

| n | run2 | magnate | control | defense | exploit_run2 | merchant | psro02 | psro03 | psro05 | psro08 |
|---|---|---|---|---|---|---|---|---|---|---|
| 3 | 0.170 | 0.244 | 0.230 | 0.170 | 0.187 | 0.000 | 0.333 | 0.333 | 0.312 | 0.333 |
| 4 | 0.066 | 0.103 | 0.060 | 0.075 | 0.096 | 0.000 | 0.212 | 0.234 | 0.247 | 0.247 |
| 5 | 0.063 | 0.103 | 0.118 | 0.095 | 0.082 | 0.000 | 0.176 | 0.173 | 0.200 | 0.200 |

Every solver output (not only the chosen one):

| n | solver | concept | gain | NashConv | support |
|---|---|---|---|---|---|
| 3 | replicator | Nash candidate (verified by gain) | 2.8e-08 | 8.5e-08 | magnate, psro08 |
| 3 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 1.1e-05 | 3.2e-05 | magnate, psro08 |
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | 4.1e-14 | 1.2e-13 | magnate, psro08 |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 6.1e-16 | 1.8e-15 | psro02, psro08 |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 1.4e-14 | 4.2e-14 | magnate, psro08 |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 2.2e-16 | 6.7e-16 | psro02, psro03, psro08 |
| 4 | replicator | not verified (replicator average iterate has gain 1.50e-03 > 1e-06); no Nash claim | 1.5e-03 | 6.0e-03 | magnate, psro02, psro03, psro05, psro08 |
| 4 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 9.9e-06 | 4.0e-05 | psro05 |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | psro05 |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | psro05 |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 5.6e-17 | 2.2e-16 | psro05, psro08 |
| 5 | replicator | not verified (replicator average iterate has gain 1.44e-03 > 1e-06); no Nash claim | 1.4e-03 | 7.2e-03 | magnate, control, defense, psro02, psro03, psro05, psro08 |
| 5 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 7.3e-06 | 3.6e-05 | psro05, psro08 |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | psro05, psro08 |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 1.2e-14 | 5.8e-14 | psro05, psro08 |

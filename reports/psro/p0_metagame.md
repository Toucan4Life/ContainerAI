# Meta-game of the initial PSRO population P0 = {run2, magnate, exploit_run2} (deployed modes, dev seeds)

Strategies: run2, magnate, exploit_run2. Tables simulated: 3 players 720 games, 0 stalled; 4 players 1,440 games, 0 stalled; 5 players 2,520 games, 0 stalled. Utility: winner share (a stalled game pays 0). Every player count is its own symmetric game.

Gains are exact in the empirical game. The bootstrap column resamples the simulated seeds of every table and re-measures the gain of the *fixed* solution: it is the noise floor under which a deviation gain cannot be told from sampling error. An equilibrium of the empirical game is not an equilibrium of the real game.

| n | solver | solution concept | support (weight) | restricted-game gain | independent check | bootstrap gain, mean / 95th pct | NashConv |
|---|---|---|---|---|---|---|---|
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | magnate 1.00 | 0.0e+00 | 0.0e+00 (enumerate) | 0.005 / 0.035 | 0.0e+00 |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | magnate 1.00 | 0.0e+00 | 0.0e+00 (enumerate) | 0.004 / 0.031 | 0.0e+00 |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | magnate 1.00 | 0.0e+00 | 0.0e+00 (enumerate) | 0.000 / 0.000 | 0.0e+00 |

Utility of every strategy against sigma_n (n-1 opponents drawn iid from sigma_n; 1/n is an equal share):

| n | run2 | magnate | exploit_run2 |
|---|---|---|---|
| 3 | 0.257 | 0.333 | 0.292 |
| 4 | 0.094 | 0.250 | 0.219 |
| 5 | 0.100 | 0.200 | 0.117 |

Every solver output (not only the chosen one):

| n | solver | concept | gain | NashConv | support |
|---|---|---|---|---|---|
| 3 | replicator | Nash candidate (verified by gain) | 2.0e-09 | 5.9e-09 | magnate |
| 3 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 2.0e-06 | 5.9e-06 | magnate |
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | magnate |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | magnate |
| 4 | replicator | Nash candidate (verified by gain) | 2.7e-09 | 1.1e-08 | magnate |
| 4 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 3.6e-06 | 1.5e-05 | magnate |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | magnate |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | magnate |
| 5 | replicator | Nash candidate (verified by gain) | 2.1e-09 | 1.0e-08 | magnate |
| 5 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 3.1e-06 | 1.5e-05 | magnate |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | magnate |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | magnate |

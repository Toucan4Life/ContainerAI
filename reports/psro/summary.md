# PSRO progress

One row per iteration. `gain` is the deviation gain of the newly trained response over the meta-strategy sigma it was trained against (mean over player counts, 95% interval, dev seeds unseen by the training); it is a lower bound on the exploitability of sigma, and NashConv = n x gain at table size n. `restricted gain` is the deviation gain of sigma inside the population itself (0 for a Nash equilibrium of the empirical game). `oracle curve` is the response's own periodic dev gain over the smoothed profile by minutes of training. Nothing here is a claim about the full game.

| iter | strategies | sigma_3 / sigma_4 / sigma_5 support | restricted gain (max over n) | gain of the new response | NashConv mean over n | largest single-seat gain | stalled / games | oracle curve | decision | oracle |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 6 | magnate / magnate+control+defense+exploit_run2 / magnate | 9.8e-11 | +0.006 [-0.019, +0.032] (guarded) | -0.005 [-0.102, +0.089] | +0.119 | 0 / 9,600 | 9: -0.054, 18: -0.048, 25: -0.050 | rejected | 25 min, lr 0.0003 |
| 2 | 6 | magnate / magnate+control+defense+exploit_run2 / magnate | 9.8e-11 | +0.098 [+0.070, +0.126] (guarded) | +0.392 [+0.283, +0.499] | +0.225 | 0 / 9,600 | 9: +0.046, 18: +0.084, 25: +0.043 | added psro02 | 25 min, lr 0.0001 falling to 0 |
| 3 | 7 | control+psro02 / psro02 / psro02 | 5.6e-17 | +0.032 [+0.005, +0.059] (guarded) | +0.140 [+0.035, +0.243] | +0.119 | 0 / 10,800 | 9: +0.019, 18: +0.004, 26: -0.007 | added psro03 | 26 min, lr 0.0001 falling to 0 |
| 4 | 8 | magnate+psro02+psro03 / psro02+psro03 / psro02+psro03 | 1.7e-11 | +0.030 [-0.002, +0.063] (sample) | +0.117 [+0.002, +0.240] | +0.089 | 0 / 12,000 | 9: +0.043, 19: +0.030, 26: +0.028 | rejected | 26 min, lr 0.0001 falling to 0 |
| 5 | 8 | magnate+psro02+psro03 / psro02+psro03 / psro02+psro03 | 1.7e-11 | +0.052 [+0.024, +0.080] (guarded) | +0.183 [+0.071, +0.291] | +0.245 | 0 / 12,000 | 9: +0.009, 19: -0.009, 26: -0.026 | added psro05 | 26 min, lr 0.0001 falling to 0 |
| 6 | 9 | psro03+psro05 / psro03+psro05 / psro03 | 5.6e-15 | +0.015 [-0.011, +0.041] (guarded) | +0.054 [-0.040, +0.156] | +0.065 | 0 / 13,200 | 10: -0.021, 19: +0.009, 26: +0.049 | rejected | 26 min, lr 0.0001 falling to 0 |
| 7 | 9 | psro03+psro05 / psro03+psro05 / psro03 | 5.6e-15 | +0.007 [-0.021, +0.036] (guarded) | +0.048 [-0.058, +0.156] | +0.085 | 1 / 13,200 | 10: -0.003, 19: +0.021, 26: +0.046 | rejected | 26 min, lr 0.0001 falling to 0 |

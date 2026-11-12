# Loan-while-bidding probe: forced RequestLoan in the bid and decide phases against the refined solution

Profile sigma: {"3": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0], "4": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0], "5": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]} over ['run2', 'magnate', 'control', 'defense', 'exploit_run2', 'merchant', 'psro02', 'psro03', 'psro05']. 100 dev seeds per player count (offset 1000), every seat of every table, 18,000 games, 0 stalled. Code version 2552fa3f751effecd0d7fb11e097fa2e67269a99+dirty-1ae8ea0f41a2.

Each row plays a net with a rule that replaces its pick by `RequestLoan`, against opponents drawn from sigma. `paired gain` is its utility minus the same net unmodified on the same seeds, seats and opponents (mean over player counts, 95% seed-clustered interval); `fired` counts the decisions the rule replaced of those where it applied (a loan was legal in that phase).

| net + rule | applied | fired | paired gain | 3 players | 4 players | 5 players | gain over sigma |
|---|---|---|---|---|---|---|---|
| psro03+bid:allin | 38,092 | 0 | +0.000 [+0.000, +0.000] | +0.000 [+0.000, +0.000] | +0.000 [+0.000, +0.000] | +0.000 [+0.000, +0.000] | -0.017 [-0.041, +0.006] |
| psro03+bid:any:0.1 | 40,565 | 4,035 | -0.079 [-0.103, -0.054] | -0.100 [-0.153, -0.048] | -0.071 [-0.119, -0.025] | -0.065 [-0.091, -0.037] | -0.095 [-0.116, -0.075] |
| psro03+decide:any:0.1 | 8,583 | 820 | -0.017 [-0.033, -0.000] | -0.027 [-0.060, +0.007] | -0.033 [-0.061, -0.006] | +0.009 [-0.014, +0.033] | -0.033 [-0.056, -0.010] |
| psro05+bid:allin | 37,465 | 0 | +0.000 [+0.000, +0.000] | +0.000 [+0.000, +0.000] | +0.000 [+0.000, +0.000] | +0.000 [+0.000, +0.000] | -0.001 [-0.012, +0.010] |
| psro05+bid:any:0.1 | 40,221 | 3,947 | -0.079 [-0.100, -0.057] | -0.048 [-0.090, -0.007] | -0.122 [-0.155, -0.089] | -0.066 [-0.107, -0.025] | -0.080 [-0.100, -0.059] |
| psro05+decide:any:0.1 | 8,567 | 861 | -0.004 [-0.021, +0.015] | +0.003 [-0.030, +0.040] | -0.024 [-0.054, +0.009] | +0.009 [-0.017, +0.037] | -0.005 [-0.023, +0.014] |

Gain over sigma of the unmodified nets (mean over player counts): psro03: -0.017 [-0.041, +0.006], psro05: -0.001 [-0.012, +0.010], run2: -0.164 [-0.181, -0.146], magnate: -0.100 [-0.120, -0.081], control: -0.130 [-0.150, -0.110], defense: -0.144 [-0.163, -0.125], exploit_run2: -0.138 [-0.157, -0.119], merchant: -0.261 [-0.261, -0.261], psro02: -0.030 [-0.051, -0.008].

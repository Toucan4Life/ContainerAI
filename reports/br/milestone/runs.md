# Milestone: the four best-response runs

One row per best-response training run. `curve` is the gain over the profile of the run's own periodic dev studies (minutes of training: mean gain; each with a 95% seed-clustered interval in the run's `result.json`). `study` is the paired deviation study of the final weights on dev seeds the training never used. A trained response is an approximate best response, so every gain here is a lower bound on the profile's true deviation gain. `best pure` is the best strategy of the population in the same study; `recovery` is the better mode's gain divided by it (shown when it is at least 0.02).

| run | init | profile | objective | seed | min | games | curve | study (mean over player counts) | best pure | recovery |
|---|---|---|---|---|---|---|---|---|---|---|
| m1_defense_s1 | defense_run2_vs_exploiter_30m | run2 0.33, magnate 0.33, exploit_run2 0.33 | canonical | 1 | 25 | 133k | 5: +0.068, 11: +0.047, 16: +0.023, 21: +0.061, 25: +0.099 | sample -0.001 [-0.036, +0.033]<br>guarded +0.069 [+0.028, +0.111] | magnate +0.085 | 81% |
| m2_defense_s2 | defense_run2_vs_exploiter_30m | run2 0.33, magnate 0.33, exploit_run2 0.33 | canonical | 2 | 25 | 130k | 5: -0.029, 10: +0.020, 16: +0.039, 21: +0.041, 25: +0.053 | sample +0.055 [+0.019, +0.090]<br>guarded +0.098 [+0.060, +0.140] | magnate +0.043 | 227% |
| m3_run1_s3 | run1 | run2 0.33, magnate 0.33, exploit_run2 0.33 | canonical | 3 | 25 | 134k | 5: -0.150, 10: -0.175, 16: -0.176, 21: -0.138, 25: -0.045 | sample -0.124 [-0.152, -0.096]<br>guarded -0.105 [-0.140, -0.072] | magnate +0.057 | -183% |
| m4_broad_s4 | defense_run2_vs_exploiter_30m | run2 0.17, magnate 0.17, control 0.17, defense 0.17, exploit_run2 0.17, merchant 0.17 | canonical | 4 | 25 | 120k | 5: +0.005, 11: +0.042, 16: +0.001, 22: -0.006, 25: +0.036 | sample +0.005 [-0.027, +0.036]<br>guarded +0.076 [+0.041, +0.111] | magnate +0.123 | 62% |

## Gain against training time

The weights saved during a run, each played as a candidate in the same paired study (same seeds and opponents; each interval is a 95% seed-clustered interval). The last two columns are the paired difference between the final weights and the saved ones: an interval above zero means that the run was still improving after that point.

| run | minutes of training | sample | guarded | final minus this, sample | final minus this, guarded |
|---|---|---|---|---|---|
| m1_defense_s1 | 5 | +0.022 [-0.015, +0.057] | +0.004 [-0.032, +0.041] | -0.023 [-0.063, +0.018] | +0.065 [+0.019, +0.112] |
| m1_defense_s1 | 11 | +0.031 [-0.005, +0.068] | +0.054 [+0.019, +0.091] | -0.033 [-0.075, +0.009] | +0.015 [-0.029, +0.061] |
| m1_defense_s1 | 16 | +0.027 [-0.008, +0.063] | +0.028 [-0.012, +0.065] | -0.029 [-0.072, +0.017] | +0.041 [-0.004, +0.089] |
| m1_defense_s1 | 21 | +0.040 [+0.001, +0.081] | +0.062 [+0.024, +0.100] | -0.042 [-0.088, +0.005] | +0.007 [-0.041, +0.060] |
| m1_defense_s1 | 25 | -0.001 [-0.036, +0.033] | +0.069 [+0.028, +0.111] |  |  |
| m2_defense_s2 | 5 | +0.022 [-0.013, +0.058] | +0.065 [+0.028, +0.101] | +0.033 [-0.013, +0.075] | +0.033 [-0.017, +0.084] |
| m2_defense_s2 | 10 | +0.009 [-0.026, +0.042] | +0.072 [+0.038, +0.108] | +0.046 [+0.002, +0.088] | +0.026 [-0.025, +0.077] |
| m2_defense_s2 | 16 | +0.032 [-0.005, +0.068] | +0.061 [+0.023, +0.099] | +0.022 [-0.020, +0.064] | +0.037 [-0.008, +0.085] |
| m2_defense_s2 | 21 | +0.011 [-0.025, +0.049] | +0.055 [+0.017, +0.092] | +0.044 [-0.001, +0.091] | +0.043 [-0.004, +0.093] |
| m2_defense_s2 | 25 | +0.055 [+0.019, +0.090] | +0.098 [+0.060, +0.140] |  |  |
| m3_run1_s3 | 5 | -0.178 [-0.209, -0.145] | -0.155 [-0.186, -0.125] | +0.054 [+0.018, +0.090] | +0.050 [+0.013, +0.088] |
| m3_run1_s3 | 10 | -0.190 [-0.220, -0.163] | -0.122 [-0.155, -0.092] | +0.067 [+0.033, +0.100] | +0.017 [-0.020, +0.055] |
| m3_run1_s3 | 16 | -0.137 [-0.168, -0.106] | -0.133 [-0.163, -0.103] | +0.013 [-0.024, +0.052] | +0.028 [-0.009, +0.065] |
| m3_run1_s3 | 21 | -0.143 [-0.175, -0.112] | -0.107 [-0.144, -0.071] | +0.020 [-0.019, +0.059] | +0.002 [-0.037, +0.040] |
| m3_run1_s3 | 25 | -0.124 [-0.152, -0.096] | -0.105 [-0.140, -0.072] |  |  |
| m4_broad_s4 | 5 | +0.009 [-0.027, +0.045] | +0.069 [+0.038, +0.101] | -0.005 [-0.050, +0.043] | +0.007 [-0.040, +0.053] |
| m4_broad_s4 | 11 | +0.015 [-0.017, +0.047] | +0.061 [+0.028, +0.094] | -0.011 [-0.050, +0.030] | +0.015 [-0.034, +0.063] |
| m4_broad_s4 | 16 | -0.002 [-0.037, +0.033] | +0.048 [+0.014, +0.085] | +0.006 [-0.037, +0.049] | +0.028 [-0.019, +0.071] |
| m4_broad_s4 | 22 | +0.012 [-0.023, +0.047] | +0.031 [-0.002, +0.064] | -0.008 [-0.052, +0.035] | +0.045 [-0.003, +0.090] |
| m4_broad_s4 | 25 | +0.005 [-0.027, +0.036] | +0.076 [+0.041, +0.111] |  |  |

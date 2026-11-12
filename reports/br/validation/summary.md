# Best-response oracle validation (12-minute runs)

One row per best-response training run. `curve` is the gain over the profile of the run's own periodic dev studies (minutes of training: mean gain; each with a 95% seed-clustered interval in the run's `result.json`). `study` is the paired deviation study of the final weights on dev seeds the training never used. A trained response is an approximate best response, so every gain here is a lower bound on the profile's true deviation gain. `best pure` is the best strategy of the population in the same study; `recovery` is the better mode's gain divided by it (shown when it is at least 0.02).

| run | init | profile | objective | seed | min | games | curve | study (mean over player counts) | best pure | recovery |
|---|---|---|---|---|---|---|---|---|---|---|
| v1_run1_vs_run1_shaped | run1 | run1 | shaped | 11 | 12 | 96k | 4: +0.027, 8: +0.180, 12: +0.128 | sample +0.111 [+0.064, +0.157]<br>guarded +0.173 [+0.129, +0.217] | magnate +0.226 | 77% |
| v1_run1_vs_run1_canonical | run1 | run1 | canonical | 12 | 12 | 96k | 4: -0.017, 8: +0.054, 12: +0.164 | sample +0.186 [+0.142, +0.233]<br>guarded +0.182 [+0.136, +0.227] | exploit_run2 +0.275 | 67% |
| v2_run2_vs_run2_shaped | run2 | run2 | shaped | 13 | 12 | 99k | 4: -0.012, 8: +0.084, 12: +0.065 | sample +0.105 [+0.060, +0.150]<br>guarded +0.167 [+0.120, +0.212] | exploit_run2 +0.157 | 106% |

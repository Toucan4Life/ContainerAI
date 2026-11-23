Old: `reports/strategist/egta_h0`. New: `reports/strategist/egta_h0b`. Same horizon (2500 actions), same policies, same games on every shared seed (checked); compared on the shared seeds only: 400 at 3 seats, 400 at 4 seats, 400 at 5 seats. Intervals: 95%, 1,000 resamples of the seeds, the same seeds for both runs, per table size.

**3 seats** (400 seeds)

| quantity | old | new | new - old, points [95% interval] | new / old [95% interval] | reading |
|---|---|---|---|---|---|
| stalled games | 11 / 12,000 = 0.09% (11 of 400 seeds) | 196 / 12,000 = 1.63% (133 of 400 seeds) | +1.54 [+1.27, +1.82] | 17.8 [11.2, 36.6] | interval excludes 0 |
| completed games with a default | 811 / 11,989 = 6.76% (329 of 400 seeds) | 2,004 / 11,804 = 16.98% (398 of 400 seeds) | +10.21 [+9.27, +11.13] | 2.5 [2.3, 2.8] | interval excludes 0 |
| producer seats dead at the end | 189 / 11,992 = 1.58% (126 of 400 seeds) | 279 / 11,918 = 2.34% (167 of 400 seeds) | +0.76 [+0.34, +1.21] | 1.5 [1.2, 1.9] | interval excludes 0 |
| producer seats stuck at the end | 460 / 11,992 = 3.84% (226 of 400 seeds) | 723 / 11,918 = 6.07% (298 of 400 seeds) | +2.23 [+1.53, +2.90] | 1.6 [1.4, 1.8] | interval excludes 0 |
| showcase seats dead at the end | 0 / 11,993 = 0.00% (0 of 400 seeds) | 0 / 11,810 = 0.00% (0 of 400 seeds) | +0.00 [+0.00, +0.00] | — | none in either run: the chance that a seed has one is below 0.8% at 95% (a loose seed-level bound) |
| showcase seats stuck at the end | 0 / 11,993 = 0.00% (0 of 400 seeds) | 0 / 11,810 = 0.00% (0 of 400 seeds) | +0.00 [+0.00, +0.00] | — | none in either run: the chance that a seed has one is below 0.8% at 95% (a loose seed-level bound) |
| shipper seats dead at the end | 177 / 11,982 = 1.48% (126 of 400 seeds) | 572 / 11,684 = 4.90% (295 of 400 seeds) | +3.42 [+2.90, +3.91] | 3.3 [2.8, 4.0] | interval excludes 0 |
| shipper seats stuck at the end | 372 / 11,982 = 3.10% (226 of 400 seeds) | 1,311 / 11,684 = 11.22% (385 of 400 seeds) | +8.12 [+7.45, +8.87] | 3.6 [3.2, 4.1] | interval excludes 0 |

**4 seats** (400 seeds)

| quantity | old | new | new - old, points [95% interval] | new / old [95% interval] | reading |
|---|---|---|---|---|---|
| stalled games | 0 / 24,000 = 0.00% (0 of 400 seeds) | 0 / 24,000 = 0.00% (0 of 400 seeds) | +0.00 [+0.00, +0.00] | — | none in either run: the chance that a seed has one is below 0.8% at 95% (a loose seed-level bound) |
| completed games with a default | 695 / 24,000 = 2.90% (298 of 400 seeds) | 1,694 / 24,000 = 7.06% (390 of 400 seeds) | +4.16 [+3.74, +4.63] | 2.4 [2.2, 2.7] | interval excludes 0 |
| producer seats dead at the end | 205 / 32,000 = 0.64% (129 of 400 seeds) | 439 / 32,000 = 1.37% (231 of 400 seeds) | +0.73 [+0.55, +0.92] | 2.1 [1.8, 2.6] | interval excludes 0 |
| producer seats stuck at the end | 332 / 32,000 = 1.04% (182 of 400 seeds) | 700 / 32,000 = 2.19% (284 of 400 seeds) | +1.15 [+0.92, +1.38] | 2.1 [1.8, 2.5] | interval excludes 0 |
| showcase seats dead at the end | 0 / 32,000 = 0.00% (0 of 400 seeds) | 0 / 32,000 = 0.00% (0 of 400 seeds) | +0.00 [+0.00, +0.00] | — | none in either run: the chance that a seed has one is below 0.8% at 95% (a loose seed-level bound) |
| showcase seats stuck at the end | 0 / 32,000 = 0.00% (0 of 400 seeds) | 0 / 32,000 = 0.00% (0 of 400 seeds) | +0.00 [+0.00, +0.00] | — | none in either run: the chance that a seed has one is below 0.8% at 95% (a loose seed-level bound) |
| shipper seats dead at the end | 252 / 32,000 = 0.79% (159 of 400 seeds) | 677 / 32,000 = 2.12% (318 of 400 seeds) | +1.33 [+1.13, +1.54] | 2.7 [2.3, 3.2] | interval excludes 0 |
| shipper seats stuck at the end | 398 / 32,000 = 1.24% (223 of 400 seeds) | 1,061 / 32,000 = 3.32% (366 of 400 seeds) | +2.07 [+1.83, +2.33] | 2.7 [2.4, 3.0] | interval excludes 0 |

**5 seats** (400 seeds)

| quantity | old | new | new - old, points [95% interval] | new / old [95% interval] | reading |
|---|---|---|---|---|---|
| stalled games | 0 / 42,000 = 0.00% (0 of 400 seeds) | 0 / 42,000 = 0.00% (0 of 400 seeds) | +0.00 [+0.00, +0.00] | — | none in either run: the chance that a seed has one is below 0.8% at 95% (a loose seed-level bound) |
| completed games with a default | 685 / 42,000 = 1.63% (314 of 400 seeds) | 1,903 / 42,000 = 4.53% (397 of 400 seeds) | +2.90 [+2.64, +3.17] | 2.8 [2.5, 3.1] | interval excludes 0 |
| producer seats dead at the end | 166 / 70,000 = 0.24% (113 of 400 seeds) | 528 / 70,000 = 0.75% (274 of 400 seeds) | +0.52 [+0.43, +0.61] | 3.2 [2.6, 4.1] | interval excludes 0 |
| producer seats stuck at the end | 278 / 70,000 = 0.40% (174 of 400 seeds) | 758 / 70,000 = 1.08% (322 of 400 seeds) | +0.69 [+0.58, +0.80] | 2.7 [2.3, 3.2] | interval excludes 0 |
| showcase seats dead at the end | 0 / 70,000 = 0.00% (0 of 400 seeds) | 0 / 70,000 = 0.00% (0 of 400 seeds) | +0.00 [+0.00, +0.00] | — | none in either run: the chance that a seed has one is below 0.8% at 95% (a loose seed-level bound) |
| showcase seats stuck at the end | 0 / 70,000 = 0.00% (0 of 400 seeds) | 0 / 70,000 = 0.00% (0 of 400 seeds) | +0.00 [+0.00, +0.00] | — | none in either run: the chance that a seed has one is below 0.8% at 95% (a loose seed-level bound) |
| shipper seats dead at the end | 312 / 70,000 = 0.45% (196 of 400 seeds) | 844 / 70,000 = 1.21% (343 of 400 seeds) | +0.76 [+0.65, +0.86] | 2.7 [2.3, 3.1] | interval excludes 0 |
| shipper seats stuck at the end | 435 / 70,000 = 0.62% (253 of 400 seeds) | 1,229 / 70,000 = 1.76% (378 of 400 seeds) | +1.13 [+1.00, +1.26] | 2.8 [2.5, 3.2] | interval excludes 0 |


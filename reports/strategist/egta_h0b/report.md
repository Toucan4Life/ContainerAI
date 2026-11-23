# EGTA Lite, Stage D: cross-play of Producer, Showcase and Shipper

Population: producer, showcase, shipper (scripted, the hand-set profiles of `crates/container-bots/src/archetype.rs`; no Bank play, no nets). Peddler v2, Magnate and Consortium are not in it. Design: every composition of the three archetypes (10 at 3 seats, 15 at 4 seats, 21 at 5 seats), 400 seeds each (seed block `DEV`, offset 0), every composition seated in all its rotations on every seed, `Standard` length, horizon 2500 actions, commit `6e4ad64eee30e5f3a0b7c63f89a5e130785263d2+dirty-83b3e0474d08`. No training, no reward, no change to a heuristic.

**78,000 games: 77,804 completed, 196 stalled (0.251%).** Utility: winner share, a stalled game pays 0 to every seat. Every table size is its own game and is never pooled with another. Intervals are 95% percentile bootstraps over seeds (1,000 resamples), written `±` for half the interval; differences are paired on the seeds the two cells share. Tables are for one seat of the archetype named.

## 3 seats: the empirical payoff table

Winner share of one seat of each archetype in every composition (fair share 0.333). `games` are the n rotations of 400 seeds; `defaults` are completed games in which some seat missed an interest payment.

| composition | games | stalled | defaults | actions | producer | showcase | shipper |
|---|---|---|---|---|---|---|---|
| P P P | 1,200 | 9 | 18.6% | 595 | 0.331 ±0.003 | — | — |
| P P Sc | 1,200 | 4 | 12.1% | 545 | 0.356 ±0.011 | 0.285 ±0.023 | — |
| P P Sh | 1,200 | 6 | 14.8% | 487 | 0.404 ±0.010 | — | 0.188 ±0.021 |
| P Sc Sc | 1,200 | 11 | 14.4% | 441 | 0.100 ±0.018 | 0.445 ±0.009 | — |
| P Sc Sh | 1,200 | 21 | 18.9% | 428 | 0.275 ±0.025 | 0.502 ±0.029 | 0.206 ±0.024 |
| P Sh Sh | 1,200 | 3 | 14.6% | 379 | 0.378 ±0.026 | — | 0.310 ±0.013 |
| Sc Sc Sc | 1,200 | 0 | 0.0% | 501 | — | 0.333 ±0.000 | — |
| Sc Sc Sh | 1,200 | 34 | 25.1% | 546 | — | 0.391 ±0.011 | 0.190 ±0.022 |
| Sc Sh Sh | 1,200 | 75 | 35.8% | 661 | — | 0.235 ±0.024 | 0.351 ±0.012 |
| Sh Sh Sh | 1,200 | 33 | 16.7% | 490 | — | — | 0.324 ±0.005 |

Final ranking in the same tables: rank share (the share of the other seats a seat out-scores, ties half, 0.5 is par; a stalled game counts 0) ± half-interval, then the mean score margin (own total minus the mean of the others'; completed games).

| composition | producer | showcase | shipper |
|---|---|---|---|
| P P P | 0.496 ±0.004 · 0.0 | — | — |
| P P Sc | 0.506 ±0.010 · -1.9 | 0.483 ±0.021 · 3.9 | — |
| P P Sh | 0.573 ±0.011 · 7.8 | — | 0.347 ±0.021 · -15.5 |
| P Sc Sc | 0.193 ±0.018 · -27.7 | 0.647 ±0.009 · 13.8 | — |
| P Sc Sh | 0.435 ±0.022 · -6.5 | 0.666 ±0.021 · 16.8 | 0.373 ±0.024 · -10.3 |
| P Sh Sh | 0.530 ±0.021 · 0.8 | — | 0.483 ±0.011 · -0.4 |
| Sc Sc Sc | — | 0.500 ±0.000 · 0.0 | — |
| Sc Sc Sh | — | 0.577 ±0.012 · 12.5 | 0.304 ±0.021 · -25.0 |
| Sc Sh Sh | — | 0.429 ±0.021 · 8.7 | 0.489 ±0.012 · -4.4 |
| Sh Sh Sh | — | — | 0.486 ±0.008 · 0.0 |

## 4 seats: the empirical payoff table

Winner share of one seat of each archetype in every composition (fair share 0.250). `games` are the n rotations of 400 seeds; `defaults` are completed games in which some seat missed an interest payment.

| composition | games | stalled | defaults | actions | producer | showcase | shipper |
|---|---|---|---|---|---|---|---|
| P P P P | 1,600 | 0 | 4.8% | 792 | 0.250 ±0.000 | — | — |
| P P P Sc | 1,600 | 0 | 6.4% | 815 | 0.267 ±0.006 | 0.200 ±0.019 | — |
| P P P Sh | 1,600 | 0 | 6.8% | 722 | 0.295 ±0.005 | — | 0.114 ±0.016 |
| P P Sc Sc | 1,600 | 0 | 6.3% | 796 | 0.229 ±0.014 | 0.271 ±0.014 | — |
| P P Sc Sh | 1,600 | 0 | 9.2% | 707 | 0.281 ±0.012 | 0.277 ±0.022 | 0.162 ±0.018 |
| P P Sh Sh | 1,600 | 0 | 4.8% | 570 | 0.177 ±0.013 | — | 0.323 ±0.013 |
| P Sc Sc Sc | 1,600 | 0 | 6.0% | 706 | 0.162 ±0.018 | 0.279 ±0.006 | — |
| P Sc Sc Sh | 1,600 | 0 | 11.1% | 655 | 0.241 ±0.020 | 0.292 ±0.012 | 0.175 ±0.018 |
| P Sc Sh Sh | 1,600 | 0 | 11.8% | 615 | 0.278 ±0.022 | 0.130 ±0.016 | 0.296 ±0.012 |
| P Sh Sh Sh | 1,600 | 0 | 6.1% | 507 | 0.148 ±0.016 | — | 0.284 ±0.005 |
| Sc Sc Sc Sc | 1,600 | 0 | 0.0% | 796 | — | 0.250 ±0.000 | — |
| Sc Sc Sc Sh | 1,600 | 0 | 7.0% | 806 | — | 0.259 ±0.007 | 0.223 ±0.021 |
| Sc Sc Sh Sh | 1,600 | 0 | 15.1% | 781 | — | 0.197 ±0.014 | 0.303 ±0.014 |
| Sc Sh Sh Sh | 1,600 | 0 | 8.2% | 742 | — | 0.034 ±0.009 | 0.322 ±0.003 |
| Sh Sh Sh Sh | 1,600 | 0 | 2.2% | 533 | — | — | 0.250 ±0.000 |

Final ranking in the same tables: rank share (the share of the other seats a seat out-scores, ties half, 0.5 is par; a stalled game counts 0) ± half-interval, then the mean score margin (own total minus the mean of the others'; completed games).

| composition | producer | showcase | shipper |
|---|---|---|---|
| P P P P | 0.500 ±0.000 · -0.0 | — | — |
| P P P Sc | 0.515 ±0.006 · 0.3 | 0.456 ±0.017 · -0.8 | — |
| P P P Sh | 0.562 ±0.005 · 6.6 | — | 0.314 ±0.016 · -19.7 |
| P P Sc Sc | 0.463 ±0.012 · -4.6 | 0.537 ±0.012 · 4.6 | — |
| P P Sc Sh | 0.530 ±0.011 · 2.9 | 0.558 ±0.017 · 7.1 | 0.382 ±0.019 · -12.8 |
| P P Sh Sh | 0.427 ±0.011 · -5.9 | — | 0.573 ±0.011 · 5.9 |
| P Sc Sc Sc | 0.372 ±0.017 · -15.6 | 0.543 ±0.006 · 5.2 | — |
| P Sc Sc Sh | 0.481 ±0.017 · -2.5 | 0.567 ±0.010 · 7.7 | 0.385 ±0.018 · -13.0 |
| P Sc Sh Sh | 0.519 ±0.019 · 0.2 | 0.371 ±0.016 · -7.1 | 0.555 ±0.010 · 3.5 |
| P Sh Sh Sh | 0.374 ±0.017 · -9.2 | — | 0.542 ±0.006 · 3.1 |
| Sc Sc Sc Sc | — | 0.500 ±0.000 · -0.0 | — |
| Sc Sc Sc Sh | — | 0.513 ±0.006 · 3.0 | 0.460 ±0.017 · -8.9 |
| Sc Sc Sh Sh | — | 0.459 ±0.012 · 0.5 | 0.541 ±0.012 · -0.5 |
| Sc Sh Sh Sh | — | 0.197 ±0.014 · -24.8 | 0.601 ±0.005 · 8.3 |
| Sh Sh Sh Sh | — | — | 0.500 ±0.000 · 0.0 |

## 5 seats: the empirical payoff table

Winner share of one seat of each archetype in every composition (fair share 0.200). `games` are the n rotations of 400 seeds; `defaults` are completed games in which some seat missed an interest payment.

| composition | games | stalled | defaults | actions | producer | showcase | shipper |
|---|---|---|---|---|---|---|---|
| P P P P P | 2,000 | 0 | 2.8% | 1120 | 0.200 ±0.000 | — | — |
| P P P P Sc | 2,000 | 0 | 3.1% | 1057 | 0.200 ±0.004 | 0.200 ±0.016 | — |
| P P P P Sh | 2,000 | 0 | 2.5% | 943 | 0.218 ±0.004 | — | 0.128 ±0.014 |
| P P P Sc Sc | 2,000 | 0 | 4.0% | 1062 | 0.199 ±0.007 | 0.202 ±0.011 | — |
| P P P Sc Sh | 2,000 | 0 | 2.9% | 943 | 0.198 ±0.007 | 0.238 ±0.020 | 0.168 ±0.018 |
| P P P Sh Sh | 2,000 | 0 | 2.0% | 825 | 0.142 ±0.007 | — | 0.287 ±0.011 |
| P P Sc Sc Sc | 2,000 | 0 | 4.3% | 997 | 0.220 ±0.011 | 0.187 ±0.007 | — |
| P P Sc Sc Sh | 2,000 | 0 | 5.4% | 907 | 0.201 ±0.011 | 0.215 ±0.011 | 0.169 ±0.017 |
| P P Sc Sh Sh | 2,000 | 0 | 5.2% | 795 | 0.144 ±0.010 | 0.129 ±0.015 | 0.292 ±0.011 |
| P P Sh Sh Sh | 2,000 | 0 | 3.8% | 719 | 0.117 ±0.009 | — | 0.255 ±0.006 |
| P Sc Sc Sc Sc | 2,000 | 0 | 3.1% | 990 | 0.235 ±0.019 | 0.191 ±0.005 | — |
| P Sc Sc Sc Sh | 2,000 | 0 | 9.4% | 897 | 0.206 ±0.017 | 0.205 ±0.007 | 0.179 ±0.018 |
| P Sc Sc Sh Sh | 2,000 | 0 | 8.7% | 848 | 0.189 ±0.017 | 0.147 ±0.010 | 0.259 ±0.011 |
| P Sc Sh Sh Sh | 2,000 | 0 | 7.0% | 768 | 0.128 ±0.015 | 0.104 ±0.014 | 0.256 ±0.006 |
| P Sh Sh Sh Sh | 2,000 | 0 | 5.3% | 621 | 0.030 ±0.008 | — | 0.242 ±0.002 |
| Sc Sc Sc Sc Sc | 2,000 | 0 | 0.0% | 1032 | — | 0.200 ±0.000 | — |
| Sc Sc Sc Sc Sh | 2,000 | 0 | 3.5% | 999 | — | 0.183 ±0.005 | 0.270 ±0.020 |
| Sc Sc Sc Sh Sh | 2,000 | 0 | 6.3% | 955 | — | 0.142 ±0.007 | 0.287 ±0.011 |
| Sc Sc Sh Sh Sh | 2,000 | 0 | 5.5% | 906 | — | 0.110 ±0.009 | 0.260 ±0.006 |
| Sc Sh Sh Sh Sh | 2,000 | 0 | 5.2% | 851 | — | 0.014 ±0.005 | 0.246 ±0.001 |
| Sh Sh Sh Sh Sh | 2,000 | 0 | 5.0% | 766 | — | — | 0.200 ±0.000 |

Final ranking in the same tables: rank share (the share of the other seats a seat out-scores, ties half, 0.5 is par; a stalled game counts 0) ± half-interval, then the mean score margin (own total minus the mean of the others'; completed games).

| composition | producer | showcase | shipper |
|---|---|---|---|
| P P P P P | 0.500 ±0.000 · 0.0 | — | — |
| P P P P Sc | 0.500 ±0.004 · -0.1 | 0.499 ±0.015 · 0.3 | — |
| P P P P Sh | 0.526 ±0.004 · 3.0 | — | 0.394 ±0.015 · -11.9 |
| P P P Sc Sc | 0.496 ±0.006 · -1.0 | 0.507 ±0.009 · 1.4 | — |
| P P P Sc Sh | 0.501 ±0.007 · 0.6 | 0.544 ±0.017 · 4.2 | 0.454 ±0.017 · -6.1 |
| P P P Sh Sh | 0.426 ±0.006 · -6.7 | — | 0.610 ±0.009 · 10.1 |
| P P Sc Sc Sc | 0.517 ±0.010 · -0.1 | 0.489 ±0.006 · 0.0 | — |
| P P Sc Sc Sh | 0.501 ±0.010 · 0.4 | 0.524 ±0.010 · 2.7 | 0.450 ±0.016 · -6.1 |
| P P Sc Sh Sh | 0.445 ±0.009 · -4.7 | 0.427 ±0.015 · -5.7 | 0.592 ±0.010 · 7.6 |
| P P Sh Sh Sh | 0.385 ±0.009 · -9.6 | — | 0.577 ±0.006 · 6.4 |
| P Sc Sc Sc Sc | 0.529 ±0.015 · 0.4 | 0.493 ±0.004 · -0.1 | — |
| P Sc Sc Sc Sh | 0.512 ±0.015 · 0.7 | 0.511 ±0.007 · 2.3 | 0.456 ±0.019 · -7.7 |
| P Sc Sc Sh Sh | 0.496 ±0.014 · -0.3 | 0.447 ±0.011 · -3.8 | 0.555 ±0.010 · 3.9 |
| P Sc Sh Sh Sh | 0.421 ±0.015 · -6.9 | 0.376 ±0.017 · -10.6 | 0.568 ±0.007 · 5.8 |
| P Sh Sh Sh Sh | 0.198 ±0.012 · -25.3 | — | 0.575 ±0.003 · 6.3 |
| Sc Sc Sc Sc Sc | — | 0.500 ±0.000 · 0.0 | — |
| Sc Sc Sc Sc Sh | — | 0.485 ±0.004 · -0.6 | 0.560 ±0.016 · 2.2 |
| Sc Sc Sc Sh Sh | — | 0.445 ±0.006 · -3.9 | 0.582 ±0.009 · 5.9 |
| Sc Sc Sh Sh Sh | — | 0.396 ±0.009 · -8.6 | 0.570 ±0.006 · 5.7 |
| Sc Sh Sh Sh Sh | — | 0.209 ±0.010 · -25.8 | 0.573 ±0.003 · 6.5 |
| Sh Sh Sh Sh Sh | — | — | 0.500 ±0.000 · 0.0 |

## Identity: does the Producer produce more than the others?

In every table that seats the Producer next to another archetype, the containers a Producer seat produces per game and the paired difference to the other archetype's seats of the same games (completed games; a positive difference means the Producer produces more).

| seats | composition | producer produced | showcase produced | shipper produced | producer − showcase | producer − shipper |
|---|---|---|---|---|---|---|
| 3 | P P Sc | 17.23 | 2.28 | — | 14.94 ±0.10 | — |
| 3 | P P Sh | 16.90 | — | 2.70 | — | 14.20 ±0.13 |
| 3 | P Sc Sc | 19.67 | 4.58 | — | 15.09 ±0.19 | — |
| 3 | P Sc Sh | 19.78 | 4.22 | 5.69 | 15.56 ±0.21 | 14.09 ±0.34 |
| 3 | P Sh Sh | 19.86 | — | 5.66 | — | 14.20 ±0.21 |
| 4 | P P P Sc | 15.44 | 2.66 | — | 12.78 ±0.10 | — |
| 4 | P P P Sh | 15.36 | — | 3.05 | — | 12.32 ±0.14 |
| 4 | P P Sc Sc | 19.59 | 4.53 | — | 15.06 ±0.23 | — |
| 4 | P P Sc Sh | 19.41 | 4.28 | 4.12 | 15.13 ±0.21 | 15.29 ±0.25 |
| 4 | P P Sh Sh | 20.08 | — | 2.56 | — | 17.52 ±0.22 |
| 4 | P Sc Sc Sc | 25.47 | 5.95 | — | 19.52 ±0.15 | — |
| 4 | P Sc Sc Sh | 24.84 | 5.99 | 6.21 | 18.85 ±0.15 | 18.63 ±0.25 |
| 4 | P Sc Sh Sh | 24.30 | 6.46 | 6.12 | 17.85 ±0.21 | 18.19 ±0.27 |
| 4 | P Sh Sh Sh | 24.04 | — | 6.07 | — | 17.97 ±0.19 |
| 5 | P P P P Sc | 14.93 | 2.56 | — | 12.37 ±0.07 | — |
| 5 | P P P P Sh | 14.77 | — | 3.51 | — | 11.26 ±0.13 |
| 5 | P P P Sc Sc | 17.79 | 3.83 | — | 13.96 ±0.11 | — |
| 5 | P P P Sc Sh | 18.08 | 3.52 | 3.27 | 14.55 ±0.12 | 14.80 ±0.17 |
| 5 | P P P Sh Sh | 18.72 | — | 2.61 | — | 16.11 ±0.12 |
| 5 | P P Sc Sc Sc | 21.38 | 4.63 | — | 16.75 ±0.14 | — |
| 5 | P P Sc Sc Sh | 21.91 | 4.55 | 4.54 | 17.36 ±0.15 | 17.37 ±0.22 |
| 5 | P P Sc Sh Sh | 22.60 | 4.05 | 3.62 | 18.55 ±0.18 | 18.97 ±0.19 |
| 5 | P P Sh Sh Sh | 22.83 | — | 3.40 | — | 19.43 ±0.16 |
| 5 | P Sc Sc Sc Sc | 26.11 | 7.25 | — | 18.86 ±0.18 | — |
| 5 | P Sc Sc Sc Sh | 26.30 | 6.30 | 8.17 | 19.99 ±0.18 | 18.12 ±0.31 |
| 5 | P Sc Sc Sh Sh | 26.27 | 6.42 | 8.03 | 19.85 ±0.21 | 18.24 ±0.32 |
| 5 | P Sc Sh Sh Sh | 26.19 | 5.72 | 7.23 | 20.47 ±0.26 | 18.96 ±0.31 |
| 5 | P Sh Sh Sh Sh | 26.47 | — | 5.83 | — | 20.64 ±0.20 |

Tables in which the lower bound of the difference is above zero: 3 seats vs shipper: 3 of 3; 3 seats vs showcase: 3 of 3; 4 seats vs shipper: 6 of 6; 4 seats vs showcase: 6 of 6; 5 seats vs shipper: 10 of 10; 5 seats vs showcase: 10 of 10.

### Each archetype's own metric in the mixed tables

Seats of each archetype pooled over the tables of one size that seat at least two archetypes (mirrors left out; compositions weighted by their seats).

3 seats:

| metric | producer | showcase | shipper |
|---|---|---|---|
| produced | 18.22 ±0.04 | 6.22 ±0.06 | 8.91 ±0.09 |
| sold | 24.21 ±0.08 | 23.25 ±0.12 | 20.59 ±0.14 |
| sold/produced | 1.33 ±0.00 | 3.74 ±0.02 | 2.31 ±0.01 |
| avg sale $ | 1.44 ±0.00 | 2.45 ±0.00 | 1.91 ±0.00 |
| unsold at the end | 2.08 ±0.03 | 1.16 ±0.03 | 1.36 ±0.03 |
| cheap shelf stock (own lots at $3 or less, per decision) | 2.36 ±0.02 | 2.74 ±0.02 | 1.36 ±0.01 |
| colors sold | 4.03 ±0.02 | 4.37 ±0.01 | 4.21 ±0.02 |
| reprices | 1.5 ±0.1 | 9.9 ±0.1 | 3.0 ±0.1 |
| deliveries | 8.83 ±0.06 | 7.76 ±0.06 | 10.16 ±0.06 |
| containers per delivery | 1.26 ±0.00 | 1.20 ±0.00 | 1.16 ±0.00 |
| delivery cash minus purchases | 15.2 ±0.4 | -9.5 ±0.3 | 7.3 ±0.4 |
| win share | 0.324 ±0.007 | 0.385 ±0.007 | 0.272 ±0.008 |
| score | 63.7 ±0.7 | 66.7 ±0.5 | 58.4 ±0.8 |

4 seats:

| metric | producer | showcase | shipper |
|---|---|---|---|
| produced | 19.33 ±0.06 | 7.24 ±0.04 | 8.15 ±0.05 |
| sold | 26.40 ±0.07 | 25.92 ±0.08 | 20.19 ±0.09 |
| sold/produced | 1.37 ±0.00 | 3.58 ±0.01 | 2.48 ±0.01 |
| avg sale $ | 1.48 ±0.00 | 2.55 ±0.00 | 1.94 ±0.00 |
| unsold at the end | 1.44 ±0.01 | 0.87 ±0.01 | 1.11 ±0.01 |
| cheap shelf stock (own lots at $3 or less, per decision) | 2.05 ±0.01 | 2.18 ±0.01 | 0.94 ±0.01 |
| colors sold | 4.23 ±0.01 | 4.81 ±0.01 | 4.15 ±0.01 |
| reprices | 1.4 ±0.0 | 7.1 ±0.1 | 1.8 ±0.0 |
| deliveries | 10.07 ±0.04 | 8.56 ±0.03 | 10.41 ±0.03 |
| containers per delivery | 1.09 ±0.00 | 1.20 ±0.00 | 1.26 ±0.00 |
| delivery cash minus purchases | 47.7 ±0.3 | 18.6 ±0.3 | 45.5 ±0.3 |
| win share | 0.243 ±0.004 | 0.236 ±0.004 | 0.271 ±0.004 |
| score | 94.9 ±0.5 | 93.6 ±0.4 | 92.4 ±0.5 |

5 seats:

| metric | producer | showcase | shipper |
|---|---|---|---|
| produced | 19.71 ±0.03 | 7.18 ±0.03 | 7.52 ±0.04 |
| sold | 27.65 ±0.04 | 25.79 ±0.06 | 19.20 ±0.08 |
| sold/produced | 1.40 ±0.00 | 3.59 ±0.01 | 2.55 ±0.01 |
| avg sale $ | 1.49 ±0.00 | 2.53 ±0.00 | 1.92 ±0.00 |
| unsold at the end | 1.13 ±0.01 | 0.72 ±0.01 | 0.95 ±0.01 |
| cheap shelf stock (own lots at $3 or less, per decision) | 1.93 ±0.00 | 1.99 ±0.01 | 0.81 ±0.00 |
| colors sold | 4.30 ±0.01 | 4.71 ±0.00 | 4.01 ±0.01 |
| reprices | 1.3 ±0.0 | 4.9 ±0.0 | 1.5 ±0.0 |
| deliveries | 9.84 ±0.02 | 8.59 ±0.02 | 9.84 ±0.02 |
| containers per delivery | 1.09 ±0.00 | 1.19 ±0.00 | 1.39 ±0.00 |
| delivery cash minus purchases | 65.0 ±0.2 | 35.9 ±0.2 | 67.3 ±0.2 |
| win share | 0.181 ±0.002 | 0.171 ±0.002 | 0.248 ±0.002 |
| score | 109.2 ±0.3 | 101.3 ±0.3 | 106.1 ±0.3 |

## Economic outcomes of one archetype against each field

A seat of the focal archetype at a table whose other seats are all the second archetype (the mirror row is the all-same table). Counts are per game, completed games only; average prices are total income over total containers; `unsold` is the exact stock at the end of seats that did not default.

### 3 seats

Result:

| focal seat | other seats | won | rank share | score margin | score | final cash |
|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 0.331 ±0.003 | 0.496 | 0.0 | 79.90 | 31.28 |
| producer | showcase | 0.100 ±0.017 | 0.193 | -27.7 | 30.79 | 14.51 |
| producer | shipper | 0.378 ±0.027 | 0.530 | 0.8 | 60.89 | 25.34 |
| showcase | producer | 0.285 ±0.024 | 0.483 | 3.9 | 74.96 | 30.19 |
| **showcase** | showcase (mirror) | 0.333 ±0.000 | 0.500 | 0.0 | 61.34 | 22.85 |
| showcase | shipper | 0.235 ±0.024 | 0.429 | 8.7 | 75.83 | 24.10 |
| shipper | producer | 0.188 ±0.020 | 0.347 | -15.5 | 65.33 | 21.14 |
| shipper | showcase | 0.190 ±0.021 | 0.304 | -25.0 | 42.21 | 16.21 |
| **shipper** | shipper (mirror) | 0.324 ±0.005 | 0.486 | 0.0 | 85.56 | 27.17 |

Market:

| focal seat | other seats | produced | sold | sold/produced | avg sale $ | bought | avg purchase $ | unsold | reprices |
|---|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 13.11 | 25.93 | 1.98 | 1.88 | 25.93 | 1.88 | 1.95 | 2.81 |
| producer | showcase | 19.67 | 20.90 | 1.06 | 1.10 | 10.69 | 2.54 | 1.85 | 1.86 |
| producer | shipper | 19.86 | 22.98 | 1.16 | 1.24 | 13.98 | 1.87 | 2.54 | 1.09 |
| showcase | producer | 2.28 | 17.67 | 7.75 | 2.42 | 27.12 | 1.80 | 1.54 | 6.92 |
| **showcase** | showcase (mirror) | 10.65 | 22.20 | 2.08 | 2.35 | 22.20 | 2.35 | 1.17 | 4.94 |
| showcase | shipper | 9.05 | 36.93 | 4.08 | 2.48 | 33.71 | 1.92 | 0.72 | 29.40 |
| shipper | producer | 2.70 | 24.79 | 9.17 | 1.94 | 30.83 | 1.55 | 1.22 | 3.81 |
| shipper | showcase | 13.97 | 17.31 | 1.24 | 1.91 | 16.54 | 2.61 | 1.38 | 3.20 |
| **shipper** | shipper (mirror) | 13.47 | 27.55 | 2.05 | 1.93 | 27.55 | 1.93 | 1.36 | 4.85 |

Delivery and finance:

| focal seat | other seats | deliveries | per delivery | loans taken | buy-outs | factories | warehouses | defaulted |
|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 11.25 | 1.09 | 0.59 | 2.04 | 1.00 | 1.24 | 6.3% |
| producer | showcase | 7.93 | 1.12 | 0.90 | 1.39 | 1.00 | 1.79 | 14.4% |
| producer | shipper | 6.37 | 1.48 | 0.48 | 0.97 | 1.00 | 1.50 | 3.5% |
| showcase | producer | 9.32 | 1.20 | 0.00 | 1.45 | 0.00 | 3.01 | 0.0% |
| **showcase** | showcase (mirror) | 9.33 | 1.12 | 0.00 | 1.74 | 0.29 | 3.00 | 0.0% |
| showcase | shipper | 5.61 | 1.09 | 0.00 | 1.09 | 0.00 | 3.45 | 0.0% |
| shipper | producer | 8.30 | 1.03 | 1.12 | 1.39 | 0.54 | 2.01 | 10.1% |
| shipper | showcase | 12.68 | 1.04 | 1.79 | 2.85 | 1.00 | 2.58 | 25.1% |
| **shipper** | shipper (mirror) | 6.82 | 1.93 | 1.40 | 1.02 | 1.00 | 2.00 | 5.6% |

### 4 seats

Result:

| focal seat | other seats | won | rank share | score margin | score | final cash |
|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 0.250 ±0.000 | 0.500 | -0.0 | 102.80 | 53.01 |
| producer | showcase | 0.162 ±0.018 | 0.372 | -15.6 | 72.06 | 30.14 |
| producer | shipper | 0.148 ±0.016 | 0.374 | -9.2 | 78.75 | 40.31 |
| showcase | producer | 0.200 ±0.020 | 0.456 | -0.8 | 99.33 | 52.28 |
| **showcase** | showcase (mirror) | 0.250 ±0.000 | 0.500 | -0.0 | 102.18 | 46.91 |
| showcase | shipper | 0.034 ±0.008 | 0.197 | -24.8 | 76.73 | 36.11 |
| shipper | producer | 0.114 ±0.016 | 0.314 | -19.7 | 90.08 | 32.53 |
| shipper | showcase | 0.223 ±0.020 | 0.460 | -8.9 | 94.61 | 36.43 |
| **shipper** | shipper (mirror) | 0.250 ±0.000 | 0.500 | 0.0 | 102.00 | 42.83 |

Market:

| focal seat | other seats | produced | sold | sold/produced | avg sale $ | bought | avg purchase $ | unsold | reprices |
|---|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 12.33 | 25.46 | 2.06 | 1.94 | 25.46 | 1.94 | 1.28 | 0.99 |
| producer | showcase | 25.47 | 26.82 | 1.05 | 1.10 | 12.70 | 2.64 | 1.79 | 2.23 |
| producer | shipper | 24.04 | 29.23 | 1.22 | 1.32 | 13.63 | 1.91 | 2.16 | 1.69 |
| showcase | producer | 2.66 | 19.95 | 7.49 | 2.63 | 29.56 | 1.83 | 1.11 | 6.69 |
| **showcase** | showcase (mirror) | 12.34 | 25.90 | 2.10 | 2.41 | 25.90 | 2.41 | 0.97 | 4.36 |
| showcase | shipper | 9.05 | 43.10 | 4.76 | 2.69 | 38.16 | 1.96 | 0.76 | 19.30 |
| shipper | producer | 3.05 | 31.39 | 10.30 | 1.97 | 39.31 | 1.56 | 1.19 | 2.08 |
| shipper | showcase | 16.24 | 19.38 | 1.19 | 1.90 | 19.05 | 2.73 | 1.23 | 3.73 |
| **shipper** | shipper (mirror) | 12.27 | 25.22 | 2.06 | 1.97 | 25.22 | 1.97 | 1.48 | 0.95 |

Delivery and finance:

| focal seat | other seats | deliveries | per delivery | loans taken | buy-outs | factories | warehouses | defaulted |
|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 11.70 | 1.03 | 0.35 | 0.73 | 1.00 | 1.00 | 1.2% |
| producer | showcase | 10.34 | 1.03 | 0.64 | 0.78 | 1.00 | 1.56 | 6.0% |
| producer | shipper | 5.27 | 1.39 | 0.39 | 0.15 | 1.00 | 1.30 | 1.9% |
| showcase | producer | 10.27 | 1.18 | 0.00 | 0.54 | 0.00 | 3.17 | 0.0% |
| **showcase** | showcase (mirror) | 11.52 | 1.07 | 0.00 | 0.78 | 0.31 | 3.00 | 0.0% |
| showcase | shipper | 3.05 | 1.43 | 0.00 | 0.08 | 0.00 | 3.65 | 0.0% |
| shipper | producer | 9.77 | 1.11 | 1.13 | 0.66 | 0.62 | 2.00 | 5.2% |
| shipper | showcase | 14.65 | 1.08 | 1.44 | 1.31 | 1.00 | 2.64 | 7.0% |
| **shipper** | shipper (mirror) | 5.91 | 2.00 | 1.24 | 0.16 | 1.00 | 2.06 | 0.6% |

### 5 seats

Result:

| focal seat | other seats | won | rank share | score margin | score | final cash |
|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 0.200 ±0.000 | 0.500 | 0.0 | 133.08 | 72.71 |
| producer | showcase | 0.234 ±0.019 | 0.529 | 0.4 | 97.63 | 52.11 |
| producer | shipper | 0.030 ±0.008 | 0.198 | -25.3 | 69.43 | 36.11 |
| showcase | producer | 0.200 ±0.017 | 0.499 | 0.3 | 119.19 | 67.53 |
| **showcase** | showcase (mirror) | 0.200 ±0.000 | 0.500 | 0.0 | 110.31 | 57.39 |
| showcase | shipper | 0.015 ±0.005 | 0.209 | -25.8 | 83.82 | 43.65 |
| shipper | producer | 0.128 ±0.014 | 0.394 | -11.9 | 114.00 | 47.75 |
| shipper | showcase | 0.270 ±0.019 | 0.560 | 2.2 | 108.11 | 51.61 |
| **shipper** | shipper (mirror) | 0.200 ±0.000 | 0.500 | 0.0 | 127.33 | 57.93 |

Market:

| focal seat | other seats | produced | sold | sold/produced | avg sale $ | bought | avg purchase $ | unsold | reprices |
|---|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 13.42 | 27.92 | 2.08 | 1.93 | 27.92 | 1.93 | 1.21 | 1.08 |
| producer | showcase | 26.11 | 28.22 | 1.08 | 1.10 | 14.63 | 2.65 | 1.11 | 2.24 |
| producer | shipper | 26.47 | 35.40 | 1.34 | 1.43 | 14.03 | 1.85 | 1.64 | 1.88 |
| showcase | producer | 2.56 | 21.36 | 8.35 | 2.59 | 31.35 | 1.81 | 0.83 | 4.42 |
| **showcase** | showcase (mirror) | 11.93 | 25.21 | 2.11 | 2.38 | 25.21 | 2.38 | 0.81 | 3.93 |
| showcase | shipper | 7.64 | 50.21 | 6.57 | 2.75 | 46.40 | 1.93 | 0.78 | 7.72 |
| shipper | producer | 3.51 | 34.62 | 9.87 | 1.96 | 43.33 | 1.56 | 0.94 | 1.51 |
| shipper | showcase | 10.48 | 13.35 | 1.27 | 1.88 | 19.55 | 2.73 | 0.96 | 2.17 |
| **shipper** | shipper (mirror) | 13.70 | 28.43 | 2.08 | 1.94 | 28.43 | 1.94 | 1.30 | 0.77 |

Delivery and finance:

| focal seat | other seats | deliveries | per delivery | loans taken | buy-outs | factories | warehouses | defaulted |
|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 12.54 | 1.05 | 0.30 | 0.36 | 1.00 | 1.00 | 0.5% |
| producer | showcase | 12.34 | 1.01 | 0.45 | 0.35 | 1.00 | 1.20 | 3.1% |
| producer | shipper | 3.38 | 1.33 | 0.53 | 0.04 | 1.00 | 1.75 | 2.2% |
| showcase | producer | 10.40 | 1.22 | 0.00 | 0.24 | 0.00 | 3.06 | 0.0% |
| **showcase** | showcase (mirror) | 11.44 | 1.06 | 0.00 | 0.33 | 0.37 | 3.00 | 0.0% |
| showcase | shipper | 2.96 | 1.37 | 0.00 | 0.03 | 0.02 | 3.78 | 0.0% |
| shipper | producer | 9.68 | 1.27 | 0.81 | 0.27 | 0.61 | 2.00 | 1.7% |
| shipper | showcase | 14.73 | 1.14 | 1.28 | 0.54 | 1.00 | 2.59 | 3.5% |
| **shipper** | shipper (mirror) | 6.72 | 2.00 | 1.27 | 0.08 | 1.00 | 2.12 | 1.1% |

## Cash ledger flows, apart from the reward

Dollars per game and seat from the counters of the engine (`training/ledger.py`). In: containers sold, delivery proceeds (twice the winning bid), wages received, loan principal not repaid. Out: cargo auctions won, containers bought, buy-outs, factories and warehouses built, wages paid, interest due. `net cash change` is final cash minus the $20 start. None of these sums is a profit: the reward is the score, which also counts the Island, leftovers and loans, and `delivery_cash_minus_purchases` ignores most of this table. `unpaid` is the interest due that a defaulting seat did not pay (what the ledger cannot explain).

### 3 seats

| focal seat | other seats | sales | delivery | wages in | loans | auctions | purchases | buy-outs | builds | wages out | interest | net cash change | unpaid |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 48.80 | 58.06 | 7.61 | 2.07 | 29.03 | 48.80 | 4.06 | 11.20 | 7.61 | 5.73 | 11.28 | 1.17 |
| producer | showcase | 23.07 | 38.12 | 4.85 | 6.00 | 16.68 | 27.16 | 2.13 | 13.97 | 10.47 | 11.97 | -5.49 | 4.85 |
| producer | shipper | 28.41 | 42.51 | 3.87 | 2.00 | 17.38 | 26.09 | 2.23 | 12.52 | 10.49 | 3.96 | 5.34 | 1.22 |
| showcase | producer | 42.70 | 53.29 | 9.04 | 0.00 | 25.42 | 48.86 | 3.11 | 15.16 | 2.28 | 0.00 | 10.19 | 0.00 |
| **showcase** | showcase (mirror) | 52.16 | 44.95 | 8.40 | 0.00 | 22.47 | 52.16 | 2.91 | 16.71 | 8.40 | 0.00 | 2.85 | 0.00 |
| showcase | shipper | 91.74 | 31.45 | 6.51 | 0.00 | 31.74 | 64.66 | 1.99 | 18.15 | 9.05 | 0.00 | 4.10 | 0.00 |
| shipper | producer | 48.21 | 42.52 | 8.89 | 4.26 | 31.85 | 47.83 | 2.77 | 12.30 | 2.00 | 7.84 | 1.14 | 1.84 |
| shipper | showcase | 32.99 | 56.17 | 9.25 | 7.36 | 25.22 | 43.25 | 4.15 | 18.47 | 7.03 | 17.25 | -3.79 | 5.80 |
| **shipper** | shipper (mirror) | 53.08 | 58.94 | 7.73 | 2.60 | 29.47 | 53.08 | 3.38 | 15.03 | 7.73 | 7.54 | 7.17 | 1.06 |

### 4 seats

| focal seat | other seats | sales | delivery | wages in | loans | auctions | purchases | buy-outs | builds | wages out | interest | net cash change | unpaid |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 49.39 | 93.28 | 6.73 | 0.41 | 46.64 | 49.39 | 2.34 | 9.99 | 6.73 | 1.89 | 33.01 | 0.17 |
| producer | showcase | 29.52 | 75.41 | 6.40 | 2.31 | 37.82 | 33.59 | 1.99 | 12.78 | 13.20 | 6.90 | 10.14 | 2.77 |
| producer | shipper | 38.61 | 57.17 | 3.82 | 0.94 | 27.37 | 26.05 | 0.59 | 11.52 | 12.52 | 2.65 | 20.32 | 0.47 |
| showcase | producer | 52.46 | 90.12 | 8.54 | 0.00 | 43.89 | 54.17 | 1.86 | 16.25 | 2.66 | 0.00 | 32.28 | 0.00 |
| **showcase** | showcase (mirror) | 62.39 | 92.08 | 9.27 | 0.00 | 46.04 | 62.39 | 2.27 | 16.86 | 9.27 | 0.00 | 26.91 | 0.00 |
| showcase | shipper | 115.86 | 35.01 | 5.50 | 0.00 | 36.73 | 74.61 | 0.28 | 19.59 | 9.03 | 0.00 | 16.11 | 0.00 |
| shipper | producer | 61.81 | 81.18 | 8.33 | 2.16 | 58.48 | 61.23 | 2.22 | 12.71 | 1.99 | 5.36 | 12.53 | 1.03 |
| shipper | showcase | 36.79 | 111.74 | 11.52 | 2.25 | 57.13 | 52.03 | 3.60 | 18.81 | 8.18 | 8.02 | 16.43 | 1.93 |
| **shipper** | shipper (mirror) | 49.62 | 82.61 | 6.40 | 0.51 | 41.31 | 49.62 | 0.78 | 15.36 | 6.40 | 2.92 | 22.83 | 0.08 |

### 5 seats

| focal seat | other seats | sales | delivery | wages in | loans | auctions | purchases | buy-outs | builds | wages out | interest | net cash change | unpaid |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 53.83 | 130.06 | 7.17 | 0.19 | 65.03 | 53.83 | 1.46 | 10.00 | 7.17 | 1.21 | 52.71 | 0.15 |
| producer | showcase | 31.05 | 110.69 | 8.39 | 1.05 | 51.56 | 38.75 | 1.14 | 11.00 | 14.33 | 3.59 | 32.11 | 1.32 |
| producer | shipper | 50.68 | 44.85 | 3.02 | 1.28 | 27.45 | 25.95 | 0.20 | 13.77 | 13.77 | 2.94 | 16.11 | 0.35 |
| showcase | producer | 55.40 | 115.95 | 7.75 | 0.00 | 55.87 | 56.60 | 1.05 | 15.49 | 2.56 | 0.00 | 47.53 | 0.00 |
| **showcase** | showcase (mirror) | 59.94 | 111.58 | 8.87 | 0.00 | 55.79 | 59.94 | 1.21 | 17.19 | 8.87 | 0.00 | 37.39 | 0.00 |
| showcase | shipper | 138.10 | 38.81 | 3.27 | 0.00 | 38.80 | 89.56 | 0.12 | 20.56 | 7.50 | 0.00 | 23.65 | 0.00 |
| shipper | producer | 67.82 | 110.17 | 7.85 | 0.83 | 72.89 | 67.62 | 1.26 | 12.66 | 2.24 | 2.52 | 27.75 | 0.29 |
| shipper | showcase | 25.14 | 143.23 | 10.99 | 1.12 | 65.39 | 53.39 | 1.87 | 18.55 | 5.57 | 5.21 | 31.61 | 1.11 |
| **shipper** | shipper (mirror) | 55.01 | 113.30 | 7.03 | 0.56 | 56.65 | 55.01 | 0.54 | 15.69 | 7.03 | 3.23 | 37.93 | 0.19 |

## Strategic interaction: does behaviour depend on who else is at the table?

For each archetype, table size and metric, the largest paired contrast between two of the three fields (all other seats Producers, Showcases or Shippers), in SDs of the seat's per-game value. A response is a contrast of at least 0.2 SD whose interval excludes zero. *Decisions* are what the seat chooses to do (produce, buy, reprice, deliver, borrow, buy out, build); *outcomes* are what the market and the scoring give back (containers sold, prices, cash, score). The heuristics model no opponent: each follows fixed rules on what it sees, so a response is a state-dependent reaction to the market the others create, not opponent modelling. With this many comparisons some flagged contrasts are chance; the effect threshold is what keeps the list short.

### 3 seats, decisions: 25 of 27 (archetype, metric) pairs respond

| focal | metric | vs producers | vs showcases | vs shippers | largest contrast | difference [95%] | SD |
|---|---|---|---|---|---|---|---|
| showcase | produced | 2.28 | 10.65 | 9.05 | producer vs shipper | -6.76 [-6.87, -6.66] | -5.18 |
| showcase | reprices | 6.92 | 4.94 | 29.40 | showcase vs shipper | -24.46 [-24.89, -24.01] | -4.42 |
| shipper | produced | 2.70 | 13.97 | 13.47 | producer vs shipper | -10.76 [-10.92, -10.62] | -4.41 |
| shipper | cargo_size | 1.03 | 1.04 | 1.93 | producer vs shipper | -0.90 [-0.91, -0.88] | -4.31 |
| producer | bought | 25.93 | 10.69 | 13.98 | producer vs showcase | +15.24 [+14.84, +15.61] | +3.38 |
| shipper | bought | 30.83 | 16.54 | 27.55 | producer vs showcase | +14.28 [+13.91, +14.63] | +2.91 |
| producer | produced | 13.11 | 19.67 | 19.86 | producer vs shipper | -6.74 [-6.87, -6.61] | -2.84 |
| showcase | bought | 27.12 | 22.20 | 33.71 | showcase vs shipper | -11.52 [-11.86, -11.16] | -2.69 |
| shipper | deliveries | 8.30 | 12.68 | 6.82 | showcase vs shipper | +5.86 [+5.68, +6.03] | +2.34 |
| producer | cargo_size | 1.09 | 1.12 | 1.48 | producer vs shipper | -0.39 [-0.40, -0.38] | -2.23 |

(15 smaller responses are in `economics.json`.)

### 3 seats, outcomes: 15 of 15 (archetype, metric) pairs respond

| focal | metric | vs producers | vs showcases | vs shippers | largest contrast | difference [95%] | SD |
|---|---|---|---|---|---|---|---|
| shipper | avg_purchase_price | 1.55 | 2.61 | 1.93 | producer vs showcase | -1.06 [-1.08, -1.05] | -5.82 |
| producer | avg_sale_price | 1.88 | 1.10 | 1.24 | producer vs showcase | +0.78 [+0.77, +0.79] | +5.66 |
| showcase | sold | 17.67 | 22.20 | 36.93 | producer vs shipper | -19.26 [-19.59, -18.88] | -4.41 |
| showcase | avg_purchase_price | 1.80 | 2.35 | 1.92 | showcase vs shipper | +0.43 [+0.42, +0.44] | +4.32 |
| producer | avg_purchase_price | 1.88 | 2.54 | 1.87 | showcase vs shipper | +0.67 [+0.66, +0.69] | +3.16 |
| shipper | sold | 24.79 | 17.31 | 27.55 | showcase vs shipper | -10.24 [-10.60, -9.91] | -1.86 |

(9 smaller responses are in `economics.json`.)

### 4 seats, decisions: 25 of 27 (archetype, metric) pairs respond

| focal | metric | vs producers | vs showcases | vs shippers | largest contrast | difference [95%] | SD |
|---|---|---|---|---|---|---|---|
| producer | produced | 12.33 | 25.47 | 24.04 | producer vs showcase | -13.13 [-13.26, -13.01] | -5.21 |
| shipper | deliveries | 9.77 | 14.65 | 5.91 | showcase vs shipper | +8.74 [+8.62, +8.86] | +4.73 |
| showcase | deliveries | 10.27 | 11.52 | 3.05 | showcase vs shipper | +8.47 [+8.32, +8.61] | +4.65 |
| shipper | cargo_size | 1.11 | 1.08 | 2.00 | showcase vs shipper | -0.92 [-0.93, -0.91] | -4.54 |
| shipper | bought | 39.31 | 19.05 | 25.22 | producer vs showcase | +20.27 [+19.96, +20.57] | +4.10 |
| shipper | produced | 3.05 | 16.24 | 12.27 | producer vs shipper | -9.22 [-9.36, -9.07] | -3.77 |
| producer | deliveries | 11.70 | 10.34 | 5.27 | producer vs shipper | +6.43 [+6.28, +6.59] | +3.72 |
| showcase | reprices | 6.69 | 4.36 | 19.30 | showcase vs shipper | -14.95 [-15.21, -14.69] | -3.70 |
| showcase | produced | 2.66 | 12.34 | 9.05 | producer vs shipper | -6.38 [-6.49, -6.27] | -3.57 |
| producer | bought | 25.46 | 12.70 | 13.63 | producer vs showcase | +12.76 [+12.47, +13.04] | +3.47 |

(15 smaller responses are in `economics.json`.)

### 4 seats, outcomes: 15 of 15 (archetype, metric) pairs respond

| focal | metric | vs producers | vs showcases | vs shippers | largest contrast | difference [95%] | SD |
|---|---|---|---|---|---|---|---|
| shipper | avg_purchase_price | 1.56 | 2.73 | 1.97 | producer vs showcase | -1.17 [-1.18, -1.16] | -8.09 |
| producer | avg_sale_price | 1.94 | 1.10 | 1.32 | producer vs showcase | +0.84 [+0.83, +0.84] | +5.72 |
| showcase | sold | 19.95 | 25.90 | 43.10 | producer vs shipper | -23.14 [-23.47, -22.81] | -4.76 |
| showcase | avg_purchase_price | 1.83 | 2.41 | 1.96 | showcase vs shipper | +0.45 [+0.45, +0.46] | +4.55 |
| producer | avg_purchase_price | 1.94 | 2.64 | 1.91 | showcase vs shipper | +0.73 [+0.72, +0.74] | +4.28 |
| showcase | avg_sale_price | 2.63 | 2.41 | 2.69 | showcase vs shipper | -0.28 [-0.29, -0.27] | -2.00 |

(9 smaller responses are in `economics.json`.)

### 5 seats, decisions: 25 of 27 (archetype, metric) pairs respond

| focal | metric | vs producers | vs showcases | vs shippers | largest contrast | difference [95%] | SD |
|---|---|---|---|---|---|---|---|
| producer | deliveries | 12.54 | 12.34 | 3.38 | producer vs shipper | +9.16 [+9.05, +9.26] | +5.74 |
| shipper | bought | 43.33 | 19.55 | 28.43 | producer vs showcase | +23.78 [+23.52, +24.04] | +5.61 |
| showcase | deliveries | 10.40 | 11.44 | 2.96 | producer vs shipper | +7.43 [+7.35, +7.52] | +5.16 |
| shipper | cargo_size | 1.27 | 1.14 | 2.00 | showcase vs shipper | -0.86 [-0.87, -0.85] | -4.36 |
| shipper | deliveries | 9.68 | 14.73 | 6.72 | showcase vs shipper | +8.02 [+7.92, +8.13] | +4.21 |
| showcase | bought | 31.35 | 25.21 | 46.40 | showcase vs shipper | -21.19 [-21.52, -20.86] | -4.11 |
| producer | produced | 13.42 | 26.11 | 26.47 | producer vs shipper | -13.05 [-13.19, -12.90] | -3.90 |
| producer | bought | 27.92 | 14.63 | 14.03 | producer vs showcase | +13.29 [+13.03, +13.53] | +3.54 |
| shipper | produced | 3.51 | 10.48 | 13.70 | producer vs shipper | -10.19 [-10.34, -10.05] | -3.35 |
| showcase | warehouses_built | 3.06 | 3.00 | 3.78 | showcase vs shipper | -0.78 [-0.80, -0.76] | -2.58 |

(15 smaller responses are in `economics.json`.)

### 5 seats, outcomes: 15 of 15 (archetype, metric) pairs respond

| focal | metric | vs producers | vs showcases | vs shippers | largest contrast | difference [95%] | SD |
|---|---|---|---|---|---|---|---|
| shipper | avg_purchase_price | 1.56 | 2.73 | 1.94 | producer vs showcase | -1.17 [-1.18, -1.16] | -8.34 |
| producer | avg_sale_price | 1.93 | 1.10 | 1.43 | producer vs showcase | +0.83 [+0.82, +0.83] | +5.94 |
| showcase | sold | 21.36 | 25.21 | 50.21 | producer vs shipper | -28.85 [-29.22, -28.49] | -5.02 |
| producer | avg_purchase_price | 1.93 | 2.65 | 1.85 | showcase vs shipper | +0.80 [+0.79, +0.81] | +4.90 |
| showcase | avg_purchase_price | 1.81 | 2.38 | 1.93 | showcase vs shipper | +0.45 [+0.44, +0.45] | +4.56 |
| shipper | sold | 34.62 | 13.35 | 28.43 | producer vs showcase | +21.27 [+20.89, +21.59] | +3.71 |

(9 smaller responses are in `economics.json`.)

## Game quality

Stalled games reach the action horizon with nobody scoring; a default is a seat that misses an interest payment. The per-composition counts are in the payoff tables above.

| seats | games | stalled | games with a default | most default-prone composition | mean actions |
|---|---|---|---|---|---|
| 3 | 12,000 | 196 | 17.0% | Sc Sh Sh (35.8%) | 506 |
| 4 | 24,000 | 0 | 7.1% | Sc Sc Sh Sh (15.1%) | 703 |
| 5 | 42,000 | 0 | 4.5% | P Sc Sc Sc Sh (9.4%) | 905 |

Ledger check: Bank moves of any kind in these games: 0 (sail_bank 0, call_container_lot 0, call_cash_lot 0, outbid_container_lot 0, outbid_cash_lot 0, bid_cash 0, bid_container 0, done_bidding 0). The ledger's unexplained residual is negative in 0 of 341,412 seat-games and positive in 5659; it marks a default in 5601 games, and 5593 games show a Bank seizure or deal (5593 in both). The two detectors agree when these are equal. Every container bought was sold by another seat at the same price in all but 0 games.

## The empirical game of the three heuristics

Each table size is solved on its own, with the solvers of `training/metagame.py` on the 3-strategy symmetric game (the payoff of a seat against opponents drawn iid from the mixture). The chosen output is verified by the independent checker, and the bootstrap column re-measures the gain of that fixed mixture on resampled seeds: a gain below it cannot be told from sampling error. What a row shows is an empirically estimated approximate equilibrium relative to the tested policy population (the three heuristics), these table sizes and a best-response search budget of nothing beyond the three strategies. Several solutions can exist, and the others are listed below.

| n | solver | solution concept | support (weight) | restricted-game gain | independent check | bootstrap gain, mean / 95th pct | NashConv |
|---|---|---|---|---|---|---|---|
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | producer 0.04, showcase 0.27, shipper 0.69 | 2.4e-13 | 2.4e-13 (enumerate) | 0.006 / 0.014 | 7.3e-13 |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | producer 0.36, showcase 0.37, shipper 0.27 | 5.6e-17 | 8.3e-17 (enumerate) | 0.003 / 0.005 | 2.2e-16 |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | producer 0.62, showcase 0.28, shipper 0.09 | 5.2e-13 | 5.2e-13 (enumerate) | 0.002 / 0.005 | 2.6e-12 |

Utility of every strategy against sigma_n (n-1 opponents drawn iid from sigma_n; 1/n is an equal share):

| n | producer | showcase | shipper |
|---|---|---|---|
| 3 | 0.321 | 0.321 | 0.321 |
| 4 | 0.250 | 0.250 | 0.250 |
| 5 | 0.200 | 0.200 | 0.200 |

Every solver output (not only the chosen one):

| n | solver | concept | gain | NashConv | support |
|---|---|---|---|---|---|
| 3 | replicator | Nash candidate (verified by gain) | 2.2e-09 | 6.7e-09 | showcase |
| 3 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 6.3e-06 | 1.9e-05 | showcase |
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | 2.9e-14 | 8.7e-14 | showcase, shipper |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | producer |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | showcase |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 1.3e-13 | 3.9e-13 | showcase, shipper |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 4.0e-10 | 1.2e-09 | producer, showcase |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 2.4e-13 | 7.3e-13 | producer, showcase, shipper |
| 4 | replicator | Nash candidate (verified by gain) | 2.1e-09 | 8.4e-09 | shipper |
| 4 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 5.3e-06 | 2.1e-05 | shipper |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | 5.6e-17 | 2.2e-16 | producer, showcase, shipper |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | producer |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | showcase |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | shipper |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 5.0e-14 | 2.0e-13 | showcase, shipper |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 3.9e-13 | 1.6e-12 | producer, showcase |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 2.0e-12 | 7.9e-12 | producer, shipper |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 1.6e-10 | 6.2e-10 | producer, showcase, shipper |
| 5 | replicator | Nash candidate (verified by gain) | 1.7e-09 | 8.5e-09 | shipper |
| 5 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 5.9e-06 | 3.0e-05 | shipper |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | 3.5e-15 | 1.7e-14 | producer, showcase |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | producer |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | shipper |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 1.1e-12 | 5.3e-12 | producer, showcase |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 4.9e-11 | 2.4e-10 | producer, showcase |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 5.2e-13 | 2.6e-12 | producer, showcase, shipper |

## Limits of what this shows

* The three heuristics are the whole population and each is one hand-set profile. A solution of this game is an equilibrium of the *empirical game*, relative to this tested population, these table sizes and no best-response search beyond the three strategies; it says nothing about deviations outside them and is not an equilibrium of ContainerAI.
* Their games are Bank-free by construction; the Bank-open and Bank-closed comparison is a later stage.
* The seed block is `DEV`: these are development numbers, not a claim. Any decision made from them is confirmed on seeds that were not used to make it.
* The heuristics lose to the nets (Peddler v2, Magnate, Consortium) in earlier evaluation; this report does not test that and does not try to close it.

# EGTA Lite, Stage D: cross-play of Producer, Showcase and Shipper

Population: producer, showcase, shipper (scripted, the hand-set profiles of `crates/container-bots/src/archetype.rs`; no Bank play, no nets). Peddler v2, Magnate and Consortium are not in it. Design: every composition of the three archetypes (10 at 3 seats, 15 at 4 seats, 21 at 5 seats), 400 seeds each (seed block `DEV`, offset 0), every composition seated in all its rotations on every seed, `Standard` length, horizon 2500 actions, commit `6e4ad64eee30e5f3a0b7c63f89a5e130785263d2+dirty-5d055891e353`. No training, no reward, no change to a heuristic.

**78,000 games: 77,989 completed, 11 stalled (0.014%).** Utility: winner share, a stalled game pays 0 to every seat. Every table size is its own game and is never pooled with another. Intervals are 95% percentile bootstraps over seeds (1,000 resamples), written `±` for half the interval; differences are paired on the seeds the two cells share. Tables are for one seat of the archetype named.

## 3 seats: the empirical payoff table

Winner share of one seat of each archetype in every composition (fair share 0.333). `games` are the n rotations of 400 seeds; `defaults` are completed games in which some seat missed an interest payment.

| composition | games | stalled | defaults | actions | producer | showcase | shipper |
|---|---|---|---|---|---|---|---|
| P P P | 1,200 | 0 | 6.2% | 483 | 0.333 ±0.000 | — | — |
| P P Sc | 1,200 | 0 | 12.1% | 434 | 0.254 ±0.013 | 0.491 ±0.025 | — |
| P P Sh | 1,200 | 1 | 14.0% | 458 | 0.426 ±0.010 | — | 0.148 ±0.020 |
| P Sc Sc | 1,200 | 0 | 7.6% | 419 | 0.124 ±0.019 | 0.438 ±0.009 | — |
| P Sc Sh | 1,200 | 1 | 6.5% | 460 | 0.295 ±0.027 | 0.377 ±0.030 | 0.327 ±0.026 |
| P Sh Sh | 1,200 | 5 | 13.1% | 542 | 0.301 ±0.025 | — | 0.347 ±0.013 |
| Sc Sc Sc | 1,200 | 0 | 0.0% | 402 | — | 0.333 ±0.000 | — |
| Sc Sc Sh | 1,200 | 2 | 1.6% | 502 | — | 0.305 ±0.012 | 0.389 ±0.025 |
| Sc Sh Sh | 1,200 | 2 | 2.6% | 571 | — | 0.095 ±0.016 | 0.452 ±0.008 |
| Sh Sh Sh | 1,200 | 0 | 4.0% | 692 | — | — | 0.333 ±0.000 |

Final ranking in the same tables: rank share (the share of the other seats a seat out-scores, ties half, 0.5 is par; a stalled game counts 0) ± half-interval, then the mean score margin (own total minus the mean of the others'; completed games).

| composition | producer | showcase | shipper |
|---|---|---|---|
| P P P | 0.500 ±0.000 · 0.0 | — | — |
| P P Sc | 0.420 ±0.010 · -7.0 | 0.660 ±0.019 · 13.9 | — |
| P P Sh | 0.597 ±0.010 · 8.7 | — | 0.304 ±0.019 · -17.4 |
| P Sc Sc | 0.239 ±0.020 · -19.0 | 0.630 ±0.010 · 9.5 | — |
| P Sc Sh | 0.466 ±0.023 · -4.1 | 0.533 ±0.026 · 5.2 | 0.500 ±0.022 · -1.1 |
| P Sh Sh | 0.489 ±0.020 · 0.2 | — | 0.503 ±0.010 · -0.1 |
| Sc Sc Sc | — | 0.500 ±0.000 · 0.0 | — |
| Sc Sc Sh | — | 0.477 ±0.011 · -0.5 | 0.544 ±0.022 · 1.1 |
| Sc Sh Sh | — | 0.216 ±0.019 · -19.0 | 0.641 ±0.009 · 9.5 |
| Sh Sh Sh | — | — | 0.500 ±0.000 · 0.0 |

## 4 seats: the empirical payoff table

Winner share of one seat of each archetype in every composition (fair share 0.250). `games` are the n rotations of 400 seeds; `defaults` are completed games in which some seat missed an interest payment.

| composition | games | stalled | defaults | actions | producer | showcase | shipper |
|---|---|---|---|---|---|---|---|
| P P P P | 1,600 | 0 | 0.8% | 775 | 0.250 ±0.000 | — | — |
| P P P Sc | 1,600 | 0 | 2.0% | 726 | 0.221 ±0.007 | 0.338 ±0.022 | — |
| P P P Sh | 1,600 | 0 | 6.1% | 693 | 0.294 ±0.005 | — | 0.117 ±0.015 |
| P P Sc Sc | 1,600 | 0 | 4.9% | 689 | 0.194 ±0.013 | 0.306 ±0.013 | — |
| P P Sc Sh | 1,600 | 0 | 5.5% | 690 | 0.283 ±0.013 | 0.273 ±0.023 | 0.161 ±0.018 |
| P P Sh Sh | 1,600 | 0 | 7.6% | 662 | 0.291 ±0.014 | — | 0.209 ±0.014 |
| P Sc Sc Sc | 1,600 | 0 | 2.4% | 670 | 0.163 ±0.019 | 0.279 ±0.006 | — |
| P Sc Sc Sh | 1,600 | 0 | 3.0% | 711 | 0.185 ±0.018 | 0.273 ±0.011 | 0.269 ±0.021 |
| P Sc Sh Sh | 1,600 | 0 | 2.6% | 688 | 0.265 ±0.022 | 0.139 ±0.017 | 0.298 ±0.012 |
| P Sh Sh Sh | 1,600 | 0 | 4.4% | 697 | 0.249 ±0.021 | — | 0.250 ±0.007 |
| Sc Sc Sc Sc | 1,600 | 0 | 0.0% | 715 | — | 0.250 ±0.000 | — |
| Sc Sc Sc Sh | 1,600 | 0 | 1.1% | 795 | — | 0.211 ±0.008 | 0.366 ±0.023 |
| Sc Sc Sh Sh | 1,600 | 0 | 1.3% | 776 | — | 0.153 ±0.013 | 0.347 ±0.013 |
| Sc Sh Sh Sh | 1,600 | 0 | 1.0% | 762 | — | 0.036 ±0.010 | 0.321 ±0.003 |
| Sh Sh Sh Sh | 1,600 | 0 | 0.8% | 817 | — | — | 0.250 ±0.000 |

Final ranking in the same tables: rank share (the share of the other seats a seat out-scores, ties half, 0.5 is par; a stalled game counts 0) ± half-interval, then the mean score margin (own total minus the mean of the others'; completed games).

| composition | producer | showcase | shipper |
|---|---|---|---|
| P P P P | 0.500 ±0.000 · -0.0 | — | — |
| P P P Sc | 0.462 ±0.006 · -3.1 | 0.614 ±0.017 · 9.4 | — |
| P P P Sh | 0.564 ±0.006 · 6.7 | — | 0.308 ±0.017 · -20.2 |
| P P Sc Sc | 0.421 ±0.012 · -7.6 | 0.579 ±0.012 · 7.6 | — |
| P P Sc Sh | 0.530 ±0.010 · 2.6 | 0.555 ±0.017 · 6.0 | 0.386 ±0.017 · -11.3 |
| P P Sh Sh | 0.555 ±0.012 · 5.6 | — | 0.445 ±0.012 · -5.6 |
| P Sc Sc Sc | 0.377 ±0.018 · -10.2 | 0.541 ±0.006 · 3.4 | — |
| P Sc Sc Sh | 0.411 ±0.018 · -7.3 | 0.537 ±0.009 · 3.7 | 0.515 ±0.017 · -0.1 |
| P Sc Sh Sh | 0.523 ±0.016 · 1.9 | 0.398 ±0.017 · -8.1 | 0.539 ±0.010 · 3.1 |
| P Sh Sh Sh | 0.506 ±0.019 · 2.4 | — | 0.498 ±0.006 · -0.8 |
| Sc Sc Sc Sc | — | 0.500 ±0.000 · -0.0 | — |
| Sc Sc Sc Sh | — | 0.466 ±0.006 · -2.8 | 0.602 ±0.018 · 8.5 |
| Sc Sc Sh Sh | — | 0.395 ±0.011 · -9.9 | 0.605 ±0.011 · 9.9 |
| Sc Sh Sh Sh | — | 0.230 ±0.013 · -27.1 | 0.590 ±0.004 · 9.0 |
| Sh Sh Sh Sh | — | — | 0.500 ±0.000 · 0.0 |

## 5 seats: the empirical payoff table

Winner share of one seat of each archetype in every composition (fair share 0.200). `games` are the n rotations of 400 seeds; `defaults` are completed games in which some seat missed an interest payment.

| composition | games | stalled | defaults | actions | producer | showcase | shipper |
|---|---|---|---|---|---|---|---|
| P P P P P | 2,000 | 0 | 1.2% | 983 | 0.200 ±0.000 | — | — |
| P P P P Sc | 2,000 | 0 | 0.7% | 962 | 0.181 ±0.005 | 0.276 ±0.019 | — |
| P P P P Sh | 2,000 | 0 | 1.9% | 919 | 0.212 ±0.004 | — | 0.154 ±0.015 |
| P P P Sc Sc | 2,000 | 0 | 1.6% | 967 | 0.190 ±0.007 | 0.215 ±0.010 | — |
| P P P Sc Sh | 2,000 | 0 | 2.6% | 903 | 0.202 ±0.007 | 0.231 ±0.019 | 0.162 ±0.016 |
| P P P Sh Sh | 2,000 | 0 | 3.6% | 881 | 0.212 ±0.007 | — | 0.181 ±0.010 |
| P P Sc Sc Sc | 2,000 | 0 | 1.9% | 1005 | 0.188 ±0.010 | 0.208 ±0.007 | — |
| P P Sc Sc Sh | 2,000 | 0 | 2.5% | 916 | 0.201 ±0.011 | 0.200 ±0.010 | 0.200 ±0.017 |
| P P Sc Sh Sh | 2,000 | 0 | 2.1% | 874 | 0.215 ±0.012 | 0.137 ±0.014 | 0.217 ±0.011 |
| P P Sh Sh Sh | 2,000 | 0 | 4.7% | 859 | 0.218 ±0.011 | — | 0.188 ±0.008 |
| P Sc Sc Sc Sc | 2,000 | 0 | 0.7% | 1014 | 0.228 ±0.019 | 0.193 ±0.005 | — |
| P Sc Sc Sc Sh | 2,000 | 0 | 1.8% | 902 | 0.158 ±0.016 | 0.182 ±0.008 | 0.297 ±0.021 |
| P Sc Sc Sh Sh | 2,000 | 0 | 1.8% | 874 | 0.147 ±0.017 | 0.164 ±0.010 | 0.262 ±0.011 |
| P Sc Sh Sh Sh | 2,000 | 0 | 1.4% | 854 | 0.165 ±0.017 | 0.063 ±0.010 | 0.257 ±0.006 |
| P Sh Sh Sh Sh | 2,000 | 0 | 2.1% | 882 | 0.186 ±0.016 | — | 0.203 ±0.004 |
| Sc Sc Sc Sc Sc | 2,000 | 0 | 0.0% | 1118 | — | 0.200 ±0.000 | — |
| Sc Sc Sc Sc Sh | 2,000 | 0 | 0.3% | 990 | — | 0.153 ±0.005 | 0.388 ±0.022 |
| Sc Sc Sc Sh Sh | 2,000 | 0 | 0.7% | 922 | — | 0.122 ±0.007 | 0.317 ±0.011 |
| Sc Sc Sh Sh Sh | 2,000 | 0 | 0.6% | 872 | — | 0.089 ±0.008 | 0.274 ±0.006 |
| Sc Sh Sh Sh Sh | 2,000 | 0 | 1.1% | 832 | — | 0.014 ±0.005 | 0.246 ±0.001 |
| Sh Sh Sh Sh Sh | 2,000 | 0 | 0.8% | 976 | — | — | 0.200 ±0.000 |

Final ranking in the same tables: rank share (the share of the other seats a seat out-scores, ties half, 0.5 is par; a stalled game counts 0) ± half-interval, then the mean score margin (own total minus the mean of the others'; completed games).

| composition | producer | showcase | shipper |
|---|---|---|---|
| P P P P P | 0.500 ±0.000 · 0.0 | — | — |
| P P P P Sc | 0.471 ±0.003 · -2.5 | 0.617 ±0.014 · 10.2 | — |
| P P P P Sh | 0.519 ±0.004 · 2.2 | — | 0.424 ±0.016 · -8.9 |
| P P P Sc Sc | 0.467 ±0.006 · -2.9 | 0.550 ±0.009 · 4.4 | — |
| P P P Sc Sh | 0.494 ±0.007 · 0.2 | 0.576 ±0.014 · 6.2 | 0.442 ±0.016 · -6.8 |
| P P P Sh Sh | 0.522 ±0.006 · 2.9 | — | 0.467 ±0.008 · -4.3 |
| P P Sc Sc Sc | 0.464 ±0.010 · -3.4 | 0.524 ±0.007 · 2.3 | — |
| P P Sc Sc Sh | 0.493 ±0.010 · -0.2 | 0.519 ±0.009 · 2.0 | 0.476 ±0.016 · -3.6 |
| P P Sc Sh Sh | 0.503 ±0.011 · 1.4 | 0.456 ±0.014 · -4.4 | 0.519 ±0.010 · 0.8 |
| P P Sh Sh Sh | 0.522 ±0.010 · 3.4 | — | 0.485 ±0.007 · -2.3 |
| P Sc Sc Sc Sc | 0.516 ±0.016 · 1.2 | 0.496 ±0.004 · -0.3 | — |
| P Sc Sc Sc Sh | 0.435 ±0.016 · -5.7 | 0.496 ±0.007 · -0.2 | 0.576 ±0.017 · 6.3 |
| P Sc Sc Sh Sh | 0.425 ±0.016 · -5.5 | 0.481 ±0.010 · -2.1 | 0.556 ±0.009 · 4.8 |
| P Sc Sh Sh Sh | 0.464 ±0.018 · -1.6 | 0.322 ±0.012 · -16.4 | 0.572 ±0.006 · 6.0 |
| P Sh Sh Sh Sh | 0.439 ±0.016 · -3.9 | — | 0.515 ±0.004 · 1.0 |
| Sc Sc Sc Sc Sc | — | 0.500 ±0.000 · 0.0 | — |
| Sc Sc Sc Sc Sh | — | 0.456 ±0.003 · -4.1 | 0.676 ±0.014 · 16.4 |
| Sc Sc Sc Sh Sh | — | 0.420 ±0.006 · -7.8 | 0.620 ±0.009 · 11.6 |
| Sc Sc Sh Sh Sh | — | 0.352 ±0.008 · -14.0 | 0.599 ±0.005 · 9.3 |
| Sc Sh Sh Sh Sh | — | 0.168 ±0.009 · -33.4 | 0.583 ±0.002 · 8.4 |
| Sh Sh Sh Sh Sh | — | — | 0.500 ±0.000 · 0.0 |

## Identity: does the Producer produce more than the others?

In every table that seats the Producer next to another archetype, the containers a Producer seat produces per game and the paired difference to the other archetype's seats of the same games (completed games; a positive difference means the Producer produces more).

| seats | composition | producer produced | showcase produced | shipper produced | producer − showcase | producer − shipper |
|---|---|---|---|---|---|---|
| 3 | P P Sc | 11.96 | 4.00 | — | 7.96 ±0.11 | — |
| 3 | P P Sh | 13.34 | — | 4.17 | — | 9.17 ±0.17 |
| 3 | P Sc Sc | 13.09 | 6.74 | — | 6.36 ±0.13 | — |
| 3 | P Sc Sh | 16.97 | 5.54 | 6.87 | 11.43 ±0.25 | 10.10 ±0.38 |
| 3 | P Sh Sh | 16.24 | — | 8.85 | — | 7.40 ±0.18 |
| 4 | P P P Sc | 13.12 | 5.75 | — | 7.37 ±0.11 | — |
| 4 | P P P Sh | 13.03 | — | 4.95 | — | 8.08 ±0.17 |
| 4 | P P Sc Sc | 14.06 | 7.06 | — | 7.00 ±0.14 | — |
| 4 | P P Sc Sh | 14.62 | 7.13 | 6.83 | 7.49 ±0.23 | 7.79 ±0.33 |
| 4 | P P Sh Sh | 15.11 | — | 5.60 | — | 9.51 ±0.25 |
| 4 | P Sc Sc Sc | 16.63 | 8.01 | — | 8.62 ±0.22 | — |
| 4 | P Sc Sc Sh | 17.46 | 8.19 | 11.15 | 9.27 ±0.40 | 6.31 ±0.60 |
| 4 | P Sc Sh Sh | 18.57 | 8.54 | 7.99 | 10.03 ±0.47 | 10.57 ±0.49 |
| 4 | P Sh Sh Sh | 18.10 | — | 8.46 | — | 9.63 ±0.37 |
| 5 | P P P P Sc | 12.44 | 6.15 | — | 6.29 ±0.11 | — |
| 5 | P P P P Sh | 12.37 | — | 4.92 | — | 7.45 ±0.14 |
| 5 | P P P Sc Sc | 13.41 | 7.81 | — | 5.60 ±0.15 | — |
| 5 | P P P Sc Sh | 13.56 | 6.63 | 6.15 | 6.93 ±0.18 | 7.41 ±0.31 |
| 5 | P P P Sh Sh | 14.16 | — | 5.08 | — | 9.09 ±0.14 |
| 5 | P P Sc Sc Sc | 14.93 | 9.31 | — | 5.62 ±0.15 | — |
| 5 | P P Sc Sc Sh | 14.80 | 8.51 | 7.30 | 6.29 ±0.24 | 7.50 ±0.41 |
| 5 | P P Sc Sh Sh | 15.30 | 7.77 | 7.42 | 7.53 ±0.34 | 7.88 ±0.44 |
| 5 | P P Sh Sh Sh | 17.40 | — | 5.71 | — | 11.69 ±0.14 |
| 5 | P Sc Sc Sc Sc | 18.00 | 10.06 | — | 7.94 ±0.26 | — |
| 5 | P Sc Sc Sc Sh | 19.82 | 8.25 | 8.87 | 11.58 ±0.38 | 10.95 ±0.52 |
| 5 | P Sc Sc Sh Sh | 18.34 | 8.59 | 8.81 | 9.75 ±0.46 | 9.52 ±0.54 |
| 5 | P Sc Sh Sh Sh | 18.78 | 9.03 | 8.45 | 9.75 ±0.55 | 10.33 ±0.66 |
| 5 | P Sh Sh Sh Sh | 21.64 | — | 7.94 | — | 13.70 ±0.10 |

Tables in which the lower bound of the difference is above zero: 3 seats vs shipper: 3 of 3; 3 seats vs showcase: 3 of 3; 4 seats vs shipper: 6 of 6; 4 seats vs showcase: 6 of 6; 5 seats vs shipper: 10 of 10; 5 seats vs showcase: 10 of 10.

### Each archetype's own metric in the mixed tables

Seats of each archetype pooled over the tables of one size that seat at least two archetypes (mirrors left out; compositions weighted by their seats).

3 seats:

| metric | producer | showcase | shipper |
|---|---|---|---|
| produced | 13.84 ±0.03 | 7.04 ±0.03 | 10.46 ±0.08 |
| sold | 20.72 ±0.07 | 23.90 ±0.08 | 19.67 ±0.13 |
| sold/produced | 1.50 ±0.00 | 3.40 ±0.01 | 1.88 ±0.01 |
| avg sale $ | 1.59 ±0.00 | 2.53 ±0.00 | 1.91 ±0.00 |
| unsold at the end | 1.30 ±0.02 | 0.90 ±0.02 | 1.64 ±0.02 |
| cheap shelf stock (own lots at $3 or less, per decision) | 1.24 ±0.01 | 1.68 ±0.01 | 1.13 ±0.01 |
| colors sold | 3.52 ±0.01 | 3.85 ±0.01 | 3.96 ±0.01 |
| reprices | 1.4 ±0.1 | 8.1 ±0.1 | 3.9 ±0.1 |
| deliveries | 9.89 ±0.06 | 7.94 ±0.04 | 12.18 ±0.06 |
| containers per delivery | 1.01 ±0.00 | 1.03 ±0.00 | 1.02 ±0.00 |
| delivery cash minus purchases | 15.7 ±0.5 | -7.9 ±0.4 | 17.4 ±0.4 |
| win share | 0.297 ±0.008 | 0.350 ±0.009 | 0.352 ±0.007 |
| score | 65.3 ±0.8 | 72.8 ±0.5 | 79.9 ±0.7 |

4 seats:

| metric | producer | showcase | shipper |
|---|---|---|---|
| produced | 14.80 ±0.05 | 8.75 ±0.04 | 10.20 ±0.07 |
| sold | 23.08 ±0.06 | 28.54 ±0.07 | 18.98 ±0.09 |
| sold/produced | 1.56 ±0.00 | 3.26 ±0.01 | 1.86 ±0.01 |
| avg sale $ | 1.65 ±0.00 | 2.59 ±0.00 | 1.94 ±0.00 |
| unsold at the end | 1.05 ±0.01 | 0.79 ±0.01 | 1.12 ±0.01 |
| cheap shelf stock (own lots at $3 or less, per decision) | 0.85 ±0.01 | 1.07 ±0.00 | 0.83 ±0.01 |
| colors sold | 3.90 ±0.01 | 4.44 ±0.01 | 4.04 ±0.01 |
| reprices | 2.1 ±0.0 | 5.4 ±0.0 | 3.1 ±0.0 |
| deliveries | 11.04 ±0.03 | 9.31 ±0.03 | 12.49 ±0.04 |
| containers per delivery | 1.01 ±0.00 | 1.04 ±0.00 | 1.05 ±0.00 |
| delivery cash minus purchases | 50.1 ±0.4 | 18.9 ±0.3 | 58.6 ±0.3 |
| win share | 0.246 ±0.005 | 0.233 ±0.005 | 0.271 ±0.005 |
| score | 99.8 ±0.5 | 104.4 ±0.4 | 109.2 ±0.5 |

5 seats:

| metric | producer | showcase | shipper |
|---|---|---|---|
| produced | 14.80 ±0.04 | 9.23 ±0.03 | 8.90 ±0.04 |
| sold | 23.87 ±0.05 | 29.47 ±0.06 | 15.87 ±0.06 |
| sold/produced | 1.61 ±0.00 | 3.19 ±0.01 | 1.78 ±0.00 |
| avg sale $ | 1.67 ±0.00 | 2.54 ±0.00 | 1.90 ±0.00 |
| unsold at the end | 0.91 ±0.01 | 0.61 ±0.01 | 1.15 ±0.01 |
| cheap shelf stock (own lots at $3 or less, per decision) | 0.66 ±0.00 | 0.77 ±0.00 | 0.74 ±0.00 |
| colors sold | 3.96 ±0.01 | 4.42 ±0.01 | 3.81 ±0.01 |
| reprices | 2.3 ±0.0 | 4.0 ±0.0 | 2.2 ±0.0 |
| deliveries | 10.86 ±0.03 | 9.22 ±0.02 | 11.72 ±0.02 |
| containers per delivery | 1.01 ±0.00 | 1.03 ±0.00 | 1.10 ±0.00 |
| delivery cash minus purchases | 67.3 ±0.3 | 35.4 ±0.2 | 78.1 ±0.2 |
| win share | 0.197 ±0.002 | 0.166 ±0.002 | 0.237 ±0.003 |
| score | 112.2 ±0.3 | 112.7 ±0.3 | 113.1 ±0.3 |

## Economic outcomes of one archetype against each field

A seat of the focal archetype at a table whose other seats are all the second archetype (the mirror row is the all-same table). Counts are per game, completed games only; average prices are total income over total containers; `unsold` is the exact stock at the end of seats that did not default.

### 3 seats

Result:

| focal seat | other seats | won | rank share | score margin | score | final cash |
|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 0.333 ±0.000 | 0.500 | 0.0 | 81.16 | 42.77 |
| producer | showcase | 0.124 ±0.018 | 0.239 | -19.0 | 44.87 | 24.29 |
| producer | shipper | 0.301 ±0.023 | 0.489 | 0.2 | 79.91 | 42.74 |
| showcase | producer | 0.491 ±0.024 | 0.660 | 13.9 | 72.76 | 41.50 |
| **showcase** | showcase (mirror) | 0.333 ±0.000 | 0.500 | 0.0 | 57.36 | 29.06 |
| showcase | shipper | 0.095 ±0.016 | 0.216 | -19.0 | 78.16 | 30.08 |
| shipper | producer | 0.148 ±0.020 | 0.304 | -17.4 | 57.60 | 30.16 |
| shipper | showcase | 0.389 ±0.026 | 0.544 | 1.1 | 80.88 | 38.27 |
| **shipper** | shipper (mirror) | 0.333 ±0.000 | 0.500 | 0.0 | 114.31 | 52.11 |

Market:

| focal seat | other seats | produced | sold | sold/produced | avg sale $ | bought | avg purchase $ | unsold | reprices |
|---|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 11.04 | 22.73 | 2.06 | 1.94 | 22.73 | 1.94 | 1.05 | 1.61 |
| producer | showcase | 13.09 | 16.56 | 1.26 | 1.34 | 12.36 | 2.65 | 0.97 | 0.76 |
| producer | shipper | 16.24 | 24.73 | 1.52 | 1.62 | 19.63 | 1.89 | 1.56 | 1.90 |
| showcase | producer | 4.00 | 18.55 | 4.64 | 2.56 | 22.90 | 1.74 | 1.00 | 6.03 |
| **showcase** | showcase (mirror) | 8.61 | 18.40 | 2.14 | 2.41 | 18.40 | 2.41 | 0.84 | 4.49 |
| showcase | shipper | 9.24 | 35.93 | 3.89 | 2.50 | 30.62 | 1.90 | 0.44 | 23.53 |
| shipper | producer | 4.17 | 17.62 | 4.23 | 1.89 | 23.47 | 1.79 | 1.58 | 4.41 |
| shipper | showcase | 17.00 | 18.81 | 1.11 | 1.89 | 16.24 | 2.78 | 1.65 | 3.78 |
| **shipper** | shipper (mirror) | 14.30 | 27.96 | 1.96 | 1.94 | 27.96 | 1.94 | 1.76 | 10.65 |

Delivery and finance:

| focal seat | other seats | deliveries | per delivery | loans taken | buy-outs | factories | warehouses | defaulted |
|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 11.00 | 1.00 | 0.17 | 1.73 | 0.33 | 0.00 | 2.1% |
| producer | showcase | 8.88 | 1.01 | 0.39 | 1.28 | 0.35 | 0.00 | 7.6% |
| producer | shipper | 10.62 | 1.00 | 0.18 | 1.46 | 0.67 | 0.00 | 2.6% |
| showcase | producer | 8.32 | 1.00 | 0.00 | 1.37 | 0.00 | 1.46 | 0.0% |
| **showcase** | showcase (mirror) | 8.59 | 1.02 | 0.00 | 1.53 | 0.00 | 2.02 | 0.0% |
| showcase | shipper | 4.49 | 1.00 | 0.00 | 0.70 | 0.00 | 1.90 | 0.0% |
| shipper | producer | 9.54 | 1.00 | 0.54 | 1.55 | 0.67 | 0.00 | 8.5% |
| shipper | showcase | 13.44 | 1.03 | 0.29 | 2.22 | 1.00 | 0.00 | 1.6% |
| **shipper** | shipper (mirror) | 13.56 | 1.00 | 0.11 | 2.01 | 0.67 | 0.00 | 1.3% |

### 4 seats

Result:

| focal seat | other seats | won | rank share | score margin | score | final cash |
|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 0.250 ±0.000 | 0.500 | -0.0 | 118.53 | 66.22 |
| producer | showcase | 0.163 ±0.019 | 0.377 | -10.2 | 78.33 | 45.01 |
| producer | shipper | 0.249 ±0.023 | 0.506 | 2.4 | 103.84 | 70.80 |
| showcase | producer | 0.338 ±0.022 | 0.614 | 9.4 | 112.15 | 65.55 |
| **showcase** | showcase (mirror) | 0.250 ±0.000 | 0.500 | -0.0 | 93.26 | 52.45 |
| showcase | shipper | 0.036 ±0.009 | 0.230 | -27.1 | 102.51 | 53.12 |
| shipper | producer | 0.117 ±0.016 | 0.308 | -20.2 | 87.69 | 40.78 |
| shipper | showcase | 0.366 ±0.024 | 0.602 | 8.5 | 125.38 | 60.48 |
| **shipper** | shipper (mirror) | 0.250 ±0.000 | 0.500 | 0.0 | 132.27 | 76.93 |

Market:

| focal seat | other seats | produced | sold | sold/produced | avg sale $ | bought | avg purchase $ | unsold | reprices |
|---|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 12.37 | 25.85 | 2.09 | 1.96 | 25.85 | 1.96 | 0.87 | 1.94 |
| producer | showcase | 16.63 | 19.88 | 1.20 | 1.27 | 14.25 | 2.72 | 0.89 | 2.53 |
| producer | shipper | 18.10 | 27.04 | 1.49 | 1.59 | 18.99 | 1.92 | 1.30 | 2.02 |
| showcase | producer | 5.75 | 25.00 | 4.35 | 2.63 | 29.07 | 1.72 | 0.67 | 4.65 |
| **showcase** | showcase (mirror) | 10.81 | 23.00 | 2.13 | 2.44 | 23.00 | 2.44 | 0.74 | 5.82 |
| showcase | shipper | 11.88 | 45.32 | 3.82 | 2.66 | 38.35 | 1.94 | 0.52 | 9.89 |
| shipper | producer | 4.95 | 18.41 | 3.72 | 1.94 | 24.72 | 1.92 | 0.96 | 3.31 |
| shipper | showcase | 20.25 | 23.64 | 1.17 | 1.90 | 19.73 | 2.78 | 1.42 | 4.86 |
| **shipper** | shipper (mirror) | 13.31 | 26.33 | 1.98 | 1.95 | 26.33 | 1.95 | 1.52 | 2.15 |

Delivery and finance:

| focal seat | other seats | deliveries | per delivery | loans taken | buy-outs | factories | warehouses | defaulted |
|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 12.50 | 1.00 | 0.06 | 0.77 | 0.25 | 0.00 | 0.2% |
| producer | showcase | 11.09 | 1.00 | 0.23 | 0.55 | 0.33 | 0.00 | 2.4% |
| producer | shipper | 9.75 | 1.00 | 0.10 | 0.36 | 0.50 | 0.00 | 1.1% |
| showcase | producer | 10.15 | 1.00 | 0.00 | 0.66 | 0.00 | 1.08 | 0.0% |
| **showcase** | showcase (mirror) | 11.03 | 1.00 | 0.00 | 0.68 | 0.00 | 2.00 | 0.0% |
| showcase | shipper | 5.39 | 1.00 | 0.00 | 0.15 | 0.00 | 1.86 | 0.0% |
| shipper | producer | 11.33 | 1.00 | 0.52 | 0.79 | 0.35 | 0.00 | 3.9% |
| shipper | showcase | 14.86 | 1.07 | 0.20 | 1.12 | 1.00 | 0.00 | 1.1% |
| **shipper** | shipper (mirror) | 12.79 | 1.00 | 0.10 | 0.62 | 0.29 | 0.00 | 0.2% |

### 5 seats

Result:

| focal seat | other seats | won | rank share | score margin | score | final cash |
|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 0.200 ±0.000 | 0.500 | 0.0 | 122.75 | 75.17 |
| producer | showcase | 0.228 ±0.019 | 0.516 | 1.2 | 119.13 | 70.82 |
| producer | shipper | 0.186 ±0.016 | 0.439 | -3.9 | 96.72 | 65.63 |
| showcase | producer | 0.276 ±0.019 | 0.617 | 10.2 | 126.71 | 80.63 |
| **showcase** | showcase (mirror) | 0.200 ±0.000 | 0.500 | 0.0 | 133.22 | 75.50 |
| showcase | shipper | 0.014 ±0.005 | 0.168 | -33.4 | 91.10 | 50.82 |
| shipper | producer | 0.154 ±0.015 | 0.424 | -8.9 | 107.71 | 60.11 |
| shipper | showcase | 0.388 ±0.022 | 0.676 | 16.4 | 130.74 | 75.53 |
| **shipper** | shipper (mirror) | 0.200 ±0.000 | 0.500 | 0.0 | 111.48 | 70.34 |

Market:

| focal seat | other seats | produced | sold | sold/produced | avg sale $ | bought | avg purchase $ | unsold | reprices |
|---|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 11.54 | 24.50 | 2.12 | 1.94 | 24.50 | 1.94 | 0.68 | 2.11 |
| producer | showcase | 18.00 | 21.55 | 1.20 | 1.26 | 17.35 | 2.65 | 1.02 | 3.71 |
| producer | shipper | 21.64 | 33.11 | 1.53 | 1.63 | 19.52 | 1.87 | 1.63 | 1.42 |
| showcase | producer | 6.15 | 26.94 | 4.38 | 2.59 | 30.57 | 1.67 | 0.41 | 2.71 |
| **showcase** | showcase (mirror) | 12.74 | 26.98 | 2.12 | 2.38 | 26.98 | 2.38 | 0.65 | 6.36 |
| showcase | shipper | 10.04 | 50.75 | 5.05 | 2.68 | 43.63 | 1.90 | 0.51 | 4.77 |
| shipper | producer | 4.92 | 14.72 | 2.99 | 1.90 | 22.45 | 2.12 | 0.83 | 1.78 |
| shipper | showcase | 11.01 | 14.64 | 1.33 | 1.88 | 19.65 | 2.74 | 0.77 | 2.88 |
| **shipper** | shipper (mirror) | 12.44 | 21.72 | 1.75 | 1.92 | 21.72 | 1.92 | 3.08 | 2.20 |

Delivery and finance:

| focal seat | other seats | deliveries | per delivery | loans taken | buy-outs | factories | warehouses | defaulted |
|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 11.86 | 1.00 | 0.07 | 0.31 | 0.20 | 0.00 | 0.2% |
| producer | showcase | 13.73 | 1.00 | 0.17 | 0.37 | 0.30 | 0.00 | 0.7% |
| producer | shipper | 7.42 | 1.00 | 0.10 | 0.16 | 0.60 | 0.00 | 0.8% |
| showcase | producer | 10.38 | 1.00 | 0.00 | 0.24 | 0.00 | 1.00 | 0.0% |
| **showcase** | showcase (mirror) | 13.08 | 1.00 | 0.00 | 0.38 | 0.00 | 2.00 | 0.0% |
| showcase | shipper | 3.41 | 1.00 | 0.00 | 0.04 | 0.00 | 1.89 | 0.0% |
| shipper | producer | 12.83 | 1.00 | 0.34 | 0.34 | 0.40 | 0.00 | 1.3% |
| shipper | showcase | 15.65 | 1.04 | 0.13 | 0.50 | 1.00 | 0.00 | 0.4% |
| **shipper** | shipper (mirror) | 10.36 | 1.00 | 0.06 | 0.21 | 0.80 | 0.00 | 0.1% |

## Cash ledger flows, apart from the reward

Dollars per game and seat from the counters of the engine (`training/ledger.py`). In: containers sold, delivery proceeds (twice the winning bid), wages received, loan principal not repaid. Out: cargo auctions won, containers bought, buy-outs, factories and warehouses built, wages paid, interest due. `net cash change` is final cash minus the $20 start. None of these sums is a profit: the reward is the score, which also counts the Island, leftovers and loans, and `delivery_cash_minus_purchases` ignores most of this table. `unpaid` is the interest due that a defaulting seat did not pay (what the ledger cannot explain).

### 3 seats

| focal seat | other seats | sales | delivery | wages in | loans | auctions | purchases | buy-outs | builds | wages out | interest | net cash change | unpaid |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 44.19 | 56.62 | 8.54 | 0.93 | 28.31 | 44.19 | 3.29 | 2.00 | 8.54 | 1.65 | 22.77 | 0.47 |
| producer | showcase | 22.14 | 42.12 | 7.24 | 3.18 | 20.35 | 32.75 | 2.09 | 2.12 | 10.23 | 4.06 | 4.29 | 1.21 |
| producer | shipper | 39.96 | 58.59 | 5.44 | 1.11 | 25.75 | 37.18 | 2.89 | 4.00 | 11.05 | 2.87 | 22.74 | 1.38 |
| showcase | producer | 47.51 | 40.74 | 9.44 | 0.00 | 23.60 | 39.96 | 2.37 | 6.28 | 4.00 | 0.00 | 21.50 | 0.00 |
| **showcase** | showcase (mirror) | 44.34 | 41.61 | 8.61 | 0.00 | 20.80 | 44.34 | 2.62 | 9.12 | 8.61 | 0.00 | 9.06 | 0.00 |
| showcase | shipper | 89.92 | 26.33 | 5.42 | 0.00 | 34.15 | 58.12 | 1.56 | 8.50 | 9.24 | 0.00 | 10.08 | 0.00 |
| shipper | producer | 33.25 | 47.96 | 8.71 | 3.07 | 27.48 | 42.13 | 2.85 | 4.00 | 2.78 | 4.81 | 10.16 | 1.22 |
| shipper | showcase | 35.52 | 68.75 | 9.31 | 1.02 | 31.30 | 45.18 | 4.27 | 6.00 | 8.53 | 1.44 | 18.27 | 0.40 |
| **shipper** | shipper (mirror) | 54.31 | 82.17 | 8.79 | 0.47 | 41.09 | 54.31 | 4.78 | 4.00 | 8.79 | 1.28 | 32.11 | 0.62 |

### 4 seats

| focal seat | other seats | sales | delivery | wages in | loans | auctions | purchases | buy-outs | builds | wages out | interest | net cash change | unpaid |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 50.69 | 100.57 | 9.66 | 0.11 | 50.29 | 50.69 | 2.42 | 1.50 | 9.66 | 0.31 | 46.22 | 0.06 |
| producer | showcase | 25.20 | 83.95 | 9.55 | 1.02 | 37.62 | 38.74 | 1.68 | 1.97 | 13.13 | 2.19 | 25.01 | 0.61 |
| producer | shipper | 43.12 | 90.88 | 6.42 | 0.35 | 34.93 | 36.43 | 1.25 | 3.01 | 13.59 | 1.40 | 50.80 | 0.66 |
| showcase | producer | 65.78 | 78.93 | 10.17 | 0.00 | 47.24 | 49.90 | 2.05 | 4.41 | 5.75 | 0.00 | 45.55 | 0.00 |
| **showcase** | showcase (mirror) | 56.05 | 87.07 | 10.81 | 0.00 | 43.54 | 56.05 | 2.08 | 9.00 | 10.81 | 0.00 | 32.45 | 0.00 |
| showcase | shipper | 120.40 | 48.94 | 6.50 | 0.00 | 47.63 | 74.35 | 0.56 | 8.32 | 11.88 | 0.00 | 33.12 | 0.00 |
| shipper | producer | 35.71 | 86.09 | 10.49 | 1.64 | 54.87 | 47.39 | 2.48 | 2.11 | 3.84 | 3.29 | 20.78 | 0.84 |
| shipper | showcase | 45.01 | 122.92 | 12.30 | 0.42 | 64.84 | 54.86 | 3.74 | 6.00 | 10.13 | 0.82 | 40.48 | 0.21 |
| **shipper** | shipper (mirror) | 51.47 | 122.19 | 11.15 | 0.17 | 61.10 | 51.47 | 2.19 | 1.76 | 11.15 | 0.49 | 56.93 | 0.11 |

### 5 seats

| focal seat | other seats | sales | delivery | wages in | loans | auctions | purchases | buy-outs | builds | wages out | interest | net cash change | unpaid |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **producer** | producer (mirror) | 47.65 | 115.26 | 9.56 | 0.11 | 57.63 | 47.65 | 1.17 | 1.20 | 9.56 | 0.26 | 55.17 | 0.05 |
| producer | showcase | 27.25 | 131.34 | 12.70 | 0.29 | 56.03 | 45.99 | 1.42 | 1.78 | 14.57 | 1.27 | 50.82 | 0.30 |
| producer | shipper | 54.08 | 82.14 | 5.63 | 0.29 | 40.03 | 36.47 | 0.67 | 3.60 | 15.21 | 0.77 | 45.63 | 0.24 |
| showcase | producer | 69.68 | 101.21 | 8.58 | 0.00 | 56.62 | 51.16 | 0.89 | 4.01 | 6.15 | 0.00 | 60.63 | 0.00 |
| **showcase** | showcase (mirror) | 64.26 | 131.99 | 12.74 | 0.00 | 65.99 | 64.26 | 1.49 | 9.00 | 12.74 | 0.00 | 55.50 | 0.00 |
| showcase | shipper | 136.23 | 36.33 | 3.03 | 0.00 | 43.31 | 82.78 | 0.18 | 8.46 | 10.04 | 0.00 | 30.82 | 0.00 |
| shipper | producer | 28.04 | 122.54 | 9.64 | 0.48 | 64.32 | 47.63 | 1.29 | 2.40 | 3.71 | 1.53 | 40.11 | 0.28 |
| shipper | showcase | 27.53 | 152.25 | 13.39 | 0.17 | 69.32 | 53.87 | 1.98 | 5.98 | 6.24 | 0.49 | 55.53 | 0.09 |
| **shipper** | shipper (mirror) | 41.69 | 112.29 | 8.21 | 0.06 | 56.15 | 41.69 | 0.90 | 4.80 | 8.21 | 0.17 | 50.34 | 0.01 |

## Strategic interaction: does behaviour depend on who else is at the table?

For each archetype, table size and metric, the largest paired contrast between two of the three fields (all other seats Producers, Showcases or Shippers), in SDs of the seat's per-game value. A response is a contrast of at least 0.2 SD whose interval excludes zero. *Decisions* are what the seat chooses to do (produce, buy, reprice, deliver, borrow, buy out, build); *outcomes* are what the market and the scoring give back (containers sold, prices, cash, score). The heuristics model no opponent: each follows fixed rules on what it sees, so a response is a state-dependent reaction to the market the others create, not opponent modelling. With this many comparisons some flagged contrasts are chance; the effect threshold is what keeps the list short.

### 3 seats, decisions: 23 of 27 (archetype, metric) pairs respond

| focal | metric | vs producers | vs showcases | vs shippers | largest contrast | difference [95%] | SD |
|---|---|---|---|---|---|---|---|
| shipper | bought | 23.47 | 16.24 | 27.96 | showcase vs shipper | -11.72 [-11.90, -11.53] | -4.67 |
| showcase | reprices | 6.03 | 4.49 | 23.53 | showcase vs shipper | -19.04 [-19.34, -18.78] | -4.26 |
| shipper | produced | 4.17 | 17.00 | 14.30 | producer vs showcase | -12.83 [-13.08, -12.57] | -3.93 |
| producer | bought | 22.73 | 12.36 | 19.63 | producer vs showcase | +10.37 [+10.18, +10.58] | +3.42 |
| showcase | deliveries | 8.32 | 8.59 | 4.49 | showcase vs shipper | +4.10 [+4.02, +4.18] | +3.41 |
| showcase | produced | 4.00 | 8.61 | 9.24 | producer vs shipper | -5.24 [-5.35, -5.12] | -2.92 |
| showcase | bought | 22.90 | 18.40 | 30.62 | showcase vs shipper | -12.22 [-12.53, -11.91] | -2.87 |
| shipper | reprices | 4.41 | 3.78 | 10.65 | showcase vs shipper | -6.88 [-7.08, -6.67] | -2.47 |
| shipper | deliveries | 9.54 | 13.44 | 13.56 | producer vs showcase | -3.90 [-4.08, -3.71] | -1.75 |
| showcase | warehouses_built | 1.46 | 2.02 | 1.90 | producer vs showcase | -0.57 [-0.59, -0.55] | -1.54 |

(13 smaller responses are in `economics.json`.)

### 3 seats, outcomes: 15 of 15 (archetype, metric) pairs respond

| focal | metric | vs producers | vs showcases | vs shippers | largest contrast | difference [95%] | SD |
|---|---|---|---|---|---|---|---|
| shipper | avg_purchase_price | 1.79 | 2.78 | 1.94 | showcase vs shipper | +0.84 [+0.83, +0.85] | +10.43 |
| showcase | avg_purchase_price | 1.74 | 2.41 | 1.90 | showcase vs shipper | +0.51 [+0.51, +0.52] | +7.52 |
| producer | avg_purchase_price | 1.94 | 2.65 | 1.89 | showcase vs shipper | +0.75 [+0.74, +0.77] | +5.28 |
| showcase | sold | 18.55 | 18.40 | 35.93 | showcase vs shipper | -17.53 [-17.82, -17.24] | -4.38 |
| shipper | sold | 17.62 | 18.81 | 27.96 | showcase vs shipper | -9.15 [-9.36, -8.94] | -2.85 |
| producer | sold | 22.73 | 16.56 | 24.73 | showcase vs shipper | -8.18 [-8.45, -7.91] | -2.08 |

(9 smaller responses are in `economics.json`.)

### 4 seats, decisions: 23 of 27 (archetype, metric) pairs respond

| focal | metric | vs producers | vs showcases | vs shippers | largest contrast | difference [95%] | SD |
|---|---|---|---|---|---|---|---|
| showcase | warehouses_built | 1.08 | 2.00 | 1.86 | producer vs showcase | -0.92 [-0.93, -0.90] | -4.76 |
| showcase | bought | 29.07 | 23.00 | 38.35 | showcase vs shipper | -15.35 [-15.60, -15.12] | -4.28 |
| producer | bought | 25.85 | 14.25 | 18.99 | producer vs showcase | +11.60 [+11.35, +11.86] | +3.52 |
| showcase | deliveries | 10.15 | 11.03 | 5.39 | showcase vs shipper | +5.64 [+5.53, +5.75] | +3.49 |
| shipper | produced | 4.95 | 20.25 | 13.31 | producer vs showcase | -15.31 [-15.63, -14.99] | -3.24 |
| showcase | produced | 5.75 | 10.81 | 11.88 | producer vs shipper | -6.13 [-6.26, -6.00] | -2.75 |
| shipper | bought | 24.72 | 19.73 | 26.33 | showcase vs shipper | -6.60 [-6.79, -6.39] | -2.66 |
| shipper | factories_built | 0.35 | 1.00 | 0.29 | showcase vs shipper | +0.71 [+0.70, +0.72] | +2.20 |
| shipper | deliveries | 11.33 | 14.86 | 12.79 | producer vs showcase | -3.53 [-3.66, -3.39] | -1.87 |
| showcase | reprices | 4.65 | 5.82 | 9.89 | producer vs shipper | -5.25 [-5.46, -5.04] | -1.71 |

(13 smaller responses are in `economics.json`.)

### 4 seats, outcomes: 15 of 15 (archetype, metric) pairs respond

| focal | metric | vs producers | vs showcases | vs shippers | largest contrast | difference [95%] | SD |
|---|---|---|---|---|---|---|---|
| shipper | avg_purchase_price | 1.92 | 2.78 | 1.95 | showcase vs shipper | +0.83 [+0.82, +0.83] | +12.69 |
| showcase | avg_purchase_price | 1.72 | 2.44 | 1.94 | producer vs showcase | -0.72 [-0.73, -0.71] | -6.75 |
| producer | avg_purchase_price | 1.96 | 2.72 | 1.92 | showcase vs shipper | +0.80 [+0.79, +0.81] | +6.60 |
| showcase | sold | 25.00 | 23.00 | 45.32 | showcase vs shipper | -22.32 [-22.57, -22.09] | -5.34 |
| showcase | avg_sale_price | 2.63 | 2.44 | 2.66 | showcase vs shipper | -0.22 [-0.22, -0.22] | -2.93 |
| producer | avg_sale_price | 1.96 | 1.27 | 1.59 | producer vs showcase | +0.69 [+0.68, +0.70] | +2.59 |

(9 smaller responses are in `economics.json`.)

### 5 seats, decisions: 22 of 27 (archetype, metric) pairs respond

| focal | metric | vs producers | vs showcases | vs shippers | largest contrast | difference [95%] | SD |
|---|---|---|---|---|---|---|---|
| showcase | warehouses_built | 1.00 | 2.00 | 1.89 | producer vs showcase | -1.00 [-1.00, -1.00] | -28.25 |
| showcase | deliveries | 10.38 | 13.08 | 3.41 | showcase vs shipper | +9.67 [+9.56, +9.77] | +6.32 |
| shipper | deliveries | 12.83 | 15.65 | 10.36 | showcase vs shipper | +5.29 [+5.20, +5.38] | +4.33 |
| showcase | bought | 30.57 | 26.98 | 43.63 | showcase vs shipper | -16.65 [-16.92, -16.38] | -3.81 |
| producer | deliveries | 11.86 | 13.73 | 7.42 | showcase vs shipper | +6.31 [+6.16, +6.46] | +2.34 |
| shipper | produced | 4.92 | 11.01 | 12.44 | producer vs shipper | -7.52 [-7.67, -7.37] | -2.21 |
| showcase | produced | 6.15 | 12.74 | 10.04 | producer vs showcase | -6.59 [-6.75, -6.43] | -2.17 |
| producer | bought | 24.50 | 17.35 | 19.52 | producer vs showcase | +7.15 [+6.87, +7.42] | +1.83 |
| showcase | reprices | 2.71 | 6.36 | 4.77 | producer vs showcase | -3.65 [-3.77, -3.53] | -1.78 |
| shipper | factories_built | 0.40 | 1.00 | 0.80 | producer vs showcase | -0.60 [-0.60, -0.59] | -1.71 |

(12 smaller responses are in `economics.json`.)

### 5 seats, outcomes: 15 of 15 (archetype, metric) pairs respond

| focal | metric | vs producers | vs showcases | vs shippers | largest contrast | difference [95%] | SD |
|---|---|---|---|---|---|---|---|
| shipper | avg_purchase_price | 2.12 | 2.74 | 1.92 | showcase vs shipper | +0.82 [+0.82, +0.83] | +10.48 |
| producer | avg_purchase_price | 1.94 | 2.65 | 1.87 | showcase vs shipper | +0.78 [+0.78, +0.79] | +6.69 |
| showcase | avg_purchase_price | 1.67 | 2.38 | 1.90 | producer vs showcase | -0.71 [-0.72, -0.70] | -6.12 |
| showcase | sold | 26.94 | 26.98 | 50.75 | showcase vs shipper | -23.78 [-24.10, -23.47] | -4.30 |
| showcase | avg_sale_price | 2.59 | 2.38 | 2.68 | showcase vs shipper | -0.30 [-0.31, -0.30] | -3.11 |
| producer | avg_sale_price | 1.94 | 1.26 | 1.63 | producer vs showcase | +0.68 [+0.67, +0.69] | +2.86 |

(9 smaller responses are in `economics.json`.)

## Game quality

Stalled games reach the action horizon with nobody scoring; a default is a seat that misses an interest payment. The per-composition counts are in the payoff tables above.

| seats | games | stalled | games with a default | most default-prone composition | mean actions |
|---|---|---|---|---|---|
| 3 | 12,000 | 11 | 6.8% | P P Sh (14.0%) | 496 |
| 4 | 24,000 | 0 | 2.9% | P P Sh Sh (7.6%) | 724 |
| 5 | 42,000 | 0 | 1.6% | P P Sh Sh Sh (4.7%) | 929 |

Ledger check: Bank moves of any kind in these games: 0 (sail_bank 0, call_container_lot 0, call_cash_lot 0, outbid_container_lot 0, outbid_cash_lot 0, bid_cash 0, bid_container 0, done_bidding 0). The ledger's unexplained residual is negative in 0 of 341,967 seat-games and positive in 2203; it marks a default in 2191 games, and 2191 games show a Bank seizure or deal (2191 in both). The two detectors agree when these are equal. Every container bought was sold by another seat at the same price in all but 0 games.

## The empirical game of the three heuristics

Each table size is solved on its own, with the solvers of `training/metagame.py` on the 3-strategy symmetric game (the payoff of a seat against opponents drawn iid from the mixture). The chosen output is verified by the independent checker, and the bootstrap column re-measures the gain of that fixed mixture on resampled seeds: a gain below it cannot be told from sampling error. What a row shows is an empirically estimated approximate equilibrium relative to the tested policy population (the three heuristics), these table sizes and a best-response search budget of nothing beyond the three strategies. Several solutions can exist, and the others are listed below.

| n | solver | solution concept | support (weight) | restricted-game gain | independent check | bootstrap gain, mean / 95th pct | NashConv |
|---|---|---|---|---|---|---|---|
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | producer 0.37, showcase 0.19, shipper 0.44 | 0.0e+00 | 5.6e-17 (enumerate) | 0.005 / 0.009 | 0.0e+00 |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | producer 0.41, showcase 0.31, shipper 0.28 | 3.0e-13 | 3.0e-13 (enumerate) | 0.003 / 0.005 | 1.2e-12 |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | producer 0.57, showcase 0.20, shipper 0.23 | 0.0e+00 | 5.6e-17 (enumerate) | 0.002 / 0.004 | 0.0e+00 |

Utility of every strategy against sigma_n (n-1 opponents drawn iid from sigma_n; 1/n is an equal share):

| n | producer | showcase | shipper |
|---|---|---|---|
| 3 | 0.333 | 0.333 | 0.333 |
| 4 | 0.250 | 0.250 | 0.250 |
| 5 | 0.200 | 0.200 | 0.200 |

Every solver output (not only the chosen one):

| n | solver | concept | gain | NashConv | support |
|---|---|---|---|---|---|
| 3 | replicator | Nash candidate (verified by gain) | 3.3e-09 | 9.9e-09 | shipper |
| 3 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 1.1e-05 | 3.4e-05 | shipper |
| 3 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | producer, showcase, shipper |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | shipper |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | producer, shipper |
| 3 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 3.8e-14 | 1.1e-13 | producer, showcase, shipper |
| 4 | replicator | Nash candidate (verified by gain) | 2.0e-09 | 8.1e-09 | shipper |
| 4 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 3.6e-06 | 1.4e-05 | shipper |
| 4 | nash_conv_minimizer | Nash candidate (verified by gain) | 3.0e-13 | 1.2e-12 | producer, showcase, shipper |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | shipper |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 1.2e-15 | 4.7e-15 | producer, shipper |
| 4 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 7.8e-12 | 3.1e-11 | producer, showcase, shipper |
| 5 | replicator | not verified (replicator average iterate has gain 3.96e-03 > 1e-06); no Nash claim | 4.0e-03 | 2.0e-02 | producer, showcase, shipper |
| 5 | regret_matching | CCE (coarse correlated equilibrium of the n-player game, via average regret); the average strategy's own Nash gain is NOT guaranteed small | 3.3e-06 | 1.7e-05 | shipper |
| 5 | nash_conv_minimizer | Nash candidate (verified by gain) | 0.0e+00 | 0.0e+00 | producer, showcase, shipper |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | shipper |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 5.2e-14 | 2.6e-13 | producer, shipper |
| 5 | support_enumeration | Nash equilibrium (support enumeration, verified by gain <= 1e-07) | 0.0e+00 | 0.0e+00 | producer, showcase, shipper |

## Limits of what this shows

* The three heuristics are the whole population and each is one hand-set profile. A solution of this game is an equilibrium of the *empirical game*, relative to this tested population, these table sizes and no best-response search beyond the three strategies; it says nothing about deviations outside them and is not an equilibrium of ContainerAI.
* Their games are Bank-free by construction; the Bank-open and Bank-closed comparison is a later stage.
* The seed block is `DEV`: these are development numbers, not a claim. Any decision made from them is confirmed on seeds that were not used to make it.
* The heuristics lose to the nets (Peddler v2, Magnate, Consortium) in earlier evaluation; this report does not test that and does not try to close it.

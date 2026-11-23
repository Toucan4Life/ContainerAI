# Adaptive generalist: development run on the DEV seeds

Net under test: `runs/strategist/adaptive_s1/final.pt` (sha256 `13e277cbd2cd14d6`); references: Peddler v2 and `runs/strategist/control_s1/final.pt`. 100 seeds per composition and table size, the focus seat at every seat in turn, nets played `guarded` under the five table rules. Code: `6e4ad64eee30e5f3a0b7c63f89a5e130785263d2+dirty-7ca2196f76f1`.

Training: 923,012 games in 55 minutes, ladder rating of the final checkpoint 2431.2 (`adaptive_s1`, evaluated by the trainer on its own seeds, not on this block).

## 1. One net seat among the heuristics

The net sits at every seat in turn; the other seats are every composition of Producer (P), Showcase (Sc) and Shipper (Sh), each composition with the same seeds for every net. A stalled game pays nobody (`win`, `margin` count it as 0, the rest leave it out).

### 3 seats (an even split is 0.333)

| net | win share | score | margin | produced | sold | bought | deliveries | factories built | warehouses built | loans | opponents defaulted | stalled |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| adaptive | 0.802 [0.785, 0.818] | 98.1 [96.9, 99.1] | 38.9 [37.6, 40.1] | 9.61 | 7.88 | 11.78 | 6.43 | 0.53 | 0.00 | 0.64 | 2.1% | 0 of 1,800 |
| peddler2 | 0.591 [0.570, 0.611] | 91.4 [89.8, 93.0] | 22.5 [20.8, 24.2] | 12.03 | 9.76 | 10.85 | 7.73 | 1.11 | 0.00 | 1.01 | 3.4% | 0 of 1,800 |
| control_s1 | 0.500 [0.479, 0.521] | 91.1 [89.9, 92.5] | 15.8 [14.3, 17.4] | 8.58 | 4.67 | 6.69 | 8.93 | 0.56 | 0.00 | 0.75 | 2.5% | 1 of 1,800 |

Adaptive minus peddler2, paired on the seeds: win share +0.211 [+0.185, +0.237], score +6.6 [+4.9, +8.4], margin +16.4 [+14.5, +18.2].  
Adaptive minus control_s1, paired on the seeds: win share +0.302 [+0.277, +0.326], score +6.9 [+5.2, +8.6], margin +23.1 [+21.1, +25.0].  

By composition of the opponents (win share of the net in the focus seat):

| opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 |
|---|---|---|---|---|
| P P | 0.325 [0.273, 0.377] | 0.098 | 0.095 | +0.227 [+0.167, +0.288] |
| P Sc | 0.663 [0.610, 0.713] | 0.377 | 0.210 | +0.287 [+0.203, +0.357] |
| P Sh | 0.933 [0.903, 0.960] | 0.705 | 0.567 | +0.228 [+0.182, +0.275] |
| Sc Sc | 0.912 [0.877, 0.940] | 0.710 | 0.573 | +0.202 [+0.143, +0.265] |
| Sc Sh | 0.977 [0.957, 0.990] | 0.757 | 0.687 | +0.220 [+0.170, +0.267] |
| Sh Sh | 1.000 [1.000, 1.000] | 0.897 | 0.869 | +0.103 [+0.067, +0.140] |

### 4 seats (an even split is 0.250)

| net | win share | score | margin | produced | sold | bought | deliveries | factories built | warehouses built | loans | opponents defaulted | stalled |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| adaptive | 0.897 [0.889, 0.905] | 142.0 [140.9, 142.9] | 64.0 [62.9, 64.9] | 6.88 | 7.66 | 15.01 | 8.83 | 0.20 | 0.00 | 1.47 | 3.5% | 0 of 4,000 |
| peddler2 | 0.640 [0.627, 0.654] | 117.9 [117.0, 118.8] | 33.9 [32.9, 34.9] | 13.20 | 11.51 | 11.42 | 8.10 | 0.92 | 0.00 | 0.61 | 2.3% | 0 of 4,000 |
| control_s1 | 0.471 [0.457, 0.486] | 113.5 [112.7, 114.3] | 18.3 [17.4, 19.2] | 6.29 | 5.34 | 9.74 | 9.64 | 0.11 | 0.00 | 0.20 | 1.0% | 0 of 4,000 |

Adaptive minus peddler2, paired on the seeds: win share +0.257 [+0.242, +0.271], score +24.1 [+22.9, +25.2], margin +30.1 [+28.6, +31.3].  
Adaptive minus control_s1, paired on the seeds: win share +0.426 [+0.409, +0.442], score +28.5 [+27.3, +29.6], margin +45.7 [+44.5, +46.8].  

By composition of the opponents (win share of the net in the focus seat):

| opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 |
|---|---|---|---|---|
| P P P | 0.761 [0.719, 0.801] | 0.247 | 0.124 | +0.514 [+0.454, +0.573] |
| P P Sc | 0.814 [0.777, 0.849] | 0.412 | 0.229 | +0.401 [+0.339, +0.464] |
| P P Sh | 0.831 [0.792, 0.870] | 0.417 | 0.257 | +0.414 [+0.362, +0.470] |
| P Sc Sc | 0.831 [0.795, 0.864] | 0.609 | 0.353 | +0.223 [+0.160, +0.282] |
| P Sc Sh | 0.932 [0.907, 0.955] | 0.726 | 0.465 | +0.206 [+0.162, +0.250] |
| P Sh Sh | 0.958 [0.938, 0.978] | 0.569 | 0.624 | +0.389 [+0.339, +0.435] |
| Sc Sc Sc | 0.891 [0.860, 0.920] | 0.829 | 0.583 | +0.062 [+0.011, +0.110] |
| Sc Sc Sh | 0.970 [0.955, 0.985] | 0.935 | 0.615 | +0.035 [+0.007, +0.062] |
| Sc Sh Sh | 0.988 [0.975, 0.998] | 0.909 | 0.696 | +0.079 [+0.050, +0.108] |
| Sh Sh Sh | 0.993 [0.983, 1.000] | 0.748 | 0.767 | +0.245 [+0.208, +0.285] |

### 5 seats (an even split is 0.200)

| net | win share | score | margin | produced | sold | bought | deliveries | factories built | warehouses built | loans | opponents defaulted | stalled |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| adaptive | 0.845 [0.837, 0.853] | 160.3 [159.5, 161.0] | 66.2 [65.3, 67.0] | 6.36 | 8.38 | 15.57 | 9.12 | 0.18 | 0.01 | 1.25 | 3.3% | 0 of 7,500 |
| peddler2 | 0.649 [0.640, 0.659] | 140.5 [139.9, 141.2] | 45.1 [44.3, 45.9] | 12.76 | 11.49 | 12.24 | 8.84 | 0.81 | 0.00 | 0.26 | 2.7% | 0 of 7,500 |
| control_s1 | 0.440 [0.428, 0.453] | 127.1 [126.3, 127.9] | 23.4 [22.6, 24.3] | 5.52 | 4.07 | 8.75 | 9.91 | 0.30 | 0.00 | 0.29 | 1.5% | 0 of 7,500 |

Adaptive minus peddler2, paired on the seeds: win share +0.196 [+0.182, +0.209], score +19.7 [+18.9, +20.5], margin +21.1 [+20.1, +22.1].  
Adaptive minus control_s1, paired on the seeds: win share +0.405 [+0.390, +0.419], score +33.2 [+32.2, +34.2], margin +42.8 [+41.7, +43.9].  

By composition of the opponents (win share of the net in the focus seat):

| opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 |
|---|---|---|---|---|
| P P P P | 0.690 [0.646, 0.730] | 0.325 | 0.224 | +0.365 [+0.313, +0.419] |
| P P P Sc | 0.814 [0.780, 0.850] | 0.556 | 0.266 | +0.258 [+0.203, +0.312] |
| P P P Sh | 0.709 [0.670, 0.747] | 0.363 | 0.204 | +0.346 [+0.291, +0.405] |
| P P Sc Sc | 0.828 [0.798, 0.858] | 0.671 | 0.255 | +0.157 [+0.104, +0.209] |
| P P Sc Sh | 0.806 [0.774, 0.838] | 0.606 | 0.312 | +0.200 [+0.150, +0.252] |
| P P Sh Sh | 0.706 [0.668, 0.745] | 0.404 | 0.250 | +0.302 [+0.245, +0.360] |
| P Sc Sc Sc | 0.891 [0.866, 0.914] | 0.815 | 0.447 | +0.076 [+0.029, +0.122] |
| P Sc Sc Sh | 0.888 [0.858, 0.918] | 0.808 | 0.506 | +0.080 [+0.034, +0.126] |
| P Sc Sh Sh | 0.883 [0.856, 0.910] | 0.654 | 0.480 | +0.229 [+0.176, +0.282] |
| P Sh Sh Sh | 0.860 [0.830, 0.892] | 0.448 | 0.489 | +0.412 [+0.358, +0.464] |
| Sc Sc Sc Sc | 0.857 [0.826, 0.887] | 0.921 | 0.678 | -0.064 [-0.107, -0.024] |
| Sc Sc Sc Sh | 0.935 [0.911, 0.958] | 0.958 | 0.702 | -0.023 [-0.052, +0.006] |
| Sc Sc Sh Sh | 0.947 [0.929, 0.966] | 0.915 | 0.673 | +0.032 [+0.001, +0.064] |
| Sc Sh Sh Sh | 0.939 [0.917, 0.959] | 0.752 | 0.578 | +0.187 [+0.147, +0.226] |
| Sh Sh Sh Sh | 0.925 [0.900, 0.949] | 0.538 | 0.535 | +0.387 [+0.336, +0.436] |

## 2. One net seat among copies of a held-out net

The other seats are all the same net. The generalist never met Peddler v2, Magnate or Consortium in training (Peddler v2 is where it started; the control did train against Peddler v2 and Consortium). Peddler v2 against copies of itself is the check of the harness: an even split.

| seats | opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 | adaptive minus control_s1 | stalled (adaptive, peddler2, control_s1) |
|---|---|---|---|---|---|---|---|
| 3 | peddler2 x2 | 0.848 [0.807, 0.887] | 0.333 [0.333, 0.333] | 0.973 [0.950, 0.990] | +0.515 [+0.473, +0.553] | -0.125 [-0.168, -0.082] | 0, 0, 0 of 300 |
| 3 | magnate x2 | 0.538 [0.487, 0.592] | 0.320 [0.270, 0.370] | 0.695 [0.645, 0.745] | +0.218 [+0.148, +0.290] | -0.157 [-0.228, -0.080] | 0, 0, 0 of 300 |
| 3 | consortium x2 | 0.585 [0.523, 0.643] | 0.343 [0.293, 0.393] | 0.693 [0.648, 0.735] | +0.242 [+0.160, +0.320] | -0.108 [-0.187, -0.032] | 0, 0, 0 of 300 |
| 4 | peddler2 x3 | 0.836 [0.800, 0.870] | 0.250 [0.250, 0.250] | 0.861 [0.829, 0.890] | +0.586 [+0.550, +0.620] | -0.025 [-0.071, +0.024] | 0, 0, 0 of 400 |
| 4 | magnate x3 | 0.524 [0.471, 0.574] | 0.562 [0.519, 0.611] | 0.446 [0.395, 0.489] | -0.039 [-0.109, +0.029] | +0.078 [+0.008, +0.145] | 0, 0, 0 of 400 |
| 4 | consortium x3 | 0.356 [0.314, 0.407] | 0.359 [0.318, 0.403] | 0.279 [0.236, 0.323] | -0.003 [-0.057, +0.055] | +0.078 [+0.014, +0.144] | 0, 0, 0 of 400 |
| 5 | peddler2 x4 | 0.496 [0.454, 0.542] | 0.200 [0.200, 0.200] | 0.573 [0.527, 0.615] | +0.296 [+0.254, +0.342] | -0.077 [-0.141, -0.005] | 0, 0, 0 of 500 |
| 5 | magnate x4 | 0.602 [0.565, 0.637] | 0.657 [0.615, 0.696] | 0.672 [0.628, 0.711] | -0.055 [-0.108, +0.000] | -0.070 [-0.126, -0.015] | 0, 0, 0 of 500 |
| 5 | consortium x4 | 0.463 [0.420, 0.504] | 0.488 [0.450, 0.527] | 0.560 [0.522, 0.602] | -0.025 [-0.085, +0.035] | -0.097 [-0.156, -0.040] | 0, 0, 0 of 500 |

## 3. Does the net's play depend on its opponents?

The net's own counters per game when all the opponents are Producers, all Showcases or all Shippers, and the largest paired difference between two such fields in standard deviations of the seat's per-game value (the convention of `training/egta.py`: a response from 0.2 with the interval excluding 0). Then the smallest change of field: one opponent swapped from a Producer to a Shipper. Any net's counters move with the field, because the field changes what there is to buy, sell and deliver: that is why a reference net is read next to it (and why the flags below mostly say yes).

| seats | net | counter | vs Producers | vs Showcases | vs Shippers | largest difference [95% CI] | in SDs | responds | one Producer swapped for a Shipper [95% CI] | in SDs | responds |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 3 | adaptive | factories_built | 0.13 | 0.82 | 0.80 | P minus Sc: -0.69 [-0.74, -0.63] | -1.89 | yes | +0.15 [+0.11, +0.19] | +0.38 | yes |
| 3 | adaptive | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sc: +0.00 [+0.00, +0.00] | +0.00 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 3 | adaptive | bought | 10.73 | 8.41 | 17.79 | Sc minus Sh: -9.38 [-9.82, -8.94] | -3.06 | yes | +3.05 [+2.65, +3.48] | +1.10 | yes |
| 3 | adaptive | deliveries | 7.49 | 6.62 | 5.16 | P minus Sh: +2.32 [+2.12, +2.52] | +1.68 | yes | -2.41 [-2.61, -2.22] | -1.80 | yes |
| 3 | adaptive | sold | 0.44 | 9.35 | 15.33 | P minus Sh: -14.89 [-15.27, -14.46] | -5.94 | yes | +5.81 [+5.55, +6.09] | +3.35 | yes |
| 3 | adaptive | produced | 3.24 | 15.55 | 11.35 | P minus Sc: -12.31 [-12.81, -11.83] | -3.58 | yes | +0.92 [+0.58, +1.31] | +0.32 | yes |
| 3 | peddler2 | factories_built | 1.12 | 1.13 | 1.11 | Sc minus Sh: +0.02 [-0.05, +0.09] | +0.03 | no | -0.03 [-0.11, +0.05] | -0.06 | no |
| 3 | peddler2 | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sc: -0.00 [-0.01, +0.00] | -0.08 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 3 | peddler2 | bought | 8.43 | 9.88 | 14.38 | P minus Sh: -5.94 [-6.50, -5.38] | -1.62 | yes | +3.83 [+3.30, +4.32] | +1.10 | yes |
| 3 | peddler2 | deliveries | 7.03 | 8.38 | 7.01 | P minus Sc: -1.36 [-1.74, -0.93] | -0.64 | yes | -0.29 [-0.56, -0.04] | -0.16 | no |
| 3 | peddler2 | sold | 1.78 | 13.69 | 13.94 | P minus Sc: -11.92 [-12.46, -11.37] | -4.22 | yes | +6.88 [+6.34, +7.40] | +2.15 | yes |
| 3 | peddler2 | produced | 8.75 | 16.61 | 10.37 | P minus Sc: -7.86 [-8.43, -7.31] | -2.45 | yes | -0.08 [-0.48, +0.33] | -0.03 | no |
| 3 | control_s1 | factories_built | 0.11 | 0.78 | 0.94 | P minus Sh: -0.83 [-0.89, -0.78] | -2.01 | yes | +0.16 [+0.11, +0.20] | +0.40 | yes |
| 3 | control_s1 | warehouses_built | 0.00 | 0.00 | 0.01 | P minus Sh: -0.01 [-0.04, +0.00] | -0.08 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 3 | control_s1 | bought | 8.65 | 2.99 | 8.57 | P minus Sc: +5.66 [+5.36, +5.97] | +2.71 | yes | +1.23 [+0.88, +1.56] | +0.54 | yes |
| 3 | control_s1 | deliveries | 9.04 | 8.94 | 7.77 | P minus Sh: +1.26 [+0.97, +1.55] | +0.64 | yes | -1.54 [-1.82, -1.24] | -0.79 | yes |
| 3 | control_s1 | sold | 0.38 | 5.11 | 8.32 | P minus Sh: -7.94 [-8.34, -7.59] | -3.63 | yes | +4.51 [+4.21, +4.80] | +2.45 | yes |
| 3 | control_s1 | produced | 3.69 | 11.58 | 9.62 | P minus Sc: -7.89 [-8.37, -7.39] | -2.49 | yes | +1.57 [+1.22, +1.94] | +0.59 | yes |
| 4 | adaptive | factories_built | 0.07 | 0.27 | 0.27 | P minus Sc: -0.20 [-0.24, -0.16] | -0.56 | yes | +0.05 [+0.03, +0.08] | +0.19 | no |
| 4 | adaptive | warehouses_built | 0.00 | 0.00 | 0.01 | P minus Sh: -0.01 [-0.02, +0.00] | -0.10 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 4 | adaptive | bought | 13.48 | 14.17 | 21.47 | P minus Sh: -7.99 [-8.48, -7.49] | -2.23 | yes | +1.12 [+0.69, +1.50] | +0.34 | yes |
| 4 | adaptive | deliveries | 10.34 | 11.80 | 5.29 | Sc minus Sh: +6.51 [+6.29, +6.73] | +3.91 | yes | -1.95 [-2.16, -1.73] | -1.23 | yes |
| 4 | adaptive | sold | 1.17 | 8.96 | 16.75 | P minus Sh: -15.58 [-15.97, -15.17] | -5.55 | yes | +1.75 [+1.55, +1.94] | +1.15 | yes |
| 4 | adaptive | produced | 2.61 | 11.73 | 6.46 | P minus Sc: -9.12 [-9.59, -8.71] | -2.50 | yes | +0.50 [+0.28, +0.70] | +0.24 | yes |
| 4 | peddler2 | factories_built | 0.86 | 1.00 | 0.90 | P minus Sc: -0.14 [-0.18, -0.09] | -0.32 | yes | -0.02 [-0.05, +0.01] | -0.04 | no |
| 4 | peddler2 | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sc: +0.00 [+0.00, +0.00] | +0.00 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 4 | peddler2 | bought | 11.85 | 12.12 | 11.27 | Sc minus Sh: +0.85 [+0.23, +1.51] | +0.20 | no | -0.14 [-0.49, +0.23] | -0.04 | no |
| 4 | peddler2 | deliveries | 9.61 | 9.22 | 5.61 | P minus Sh: +4.00 [+3.80, +4.19] | +2.74 | yes | -1.29 [-1.49, -1.10] | -0.82 | yes |
| 4 | peddler2 | sold | 2.46 | 18.18 | 12.57 | P minus Sc: -15.72 [-16.34, -15.11] | -5.15 | yes | +2.45 [+2.04, +2.88] | +0.89 | yes |
| 4 | peddler2 | produced | 6.63 | 20.17 | 10.35 | P minus Sc: -13.54 [-14.10, -12.99] | -4.04 | yes | +0.60 [+0.33, +0.90] | +0.23 | yes |
| 4 | control_s1 | factories_built | 0.01 | 0.17 | 0.25 | P minus Sh: -0.24 [-0.28, -0.20] | -0.73 | yes | +0.00 [-0.01, +0.02] | +0.02 | no |
| 4 | control_s1 | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sc: +0.00 [+0.00, +0.00] | +0.00 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 4 | control_s1 | bought | 11.09 | 7.97 | 12.45 | Sc minus Sh: -4.48 [-4.88, -4.07] | -1.53 | yes | -0.68 [-0.98, -0.40] | -0.30 | yes |
| 4 | control_s1 | deliveries | 11.47 | 11.78 | 6.05 | Sc minus Sh: +5.72 [+5.51, +5.94] | +3.24 | yes | -2.14 [-2.39, -1.90] | -1.03 | yes |
| 4 | control_s1 | sold | 1.07 | 5.54 | 11.03 | P minus Sh: -9.95 [-10.30, -9.57] | -3.75 | yes | +1.82 [+1.63, +2.02] | +1.11 | yes |
| 4 | control_s1 | produced | 2.67 | 9.79 | 6.63 | P minus Sc: -7.12 [-7.47, -6.77] | -2.51 | yes | +0.53 [+0.38, +0.68] | +0.34 | yes |
| 5 | adaptive | factories_built | 0.10 | 0.23 | 0.27 | P minus Sh: -0.17 [-0.21, -0.14] | -0.45 | yes | +0.01 [+0.00, +0.02] | +0.03 | no |
| 5 | adaptive | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sh: +0.00 [-0.00, +0.01] | +0.04 | no | -0.00 [-0.01, +0.00] | -0.04 | no |
| 5 | adaptive | bought | 13.43 | 16.65 | 21.41 | P minus Sh: -7.98 [-8.38, -7.56] | -2.29 | yes | +1.33 [+0.99, +1.66] | +0.43 | yes |
| 5 | adaptive | deliveries | 10.49 | 12.53 | 5.62 | Sc minus Sh: +6.91 [+6.73, +7.09] | +4.16 | yes | -0.95 [-1.11, -0.77] | -0.65 | yes |
| 5 | adaptive | sold | 2.03 | 12.30 | 17.28 | P minus Sh: -15.25 [-15.58, -14.91] | -5.15 | yes | +1.37 [+1.19, +1.55] | +0.82 | yes |
| 5 | adaptive | produced | 3.00 | 12.14 | 5.46 | P minus Sc: -9.13 [-9.55, -8.69] | -2.05 | yes | +0.23 [+0.06, +0.40] | +0.11 | no |
| 5 | peddler2 | factories_built | 0.77 | 0.81 | 0.81 | P minus Sc: -0.04 [-0.06, -0.03] | -0.11 | no | +0.02 [+0.00, +0.04] | +0.05 | no |
| 5 | peddler2 | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sc: +0.00 [+0.00, +0.00] | +0.00 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 5 | peddler2 | bought | 14.72 | 12.94 | 11.25 | P minus Sh: +3.47 [+3.20, +3.78] | +1.23 | yes | -1.47 [-1.79, -1.17] | -0.54 | yes |
| 5 | peddler2 | deliveries | 11.07 | 10.98 | 5.59 | P minus Sh: +5.48 [+5.32, +5.63] | +3.90 | yes | -1.35 [-1.52, -1.19] | -0.85 | yes |
| 5 | peddler2 | sold | 2.71 | 21.44 | 12.90 | P minus Sc: -18.73 [-19.07, -18.39] | -5.58 | yes | +1.09 [+0.84, +1.34] | +0.49 | yes |
| 5 | peddler2 | produced | 5.43 | 22.78 | 10.69 | P minus Sc: -17.35 [-17.67, -17.02] | -4.56 | yes | +0.66 [+0.43, +0.90] | +0.29 | yes |
| 5 | control_s1 | factories_built | 0.05 | 0.41 | 0.49 | P minus Sh: -0.43 [-0.47, -0.39] | -1.10 | yes | +0.01 [-0.01, +0.03] | +0.03 | no |
| 5 | control_s1 | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sc: +0.00 [+0.00, +0.00] | +0.00 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 5 | control_s1 | bought | 10.49 | 8.78 | 10.65 | P minus Sc: +1.71 [+1.45, +1.96] | +0.80 | yes | -0.81 [-1.00, -0.61] | -0.42 | yes |
| 5 | control_s1 | deliveries | 11.84 | 12.97 | 6.24 | Sc minus Sh: +6.73 [+6.52, +6.94] | +4.14 | yes | -1.31 [-1.57, -1.08] | -0.69 | yes |
| 5 | control_s1 | sold | 1.49 | 4.34 | 9.54 | P minus Sh: -8.05 [-8.39, -7.72] | -3.14 | yes | +0.88 [+0.75, +1.04] | +0.71 | yes |
| 5 | control_s1 | produced | 2.77 | 8.37 | 6.18 | P minus Sc: -5.59 [-5.82, -5.36] | -2.48 | yes | +0.36 [+0.22, +0.51] | +0.25 | yes |

### Against the references

For the pair of fields where the generalist's own response is largest, the same difference for a reference net, and the difference between the two responses (a difference in differences, paired on the seeds) in the same SDs: did training against the heuristics change how the net answers its field? Peddler v2 is where the generalist started; `control_s1` is the same recipe on the older pool without the heuristics.

| seats | counter | fields | adaptive [95% CI] | reference | reference [95% CI] | adaptive minus reference [95% CI] | in SDs | answers differently |
|---|---|---|---|---|---|---|---|---|
| 3 | factories_built | P minus Sc | -0.69 [-0.74, -0.63] | peddler2 | -0.00 [-0.07, +0.06] | -0.68 [-0.76, -0.60] | -1.88 | yes |
| 3 | factories_built | P minus Sc | -0.69 [-0.74, -0.63] | control_s1 | -0.67 [-0.73, -0.62] | -0.02 [-0.09, +0.06] | -0.05 | no |
| 3 | warehouses_built | P minus Sc | +0.00 [+0.00, +0.00] | peddler2 | -0.00 [-0.01, +0.00] | +0.00 [+0.00, +0.01] | +0.00 | no |
| 3 | warehouses_built | P minus Sc | +0.00 [+0.00, +0.00] | control_s1 | +0.00 [+0.00, +0.00] | +0.00 [+0.00, +0.00] | +0.00 | no |
| 3 | bought | Sc minus Sh | -9.38 [-9.82, -8.94] | peddler2 | -4.50 [-5.17, -3.82] | -4.88 [-5.68, -4.06] | -1.59 | yes |
| 3 | bought | Sc minus Sh | -9.38 [-9.82, -8.94] | control_s1 | -5.58 [-5.95, -5.20] | -3.80 [-4.35, -3.23] | -1.24 | yes |
| 3 | deliveries | P minus Sh | +2.32 [+2.12, +2.52] | peddler2 | +0.02 [-0.25, +0.27] | +2.30 [+1.99, +2.60] | +1.66 | yes |
| 3 | deliveries | P minus Sh | +2.32 [+2.12, +2.52] | control_s1 | +1.26 [+0.97, +1.55] | +1.06 [+0.68, +1.41] | +0.77 | yes |
| 3 | sold | P minus Sh | -14.89 [-15.27, -14.46] | peddler2 | -12.16 [-12.64, -11.72] | -2.73 [-3.35, -2.08] | -1.09 | yes |
| 3 | sold | P minus Sh | -14.89 [-15.27, -14.46] | control_s1 | -7.94 [-8.34, -7.59] | -6.95 [-7.46, -6.40] | -2.77 | yes |
| 3 | produced | P minus Sc | -12.31 [-12.81, -11.83] | peddler2 | -7.86 [-8.43, -7.31] | -4.45 [-5.20, -3.74] | -1.30 | yes |
| 3 | produced | P minus Sc | -12.31 [-12.81, -11.83] | control_s1 | -7.89 [-8.37, -7.39] | -4.43 [-5.09, -3.78] | -1.29 | yes |
| 4 | factories_built | P minus Sc | -0.20 [-0.24, -0.16] | peddler2 | -0.14 [-0.18, -0.09] | -0.06 [-0.13, -0.01] | -0.18 | no |
| 4 | factories_built | P minus Sc | -0.20 [-0.24, -0.16] | control_s1 | -0.15 [-0.19, -0.12] | -0.05 [-0.10, +0.01] | -0.13 | no |
| 4 | warehouses_built | P minus Sh | -0.01 [-0.02, +0.00] | peddler2 | +0.00 [+0.00, +0.00] | -0.01 [-0.02, +0.00] | -0.10 | no |
| 4 | warehouses_built | P minus Sh | -0.01 [-0.02, +0.00] | control_s1 | +0.00 [+0.00, +0.00] | -0.01 [-0.02, +0.00] | -0.10 | no |
| 4 | bought | P minus Sh | -7.99 [-8.48, -7.49] | peddler2 | +0.58 [+0.13, +1.03] | -8.58 [-9.24, -7.94] | -2.40 | yes |
| 4 | bought | P minus Sh | -7.99 [-8.48, -7.49] | control_s1 | -1.36 [-1.74, -0.96] | -6.63 [-7.27, -6.02] | -1.85 | yes |
| 4 | deliveries | Sc minus Sh | +6.51 [+6.29, +6.73] | peddler2 | +3.61 [+3.29, +3.95] | +2.90 [+2.53, +3.26] | +1.74 | yes |
| 4 | deliveries | Sc minus Sh | +6.51 [+6.29, +6.73] | control_s1 | +5.72 [+5.51, +5.94] | +0.79 [+0.47, +1.10] | +0.47 | yes |
| 4 | sold | P minus Sh | -15.58 [-15.97, -15.17] | peddler2 | -10.11 [-10.62, -9.57] | -5.48 [-6.08, -4.81] | -1.95 | yes |
| 4 | sold | P minus Sh | -15.58 [-15.97, -15.17] | control_s1 | -9.95 [-10.30, -9.57] | -5.63 [-6.19, -5.07] | -2.00 | yes |
| 4 | produced | P minus Sc | -9.12 [-9.59, -8.71] | peddler2 | -13.54 [-14.10, -12.99] | +4.42 [+3.68, +5.14] | +1.21 | yes |
| 4 | produced | P minus Sc | -9.12 [-9.59, -8.71] | control_s1 | -7.12 [-7.47, -6.77] | -2.01 [-2.57, -1.46] | -0.55 | yes |
| 5 | factories_built | P minus Sh | -0.17 [-0.21, -0.14] | peddler2 | -0.04 [-0.06, -0.02] | -0.13 [-0.18, -0.09] | -0.35 | yes |
| 5 | factories_built | P minus Sh | -0.17 [-0.21, -0.14] | control_s1 | -0.43 [-0.47, -0.39] | +0.26 [+0.21, +0.31] | +0.69 | yes |
| 5 | warehouses_built | P minus Sh | +0.00 [-0.00, +0.01] | peddler2 | +0.00 [+0.00, +0.00] | +0.00 [-0.00, +0.01] | +0.04 | no |
| 5 | warehouses_built | P minus Sh | +0.00 [-0.00, +0.01] | control_s1 | +0.00 [+0.00, +0.00] | +0.00 [-0.00, +0.01] | +0.04 | no |
| 5 | bought | P minus Sh | -7.98 [-8.38, -7.56] | peddler2 | +3.47 [+3.20, +3.78] | -11.45 [-11.96, -10.97] | -3.29 | yes |
| 5 | bought | P minus Sh | -7.98 [-8.38, -7.56] | control_s1 | -0.16 [-0.49, +0.17] | -7.81 [-8.33, -7.23] | -2.25 | yes |
| 5 | deliveries | Sc minus Sh | +6.91 [+6.73, +7.09] | peddler2 | +5.38 [+5.18, +5.61] | +1.53 [+1.27, +1.78] | +0.92 | yes |
| 5 | deliveries | Sc minus Sh | +6.91 [+6.73, +7.09] | control_s1 | +6.73 [+6.52, +6.94] | +0.18 [-0.07, +0.42] | +0.11 | no |
| 5 | sold | P minus Sh | -15.25 [-15.58, -14.91] | peddler2 | -10.19 [-10.65, -9.71] | -5.07 [-5.67, -4.52] | -1.71 | yes |
| 5 | sold | P minus Sh | -15.25 [-15.58, -14.91] | control_s1 | -8.05 [-8.39, -7.72] | -7.20 [-7.66, -6.73] | -2.43 | yes |
| 5 | produced | P minus Sc | -9.13 [-9.55, -8.69] | peddler2 | -17.35 [-17.67, -17.02] | +8.22 [+7.72, +8.78] | +1.84 | yes |
| 5 | produced | P minus Sc | -9.13 [-9.55, -8.69] | control_s1 | -5.59 [-5.82, -5.36] | -3.54 [-4.03, -3.05] | -0.79 | yes |


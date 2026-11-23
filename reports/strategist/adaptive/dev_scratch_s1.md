# Adaptive generalist: development run on the DEV seeds

Net under test: `runs/strategist/adaptive_scratch_s1/final.pt` (sha256 `fcf8cf5f62dbbd3c`); references: Peddler v2 and `runs/strategist/control_s1/final.pt`. 100 seeds per composition and table size, the focus seat at every seat in turn, nets played `guarded` under the five table rules. Code: `6e4ad64eee30e5f3a0b7c63f89a5e130785263d2+dirty-7ca2196f76f1`.

Training: 1,875,865 games in 111 minutes, ladder rating of the final checkpoint 2205.7 (`adaptive_scratch_s1`, evaluated by the trainer on its own seeds, not on this block).

## 1. One net seat among the heuristics

The net sits at every seat in turn; the other seats are every composition of Producer (P), Showcase (Sc) and Shipper (Sh), each composition with the same seeds for every net. A stalled game pays nobody (`win`, `margin` count it as 0, the rest leave it out).

### 3 seats (an even split is 0.333)

| net | win share | score | margin | produced | sold | bought | deliveries | factories built | warehouses built | loans | opponents defaulted | stalled |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| adaptive | 0.689 [0.666, 0.711] | 99.2 [98.2, 100.2] | 31.1 [29.8, 32.5] | 5.32 | 8.02 | 13.75 | 4.02 | 0.00 | 0.00 | 0.15 | 2.6% | 2 of 1,800 |
| peddler2 | 0.591 [0.570, 0.611] | 91.4 [89.8, 93.0] | 22.5 [20.8, 24.2] | 12.03 | 9.76 | 10.85 | 7.73 | 1.11 | 0.00 | 1.01 | 3.4% | 0 of 1,800 |
| control_s1 | 0.500 [0.479, 0.521] | 91.1 [89.9, 92.5] | 15.8 [14.3, 17.4] | 8.58 | 4.67 | 6.69 | 8.93 | 0.56 | 0.00 | 0.75 | 2.5% | 1 of 1,800 |

Adaptive minus peddler2, paired on the seeds: win share +0.099 [+0.074, +0.126], score +7.8 [+6.1, +9.5], margin +8.6 [+6.6, +10.7].  
Adaptive minus control_s1, paired on the seeds: win share +0.189 [+0.159, +0.220], score +8.1 [+6.5, +9.6], margin +15.4 [+13.1, +17.5].  

By composition of the opponents (win share of the net in the focus seat):

| opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 |
|---|---|---|---|---|
| P P | 0.303 [0.252, 0.353] | 0.098 | 0.095 | +0.205 [+0.155, +0.252] |
| P Sc | 0.617 [0.558, 0.677] | 0.377 | 0.210 | +0.240 [+0.177, +0.308] |
| P Sh | 0.873 [0.833, 0.913] | 0.705 | 0.567 | +0.168 [+0.110, +0.228] |
| Sc Sc | 0.675 [0.622, 0.722] | 0.710 | 0.573 | -0.035 [-0.103, +0.032] |
| Sc Sh | 0.722 [0.667, 0.772] | 0.757 | 0.687 | -0.035 [-0.102, +0.033] |
| Sh Sh | 0.947 [0.917, 0.973] | 0.897 | 0.869 | +0.050 [+0.000, +0.097] |

### 4 seats (an even split is 0.250)

| net | win share | score | margin | produced | sold | bought | deliveries | factories built | warehouses built | loans | opponents defaulted | stalled |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| adaptive | 0.816 [0.804, 0.827] | 137.6 [136.6, 138.5] | 52.0 [51.0, 52.9] | 5.04 | 4.92 | 13.85 | 7.72 | 0.00 | 0.00 | 0.05 | 3.7% | 5 of 4,000 |
| peddler2 | 0.640 [0.627, 0.654] | 117.9 [117.0, 118.8] | 33.9 [32.9, 34.9] | 13.20 | 11.51 | 11.42 | 8.10 | 0.92 | 0.00 | 0.61 | 2.3% | 0 of 4,000 |
| control_s1 | 0.471 [0.457, 0.486] | 113.5 [112.7, 114.3] | 18.3 [17.4, 19.2] | 6.29 | 5.34 | 9.74 | 9.64 | 0.11 | 0.00 | 0.20 | 1.0% | 0 of 4,000 |

Adaptive minus peddler2, paired on the seeds: win share +0.176 [+0.159, +0.192], score +19.7 [+18.4, +21.0], margin +18.1 [+16.7, +19.3].  
Adaptive minus control_s1, paired on the seeds: win share +0.344 [+0.325, +0.363], score +24.1 [+22.9, +25.2], margin +33.7 [+32.4, +35.0].  

By composition of the opponents (win share of the net in the focus seat):

| opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 |
|---|---|---|---|---|
| P P P | 0.565 [0.509, 0.618] | 0.247 | 0.124 | +0.317 [+0.249, +0.380] |
| P P Sc | 0.619 [0.571, 0.664] | 0.412 | 0.229 | +0.206 [+0.150, +0.264] |
| P P Sh | 0.752 [0.715, 0.790] | 0.417 | 0.257 | +0.335 [+0.276, +0.400] |
| P Sc Sc | 0.765 [0.723, 0.802] | 0.609 | 0.353 | +0.156 [+0.101, +0.216] |
| P Sc Sh | 0.853 [0.821, 0.882] | 0.726 | 0.465 | +0.126 [+0.075, +0.175] |
| P Sh Sh | 0.875 [0.838, 0.907] | 0.569 | 0.624 | +0.306 [+0.254, +0.359] |
| Sc Sc Sc | 0.884 [0.856, 0.909] | 0.829 | 0.583 | +0.055 [+0.009, +0.098] |
| Sc Sc Sh | 0.932 [0.909, 0.953] | 0.935 | 0.615 | -0.003 [-0.030, +0.025] |
| Sc Sh Sh | 0.958 [0.938, 0.975] | 0.909 | 0.696 | +0.049 [+0.016, +0.082] |
| Sh Sh Sh | 0.955 [0.935, 0.973] | 0.748 | 0.767 | +0.207 [+0.165, +0.252] |

### 5 seats (an even split is 0.200)

| net | win share | score | margin | produced | sold | bought | deliveries | factories built | warehouses built | loans | opponents defaulted | stalled |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| adaptive | 0.837 [0.830, 0.844] | 157.8 [157.1, 158.5] | 63.7 [63.1, 64.4] | 1.93 | 2.60 | 19.31 | 9.30 | 0.00 | 0.01 | 0.03 | 3.4% | 0 of 7,500 |
| peddler2 | 0.649 [0.640, 0.659] | 140.5 [139.9, 141.2] | 45.1 [44.3, 45.9] | 12.76 | 11.49 | 12.24 | 8.84 | 0.81 | 0.00 | 0.26 | 2.7% | 0 of 7,500 |
| control_s1 | 0.440 [0.428, 0.453] | 127.1 [126.3, 127.9] | 23.4 [22.6, 24.3] | 5.52 | 4.07 | 8.75 | 9.91 | 0.30 | 0.00 | 0.29 | 1.5% | 0 of 7,500 |

Adaptive minus peddler2, paired on the seeds: win share +0.188 [+0.177, +0.198], score +17.2 [+16.3, +18.1], margin +18.5 [+17.6, +19.5].  
Adaptive minus control_s1, paired on the seeds: win share +0.397 [+0.383, +0.410], score +30.7 [+29.7, +31.6], margin +40.3 [+39.2, +41.4].  

By composition of the opponents (win share of the net in the focus seat):

| opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 |
|---|---|---|---|---|
| P P P P | 0.594 [0.555, 0.634] | 0.325 | 0.224 | +0.269 [+0.212, +0.324] |
| P P P Sc | 0.711 [0.672, 0.752] | 0.556 | 0.266 | +0.155 [+0.096, +0.215] |
| P P P Sh | 0.663 [0.626, 0.695] | 0.363 | 0.204 | +0.300 [+0.245, +0.353] |
| P P Sc Sc | 0.804 [0.766, 0.838] | 0.671 | 0.255 | +0.133 [+0.070, +0.191] |
| P P Sc Sh | 0.806 [0.768, 0.838] | 0.606 | 0.312 | +0.200 [+0.143, +0.254] |
| P P Sh Sh | 0.765 [0.727, 0.798] | 0.404 | 0.250 | +0.361 [+0.304, +0.414] |
| P Sc Sc Sc | 0.928 [0.904, 0.948] | 0.815 | 0.447 | +0.113 [+0.072, +0.158] |
| P Sc Sc Sh | 0.913 [0.886, 0.938] | 0.808 | 0.506 | +0.105 [+0.067, +0.146] |
| P Sc Sh Sh | 0.901 [0.876, 0.925] | 0.654 | 0.480 | +0.247 [+0.192, +0.297] |
| P Sh Sh Sh | 0.840 [0.807, 0.872] | 0.448 | 0.489 | +0.392 [+0.335, +0.448] |
| Sc Sc Sc Sc | 0.949 [0.931, 0.967] | 0.921 | 0.678 | +0.028 [+0.002, +0.056] |
| Sc Sc Sc Sh | 0.943 [0.922, 0.963] | 0.958 | 0.702 | -0.015 [-0.040, +0.010] |
| Sc Sc Sh Sh | 0.955 [0.937, 0.972] | 0.915 | 0.673 | +0.040 [+0.010, +0.069] |
| Sc Sh Sh Sh | 0.900 [0.873, 0.924] | 0.752 | 0.578 | +0.148 [+0.106, +0.192] |
| Sh Sh Sh Sh | 0.884 [0.854, 0.910] | 0.538 | 0.535 | +0.346 [+0.296, +0.393] |

## 2. One net seat among copies of a held-out net

The other seats are all the same net. The generalist never met Peddler v2, Magnate or Consortium in training (Peddler v2 is where it started; the control did train against Peddler v2 and Consortium). Peddler v2 against copies of itself is the check of the harness: an even split.

| seats | opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 | adaptive minus control_s1 | stalled (adaptive, peddler2, control_s1) |
|---|---|---|---|---|---|---|---|
| 3 | peddler2 x2 | 0.283 [0.230, 0.337] | 0.333 [0.333, 0.333] | 0.973 [0.950, 0.990] | -0.050 [-0.103, +0.003] | -0.690 [-0.740, -0.637] | 0, 0, 0 of 300 |
| 3 | magnate x2 | 0.577 [0.520, 0.630] | 0.320 [0.270, 0.370] | 0.695 [0.645, 0.745] | +0.257 [+0.185, +0.323] | -0.118 [-0.193, -0.050] | 0, 0, 0 of 300 |
| 3 | consortium x2 | 0.540 [0.492, 0.588] | 0.343 [0.293, 0.393] | 0.693 [0.648, 0.735] | +0.197 [+0.122, +0.263] | -0.153 [-0.217, -0.087] | 0, 0, 0 of 300 |
| 4 | peddler2 x3 | 0.164 [0.130, 0.200] | 0.250 [0.250, 0.250] | 0.861 [0.829, 0.890] | -0.086 [-0.120, -0.050] | -0.698 [-0.744, -0.646] | 0, 0, 0 of 400 |
| 4 | magnate x3 | 0.725 [0.677, 0.770] | 0.562 [0.519, 0.611] | 0.446 [0.395, 0.489] | +0.162 [+0.095, +0.221] | +0.279 [+0.212, +0.346] | 0, 0, 0 of 400 |
| 4 | consortium x3 | 0.639 [0.589, 0.684] | 0.359 [0.318, 0.403] | 0.279 [0.236, 0.323] | +0.280 [+0.218, +0.335] | +0.360 [+0.292, +0.428] | 0, 0, 0 of 400 |
| 5 | peddler2 x4 | 0.052 [0.034, 0.072] | 0.200 [0.200, 0.200] | 0.573 [0.527, 0.615] | -0.148 [-0.166, -0.128] | -0.521 [-0.565, -0.474] | 0, 0, 0 of 500 |
| 5 | magnate x4 | 0.358 [0.316, 0.401] | 0.657 [0.615, 0.696] | 0.672 [0.628, 0.711] | -0.299 [-0.351, -0.235] | -0.314 [-0.371, -0.252] | 0, 0, 0 of 500 |
| 5 | consortium x4 | 0.246 [0.209, 0.281] | 0.488 [0.450, 0.527] | 0.560 [0.522, 0.602] | -0.242 [-0.298, -0.189] | -0.314 [-0.368, -0.263] | 0, 0, 0 of 500 |

## 3. Does the net's play depend on its opponents?

The net's own counters per game when all the opponents are Producers, all Showcases or all Shippers, and the largest paired difference between two such fields in standard deviations of the seat's per-game value (the convention of `training/egta.py`: a response from 0.2 with the interval excluding 0). Then the smallest change of field: one opponent swapped from a Producer to a Shipper. Any net's counters move with the field, because the field changes what there is to buy, sell and deliver: that is why a reference net is read next to it (and why the flags below mostly say yes).

| seats | net | counter | vs Producers | vs Showcases | vs Shippers | largest difference [95% CI] | in SDs | responds | one Producer swapped for a Shipper [95% CI] | in SDs | responds |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 3 | adaptive | factories_built | 0.00 | 0.00 | 0.01 | P minus Sh: -0.01 [-0.02, +0.00] | -0.14 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 3 | adaptive | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sh: -0.00 [-0.01, +0.00] | -0.08 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 3 | adaptive | bought | 16.07 | 8.75 | 18.33 | Sc minus Sh: -9.59 [-10.11, -9.13] | -2.66 | yes | -1.89 [-2.40, -1.42] | -0.55 | yes |
| 3 | adaptive | deliveries | 4.83 | 4.11 | 3.37 | P minus Sh: +1.46 [+1.26, +1.66] | +1.05 | yes | -1.59 [-1.76, -1.42] | -1.33 | yes |
| 3 | adaptive | sold | 1.48 | 7.78 | 15.64 | P minus Sh: -14.17 [-14.57, -13.80] | -4.99 | yes | +5.24 [+4.95, +5.55] | +2.64 | yes |
| 3 | adaptive | produced | 1.22 | 8.41 | 6.72 | P minus Sc: -7.19 [-7.43, -6.97] | -4.69 | yes | +1.77 [+1.56, +2.00] | +1.23 | yes |
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
| 4 | adaptive | factories_built | 0.00 | 0.00 | 0.00 | P minus Sh: -0.00 [-0.01, +0.00] | -0.07 | no | +0.00 [+0.00, +0.01] | +0.07 | no |
| 4 | adaptive | warehouses_built | 0.00 | 0.00 | 0.01 | P minus Sh: -0.01 [-0.04, +0.00] | -0.09 | no | +0.01 [+0.00, +0.02] | +0.10 | no |
| 4 | adaptive | bought | 18.49 | 13.02 | 13.03 | P minus Sh: +5.46 [+4.97, +5.93] | +1.41 | yes | -3.35 [-3.80, -2.93] | -0.81 | yes |
| 4 | adaptive | deliveries | 7.67 | 10.03 | 5.68 | Sc minus Sh: +4.34 [+4.19, +4.51] | +3.20 | yes | -0.71 [-0.88, -0.51] | -0.50 | yes |
| 4 | adaptive | sold | 0.89 | 5.73 | 11.02 | P minus Sh: -10.14 [-10.47, -9.82] | -3.82 | yes | +1.21 [+1.01, +1.41] | +0.89 | yes |
| 4 | adaptive | produced | 1.36 | 7.88 | 5.71 | P minus Sc: -6.51 [-6.84, -6.18] | -2.64 | yes | +1.14 [+0.94, +1.32] | +0.85 | yes |
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
| 5 | adaptive | factories_built | 0.00 | 0.00 | 0.00 | P minus Sc: +0.00 [+0.00, +0.00] | +0.00 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 5 | adaptive | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sc: -0.00 [-0.01, +0.00] | -0.06 | no | +0.01 [+0.00, +0.02] | +0.10 | no |
| 5 | adaptive | bought | 21.00 | 21.48 | 15.60 | P minus Sh: +5.40 [+4.98, +5.88] | +1.61 | yes | -1.03 [-1.51, -0.54] | -0.27 | yes |
| 5 | adaptive | deliveries | 8.62 | 13.45 | 7.39 | Sc minus Sh: +6.06 [+5.86, +6.27] | +3.69 | yes | +0.01 [-0.15, +0.18] | +0.00 | no |
| 5 | adaptive | sold | 0.54 | 3.28 | 5.33 | P minus Sh: -4.79 [-4.99, -4.56] | -2.47 | yes | +0.51 [+0.39, +0.65] | +0.50 | yes |
| 5 | adaptive | produced | 0.53 | 3.19 | 2.20 | P minus Sc: -2.66 [-2.84, -2.47] | -1.65 | yes | +0.28 [+0.18, +0.37] | +0.31 | yes |
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
| 3 | factories_built | P minus Sh | -0.01 [-0.02, +0.00] | peddler2 | +0.01 [-0.06, +0.08] | -0.02 [-0.10, +0.05] | -0.33 | no |
| 3 | factories_built | P minus Sh | -0.01 [-0.02, +0.00] | control_s1 | -0.83 [-0.89, -0.78] | +0.82 [+0.76, +0.88] | +11.66 | yes |
| 3 | warehouses_built | P minus Sh | -0.00 [-0.01, +0.00] | peddler2 | -0.00 [-0.01, +0.00] | -0.00 [-0.01, +0.01] | -0.00 | no |
| 3 | warehouses_built | P minus Sh | -0.00 [-0.01, +0.00] | control_s1 | -0.01 [-0.04, +0.00] | +0.01 [-0.01, +0.04] | +0.24 | no |
| 3 | bought | Sc minus Sh | -9.59 [-10.11, -9.13] | peddler2 | -4.50 [-5.17, -3.82] | -5.09 [-5.96, -4.23] | -1.41 | yes |
| 3 | bought | Sc minus Sh | -9.59 [-10.11, -9.13] | control_s1 | -5.58 [-5.95, -5.20] | -4.01 [-4.70, -3.44] | -1.11 | yes |
| 3 | deliveries | P minus Sh | +1.46 [+1.26, +1.66] | peddler2 | +0.02 [-0.25, +0.27] | +1.44 [+1.13, +1.77] | +1.03 | yes |
| 3 | deliveries | P minus Sh | +1.46 [+1.26, +1.66] | control_s1 | +1.26 [+0.97, +1.55] | +0.20 [-0.11, +0.55] | +0.14 | no |
| 3 | sold | P minus Sh | -14.17 [-14.57, -13.80] | peddler2 | -12.16 [-12.64, -11.72] | -2.01 [-2.60, -1.39] | -0.71 | yes |
| 3 | sold | P minus Sh | -14.17 [-14.57, -13.80] | control_s1 | -7.94 [-8.34, -7.59] | -6.23 [-6.71, -5.77] | -2.19 | yes |
| 3 | produced | P minus Sc | -7.19 [-7.43, -6.97] | peddler2 | -7.86 [-8.43, -7.31] | +0.67 [+0.06, +1.28] | +0.44 | yes |
| 3 | produced | P minus Sc | -7.19 [-7.43, -6.97] | control_s1 | -7.89 [-8.37, -7.39] | +0.70 [+0.12, +1.21] | +0.45 | yes |
| 4 | factories_built | P minus Sh | -0.00 [-0.01, +0.00] | peddler2 | -0.03 [-0.07, +0.01] | +0.03 [-0.01, +0.07] | +0.84 | no |
| 4 | factories_built | P minus Sh | -0.00 [-0.01, +0.00] | control_s1 | -0.24 [-0.28, -0.20] | +0.24 [+0.19, +0.28] | +6.68 | yes |
| 4 | warehouses_built | P minus Sh | -0.01 [-0.04, +0.00] | peddler2 | +0.00 [+0.00, +0.00] | -0.01 [-0.04, +0.00] | -0.09 | no |
| 4 | warehouses_built | P minus Sh | -0.01 [-0.04, +0.00] | control_s1 | +0.00 [+0.00, +0.00] | -0.01 [-0.04, +0.00] | -0.09 | no |
| 4 | bought | P minus Sh | +5.46 [+4.97, +5.93] | peddler2 | +0.58 [+0.13, +1.03] | +4.88 [+4.23, +5.52] | +1.25 | yes |
| 4 | bought | P minus Sh | +5.46 [+4.97, +5.93] | control_s1 | -1.36 [-1.74, -0.96] | +6.82 [+6.21, +7.42] | +1.76 | yes |
| 4 | deliveries | Sc minus Sh | +4.34 [+4.19, +4.51] | peddler2 | +3.61 [+3.29, +3.95] | +0.73 [+0.40, +1.06] | +0.54 | yes |
| 4 | deliveries | Sc minus Sh | +4.34 [+4.19, +4.51] | control_s1 | +5.72 [+5.51, +5.94] | -1.38 [-1.61, -1.12] | -1.01 | yes |
| 4 | sold | P minus Sh | -10.14 [-10.47, -9.82] | peddler2 | -10.11 [-10.62, -9.57] | -0.03 [-0.66, +0.53] | -0.01 | no |
| 4 | sold | P minus Sh | -10.14 [-10.47, -9.82] | control_s1 | -9.95 [-10.30, -9.57] | -0.18 [-0.70, +0.30] | -0.07 | no |
| 4 | produced | P minus Sc | -6.51 [-6.84, -6.18] | peddler2 | -13.54 [-14.10, -12.99] | +7.03 [+6.32, +7.72] | +2.85 | yes |
| 4 | produced | P minus Sc | -6.51 [-6.84, -6.18] | control_s1 | -7.12 [-7.47, -6.77] | +0.60 [+0.15, +1.03] | +0.24 | yes |
| 5 | factories_built | P minus Sc | +0.00 [+0.00, +0.00] | peddler2 | -0.04 [-0.06, -0.03] | +0.04 [+0.03, +0.06] | +0.04 | no |
| 5 | factories_built | P minus Sc | +0.00 [+0.00, +0.00] | control_s1 | -0.35 [-0.39, -0.31] | +0.35 [+0.31, +0.39] | +0.35 | yes |
| 5 | warehouses_built | P minus Sc | -0.00 [-0.01, +0.00] | peddler2 | +0.00 [+0.00, +0.00] | -0.00 [-0.01, +0.00] | -0.06 | no |
| 5 | warehouses_built | P minus Sc | -0.00 [-0.01, +0.00] | control_s1 | +0.00 [+0.00, +0.00] | -0.00 [-0.01, +0.00] | -0.06 | no |
| 5 | bought | P minus Sh | +5.40 [+4.98, +5.88] | peddler2 | +3.47 [+3.20, +3.78] | +1.93 [+1.42, +2.49] | +0.57 | yes |
| 5 | bought | P minus Sh | +5.40 [+4.98, +5.88] | control_s1 | -0.16 [-0.49, +0.17] | +5.56 [+5.05, +6.12] | +1.66 | yes |
| 5 | deliveries | Sc minus Sh | +6.06 [+5.86, +6.27] | peddler2 | +5.38 [+5.18, +5.61] | +0.67 [+0.36, +0.99] | +0.41 | yes |
| 5 | deliveries | Sc minus Sh | +6.06 [+5.86, +6.27] | control_s1 | +6.73 [+6.52, +6.94] | -0.67 [-0.96, -0.39] | -0.41 | yes |
| 5 | sold | P minus Sh | -4.79 [-4.99, -4.56] | peddler2 | -10.19 [-10.65, -9.71] | +5.40 [+4.89, +5.97] | +2.79 | yes |
| 5 | sold | P minus Sh | -4.79 [-4.99, -4.56] | control_s1 | -8.05 [-8.39, -7.72] | +3.26 [+2.89, +3.66] | +1.68 | yes |
| 5 | produced | P minus Sc | -2.66 [-2.84, -2.47] | peddler2 | -17.35 [-17.67, -17.02] | +14.69 [+14.29, +15.07] | +9.14 | yes |
| 5 | produced | P minus Sc | -2.66 [-2.84, -2.47] | control_s1 | -5.59 [-5.82, -5.36] | +2.93 [+2.66, +3.24] | +1.83 | yes |


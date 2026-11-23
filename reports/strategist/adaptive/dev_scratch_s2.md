# Adaptive generalist: development run on the DEV seeds

Net under test: `runs/strategist/adaptive_scratch_s2/final.pt` (sha256 `6a9af1feaf0ffca2`); references: Peddler v2 and `runs/strategist/control_s1/final.pt`. 100 seeds per composition and table size, the focus seat at every seat in turn, nets played `guarded` under the five table rules. Code: `6e4ad64eee30e5f3a0b7c63f89a5e130785263d2+dirty-7ca2196f76f1`.

Training: 2,062,512 games in 111 minutes, ladder rating of the final checkpoint 2241.0 (`adaptive_scratch_s2`, evaluated by the trainer on its own seeds, not on this block).

## 1. One net seat among the heuristics

The net sits at every seat in turn; the other seats are every composition of Producer (P), Showcase (Sc) and Shipper (Sh), each composition with the same seeds for every net. A stalled game pays nobody (`win`, `margin` count it as 0, the rest leave it out).

### 3 seats (an even split is 0.333)

| net | win share | score | margin | produced | sold | bought | deliveries | factories built | warehouses built | loans | opponents defaulted | stalled |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| adaptive | 0.684 [0.667, 0.702] | 90.9 [89.9, 91.9] | 32.1 [30.8, 33.6] | 5.17 | 5.34 | 17.94 | 5.26 | 0.03 | 0.02 | 0.02 | 3.3% | 0 of 1,800 |
| peddler2 | 0.591 [0.570, 0.611] | 91.4 [89.8, 93.0] | 22.5 [20.8, 24.2] | 12.03 | 9.76 | 10.85 | 7.73 | 1.11 | 0.00 | 1.01 | 3.4% | 0 of 1,800 |
| control_s1 | 0.500 [0.479, 0.521] | 91.1 [89.9, 92.5] | 15.8 [14.3, 17.4] | 8.58 | 4.67 | 6.69 | 8.93 | 0.56 | 0.00 | 0.75 | 2.5% | 1 of 1,800 |

Adaptive minus peddler2, paired on the seeds: win share +0.094 [+0.065, +0.124], score -0.5 [-2.5, +1.3], margin +9.6 [+7.4, +11.8].  
Adaptive minus control_s1, paired on the seeds: win share +0.184 [+0.160, +0.209], score -0.2 [-1.9, +1.4], margin +16.3 [+14.2, +18.4].  

By composition of the opponents (win share of the net in the focus seat):

| opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 |
|---|---|---|---|---|
| P P | 0.218 [0.175, 0.270] | 0.098 | 0.095 | +0.120 [+0.070, +0.178] |
| P Sc | 0.417 [0.357, 0.473] | 0.377 | 0.210 | +0.040 [-0.040, +0.110] |
| P Sh | 0.933 [0.907, 0.960] | 0.705 | 0.567 | +0.228 [+0.168, +0.287] |
| Sc Sc | 0.753 [0.703, 0.802] | 0.710 | 0.573 | +0.043 [-0.022, +0.115] |
| Sc Sh | 0.798 [0.760, 0.840] | 0.757 | 0.687 | +0.042 [-0.018, +0.100] |
| Sh Sh | 0.987 [0.972, 0.997] | 0.897 | 0.869 | +0.090 [+0.053, +0.130] |

### 4 seats (an even split is 0.250)

| net | win share | score | margin | produced | sold | bought | deliveries | factories built | warehouses built | loans | opponents defaulted | stalled |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| adaptive | 0.884 [0.874, 0.895] | 129.3 [128.1, 130.4] | 56.5 [55.4, 57.7] | 3.69 | 4.58 | 21.27 | 6.72 | 0.00 | 0.00 | 0.00 | 3.1% | 3 of 4,000 |
| peddler2 | 0.640 [0.627, 0.654] | 117.9 [117.0, 118.8] | 33.9 [32.9, 34.9] | 13.20 | 11.51 | 11.42 | 8.10 | 0.92 | 0.00 | 0.61 | 2.3% | 0 of 4,000 |
| control_s1 | 0.471 [0.457, 0.486] | 113.5 [112.7, 114.3] | 18.3 [17.4, 19.2] | 6.29 | 5.34 | 9.74 | 9.64 | 0.11 | 0.00 | 0.20 | 1.0% | 0 of 4,000 |

Adaptive minus peddler2, paired on the seeds: win share +0.244 [+0.229, +0.259], score +11.5 [+10.2, +12.8], margin +22.6 [+21.2, +24.0].  
Adaptive minus control_s1, paired on the seeds: win share +0.413 [+0.396, +0.431], score +15.8 [+14.6, +17.2], margin +38.2 [+36.8, +39.7].  

By composition of the opponents (win share of the net in the focus seat):

| opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 |
|---|---|---|---|---|
| P P P | 0.672 [0.626, 0.719] | 0.247 | 0.124 | +0.425 [+0.364, +0.486] |
| P P Sc | 0.745 [0.706, 0.785] | 0.412 | 0.229 | +0.333 [+0.274, +0.393] |
| P P Sh | 0.851 [0.816, 0.884] | 0.417 | 0.257 | +0.434 [+0.376, +0.488] |
| P Sc Sc | 0.836 [0.801, 0.874] | 0.609 | 0.353 | +0.228 [+0.175, +0.281] |
| P Sc Sh | 0.927 [0.905, 0.950] | 0.726 | 0.465 | +0.201 [+0.151, +0.246] |
| P Sh Sh | 0.955 [0.934, 0.975] | 0.569 | 0.624 | +0.386 [+0.335, +0.435] |
| Sc Sc Sc | 0.945 [0.922, 0.968] | 0.829 | 0.583 | +0.116 [+0.075, +0.155] |
| Sc Sc Sh | 0.973 [0.953, 0.990] | 0.935 | 0.615 | +0.037 [+0.012, +0.065] |
| Sc Sh Sh | 0.970 [0.953, 0.985] | 0.909 | 0.696 | +0.061 [+0.032, +0.091] |
| Sh Sh Sh | 0.970 [0.954, 0.985] | 0.748 | 0.767 | +0.222 [+0.184, +0.265] |

### 5 seats (an even split is 0.200)

| net | win share | score | margin | produced | sold | bought | deliveries | factories built | warehouses built | loans | opponents defaulted | stalled |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| adaptive | 0.908 [0.902, 0.914] | 155.5 [154.5, 156.4] | 70.6 [69.7, 71.6] | 2.26 | 3.28 | 22.61 | 8.22 | 0.00 | 0.00 | 0.00 | 3.6% | 0 of 7,500 |
| peddler2 | 0.649 [0.640, 0.659] | 140.5 [139.9, 141.2] | 45.1 [44.3, 45.9] | 12.76 | 11.49 | 12.24 | 8.84 | 0.81 | 0.00 | 0.26 | 2.7% | 0 of 7,500 |
| control_s1 | 0.440 [0.428, 0.453] | 127.1 [126.3, 127.9] | 23.4 [22.6, 24.3] | 5.52 | 4.07 | 8.75 | 9.91 | 0.30 | 0.00 | 0.29 | 1.5% | 0 of 7,500 |

Adaptive minus peddler2, paired on the seeds: win share +0.259 [+0.247, +0.272], score +14.9 [+13.7, +16.1], margin +25.5 [+24.3, +26.7].  
Adaptive minus control_s1, paired on the seeds: win share +0.468 [+0.454, +0.481], score +28.4 [+27.2, +29.5], margin +47.2 [+45.9, +48.6].  

By composition of the opponents (win share of the net in the focus seat):

| opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 |
|---|---|---|---|---|
| P P P P | 0.727 [0.689, 0.767] | 0.325 | 0.224 | +0.402 [+0.351, +0.452] |
| P P P Sc | 0.849 [0.819, 0.878] | 0.556 | 0.266 | +0.293 [+0.241, +0.351] |
| P P P Sh | 0.822 [0.793, 0.850] | 0.363 | 0.204 | +0.459 [+0.406, +0.514] |
| P P Sc Sc | 0.908 [0.887, 0.929] | 0.671 | 0.255 | +0.237 [+0.181, +0.291] |
| P P Sc Sh | 0.893 [0.866, 0.916] | 0.606 | 0.312 | +0.287 [+0.236, +0.332] |
| P P Sh Sh | 0.842 [0.812, 0.871] | 0.404 | 0.250 | +0.438 [+0.387, +0.488] |
| P Sc Sc Sc | 0.947 [0.923, 0.968] | 0.815 | 0.447 | +0.132 [+0.091, +0.178] |
| P Sc Sc Sh | 0.960 [0.940, 0.978] | 0.808 | 0.506 | +0.152 [+0.115, +0.189] |
| P Sc Sh Sh | 0.943 [0.924, 0.962] | 0.654 | 0.480 | +0.289 [+0.241, +0.336] |
| P Sh Sh Sh | 0.903 [0.872, 0.927] | 0.448 | 0.489 | +0.455 [+0.409, +0.500] |
| Sc Sc Sc Sc | 0.970 [0.954, 0.985] | 0.921 | 0.678 | +0.049 [+0.021, +0.077] |
| Sc Sc Sc Sh | 0.984 [0.972, 0.994] | 0.958 | 0.702 | +0.026 [+0.007, +0.044] |
| Sc Sc Sh Sh | 0.976 [0.962, 0.988] | 0.915 | 0.673 | +0.061 [+0.035, +0.088] |
| Sc Sh Sh Sh | 0.966 [0.950, 0.980] | 0.752 | 0.578 | +0.214 [+0.177, +0.253] |
| Sh Sh Sh Sh | 0.933 [0.913, 0.953] | 0.538 | 0.535 | +0.395 [+0.345, +0.440] |

## 2. One net seat among copies of a held-out net

The other seats are all the same net. The generalist never met Peddler v2, Magnate or Consortium in training (Peddler v2 is where it started; the control did train against Peddler v2 and Consortium). Peddler v2 against copies of itself is the check of the harness: an even split.

| seats | opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 | adaptive minus control_s1 | stalled (adaptive, peddler2, control_s1) |
|---|---|---|---|---|---|---|---|
| 3 | peddler2 x2 | 0.160 [0.120, 0.200] | 0.333 [0.333, 0.333] | 0.973 [0.950, 0.990] | -0.173 [-0.213, -0.133] | -0.813 [-0.857, -0.767] | 0, 0, 0 of 300 |
| 3 | magnate x2 | 0.258 [0.213, 0.307] | 0.320 [0.270, 0.370] | 0.695 [0.645, 0.745] | -0.062 [-0.137, +0.022] | -0.437 [-0.513, -0.365] | 0, 0, 0 of 300 |
| 3 | consortium x2 | 0.267 [0.223, 0.313] | 0.343 [0.293, 0.393] | 0.693 [0.648, 0.735] | -0.077 [-0.152, -0.003] | -0.427 [-0.487, -0.365] | 0, 0, 0 of 300 |
| 4 | peddler2 x3 | 0.062 [0.043, 0.085] | 0.250 [0.250, 0.250] | 0.861 [0.829, 0.890] | -0.188 [-0.207, -0.165] | -0.799 [-0.838, -0.761] | 0, 0, 0 of 400 |
| 4 | magnate x3 | 0.471 [0.429, 0.516] | 0.562 [0.519, 0.611] | 0.446 [0.395, 0.489] | -0.091 [-0.159, -0.028] | +0.025 [-0.035, +0.089] | 0, 0, 0 of 400 |
| 4 | consortium x3 | 0.356 [0.310, 0.403] | 0.359 [0.318, 0.403] | 0.279 [0.236, 0.323] | -0.003 [-0.061, +0.055] | +0.078 [+0.012, +0.143] | 0, 0, 0 of 400 |
| 5 | peddler2 x4 | 0.022 [0.010, 0.034] | 0.200 [0.200, 0.200] | 0.573 [0.527, 0.615] | -0.178 [-0.190, -0.166] | -0.551 [-0.594, -0.503] | 0, 0, 0 of 500 |
| 5 | magnate x4 | 0.306 [0.264, 0.346] | 0.657 [0.615, 0.696] | 0.672 [0.628, 0.711] | -0.351 [-0.408, -0.294] | -0.366 [-0.424, -0.306] | 0, 0, 0 of 500 |
| 5 | consortium x4 | 0.204 [0.170, 0.237] | 0.488 [0.450, 0.527] | 0.560 [0.522, 0.602] | -0.284 [-0.335, -0.235] | -0.356 [-0.412, -0.303] | 0, 0, 0 of 500 |

## 3. Does the net's play depend on its opponents?

The net's own counters per game when all the opponents are Producers, all Showcases or all Shippers, and the largest paired difference between two such fields in standard deviations of the seat's per-game value (the convention of `training/egta.py`: a response from 0.2 with the interval excluding 0). Then the smallest change of field: one opponent swapped from a Producer to a Shipper. Any net's counters move with the field, because the field changes what there is to buy, sell and deliver: that is why a reference net is read next to it (and why the flags below mostly say yes).

| seats | net | counter | vs Producers | vs Showcases | vs Shippers | largest difference [95% CI] | in SDs | responds | one Producer swapped for a Shipper [95% CI] | in SDs | responds |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 3 | adaptive | factories_built | 0.00 | 0.01 | 0.13 | P minus Sh: -0.13 [-0.19, -0.07] | -0.36 | yes | +0.02 [+0.00, +0.05] | +0.14 | no |
| 3 | adaptive | warehouses_built | 0.00 | 0.00 | 0.10 | P minus Sh: -0.10 [-0.17, -0.04] | -0.26 | yes | +0.01 [+0.00, +0.04] | +0.08 | no |
| 3 | adaptive | bought | 18.23 | 15.56 | 19.47 | Sc minus Sh: -3.92 [-4.46, -3.34] | -1.10 | yes | -0.98 [-1.39, -0.57] | -0.39 | yes |
| 3 | adaptive | deliveries | 4.78 | 5.92 | 6.21 | P minus Sc: -1.15 [-1.32, -0.97] | -1.01 | yes | -0.97 [-1.18, -0.73] | -0.62 | yes |
| 3 | adaptive | sold | 0.75 | 6.16 | 10.57 | P minus Sh: -9.83 [-10.25, -9.43] | -4.00 | yes | +3.67 [+3.39, +4.00] | +1.96 | yes |
| 3 | adaptive | produced | 2.31 | 7.31 | 7.28 | P minus Sh: -4.97 [-5.22, -4.70] | -2.75 | yes | +0.77 [+0.56, +0.99] | +0.47 | yes |
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
| 4 | adaptive | factories_built | 0.01 | 0.00 | 0.00 | P minus Sc: +0.01 [+0.00, +0.01] | +0.10 | no | -0.01 [-0.01, +0.00] | -0.10 | no |
| 4 | adaptive | warehouses_built | 0.00 | 0.00 | 0.01 | Sc minus Sh: -0.01 [-0.01, +0.00] | -0.10 | no | -0.00 [-0.01, +0.00] | -0.07 | no |
| 4 | adaptive | bought | 23.45 | 20.61 | 19.41 | P minus Sh: +4.04 [+3.58, +4.53] | +1.04 | yes | -2.44 [-2.93, -1.96] | -0.68 | yes |
| 4 | adaptive | deliveries | 7.09 | 9.36 | 4.76 | Sc minus Sh: +4.59 [+4.42, +4.76] | +3.34 | yes | -1.12 [-1.28, -0.96] | -0.87 | yes |
| 4 | adaptive | sold | 0.51 | 5.17 | 11.32 | P minus Sh: -10.81 [-11.20, -10.42] | -3.98 | yes | +1.11 [+0.92, +1.31] | +0.85 | yes |
| 4 | adaptive | produced | 1.29 | 5.85 | 5.02 | P minus Sh: -3.73 [-3.96, -3.51] | -2.65 | yes | +0.30 [+0.13, +0.47] | +0.25 | yes |
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
| 5 | adaptive | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sh: -0.00 [-0.01, +0.00] | -0.06 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 5 | adaptive | bought | 24.30 | 23.87 | 19.77 | P minus Sh: +4.52 [+4.13, +4.90] | +1.29 | yes | -0.57 [-1.01, -0.16] | -0.16 | no |
| 5 | adaptive | deliveries | 8.11 | 12.60 | 5.38 | Sc minus Sh: +7.21 [+7.02, +7.41] | +4.74 | yes | -0.19 [-0.35, -0.02] | -0.14 | no |
| 5 | adaptive | sold | 0.46 | 3.85 | 9.31 | P minus Sh: -8.85 [-9.19, -8.51] | -3.15 | yes | +0.69 [+0.56, +0.82] | +0.63 | yes |
| 5 | adaptive | produced | 0.85 | 3.36 | 3.69 | P minus Sh: -2.83 [-2.99, -2.67] | -2.30 | yes | +0.12 [+0.00, +0.24] | +0.13 | no |
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
| 3 | factories_built | P minus Sh | -0.13 [-0.19, -0.07] | peddler2 | +0.01 [-0.06, +0.08] | -0.14 [-0.24, -0.05] | -0.40 | yes |
| 3 | factories_built | P minus Sh | -0.13 [-0.19, -0.07] | control_s1 | -0.83 [-0.89, -0.78] | +0.71 [+0.63, +0.78] | +2.02 | yes |
| 3 | warehouses_built | P minus Sh | -0.10 [-0.17, -0.04] | peddler2 | -0.00 [-0.01, +0.00] | -0.10 [-0.17, -0.04] | -0.25 | yes |
| 3 | warehouses_built | P minus Sh | -0.10 [-0.17, -0.04] | control_s1 | -0.01 [-0.04, +0.00] | -0.09 [-0.16, -0.03] | -0.22 | yes |
| 3 | bought | Sc minus Sh | -3.92 [-4.46, -3.34] | peddler2 | -4.50 [-5.17, -3.82] | +0.58 [-0.21, +1.41] | +0.16 | no |
| 3 | bought | Sc minus Sh | -3.92 [-4.46, -3.34] | control_s1 | -5.58 [-5.95, -5.20] | +1.66 [+0.93, +2.37] | +0.47 | yes |
| 3 | deliveries | P minus Sc | -1.15 [-1.32, -0.97] | peddler2 | -1.36 [-1.74, -0.93] | +0.21 [-0.21, +0.60] | +0.18 | no |
| 3 | deliveries | P minus Sc | -1.15 [-1.32, -0.97] | control_s1 | +0.10 [-0.24, +0.44] | -1.25 [-1.64, -0.85] | -1.10 | yes |
| 3 | sold | P minus Sh | -9.83 [-10.25, -9.43] | peddler2 | -12.16 [-12.64, -11.72] | +2.33 [+1.82, +2.88] | +0.95 | yes |
| 3 | sold | P minus Sh | -9.83 [-10.25, -9.43] | control_s1 | -7.94 [-8.34, -7.59] | -1.89 [-2.42, -1.37] | -0.77 | yes |
| 3 | produced | P minus Sh | -4.97 [-5.22, -4.70] | peddler2 | -1.63 [-2.01, -1.25] | -3.34 [-3.80, -2.85] | -1.85 | yes |
| 3 | produced | P minus Sh | -4.97 [-5.22, -4.70] | control_s1 | -5.93 [-6.31, -5.56] | +0.96 [+0.53, +1.40] | +0.53 | yes |
| 4 | factories_built | P minus Sc | +0.01 [+0.00, +0.01] | peddler2 | -0.14 [-0.18, -0.09] | +0.14 [+0.10, +0.18] | +2.81 | yes |
| 4 | factories_built | P minus Sc | +0.01 [+0.00, +0.01] | control_s1 | -0.15 [-0.19, -0.12] | +0.16 [+0.12, +0.20] | +3.21 | yes |
| 4 | warehouses_built | Sc minus Sh | -0.01 [-0.01, +0.00] | peddler2 | +0.00 [+0.00, +0.00] | -0.01 [-0.01, +0.00] | -0.10 | no |
| 4 | warehouses_built | Sc minus Sh | -0.01 [-0.01, +0.00] | control_s1 | +0.00 [+0.00, +0.00] | -0.01 [-0.01, +0.00] | -0.10 | no |
| 4 | bought | P minus Sh | +4.04 [+3.58, +4.53] | peddler2 | +0.58 [+0.13, +1.03] | +3.45 [+2.86, +4.11] | +0.89 | yes |
| 4 | bought | P minus Sh | +4.04 [+3.58, +4.53] | control_s1 | -1.36 [-1.74, -0.96] | +5.40 [+4.80, +5.97] | +1.40 | yes |
| 4 | deliveries | Sc minus Sh | +4.59 [+4.42, +4.76] | peddler2 | +3.61 [+3.29, +3.95] | +0.98 [+0.65, +1.32] | +0.71 | yes |
| 4 | deliveries | Sc minus Sh | +4.59 [+4.42, +4.76] | control_s1 | +5.72 [+5.51, +5.94] | -1.13 [-1.44, -0.86] | -0.82 | yes |
| 4 | sold | P minus Sh | -10.81 [-11.20, -10.42] | peddler2 | -10.11 [-10.62, -9.57] | -0.70 [-1.30, -0.07] | -0.26 | yes |
| 4 | sold | P minus Sh | -10.81 [-11.20, -10.42] | control_s1 | -9.95 [-10.30, -9.57] | -0.86 [-1.35, -0.35] | -0.32 | yes |
| 4 | produced | P minus Sh | -3.73 [-3.96, -3.51] | peddler2 | -3.72 [-4.10, -3.36] | -0.01 [-0.42, +0.41] | -0.01 | no |
| 4 | produced | P minus Sh | -3.73 [-3.96, -3.51] | control_s1 | -3.96 [-4.24, -3.71] | +0.23 [-0.13, +0.65] | +0.17 | no |
| 5 | factories_built | P minus Sc | +0.00 [+0.00, +0.00] | peddler2 | -0.04 [-0.06, -0.03] | +0.04 [+0.03, +0.06] | +0.04 | no |
| 5 | factories_built | P minus Sc | +0.00 [+0.00, +0.00] | control_s1 | -0.35 [-0.39, -0.31] | +0.35 [+0.31, +0.39] | +0.35 | yes |
| 5 | warehouses_built | P minus Sh | -0.00 [-0.01, +0.00] | peddler2 | +0.00 [+0.00, +0.00] | -0.00 [-0.01, +0.00] | -0.06 | no |
| 5 | warehouses_built | P minus Sh | -0.00 [-0.01, +0.00] | control_s1 | +0.00 [+0.00, +0.00] | -0.00 [-0.01, +0.00] | -0.06 | no |
| 5 | bought | P minus Sh | +4.52 [+4.13, +4.90] | peddler2 | +3.47 [+3.20, +3.78] | +1.05 [+0.52, +1.55] | +0.30 | yes |
| 5 | bought | P minus Sh | +4.52 [+4.13, +4.90] | control_s1 | -0.16 [-0.49, +0.17] | +4.69 [+4.15, +5.25] | +1.33 | yes |
| 5 | deliveries | Sc minus Sh | +7.21 [+7.02, +7.41] | peddler2 | +5.38 [+5.18, +5.61] | +1.83 [+1.53, +2.13] | +1.20 | yes |
| 5 | deliveries | Sc minus Sh | +7.21 [+7.02, +7.41] | control_s1 | +6.73 [+6.52, +6.94] | +0.48 [+0.18, +0.79] | +0.32 | yes |
| 5 | sold | P minus Sh | -8.85 [-9.19, -8.51] | peddler2 | -10.19 [-10.65, -9.71] | +1.34 [+0.74, +1.94] | +0.48 | yes |
| 5 | sold | P minus Sh | -8.85 [-9.19, -8.51] | control_s1 | -8.05 [-8.39, -7.72] | -0.80 [-1.28, -0.32] | -0.29 | yes |
| 5 | produced | P minus Sh | -2.83 [-2.99, -2.67] | peddler2 | -5.26 [-5.58, -4.91] | +2.42 [+2.05, +2.77] | +1.97 | yes |
| 5 | produced | P minus Sh | -2.83 [-2.99, -2.67] | control_s1 | -3.41 [-3.62, -3.21] | +0.57 [+0.32, +0.83] | +0.47 | yes |


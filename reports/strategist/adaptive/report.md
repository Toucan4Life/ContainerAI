# Adaptive generalist: one look at HOLDOUT

Net under test: `runs/strategist/adaptive_s1/final.pt` (sha256 `13e277cbd2cd14d6`); references: Peddler v2 and `runs/strategist/control_s1/final.pt`. 400 seeds per composition and table size, the focus seat at every seat in turn, nets played `guarded` under the five table rules. Code: `6e4ad64eee30e5f3a0b7c63f89a5e130785263d2+dirty-86cc696d2e5d`.

Training: 923,012 games in 55 minutes, ladder rating of the final checkpoint 2431.2 (`adaptive_s1`, evaluated by the trainer on its own seeds, not on this block).

## 1. One net seat among the heuristics

The net sits at every seat in turn; the other seats are every composition of Producer (P), Showcase (Sc) and Shipper (Sh), each composition with the same seeds for every net. A stalled game pays nobody (`win`, `margin` count it as 0, the rest leave it out).

### 3 seats (an even split is 0.333)

| net | win share | score | margin | produced | sold | bought | deliveries | factories built | warehouses built | loans | opponents defaulted | stalled |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| adaptive | 0.788 [0.780, 0.795] | 97.6 [97.1, 98.2] | 38.7 [38.0, 39.3] | 9.65 | 7.93 | 11.81 | 6.38 | 0.54 | 0.00 | 0.61 | 2.4% | 0 of 7,200 |
| peddler2 | 0.598 [0.588, 0.609] | 92.2 [91.4, 92.9] | 23.6 [22.8, 24.5] | 12.21 | 9.81 | 10.89 | 7.78 | 1.12 | 0.00 | 0.96 | 3.6% | 6 of 7,200 |
| control_s1 | 0.507 [0.496, 0.517] | 91.0 [90.4, 91.7] | 16.5 [15.6, 17.3] | 8.48 | 4.69 | 6.77 | 8.87 | 0.55 | 0.00 | 0.75 | 2.3% | 3 of 7,200 |

Adaptive minus peddler2, paired on the seeds: win share +0.189 [+0.177, +0.202], score +5.5 [+4.6, +6.3], margin +15.0 [+14.1, +16.0].  
Adaptive minus control_s1, paired on the seeds: win share +0.281 [+0.269, +0.294], score +6.6 [+5.8, +7.3], margin +22.2 [+21.2, +23.1].  

By composition of the opponents (win share of the net in the focus seat):

| opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 |
|---|---|---|---|---|
| P P | 0.301 [0.275, 0.328] | 0.094 | 0.125 | +0.207 [+0.177, +0.237] |
| P Sc | 0.615 [0.590, 0.639] | 0.334 | 0.217 | +0.281 [+0.247, +0.316] |
| P Sh | 0.945 [0.931, 0.957] | 0.711 | 0.596 | +0.233 [+0.207, +0.260] |
| Sc Sc | 0.909 [0.890, 0.925] | 0.749 | 0.565 | +0.160 [+0.130, +0.188] |
| Sc Sh | 0.960 [0.948, 0.971] | 0.799 | 0.659 | +0.161 [+0.134, +0.183] |
| Sh Sh | 0.998 [0.995, 1.000] | 0.903 | 0.879 | +0.094 [+0.079, +0.111] |

### 4 seats (an even split is 0.250)

| net | win share | score | margin | produced | sold | bought | deliveries | factories built | warehouses built | loans | opponents defaulted | stalled |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| adaptive | 0.895 [0.891, 0.900] | 140.8 [140.3, 141.3] | 63.5 [62.9, 64.0] | 6.80 | 7.68 | 15.00 | 8.80 | 0.19 | 0.00 | 1.38 | 3.5% | 0 of 16,000 |
| peddler2 | 0.661 [0.654, 0.667] | 118.7 [118.2, 119.2] | 35.4 [34.8, 35.9] | 13.03 | 11.37 | 11.55 | 8.18 | 0.92 | 0.00 | 0.54 | 2.5% | 0 of 16,000 |
| control_s1 | 0.469 [0.462, 0.476] | 113.3 [112.8, 113.7] | 18.4 [17.9, 18.8] | 6.24 | 5.23 | 9.67 | 9.64 | 0.11 | 0.00 | 0.19 | 1.0% | 0 of 16,000 |

Adaptive minus peddler2, paired on the seeds: win share +0.235 [+0.227, +0.243], score +22.1 [+21.4, +22.7], margin +28.1 [+27.3, +28.8].  
Adaptive minus control_s1, paired on the seeds: win share +0.426 [+0.418, +0.435], score +27.5 [+26.9, +28.2], margin +45.1 [+44.4, +45.8].  

By composition of the opponents (win share of the net in the focus seat):

| opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 |
|---|---|---|---|---|
| P P P | 0.721 [0.698, 0.743] | 0.284 | 0.164 | +0.437 [+0.405, +0.468] |
| P P Sc | 0.812 [0.794, 0.830] | 0.439 | 0.198 | +0.373 [+0.345, +0.402] |
| P P Sh | 0.835 [0.818, 0.852] | 0.466 | 0.254 | +0.369 [+0.343, +0.396] |
| P Sc Sc | 0.857 [0.841, 0.874] | 0.630 | 0.357 | +0.227 [+0.198, +0.255] |
| P Sc Sh | 0.912 [0.898, 0.926] | 0.721 | 0.454 | +0.191 [+0.167, +0.215] |
| P Sh Sh | 0.931 [0.919, 0.943] | 0.634 | 0.610 | +0.296 [+0.272, +0.321] |
| Sc Sc Sc | 0.917 [0.904, 0.929] | 0.838 | 0.539 | +0.079 [+0.057, +0.101] |
| Sc Sc Sh | 0.979 [0.972, 0.986] | 0.937 | 0.637 | +0.042 [+0.030, +0.057] |
| Sc Sh Sh | 0.991 [0.986, 0.996] | 0.897 | 0.679 | +0.094 [+0.081, +0.109] |
| Sh Sh Sh | 0.997 [0.994, 0.999] | 0.760 | 0.796 | +0.237 [+0.216, +0.258] |

### 5 seats (an even split is 0.200)

| net | win share | score | margin | produced | sold | bought | deliveries | factories built | warehouses built | loans | opponents defaulted | stalled |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| adaptive | 0.842 [0.837, 0.845] | 159.6 [159.2, 160.0] | 65.4 [65.0, 65.9] | 6.34 | 8.31 | 15.45 | 9.09 | 0.19 | 0.00 | 1.24 | 3.3% | 1 of 30,000 |
| peddler2 | 0.655 [0.650, 0.660] | 140.1 [139.7, 140.5] | 44.6 [44.2, 45.0] | 12.65 | 11.40 | 12.23 | 8.81 | 0.81 | 0.00 | 0.28 | 2.7% | 0 of 30,000 |
| control_s1 | 0.425 [0.420, 0.431] | 126.4 [126.0, 126.8] | 22.5 [22.1, 22.9] | 5.45 | 4.07 | 8.79 | 9.90 | 0.30 | 0.00 | 0.30 | 1.4% | 0 of 30,000 |

Adaptive minus peddler2, paired on the seeds: win share +0.187 [+0.180, +0.193], score +19.5 [+18.9, +20.1], margin +20.8 [+20.2, +21.5].  
Adaptive minus control_s1, paired on the seeds: win share +0.416 [+0.409, +0.423], score +33.2 [+32.7, +33.8], margin +43.0 [+42.4, +43.5].  

By composition of the opponents (win share of the net in the focus seat):

| opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 |
|---|---|---|---|---|
| P P P P | 0.660 [0.640, 0.681] | 0.342 | 0.185 | +0.318 [+0.291, +0.346] |
| P P P Sc | 0.796 [0.778, 0.812] | 0.565 | 0.264 | +0.231 [+0.204, +0.259] |
| P P P Sh | 0.683 [0.662, 0.703] | 0.390 | 0.194 | +0.293 [+0.263, +0.322] |
| P P Sc Sc | 0.844 [0.830, 0.858] | 0.657 | 0.308 | +0.187 [+0.162, +0.212] |
| P P Sc Sh | 0.808 [0.790, 0.824] | 0.622 | 0.305 | +0.186 [+0.160, +0.210] |
| P P Sh Sh | 0.707 [0.687, 0.728] | 0.391 | 0.234 | +0.316 [+0.287, +0.344] |
| P Sc Sc Sc | 0.867 [0.852, 0.881] | 0.812 | 0.408 | +0.055 [+0.031, +0.078] |
| P Sc Sc Sh | 0.895 [0.880, 0.908] | 0.821 | 0.478 | +0.074 [+0.052, +0.096] |
| P Sc Sh Sh | 0.880 [0.866, 0.894] | 0.702 | 0.472 | +0.179 [+0.155, +0.201] |
| P Sh Sh Sh | 0.873 [0.858, 0.887] | 0.496 | 0.461 | +0.377 [+0.352, +0.403] |
| Sc Sc Sc Sc | 0.865 [0.850, 0.878] | 0.927 | 0.643 | -0.062 [-0.082, -0.044] |
| Sc Sc Sc Sh | 0.932 [0.921, 0.943] | 0.945 | 0.676 | -0.013 [-0.028, +0.002] |
| Sc Sc Sh Sh | 0.944 [0.934, 0.954] | 0.905 | 0.651 | +0.039 [+0.025, +0.054] |
| Sc Sh Sh Sh | 0.936 [0.926, 0.945] | 0.739 | 0.558 | +0.197 [+0.176, +0.219] |
| Sh Sh Sh Sh | 0.933 [0.922, 0.944] | 0.510 | 0.541 | +0.424 [+0.399, +0.450] |

## 2. One net seat among copies of a held-out net

The other seats are all the same net. The generalist never met Peddler v2, Magnate or Consortium in training (Peddler v2 is where it started; the control did train against Peddler v2 and Consortium). Peddler v2 against copies of itself is the check of the harness: an even split.

| seats | opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 | adaptive minus control_s1 | stalled (adaptive, peddler2, control_s1) |
|---|---|---|---|---|---|---|---|
| 3 | peddler2 x2 | 0.855 [0.836, 0.875] | 0.333 [0.333, 0.333] | 0.974 [0.965, 0.983] | +0.522 [+0.503, +0.542] | -0.119 [-0.139, -0.098] | 0, 0, 0 of 1,200 |
| 3 | magnate x2 | 0.604 [0.575, 0.632] | 0.327 [0.302, 0.351] | 0.724 [0.701, 0.748] | +0.277 [+0.241, +0.315] | -0.120 [-0.156, -0.085] | 0, 0, 0 of 1,200 |
| 3 | consortium x2 | 0.538 [0.511, 0.564] | 0.368 [0.343, 0.393] | 0.695 [0.667, 0.722] | +0.170 [+0.134, +0.205] | -0.157 [-0.193, -0.121] | 0, 0, 0 of 1,200 |
| 4 | peddler2 x3 | 0.845 [0.827, 0.862] | 0.250 [0.250, 0.250] | 0.859 [0.842, 0.876] | +0.595 [+0.577, +0.612] | -0.014 [-0.038, +0.009] | 0, 0, 0 of 1,600 |
| 4 | magnate x3 | 0.500 [0.477, 0.524] | 0.528 [0.504, 0.552] | 0.466 [0.442, 0.490] | -0.028 [-0.060, +0.004] | +0.034 [-0.000, +0.069] | 0, 0, 0 of 1,600 |
| 4 | consortium x3 | 0.400 [0.377, 0.424] | 0.333 [0.310, 0.355] | 0.327 [0.306, 0.349] | +0.068 [+0.037, +0.099] | +0.074 [+0.040, +0.103] | 0, 0, 0 of 1,600 |
| 5 | peddler2 x4 | 0.494 [0.473, 0.515] | 0.200 [0.200, 0.200] | 0.602 [0.580, 0.625] | +0.294 [+0.273, +0.315] | -0.108 [-0.139, -0.078] | 0, 0, 0 of 2,000 |
| 5 | magnate x4 | 0.605 [0.585, 0.626] | 0.683 [0.665, 0.702] | 0.681 [0.659, 0.704] | -0.079 [-0.108, -0.050] | -0.076 [-0.108, -0.046] | 0, 0, 0 of 2,000 |
| 5 | consortium x4 | 0.526 [0.502, 0.547] | 0.517 [0.496, 0.539] | 0.544 [0.522, 0.565] | +0.008 [-0.023, +0.036] | -0.019 [-0.047, +0.013] | 0, 0, 0 of 2,000 |

## 3. Does the net's play depend on its opponents?

The net's own counters per game when all the opponents are Producers, all Showcases or all Shippers, and the largest paired difference between two such fields in standard deviations of the seat's per-game value (the convention of `training/egta.py`: a response from 0.2 with the interval excluding 0). Then the smallest change of field: one opponent swapped from a Producer to a Shipper. Any net's counters move with the field, because the field changes what there is to buy, sell and deliver: that is why a reference net is read next to it (and why the flags below mostly say yes).

| seats | net | counter | vs Producers | vs Showcases | vs Shippers | largest difference [95% CI] | in SDs | responds | one Producer swapped for a Shipper [95% CI] | in SDs | responds |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 3 | adaptive | factories_built | 0.14 | 0.82 | 0.78 | P minus Sc: -0.68 [-0.70, -0.65] | -1.83 | yes | +0.17 [+0.15, +0.20] | +0.42 | yes |
| 3 | adaptive | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sc: +0.00 [+0.00, +0.00] | +0.00 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 3 | adaptive | bought | 10.95 | 8.13 | 18.08 | Sc minus Sh: -9.95 [-10.21, -9.70] | -3.07 | yes | +2.73 [+2.54, +2.93] | +0.95 | yes |
| 3 | adaptive | deliveries | 7.45 | 6.43 | 5.27 | P minus Sh: +2.18 [+2.08, +2.29] | +1.55 | yes | -2.43 [-2.53, -2.34] | -1.75 | yes |
| 3 | adaptive | sold | 0.48 | 9.52 | 15.36 | P minus Sh: -14.88 [-15.09, -14.69] | -5.73 | yes | +5.80 [+5.68, +5.93] | +3.43 | yes |
| 3 | adaptive | produced | 3.20 | 15.61 | 11.18 | P minus Sc: -12.41 [-12.65, -12.17] | -3.76 | yes | +1.12 [+0.95, +1.29] | +0.40 | yes |
| 3 | peddler2 | factories_built | 1.15 | 1.13 | 1.12 | P minus Sh: +0.03 [-0.00, +0.07] | +0.06 | no | -0.05 [-0.09, -0.02] | -0.10 | no |
| 3 | peddler2 | warehouses_built | 0.00 | 0.00 | 0.01 | P minus Sh: -0.01 [-0.02, -0.00] | -0.10 | no | +0.00 [+0.00, +0.00] | +0.06 | no |
| 3 | peddler2 | bought | 8.42 | 10.02 | 14.21 | P minus Sh: -5.79 [-6.06, -5.50] | -1.55 | yes | +3.87 [+3.60, +4.15] | +1.08 | yes |
| 3 | peddler2 | deliveries | 7.15 | 8.36 | 7.25 | P minus Sc: -1.21 [-1.41, -1.01] | -0.57 | yes | -0.54 [-0.70, -0.39] | -0.30 | yes |
| 3 | peddler2 | sold | 1.97 | 13.83 | 13.48 | P minus Sc: -11.85 [-12.12, -11.56] | -4.12 | yes | +6.68 [+6.42, +6.92] | +2.01 | yes |
| 3 | peddler2 | produced | 9.02 | 16.73 | 10.36 | P minus Sc: -7.71 [-7.97, -7.42] | -2.45 | yes | -0.20 [-0.40, +0.00] | -0.07 | no |
| 3 | control_s1 | factories_built | 0.11 | 0.73 | 0.93 | P minus Sh: -0.82 [-0.85, -0.79] | -2.15 | yes | +0.15 [+0.13, +0.18] | +0.40 | yes |
| 3 | control_s1 | warehouses_built | 0.00 | 0.00 | 0.02 | P minus Sh: -0.02 [-0.03, -0.01] | -0.10 | no | +0.01 [+0.00, +0.02] | +0.06 | no |
| 3 | control_s1 | bought | 8.79 | 3.19 | 8.75 | P minus Sc: +5.59 [+5.44, +5.75] | +2.68 | yes | +0.96 [+0.81, +1.14] | +0.43 | yes |
| 3 | control_s1 | deliveries | 9.10 | 8.94 | 7.71 | P minus Sh: +1.38 [+1.25, +1.53] | +0.69 | yes | -1.61 [-1.75, -1.46] | -0.80 | yes |
| 3 | control_s1 | sold | 0.48 | 5.23 | 8.51 | P minus Sh: -8.03 [-8.20, -7.86] | -3.61 | yes | +4.33 [+4.19, +4.47] | +2.39 | yes |
| 3 | control_s1 | produced | 3.78 | 11.44 | 9.76 | P minus Sh: -5.99 [-6.18, -5.79] | -2.37 | yes | +1.40 [+1.25, +1.56] | +0.57 | yes |
| 4 | adaptive | factories_built | 0.07 | 0.27 | 0.24 | P minus Sc: -0.20 [-0.22, -0.18] | -0.55 | yes | +0.03 [+0.02, +0.04] | +0.11 | no |
| 4 | adaptive | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sh: -0.00 [-0.01, +0.00] | -0.04 | no | -0.00 [-0.00, +0.00] | -0.04 | no |
| 4 | adaptive | bought | 13.28 | 14.08 | 21.44 | P minus Sh: -8.16 [-8.37, -7.97] | -2.28 | yes | +1.50 [+1.28, +1.73] | +0.44 | yes |
| 4 | adaptive | deliveries | 10.27 | 11.70 | 5.28 | Sc minus Sh: +6.42 [+6.30, +6.53] | +3.77 | yes | -1.91 [-2.01, -1.80] | -1.20 | yes |
| 4 | adaptive | sold | 1.22 | 9.03 | 16.78 | P minus Sh: -15.56 [-15.75, -15.37] | -5.30 | yes | +1.80 [+1.69, +1.90] | +1.10 | yes |
| 4 | adaptive | produced | 2.60 | 11.67 | 6.41 | P minus Sc: -9.07 [-9.29, -8.85] | -2.41 | yes | +0.42 [+0.31, +0.53] | +0.21 | yes |
| 4 | peddler2 | factories_built | 0.86 | 0.99 | 0.89 | P minus Sc: -0.13 [-0.15, -0.11] | -0.30 | yes | -0.01 [-0.03, +0.00] | -0.03 | no |
| 4 | peddler2 | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sc: -0.00 [-0.00, +0.00] | -0.05 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 4 | peddler2 | bought | 12.52 | 11.76 | 11.36 | P minus Sh: +1.15 [+0.93, +1.39] | +0.33 | yes | -0.42 [-0.63, -0.23] | -0.13 | no |
| 4 | peddler2 | deliveries | 9.75 | 9.31 | 5.63 | P minus Sh: +4.12 [+4.02, +4.22] | +2.77 | yes | -1.39 [-1.51, -1.28] | -0.86 | yes |
| 4 | peddler2 | sold | 2.59 | 17.69 | 12.54 | P minus Sc: -15.10 [-15.35, -14.83] | -5.16 | yes | +2.04 [+1.87, +2.21] | +0.78 | yes |
| 4 | peddler2 | produced | 6.41 | 19.79 | 10.27 | P minus Sc: -13.38 [-13.63, -13.12] | -4.07 | yes | +0.60 [+0.45, +0.75] | +0.23 | yes |
| 4 | control_s1 | factories_built | 0.01 | 0.15 | 0.25 | P minus Sh: -0.23 [-0.25, -0.21] | -0.72 | yes | +0.00 [-0.00, +0.01] | +0.04 | no |
| 4 | control_s1 | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sc: +0.00 [+0.00, +0.00] | +0.00 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 4 | control_s1 | bought | 11.05 | 7.80 | 12.59 | Sc minus Sh: -4.79 [-4.98, -4.59] | -1.56 | yes | -0.59 [-0.75, -0.45] | -0.27 | yes |
| 4 | control_s1 | deliveries | 11.48 | 11.67 | 6.01 | Sc minus Sh: +5.66 [+5.54, +5.79] | +3.18 | yes | -1.98 [-2.11, -1.85] | -0.99 | yes |
| 4 | control_s1 | sold | 1.16 | 5.28 | 11.13 | P minus Sh: -9.97 [-10.18, -9.79] | -3.62 | yes | +1.58 [+1.48, +1.69] | +1.01 | yes |
| 4 | control_s1 | produced | 2.72 | 9.65 | 6.51 | P minus Sc: -6.93 [-7.11, -6.78] | -2.74 | yes | +0.48 [+0.37, +0.58] | +0.30 | yes |
| 5 | adaptive | factories_built | 0.11 | 0.23 | 0.27 | P minus Sh: -0.16 [-0.17, -0.15] | -0.42 | yes | +0.01 [+0.00, +0.01] | +0.02 | no |
| 5 | adaptive | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sc: -0.00 [-0.00, +0.00] | -0.04 | no | -0.00 [-0.00, +0.00] | -0.03 | no |
| 5 | adaptive | bought | 13.55 | 16.58 | 21.56 | P minus Sh: -8.00 [-8.20, -7.80] | -2.28 | yes | +0.97 [+0.80, +1.13] | +0.30 | yes |
| 5 | adaptive | deliveries | 10.64 | 12.47 | 5.62 | Sc minus Sh: +6.85 [+6.76, +6.94] | +4.05 | yes | -1.23 [-1.31, -1.15] | -0.83 | yes |
| 5 | adaptive | sold | 2.05 | 12.01 | 17.46 | P minus Sh: -15.41 [-15.58, -15.25] | -5.15 | yes | +1.47 [+1.37, +1.57] | +0.88 | yes |
| 5 | adaptive | produced | 3.01 | 11.85 | 5.37 | P minus Sc: -8.84 [-9.03, -8.66] | -2.05 | yes | +0.38 [+0.30, +0.46] | +0.18 | no |
| 5 | peddler2 | factories_built | 0.77 | 0.83 | 0.81 | P minus Sc: -0.06 [-0.07, -0.05] | -0.14 | no | +0.02 [+0.01, +0.03] | +0.04 | no |
| 5 | peddler2 | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sc: +0.00 [+0.00, +0.00] | +0.00 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 5 | peddler2 | bought | 14.72 | 12.48 | 11.29 | P minus Sh: +3.43 [+3.26, +3.61] | +1.17 | yes | -1.24 [-1.43, -1.06] | -0.44 | yes |
| 5 | peddler2 | deliveries | 11.07 | 10.81 | 5.57 | P minus Sh: +5.50 [+5.42, +5.58] | +3.94 | yes | -1.40 [-1.49, -1.31] | -0.87 | yes |
| 5 | peddler2 | sold | 2.75 | 21.45 | 12.52 | P minus Sc: -18.70 [-18.85, -18.55] | -5.37 | yes | +1.12 [+0.99, +1.25] | +0.48 | yes |
| 5 | peddler2 | produced | 5.44 | 22.99 | 10.36 | P minus Sc: -17.55 [-17.71, -17.40] | -4.64 | yes | +0.63 [+0.52, +0.75] | +0.27 | yes |
| 5 | control_s1 | factories_built | 0.05 | 0.40 | 0.46 | P minus Sh: -0.42 [-0.44, -0.40] | -1.07 | yes | +0.02 [+0.01, +0.02] | +0.07 | no |
| 5 | control_s1 | warehouses_built | 0.00 | 0.00 | 0.00 | P minus Sc: +0.00 [+0.00, +0.00] | +0.00 | no | +0.00 [+0.00, +0.00] | +0.00 | no |
| 5 | control_s1 | bought | 10.41 | 8.81 | 10.68 | P minus Sc: +1.60 [+1.48, +1.71] | +0.72 | yes | -0.69 [-0.80, -0.59] | -0.35 | yes |
| 5 | control_s1 | deliveries | 11.87 | 13.01 | 6.23 | Sc minus Sh: +6.78 [+6.67, +6.88] | +3.99 | yes | -1.30 [-1.41, -1.19] | -0.69 | yes |
| 5 | control_s1 | sold | 1.47 | 4.13 | 9.58 | P minus Sh: -8.11 [-8.26, -7.96] | -3.12 | yes | +0.88 [+0.80, +0.96] | +0.68 | yes |
| 5 | control_s1 | produced | 2.77 | 8.06 | 6.11 | P minus Sc: -5.29 [-5.41, -5.17] | -2.33 | yes | +0.17 [+0.08, +0.25] | +0.12 | no |

### Against the references

For the pair of fields where the generalist's own response is largest, the same difference for a reference net, and the difference between the two responses (a difference in differences, paired on the seeds) in the same SDs: did training against the heuristics change how the net answers its field? Peddler v2 is where the generalist started; `control_s1` is the same recipe on the older pool without the heuristics.

| seats | counter | fields | adaptive [95% CI] | reference | reference [95% CI] | adaptive minus reference [95% CI] | in SDs | answers differently |
|---|---|---|---|---|---|---|---|---|
| 3 | factories_built | P minus Sc | -0.68 [-0.70, -0.65] | peddler2 | +0.02 [-0.01, +0.06] | -0.70 [-0.74, -0.66] | -1.89 | yes |
| 3 | factories_built | P minus Sc | -0.68 [-0.70, -0.65] | control_s1 | -0.62 [-0.65, -0.59] | -0.06 [-0.10, -0.02] | -0.15 | no |
| 3 | warehouses_built | P minus Sc | +0.00 [+0.00, +0.00] | peddler2 | +0.00 [+0.00, +0.00] | +0.00 [+0.00, +0.00] | +0.00 | no |
| 3 | warehouses_built | P minus Sc | +0.00 [+0.00, +0.00] | control_s1 | +0.00 [+0.00, +0.00] | +0.00 [+0.00, +0.00] | +0.00 | no |
| 3 | bought | Sc minus Sh | -9.95 [-10.21, -9.70] | peddler2 | -4.19 [-4.54, -3.81] | -5.76 [-6.23, -5.31] | -1.78 | yes |
| 3 | bought | Sc minus Sh | -9.95 [-10.21, -9.70] | control_s1 | -5.56 [-5.75, -5.37] | -4.39 [-4.72, -4.06] | -1.36 | yes |
| 3 | deliveries | P minus Sh | +2.18 [+2.08, +2.29] | peddler2 | -0.11 [-0.26, +0.05] | +2.29 [+2.08, +2.47] | +1.62 | yes |
| 3 | deliveries | P minus Sh | +2.18 [+2.08, +2.29] | control_s1 | +1.38 [+1.25, +1.53] | +0.80 [+0.61, +0.98] | +0.57 | yes |
| 3 | sold | P minus Sh | -14.88 [-15.09, -14.69] | peddler2 | -11.51 [-11.79, -11.23] | -3.37 [-3.71, -3.04] | -1.30 | yes |
| 3 | sold | P minus Sh | -14.88 [-15.09, -14.69] | control_s1 | -8.03 [-8.20, -7.86] | -6.86 [-7.11, -6.61] | -2.64 | yes |
| 3 | produced | P minus Sc | -12.41 [-12.65, -12.17] | peddler2 | -7.71 [-7.97, -7.42] | -4.70 [-5.08, -4.35] | -1.43 | yes |
| 3 | produced | P minus Sc | -12.41 [-12.65, -12.17] | control_s1 | -7.66 [-7.93, -7.37] | -4.75 [-5.16, -4.35] | -1.44 | yes |
| 4 | factories_built | P minus Sc | -0.20 [-0.22, -0.18] | peddler2 | -0.13 [-0.15, -0.11] | -0.07 [-0.10, -0.04] | -0.18 | no |
| 4 | factories_built | P minus Sc | -0.20 [-0.22, -0.18] | control_s1 | -0.14 [-0.15, -0.12] | -0.06 [-0.09, -0.03] | -0.18 | no |
| 4 | warehouses_built | P minus Sh | -0.00 [-0.01, +0.00] | peddler2 | +0.00 [+0.00, +0.00] | -0.00 [-0.01, +0.00] | -0.04 | no |
| 4 | warehouses_built | P minus Sh | -0.00 [-0.01, +0.00] | control_s1 | +0.00 [+0.00, +0.00] | -0.00 [-0.01, +0.00] | -0.04 | no |
| 4 | bought | P minus Sh | -8.16 [-8.37, -7.97] | peddler2 | +1.15 [+0.93, +1.39] | -9.31 [-9.62, -9.02] | -2.60 | yes |
| 4 | bought | P minus Sh | -8.16 [-8.37, -7.97] | control_s1 | -1.54 [-1.76, -1.35] | -6.61 [-6.91, -6.32] | -1.85 | yes |
| 4 | deliveries | Sc minus Sh | +6.42 [+6.30, +6.53] | peddler2 | +3.68 [+3.51, +3.85] | +2.74 [+2.54, +2.93] | +1.60 | yes |
| 4 | deliveries | Sc minus Sh | +6.42 [+6.30, +6.53] | control_s1 | +5.66 [+5.54, +5.79] | +0.76 [+0.61, +0.90] | +0.45 | yes |
| 4 | sold | P minus Sh | -15.56 [-15.75, -15.37] | peddler2 | -9.95 [-10.17, -9.72] | -5.61 [-5.92, -5.34] | -1.91 | yes |
| 4 | sold | P minus Sh | -15.56 [-15.75, -15.37] | control_s1 | -9.97 [-10.18, -9.79] | -5.59 [-5.83, -5.32] | -1.91 | yes |
| 4 | produced | P minus Sc | -9.07 [-9.29, -8.85] | peddler2 | -13.38 [-13.63, -13.12] | +4.31 [+3.96, +4.66] | +1.15 | yes |
| 4 | produced | P minus Sc | -9.07 [-9.29, -8.85] | control_s1 | -6.93 [-7.11, -6.78] | -2.13 [-2.42, -1.83] | -0.57 | yes |
| 5 | factories_built | P minus Sh | -0.16 [-0.17, -0.15] | peddler2 | -0.03 [-0.04, -0.02] | -0.13 [-0.14, -0.11] | -0.33 | yes |
| 5 | factories_built | P minus Sh | -0.16 [-0.17, -0.15] | control_s1 | -0.42 [-0.44, -0.40] | +0.26 [+0.23, +0.28] | +0.67 | yes |
| 5 | warehouses_built | P minus Sc | -0.00 [-0.00, +0.00] | peddler2 | +0.00 [+0.00, +0.00] | -0.00 [-0.00, +0.00] | -0.04 | no |
| 5 | warehouses_built | P minus Sc | -0.00 [-0.00, +0.00] | control_s1 | +0.00 [+0.00, +0.00] | -0.00 [-0.00, +0.00] | -0.04 | no |
| 5 | bought | P minus Sh | -8.00 [-8.20, -7.80] | peddler2 | +3.43 [+3.26, +3.61] | -11.43 [-11.70, -11.19] | -3.26 | yes |
| 5 | bought | P minus Sh | -8.00 [-8.20, -7.80] | control_s1 | -0.26 [-0.42, -0.11] | -7.74 [-7.99, -7.48] | -2.21 | yes |
| 5 | deliveries | Sc minus Sh | +6.85 [+6.76, +6.94] | peddler2 | +5.24 [+5.12, +5.36] | +1.61 [+1.46, +1.77] | +0.95 | yes |
| 5 | deliveries | Sc minus Sh | +6.85 [+6.76, +6.94] | control_s1 | +6.78 [+6.67, +6.88] | +0.07 [-0.07, +0.21] | +0.04 | no |
| 5 | sold | P minus Sh | -15.41 [-15.58, -15.25] | peddler2 | -9.77 [-9.98, -9.57] | -5.64 [-5.89, -5.39] | -1.88 | yes |
| 5 | sold | P minus Sh | -15.41 [-15.58, -15.25] | control_s1 | -8.11 [-8.26, -7.96] | -7.30 [-7.53, -7.08] | -2.44 | yes |
| 5 | produced | P minus Sc | -8.84 [-9.03, -8.66] | peddler2 | -17.55 [-17.71, -17.40] | +8.71 [+8.46, +8.96] | +2.02 | yes |
| 5 | produced | P minus Sc | -8.84 [-9.03, -8.66] | control_s1 | -5.29 [-5.41, -5.17] | -3.55 [-3.78, -3.31] | -0.82 | yes |


# Ladder ratings of the scripted bots (3000 games per bot and ladder, `DEV` seeds)

Rating on the ladder of `training/ladder.json` (Random = 0, Simple 263, Merchant 932, the anchor nets 1907 to 2286), 95% bootstrap interval over games. The last four rows are the check: their listed ratings are in the last column (Solitaire's 1200 is its scripted-ladder rating).

| bot | full ladder | scripted ladder (Random, Simple, Merchant) | stalled games (full / scripted) | listed |
|---|---|---|---|---|
| producer | 2033 [2016, 2049] | 1509 [1483, 1534] | 0 / 0 of 3000 |  |
| showcase | 1911 [1889, 1925] | 1451 [1428, 1475] | 1 / 4 of 3000 |  |
| shipper | 1860 [1840, 1882] | 1470 [1448, 1496] | 4 / 6 of 3000 |  |
| random | 6 [-11, 26] | -41 [-54, -31] | 102 / 3 of 3000 | 0 |
| simple | 300 [286, 318] | 290 [280, 300] | 8 / 0 of 3000 | 263 |
| merchant | 934 [923, 944] | 934 [924, 945] | 0 / 0 of 3000 | 932 |
| solitaire | 1006 [991, 1020] | 1178 [1165, 1193] | 11 / 0 of 3000 | 1200 |

Share of the duels each bot wins against each member of the full ladder (the rated bot against that opponent only):

| bot | random | simple | merchant | traders | run1 | exploit_run2 | run2 | consortium |
|---|---|---|---|---|---|---|---|---|
| producer | 1.00 | 1.00 | 0.97 | 0.58 | 0.35 | 0.26 | 0.38 | 0.29 |
| showcase | 1.00 | 1.00 | 0.96 | 0.47 | 0.21 | 0.17 | 0.20 | 0.23 |
| shipper | 1.00 | 1.00 | 0.96 | 0.46 | 0.18 | 0.11 | 0.16 | 0.16 |
| random | 0.51 | 0.08 | 0.02 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| simple | 0.92 | 0.53 | 0.02 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| merchant | 1.00 | 0.99 | 0.50 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| solitaire | 0.97 | 0.94 | 0.59 | 0.09 | 0.02 | 0.02 | 0.01 | 0.02 |

Share of the duels each bot wins against each member of the scripted ladder (the rated bot against that opponent only):

| bot | random | simple | merchant |
|---|---|---|---|
| producer | 1.00 | 1.00 | 0.97 |
| showcase | 1.00 | 1.00 | 0.95 |
| shipper | 1.00 | 1.00 | 0.96 |
| random | 0.50 | 0.05 | 0.02 |
| simple | 0.95 | 0.49 | 0.00 |
| merchant | 1.00 | 1.00 | 0.49 |
| solitaire | 1.00 | 1.00 | 0.81 |

# Actions of the adaptive nets, the references and the heuristics (4 seats, 100 DEV seeds)

Each net sits at every seat in turn; the other seats are Producer, Showcase and Shipper (a table of 3 has no Shipper, one of 5 has two Producers). The three heuristic columns are their seats in the games of `warm`. Every number is per game and seat. Every player starts with 1 factory and 1 warehouse.

**Result, building and shipping, per game**

| | warm | scratch_s1 | scratch_s2 | peddler2 | control_s1 | producer | showcase | shipper |
|---|---|---|---|---|---|---|---|---|
| win share at this table (an even split is 0.25) | 0.91 | 0.89 | 0.89 | 0.73 | 0.43 | 0.04 | 0.03 | 0.03 |
| score | 129 | 130 | 109 | 123 | 98 | 79 | 70 | 66 |
| containers produced | 6.1 | 4.6 | 2.5 | 14.5 | 5.5 | 24.5 | 7.2 | 4.3 |
| containers bought | 13.3 | 12.1 | 21.6 | 11.3 | 9.0 | 13.6 | 25.6 | 24.1 |
| containers sold | 5.8 | 3.7 | 3.4 | 12.2 | 4.2 | 26.6 | 27.4 | 16.8 |
| factories built | 0.20 | 0 | <0.005 | 0.94 | 0.09 | 1.00 | 0 | 0.99 |
| warehouses built (harbor capacity) | 0 | 0 | 0 | <0.005 | 0 | 1.24 | 3.03 | 2.11 |
| factories owned, mean over turns | 1.19 | 1.00 | 1.00 | 1.71 | 1.07 | 1.98 | 1.00 | 1.94 |
| warehouses owned, mean over turns | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 2.16 | 3.84 | 3.05 |
| deliveries (sails to the Island) | 8.4 | 7.3 | 5.5 | 8.2 | 8.7 | 10.2 | 5.2 | 11.0 |
| containers per delivery | 1.51 | 2.08 | 3.01 | 1.49 | 1.03 | 1.06 | 1.11 | 1.06 |

**Share of the trips to the Island by number of containers on the ship (a ship holds 5)**

| | warm | scratch_s1 | scratch_s2 | peddler2 | control_s1 | producer | showcase | shipper |
|---|---|---|---|---|---|---|---|---|
| 1 | 64% | 46% | 25% | 65% | 97% | 94% | 89% | 95% |
| 2 | 25% | 26% | 19% | 24% | 3% | 6% | 11% | 4% |
| 3 | 8% | 12% | 16% | 9% | 0% | 0% | 0% | 1% |
| 4 | 2% | 6% | 9% | 2% | 0% | 0% | 0% | 0% |
| 5 | 1% | 10% | 31% | 0% | 0% | 0% | 0% | 0% |

**Moves per game: main action: making, building and buying**

| | warm | scratch_s1 | scratch_s2 | peddler2 | control_s1 | producer | showcase | shipper |
|---|---|---|---|---|---|---|---|---|
| Produce | 5.6 | 4.7 | 2.6 | 11.0 | 5.6 | 12.7 | 7.2 | 2.3 |
| Build factory | 0.2 | 0 | <0.05 | 0.9 | 0.1 | 1.0 | 0 | 1.0 |
| Build warehouse | 0 | 0 | 0 | <0.05 | 0 | 1.2 | 3.0 | 2.1 |
| Start buying at a factory | 5.6 | 4.7 | 4.8 | 3.2 | 3.2 | 2.2 | 13.5 | 7.4 |
| Start buying at a harbor | 0 | 1.3 | 3.6 | 0 | 0 | <0.05 | 0.1 | 0 |
| Start loading at the Bank | 0 | 1.4 | 0.2 | <0.05 | 0 | 0 | 0 | 0 |
| Reprice factory | 0.3 | 0.5 | 0.1 | 0.6 | 0.5 | 1.0 | 12.2 | 0.8 |
| Reprice harbor | 0.5 | 0.5 | 0.2 | 0.2 | 0.7 | 0.1 | 0.5 | 0 |
| Pass | 0.1 | 0.5 | 0 | 0.1 | 0.3 | 1.3 | 1.0 | 1.5 |

**Moves per game: main action: ship, bank and loans**

| | warm | scratch_s1 | scratch_s2 | peddler2 | control_s1 | producer | showcase | shipper |
|---|---|---|---|---|---|---|---|---|
| Sail to ocean | 16.9 | 14.5 | 11.0 | 16.4 | 17.4 | 19.6 | 10.1 | 21.6 |
| Sail to Bank | 3.4 | 5.2 | 0.3 | 4.2 | 3.7 | 0 | 0 | 0 |
| Sail to a harbor | 5.7 | 2.6 | 5.8 | 4.6 | 5.7 | 10.3 | 5.6 | 11.5 |
| Sail to Island | 8.4 | 7.3 | 5.5 | 8.2 | 8.7 | 10.2 | 5.2 | 11.0 |
| Call Bank: container lot | 4.6 | 8.0 | 6.2 | 7.0 | 5.9 | 0 | 0 | 0 |
| Call Bank: cash lot | 5.4 | 6.0 | 4.2 | 5.1 | 4.3 | 0 | 0 | 0 |
| Outbid: container lot | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Outbid: cash lot | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Request loan | 1.0 | <0.05 | 0 | 0.5 | 0.1 | 0.3 | 0 | 1.2 |
| Repay loan | 0.9 | <0.05 | 0 | 0.5 | 0.1 | 0.2 | 0 | 1.0 |

**Moves per game: inside an action: producing, buying, loading and laying out**

| | warm | scratch_s1 | scratch_s2 | peddler2 | control_s1 | producer | showcase | shipper |
|---|---|---|---|---|---|---|---|---|
| Pick a colour to produce | 0.1 | 0 | 0 | 0.5 | <0.05 | <0.05 | 0 | <0.05 |
| Take a container, factory lot | 5.6 | 4.7 | 4.8 | 3.2 | 3.2 | 2.6 | 19.5 | 11.8 |
| Take a container, harbor lot | 7.7 | 7.5 | 16.8 | 8.2 | 5.8 | 10.9 | 6.1 | 12.3 |
| Done buying | 1.3 | 0 | 0 | 0.2 | 3.4 | 5.6 | 1.7 | 7.5 |
| Load a container | 5.8 | 8.6 | 1.0 | 4.4 | 3.5 | 0 | 0 | 0 |
| Done loading | 0.4 | 0.3 | 0 | 1.5 | 2.8 | 0 | 0 | 0 |
| Place a container in a lot | 15.7 | 11.5 | 8.3 | 27.8 | 13.2 | 32.5 | 72.6 | 20.1 |

**Moves per game: inside an action: auctions and defaults**

| | warm | scratch_s1 | scratch_s2 | peddler2 | control_s1 | producer | showcase | shipper |
|---|---|---|---|---|---|---|---|---|
| Delivery bid | 36.8 | 35.9 | 24.7 | 39.2 | 36.1 | 30.7 | 36.6 | 28.1 |
| Accept a bid | 8.4 | 7.3 | 5.5 | 8.2 | 8.7 | 9.7 | 5.0 | 10.3 |
| Buy out | 0 | 0 | 0 | 0 | 0 | 0.5 | 0.2 | 0.7 |
| Bank bid: cash | 4.6 | 8.0 | 6.2 | 7.0 | 5.9 | 0 | 0 | 0 |
| Bank bid: container | 5.4 | 6.0 | 4.2 | 5.1 | 4.4 | 0 | 0 | 0 |
| Done Bank bid | 5.4 | 6.0 | 4.2 | 5.1 | 4.3 | 0 | 0 | 0 |
| Seize | 0.3 | 0.2 | 0.2 | 0.1 | 0.1 | 0 | 0.2 | <0.05 |
| Deal | 0.1 | <0.05 | <0.05 | <0.05 | <0.05 | 0 | <0.05 | <0.05 |

## Histogram of each move

One chart per move, so the bars compare the columns for that move only.

**Main action: making, building and buying**

```
Produce  (bars to 12.7 a game)
  warm       ████████████                 5.6
  scratch_s1 ██████████                   4.7
  scratch_s2 ██████                       2.6
  peddler2   ████████████████████████     11.0
  control_s1 ████████████                 5.6
  producer   ████████████████████████████ 12.7
  showcase   ████████████████             7.2
  shipper    █████                        2.3

Build factory  (bars to 1.0 a game)
  warm       ██████                       0.2
  scratch_s1                              0
  scratch_s2                              <0.05
  peddler2   ██████████████████████████   0.9
  control_s1 ███                          0.1
  producer   ████████████████████████████ 1.0
  showcase                                0
  shipper    ████████████████████████████ 1.0

Build warehouse  (bars to 3.0 a game)
  warm                                    0
  scratch_s1                              0
  scratch_s2                              0
  peddler2                                <0.05
  control_s1                              0
  producer   ███████████                  1.2
  showcase   ████████████████████████████ 3.0
  shipper    ███████████████████          2.1

Start buying at a factory  (bars to 13.5 a game)
  warm       ████████████                 5.6
  scratch_s1 ██████████                   4.7
  scratch_s2 ██████████                   4.8
  peddler2   ███████                      3.2
  control_s1 ███████                      3.2
  producer   █████                        2.2
  showcase   ████████████████████████████ 13.5
  shipper    ███████████████              7.4

Start buying at a harbor  (bars to 3.6 a game)
  warm                                    0
  scratch_s1 ██████████                   1.3
  scratch_s2 ████████████████████████████ 3.6
  peddler2                                0
  control_s1                              0
  producer                                <0.05
  showcase   █                            0.1
  shipper                                 0

Start loading at the Bank  (bars to 1.4 a game)
  warm                                    0
  scratch_s1 ████████████████████████████ 1.4
  scratch_s2 ████                         0.2
  peddler2                                <0.05
  control_s1                              0
  producer                                0
  showcase                                0
  shipper                                 0

Reprice factory  (bars to 12.2 a game)
  warm       █                            0.3
  scratch_s1 █                            0.5
  scratch_s2 █                            0.1
  peddler2   █                            0.6
  control_s1 █                            0.5
  producer   ██                           1.0
  showcase   ████████████████████████████ 12.2
  shipper    ██                           0.8

Reprice harbor  (bars to 0.7 a game)
  warm       █████████████████            0.5
  scratch_s1 ██████████████████           0.5
  scratch_s2 ███████                      0.2
  peddler2   ███████                      0.2
  control_s1 ████████████████████████████ 0.7
  producer   █████                        0.1
  showcase   ██████████████████           0.5
  shipper                                 0

Pass  (bars to 1.5 a game)
  warm       ███                          0.1
  scratch_s1 ██████████                   0.5
  scratch_s2                              0
  peddler2   █                            0.1
  control_s1 █████                        0.3
  producer   █████████████████████████    1.3
  showcase   ██████████████████           1.0
  shipper    ████████████████████████████ 1.5

```

**Main action: ship, Bank and loans**

```
Sail to ocean  (bars to 21.6 a game)
  warm       ██████████████████████       16.9
  scratch_s1 ███████████████████          14.5
  scratch_s2 ██████████████               11.0
  peddler2   █████████████████████        16.4
  control_s1 ███████████████████████      17.4
  producer   █████████████████████████    19.6
  showcase   █████████████                10.1
  shipper    ████████████████████████████ 21.6

Sail to Bank  (bars to 5.2 a game)
  warm       ██████████████████           3.4
  scratch_s1 ████████████████████████████ 5.2
  scratch_s2 █                            0.3
  peddler2   ███████████████████████      4.2
  control_s1 ████████████████████         3.7
  producer                                0
  showcase                                0
  shipper                                 0

Sail to a harbor  (bars to 11.5 a game)
  warm       ██████████████               5.7
  scratch_s1 ██████                       2.6
  scratch_s2 ██████████████               5.8
  peddler2   ███████████                  4.6
  control_s1 ██████████████               5.7
  producer   █████████████████████████    10.3
  showcase   ██████████████               5.6
  shipper    ████████████████████████████ 11.5

Sail to Island  (bars to 11.0 a game)
  warm       █████████████████████        8.4
  scratch_s1 ███████████████████          7.3
  scratch_s2 ██████████████               5.5
  peddler2   █████████████████████        8.2
  control_s1 ██████████████████████       8.7
  producer   ██████████████████████████   10.2
  showcase   █████████████                5.2
  shipper    ████████████████████████████ 11.0

Call Bank: container lot  (bars to 8.0 a game)
  warm       ████████████████             4.6
  scratch_s1 ████████████████████████████ 8.0
  scratch_s2 ██████████████████████       6.2
  peddler2   █████████████████████████    7.0
  control_s1 █████████████████████        5.9
  producer                                0
  showcase                                0
  shipper                                 0

Call Bank: cash lot  (bars to 6.0 a game)
  warm       █████████████████████████    5.4
  scratch_s1 ████████████████████████████ 6.0
  scratch_s2 ████████████████████         4.2
  peddler2   ████████████████████████     5.1
  control_s1 ████████████████████         4.3
  producer                                0
  showcase                                0
  shipper                                 0

Request loan  (bars to 1.2 a game)
  warm       ███████████████████████      1.0
  scratch_s1                              <0.05
  scratch_s2                              0
  peddler2   ████████████                 0.5
  control_s1 ███                          0.1
  producer   ████████                     0.3
  showcase                                0
  shipper    ████████████████████████████ 1.2

Repay loan  (bars to 1.0 a game)
  warm       █████████████████████████    0.9
  scratch_s1                              <0.05
  scratch_s2                              0
  peddler2   ██████████████               0.5
  control_s1 ████                         0.1
  producer   ███████                      0.2
  showcase                                0
  shipper    ████████████████████████████ 1.0

```
Not played 0.05 times a game by any column: Outbid: container lot, Outbid: cash lot.

**Inside an action: producing, buying, loading and laying out**

```
Pick a colour to produce  (bars to 0.5 a game)
  warm       █████                        0.1
  scratch_s1                              0
  scratch_s2                              0
  peddler2   ████████████████████████████ 0.5
  control_s1                              <0.05
  producer                                <0.05
  showcase                                0
  shipper                                 <0.05

Take a container, factory lot  (bars to 19.5 a game)
  warm       ████████                     5.6
  scratch_s1 ███████                      4.7
  scratch_s2 ███████                      4.8
  peddler2   █████                        3.2
  control_s1 █████                        3.2
  producer   ████                         2.6
  showcase   ████████████████████████████ 19.5
  shipper    █████████████████            11.8

Take a container, harbor lot  (bars to 16.8 a game)
  warm       █████████████                7.7
  scratch_s1 ████████████                 7.5
  scratch_s2 ████████████████████████████ 16.8
  peddler2   ██████████████               8.2
  control_s1 ██████████                   5.8
  producer   ██████████████████           10.9
  showcase   ██████████                   6.1
  shipper    ████████████████████         12.3

Done buying  (bars to 7.5 a game)
  warm       █████                        1.3
  scratch_s1                              0
  scratch_s2                              0
  peddler2   █                            0.2
  control_s1 █████████████                3.4
  producer   █████████████████████        5.6
  showcase   ██████                       1.7
  shipper    ████████████████████████████ 7.5

Load a container  (bars to 8.6 a game)
  warm       ███████████████████          5.8
  scratch_s1 ████████████████████████████ 8.6
  scratch_s2 ███                          1.0
  peddler2   ██████████████               4.4
  control_s1 ███████████                  3.5
  producer                                0
  showcase                                0
  shipper                                 0

Done loading  (bars to 2.8 a game)
  warm       ████                         0.4
  scratch_s1 ███                          0.3
  scratch_s2                              0
  peddler2   ███████████████              1.5
  control_s1 ████████████████████████████ 2.8
  producer                                0
  showcase                                0
  shipper                                 0

Place a container in a lot  (bars to 72.6 a game)
  warm       ██████                       15.7
  scratch_s1 ████                         11.5
  scratch_s2 ███                          8.3
  peddler2   ███████████                  27.8
  control_s1 █████                        13.2
  producer   █████████████                32.5
  showcase   ████████████████████████████ 72.6
  shipper    ████████                     20.1

```

**Inside an action: auctions and defaults**

```
Delivery bid  (bars to 39.2 a game)
  warm       ██████████████████████████   36.8
  scratch_s1 ██████████████████████████   35.9
  scratch_s2 ██████████████████           24.7
  peddler2   ████████████████████████████ 39.2
  control_s1 ██████████████████████████   36.1
  producer   ██████████████████████       30.7
  showcase   ██████████████████████████   36.6
  shipper    ████████████████████         28.1

Accept a bid  (bars to 10.3 a game)
  warm       ███████████████████████      8.4
  scratch_s1 ████████████████████         7.3
  scratch_s2 ███████████████              5.5
  peddler2   ██████████████████████       8.2
  control_s1 ███████████████████████      8.7
  producer   ██████████████████████████   9.7
  showcase   █████████████                5.0
  shipper    ████████████████████████████ 10.3

Buy out  (bars to 0.7 a game)
  warm                                    0
  scratch_s1                              0
  scratch_s2                              0
  peddler2                                0
  control_s1                              0
  producer   ████████████████████         0.5
  showcase   ██████████                   0.2
  shipper    ████████████████████████████ 0.7

Bank bid: cash  (bars to 8.0 a game)
  warm       ████████████████             4.6
  scratch_s1 ████████████████████████████ 8.0
  scratch_s2 ██████████████████████       6.2
  peddler2   █████████████████████████    7.0
  control_s1 █████████████████████        5.9
  producer                                0
  showcase                                0
  shipper                                 0

Bank bid: container  (bars to 6.0 a game)
  warm       █████████████████████████    5.4
  scratch_s1 ████████████████████████████ 6.0
  scratch_s2 ████████████████████         4.2
  peddler2   ████████████████████████     5.1
  control_s1 ████████████████████         4.4
  producer                                0
  showcase                                0
  shipper                                 0

Done Bank bid  (bars to 6.0 a game)
  warm       █████████████████████████    5.4
  scratch_s1 ████████████████████████████ 6.0
  scratch_s2 ████████████████████         4.2
  peddler2   ████████████████████████     5.1
  control_s1 ████████████████████         4.3
  producer                                0
  showcase                                0
  shipper                                 0

Seize  (bars to 0.3 a game)
  warm       ████████████████████████████ 0.3
  scratch_s1 █████████████████████        0.2
  scratch_s2 █████████████████████        0.2
  peddler2   █████████████                0.1
  control_s1 █████                        0.1
  producer                                0
  showcase   ██████████████████           0.2
  shipper                                 <0.05

Deal  (bars to 0.1 a game)
  warm       ████████████████████████████ 0.1
  scratch_s1                              <0.05
  scratch_s2                              <0.05
  peddler2                                <0.05
  control_s1                              <0.05
  producer                                0
  showcase                                <0.05
  shipper                                 <0.05

```

Finished seat-games per column: warm 400, scratch_s1 400, scratch_s2 400, peddler2 400, control_s1 400, producer 400, showcase 400, shipper 400. Stalled games, left out: warm 0, scratch_s1 0, scratch_s2 0, peddler2 0, control_s1 0.

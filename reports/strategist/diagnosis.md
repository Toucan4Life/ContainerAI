# Diagnosis after Run A (evaluation only; no training, no reward change)

Tool: `python -m training.diagnose peddler2 --games 200 --compose current=... --compose complementary=...` (800 games per table, focus net seated at every seat, same seeds). Seat 0 of the current table is the focus Peddler v2; the second `peddler2` row there is the table's own Peddler v2.

## 1 and 2. Tables, winners, shelves

**Peddler v2 at the current table** (fair share 0.25)

| entry | win share | mean score | stock per decision | cheap share of stock | sold/game | bought/game | price paid | factories held | mean lot price laid out |
|---|---|---|---|---|---|---|---|---|---|
| peddler2 (focus) | 0.500 | 94.8 | 2.64 | 0.80 | 17.3 | 14.6 | 4.15 | 1.85 | 3.41 |
| peddler2 | 0.316 | 88.3 | 2.77 | 0.81 | 16.3 | 14.9 | 4.26 | 1.85 | 3.42 |
| consortium | 0.064 | 71.2 | 1.45 | 0.01 | 13.2 | 15.9 | 4.48 | 1.00 | 4.38 |
| magnate | 0.121 | 78.7 | 1.46 | 0.03 | 15.8 | 17.2 | 4.05 | 1.00 | 4.35 |

**Peddler v2 at the complementary table**

| entry | win share | mean score | stock per decision | cheap share of stock | sold/game | bought/game | price paid | factories held | mean lot price laid out |
|---|---|---|---|---|---|---|---|---|---|
| peddler2 (focus) | 0.068 | 52.7 | 3.60 | 0.77 | 10.5 | 13.0 | 4.78 | 1.88 | 3.27 |
| producer_s2 | 0.267 | 69.6 | 2.93 | 0.67 | 12.3 | 12.1 | 4.13 | 2.15 | 3.43 |
| showcase_fair_s2 | 0.209 | 66.0 | 3.54 | 0.74 | 12.4 | 10.9 | 3.91 | 2.46 | 3.08 |
| control_s2 | 0.456 | 77.1 | 2.74 | 0.23 | 12.9 | 12.0 | 3.69 | 1.91 | 4.26 |

Win share by seat position, complementary: 0.237, 0.200, 0.225, 0.338 (no seat artefact; one winner per game). Current table, focus entry by the seat it sat in: 0.63, 0.18, 0.53, 0.66 (large seat effect at the current table, unexplained).
Containers bought per game by colour (colours 1..5), complementary: peddler2 2.7/2.4/1.9/2.2/3.7; producer_s2 2.0/2.6/1.8/2.3/3.4; showcase_fair_s2 2.8/1.8/1.6/1.4/3.4; control_s2 2.4/2.2/1.7/1.6/4.1. Current: peddler2 4.1/2.5/2.3/3.0/2.6; magnate 5.8/2.6/2.6/3.3/3.0.

## 3. What the Shipper reward includes (unchanged)

| transaction | in the reward? | where |
|---|---|---|
| accepted delivery: deliverer receives twice the top bid (the bidder's bid plus an equal subsidy minted by the supply) | yes, gross, all cargo sizes | `CARGO_CASH` counters |
| container purchases from lots (inventory investment) | yes, deducted | `SPENT` |
| containers shipped, cargo fill (containers per delivery / 5) | yes, as volume and utilisation terms | `SHIPPED`, `DELIVERIES` |
| bids the seat places as a rival (auction expenditure; the winner pays the bid, loses the cargo's cash, gains Island points) | no (not charged, not credited) | `BID_MONEY` is logged only |
| Island points won as a bidder | no | |
| buying out a delivery (deliverer pays the top bid to the Bank, keeps the cargo) | no revenue, no cost counted | not in `CARGO_CASH`, not in `SPENT` |
| factory and warehouse costs | no | not counted |
| production | no cash cost in the rules | n/a |
| sales of containers to others (`EARNED`) | no | not credited, so selling stock is invisible to the reward |
| realised economic profit (cash at end less cash at start) | not measured | the reward is a proxy, not profit |

`delivery_net` = gross delivery cash minus container purchases, dollars per game: it is not profit (misses building costs, bids paid, sales income, buy-outs).

## 4. Cargo size (from the existing probes; see economics.md)

Peddler v2, rivals bidding 50% of card value, forced to hold N containers (per game): cash per trip $9.5 (N=1), $17.0 (2), $24.1 (3); delivery cash per game 102, 97, 85, 73, 61; purchases rise (cash less purchases 37, 24, 10, -2, -14); sails per container 4.0, 3.3, 3.3, 3.4, 3.7; win 0.70, 0.58, 0.47, 0.37, 0.28. With rich rivals (100%, cash lent): delivery cash 188, 193, 191, 179, 159, cash less purchases 121, 118, 110, 96, 76, win 0.86, 0.85, 0.80, 0.75, 0.63.

# Offer exposure: the heuristics and the scripted bots

100 `DEV` seeds per composition, all rotations, completed games, per seat-game of each bot, each population playing among itself (no learner seated). The heuristics are Producer, Showcase and Shipper; the scripted bots are Merchant, Simple and Random, which the trainer seats 60% / 30% / 10% of its scripted seats. The nets of the pool and the learner's own snapshots are not measured.

| seats | bot | containers / delivery | deliveries of 2+ containers | factories built | warehouses built | seat-games with a warehouse built |
|---|---|---|---|---|---|---|
| 3 | Producer | 1.20 | 14.5% | 1.00 | 1.32 | 100% |
| 3 | Showcase | 1.17 | 14.7% | 0.10 | 3.09 | 100% |
| 3 | Shipper | 1.33 | 27.7% | 0.92 | 2.19 | 100% |
| 3 | **all three, equal weights** | 1.24 | 19.1% | 0.68 | 2.20 | 100% |
| 3 | Merchant | 1.29 | 26.6% | 0.00 | 0.00 | 0% |
| 3 | Simple | 2.87 | 59.2% | 0.00 | 0.00 | 0% |
| 3 | Random | 2.07 | 53.3% | 2.50 | 2.57 | 95% |
| 3 | **the trainer's mix 60/30/10** | 1.57 | 32.7% | 0.25 | 0.26 | 9% |
| 4 | Producer | 1.08 | 7.1% | 1.00 | 1.12 | 100% |
| 4 | Showcase | 1.17 | 13.9% | 0.09 | 3.17 | 100% |
| 4 | Shipper | 1.35 | 30.9% | 0.86 | 2.24 | 100% |
| 4 | **all three, equal weights** | 1.20 | 17.0% | 0.65 | 2.17 | 100% |
| 4 | Merchant | 1.29 | 26.1% | 0.00 | 0.00 | 0% |
| 4 | Simple | 3.01 | 62.0% | 0.00 | 0.00 | 0% |
| 4 | Random | 2.10 | 53.4% | 2.55 | 2.37 | 93% |
| 4 | **the trainer's mix 60/30/10** | 1.60 | 32.6% | 0.25 | 0.24 | 9% |
| 5 | Producer | 1.09 | 8.3% | 1.00 | 1.07 | 100% |
| 5 | Showcase | 1.16 | 13.8% | 0.10 | 3.22 | 100% |
| 5 | Shipper | 1.45 | 41.1% | 0.86 | 2.26 | 100% |
| 5 | **all three, equal weights** | 1.23 | 20.8% | 0.65 | 2.18 | 100% |
| 5 | Merchant | 1.40 | 35.8% | 0.00 | 0.00 | 0% |
| 5 | Simple | 3.05 | 62.4% | 0.00 | 0.00 | 0% |
| 5 | Random | 2.02 | 50.6% | 2.51 | 2.27 | 91% |
| 5 | **the trainer's mix 60/30/10** | 1.73 | 41.2% | 0.25 | 0.23 | 9% |

Left out as stalled: heuristics at 3 seats: 48 of 3,000 games stalled and left out; scripted at 3 seats: 0 of 3,000 games stalled and left out; heuristics at 4 seats: 0 of 6,000 games stalled and left out; scripted at 4 seats: 0 of 6,000 games stalled and left out; heuristics at 5 seats: 0 of 10,500 games stalled and left out; scripted at 5 seats: 27 of 10,500 games stalled and left out.

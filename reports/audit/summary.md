# Action abstraction and information audit

Every net plays 300 self-play games (players 3, 4, 5; seed 1) through the deployed `Neural` bot of the Rust crate `container-bots`, which offers at most 48 actions per decision (`thin`). Temperature 0 is the deployed Magnate (greedy plus repetition guard); temperature 1 samples from the net. Produced by `container-arena --bin audit` (`reports/audit/*.json`).

| audit | games | stalled | actions/game | decisions | over the 48 cap | full-set argmax dropped | softmax mass dropped | guard overrides | `RequestLoan` while bidding or deciding (legal / chosen) |
|---|---|---|---|---|---|---|---|---|---|
| control_run2_30m_sample | 300 | 0 | 767 | 205,275 | 3.36% | 8.4% of 6,900 | 8.3% | n/a (samples) | 40,904 / 0 |
| defense_run2_vs_exploiter_30m_sample | 300 | 0 | 765 | 205,250 | 3.73% | 8.0% of 7,651 | 7.9% | n/a (samples) | 41,286 / 0 |
| exploit_run2_sample | 300 | 0 | 783 | 210,501 | 1.97% | 2.6% of 4,140 | 2.7% | n/a (samples) | 38,673 / 0 |
| magnate_deployed | 300 | 0 | 771 | 206,760 | 3.86% | 8.7% of 7,985 | 8.4% | 0.16% | 42,771 / 0 |
| run1_sample | 300 | 1 | 774 | 205,588 | 2.45% | 7.7% of 5,028 | 7.1% | n/a (samples) | 38,771 / 0 |
| run2_guarded | 300 | 0 | 722 | 192,054 | 3.74% | 7.3% of 7,184 | 7.5% | 0.11% | 42,697 / 0 |
| run2_sample | 300 | 0 | 731 | 194,785 | 3.83% | 6.9% of 7,461 | 7.4% | n/a (samples) | 41,707 / 0 |

Over the cap, the thinning keeps `i*(n-1)/47` of the n actions, so the endpoints (`Bid(0)`, `Bid(cash)`, `DoneBidding`, `RequestLoan`) always survive. "Full-set argmax dropped" and "softmax mass dropped" evaluate the net on the unthinned list at decisions where thinning applied; the nets were trained on the thinned list, so these describe how smooth their bid preferences are, not a measured loss.

## Deployed Magnate by phase

| phase | decisions | legal actions: mean | p99 | max | over the cap | cash p50 | cash p99 | `RequestLoan` legal / chosen |
|---|---|---|---|---|---|---|---|---|
| arrange | 24,227 | 4.4 | 5 | 5 | 0 |  |  | 0 / 0 |
| bank_bid_cash | 12,423 | 33.1 | 85 | 113 | 2,125 | 30 | 87 | 0 / 0 |
| bank_bid_containers | 3,387 | 2.2 | 3 | 3 | 0 |  |  | 0 / 0 |
| bid | 34,848 | 34.3 | 84 | 123 | 5,860 | 28 | 82 | 34,801 / 0 |
| buy | 5,529 | 2.0 | 2 | 2 | 0 |  |  | 0 / 0 |
| decide | 8,344 | 3.3 | 5 | 6 | 0 | 26 | 78 | 7,970 / 0 |
| main | 118,002 | 12.3 | 19 | 23 | 0 |  |  | 106,786 / 13,282 |

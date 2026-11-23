# Interaction program: baselines, archetypes and the first controlled experiment

Goal: an opponent that takes part in the table's economy and still wins. Competitive strength and interaction are
always reported together. No ELO figure is used as evidence here.

## Tools added

| Tool | Purpose |
|---|---|
| `interact` (`crates/container-arena/src/bin/interact.rs`) | Plays mixed tables with one **focus** bot (random seat) and fillers from a pool, 3 to 5 seats, and prints per-seat-game means of competitive and interaction metrics. Same arguments, same JSON (checked across thread counts). About 13 games/s on 12 cores. |
| `net:PATH.bin` bot id | Plays an unreleased candidate net under Dealer's rules (bid at least $3 to open, no same-turn repay), so a candidate is measured by the same code as the shipped bots. |
| `netr:PATH.bin` bot id | The same, and also under the three table rules (`Neural::with_table_rules`): for a net trained with `--buy-at-harbor --bank-needs-load --no-double-reprice`. |
| Archetypes `discounter`, `bargain`, `trader` (`container_bots::ARCHETYPES`, not in the UI) | Scripted economic opponents. `discounter` lays everything at $1 (factory) and buys at any price; `bargain` prices around $2 and pays at most $3; `trader` is the existing Merchant trader. |
| Env seat kinds `-5` discounter, `-6` bargain; `train --seller-share P` | Cheap sellers as scripted league opponents in training. |
| `train --league-nets`, `--peddler`, `--peddler-share`; `training/league/peddler2.pt` | Which anchors and which frozen net the learner trains against. Defaults since Round 5: Consortium and Peddler v2 (15% of the opponent seats); the ladder keeps all five anchors. See [The league after Round 5](#the-league-after-round-5). |
| `reports/interaction/baseline.sh`, `summarize.py` | Reproduce and print the baselines. |

Metrics (per seat-game, from the actions a bot played and what its seat could see; nothing hidden is used):
`win_share` with 95% CI against `fair_share`, `score`; `bought` / `mean_price_paid` / `bought_fair_share` (purchases from
opponents, fair = $3 or less); `sold` / `mean_price_received`; `bids`, `mean_bid`, `bids_ge3_share` and `auctions_won`
(delivery participation); `deliveries`, `accepted`; `bank_actions`, `harbor_sails`, `reprices` (idling and Bank reliance);
`listed_fair_share` (share of its own containers on lots priced $3 or less, sampled at each of its main decisions);
`cheap_opportunities` and `cheap_take_rate` (main decisions with an opponent offer at $2 or less in reach, factory or the
docked harbor with cash to pay, and the share where it took one). The last is per decision, so it is low by construction
(two actions a turn); read it relative to other bots, not against 1.

## Baselines

`sh reports/interaction/baseline.sh 1500 1`: each focus bot, 1,500 games per pool, seed 1, 3/4/5 seats rotating.
Pools: **nets** = magnate, dealer, consortium (psro08); **mixed** = those plus merchant, solitaire, discounter, bargain,
trader; **cheap** = discounter, bargain, trader. In *nets* and *mixed* fillers can be the same net as the focus, so a fair
share is about 0.29 (3 to 5 seats). Raw files: `reports/interaction/baseline_<focus>_<pool>.json`.

| focus | pool | win ±95% | score | bought | $paid | sold | bids | $bid | auctions won | bank | listed ≤$3 | cheap take |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| magnate | nets | 0.132 ±0.017 | 78 | 15.8 | 4.54 | 14.6 | 23.0 | 2.39 | 5.6 | 14.2 | 0.03 | 0.163 |
| magnate | mixed | 0.594 ±0.025 | 125 | 12.7 | 2.88 | 10.5 | 16.8 | 2.50 | 3.0 | 14.2 | 0.05 | 0.081 |
| magnate | cheap | 0.996 ±0.003 | 151 | 11.6 | 2.25 | 8.9 | 15.9 | 2.63 | 0.7 | 8.9 | 0.09 | 0.082 |
| dealer | nets | 0.427 ±0.025 | 96 | 11.1 | 4.21 | 13.2 | 24.6 | 3.40 | 10.2 | 18.8 | 0.31 | 0.200 |
| dealer | mixed | 0.516 ±0.025 | 126 | 9.1 | 2.77 | 10.4 | 17.9 | 3.73 | 5.8 | 14.1 | 0.39 | 0.066 |
| dealer | cheap | 0.983 ±0.006 | 150 | 8.2 | 2.25 | 9.3 | 16.7 | 3.93 | 1.4 | 9.6 | 0.41 | 0.054 |
| consortium | nets | 0.223 ±0.021 | 82 | 15.5 | 4.58 | 14.6 | 23.1 | 2.86 | 7.0 | 14.2 | 0.01 | 0.124 |
| consortium | mixed | 0.604 ±0.025 | 125 | 12.6 | 3.01 | 10.2 | 16.8 | 2.67 | 3.4 | 13.5 | 0.02 | 0.062 |
| consortium | cheap | 0.996 ±0.003 | 152 | 11.3 | 2.32 | 8.4 | 15.8 | 2.67 | 0.7 | 8.8 | 0.04 | 0.065 |

What the baselines say:

- **The ratings do not order the nets here.** At nets-only tables Dealer (listed 2170) wins 0.43, Consortium (2378) 0.22 and
  Magnate (2280) 0.13, with the fillers of the same pool. Against the mixed and cheap pools all three beat the scripted
  fields about equally (0.52 to 0.60, 0.97 to 1.00). A single ladder number hides which field a bot is strong against.
- **Dealer's interaction is in what it offers and how it bids, not in buying.** It lists 31 to 41% of its containers at
  $3 or less (Magnate 3 to 9%, Consortium 1 to 4%), bids $3.4 to $3.9 on average (the others $2.4 to $2.9) and wins 1.5 to
  2 times as many auctions, but buys 25 to 30% fewer containers (8 to 11 against 12 to 16).
- **No net uses a cheap offer.** Against cheap sellers all three pay about $2.3 per container and take a cheap offer in
  5 to 8% of the decisions where one is in reach, about the rate they take it at nets-only tables where cheap offers are
  rare (12 to 20%, which includes the nets' own harbor purchases). The cheap sellers are pushed over, so
  winning (0.97 to 1.00) says nothing about how well the offers were used.
- All stall rates are at most 0.3%.

## Experiment 1: do cheap-seller opponents in training teach a net to buy from them?

Hypothesis H1: continuing Dealer with 35% of scripted seats replaced by cheap sellers (`--seller-share 0.35`) raises the
purchases and the cheap-take rate against cheap sellers without losing strength.

Design (fixed before the runs): the same start (`inter_B/final.pt`, Dealer), the same 30 minutes, seed 11, flags
`--no-same-turn-repay --min-bid 3 --interaction-weight 0.5`, one variable, `--seller-share` 0 (A, control) or 0.35 (B).
No new reward term. Evaluation: the three pools above, 1,500 games each, the same seeds as the baselines (so the tables are
the same), plus seed 2 on the cheap pool. Acceptance for B as a balanced candidate: win share within 0.04 of Dealer on
nets and not lower on mixed, **and** cheap-take rate at least 0.03 above A on the cheap pool.

| bot | pool | win ±95% | score | bought | $paid | bids | $bid | auctions won | listed ≤$3 | cheap take |
|---|---|---|---|---|---|---|---|---|---|---|
| dealer | nets / mixed / cheap | .427 / .516 / .983 | 96 / 126 / 150 | 11.1 / 9.1 / 8.2 | 4.21 / 2.77 / 2.25 | 24.6 / 17.9 / 16.7 | 3.40 / 3.73 / 3.93 | 10.2 / 5.8 / 1.4 | .31 / .39 / .41 | .200 / .066 / .054 |
| A (control) | nets / mixed / cheap | .420 / .535 / .980 | 97 / 127 / 150 | 12.4 / 9.6 / 8.1 | 4.19 / 2.70 / 2.23 | 24.6 / 17.9 / 16.6 | 3.39 / 3.83 / 4.16 | 9.8 / 6.0 / 1.8 | .20 / .30 / .33 | .209 / .072 / .050 |
| B (sellers) | nets / mixed / cheap | .404 / .555 / .979 | 97 / 128 / 150 | 12.6 / 10.3 / 9.1 | 4.20 / 2.73 / 2.32 | 24.5 / 17.6 / 16.4 | 3.33 / 3.66 / 3.85 | 9.3 / 5.5 / 1.2 | .28 / .36 / .38 | .203 / .079 / .060 |

(±0.025 on every nets and mixed win share, ±0.007 on cheap.) Seed 2, cheap pool, bought per game: Dealer 8.2, A 8.0, B 9.1.

Result:

- **Strength is preserved.** B is within the CI of Dealer and A at every pool. (The trainer's own eval gave 2197 for B and
  2211 for A against Dealer's 2170; not used as evidence.)
- **B buys about one container more per game than A against cheap sellers** (+1.0 on seed 1, +1.1 on seed 2, +0.7 on mixed)
  and keeps more of the fair pricing that plain continuation erodes (listed at $3 or less: .28 against A's .20 on nets). Both
  effects are small and come from one training run each.
- **The target behavior did not move.** The cheap-take rate is .060 for B against .050 for A (+0.010, acceptance needed
  +0.03) and the price paid stays at $2.3. H1 is **not supported** as stated; B is kept as a candidate, not adopted.
- Control A, which is only more training with the interaction reward, lost a part of Dealer's fair pricing (.41 to .33
  on the cheap pool): continuing Dealer without the sellers drifts away from its interaction profile.
- Nothing was exported to the shipped bots. Dealer, Magnate and Consortium stay the champions; the candidates are
  `runs/exp/exp_A/` and `runs/exp/exp_B/` (untracked, as `runs/` is).

## Round 2: probe, opponent memory, constrained training

### Why the nets ignore cheap offers (`probe`, `reports/probe/*.json`)

`probe` stops a game at a main decision where the focus net could buy a container for $2 or less, then compares forced
"buy every cheap container" with the net's own choice by paired rollouts (same net at temperature 0.3, same seeds and
fillers). 250 positions x 24 rollouts per cell, seed 3; `sh reports/probe/run.sh` reproduces it.

| cell | P(buy) | buy is top | logit gap | value gain | truth score gain | truth win gain |
|---|---|---|---|---|---|---|
| dealer / cheap pool | .12 | .12 | 28.5 | +.001 | -0.86 ±0.71 | -.0005 ±.0055 |
| magnate / cheap | .10 | .11 | 53.9 | +.003 | +0.42 ±0.81 | +.0002 ±.0019 |
| consortium / cheap | .09 | .10 | 64.1 | +.008 | -0.75 ±0.70 | -.0011 ±.0032 |
| dealer / mixed | .18 | .18 | 25.7 | +.001 | +0.25 ±0.76 | +.0071 ±.0145 |
| consortium / mixed | .10 | .11 | 63.1 | +.011 | -0.63 ±0.78 | -.0099 ±.0160 |

Reading: the policy and the value head agree with the rollouts. Forcing the cheap purchase gains nothing measurable
(every interval contains 0, buying is better in 44-54% of positions), and the value head predicts that. The nets are not
blind to the offer; they judge, correctly as far as this test can tell, that the action it displaces is worth as much.
So "cheap take" is not an error to fix, and a bonus for it would be an arbitrary transaction bonus. What cheap sellers
change is the price level of a game, which the reward in the next sections can reflect without a purchase bonus.
Limit: rollouts use the same net as the continuation, so a net with a systematic blind spot would share it.

### Opponent memory (`container_bots::memory`, `training/add_memory.py`)

Public history only: per opponent (viewer-relative) mean price paid, containers bought and sold, mean price received,
share of its containers laid out at $3 or less, containers it bought from the viewer and the viewer from it. 28 inputs
are appended to the 520-input state (548). `Opponent::observe` feeds every action to every bot; `Neural` keeps a
`Memory` when the weights are 548 wide; the env keeps one per game (`CAI_MEMORY=1` makes the env emit them). Nets
without memory still load (520) and in training see the front of the row. `add_memory` zero-pads the first layer, and
the padded Dealer plays identically to Dealer (200 games, same seed, identical results).

### Constrained training (`--win-slack`, `--dual-lr`, `--dual-max`)

The interaction share of the reward is a dual variable mu: raised while the learner's training-league win share stays
above (its first six iterations - slack 0.02), cut while it is below. No transaction bonus beyond the existing
`interaction_term`. exp_C: Dealer init, mu starts at 0.3, seller-share 0.35, 40 min.
Caveat: mu reached its cap (0.6) within ~90 iterations and stayed there, because the league (with 35% cheap sellers)
is easy and the win share never approached the floor. The constraint was never binding, so this run shows the
mechanism works but not that it protects strength; the competitive check is the evaluation below.

### Results (1500 games per pool, seed 1, +-0.025 on nets/mixed)

| candidate | nets win | mixed win | cheap win | nets buy | nets list<=3 | nets cheap-take | mixed cheap-take |
|---|---|---|---|---|---|---|---|
| Dealer | .427 | .516 | .983 | 11.1 | .31 | .200 | .066 |
| exp_A control (30 min) | .420 | .535 | .980 | 12.4 | .20 | .209 | .072 |
| exp_B interaction .5 + sellers (30 min) | .404 | .555 | .979 | 12.6 | .28 | .203 | .079 |
| exp_C constrained (40 min) | .374 | .553 | .983 | 11.9 | .37 | .189 | .072 |
| exp_M memory + B recipe (40 min) | .388 | .572 | .987 | 12.2 | .39 | .224 | .102 |

- exp_M is the best interaction profile (most cheap-taking and fair listing, buys more than Dealer on every pool) and
  the best mixed-pool win share, with a lower nets-pool win share than Dealer (-.04, about 1.1 combined standard errors).
- exp_C lists more fairly and bids lower, but loses .05 on the nets pool (about 1.4 combined standard errors).
- Neither is a champion replacement: both lose some strength against the strong nets, and no shipped weights changed.
- Not shown: that exp_M *uses* its memory. The control that would show it (exp_B, same recipe, no memory) differs by
  +.02 cheap-take and -.02 nets win, inside the noise, and arms differ in length (30 vs 40 min). One seed each.

## Round 3: from-scratch interactive nets and Haggler

Starting from scratch instead of from Dealer/Magnate (30-35 min, same league, `--seller-share 0.35 --min-bid 3
--no-same-turn-repay`):

| run | reward | ELO (trainer eval) | nets win | mixed win | nets: bought / sold | list<=3 |
|---|---|---|---|---|---|---|
| scratch_I | 0.5 interaction + 0.5 default | 2062 | .108 | .427 | 16.2 / 15.2 | .38 |
| scratch_max | 1.0 interaction | 361 | 0 | 0 | 12.5 / 11.8 | .99 |
| scratch_w08 = **Haggler** | 0.8 interaction + 0.2 win share, no margin (`--no-margin`) | 2002 | .039 | .419 | 13.2 / 13.4 | .75 |

Pure interaction reward produces a net that trades and cannot play; any win term fixes it. One seed each, 400-600 games.
Haggler is shipped as the `haggler` bot (`weights/haggler.bin`, listed at 2000 ELO). It is not a competitive champion.

### What each net does (`actions`, `reports/interaction/actions_4nets.json`)

`./target/release/actions --games 400 --seed 1`: all four nets at one table (4 seats, rotated), per seat-game, forced
moves (a single legal action) excluded. `Deal` and `BuyOut` are ~0 for every net.

| action | Haggler | Dealer | Magnate | Consortium |
|---|---|---|---|---|
| Sail | 45.5 | 40.9 | 40.5 | 49.6 |
| Bid | 44.7 | 45.0 | 34.5 | 35.2 |
| Place | 24.9 | 22.7 | 25.5 | 16.3 |
| Produce | 9.9 | 12.9 | 12.6 | 12.8 |
| BidCash | 11.2 | 13.4 | 9.2 | 8.1 |
| Accept | 8.1 | 7.9 | 7.9 | 10.8 |
| RequestLoan | 1.0 | 0.1 | 12.9 | 19.8 |
| RepayLoan | 0.7 | 0.1 | 12.8 | 19.8 |
| FactoryPurchase | 6.6 | 7.0 | 13.4 | 4.2 |
| Outbid | 9.3 | 12.3 | 4.3 | 3.6 |
| StartAuction | 3.0 | 5.5 | 9.2 | 6.3 |
| Buy | 2.0 | 2.5 | 6.0 | 8.7 |
| HarborPurchase | 4.9 | 2.8 | 2.9 | 3.2 |
| DoneBidding | 0.6 | 3.0 | 2.8 | 1.0 |
| Load | 2.4 | 4.1 | 0 | 0 |
| BidContainer | 0.4 | 2.3 | 1.4 | 0.9 |
| Reprice | 2.5 | 2.0 | 0 | 0 |
| DoneBuying | 1.2 | 0 | 0 | 0 |
| DoneLoading | 0.3 | 0.3 | 0 | 0 |
| Seize | 0 | 0 | 0 | 0.2 |

- Haggler and Dealer bid most (about 45 `Bid` per game against about 35) and outbid most (9-12 against about 4).
- Haggler buys at factories as often as Dealer (`FactoryPurchase`) and uses the paid `HarborPurchase` action more than any
  net; Magnate buys twice as many at factories. These columns count the paid actions that open a purchase only: a free
  purchase on arriving at a harbor is a `Buy`, and `Buy` leaves out forced moves, so none of them is a count of purchases
  (Round 4 gives those, from `interact`; Magnate and Consortium make as many harbor purchases as any net).
- Haggler and Dealer almost never take loans (0-1 per game); Magnate and Consortium take 13-20 and repay them. This is a
  large strategic difference and probably part of Haggler's weakness against them.
- Haggler starts the fewest Bank auctions (3.0 against 5.5-9.2) and produces least.
- Counts say what a net does, not whether it is right to; they carry no win-share information.

## Round 4: table rules (`scratch_rules`)

Three rules, applied to the learner seat only (opponents in the league are unrestricted): no repricing the same district
twice in a row within a turn (`--no-double-reprice`); sailing to a player must be followed by a purchase
(`--buy-at-harbor`, and the bot refuses a harbor with nothing buyable); sailing to the Bank only with a container to
pick up and room on the ship (`--bank-needs-load`). Same recipe as Haggler (from scratch, 0.8 interaction, `--no-margin`,
seller share .35, min bid 3, 30 min, seed 41). Deployed with `netr:PATH` (`with_table_rules()`); a test checks that a
net played under it never breaks a rule. Trainer ELO 1903 (Haggler 2002); 2 of 1024 eval games stalled.

Interaction (`interact`, 500 games per pool, seed 1, focus seat averaged over 3-5 players):

| pool | win | bought | sold | bids | list<=3 | cheap-take |
|---|---|---|---|---|---|---|
| nets | .061 ±.021 | 14.2 | 12.4 | 20.9 | .68 | .27 |
| mixed | .376 ±.043 | 12.9 | 10.7 | 16.7 | .51 | .05 |
| cheap sellers | 1.000 | 11.0 | 9.4 | 15.8 | .38 | .03 |

Versus Haggler (nets .039, mixed .419, bought 13.2 / sold 13.4): similar strength, slightly more buying, slightly less
selling, fewer cheap listings (.68 against .75).

Actions per seat-game, 5 nets seated in rotation (`reports/interaction/actions_rules.json`, 400 seat-games each):

| action | scratch_rules | Haggler | Dealer | Magnate | Consortium |
|---|---|---|---|---|---|
| Sail | 51.1 | 40.5 | 35.4 | 36.6 | 38.2 |
| Bid | 37.3 | 44.7 | 48.0 | 42.9 | 42.2 |
| Place | 22.4 | 31.7 | 30.3 | 25.4 | 22.4 |
| Outbid | 12.1 | 9.4 | 7.8 | 7.3 | 6.5 |
| Accept | 12.5 | 7.6 | 6.2 | 6.6 | 8.0 |
| Buy | 10.1 | 2.8 | 2.5 | 4.0 | 5.5 |
| Produce | 9.4 | 15.7 | 17.4 | 14.1 | 17.0 |
| StartAuction | 3.7 | 8.3 | 13.0 | 11.8 | 10.2 |
| FactoryPurchase | 5.9 | 5.3 | 8.7 | 11.2 | 7.1 |
| Load | 3.3 | 4.4 | 3.8 | 0 | 0 |
| Reprice | 1.8 | 1.1 | 1.0 | 0 | 0 |
| RequestLoan / Repay | 2.1 / 2.1 | 0.1 | 0.1 | 11.3 | 27.6 |

`actions` leaves out forced moves (one legal action) in both tables and the seat mix differs between them, so compare
columns within a table only. It is also **not a purchase count**: Magnate's `Buy` is 4.0 here against 15.8 purchases a game
in `interact`. The rule net's `Buy` of 10.1 is mostly the free purchase on arriving at a harbor, which the engine lets a
bot decline and so counts as a choice; it never uses the paid `HarborPurchase` action (0.0 against 2.3-4.2).
`interact` counts every purchase. On the same nets pool (3-5 seats, seed 1; `baseline_*_nets.json`, `scratchw08_nets.json`,
`peddler_nets.json`):

| focus | bought (factory / harbor) | sold | deliveries | delivery auctions won | list<=3 |
|---|---|---|---|---|---|
| Peddler | 14.2 (6.3 / 7.8) | 12.4 | 10.9 | 10.6 | .68 |
| Haggler | 13.2 (7.4 / 5.8) | 13.4 | 7.5 | 9.0 | .75 |
| Dealer | 11.1 (7.4 / 3.7) | 13.3 | 7.9 | 10.2 | .31 |
| Magnate | 15.8 (7.5 / 8.3) | 14.6 | 7.5 | 5.6 | .03 |
| Consortium | 15.5 (7.1 / 8.4) | 14.6 | 7.7 | 7.0 | .01 |

So the rule net buys about as much as Magnate and Consortium, and not 2-4 times more than the other nets as the `Buy` column
of the action table suggested (an earlier version of this section said so; it was a counting artifact). What the evidence
does show: it gets its harbor purchases by sailing and buying on arrival, it delivers most often, it sells slightly less than
the others, and it lists cheaply (.68, with Haggler .75). It also sails most (51
against 35-40), produces least, starts the fewest Bank auctions, and takes few loans (2.1 against 11-28 for Magnate and
Consortium). The double-reprice rule is respected (Reprice 1.8, never repeated).

Caveats: one seed; the rules bind only the learner, so the net faces opponents free of them; the 500-game pools carry
about ±.04 on win share; the action table has no win information.

### Shipped as `peddler`: what was checked, and the limits

The net is shipped as the `peddler` bot (`crates/container-bots/src/peddler.rs`; `weights/peddler.bin` is a
byte-identical copy of `runs/exp/scratch_rules/weights.bin`), listed at **1900**. That is the trainer's final eval with
the rules on its seat only, so it is not comparable with an unrestricted rating (the README says so too).

- The shipped bot is the evaluated net: `interact --focus peddler` on the nets pool gives the numbers of the `netr:` run
  (win .061, bought 14.16, sold 12.44; `peddler_nets.json` against `scratch_rules_nets.json`).
- Two tests: the Rust rule filter bars exactly the `Sail(Bank|Harbor)` actions that open no Buy/Load phase, checked
  against the engine at every Main-phase decision (`the_table_rules_bar_exactly_the_sails_that_open_nothing`), and Peddler
  still makes harbor and Bank trips at every table size (`peddler_keeps_the_table_rules_at_every_table`).

Win share of the focus seat against the weak bots (`interact`, seed 1; Haggler is the shipped bot, no rules):

| pool, table | Peddler | Haggler |
|---|---|---|
| Merchant, 3-5 seats (600 games) | 1.000 | 1.000 |
| Merchant + Solitaire, 3-5 seats (500) | .654 ±.042 | .945 ±.020 |
| Solitaire, 3-5 seats (600) | .603 ±.039 | .940 ±.019 |
| Solitaire, 3 seats (300) | .990 ±.012 | .973 ±.018 |
| Solitaire, 4 seats (300) | .657 ±.054 | .973 ±.018 |
| Solitaire, 5 seats (300) | **.243** ±.049 (fair share .20) | .893 ±.035 |

So against the weak bots Peddler is clearly worse than Haggler: it beats Merchant at every table, and Solitaire at 3 and 4
seats, but is barely ahead of four Solitaires at 5 seats. Against Magnate, Dealer and Consortium it wins .061 ±.021.

- **Not the rules.** Haggler's net played under the same three rules (`netr:`, `hagglerrules_*.json`) wins .967 ±.014
  against Solitaire (.950 at 5 seats), .028 against the nets and .423 on the mixed pool: the rules cost it nothing. Peddler's
  net played without them (`net:`) is worse still at 5 seats (.037, `peddler_norules_vs_solitaire_5p.json`), as it was trained
  under them. The weakness is in the policy this run learned, which never met a Solitaire in training.
- What the counts show at 5 seats against Solitaire (not a measured cause): Peddler wins 14.0 delivery auctions per game with a
  mean bid of 6.5 (Haggler 0.4 and 3.2) and sells 8.6 containers (Haggler 12.4). It spends on other seats' deliveries, which
  the interaction reward credits to both sides.
- **Stalls.** In the pooled 600-game run against Solitaire, 8 games hit the 2500-action horizon with no winner, all of them
  3-seat tables (two Solitaires; about 200 such games, so 4%). A separate 300-game run at 3 seats gives the same rate, .040
  (`peddler_vs_solitaire_3p.json`); Haggler has none (0 of 600). A trace of the first stalled game shows both Solitaires out
  of cash with 2 loans, only repricing, while Peddler is rich (about $170, 22 containers) and cycles through a Bank auction,
  produce, a Bank trip, a delivery and a $0 delivery bid. The supply stays one container short of ending (one colour empty,
  another at 1), and the deadlock check does not fire because Peddler has cash. Peddler has the repetition guard every greedy
  `Neural` has (it varies the action only for an identical input), and the guard did not end the cycle. The README
  already records 2.5-11% stalls in training for the harbor-rule runs.

Not done: a Solitaire in the league, a guard against cycles that never repeat an input, another seed.

## Round 5: Solitaire, Merchant and Peddler in the league

Peddler (`scratch_rules`) lost to Solitaire at 5 seats (.24) and stalled in 4% of 3-seat tables with two Solitaires. Two
runs with the same recipe as Round 4 (`--no-same-turn-repay --min-bid 3 --buy-at-harbor --bank-needs-load
--no-double-reprice --interaction-weight 0.8 --no-margin --seller-share 0.35`) and a changed league: every opponent seat is
first drawn as the frozen Peddler (15%), else Solitaire (20%), else an extra Merchant (15%), else the old league. Per
opponent seat that makes: Peddler 15%, Solitaire 17%, Merchant 23% (10% extra, 12% among the scripted), earlier snapshots
and anchor nets 26% (45% in Round 4), cheap sellers 11% (19% in Round 4), Simple 6%, Random 2%; a quarter of the tables
are the learner alone, as before.

New plumbing: seat kind `-7` Solitaire in the env (`SOLITAIRE`), `--solitaire-share`, `--merchant-share`, `--peddler CKPT
--peddler-share P`. The frozen Peddler plays under the learner's table rules (`cenv_mirror_rules`, `BatchEnv.mirror_rules`),
as the shipped bot does; the other frozen snapshots and anchor nets stay unrestricted, as before.

| Run | What | Trainer ELO |
|---|---|---|
| `scratch_rules_sol` | from scratch, 30 min, seed 43 | 1668 (v1: 1903) |
| `scratch_rules_sol_ft` | warm start from Peddler's checkpoint, no imitation, 20 min, seed 44 | 2226 |

`interact`, seed 1, win share (fair share .20 to .33 by table size):

| Pool | Peddler v1 | `sol` (scratch) | `sol_ft` (fine-tuned) |
|---|---|---|---|
| Solitaire, 3 seats (300) | .990, stall .04 | .935 | .993, stall .023 |
| Solitaire, 4 seats (300) | .657 | .232 | .997 |
| Solitaire, 5 seats (300) | .243 | .060 | .967 |
| Merchant, 3-5 seats (600) | 1.000 | .993 | .998 |
| Merchant + Solitaire (500) | .654 | .496 | .940 |
| Magnate, Dealer, Consortium (500) | .061 | .002 | .391 |
| Mixed pool of 8 bots (500) | .376 | | .629 |
| Discounter, Bargain, Trader (500) | 1.000 | | .998 |
| Peddler v1 + Haggler (500) | | | .714 (Peddler .084, Haggler .107) |

Result: the from-scratch run is worse than Peddler v1 everywhere, so a league with weak bots seated does not by itself fix
the weakness (one seed; the run is one draw from a noisy process). The fine-tuned net fixes it, and it is also much stronger
against the strong nets than Peddler v1 (.39 against a fair .26, from .06), though not against all three alike (see the
next section). Fine-tuning for 20 more minutes in a league that holds the strong anchors, Solitaire and Peddler is the only
change, so some of the gain is more training, not the league; no run of v1 trained 20 more minutes in the old league
exists to separate the two.

Interaction of `sol_ft` against the strong nets, with Peddler v1 in brackets: bought 15.0 (14.2), sold 17.5 (12.4), delivers
11.0 (10.9), about 80% of its lots listed at $3 or less (68%), cheap-offer take rate .32 (.24). Against the Magnate-class bots it
sells more and buys no less. It still stalls 2.3% of 3-seat tables with two Solitaires (7 of 300): the cycle guard is not done.

Caveats: one seed per run, the CI is about +-.04 on 500 games; the trainer ELO is measured without table rules for the
anchors and is not comparable across rule sets; the human-play check is still missing.

### Shipped as `peddler2`: what was checked, and the limits

The fine-tuned net is shipped as the `peddler2` bot, "Peddler v2" (`crates/container-bots/src/peddler2.rs`;
`weights/peddler2.bin` is a byte-identical copy of `runs/exp/scratch_rules_sol_ft/weights.bin`, sha256 `1827353a...`),
listed at **2225**, between Dealer and Classic. That is the trainer's final eval (2225.7, 1024 games) with the rules on its
seat only, so it is not comparable with an unrestricted rating. `peddler` is unchanged; the two bots share the five rules
and differ only in the weights.

- **The shipped bot is the evaluated net.** `interact --focus peddler2` on nine pools (`peddler2_*.json`, the same seeds,
  sizes and game counts as the `netr:` runs in `solft_*.json`) gives identical statistics for every bot, in every field.
- **Tests.** `the_peddlers_keep_the_table_rules_at_every_table` now runs both ids, and the registry tests cover the new id
  without changes (creatable, deterministic, different seeds differ, finishes every kind of game alone and in a mixed
  table, list ordered by rating). `container-bots` (24), `container-web` (9) and `container-env` (17) pass, and the WASM
  build compiles: the module is now 12.4 MB (11.4 MB gzipped), of which the new weights are 1.5 MB.

Win share of the focus seat against one kind of strong net (`interact`, 500 games each at 3-5 seats, seed 1, +-.04):

| Opponent | Peddler v2 (even share .26) | one opponent seat (even share .25) |
|---|---|---|
| Magnate | .516 | .16 |
| Consortium | .369 | .21 |
| Dealer | .272 | .24 |
| the three in one pool | .391 | Dealer .35, Consortium .15, Magnate .11 |

So Peddler v2 is not simply stronger than the strong nets: it beats Magnate and Consortium, and is level with Dealer (.27
is within the interval of an even .26). One reading, not tested: Magnate and Consortium are one lineage (`run1`, `run2`,
Magnate, then psro02 to psro08; `populations/psro/manifest.json`; their weights are 0.996 alike by cosine), and the
league's anchors held `run1`, `run2`, `exploit_run2` (a child of `run2`), Consortium itself and `traders` (a from-scratch
net trained against price-sensitive traders, rated 1907, outside that family), whereas Dealer, from the from-scratch
Broker, is not in that lineage and was not seated (nor were Magnate itself, Broker, Haggler or Classic). Part of the edge
over Magnate and Consortium may be specialization to that family. A second run
with Dealer seated, compared with this one, would test it. The label is the trainer's figure for the same reason: read as
ratings, these results would put the net above Magnate (2280) and Consortium (2378) but only level with Dealer (2170),
which no single number expresses, and it was not rated on the ladder's scale (a Bradley-Terry fit against Magnate, as for
Consortium's label, would give a figure for that one pairing).

Limits:
- **Stalls.** 7 of the 300 three-seat tables with two Solitaires (2.3%, Peddler v1 4%) reach the 2500-action horizon, and
  .4% of the Merchant and Solitaire pool. The cause was traced for v1 (above) and not again for v2; the cycle guard is not
  built.
- **One seed, no control.** A run of v1 trained 20 more minutes in the old league does not exist, so the league is not
  shown to be the cause of the gain; the 30-minute from-scratch run in the same league was worse than v1.
- **No human play yet.** All of the above is bots against bots. Nothing here shows how it plays against a person.

### Later measurements on Peddler v2: building, ship loads, where the score comes from

All with seed 1; `actions` and `interact` are in `crates/container-arena`.

- **Building is rare.** `actions` (5 seats, 500 games, all five nets at one table) counts 0.82 factories per game for Peddler
  v2 and none for Peddler, Dealer, Magnate or Consortium; nobody builds a warehouse. Against Solitaire, Peddler v2 builds
  1.39, 0.96 and 0.86 factories per game at 3, 4 and 5 seats (0.19 warehouses at 3 seats). `actions` now reports
  `BuildFactory` and `BuildWarehouse` separately. There is no harbor-building action; `bought_factory` and `bought_harbor`
  in the interact JSON count containers bought, not buildings.
- **One container per harbor visit.** Over 300 games at a 5-seat table, every trained net (Peddler v2, Peddler, Dealer,
  Magnate, Consortium) bought exactly one container in every harbor purchase (about 10,600 purchases), and Peddler v2 sailed
  to the Island with a single container in 3151 of 3160 deliveries (Dealer: 1599 of 1983). The scripted Merchant does buy 2-4
  at once, so the measurement can see it. A probe over about 3,700 harbor visits showed why: the opponent's harbor shelf held
  exactly one container at essentially every visit (0% held two at 3 and 4 seats, 0.3% at 5 seats), so no second purchase was
  legal. In the 3 cases where it was, Peddler v2 chose "done buying" (p about .96). So the nets have almost never been offered
  a second container, in play or in training; whether they would decline it if offered often is not known. A test needs
  positions with a stocked harbor. The high delivery count (11 a game) is therefore mostly one-container trips.
- **Where the score comes from** (`cash`, `island`, `leftovers`, `loans` and `won_*` in the interact JSON; pool Magnate,
  Dealer, Consortium and Peddler v2, 500 games). Peddler v2 wins on cash: 78 of its 111 points in won games, Island 28,
  leftovers 4. Dealer wins as often with cash 56 and Island 48; Magnate and Consortium carry 11-14 points of leftovers.
  Against Merchant and Solitaire, Peddler v2 uses the Island little (11 and 1 points) and wins on cash plus 19-27 of
  leftovers. Peddler v1 carried about 7 points of loans a game. The committed `peddler2_*.json` files predate these fields.

The hypothesis that the table rules (no double reprice, buy at harbor, bank needs a load) make the bot more interactive,
because every action then changes the board, is plausible and untested. Haggler, trained without them, buys 13.2 and sells
13.4 a game against Peddler's 14.2 and 12.4, so the rules are not obviously the cause, but those runs differ in seed and
league. A controlled test is the same recipe with and without the rules, over the same seeds.

## The league after Round 5

Request: keep high-rated nets that do not trade out of the training league. `run1`, `run2`, `exploit_run2` and `traders`
leave it, Consortium stays, and Peddler v2 joins.

What changed (`training/train.py`; no weight and no rating moved):

- `--league-nets NAMES` picks the rated anchors that are training opponents: `consortium` by default, `all` for the league
  as it was, `none`, or a list such as `consortium,traders`. A name that is not an anchor stops the run and lists them.
- `--peddler` now defaults to `training/league/peddler2.pt` and `--peddler-share` to 0.15, the share of the Round 5 recipe.
  The file is `runs/exp/scratch_rules_sol_ft/final.pt`, and a test checks that it exports byte-identical to
  `weights/peddler2.bin`. `.gitignore` lets `training/league/*.pt` through. The seat is skipped in `--exploit` runs, with
  `--peddler-share 0` and with `--peddler ''`.
- The ladder is untouched. `training/ladder.json` and `training/anchors/*.pt` still hold all five anchors, and the evaluations
  (the trainer's, `anchors rate`, `duel`) still rate against all of them. No ELO in this repository changes meaning,
  `--exploit run2` still works, and no checkpoint is lost (`traders.pt` exists only there and in git: its run directory is
  gone). Taking the four nets off the ladder instead would have moved every rating and every label.
- The trainer prints `league: anchor nets [...], peddler seat ...` at the start. It also warns when the learner lacks any of
  Peddler v2's five rules, because the seat plays under the learner's rules and no others (`BatchEnv.mirror_rules`).

Share of an opponent seat by default (the arithmetic of `league_table` with five snapshots in the league; the Round 5
recipe puts Solitaire, extra Merchants and cheap sellers in front of the old draws):

| Seat | Before | Now |
|---|---|---|
| Peddler v2 | none | 15.0% |
| Consortium | 4.5% | 6.4% |
| `run1`, `run2`, `exploit_run2`, `traders` | 4.5% each | none |
| the learner's five snapshots | 22.5% | 31.9% |
| Merchant, Simple, Random | 55.0% | 46.8% |

Before a first snapshot exists (the first evaluation) the whole 38% that is not scripted or Peddler is Consortium.

**Who trades.** `python -m training.audit_interaction --games 300 magnate=populations/baselines/defense_run2_vs_exploiter_30m.pt
run1=training/anchors/run1.pt run2=training/anchors/run2.pt exploit_run2=training/anchors/exploit_run2.pt
consortium=training/anchors/consortium.pt traders=training/anchors/traders.pt` (three copies of the net and a Merchant at
each four-seat table, unrestricted rules, deterministic seeds; it gave the same table on a second run). Per game, summed
over the net's three seats:

| Net | Containers bought | Bids on players | Accepted | Mean bid | Laid out in lots 0-2 | Cash / island / leftovers |
|---|---|---|---|---|---|---|
| Magnate | 15.7 | 66.6 | 20.1 | $1.83 | 8% | 57.8 / 25.7 / 20.9 |
| `run1` | 16.9 | 72.6 | 21.5 | $1.87 | 9% | 59.1 / 27.1 / 18.8 |
| `run2` | 15.8 | 70.1 | 20.4 | $1.89 | 11% | 58.5 / 25.8 / 19.6 |
| `exploit_run2` | 16.9 | 69.4 | 21.6 | $1.42 | 15% | 54.4 / 23.3 / 19.3 |
| Consortium | 17.6 | 76.7 | 22.2 | $1.93 | 1% | 60.7 / 28.2 / 18.6 |
| `traders` | 21.5 | 90.0 | 30.0 | $3.88 | 25% (33% in lot 4) | 91.0 / 43.4 / 5.2 |

(Lots 0 to 2 are the cheap ones the trainer counts as fair: a lot costs its index plus 1 in a factory, plus 2 in a harbor.)
The audit does not show that the first five do not interact: they buy and bid about as much as Magnate, and the arena puts
Magnate and Consortium at 16.5 and 16.3 containers bought a game against Peddler v2's 15.0. What sets them apart is price:
they lay out 1 to 15% of their containers in the cheap lots (the arena counts Magnate at 3 to 9% listed at $3 or less),
where Peddler v2 lists about 80% at $3 or less (`interact`, nets pool). `traders` is the exception on buying: the highest of
the six in every buying column, though it also lays out a third of its containers in lot 4. It leaves the league because
the request names it, and `--league-nets consortium,traders` puts it back. Apart from this audit `run1`, `run2` and
`exploit_run2` have no interaction measurement.

What this does not show:

- **Nothing was trained in the new league.** Whether it makes a better opponent, or a weaker one, is not measured. The
  smoke runs (2 minutes from Peddler v2's weights, one seed) only checked the plumbing: the `league:` line, the new
  settings in `config.json`, the rule warning, the typo and bad-path errors, the `--exploit` path and `--league-nets none`.
- **Fewer strong opponents.** The strong nets are now two lineages (Consortium: `run1`, `run2`, Magnate, `psro`; and
  Peddler v2) instead of five nets of which four were Magnate's family, which makes it easier to specialise to them. The
  evaluation still rates against all five anchors, so a loss shows in the ELO, and `interact` pools would show the rest.
- **Round 5's specialization question is only half addressed.** Peddler v2 is outside Magnate's lineage and is now seated;
  Dealer, Broker, Haggler, Classic and Magnate itself still are not.
- **A run is not comparable to Round 5 without a control.** The same recipe, seed and budget with `--league-nets all
  --peddler-share 0` is the old league; the difference between the two runs is the league.

### Training in the new league

```sh
.venv/bin/python -m training.train --name next --budget-minutes 30 --init training/league/peddler2.pt --no-imitation \
    --no-same-turn-repay --min-bid 3 --buy-at-harbor --bank-needs-load --no-double-reprice \
    --interaction-weight 0.8 --no-margin --solitaire-share 0.2 --merchant-share 0.15 --seller-share 0.35
```

The league is Consortium and Peddler v2 (both on by default) with the rest as in Round 5. Control: add `--league-nets all
--peddler-share 0`.

## Not done, and next steps

1. **A direct capture metric.** The cheap-take rate per decision is a coarse signal. Count cheap containers offered at the
   start of a seat's turn against the ones bought, and the money left on the table.
2. **Memory ablation.** Evaluate exp_M with the memory inputs zeroed, and repeat with matched length and 3 seeds.
3. **A binding constraint.** Measure the floor against the strong nets (anchor win share), not the easy training league.
4. **Memory targets.** Test adaptation directly: an opponent that switches from cheap to dear mid-game.
5. **Mixed populations in the league.** PSRO-style evaluation with the archetypes as members, using these three pools.
6. **Human play.** Not run. Needed: a log of a person against Dealer, Magnate and Consortium (what they could buy at
   which price, and whether they did), which the UI session can record with the same metric definitions.
7. **Seeds.** One training seed per arm. Repeat A and B with 3 seeds before concluding anything about H1.
8. **Peddler v2.** Shipped (Round 5). Left: a guard for cycles that never repeat an input (2.3% of 3-seat tables with two
   Solitaires still stall), a control run of Peddler v1 trained 20 more minutes in the old league, a second seed, a league
   with Dealer seated (does the edge over Magnate and Consortium hold, and does Dealer fall below an even share?), and a
   rating against Magnate on the ladder's scale.
9. **Multi-container purchases.** Test whether the nets take a second container when a harbor is stocked (positions with
   2+ on the shelf), and run the with/without table rules comparison.
10. **The new league.** Train in it (command in [The league after Round 5](#the-league-after-round-5)) and in the old one
    (`--league-nets all --peddler-share 0`) with the same recipe, seed and budget, then compare win share and interaction
    on the `interact` pools, not the ladder ELO.

# Milestone: how much can a new agent gain by deviating from the Magnate / run2 population, and how reliably can that be measured?

This is a measurement of the deviation gain of one mixture of opponents. It is not a stronger checkpoint and it is not a
claim that anything is a Nash equilibrium. Definitions (utility, stalls, modes, seeds) are in
[`docs/equilibrium_spec.md`](../../docs/equilibrium_spec.md).

## Answer

* **Population sigma** = run2, Magnate, exploit_run2 with equal weights, Standard length, 3, 4 and 5 players, every other
  seat drawing its strategy iid from sigma. Its value is 0.326 / 0.255 / 0.203 at 3 / 4 / 5 players (mean 0.261; an equal
  share is 1/n).
* **The best deviation found raises the expected utility by +0.091 [+0.067, +0.115]** (mean over player counts, 95%
  seed-clustered interval; 150 held-out seeds per player count, 23,400 games, 2 stalled). That is about a third more than
  sigma's own value. The deviation is a 25-minute PPO fine-tune of the `defense` network (BR seed 1), deployed in the
  repetition-guarded greedy mode. Sum of the seats' gains (NashConv): **+0.368 [+0.274, +0.464]**.
* Because the oracle is approximate, **this is a lower bound** on sigma's exploitability. The true deviation gain is at
  least this large; how much larger is unknown (section "Oracle quality").
* **Only about half of it needs new training.** Pure Magnate alone deviates by +0.048 [+0.031, +0.066]; the trained
  response adds +0.042 [+0.013, +0.072] over Magnate for seed 1 and +0.015 [-0.015, +0.046] for seed 2 (not
  distinguishable from zero).
* **Reliability.** One 150-seed study pins a gain down to about +-0.025, the paired comparison of two responses to about
  +-0.03, and the two BR seeds are not separable (+0.027 [-0.005, +0.058]). Picking the better of the two on the
  60-seed dev study picked the wrong one. A measurement that can promise +-0.01 needs about 900 seeds per player count.

Allowed claim: *relative to this tested policy population, these game configurations and a 25-minute PPO best-response
search, a unilateral deviation improves the deviating player's expected utility by at least +0.09 (95% interval
+0.07 to +0.12).* Nothing follows about other populations, game lengths or human opponents.

## Setup

| | |
|---|---|
| utility | canonical: a winner takes 1/\|W\|, a stalled game (2,500 engine actions) pays 0 to everybody |
| population | `populations/baselines` (hash `72d8e2385e8fe4a5`); profile P0 = run2, Magnate, exploit_run2; BROAD = P0 + control, defense, merchant |
| oracle | PPO from an initial network, 256 parallel games, Adam 3e-4, entropy 0.01, 25 min; the learner sits in one seat, the others are drawn from the profile; trained in `sample` mode |
| runs | m1 (init `defense`, seed 1), m2 (`defense`, seed 2), m3 (`run1`, seed 3), all against P0; m4 (`defense`, seed 4) against BROAD |
| modes | `sample` (as trained) and `guarded` (greedy plus the repetition guard, the Magnate deployment) |
| dev study | 60 seeds per player count, seed block `dev` offset 100, every seat, all pure strategies as reference |
| holdout study | 150 seeds per player count, block `holdout`, looked at once, hashes in [`reports/holdout_log.jsonl`](../holdout_log.jsonl) |

Turn order is balanced by design: the engine seat is not the turn position and the first player is drawn per game.

```
python -m training.br train --name m1_defense_s1 --init populations/baselines/defense_run2_vs_exploiter_30m.pt \
    --profile run2,magnate,exploit_run2 --utility canonical --seed 1 --budget-minutes 25 --eval-minutes 5 --eval-seeds 25
python -m training.br eval --profile run2,magnate,exploit_run2 --block holdout --seeds 150 --all-focals \
    --modes sample,guarded --candidates m1_defense_s1=runs/br/m1_defense_s1/final.pt,... --out reports/br/milestone/holdout_p0
```

Each study's json records its policies and the code version; the [Reproduction](#reproduction) section lists the
commands. The frozen responses are in [`populations/milestone/`](../../populations/milestone) with their `SHA256SUMS`.

## Results on the holdout seeds (profile P0)

Gain = utility of the focal strategy minus sigma's value at the same tables; intervals are 95% seed-clustered
bootstrap intervals of the paired difference ([`holdout_p0.md`](milestone/holdout_p0.md)).

| focal | 3 players | 4 players | 5 players | mean over player counts | NashConv |
|---|---|---|---|---|---|
| **m1 / guarded** | +0.086 | +0.086 | +0.101 | **+0.091 [+0.067, +0.115]** | +0.368 [+0.274, +0.464] |
| m2 / guarded | +0.076 | +0.031 | +0.085 | +0.064 [+0.041, +0.089] | +0.258 [+0.165, +0.359] |
| m1 / sample | +0.059 | +0.024 | +0.045 | +0.043 [+0.018, +0.067] | +0.167 [+0.072, +0.259] |
| m2 / sample | +0.059 | -0.008 | +0.042 | +0.031 [+0.009, +0.055] | +0.118 [+0.031, +0.214] |
| m3 / guarded (init `run1`) | -0.142 | -0.066 | -0.029 | -0.079 [-0.098, -0.058] | -0.278 |
| m3 / sample (init `run1`) | -0.131 | -0.099 | -0.095 | -0.109 [-0.128, -0.089] | -0.422 |
| *pure Magnate* | +0.038 | +0.041 | +0.067 | +0.048 [+0.031, +0.066] | +0.203 [+0.137, +0.268] |
| *pure control* | | | | +0.021 [-0.003, +0.044] | |
| *pure exploit_run2* | | | | +0.006 [-0.009, +0.024] | |
| *pure defense* | | | | -0.010 [-0.032, +0.013] | |
| *pure run2* | | | | -0.055 [-0.072, -0.038] | |
| *pure run1, merchant* | | | | -0.185, -0.261 | |

NashConv is the sum of the seats' gains (n times the mean gain at table size n, averaged over n). A trained candidate
is one approximate deviation, so each row is a lower bound; the pure strategies are exact evaluations of fixed policies
and are lower bounds too.

Choosing the best row is optimistic, so the choice was re-measured with the split-half rule of `br_report select`
([`holdout_p0_selection.md`](milestone/holdout_p0_selection.md)): m1/guarded is picked on both halves of the seeds and
measures **+0.091 [+0.066, +0.115]** on the half it was not picked on; among the population's pure strategies the pick
is Magnate, +0.048 [+0.031, +0.065].

### Where the gain comes from

Paired differences on the same seeds and opponents (mean over player counts;
[`holdout_p0_pairs.md`](milestone/holdout_p0_pairs.md), [`holdout_p0_pairs_init.md`](milestone/holdout_p0_pairs_init.md)):

| comparison | difference |
|---|---|
| guard alone: Magnate - defense | +0.058 [+0.030, +0.088] |
| training alone (sample mode): m1 - defense / m2 - defense | +0.053 [+0.026, +0.080] / +0.041 [+0.013, +0.071] |
| guard on the trained weights: guarded - sample for m1 / m2 / m3 | +0.048 [+0.018, +0.079] / +0.033 [+0.006, +0.060] / +0.030 [+0.009, +0.052] |
| trained and guarded against the untrained deployment: m1/guarded - Magnate, m2/guarded - Magnate | +0.042 [+0.013, +0.072], +0.015 [-0.015, +0.046] |
| BR seed: m1 - m2, sample / guarded | +0.012 [-0.020, +0.042] / +0.027 [-0.005, +0.058] |
| initial network: m1 - m3, sample / guarded | +0.151 [+0.124, +0.179] / +0.170 [+0.144, +0.196] |
| training from `run1`: m3/sample - run1 | +0.076 [+0.056, +0.098] |

* The `defense` network played as trained is neutral against sigma (-0.010); the repetition guard is worth +0.058 and
  training on top of that +0.04 to +0.05. The oracle trains in `sample` mode and is deployed greedily with the guard,
  so part of every "guarded" gain is a mode effect that the oracle never optimised.
* The oracle is local. From `run1` it improves its own start by +0.076 and is still 0.08 to 0.11 below sigma's value, so a
  25-minute search is far from a global best response when started from a poor network. The initial network has to be
  chosen with care (PSRO starts from the population strategy with the highest value against sigma).

## How reliably can it be measured?

1. **Sampling error.** Half-widths predicted by the per-seed variance of the holdout study, for the gain of m1/guarded
   (the m1 and m2 candidates agree within 0.002, the m3 candidates are 10-20% narrower), mean over player counts:

   | seeds per player count | 5 | 10 | 25 | 50 | 100 | 150 |
   |---|---|---|---|---|---|---|
   | 95% half-width | +-0.135 | +-0.096 | +-0.060 | +-0.043 | +-0.030 | +-0.025 |

   The half-width falls with the square root of the seeds, so +-0.01 needs about 900 seeds per player count
   (about 140k games for the 13 focals of this study, far fewer for one candidate). The periodic 25-seed dev studies of
   the trainer are accordingly only a progress indicator.
2. **Choosing among candidates.** On the 60-seed dev studies the two BR seeds ranked m2/guarded (+0.098) above
   m1/guarded (+0.069); on the holdout the order reversed (+0.064 vs +0.091). Both differences are inside the dev interval
   (+-0.04), so a dev study cannot choose between seeds of this quality. The paired holdout difference between the seeds
   is +0.027 [-0.005, +0.058]. Use `br_report select` and report the holdout value of the pick.
3. **Oracle seeds.** The two seeds differ by about 0.03; a claim about "the" best response should be based on at least
   two or three seeds and on the best of them confirmed on fresh seeds.
4. **Oracle quality (is the budget enough?).** Training utility of the learner against the training mixture
   (canonical, `sample` mode, mean over the iterations in each window with a 95% interval of the iteration means;
   source `runs/br/*/metrics.jsonl`):

   | run | 0-5 min | 5-10 | 10-15 | 15-20 | 20-25 |
   |---|---|---|---|---|---|
   | m1 (defense, P0) | 0.262 +-0.005 | 0.280 +-0.005 | 0.282 +-0.005 | 0.299 +-0.005 | 0.311 +-0.006 |
   | m2 (defense, P0) | 0.270 +-0.006 | 0.283 +-0.006 | 0.291 +-0.005 | 0.301 +-0.006 | 0.300 +-0.006 |
   | m3 (run1, P0) | 0.092 +-0.003 | 0.110 +-0.004 | 0.110 +-0.004 | 0.126 +-0.004 | 0.146 +-0.004 |
   | m4 (defense, BROAD) | 0.301 +-0.006 | 0.286 +-0.005 | 0.277 +-0.007 | 0.286 +-0.006 | 0.284 +-0.006 |

   Two of the three P0 runs (m1, m3) are still climbing in the last window and m2 has flattened (0.301, 0.300); the
   25-minute budget ends before convergence for at least some seeds, so a longer budget would probably measure a
   larger gain and the numbers above are probably conservative. The periodic dev gains of the weights saved during a
   run ([`runs.md`](milestone/runs.md)) are consistent with that but too noisy to show it (each point is +-0.04 to
   +-0.06); the table's `recovery` column is a ratio of two noisy gains (81%, 227%, -183%, 62%) and should not be
   quoted.
5. **The broad profile exposes a failure of the oracle.** Against BROAD (the six strategies, uniformly) the same
   procedure did not improve the network: the training utility fell from 0.301 to 0.277-0.286 over the run, and on
   the holdout the trained weights in `sample` mode are **below their init**:
   m4/sample - defense = -0.037 [-0.064, -0.010]. The guarded m4 gain (+0.053 [+0.032, +0.074], NashConv
   +0.210 [+0.128, +0.290]) is below pure Magnate's +0.118 [+0.099, +0.135] on the same seeds (m4/guarded - Magnate =
   -0.064 [-0.094, -0.036]); value of BROAD: 0.332 / 0.259 / 0.196
   ([`holdout_broad.md`](milestone/holdout_broad.md), [`holdout_broad_pairs.md`](milestone/holdout_broad_pairs.md)).
   So for a profile on which the init is already strong, the oracle at lr 3e-4 can be worse than doing nothing, and
   its gain then reflects only the guard. The honest lower bound is the best of the trained responses and the pure
   strategies: the meta-solver already contains the pure strategies (sigma's gain inside the population is 0), the PSRO
   loop admits a response only when its own gain over sigma is clearly positive, and the final audit applies
   `br_report select` to the trained candidates and the pure strategies together.
6. **Stalls.** 2 of 23,400 holdout games (P0) and 5 of 16,200 (BROAD) hit the 2,500-action horizon; the dev studies had
   1, 0, 2 and 9 per 12,240 games (m1 to m4). Stalls pay 0, so they cost the player that stalled as much as everyone else.
7. **Turn position.** Per-position gains differ (m1/sample at 4 players: +0.112, +0.039, -0.029, -0.026 from the first
   mover to the last, each +-0.07), consistent with the measured first-player advantage (fair share 1.18 at 3 players).
   Every study is balanced over positions, and no per-position claim is made.

## What this does and does not show

* It shows that the population {run2, Magnate, exploit_run2} is exploitable by a freshly trained agent: it is not
  an equilibrium of the game, and the lower bound on its NashConv is about 0.37 (0.27 to 0.46 with the interval).
* It does not show that the best response found is a best response, that Magnate is close to an equilibrium
  (its +0.048 deviation gain is against a mixture, and the broad profile says that it is the strongest single strategy
  of the six), or that results carry over to other game lengths (Standard only here; cross-play shows the same order
  of the six strategies at Short and Extended length).
* It cannot say how much of the +0.042 over Magnate is real novelty of the oracle: it is significant for one seed and
  not for the other.

## Reproduction

* Dev studies: [`dev_m1_defense_s1`](milestone/dev_m1_defense_s1.md), [`dev_m2_defense_s2`](milestone/dev_m2_defense_s2.md),
  [`dev_m3_run1_s3`](milestone/dev_m3_run1_s3.md), [`dev_m4_broad_s4`](milestone/dev_m4_broad_s4.md) (each with its json).
  Command: `br eval --profile <P0 or BROAD> --candidates <label=path,...> --modes sample,guarded --block dev --seeds 60
  --seed-offset 100 --all-focals --out reports/br/milestone/dev_<run>`, the candidates being the checkpoints saved
  during the run and its `final.pt`.
* Holdout studies: `br eval --profile <P0> --block holdout --seeds 150 --all-focals --modes sample,guarded
  --candidates m1_defense_s1=...,m2_defense_s2=...,m3_run1_s3=... --out reports/br/milestone/holdout_p0` and the same
  for BROAD with `m4_broad_s4` (hashes of every candidate in `reports/holdout_log.jsonl`).
* Tables: `br_report runs`, `br_report pairs`, `br_report select`, all reading the studies' json files.
* Frozen responses (read-only) and their SHA-256:

  | file | SHA-256 |
  |---|---|
  | `populations/milestone/m1_defense_s1.pt` | `cd8f3985ed05d908217345c455eca7339d37e68013012b597a51e7b63d3d319d` |
  | `populations/milestone/m2_defense_s2.pt` | `4f9426cb72d30a6b1ccfb96192c53259081d152a9397b1b24ce48626eb5f2e62` |
  | `populations/milestone/m3_run1_s3.pt` | `e982b52276ac4557a49edccef171481026fcd6f4d21c2ea98bdf8e088a68312e` |
  | `populations/milestone/m4_broad_s4.pt` | `64ad9cf27767479e2b22ed7b23312a4354a284d195eace992ba0d061b40400d2` |

# Equilibrium specification

This document fixes what the equilibrium tooling measures, how, and what may be claimed from it. It is
normative for [`evalcore.py`](../training/evalcore.py), [`population.py`](../training/population.py),
[`crossplay.py`](../training/crossplay.py), [`br.py`](../training/br.py),
[`metagame.py`](../training/metagame.py), [`psro.py`](../training/psro.py) and everything under
`reports/`. A definition changes only by an entry in section 12, never silently.

## 1. Purpose and permitted claims

Until now a net was judged by its ELO against scripted bots and earlier nets. ELO says how well a net
beats a particular field. It says nothing about how much a *different* opponent could gain against it,
and the question of this project is the second one: **how much can a player improve its expected utility
by unilaterally deviating, and how reliably can that be measured?**

Permitted wording, and no stronger:

> An empirically estimated approximate equilibrium relative to the tested policy population, game
> configurations and best-response search budget.

A statement of that form names the population, table sizes, game length, strategy modes, utility,
best-response budget, seed block and interval. Not permitted:

* calling a checkpoint, a meta-strategy or a restricted-game solution a Nash equilibrium of ContainerAI
  ("unexploitable", "optimal", "converged to Nash");
* offering an ELO, a win rate against the training field or a stall-free record as evidence of
  equilibrium;
* reading "no deviation found" as "no deviation exists", or a trained candidate's gain as *the*
  exploitability (it is a lower bound, section 7);
* pooling strategy modes, table sizes or game lengths into one number.

## 2. The game

ContainerAI has 3 to 5 players and imperfect information: other players' cash and scoring cards stay
hidden until the game is over and bids are face down (README, "Rules decisions"). Chance enters through
the seed (deals and markets). Three lengths exist (`Short`, `Standard`, `Extended`); `Standard` is the
shipped default and every headline number is `Standard`, with `Short` and `Extended` reported separately.

The nets observe the number of players, so **every table size n ∈ {3, 4, 5} is its own game** with its
own meta-strategy. Nothing is averaged over n except where a report says "mean over n" (equal weights,
a declared and arbitrary choice).

## 3. Strategies

### 3.1 Memoryless behavioural policies

A strategy maps the current observation and the offered action list to a distribution over actions. The
nets are feed-forward on the encoding of [`features.rs`](../crates/container-bots/src/features.rs), which sees
only the `Observation`: there is no recurrent state and no history. Hiding was audited and is correct
(others' cash and scoring cards are not features).

So the strategy space is the memoryless behaviour policies over the abstraction below. It is a subset of
the strategies of the extensive game, and a best response *within it* can only understate the best
response in the full game. Every deviation gain in this project is a **lower bound** for that reason too.

### 3.2 Action abstraction

At most 48 actions are offered per decision (`MAX_ACTIONS`; the Rust repetition guard keeps its tried
set in a `u64`). Above 48 the list is thinned to indices `i·(n−1)/47`, so `Bid(0)`, `Bid(cash)`,
`DoneBidding` and `RequestLoan` always survive. The audit of 7 policies × 300 games
([`reports/audit/summary.md`](../reports/audit/summary.md)):

* decisions over the cap are 2.0–3.9% (deployed Magnate 3.86%), only in the `bid` and `bank_bid_cash`
  phases, with cash at least 47; only `Bid` and `BidCash` are ever dropped;
* at thinned decisions the argmax of the full set is dropped in 7–9% of cases (`exploit_run2`: 2.6%) and
  the softmax mass on dropped actions averages 7–8% (`exploit_run2`: 2.7%);
* the nets were trained on the thinned list, so those figures describe smoothness, not a measured loss.

The best-response oracle uses the same abstraction, so exploitability found with it is again a lower
bound. `RequestLoan` while bidding or deciding is legal about 41,000 times per net and is chosen **never**
by any net. Three fixed rules that force it were probed without training and none gained (section 9.3); a
trained use of the action remains unexplored (section 11).

### 3.3 Modes

A strategy is a pair *(net, mode)*; modes are never pooled.

| mode | play | use |
|---|---|---|
| `sample` | draw from the softmax (the way nets are trained) | training opponents, best-response candidates |
| `greedy` | arg-max | only to measure nontermination of unguarded play |
| `guarded` | arg-max plus the repetition guard of the deployed bot | what ships |

The guard (`selfplay.Guard`, a port of the Rust `Neural::best_untried`) forbids a greedy net to repeat an
action from the same exact input (state and offered actions) until it has tried all others. Against the
Rust bot on 600 all-Magnate games: 599 identical scores and 600 identical winners (one float near-tie).
It overrides the arg-max in 0.1–0.2% of decisions.

**Magnate** is the net of `defense_run2_vs_exploiter_30m` in `guarded` mode; its weights are those of
`crates/container-bots/weights/magnate.bin`. **Merchant** is the scripted heuristic of `container-bots`.

## 4. Utility and nontermination

Let `W` be the winners of a finished game as the engine decides them (most cash; ties by containers in
the factory district; remaining ties share the win). Seat *i* receives

* `U_i = 1/|W|` if *i* ∈ `W`, otherwise `0`.

An exchangeable table therefore pays `1/n` on average. This **winner share**
(`evalcore.winner_share`) is the only utility used for deviation gains, meta-games and acceptance
decisions. Secondary metrics are reported but never optimised or selected on: *rank share* (share of
opponents beaten on score, ties half), *score margin* (own score minus the opponents' mean; undefined for
stalled games) and ELO (continuity with older reports only). The trainer's shaped reward
(0.6 · rank share + 0.4 · tanh margin) is a training surrogate and is labelled so wherever it is used.

**Nontermination.** The base rules do not force progress. A game that has not ended after
**H = 2,500 engine actions** is *stalled* and every seat receives 0; a game that ends exactly at H is
finished. One horizon is shared by the training environment, the Python evaluation and the Rust arena
(`STALL_HORIZON`). The engine's deadlock rule (everyone broke, only idle moves) is an ordinary ending,
not a stall. The longest finished game in the audit was 1,923 actions.

Stalling never pays more than losing and costs a leader everything, but a seat that cannot win is
indifferent to it, so a policy could *spoil* games when behind at no cost to itself. For that reason
nothing is dropped: every table, cell and study reports its stall count, and a candidate whose gain
came with a rising stall rate is flagged rather than accepted at face value. Measured stall rates
(Standard length, all compositions of the strategies, seeds from the dev block):

| protocol | games | stalled |
|---|---:|---:|
| deployed modes (`reports/crossplay/deployed_dev`, 7 strategies) | 81,648 | 6 (0.007%) |
| all strategies `sample` (`mode_sample`) | 23,184 | 1 (0.004%) |
| all strategies `greedy`, unguarded (`mode_greedy`) | 23,184 | 390 (**1.68%**) |
| all strategies `guarded` (`mode_guarded`) | 23,184 | 0 |

A stall pays 0 to every seat of the table, so a single greedy policy that loops lowers the stalled
share of every strategy that happens to be seated with it. Rates by strategy are in the reports.

**Is H = 2,500 enough for the other lengths?** Yes: in the deployed-mode cross-plays at Short and
Extended length (`deployed_dev_short`, `deployed_dev_extended`, 6 seeds per composition, 20,412 games
each) 2 games stalled (0.01%) in each. The mean game takes 319 / 539 / 725 actions at Short and
467 / 869 / 1,223 at Extended (3 / 4 / 5 players), against 421 / 738 / 1,008 at Standard. At 5 players
the 99th percentile is 902 (Short), 1,227 (Standard) and 1,490 (Extended) actions, so H is at least 1.6
times the 99th percentile of the longest configuration.

## 5. Symmetric meta-games

Fix a table size n and a population of m strategies. A *composition* is a multiset of n strategies. By
rotating all seats and averaging the seats a strategy occupies, the mean winner share of each distinct
member of every composition is estimated by simulation (a **cell**).

**Turn order.** The engine draws the first player of every game at random from the game seed, so the
engine seat index carries no turn-order meaning (seats are exchangeable draws, including the endowment).
One seed is one deal and one first player, played in all `n` rotations of an arrangement, so every
strategy of a composition sits in every turn position equally often and the position effect is averaged
out of the cells, not modelled. It is not negligible: in the Standard cross-play the fair share of the
first player (n × winner share, 1 = equal) is 1.18 [1.05, 1.30] at 3 players, 1.10 [0.99, 1.20] at 4 and
1.05 [0.98, 1.12] at 5, and the second player at 3 players is at 0.83 [0.74, 0.91]
(`reports/crossplay/deployed_dev/report.md`, "By turn position"; the 6-seed Short and Extended runs are
too noisy to say). The meta-game, and every equilibrium claim, is therefore about the game *before the
first player is drawn*; strategies can still condition on their turn position, because the feature
encoder shows the first player's seat relative to the viewer (`features.rs`, `out.seat(obs, obs.first_player)`).

For a mixed strategy σ over the population, with n−1 opponents drawn iid from σ:

* `u_n(s | σ)` is the expected utility of strategy *s*, computed exactly from the cells (multinomial
  weights over compositions);
* `value_n(σ) = Σ_s σ(s) · u_n(s | σ)`;
* **deviation gain** `gain_n(σ) = max_s u_n(s | σ) − value_n(σ)`; "everyone plays σ" is an
  `gain_n(σ)`-Nash equilibrium of the n-player game restricted to the population;
* **NashConv** `= n · gain_n(σ)`: the sum over the n seats of their best-response gains, equal for
  every seat by symmetry; **max individual regret** is therefore `gain_n(σ)` itself;
* **restricted-game Nash regret** is `gain_n(σ)` with *s* ranging over the population only; it is 0
  for an exact solution of the empirical game and is meaningful only against the bootstrap noise floor
  below.

An equilibrium of the empirical game is not an equilibrium of the real game. Every solved meta-game
therefore reports a **noise floor**: the cells resample their seeds with replacement (each cell on
its own, which ignores the positive correlation that shared deals give and so is conservative), the gain
of the *fixed* σ is re-measured in each resampled game, and the mean and 95th percentile are shown.
A restricted-game gain below that floor cannot be told from sampling error.

## 6. Meta-solvers and their labels

[`metagame.py`](../training/metagame.py) runs four solvers, and each output carries the concept it
actually targets:

| solver | targets | label |
|---|---|---|
| replicator dynamics | Nash | "Nash candidate" only if the measured gain is ≤ 1e-6, otherwise "not verified" |
| NashConv minimiser (multi-start Adam on a smooth regret surrogate, Newton clean-up on the support) | Nash | same as above |
| support enumeration (m ≤ 14; run by `solve_all` for m ≤ 10) | Nash | "Nash equilibrium (support enumeration, verified …)" for roots with gain ≤ 1e-7 |
| regret matching | coarse correlated equilibrium | "CCE" — **never** called Nash; its average strategy has no small-gain guarantee |

The meta-strategy used downstream is chosen by `psro.choose`: among the outputs whose concept starts with
"Nash" the one of **largest entropy** (it keeps opponents diverse), otherwise the output of smallest gain,
flagged "not a Nash equilibrium". Whatever is chosen is re-verified by `metagame.verify_regret`, which
shares no code with the solvers (exact enumeration of compositions, or Monte Carlo above a size limit);
the two gains must agree to 1e-6 or the run aborts.

The restricted game can have several equilibria (the baseline 4-player game has eight); all are listed
in the report, and the choice among them is a rule above, not a finding.

## 7. The best-response oracle

[`br.py`](../training/br.py) trains a fresh learner by PPO against opponents drawn iid from a profile
(σ per table size, optionally smoothed towards the uniform mixture), one learner seat in a random
position. The objective is the canonical winner share or, labelled, the shaped surrogate.

Its quality is measured, not assumed. On profiles whose deviation gains are known from cross-play (for
example, `run2` against a field of `run1`) the trainer must recover them; see
`reports/br/validation/summary.md`. A candidate's gain is a **lower bound** on the true deviation gain: the
oracle is an approximate best response, restricted to memoryless policies over the 48-action abstraction,
trained for a stated budget.

Measured so far (60 dev seeds per player count, 12 minutes of training, mean over 3, 4 and 5 players, with
the 95% seed-clustered interval): trained from the weights of `run1` against a field of `run1`, the response
gains +0.11 to +0.19 over the value of the profile, which is 67–77% of the best pure strategy of the
population against the same field (`magnate` or `exploit_run2`, +0.23 to +0.28; a pure strategy needs no
training, so it is the gain that is known to exist). Trained from `run2` against `run2`, the 12-minute response
gains +0.11 (sampled) to +0.17 (guarded) against +0.16 for `exploit_run2`, a 30-minute PPO exploiter. The
canonical objective learns as fast as the shaped one. The same weights differ by about 0.06 between the
sampled and the guarded mode (up to 0.1 at one table size), which is why both are always evaluated. Training time, seeds and
initialisations are varied in the milestone experiment (`reports/br/milestone.md`).

**Milestone** (holdout block, 150 seeds per player count, 25-minute oracles). Against σ = run2, Magnate and
exploit_run2 at equal weights, the best trained response (the `defense` weights fine-tuned, deployed with
the guard) gains **+0.091 [+0.067, +0.115]** over the value of σ (NashConv +0.368 [+0.274, +0.464]); pure Magnate
gains +0.048 [+0.031, +0.066], so about half of the gain needs no new training. Two BR seeds differ by +0.027
[−0.005, +0.058] and were ranked the other way round by the 60-seed dev study. From the weights of `run1` the
oracle is still 0.08–0.11 below σ's value after 25 minutes. Against a broad profile of all six strategies the
oracle ended **below its initialisation** (m4/sample − defense = −0.037 [−0.064, −0.010]): when the
initialisation is already strong, a constant learning rate of 3e-4 can undo it. The training utility of the
runs against σ was still rising after 25 minutes in two of three runs. The consequence for every claim in
this specification: the oracle's gain is a lower bound, it is probably conservative against a profile that
the init does not already handle, and it is not informative against one that it does. The interval
half-width is ±0.025 at 150 seeds per player count, ±0.030 at 100, ±0.043 at 50, ±0.060 at 25.

**Deviation study.** The gain of a candidate *c* over σ_n is `u_n(c | σ_n) − value_n(σ_n)` with the
opponents' draws *paired*: for every (table size, seed, learner seat) one draw of opponents is shared by
all focals (candidate and every pure strategy of the population), so differences are measured under
common random numbers. Intervals are seed-clustered percentile bootstraps (95%). "Best pure deviation in
the population" is chosen on the same seeds and is optimistic; the candidate is never selected among
checkpoints (`final.pt` is always used) on the seeds it is judged on.

**Quoting the best of several.** The best of k focals stands above the truth even when they are all equal:
about 1.6 standard errors for k = 12 (`test_picking_the_best_focal_is_measured_on_seeds_that_did_not_pick_it`
simulates it). A gain in these studies has a standard error of about 0.02, so the optimism is about 0.03,
comparable to the gains themselves. Wherever a number is the best of several (the best pure deviation,
the best response found by an iteration, the largest gain of the final audit)
`python -m training.br_report select --study …` also reports the gain of the focal that looks best on the
even seeds, measured on the odd ones, and the other way round, pooled into one interval. That number
carries no selection bias and is the one to quote as a lower bound.

## 8. Seeds, statistics, reproducibility

* **Seed blocks** (`game_seed(block, n, length, index)`), disjoint from each other and from the 2^40 range
  the trainer draws from: `DEV` for development and selection, `HOLDOUT` for final claims, `AUDIT` for
  audits. Every holdout look is appended to `reports/holdout_log.jsonl`; a block is not reused for
  selection after a claim has been made on it.
* **Common random numbers.** The same seed index is shared by all compositions and all focals.
* **Clusters.** The unit of resampling is the seed, never the game or the seat.
* Cross-play reports give the interval and the number of games, stalls and seeds of every cell.
  `br.reliability` gives the interval half-width as a function of the number of seeds, so the number
  of seeds of a decision is justified, not assumed.
* **Reproducibility.** Every payoff file and study carries the git commit (with a dirty flag), the
  strategies and modes with the sha256 of every checkpoint played, the horizon, the length, the block and
  the seed range; payoff files also carry the hash of the population manifest. `Population.verify`
  re-hashes the checkpoints; checkpoints sit next to their manifest and are read-only.

## 9. The population and PSRO

A **population member** records: checkpoint, sha256, configuration and seed, parent, objective, mode,
and the coverage (player counts and lengths) for which it has cross-play results and best-response
results. A **strategy** is a member in a mode. Members are never removed and never replaced because of
ELO.

One PSRO iteration for the population P (m strategies), implemented in
[`psro.py`](../training/psro.py):

1. **Solve.** For each n, solve the n-player meta-game on P (section 6); report σ_n, the other solver
   outputs, the independent verification, the noise floor and the stall count.
2. **Train.** A fresh best response (initialised from the strategy with the highest mean utility against
   σ) trains against opponents drawn from σ_n smoothed to `(1−γ)·σ_n + γ·uniform` (γ = 0.1 by default, so
   that the learner does not fixate on one narrow support).
3. **Measure.** A paired deviation study on seeds the training never saw (fresh seeds every iteration)
   measures the candidate's gain over the **unsmoothed** σ_n, per n and as the mean over n. The weights are
   played in each of the configured `modes` (default `sample` and `guarded`): a best response is not obliged
   to randomise, and the same weights gain about 0.06 more under the guard against a run1 field (section 7).
   The mode with the larger mean gain represents the candidate; the study keeps the gain of every mode.
4. **Admit** only if the lower 95% bound of the representative mode's mean gain exceeds `min_gain` (0 by
   default); the member is then added as a strategy in that mode. Otherwise the oracle found no meaningful
   improvement within its budget and the iteration records that. Taking the better of two modes is mildly
   optimistic, which is why the final claim is re-measured on `HOLDOUT`.
5. **Extend.** The new member is cross-played against P (only the compositions seating it), what was
   measured about it is written to the results sidecar (below) and the population is solved again.

**Results sidecar.** `results.json` next to the manifest records, per strategy, `coverage` (the player
counts and lengths it was trained and evaluated on), `crossplay[report]` (win and tie rates, margin, game
length, stalls of every cross-play that seated it) and `best_response` (`as_target[report]`: the weight of
the strategy in an attacked profile and the gains of the responses found; `as_candidate[report]`: the gain,
NashConv and largest single-seat gain of the strategy as a trained response, per mode). It lives apart from
the manifest so that adding a result never changes the manifest hash that the payoff files cite.
`python -m training.population annotate MANIFEST --crossplay … --br … [--baselines]` fills it from
reports.

Each iteration reports: the meta-strategy, restricted-game Nash regret and its noise floor, the
candidate's gains (BR deviation gain per mode), NashConv and the largest single-seat gain (the maximum of
the n per-seat estimates of one quantity, since seats are exchangeable, so a noisy overestimate of the
individual regret, which is the mean gain; shown for completeness),
the training-utility curve and the periodic dev studies (oracle quality over time), stall frequency, the
utility of the new member against each member on its own (generalisation, from the new cross-play
cells), and the intervals. A per-iteration "oracle quality" ratio against the best pure deviation is
deliberately not computed: the pure strategies in σ's support have about zero gain over σ by
construction, so the ratio is meaningless there.

**Closing audit** (`python -m training.psro final`, written to `reports/psro/final/`; every look at the
holdout block is logged in `reports/holdout_log.jsonl`). The claim of a run is made on `HOLDOUT` seeds, not
on the dev seeds that decided the admissions:

1. the meta-strategy that the last response was trained against is played against every strategy of the
   final population and against every response that was not admitted (in `sample` and `guarded`); the best of
   these is also measured on the half of the seeds that did not pick it (`br_report select`, section 7);
2. the meta-game of the final population is solved from its cross-play cells and every strategy of the
   population deviates from that solution on other holdout seeds (no new training). The solver reports a
   regret of about 0 on the cells by construction, so what the study finds is the estimation error of the
   cells plus the deviation gain that is really inside the population;
3. each response is compared with the best member of the population it was measured against, paired on the
   same dev seeds (section 9.1).

A response that the rule admitted by a small margin is re-measured against the σ it was trained against on seeds
of its own: `br eval --population populations/psro/manifest.json --profile @reports/psro/iter_NN/profile_eval.json
--block holdout --seed-offset K --all-focals`, with an offset that no earlier look used. That is not part of
`psro final`; the first run did it by hand for iterations 2, 3 and 5 (section 9.2).

A study that an earlier run of the command left in the output directory is reused only if it is the look being
asked for (block, seeds, seed offset, player counts, length, profile, strategies and the hashes of the
responses that were played); otherwise the command stops and names the file, so that a stale study cannot be
reported as the audit of the final population. A rejected response whose checkpoint is missing also stops it.

The solution of the final game is not attacked by a new response unless another iteration is run.

### 9.1 What the first PSRO run showed about the procedure

These are properties of the procedure that change how a report is to be read; the numbers of every
iteration are in `reports/psro/summary.md` and `reports/psro/iter_NN/`.

* **A rejection is not evidence of near-equilibrium.** Iterations 1 and 2 attacked the same σ from the same
  initial network with the same 25-minute budget. With Adam at 3e-4, constant, the oracle gained +0.006
  [−0.019, +0.032] (its training utility did not move from the first to the last fifth of the run and one
  PPO update moved the policy by KL 0.013); with 1e-4 falling linearly to zero it gained +0.098 [+0.070,
  +0.126] (training utility 0.249 → 0.342 over the fifths, KL 0.0035 → 0.0003). The first
  admitted response re-measured on `HOLDOUT` (100 seeds per table size, offset 500, against the σ it was
  trained against) gains +0.120 [+0.093, +0.145] with the guard and +0.093 [+0.068, +0.120] when sampling
  (`reports/psro/final/confirm_iter_02.md`), so the gain did not shrink on seeds the decision never saw. The
  difference between the two iterations is an oracle effect (the game, the opponents, the initial network and the
  budget were the same): how much a response gains is bounded by how well the oracle was trained, so only a gain
  is evidence.
* **The meta-strategy has an estimation error of its own.** σ is solved from cross-play cells of 16 seeds.
  The study re-measures every pure strategy of the population against σ on other seeds, and a member of σ's
  support can then show a gain: against the σ of iteration 4 (support: Magnate, psro02, psro03) psro03
  itself gains +0.023 [+0.007, +0.039], while the restricted game solved on the cells says below 1e-10 and the
  bootstrap noise floor of that solution (mean gain 0.012 to 0.023 by table size, 95th percentile 0.035
  to 0.060) had predicted a gain of this size. A new response must be compared with this, not only with 0: the
  study table lists the best pure deviation in the population, and a response whose gain is no larger than
  that of an existing member has not been shown to be a new strategy.
* **Gains of about 0.03 are at the resolution of the study.** With 100 seeds per table size the half-width of
  the mean gain is about 0.03. Iteration 3 was admitted at +0.032 [+0.005, +0.059] and iteration 4 rejected at
  +0.030 [−0.002, +0.063]: the same estimate, on opposite sides of the rule. Neither is evidence of more than
  "this oracle finds at most a small improvement over σ"; both are lower bounds on σ's exploitability.
* **The most one can say about the last σ** is the holdout deviation gain of the best response found against it
  (section 7's lower bound; `psro final` measures it, section 9). The σ solved after the last admission is
  attacked only by the responses of the iterations that followed it (here iterations 6 and 7, both rejected on
  the dev seeds) and by the policies the closing audit adds; no response is trained against it after that.

### 9.2 Outcome of the first run

The run started from the six baseline strategies (run2, Magnate, control, defense, exploit_run2, Merchant). Each
iteration trained one fresh response for 25 minutes on one GPU, measured it on 100 dev seeds per table size that
the training had not seen (Standard length; 3, 4 and 5 players; canonical utility) and admitted it only if the
lower 95% bound of its gain over σ was above 0. Seven iterations ran and three responses were admitted (psro02,
psro03, psro05), so the final population has nine strategies. The loop ended at its time guard (no iteration
starts after 20:40 on the machine clock), not at a convergence test. A gain is that of the response over the σ it
was trained against (mean over table sizes, the better of the two modes, dev seeds); NashConv is n × gain
averaged over the table sizes; the paired difference compares the response with the best pure member of the
population it was measured against on the same seeds. All of them are lower bounds on the exploitability of σ,
because the oracle is an approximate best response.

| iteration | σ attacked (support at 3 / 4 / 5 players) | gain of the response | NashConv implied | minus the best pure member (paired) | decision |
|---|---|---|---|---|---|
| 1 | Magnate / Magnate, control, defense, exploit_run2 / Magnate | +0.006 [−0.019, +0.032] | −0.005 [−0.102, +0.089] | −0.011 [−0.038, +0.017] | rejected (Adam 3e-4, constant) |
| 2 | the same σ | +0.098 [+0.070, +0.126] | +0.392 [+0.283, +0.499] | +0.080 [+0.051, +0.108] | psro02 |
| 3 | control, psro02 / psro02 / psro02 | +0.032 [+0.005, +0.059] | +0.140 [+0.035, +0.243] | +0.027 [−0.000, +0.055] | psro03 |
| 4 | Magnate, psro02, psro03 / psro02, psro03 / psro02, psro03 | +0.030 [−0.002, +0.063] | +0.117 [+0.002, +0.240] | +0.007 [−0.026, +0.045] | rejected |
| 5 | the same σ, another seed | +0.052 [+0.024, +0.080] | +0.183 [+0.071, +0.291] | +0.029 [−0.003, +0.062] | psro05 |
| 6 | psro03, psro05 / psro03, psro05 / psro03 | +0.015 [−0.011, +0.041] | +0.054 [−0.040, +0.156] | +0.008 [−0.023, +0.038] | rejected |
| 7 | the same σ, another seed | +0.007 [−0.021, +0.036] | +0.048 [−0.058, +0.156] | −0.007 [−0.038, +0.026] | rejected |

Iterations 2 to 7 used Adam at 1e-4 falling linearly to 0 (section 12); `reports/psro/summary.md` has the training
curves and `reports/psro/iter_NN/` the study of each iteration.

* **What the oracle can find against σ fell until it reached the resolution of the study.** The NashConv implied by
  the best response found went from +0.39 against the σ of the baseline population (iteration 2) to about +0.05
  against the σ of the nine-strategy population (iterations 6 and 7), and the intervals of the last two contain 0.
  That is a smaller lower bound; it is not an upper bound (section 9.1).
* **Only iteration 2 is clearly a new strategy.** It gained +0.080 [+0.051, +0.108] over the best pure member of
  the population it joined. Iterations 3 and 5 were admitted by the rule but are +0.027 and +0.029 above their
  best member with intervals that reach 0, and iterations 4, 6 and 7 are +0.007, +0.008 and −0.007.
* **The only seed replication of the oracle** are the pairs of iterations that attacked one σ (4 and 5; 6 and 7).
  The first pair split (+0.030, +0.052), the second did not (+0.015, +0.007).

**Closing audit** (`psro final --seeds 150`, [`reports/psro/final/report.md`](../reports/psro/final/report.md)).
Every study below uses `HOLDOUT` seeds that no decision used; each look is in `reports/holdout_log.jsonl`.

* **The last σ on held-out seeds.** σ of iteration 7 (psro03 and psro05 at 3 and 4 players, psro03 alone at 5) was
  attacked by the nine strategies of the final population and by the sample and guarded versions of the four
  responses that were not admitted (150 seeds per table size at offset 200; 30,600 games, 3 stalled). The largest
  gain is that of the response of iteration 7 played guarded, **+0.027 [+0.005, +0.049]** (NashConv +0.106
  [+0.022, +0.192]; by table size +0.032, +0.019 and +0.029, each interval containing 0). On the dev seeds,
  where it was rejected, the same response measured +0.007 [−0.021, +0.036]. The two measurements differ by
  less than the resolution of the study (section 9.1), so the reading is a gain of order +0.02 that neither
  block pins down. The pick is the best of 17 rows, which is optimistic; the split-half check (pick on one half
  of the seeds, measure on the other) chose the same response both ways and measured +0.027 [+0.004, +0.049].
  The best strategy that needs no training is psro05 with +0.005 [−0.009, +0.018]; psro03 and psro02 gain −0.012
  and −0.024.
* **The solution of the final game.** The nine-strategy restricted game was solved on all of its cross-play cells
  (158,016 games: 9,264 at 3 players, 35,712 at 4, 113,040 at 5): psro03 0.20 and psro05 0.80 at 3 players,
  psro02 0.002, psro03 0.643 and psro05 0.355 at 4, psro03 alone at 5. Support enumeration finds an equilibrium of
  the empirical game at every size and agrees with the NashConv minimiser on the support (the replicator's
  average iterate at 4 players did not converge and is labelled so). The bootstrap noise floor of the gain of
  such a solution is 0.012 to 0.022 on average and 0.051 to 0.070 at the 95th percentile, the order of the gain
  found against it above. It is nearly the σ of iteration 7, because no strategy was admitted after iteration 5.
  A second block (offset 350, no candidates; 16,200 games, 1 stalled) audits the solution itself: the best strategy
  of the population against it is psro05, a member of its own support, with +0.017 [+0.004, +0.031] (NashConv
  +0.066 [+0.004, +0.128]; the split-half check picks it both ways), while psro03 and psro02 gain −0.023 and
  −0.036. The solver's gain of at most 6e-15 is exact on the cells; on fresh seeds the same solution shows the
  estimation error that section 9.1 describes, of the size the noise floor predicted. σ is an equilibrium of the
  empirical game to within about 0.02 of utility, not an exact one.
* **The two marginal admissions, re-measured.** Each response was measured against the σ it was trained
  against on 100 further holdout seeds per table size (`confirm_iter_03.md`, `confirm_iter_05.md`). Iteration 3
  is confirmed: psro03 gains +0.033 [+0.007, +0.060] over σ_3 (dev: +0.032 [+0.005, +0.059]). Iteration 5 is not:
  psro05 gains +0.021 [−0.009, +0.050] over σ_4 (dev: +0.052 [+0.024, +0.080]), and psro03, a member of σ_4's
  support, gains +0.029 [+0.014, +0.044] over the same σ on those seeds. psro05 was not shown to be a strategy of
  its own on held-out seeds. It stays in the population, and in the support of the final solution at 3 and 4
  players, with that label.

**What the run supports.** An empirically estimated approximate equilibrium relative to the tested policy
population (the nine strategies of `populations/psro/manifest.json`), game configurations (Standard length; 3, 4
and 5 players) and best-response search budget (a 25-minute PPO oracle, seven attempts of which four were not
admitted). Against σ of the final population, on seeds that no decision used, the best deviation that was found
gains +0.027 [+0.005, +0.049] of utility per seat (about a tenth of an equal share), and the best member of the
population gains +0.017 [+0.004, +0.031] against the solution of the final game. Both are lower bounds on the
exploitability of σ, not limits on it, and the second is partly σ's own estimation error. It does not support a
statement about Short or Extended length, about strategies
that the oracle cannot reach in 25 minutes (a longer budget, a larger network, history in the state, a different
action set), or about the deployed bot as a stochastic policy (Magnate plays greedily with a guard).

### 9.3 Refining the final game, the loan probe and the deployed bot

The closing audit ended on a solution that its own numbers called weakly identified: its cells had 16 seeds per
composition, the noise floor of its gain was 0.012 to 0.022 on average (0.051 to 0.070 at the 95th percentile), and
the best member of the population gained +0.017 [+0.004, +0.031] against it on seeds it never saw, which was put
down to the solution's estimation error. Section 11 listed more seeds for the cells of the strategies that can be
in the support as the first follow-up. This section records that follow-up, a probe of the `RequestLoan` surface,
one last best response, and the bot that was built from the result. Decisions use dev seeds, claims use holdout
seeds, and every holdout look is in `reports/holdout_log.jsonl`.

* **More seeds where the support is.** 48 further dev seeds (offset 24) were played for every composition of
  psro02, psro03, psro05, Magnate and control at 3, 4 and 5 players, each in the mode of the population
  (`reports/crossplay/refine_dev`: 48,720 games, 1 stalled, 16 minutes). Those cells now hold 64 to 72 seeds; the
  cells that involve run2, defense, exploit_run2 or Merchant keep their 16 or 24. The nine-strategy game was solved
  again from all of them (206,736 games; [`reports/psro/refined_metagame.md`](../reports/psro/refined_metagame.md)).
* **The solution is pure at every table size.** It is psro05 alone at 3 and 4 players and psro03 alone at 5.
  Support enumeration and the NashConv minimiser find it, and so does the replicator at 3 and 4 players; at 5
  players the replicator's average iterate (gain 1.9·10⁻³, seven strategies in its support) is labelled not
  verified and makes no Nash claim. The mixtures of section 9.2
  (psro03 0.20 and psro05 0.80 at 3 players; psro02 0.002, psro03 0.643 and psro05 0.355 at 4) were not identified
  by 16-seed cells. The bootstrap noise floor of the gain of the solution fell from 0.022, 0.019 and 0.012 on
  average (0.070, 0.051 and 0.062 at the 95th percentile) at 3, 4 and 5 players to 0.004, 0.005 and 0.006 (0.029,
  0.033 and 0.025).

  | n | psro02 | psro03 | psro05 | Magnate |
  |---|---:|---:|---:|---:|
  | 3 | 0.266 | 0.305 | 0.333 | 0.253 |
  | 4 | 0.188 | 0.238 | 0.250 | 0.096 |
  | 5 | 0.192 | 0.200 | 0.178 | 0.142 |

  Utility of each strategy against the solution (the other n−1 seats drawn from it; 1/n is an equal share, which
  the strategy of the solution earns by construction).
* **Against other fields.** An equilibrium is the best answer to itself, and the opponents of a bot are not σ.
  `training/fields.py` reads the utility of any mixture against any field of opponents from the cells, with a
  paired bootstrap and no new game ([`reports/psro/refined_fields.md`](../reports/psro/refined_fields.md)). Against
  the uniform mixture of the nine strategies psro05 beats psro03 at 3 players by +0.051 [+0.029, +0.082]. At 4
  and 5 players the paired gaps between psro05, psro03, the mixtures of section 9.2 and a half-and-half mixture
  all have intervals that contain 0 on every field (the largest point estimate is psro03 against control at 4
  players, +0.047 [−0.029, +0.113]), and against Magnate alone psro03 is nominally ahead at 3 players (+0.042
  [−0.065, +0.130]; not significant). The choice at 4 and 5 players is a tie that follows the solver. All of
  these are dev values and the solution was picked on them, so the holdout validation below is the unbiased
  reading.
* **The loan probe** ([`training/loanprobe.py`](../training/loanprobe.py),
  [`reports/loanprobe/dev.md`](../reports/loanprobe/dev.md)). `RequestLoan` is legal in the bid and decide phases
  about 41,000 times per net in 300 games and no net picks it. The probe replaced the pick of psro03 and psro05
  (guarded) by `RequestLoan` where a rule applied and played them against σ on 100 dev seeds per table size
  (offset 1000; 18,000 games, none stalled), paired with the same net unmodified. The rules were fixed before the
  run: `bid:allin` (the pick is a bid of all the net's cash), `bid:any:0.1` and `decide:any:0.1` (a loan instead
  of the pick in 10% of those decisions). `bid:allin` never fired: in 38,092 and 37,465 decisions with a loan
  legal neither net picks a bid of its whole cash. A loan in 10% of the bidding decisions costs −0.079 [−0.103,
  −0.054] (psro03) and −0.079 [−0.100, −0.057] (psro05) of utility per seat, and in 10% of the deciding decisions
  −0.017 [−0.033, −0.000] and −0.004 [−0.021, +0.015]. No rule has a paired lower bound above 0, so none was kept
  and no net was changed. That is a negative result for these three fixed rules, not a proof that no loan policy
  helps.
* **The bot** is described in section 9.4 (it first played the pure solution above, psro05 at 3 and 4 players and psro03 at 5).

### 9.4 The final response, the ten-strategy game and the bot

One more oracle was trained after section 9.3, outside the loop: 50 minutes (twice the loop's), Adam 1e-4 falling
linearly to 0, canonical utility, exploration gamma 0.1, initialised from psro05 (the member that did best against
the refined σ), against that σ. Admission used the loop's rule plus the paired check against the best pure member:

* dev (offset 1700, 100 seeds per table size): guarded gain +0.033 [+0.005, +0.060], sample +0.010
  (`reports/psro/final_br/study.md`);
* holdout (offset 1000, 300 seeds): guarded gain +0.022 [+0.008, +0.035] and the best pure members −0.024 (psro03)
  and −0.002 (psro05) (`holdout_confirm.md`). It was admitted as **psro08** in `guarded` mode. The gain is the
  size of the study's resolution; by table size only 3 players has an interval above 0 (+0.047 [+0.019, +0.076]).

Its cells were simulated against the whole population (`reports/crossplay/final_br_dev`, 110,880 games, 4 stalled,
24 seeds per composition) and the ten-strategy game was solved from all cells (`reports/psro/final_br/metagame.md`).
The solutions are mixtures (3 players psro02 0.42, psro03 0.14, psro08 0.44; 4 players psro05 0.30, psro08 0.70;
5 players psro05 0.06, psro08 0.94) whose noise floor is 0.014-0.019 on average, so the weights are not identified,
and psro02 at 3 players owes its weight to 24-seed cells. What is stable is that psro08 is in the support at every
table size. `training.fields` (`reports/psro/final_br/fields.md`) compares mixtures from the cells: pure psro08 is
within noise of the solved mixture on every field (σ, uniform, Magnate) and of the previous solution, except at 5
players against the uniform field, where it is +0.022 [+0.017, +0.026] ahead. **The bot therefore plays psro08 alone**,
which also avoids a draw that the data cannot weight.

Fresh holdout blocks, nothing tuned on them: against the ten-strategy σ (offset 1400, 150 seeds per table size,
18,000 games, 3 stalled) no strategy gains by more than noise (psro05 +0.019 [−0.002, +0.041], psro08 +0.008
[−0.003, +0.020], psro03 −0.009, psro02 −0.036, Magnate −0.107); against a table of Magnate (offset 1600, 100 seeds,
12,000 games, 0 stalled) psro08 gains +0.134 [+0.110, +0.159] per seat, psro05 +0.115, psro03 +0.118, psro02 +0.093
(`holdout_final_sigma.md`, `holdout_final_magnate.md`). **What this supports:** psro08 is the strongest net measured
for this population and 25-50 minute oracles; it is not shown to be better than psro05 at 3 and 4 players beyond
0.02, and nothing here is a statement about a Nash equilibrium of the game. Not done: two oracle seeds, a
budget beyond 50 minutes, other game lengths for psro08.

## 10. Baselines

Frozen in `populations/baselines/` (read-only checkpoints plus `manifest.json`; the state
dimension 520, action dimension 52, `MAX_ACTIONS` 48 and the sha256 of `features.rs` are in the manifest
header).

| strategy | member | mode | sha256 | origin |
|---|---|---|---|---|
| merchant | scripted | n/a | n/a | Merchant heuristic of `container-bots` |
| run1 | `run1` | sample | `f26f9e29…` | imitation of Merchant, then league PPO (60 min) |
| run2 | `run2` | sample | `c965d6a6…` | league PPO from run1 (30 min) |
| exploit_run2 | `exploit_run2` | sample | `c6b233c0…` | PPO exploiter of greedy run2 (30 min) |
| control | `control_run2_30m` | sample | `6fe0b5d0…` | league PPO from run2, control for the defense run |
| defense | `defense_run2_vs_exploiter_30m` | sample | `0261429a…` | league PPO from run2 with `exploit_run2` in the field |
| magnate | `defense_run2_vs_exploiter_30m` | guarded | `0261429a…` | the deployed bot |

Cross-play of these (`reports/crossplay/deployed_dev`, 24 seeds per composition and table size), the
mode comparison (`mode_*`, 12 seeds) and the other game lengths (`deployed_dev_short`,
`deployed_dev_extended`, 6 seeds) are the reference tables. The order of the strategies by mean winner
share (magnate, control, defense, run2, exploit_run2, run1, merchant) is the same at all three lengths
(point estimates; the Short and Extended cross-plays have a fourth of the seeds).

## 11. Known defects of the previous evaluation, and follow-ups

[`anchors.py`](../training/anchors.py) (and so every ELO in the README before this work):

* `evaluate` and `evaluate_exploit` dropped stalled games while reporting the requested game count;
* the learner plays greedy and unguarded, so a rating is conditioned on the games that did not stall;
* ELO compares pairwise scores rather than winner share and draws opponents uniformly from the ladder.

Repair: both functions now return `completed`, `stalled` and `stall_rate`; ELO semantics are unchanged
for continuity; canonical evaluation lives in `evalcore`.

Follow-ups, deliberately **not** done before the measurements exist (changes to the features invalidate
checkpoints):

* a stronger oracle comes before any larger net: the milestone shows an oracle that is still improving at
  the end of its budget (P0), can undo a strong initialisation (broad profile) and has a seed-to-seed
  spread of about 0.03, and the PSRO run showed that its learning-rate schedule alone moved the gain over one
  σ from +0.006 to +0.098 (section 9.1; `--lr 1e-4 --lr-decay linear` is what the later iterations used).
  Candidates still open, in order: more BR seeds per iteration chosen with `br_report select`, longer
  budgets, training through the guard (the oracle is trained in `sample` mode and deployed greedily with the
  guard);
* the PSRO admission rule compares a response with σ, which carries the estimation error of its cross-play
  cells (section 9.1). A response should also be compared with the best pure member in the same study (paired,
  on the same seeds), and the cells of the strategies in σ's support deserve more seeds than the rest of the
  population (cross-play seeds are 16 per composition, and the cost of an iteration is dominated by the
  cross-play once the population has about 9 strategies). The second half was done once, after the first run,
  for the five strategies that matter most (section 9.3); it would have to be repeated for any later population;
* full legal-action coverage instead of 48;
* history and recurrent features, so that strategies are not memoryless;
* deployment stochasticity (a greedy deployed bot is trivially exploitable by prediction);
* per-length and per-table-size populations and evaluation (Short and Extended are covered by smaller
  cross-plays only);
* the loan-signal surface (`RequestLoan` while bidding is never chosen): forcing it by three fixed rules did not
  gain (section 9.3), but an oracle trained to explore the action has not been tried;
* larger nets, only if the best-response oracle is shown to be capacity-limited.

## 12. Changes

* Initial version.
* 2026-11-11, turn order (section 5). The first version of the cross-play and study reports grouped
  results by engine seat index and called it "by seat". The first player is drawn per game, so the seat index
  is not turn order. Reports now group by turn position (`evalcore.first_seat`); the old field is
  `by_seat` in study files and means the engine seat. No payoff, cell or gain changed (all of them
  average over every seat); `payoffs.json` of all six cross-plays is byte-identical.
* 2026-11-11, quoting the best of several (section 7): `br_report select` and the winner's-curse rule.
  The PSRO admission rule is unchanged.
* 2026-11-11, `crossplay run --block holdout` writes to `reports/holdout_log.jsonl` like `br eval` does.
* 2026-11-11, studies and cross-plays record the sha256 of every checkpoint played in `policies` (section 8
  claimed it; before this change only the holdout log and the manifests carried hashes).
  `sha256_file` moved from `population.py` to `evalcore.py`; `population.sha256_file` is the same function.
* 2026-11-11, `br train --lr-decay {none,linear}` (default `none`, so every earlier run is reproduced
  unchanged) and `psro … --lr-decay`; the learning rate is logged per training iteration and the PSRO
  summary shows the oracle's learning rate.
* 2026-11-11, oracle settings of the PSRO run changed after its first iteration. Iteration 1 used the
  milestone settings (Adam 3e-4, constant) and was rejected (+0.006 [−0.019, +0.032]); its training utility
  was flat from the first to the last fifth of the run (0.227, 0.225) while the policy kept sharpening, and the
  KL of one PPO update was 0.013, as in the milestone run that ended below its initial network. The loop was
  restarted from iteration 2 with `--lr 1e-4 --lr-decay linear` (KL of one update about 0.004). The settings
  of every iteration are in its `record.json` (`train_config`) and in the "oracle" column of
  `reports/psro/summary.md`. The change was made after looking at a dev result, so it is tuning on the dev
  block; admission decisions stay dev-block decisions and the final audit re-measures on `HOLDOUT`.
* 2026-11-11, robustness of the PSRO driver, after an independent code review of `psro.py` and `br.py`
  (none of it changes a number): an iteration pins its settings in `iter_NN/cfg.json` when it starts, and a
  resumed iteration keeps them and prints the flags it ignored; flags given to `psro step|run` are saved to
  `config.json` (the file of the loop that was running then was set by hand to the settings it was started with, lr
  1e-4 falling linearly, because those flags had not been saved); the first seed of a study is taken from the seeds
  of the earlier studies, so a change of `study_seeds` cannot repeat one; the independent check of a
  meta-solution enumerates every ordered opponent tuple up to m^(n-1) = 2·10^6 (it was 2·10^4, which is
  m = 12 at 5 players, beyond which the strict comparison with the solver would have failed on Monte-Carlo
  noise), and falls back to sampling with a tolerance of five standard errors; `admit` replaces a read-only
  checkpoint left by a run that was killed before it saved the manifest; the `metrics.jsonl` of a killed
  trainer is set aside before the rerun trains; `payoffs.json` of a new member carries `manifest_sha256_16`.
  The loop that was running then kept the code it was started with, so its cross-plays carry the manifest path only
  (the checkpoints they played are identified by the sha256 in `meta.policies`); the changes were in force for a
  restart, for `psro solve`, for `psro final` and for `psro report`.
* 2026-11-11, `psro final`, the closing audit of a run, as a subcommand (section 9) rather than a shell recipe:
  it attacks the last meta-strategy with every strategy of the final population and every response that was not
  admitted on `HOLDOUT` seeds, solves the final restricted game and audits its solution on other holdout seeds, and
  tables each response against the best member of the population it was measured against. `psro solve` was split
  into `write_meta` and a thin command so that both write the same report. An independent code review of the
  subcommand changed three things: a study left in the output directory by an earlier run is reused only if it is
  the look being asked for (`kept_study_differs`; checked against a study file written by `br eval`); a rejected
  response whose checkpoint is missing stops the audit instead of being left out; and the report no longer says that
  the gains of the solution are about 0 by construction where no solver reached a Nash equilibrium of the
  simulated game.
* 2026-11-11, the first run was closed: `psro final --seeds 150` (holdout offsets 200 and 350) and, by hand, the
  holdout re-measures of iterations 3 and 5 (offsets 900 and 700) after the loop stopped at its time guard; the
  results are in section 9.2 and `reports/psro/final/`.
* 2026-11-12, tooling for section 9.3 (no earlier number changes): `psro solve` also writes `<out>_profile.json`,
  the solution in the profile format that `br train` and `br eval` read with `--profile @file`, so that a solution is
  attacked without its weights being copied by hand; `training/fields.py` reads the utility of a mixture against a
  field of opponents from the cells of a cross-play (paired bootstrap, no game); `training/loanprobe.py` plays a net
  with its pick replaced by `RequestLoan` where a rule applies (the hook is `adjust` of `selfplay.Roller.collect`
  and of `evalcore.Policy`, `None` by default, so every earlier study is reproduced unchanged).
* 2026-11-12, the cross-play of psro02, psro03, psro05, Magnate and control was extended by 48 dev seeds per
  composition (`reports/crossplay/refine_dev`) and the nine-strategy game was solved again
  (`reports/psro/refined_metagame.md`); the solution is pure at every table size (section 9.3).
* 2026-11-12, the `consortium` bot in `container-bots` (psro08, weights in `crates/container-bots/weights/`), the
  `OPPONENTS` entry that makes it a web opponent, and a test that its games are those of the Python policy.
* 2026-11-12, psro08, its cross-play and the ten-strategy game (section 9.4); holdout offsets 1000-1299, 1400-1549 and
  1600-1699 were looked at (`reports/holdout_log.jsonl`; the first Magnate-field look at 1600 ran once before the
  population flag was fixed and its output was overwritten by the identical rerun).

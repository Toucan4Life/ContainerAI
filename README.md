# ContainerAI

A digital version of the board game **Container**, built to:

1. Play the game through a UI, including against AI opponents.
2. Train AI agents to play it.

## Stack

- **Game engine:** Rust, for fast simulation during training.
- **Training:** Python with PyTorch (CUDA).
- **UI:** React + TypeScript + Vite, with the rules and bots compiled to WebAssembly.

## Development

The project is developed in a devcontainer (Rust, Python 3.12, Node, PyTorch with CUDA).

Requirements on the host:

- Docker
- An NVIDIA GPU with the driver and the [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html)

Open the folder in VS Code and choose **Reopen in Container**. After setup, check the GPU:

```sh
python -c "import torch; print(torch.cuda.is_available())"
```

## Frontend

The UI lives in [`frontend/`](frontend/): React + TypeScript + Vite. You play one human seat against 2–4 computer opponents, from setup to the final scores.

The rules run in the browser. `crates/container-web` wraps the engine and bots in a JSON session and compiles to WASM. The UI renders each snapshot and sends back the id of a legal action. Clickable containers on the board (buying, seizing, Bank cash-lot bids) are the actions you can take.

Requirements: the `wasm32-unknown-unknown` target and `wasm-bindgen-cli` matching the crate version (0.2.130).

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.130
cd frontend
npm install
npm run dev     # builds the WASM first (npm run wasm), then serves on :5173
```

## Engine

The rules engine is the library crate `crates/container-engine` (10th Anniversary rules, 3–5 players).

- `Game::new(GameConfig)`, `legal_actions()`, `apply(action)`, `observation(viewer)` and `results()` are the main entry points. The action model is a flat legal-action list.
- `observation` hides what the viewer cannot know: other players' cash, scoring cards and face-down bids.
- Delivery auctions are simultaneous blind bids: each opponent places one hidden bid, and the bids are revealed together. A bid cannot be revised.
- A table where every player is broke at the loan cap with no containers ends the game, as nothing could progress.

```sh
cargo test -p container-engine
```

## Opponents

The crate `crates/container-bots` holds the computer opponents. An opponent implements `Opponent::choose(&Observation, &[Action]) -> Action`, so it only sees what its seat sees. `OPPONENTS` lists the ones the UI offers, and `create(id, seed)` builds one. A seed gives the same choices every time. Every entry carries an `elo` rating, which the UI shows next to the name. The scripted bots' ratings come from `training/ladder.json`. Magnate is not on the ladder, so its rating is the result of `anchors rate` (see [Exporting a net to the web UI](#exporting-a-net-to-the-web-ui)). Opponents trained with PyTorch are added to the same list once they are rated.

| Id | Name | ELO | Plays |
|---|---|---|---|
| `random` | Random | 0 | Any legal action, chosen at random. |
| `simple` | Simple | 263 | The legal action with the highest fixed priority, and a random one among equals. It produces, loads the ship, delivers to the Island and buys cheap containers, and bids a quarter of its cash in delivery auctions. It takes a loan only at $0, repays one when it holds $25 or more, and never builds, calls the Bank or buys out. |
| `merchant` | Merchant | 932 | Simple's priorities with 32 parameters tuned by evolution strategy in self-play (`arena` crate, `tune` binary): it also builds, borrows and bids by cargo. |
| `broker` | Broker | 2150 | The net from `runs/scratch_constrained/final.pt`, embedded as `weights/broker.bin`, trained from scratch with `--no-same-turn-repay --min-bid 3 --min-trade 45 --scripted-share 0.7`. The bot keeps both rules: it bids at least $3 to open a delivery auction and never repays a loan the turn it takes it. Its rating is the trainer's final eval (1024 games), not `anchors rate`. |
| `magnate` | Magnate | 2280 | The net from `runs/defense_run2_vs_exploiter_30m/final.pt`, embedded as `weights/magnate.bin`. It plays the action it rates best and never repeats itself in an identical position (see [Stalls and memory safety](#stalls-and-memory-safety)). It beats Merchant about 99.9% of the time. |

```sh
cargo test -p container-bots
```

Unoptimised, a net's decision is tens of times slower than in release (about 0.64 ms there). The first debug run of the suite took about 5 minutes on one thread, so a CPU/GPU monitor looked idle and it seemed hung. The workspace `Cargo.toml` therefore builds `container-bots` at `opt-level = 3` even in the dev profile, which brings the suite to about 25 seconds.

`features.rs` encodes a state and its legal actions as vectors (seats relative to the viewer), and `neural.rs` runs a trained policy/value net (`weights.bin`, format `CNTW`) in pure Rust, so the same net runs natively and in the browser's WebAssembly. A fixture exported from Python (`python -m training.make_fixture`) keeps the Rust forward pass equal to the numpy reference.

## Arena and ratings

`crates/container-arena` plays bots against each other at mixed 3-, 4- and 5-seat tables, with a seed per game so runs are reproducible.

```sh
cargo run --release -p container-arena -- --bots random,simple,merchant --games 2000 --players 3,4,5 --out games.jsonl
```

Because Container is a multiplayer game, ratings come from a Bradley-Terry fit over every pairing at every table (`training/elo.py`), anchored at Random = 0, with bootstrap confidence intervals. A rating depends on the field it was measured against, so the ladder anchors are rated at mixed tables of Random, Simple and Merchant. A game that stalls (the engine has no anti-stall rule) has no result to rate. The Rust arena stops it after `STALL_HORIZON` (2,500 actions; `--horizon` changes it) and writes it with `"stalled":true` instead of dropping it. The Python evaluations (`anchors rate`, `duel`, the trainer's final evaluation) leave it out of the rating and report `completed`, `stalled` and `stall_rate` beside it. The rating still never counts a stall as a loss, so a net that stalls often is not penalised in its ELO: read the stall columns, or the utility of [Empirical game-theoretic evaluation](#empirical-game-theoretic-evaluation), which pays nothing for a stalled game.

## Training

`training/` is a PyTorch pipeline. The environment is a Rust cdylib (`crates/container-env`) that steps hundreds of games in parallel behind a C ABI.

```sh
cargo build --release -p container-env
.venv/bin/python -m training.train --name run1 --budget-minutes 30   # budget is adjustable upward
```

A run does imitation of Merchant (DAgger), then PPO league self-play: the learner sits at tables with frozen earlier checkpoints and the scripted bots, rewarded by final rank and score margin. 8% of the budget is kept for the final evaluation and export. Everything is written to `runs/<name>/` (`metrics.jsonl`, `status.json`, checkpoints, and `weights.bin` for the Rust bot). Training rows include `games_per_sec`, the number to watch when optimising.

### Anchors

The scripted bots stop measuring a strong net: beating Merchant 97% of the time cannot be told from 99%. Trained nets therefore join the ladder as fixed, rated opponents.

```sh
.venv/bin/python -m training.anchors add scout runs/run1/final.pt   # rates it against the whole ladder, stores it in training/anchors/
.venv/bin/python -m training.anchors rate runs/run1/final.pt        # rates it the same way without storing it
.venv/bin/python -m training.anchors duel runs/exploit_run2/final.pt run2   # alone against copies of one anchor; stores nothing
.venv/bin/python -m training.anchors list
```

`rate` and `duel` take `--games` (default 3000) and `--seed` (default 7). Rate with two seeds before believing a gap of a few ELO: the same net rated with seeds 7 and 8 at 2,000 games differed by about 12 ELO. `duel` also takes `--sample-target`, which makes the anchor sample from its policy instead of playing its best action.

Evaluation seats the net with two to four opponents drawn from the scripted bots and every anchor (Merchant is always in the field) and rates it by maximum likelihood against their ratings. A run started afterwards also meets the anchors as permanent league opponents. Re-adding a name replaces it and re-rates it against the others; the other anchors are not refitted, so add them in order of strength.

A game that stalls (the engine has no anti-stall rule; the env ends one after 2500 actions) counts as a loss for every seat (reward 0) instead of being dropped, so the policy learns to finish games. `stalled` is logged per iteration and charted. This applies to training only. The evaluations in `training/anchors.py` (`anchors rate`, `anchors duel` and the trainer's checkpoint evaluations) leave stalled games out of the rating instead, so a stall never lowers a rating, but they report how many it was (`completed`, `stalled`, `stall_rate`; `games` is the number requested).

### Exploiters

A best-response exploiter is a net trained only to beat one fixed anchor, to expose that anchor's weaknesses (the "main exploiter" of league training).

```sh
.venv/bin/python -m training.train --name exploit_run2 --exploit run2 --budget-minutes 30
```

Every table seats the learner once (3 to 5 players) and every other seat plays the anchor. The learner starts from the anchor's weights (override with `--init`), skips imitation, keeps no frozen snapshots and runs PPO against that one opponent. `--target-mode greedy` (the default) makes the anchor play its best action, the way the ladder rates it; `sample` makes it sample from its policy as it does in league training.

Each evaluation logs two things. `elo`, `games` and `vs_<opponent>` are the normal ladder evaluation, identical to any other run, so an exploiter is directly comparable with every other net. The `duel_*` keys come from exploit tables (the net in one seat, the anchor greedy in all the others): `duel_vs_<anchor>` is the share of opponents beaten (0.5 for equal play), `duel_win_rate` is the share of games won against `duel_fair_rate` (what equal players would win) and `duel_margin` is the points ahead of the average opponent. The dashboard shows them on an "Exploit duel" card, which only appears for runs that have them. An exploit evaluation plays both sets of games, so it takes about twice as long as a normal one; with a budget of a couple of minutes the final evaluation can overrun the 8% reserve (a 1.5-minute smoke test finished after 103 s). At 30 minutes the reserve is ample.

An exploiter's duel numbers say how well it beats copies of the anchor, not how strong it is: see the results below. Rate it on the whole ladder with `python -m training.anchors rate runs/exploit_run2/final.pt`. `anchors add` registers it, which also makes it a permanent opponent in later runs. `exploit_run2` is registered at 2174.4 ELO and is therefore a permanent opponent for runs started after the control experiment.

### Throughput

`python -m training.profile_iter [--envs 256 --cprofile]` times one self-play iteration (rollout, advantages, PPO update) and prints games per second. On the dev box (12 CPUs, RTX 3080) it took the loop from about 26 to about 130 games/s:

| Change | Why it helped |
|---|---|
| Rolling refill (`Roller`): a finished game is replaced at once and unfinished games carry over | With lockstep batches over half of all steps ran with fewer than 32 of 256 games alive (the long tail); about 2x |
| Action layers run only on real actions (`PolicyNet` compacts padded batches) | Batches were padded to 48 actions while most decisions have under 20; the PPO update went from 1.4 s to 0.2 s |
| Rollout forward in CUDA graphs on static, width-bucketed buffers, Gumbel-max sampling | The loop was launch-bound: four tiny nets per step, a sync in `Categorical` and in `nonzero` |
| One `.cpu()` per step, pinned env buffers, vectorised rollout packaging, TF32 | Fewer host/device round trips |

`test_roller.py` checks the optimised rollout: the logged log-probs and values equal a fresh forward pass, sampling follows the policy, games carry over, and evaluation plays exactly the requested games. More than about 384 envs does not help and the GPU may be shared, so mind its memory.

### Dashboard

```sh
.venv/bin/python -m training.dashboard.server --runs runs --port 8765
```

Open http://127.0.0.1:8765 to follow a run live: ELO over time against the ladder, win rate per scripted bot and anchor, reward, losses, entropy/KL, throughput, budget used, and an overlay to compare runs. Exploiter runs add an "Exploit duel" card. The evaluations table scrolls sideways inside its card when a run logs many columns.

```sh
.venv/bin/python -m pytest training
```

### Training research and operational notes

This section records measured facts from the initial training work so future changes can be
compared against a known baseline. The figures below apply to the development machine used for
these runs (12 CPU threads and an RTX 3080); do not assume they transfer unchanged to another
machine.

#### Current ladder and completed runs

The ratings are field-dependent estimates, not absolute measures of playing strength. They are
anchored at Random = 0 and are only meaningful relative to the opponents used during evaluation.

| Opponent | ELO | Evidence |
|---|---:|---|
| Random | 0 | Fixed rating anchor. |
| Simple | 263 | Scripted-bot arena evaluation. |
| Merchant | 932 | Scripted-bot arena evaluation. |
| `traders` | 1907 | 3,000-game ladder rating; `runs/scratch_traders` (from scratch, against price-sensitive traders). Rated and played unrestricted, without its min-bid and repay rules. |
| `run1` | 2173 | 3,000 anchor-rating games; beats Merchant about 99.9%. |
| `exploit_run2` | 2174 | 3,000-game ladder rating; specialist exploiter of `run2`. |
| `run2` | 2239 | 3,000 anchor-rating games; beats `run1` 59.8%. |
| `consortium` | 2286 | 3,000 anchor-rating games, from `populations/psro/psro08.pt`; plain greedy, without the UI's repetition guard (2.4% stalls). |

The strongest nets so far are not on the ladder: `defense_run2_vs_exploiter_30m` (the UI's Magnate,
about 2280) and `control_run2_30m` (about 2279) are tied; see the control experiment and
[Exporting a net to the web UI](#exporting-a-net-to-the-web-ui).
`run1` used a 60-minute requested budget. The trainable portion completed in 3,316.5 seconds,
covering 798,941 games and 169,924,042 learner decisions. Its final scripted-bot-only evaluation
reported 2106 ELO, but that score is weak evidence because it had effectively saturated the
scripted field.

`run2` resumed from `run1/final.pt`, skipped imitation, used a 30-minute requested budget, and
trained for 1,661.0 seconds: 358,992 games and 58,450,496 decisions. Its final 1,024-game
evaluation beat `run1` 60.7% of the time. The independent 3,000-game anchor rating was 59.8%,
which supports a small, real improvement (roughly 66 ELO), not a large jump.

The corresponding checkpoints are preserved at `runs/run1/`, `runs/run2/`, and
`runs/exploit_run2/`; their evaluated copies are `training/anchors/run1.pt`,
`training/anchors/run2.pt`, and `training/anchors/exploit_run2.pt`. Keep anchor checkpoints and
`training/ladder.json` together: a ladder entry without its checkpoint cannot be used as a league
opponent.

#### How the learner works

* The policy/value net is a feed-forward, hidden-size-256 model. It has a two-layer ReLU state
  trunk, combines the trunk with each action vector to emit one logit per legal action, and has a
  separate value head. The exact implementation is `training/model.py`.
* `features.rs` encodes observations from the current seat's relative perspective. It includes
  visible player holdings, lots, ships, loans, the bank, supply, scoring card, turn/phase data and
  phase-specific data. Hidden information remains hidden: other players' cash, scoring cards and
  face-down bids are not exposed by the engine.
* There can be up to 145 legal actions. Training and inference thin a longer list evenly to at
  most 48 actions. A training game normally has roughly 650--800 engine actions and about 613
  learner decisions with more than one choice.
* Imitation uses Merchant labels with DAgger-style relabelling for the first 17% of the requested
  wall-clock budget. PPO league self-play uses four epochs, an 8,192-decision minibatch, clipping
  0.2, entropy coefficient 0.01 and value coefficient 0.5. Eight percent of the budget is reserved
  for final evaluation and export.
* Learner reward is `0.6 * win fraction + 0.4 * (0.5 + 0.5 * tanh(score margin / 50))`. The value
  head learns that final outcome; there is no discounted intermediate reward.
* League tables have 3--5 seats. In self-play, 25% of tables contain only the learner. The other
  tables mix learner seats with frozen earlier checkpoints and scripted opponents. Scripted
  selection is Merchant 60%, Simple 30%, Random 10%; a non-learner seat uses a frozen checkpoint
  45% of the time when one is available.

The saved Rust/WASM weights use the `CNTW` binary format. Any feature-layout or architecture
change invalidates existing checkpoints and requires retraining, a new weights fixture, and a
Rust/Python parity test. The browser does not automatically expose newly trained nets: see
[Exporting a net to the web UI](#exporting-a-net-to-the-web-ui) for the steps.

#### Measured performance facts

The initial unoptimised training loop managed about 26 games/s. The current profiler reaches about
130 games/s; completed real runs averaged around 100 games/s because the mixture of tables,
evaluation and PPO work differs from the steady-state profiler.

The profiler shows that the engine is not currently the main bottleneck. In a representative
1.764-second iteration, the C-ABI environment calls (`advance`, `observe`, `step`) took about
0.364 seconds, while GPU-to-host copies alone took 0.396 seconds and tensor transfers took 0.236
seconds. This is a Python-level profile, not a native `perf` flamegraph. No full engine-only
flamegraph has been collected yet.

The optimisations already made are:

* A rolling refill keeps the 256 environment slots busy: finished games are immediately replaced,
  while unfinished games carry into the next rollout. Before this, more than half of lockstep
  rollout steps had fewer than 32 of 256 games still alive.
* The action head skips padded actions when fewer than 60% of action slots are real. Most
  decisions have fewer than 20 legal actions despite padding to 48.
* CUDA graphs use width buckets of 8, 16, 32 and 48 actions; Gumbel-max sampling happens inside
  the graph. Their graph pool is shared and released after evaluation to avoid GPU memory growth.
* Environment buffers are pinned, the rollout performs one CPU transfer per step, rollout
  packaging is vectorised, and TF32 matrix multiplication is enabled.
* The Rust batch environment caches player counts, avoids `advance` work except for idle
  environments, and avoids clearing observations for non-pending games.

More than about 384 environments did not improve throughput on the measured GPU, while 512
environments ran out of memory. The default 256 environments is therefore intentional. A possible
future experiment is running two environment batches in parallel to overlap CPU simulation with
GPU inference; it is unmeasured and must not be described as a realised gain.

#### Stalls and memory safety

The base game rules do not force progress: a player may repeatedly choose legal, non-progressing
actions, including pass-like repricing when there is nothing to move. Before the fix, learner-only
tables could stall and be discarded, which both removed their learning signal and allowed their
stored decisions to accumulate on the GPU.

The training environment now declares a game stalled after 2,500 actions. Every seat receives
reward 0, rather than the game being dropped. `stalled` is logged per iteration and shown in the
dashboard. In the post-fix smoke test stalls stayed at approximately zero; over the completed runs
0.07-1.7% of tables stalled, and 2.5-11% in the harbor-rule runs (rates and cost in "Restricted
learners" below). This is a training safeguard, not an engine rule change; normal UI games
still use the underlying rules unchanged.

A net can also stall when it plays greedily (always the action it rates best), which is how every
rating protocol and every shipped bot plays. The net was trained under sampling, so nothing taught
it to leave a loop that argmax repeats: when its best action leaves its own input unchanged, the
same input gives the same output and the same action forever. The engine only forces an end when
every player is broke (no cash, all loans taken, no containers) and has nothing but idle moves, so
a loop in which one seat can still afford things never ends. Plain greedy Magnate (without the
guard below) stalled like this:

| Protocol | Games | Stalled |
|---|---:|---:|
| `anchors rate` (Python, seed 7) | 3,000 | 91 (3.0%) |
| the same protocol replicated in Rust (seed 11) | 3,000 | 109 (3.6%) |
| Rust, all Magnate, 3-5 players, 3 lengths | 60 per cell | 3-5 per cell |
| Rust, Magnate + Merchant | 60 per cell | 1-3 per cell |
| Rust, Magnate + Random + Simple + Merchant | 60 per cell | 4-11 per cell |
| Rust, Magnate + Random | 60 per cell | 14-17 per cell |

Tables with Random froze most (14 of 22 tables of Magnate, Merchant, Random, Random in the Python
protocol). Two traces show the loops. An all-Magnate 3-player game repeated Sail(Ocean) /
Sail(Harbor) and RequestLoan / RepayLoan indefinitely. At a mixed 4-player table the three other
seats were broke at the loan cap, while Magnate had $154, 50 containers in its holding and Produce
legal, and idle-sailed one colour short of the end of the game.

The fix is a repetition guard in `Neural` (`neural.rs`) that applies only to greedy play
(temperature 0), so training and sampling are untouched. It fingerprints the encoded input (FNV-1a
over the f32 bits; std's `DefaultHasher` is not stable across Rust versions) and remembers which
actions it has already played from that exact input. When an input comes back it plays the best
action not yet played from it, cycling through the actions in score order and starting over when all
have been tried. The match is exact, so it only changes play inside a true loop.

Measured effect: 0 stalled games in 720 Rust games (the four table mixes above, 3 lengths, 3-5
players, 20 seeds each), where plain greedy froze up to 17 of 60 in a cell. Rating did not move:
2285.8 plain and 2285.8 guarded on identical tables (3,000 games, seed 11). Games with Random now
take longer, up to about 4,600 actions (Extended, Magnate + Random), where they used to freeze.

The Python evaluations have no guard and drop stalled games, so the Python ratings of this net are
the plain greedy net with 3% of its games missing. They agree with the guarded Rust bot, so the
guard did not inflate the number. No other trained net has been checked for greedy stalls.

GPU memory also requires care: captured CUDA graphs retain their static buffers. Reusing one
shared graph pool, discarding obsolete frozen-net graphs and releasing graphs after evaluation
keeps allocated memory roughly flat in the observed training runs (about 2--2.4 GiB allocated).
The trainer also uses PyTorch's `expandable_segments` allocator setting because rollout sizes vary.

#### Exploiter experiment (`exploit_run2`)

`exploit_run2` is a best-response exploiter of `run2` (`--exploit run2`, seed 3, greedy target). It
started as a copy of `run2`, trained for 1,665.6 seconds of a 30-minute budget (197,144 games,
37,236,209 decisions, about 118 games/s). It was later registered at 2174.4 ELO as a permanent
league opponent for the defense experiment below. Its training-time duel evaluations (256 games,
1,024 for the final one) show the share of opponents beaten rising from 0.52 to about 0.64 in
roughly the first 10 minutes and then plateauing between 0.64 and 0.71. The final evaluation won
42.1% of games against a fair share of 25.8%, a margin of +8.67.

Pure duels of 3,000 games each, where every other seat is the named net:

| Player | Opponent seats | Opponent plays | Share beaten | Win rate (fair share) | Margin |
|---|---|---|---:|---:|---:|
| exploiter | `run2` | greedy | 0.659 | 0.394 (0.257) | +8.04 |
| exploiter | `run2` | sampling | 0.690 | 0.433 (0.260) | +9.52 |
| exploiter | `run1` | greedy | 0.741 | 0.487 (0.254) | +13.6 |
| `run2` | `run1` | greedy | 0.690 | 0.433 (0.253) | +10.63 |
| `run2` | `run2` | greedy | 0.497 | 0.247 (0.256) | -0.39 |
| `run2` | `run2` | sampling | 0.561 | 0.316 (0.260) | +3.13 |

The `run2` against `run2` row is the calibration check: equal players score about 0.5, and its duel
ELO of 2237 matches its ladder rating of 2239. Duel ELO is rated against the opponent alone, so it
is not a ladder rating and must not be put next to one.

On the mixed ladder tables, which are the measure of strength, the exploiter is weaker than the net
it was copied from:

| Net | ELO, seed 7 | ELO, seed 8 | Beats `run2` | Beats `run1` |
|---|---:|---:|---:|---:|
| exploiter | 2174.4 | 2186.9 | 0.414, 0.406 | 0.499, 0.543 |
| `run2`, re-rated with the same seeds and 2,000 games | 2233.8 | 2245.6 | n/a | 0.574, 0.614 |

Both beat Merchant, Simple and Random about 100% of the time. Across the run the exploiter's
ladder rating started at `run2`'s level (2236 at the first checkpoint) and drifted down: the mean
over the first nine checkpoints was 2221 and over the last nine 2195, ending at 2181, while its
duel share kept rising. Each point is one 2,000-game rating, and re-rating the same net with
another seed moved it by about 12 ELO, so only the trend over many checkpoints means anything. The
downward trend and the `vs_run2` fall from 0.51 to about 0.43 agree.

What this establishes:

* The exploiter is a specialist. It beats tables full of copies of one net (even `run1`, and even a
  sampling `run2`) but is about 60 ELO below `run2` on mixed tables and loses to it about 59% of the
  time there.
* Duel results and ladder results can disagree sharply. `run2` beats `run1` 69% in pure duels but
  only 57-61% on the mixed ladder, and the exploiter beats `run2` in duels yet loses to it on the
  ladder. Use the ladder protocol as the measure of strength; the duel only reports how well a net
  exploits one homogeneous field.
* The first version of this run logged duel ELO as `elo`, so the dashboard showed 2370 for a net
  that rates about 2181. The trainer now logs the ladder rating as `elo` and the duel numbers as
  `duel_*`, and the `exploit_run2` log was rewritten to match: every checkpoint was re-rated with
  2,000 ladder games (seed 7), and the `duel_*` values are the originals logged during training.

#### Exploiter-in-league control experiment

Two otherwise matching 30-minute continuations of `run2` (both seed 4, no imitation, starting
from `runs/run2/final.pt`) tested whether putting the specialist in the frozen-opponent league
improves a successor:

| Run | League during training | Elapsed | Games | Final 1,024-game ELO |
|---|---|---:|---:|---:|
| `control_run2_30m` | `run1`, `run2`, scripted bots | 1,662.1 s | 310,225 | 2260.4 |
| `defense_run2_vs_exploiter_30m` | control field plus `exploit_run2` | 1,669.6 s | 280,339 | 2273.6 |

Final training evaluations are noisy and cannot settle a 13-ELO gap, so both checkpoints were
evaluated under the same high-confidence protocol: 10,000 mixed-ladder games and 10,000
homogeneous duels against greedy `exploit_run2`, both with seed 9.

| Checkpoint | Mixed-ladder ELO | vs `run2` | vs `run1` | vs exploiter, mixed | vs exploiter, duel | Duel win rate (fair share) |
|---|---:|---:|---:|---:|---:|---:|
| control | 2279.3 | 0.549 | 0.657 | 0.649 | 0.553 | 0.325 (0.258) |
| defense | 2278.0 | 0.544 | 0.655 | 0.651 | 0.558 | 0.320 (0.258) |

The result is a tie. The defense run was 1.3 ELO lower on the 10,000-game ladder, while its duel
share against the exploiter was 0.5 percentage points higher; neither difference establishes a
benefit. The two 3,000-game ladder seeds also disagreed about the ordering (control 2269.9/2256.5,
defense 2266.9/2282.9), reinforcing that the shorter final-evaluation gap was noise. Both are
clearly stronger than their `run2` parent and both now beat the former exploiter in homogeneous
duels, so ordinary league continuation alone learned a defense.

This is one seed and one specialist, not evidence that exploiters never help. A useful next test
would use several independent seeds or multiple exploiters and predeclare the evaluation games.

What is not known:

* Why the original exploiter wins only in homogeneous tables. Candidates are a niche effect
  (copies of one deterministic net hurt each other in a way a mixed field does not) and
  distribution shift from training against a single opponent. Neither has been tested and no
  behaviour analysis was done.

#### Exporting a net to the web UI

The strongest net is in the UI as **Magnate** (`crates/container-bots/src/magnate.rs`). It is
`runs/defense_run2_vs_exploiter_30m/final.pt`. That run and `control_run2_30m` tied (10,000-game
ladder: 2278.0 against 2279.3), so the pick was a tie-break, not a finding: the defense net has the
higher rating pooled over the rating runs (about 2277 against about 2273) and the higher final
training evaluation (2273.6 against 2260.4). To ship the control net instead, replace
`crates/container-bots/weights/magnate.bin` with `runs/control_run2_30m/weights.bin`, re-rate it and
update `elo`; nothing else changes.

Steps for exporting any net (use `magnate.rs` as the template):

1. Rate it with at least two seeds and 10,000 games, for example
   `.venv/bin/python -m training.anchors rate runs/<run>/final.pt --games 10000 --seed 9`. Two
   3,000-game ratings of Magnate differed by 16 ELO (2266.9 and 2282.9).
2. Copy the weights the trainer wrote next to the checkpoint:
   `cp runs/<run>/weights.bin crates/container-bots/weights/<id>.bin` (it is byte-identical to
   `export_weights(final.pt)`).
3. Add `crates/container-bots/src/<id>.rs` with `ID`, `WEIGHTS = include_bytes!(..)`, a `net()`
   behind a `OnceLock` and a `new(seed)`. Declare the module in `lib.rs`, add an `OpponentInfo` to
   `OPPONENTS` and a branch to `create()`. `OPPONENTS` must stay in ascending `elo` order (a
   `container-web` test checks it).
4. `cargo test -p container-bots -p container-web`. The Magnate tests are the template: it must beat
   Merchant at every game length and play a whole game inside a `Session` with other bots.
5. `cd frontend && npm run wasm` (`npm run dev` and `npm run build` run it first). The setup
   dropdown is filled from `opponents()` as "Name · ELO n", so no frontend change is needed.

Magnate's `elo` label is 2280. The six ratings of it at Standard length, four in Python and two in
Rust, range from 2267 to 2286 and pool (about 39,000 games) to about 2280 with a standard error of
roughly 3-4. It is not a ladder anchor: `training/ladder.json` is unchanged, and `anchors add` would
make it a permanent league opponent for later runs. The UI label measures it against the ladder
bots, not against a human.

Length: the trainer and the arena only play Standard length (`GameConfig::new(n)`), yet the net
holds its strength at the others. Rust, guarded, 3,000 games each (seed 11, noise about 12 ELO;
the anchors were rated at Standard, so treat the other rows as approximate):

| Length | ELO | Beats `run2` |
|---|---:|---:|
| Short | 2268.7 | 0.538 |
| Standard | 2286.5 | 0.549 |
| Extended | 2289.5 | 0.574 |

It beats Merchant 99.8-99.9% at every length.

Rust against Python: the Rust bot is a pure-Rust forward pass of the same weights, and a Rust replica
of the `anchors rate` protocol reproduces the Python shares. Plain greedy Rust against `run2`,
`exploit_run2`, `run1` and Merchant won a share of 0.549, 0.664, 0.669 and 0.999; Python (10,000
games) 0.546, 0.658, 0.656 and 0.999.

Speed in the browser: in a Node run of the release WASM, one bot step took about 0.5 ms and whole
games of 3-5 seats with Magnate took 0.2-0.6 s (340-1,200 steps). The weights are 1.5 MB of the
1.8 MB bundle.

**Consortium** (`crates/container-bots/src/consortium.rs`) is psro08, the last net of the population study
(`weights/psro08.bin` is `training.model.export_weights` of `populations/psro/psro08.pt`). It plays the action it
rates best with the repetition guard, the `guarded` mode the study measured, at every table size (spec section
9.4 says why one net and not a mixture). The Rust bot and the Python net play the same games:
`training/test_evalcore.py` checks the scores, lengths and winners of six games against the Rust arena. It
takes the WASM bundle from about 1.8 MB to 3.3 MB. Its UI label is **2378**: Magnate's 2280 plus +98 [+91, +111]
from 6,000 Rust-arena games of Consortium against Magnate at 3-5 seats (seed 21, 2 stalled; a Bradley-Terry fit of
winner share). The ladder protocol cannot show the difference (10,000 games, greedy and unguarded, rates psro08 2280.8 with 3%
stalls, because the anchor field is saturated), so the label is a head-to-head figure on the ladder's scale, not a ladder rating.

#### Restricted learners (`--no-call-bank`, `--buy-at-harbor`, `--no-same-turn-repay`) and throughput

Training flags (also on `anchors rate`) restrict only the learner seat, in training and in the
trainer's evals; scripted bots and anchors play normally. They are **not** enforced in the web
UI bots, so a net trained this way needs the same filter in its Rust bot before export.

- `--no-call-bank`: removes `StartAuction` and `Outbid` (calling the Bank). Sailing to the Bank
  and buy-outs stay legal.
- `--buy-at-harbor`: forbids sailing to a harbor where nothing can be bought, and forbids
  `DoneBuying` with an empty purchase list at a harbor.
- `--no-same-turn-repay`: removes `RepayLoan` in a turn in which the seat has requested a loan, so a request
  is no longer free. Loans a bidder takes on another seat's turn do not count. A deployed net trained with it
  plays under `Neural::without_same_turn_repay`; an unretrained net must not, as it keeps the loans it took.
- If a filter would leave no legal move it is ignored for that decision.
- Ratings under restrictions are not comparable to unrestricted ones.

`--welfare-weight W` (0..1, default 0) moves that share of the learner's reward from its own rank and
score margin to `selfplay.table_welfare`: the table's mean final score, the same for every seat. A seat
that hands money to another gains nothing from it by itself, because the total is unchanged; what raises
it is the supply's subsidy on accepted delivery bids and containers landed on the Island, so it should favour
higher bids and fewer wasted moves (not measured yet). Only the training reward changes: evals and ratings are as before.
Scripted bots and anchors do not share the goal, so tables of the learner's own copies are where it can
pay off. Which `W` gives a bot that is worth playing against is not known yet; sweep it.

Interaction options (measured with `python -m training.audit_interaction`, see its docstring; in 30-minute
scratch runs the blended rewards below did little, so treat them as experiments):

- `--win-only`: the reward is only the share of opponents beaten, with no margin term.
- `--geomean-weight G`, `--trade-weight T`: that share blended with the table's geometric mean score, or with the
  money of the player-to-player deliveries the seat took part in (`selfplay.trade_term`).
- `--min-bid N`: the learner opens every delivery auction with at least N (all its cash if less). Not in the
  Rust bot yet.
- `--min-trade M` (with `--penalty-lr`): a constraint instead of a weight. The reward is the share beaten
  minus a penalty times the seat's relative shortfall below M of traded money; the penalty rises while the
  learners fall short and falls when they exceed it (logged as `penalty` and `traded`).
- `--scripted-share P`: the chance that an opponent seat in a league table is a scripted bot, standing in for
  a human, instead of a frozen copy of the learner.
- `--trader-share P`: the chance that a scripted seat is a trader (seat kind `-4`, `merchant::Trader`): Merchant that
  pays only up to $2-$4 per container (fixed per game) and prices its own around one lot ($1-$4). Merchant buys at any
  price and sells at $1, which teaches a net that pricing at the maximum costs nothing. `audit_interaction --stand-in trader`
  audits against one.
- `--island-weight W`: that share of the reward is the points the seat's score took from the Island (`selfplay.island_term`,
  1 at 80 points), the rest the share of opponents beaten. Exclusive with the other reward options.
- `--classic`: the classic rule. Nobody at the table may call the Bank (scripted bots and frozen copies included), so
  `--no-call-bank`'s learner-only ban is not needed. Ladder ELOs were measured with the Bank open, so read the win rates.
  `audit_interaction` takes `:classic` after a net's path.

Runs: `nobank_magnate_30m` (init Magnate, Bank banned only, stopped at 751 s because it stalled,
ELO ~2219) and a chain that started from nothing (seed 7, no `--init`) with both rules. Each link
resumed from the previous one's checkpoint:

| Run | Budget used | Envs | Final eval ELO | Stalled tables |
|---|---:|---:|---:|---:|
| `nobank_harbor_scratch_30m` | 1,266 s, died of CUDA OOM (last eval 1852 at 1259 s) | 256 | n/a | 7.7% |
| `nobank_harbor_scratch_30m_cont` (from `ckpt_014.pt`) | 506 s of 540 | 192 | 2010.6 | 11.2% |
| `nobank_harbor_scratch_30m_cont_30m` | 1,666 s of 1,800 | 192 | 2255.0 | 5.7% |
| `nobank_harbor_scratch_2h` | 6,634 s of 7,200 | 192 | 2301.1 | 2.45% |

About 3.3 GB of the 10 GB GPU is held by other processes, so keep headroom: use `--envs 192` if
the default OOMs (the error is in `selfplay._package`). The budget figure is the trainable part;
8% is reserved for the final evaluation and export.

The 2-hour run (`final.pt` and `weights.bin` in `runs/nobank_harbor_scratch_2h/`, 727k learner
outcomes, 189M decisions) ended at 2301.1 ELO (1,024 games): 99.7% vs Merchant, 71.2% vs `run1`,
59.7% vs `run2`, 63.9% vs `exploit_run2`. These opponents play without the restrictions, the net
plays with both, and the ratings are not comparable to the unrestricted ones. Findings:

- **It plateaued after about 1,000 s.** Periodic evals (256 games each, so noisy by roughly 30
  ELO) read 2238 at 11 s, 2279 at 978 s, then stayed between 2275 and 2330 for the remaining
  5,600 s. The best reading, 2329.6 at 3,393 s, was not kept: the run exports the last net, not
  the best one. The mean reward per iteration stayed at 0.62 throughout (0.623 in the first 900 s,
  0.619 in the last). One 2-hour run does not show whether a longer budget or another seed would
  go higher.
- **Throughput stopped falling.** After the league filled to 5 frozen nets (about 500 s),
  tables/s stayed at 39-49: 40 around 900-2,700 s when 3.5% of tables stalled, then 45-46 once the
  stall rate settled near 2%. So the late stall rate was the only thing moving throughput,
  consistent with the fit above.
- **Stalls fell with training** (2.1%, 3.6%, 3.5%, then 1.7-2.4% by the second hour) but did not
  vanish, against 0.07-0.15% for the unrestricted runs. A stalled table costs 2.8x a normal one in
  learner decisions.
- **Not done:** the net was not re-rated with `anchors rate --no-call-bank --buy-at-harbor`
  (3,000 games), it has not been exported to the UI, and the cause of the remaining stalls was not
  traced. A stall penalty (a reward below the loss floor, or a small per-decision penalty on
  repeated identical inputs) is only an idea; nothing has been implemented.

Why games/s falls during a run. Units first: an iteration plays until `--envs` tables have
finished, so `games_per_sec` in `metrics.jsonl` is finished tables per second and `stalled` counts
stalled tables among them (a stall rate is `stalled / envs`). `games` counts the learner-seat
outcomes of tables that finished normally, about 2.3 per table, so it is not a table count.

- Frozen opponents (`python -m training.profile_iter --frozen N`, 256 envs): 0 frozen nets
  156-192 tables/s (play 1.0-1.2 s); 5 frozen nets about 102 (play 2.1 s). Update and advantages
  cost the same, so the loss is in the play phase: every frozen opponent needs its own forward
  passes. The league fills to 5 nets within 400-900 s, which explains the 25-40% decline in every
  run, and throughput is flat once it is full. In `nobank_harbor_scratch_30m_cont_30m` it fell
  from 65 tables/s (1 net) to 45-46 by t=500 s (5 nets) and stayed there. Only 0 and 5 nets were
  profiled, not the sizes between.
- Stalled tables run to the 2,500-action limit. Per-table stall rates (stalled / (iterations x
  envs)): run2 0.15%, control 0.12%, defense 0.07%, run1 1.0%, exploit_run2 1.7%,
  `nobank_magnate_30m` 0.7%, and 2.5-11% in the harbor-rule runs. A least-squares fit on a live run
  gives 1,445 learner decisions per stalled table against 525 per normal one (2.8x), so stalls hold
  11-14% of the learner decisions and cost about 7-14% more work per finished table. It matches
  the live run: tables/s rose from 45.7 to 49.0 when stalled tables fell from 4.2% to 3.2%.

#### Interpreting evaluations and planning experiments
Do not treat a high scripted-bot ELO as proof that a new net is better. Once a net is close to
100% against Merchant, the scripted field cannot resolve further differences. Add good checkpoints
as anchors in order of increasing strength, then assess candidates against the entire anchor field.
Adding an anchor does not refit ratings of existing anchors.

`anchors.evaluate` and `evaluate_exploit` now report `completed`, `stalled` and `stall_rate`
(`games` is the number requested, and stalled games are still left out of the rating). Before
that change no rating reported a stall count: a net that froze in 3% of its games (greedy Magnate
did before the guard) got a clean-looking ELO from the other 97%. A rating is still a
statement about a field, not about exploitability; for that see
[Empirical game-theoretic evaluation](#empirical-game-theoretic-evaluation).

The current evidence says `run2` is only modestly stronger than `run1`, and the self-play curve was
noisy. The next experiments should therefore measure head-to-head performance against anchors,
not only ELO against scripted bots. A best-response exploiter of `run2` did not produce a stronger
net (see above). A control and a successor trained with that exploiter in the league both improved
to about 2279 ELO, but the measured difference was a tie; do not claim a defense-league gain from
this single paired run. Plausible but unvalidated directions are a more diverse frozen league,
longer runs, larger or richer feature models, CPU/GPU overlap, and an information-set search layer
on top of the net. Each requires a controlled before/after comparison and should be recorded here
after it is measured. The equilibrium work suggests an order: a better-trained best-response oracle (the
learning-rate schedule first; longer budgets and several seeds are still open) should come before any larger net,
because the schedule alone moved the measured gain from +0.006 to +0.098 against the same opponent (spec section 11).

### A strong AI that interacts with its opponents

The goal: a net that is very strong and also plays with the table, meaning it buys containers from opponents'
factories and harbors, pays fair prices for deliveries, and delivers to the Island, instead of living off the
Bank. This section collects what was measured on the way. Every run below is **one 30-minute run with no noise
estimate**; ELO gaps under about 100 are not significant (one run's successive 256-game evals moved 1943 to 2188).
The `runs/` outputs are untracked.

#### The behavior to fix

Magnate and Consortium all play one way:

- they price their own containers in the top lots so nobody buys them, and take cash and containers from the Bank;
- they score from cash in hand and from containers left at the Bank, and mostly ignore the Island;
- they bid $1.5 to $2 on a delivery, because the bidder only gets containers (the deliverer gets twice the bid, and the
  equal amount is minted), so a bid is a small price;
- half their turns are idle harbor sails, and they took a loan and repaid it the same turn.

Measured on Magnate (150 to 200 games, three nets plus a Merchant): about 120 bids on players and 88 Bank bids per game,
15.6 containers bought, and a score of cash 57 / island 25 / leftovers 21.

#### Why training does not fix it by itself

- **The opponents teach it.** The scripted bots and frozen copies never offer a cheap container: Merchant buys at any price
  and sells at $1, and nets price high. A $1 lot is an input the nets have almost never seen. In 40 games per net a $1 lot was
  offered 0 times to Broker, 3 to Magnate and 12 to Consortium, and none was taken. Against Merchants under the classic rule
  (which sell at $1) the classic net took 13.5% of $1 offers and 16.6% of dearer ones, so it does not prefer cheap. Consortium
  took 8.2% and 4.6%.
- **Reward blends change little.** A welfare term (table mean), a geometric-mean term, win-only and a trade bonus all left
  the engagement about as it was (about 120 bids per game, mean bid $1.5 to $2.5). The welfare term cannot be attributed to
  one seat's action, and the money minted is only about $2 per accepted delivery.
- **Same-turn repay banned, loans gone.** `--no-same-turn-repay` removes the take-and-repay pattern. The retrained nets
  hold no loans at the end. The Rust bot needs the same filter (`Neural::without_same_turn_repay`).

#### Tools added

| Option | What it does |
|---|---|
| `--no-same-turn-repay`, `--min-bid N` | Hard action masks on the learner: no repayment in a turn that took a loan; a delivery auction opens with a bid of at least N (all its cash, if less). |
| `--classic` | Nobody at the table may call the Bank, scripted seats and frozen copies included. |
| `--trader-share P` | Scripted opponents that are traders (seat kind `-4`, `merchant::Trader`): Merchant that pays only $2 to $4 per container and prices its own around one lot. |
| `--scripted-share P` | How often an opponent seat is a scripted bot (a human stand-in) instead of a frozen copy. |
| `--win-only`, `--welfare-weight`, `--geomean-weight`, `--trade-weight` | Reward blends. Win-only buys the most containers; the trade bonus raises bids. |
| `--min-trade M` with `--penalty-lr` | A constraint on the money of the deliveries the seat takes part in: reward is the share beaten minus a penalty times the shortfall. It never bound (traded money was 92 against a target of 45). |
| `--island-weight W` | Reward share for the points taken from the Island. |
| `python -m training.audit_interaction` | The measurement: containers bought, purchases from the stand-in and from other nets, harbor sails, bids, mean bid, Bank bids, where the score comes from (cash, island, leftovers, loans), and the share of containers priced and bought per lot. `--stand-in trader` and `:classic` options. |
| `anchors add` | `consortium` (2286) and `traders` (1907) are on the ladder. |

Deployed bots: **Broker** (`--min-bid 3 --min-trade 45`, 2150) and **Classic** (`--classic`, 2230) are in the UI list
and the arena, with their rules built into the bot (`Neural::with_min_bid`, `without_bank`, `without_same_turn_repay`).
A net trained under a rule must be deployed under it.

#### Results

Audit against a Merchant stand-in, 3 nets at the table (cash / island / leftovers are points of the final score):

| Net | ELO (trainer eval) | Containers bought | Bids on players | Mean bid | Score: cash / island / leftovers |
|---|---|---|---|---|---|
| Magnate | 2280 | 15.6 | 66 | $1.83 | 57 / 26 / 21 |
| `scratch_winonly` (win-only) | 2135 | 24.1 | 90 | $1.44 | 57 / 26 / 17 |
| `scratch_trade` (trade bonus 0.4) | 2217 | 19.5 | 91 | $2.51 | 73 / 38 / 9 |
| `scratch_constrained` = Broker | 2147 | 16.6 | 107 | $2.56 | 80 / 51 / 4 |
| `scratch_traders` (trader opponents) | 1934 | 21.4 | 93 | $3.87 | 91 / 43 / 5 |
| `classic_net` (no Bank, `--classic`) | 2234, see note | 26.0 | 103 | $2.10 | 72 / 42 / 1 |
| `scratch_island` (island reward) | 1866 | 23.4, see below | 96 | $10.82 | 196 / 48 / 3 |

Runs that did not help: `scratch_geo` (geometric mean 0.3, 2052), `scratch_geo70` (1926).

- **The classic rule works for interaction.** With no Bank call the net buys 26 to 29 containers per game, makes 36 to 38
  accepted deliveries, and gets 34 to 42 points from the Island, against 23 to 25 containers and 24 to 33 points for Magnate and
  Consortium played under the same rule. It prices most of its containers in the top lots (63% in lot 4, the harbor's $6, and
  29% in lot 3) and buys mostly from the top lots, so it trades with the other nets at the highest prices. Its eval ELO is
  against opponents that could not call the Bank either, so it is not comparable with the Bank-open ratings. Its final
  eval stalled 25% of games; the shipped bot keeps the repetition guard.
- **Harbor and factory purchases barely moved.** The nets buy about 5 containers per seat per game from the opponents
  whatever the reward (Magnate 15.6 across three seats). Win-only and the trade bonus buy more; none reaches a human's level.
- **Score sources.** The nets' points are about 55 to 60% cash, 25% island and 10 to 20% leftover containers; none holds
  a loan at the end. Broker and `scratch_trade` convert best: island 38 to 51 points, leftovers 4 to 9.
- **The island reward found a loop, not a strategy.** `scratch_island` bids about $11 on average. Three copies of one net bid
  each other up, every accepted bid mints an equal sum, and each reaches a cash score near 196. It took 48 island points,
  about the same as Broker's 47, played 8.4 factory purchases per game against 18 to 21 for the others, and has the
  lowest strength of the set. The audit table is all copies of one net, so it exaggerates this; the loop is real in
  self-play, and a bid-money cap or penalty is needed before the island reward can be used.
- **Ratings cluster.** The five from-scratch runs sit within about 300 ELO of each other, and four of them within 165. Part of
  that is noise (256-game evals), part is that Simple, Merchant and Random are all beaten about 100% of the time so only three
  anchors discriminate (now six with `consortium` and `traders`). Compare nets in head-to-head games, with several seeds,
  before ranking them.

#### What is still open

1. **A stand-in that sells cheap.** None of the opponents offers a $1 container on purpose, so no run has learned that buying
   one is a bargain. Add scripted sellers that price at $1 or $2, and a reward or constraint that a cheap offer is taken when
   the seat can afford it.
2. **Cap the bid money.** The mint loop makes high bids profitable among copies of the same net. Cap bids, or pay the
   reward on the score net of minted money, before using `--island-weight` or `--trade-weight` at high values.
3. **Strength.** Every from-scratch run is 60 to 350 ELO under Magnate after 30 minutes. Start from Consortium or Magnate under
   the new constraints and train for 90 minutes or more, with several seeds.
4. **A better human proxy.** Merchant and the traders are not humans. The only real check is playing the UI against Broker and
   Classic; record what a person sees there.
5. **Mixed tables.** The audit puts three copies of one net at the table. Audit one net against Magnate and Consortium.
6. **Restrictions in the UI.** The engine lets every seat call the Bank; only the Classic bot is bound by `without_bank`.
   A table-wide classic rule in the web session would need a rule in the engine.

### Empirical game-theoretic evaluation

An ELO says how a net does against one particular field. It does not say how much a *different*
opponent could gain against it, which is what "close to an equilibrium" would mean. The tools below
estimate that gain empirically. [`docs/equilibrium_spec.md`](docs/equilibrium_spec.md) defines every
quantity and fixes what may be claimed. The most any result here supports is: *an empirically
estimated approximate equilibrium relative to the tested policy population, game configurations and
best-response search budget.* No checkpoint is claimed to approximate a Nash equilibrium of the game.

**Utility.** A completed game pays each winner `1/|winners|` and every other seat 0 (ties are broken
by the engine's rule). A game still running after 2,500 engine actions is stalled and pays every
seat 0, so stalling never helps a leader; a seat that cannot win is indifferent to it, so every
report counts stalled games. ELO, rank share and score margin are secondary metrics. The table sizes
3, 4 and 5 are separate games (the nets see the player count), and the `sample`, `greedy` and
`guarded` modes of a net are separate strategies that are never pooled. Every report names its seed
block: decisions use `dev`, claims use `holdout`, and each holdout look is logged in
`reports/holdout_log.jsonl`.

```sh
.venv/bin/python -m training.population freeze   # populations/baselines/: read-only copies, hashes and configs
.venv/bin/python -m training.crossplay run --out reports/crossplay/deployed_dev --seeds 24
.venv/bin/python -m training.br train --name m1 --init populations/baselines/defense_run2_vs_exploiter_30m.pt \
    --profile run2,magnate,exploit_run2 --budget-minutes 25
.venv/bin/python -m training.br eval --candidates m1=runs/br/m1/final.pt --profile run2,magnate,exploit_run2 \
    --block holdout --seeds 150 --all-focals --out reports/br/m1
.venv/bin/python -m training.psro solve --cells reports/crossplay/deployed_dev/payoffs.json \
    --strategies run2,magnate,exploit_run2 --out reports/psro/p0_metagame
.venv/bin/python -m training.psro init --strategies run2,magnate,control,defense,exploit_run2,merchant \
    --cells reports/crossplay/deployed_dev/payoffs.json
.venv/bin/python -m training.psro run --iterations 20 --no-start-after 20:40 --lr 1e-4 --lr-decay linear
# the command of the first run: an iteration is not started after 20:40 on the machine clock
.venv/bin/python -m training.psro final --seeds 150    # closing audit on the holdout block: reports/psro/final/report.md
# the follow-up on the final game (spec section 9.3): 48 more dev seeds for the strategies that can be in the
# support, a new solve of the nine-strategy game from every cell (it also writes refined_metagame_profile.json),
# mixtures against other fields, and a probe of the loan action that plays no oracle
.venv/bin/python -m training.crossplay run --population populations/psro/manifest.json \
    --strategies psro02,psro03,psro05,magnate,control --seeds 48 --seed-offset 24 --out reports/crossplay/refine_dev
.venv/bin/python -m training.psro solve --cells reports/crossplay/deployed_dev/payoffs.json,reports/psro/iter_02/crossplay/payoffs.json,reports/psro/iter_03/crossplay/payoffs.json,reports/psro/iter_05/crossplay/payoffs.json,reports/crossplay/refine_dev/payoffs.json \
    --strategies run2,magnate,control,defense,exploit_run2,merchant,psro02,psro03,psro05 --out reports/psro/refined_metagame
.venv/bin/python -m training.fields --cells <the same five files> --strategies <the same nine> \
    --mix refined=@reports/psro/refined_metagame_profile.json --mix old=@reports/psro/final/profile_final.json \
    --field sigma=@reports/psro/refined_metagame_profile.json --field uniform --field magnate --out reports/psro/refined_fields
.venv/bin/python -m training.loanprobe --profile @reports/psro/refined_metagame_profile.json --out reports/loanprobe/dev
```

* **Cross-play** (`training/crossplay.py`) plays every composition of a population at every table
  size and keeps every game: completed or stalled, score margin, game length, performance by turn
  position (the first player is random, so the seat index is not turn order), with seed-clustered
  bootstrap intervals. It writes a `payoffs.json` that the meta-game code reads
  and a `report.md`. `--length short|extended` runs the other game lengths; Standard is the headline.
* **Meta-game** (`training/metagame.py`) turns the payoffs into one symmetric game per table size,
  runs several solvers, each labelled with the solution concept it really targets, and checks every
  solution with a verifier that shares no code with them. The result is the regret of the solution
  inside the restricted game, with a bootstrap noise floor. That regret says nothing about
  strategies outside the population.
* **Best-response oracle** (`training/br.py`) trains a PPO learner against a fixed mixture of
  opponents (canonical utility or the shaped training reward) and then measures, on seeds it never
  saw and paired by common random numbers, how much a seat gains by deviating from the mixture to it.
  Summed over the seats that is the mixture's NashConv. The learner is only an approximate best
  response, so its gain is a **lower bound** on the true deviation gain, and a small gain proves
  nothing until the oracle is shown to find improvements known to exist (see the validation report).
  The optimiser is Adam at `--lr` (3e-4 by default), constant or, with `--lr-decay linear`, falling to
  zero over the budget; when the start is already a strong network a lower rate matters (see the
  milestone report and the PSRO summary).
* **PSRO** (`training/psro.py`) solves the empirical meta-game, trains a best response to the
  solution, and admits it only when the lower 95% bound of its measured gain is above zero. It
  never removes a member because of its ELO. Each iteration writes a report, and
  `populations/*/results.json` keeps what was measured about every member (coverage, cross-play, best
  responses), apart from the manifest whose hash the payoff files cite. `psro final` is the closing audit
  of a run on the holdout block (the last meta-strategy attacked by every strategy and every rejected
  response, the solution of the final game audited on other seeds, each response against the best member).
* `training/audit_report.py` and the Rust `audit` binary measure what the 48-action interface does
  to the game; `training/br_report.py` tabulates best-response training runs against their studies.

**Results so far** (Standard length, 3, 4 and 5 players; every number has its interval in the linked
report, and none of this is a claim about a Nash equilibrium of the game).

* *Cross-play* (`reports/crossplay/deployed_dev`, 81,648 games, 6 stalled): by winner share relative to an
  equal share the strategies order as Magnate (the deployed bot) 1.55, control 1.34, defense 1.30, run2 1.13,
  exploit_run2 1.05, run1 0.62, Merchant 0.02, and in the same order at Short and Extended length. Greedy
  play without the repetition guard stalls in 1.7% of games, guarded play never did and sampled play in
  0.004%. The first player wins 1.18 [1.05, 1.30] times an equal share with 3 players (1.10 and 1.05 with 4
  and 5, intervals that include 1), so every design is balanced over turn position.
* *Interface audit* ([`reports/audit/summary.md`](reports/audit/summary.md); 300 self-play games per net through
  the Rust bot): the 48-action cap thins the legal list in 2.0-3.9% of decisions (bidding, up to 123 legal
  actions), the net's preferred action on the full list is among the dropped ones in 2.6-8.7% of those (well
  under 0.5% of all decisions), and the repetition guard overrides 0.16% of Magnate's decisions. `RequestLoan`
  is legal in 39,000-43,000 bidding and deciding decisions per 300 games and no net chose it there (Magnate takes
  loans in the main phase: 13,282 of 106,786 legal decisions). This was not shown to cost game results; it says
  which parts of the action space the nets have never used.
* *Meta-game*: Magnate alone is the equilibrium of the seven-strategy empirical game of the baselines at 3
  and 5 players (4 players has eight equilibria); `reports/psro/baseline_metagame.md`.
* *Oracle validation* ([`reports/br/validation/summary.md`](reports/br/validation/summary.md)): three 12-minute
  oracles (σ = run1 or run2, shaped or canonical objective) gain +0.105 to +0.186 when sampling and +0.167 to
  +0.182 with the guard over σ on dev seeds, 67-106% of the best pure deviation known in the population
  (exploit_run2 or Magnate). The canonical winner-share objective learned as fast as the shaped reward, so the
  later runs use it. The periodic 25-seed study during a run has a half-width of about 0.07 and cannot be used
  to follow one.
* *Milestone* ([`reports/br/milestone.md`](reports/br/milestone.md)): against run2, Magnate and
  exploit_run2 at equal weights, a 25-minute PPO deviation gains at least +0.091 [+0.067, +0.115]
  expected utility on held-out seeds (NashConv at least +0.37); pure Magnate gains +0.048 of that without new
  training. The gain is measured to about ±0.025 with 150 seeds per table size, two oracle seeds differ by
  about 0.03, and against a broader opponent mix the oracle's weights were worse than its initial network
  when sampled. A small gain from this oracle is therefore not evidence of near-equilibrium.
* *PSRO* ([`reports/psro/summary.md`](reports/psro/summary.md), closing audit in
  [`reports/psro/final/report.md`](reports/psro/final/report.md)): from six of those strategies (run2, Magnate,
  control, defense, exploit_run2, Merchant), seven iterations each trained a 25-minute response against the
  meta-strategy σ of the current population and admitted it
  only if the lower 95% bound of its gain over σ was above 0 (100 dev seeds per table size). psro02, psro03 and
  psro05 were admitted (nine strategies) and four responses were not. The gain of the response fell from +0.098
  [+0.070, +0.126] against the σ of the baselines (iteration 2; +0.120 [+0.093, +0.145] on held-out seeds) to
  +0.015 [−0.011, +0.041] and +0.007 [−0.021, +0.036] against the σ of the final population (iterations 6 and 7),
  which is the resolution of the study (about ±0.03). Only the first admitted response is clearly better than the
  best member of its population (+0.080 [+0.051, +0.108], paired); iterations 3 and 5 are +0.027 and +0.029 above
  it with intervals that reach 0. The same σ attacked with Adam at 3e-4 constant gained +0.006 and with 1e-4
  falling to 0 gained +0.098 (iterations 1 and 2), so a small gain is evidence about the oracle before it is
  evidence about σ. On held-out seeds the best deviation found against the σ of the final population is the
  response of iteration 7, which was rejected on the dev seeds: +0.027 [+0.005, +0.049]; the nine strategies gain
  at most +0.005 against it, and on a second block the best member (psro05) gains +0.017 [+0.004, +0.031] against
  the solution of the final game, which is σ's own estimation error. Of the two marginal admissions iteration 3 was
  confirmed on held-out seeds (+0.033 [+0.007, +0.060]) and iteration 5 was not (+0.021 [−0.009, +0.050]). The loop
  stopped at a time guard, not at a convergence test. All of these are lower bounds on exploitability from a
  25-minute oracle, for this population only; none of it is a claim about a Nash equilibrium of the game.
* *How to read the PSRO numbers* (spec section 9.1 and 12). (1) The oracle's settings were changed after
  iteration 1 (Adam 3e-4 constant, then 1e-4 falling to 0) because of what that iteration showed, so the loop is
  tuned on the dev block; the closing audit and the confirmations use held-out seeds. (2) Iterations 4 and 5, and
  6 and 7, attacked the same σ from the same initial network (psro03) with different oracle and study seeds: the
  first pair gained +0.030 and +0.052 and the admission rule split them, the second +0.015 and +0.007. Oracle
  seeds can therefore differ by 0.02-0.03 (as in the milestone), the size of the gains being decided. (3) σ is
  solved from 16-seed cross-play cells and has an estimation error of its own: a member of its support (psro03
  against the σ of iteration 4) gained +0.023 [+0.007, +0.039] on dev seeds the solution never saw, which the
  bootstrap noise floor (mean gain 0.012-0.023 by table size, 95th percentile 0.035-0.060) had predicted. (4) The
  NashConv implied by the response fell from +0.39 [+0.28, +0.50] (iteration 2) to about +0.05 (iterations 6 and
  7, intervals that include 0), and the closing audit on held-out seeds measures at least +0.106 [+0.022, +0.192]
  for the last σ. Every look at the holdout block is in `reports/holdout_log.jsonl`; offsets 0-149, 200-599,
  700-799 and 900-999 have been used (and 1000-1299, 1400-1549 and 1600-1699 since, see spec section 9.4), so take a new look at 1,700 or later.
* *The final game, refined, and the loan probe* (spec section 9.3). 48 more dev seeds for psro02, psro03, psro05,
  Magnate and control (48,720 games, 1 stalled) cut the bootstrap noise floor of the gain of σ from 0.012-0.022 to
  0.004-0.006 on average (95th percentile from 0.051-0.070 to 0.025-0.033), and the nine-strategy game was solved
  again from 206,736 simulated games ([`reports/psro/refined_metagame.md`](reports/psro/refined_metagame.md)). Its
  empirical equilibrium is pure: psro05 at 3 and 4 players and psro03 at 5; the mixtures of the first audit were
  not identified by 16-seed cells. Against σ and against the uniform mixture of the nine, psro05 is ahead at 3
  players (+0.051 [+0.029, +0.082] over psro03 against the uniform mixture), while at 4 and 5 players psro05 and
  psro03 are within noise on every field ([`reports/psro/refined_fields.md`](reports/psro/refined_fields.md);
  `training/fields.py` computes these from the cells, no new game). Forcing `RequestLoan` instead of the net's pick
  in 10% of its bidding decisions cost psro03 and psro05 −0.079 [−0.103, −0.054] and −0.079 [−0.100, −0.057] per
  seat, and in 10% of its deciding decisions −0.017 [−0.033, −0.000] and −0.004 [−0.021, +0.015]; a rule that
  loans when the pick is a bid of all the net's cash never fired
  ([`reports/loanprobe/dev.md`](reports/loanprobe/dev.md)). That is a negative result for three fixed rules, not
  a proof that no loan policy helps.
* *The final response, psro08, and the bot* (spec section 9.4). One more oracle, 50 minutes (twice the loop's
  budget, Adam 1e-4 falling to 0, canonical utility, exploration 0.1), started from psro05 against the refined
  solution (psro05 at 3 and 4 players, psro03 at 5). On 100 dev seeds per table size the guarded response gained
  +0.033 [+0.005, +0.060]; on 300 held-out seeds (offset 1000) +0.022 [+0.008, +0.035] (psro03 −0.024, psro05
  −0.002 against the same σ). It was admitted as psro08 (ten strategies). Its cells were simulated (110,880 games,
  24 seeds each) and the ten-strategy game solved again: the equilibria mix psro02, psro03, psro05 and psro08 with
  weights that 24-seed cells do not identify, and psro08 is in the support at every table size and earns the
  equal share against every solution (3 players 0.333, 4 players 0.247, 5 players 0.200). Pure psro08 is within
  noise of the solved mixture against σ, the uniform field and Magnate, and is ahead of the previous solution
  at 5 players against the uniform field (+0.022 [+0.017, +0.026]), so the bot plays psro08 alone. On a fresh
  holdout block (offset 1400, 150 seeds, 18,000 games, 3 stalled) no strategy gains over the ten-strategy σ by
  more than noise (psro05 +0.019 [−0.002, +0.041], psro08 +0.008 [−0.003, +0.020]); against a table of Magnate
  (offset 1600, 100 seeds, 12,000 games, 0 stalled) psro08 gains +0.134 [+0.110, +0.159] per seat, psro05
  +0.115 [+0.089, +0.144], psro03 +0.118 [+0.092, +0.143]. No result separates psro08 from psro05 at 3 and 4
  players beyond the 0.02 resolution of the study, so psro08 is the strongest net measured, not a demonstrated
  improvement over psro05 there. Reports: `reports/psro/final_br/`.

## Rules decisions

The engine plays the [10th Anniversary rulebook](https://bghub.org/r/container.pdf), the developer's errata and the rulings the project owner gave while it was built. This section records every point where the rulebook leaves a question open or where the engine had to commit to one reading of it, and what the errata changed. Rules the rulebook states plainly are not repeated. Every decision is covered by tests in `crates/container-engine/src/tests/`, in files named after the area, and `crates/container-engine/tests/playouts.rs` plays whole games.

### Sources

| Tag | Meaning |
|---|---|
| **Errata** | The developer's errata. It overrides the rulebook and every earlier decision. |
| **Ruling** | The owner answered a question about it. A ruling overrides the rulebook text and any assumption. |
| **Reading** | The rulebook speaks to it, but not precisely enough, so its text was interpreted. |
| **Assumption** | The rulebook is silent, so the choice was made while building the engine. *Announced* means the owner was told in the conversation and did not object. |
| **Rulebook** | Stated by the rulebook. Listed only where the engine's modelling of it is not obvious. |

### Errata

The errata arrived after the owner's first two rounds of rulings, so it overrides them. All of it is applied. One earlier ruling had to be reversed, and the other items confirm what was already decided.

| Errata | What the engine does | Earlier decision |
|---|---|---|
| **Loading Bank containers.** While docked at the Bank, a Harbor Purchase action loads containers from the holding area onto the ship, for free. | `HarborPurchase` while docked at the Bank loads containers from the holding area onto the ship. It is free, costs one of the two actions, must load at least one container, and is limited by the room on the ship. The optional free load on arrival at the Bank stays. | **Reversed.** The owner had ruled that sailing to the Bank is barred while the holding area is empty (`sail_bank_empty`). Now that a docked ship can load, waiting at the Bank makes sense, so sailing to the Bank is always legal. |
| **Bidding $0 in Bank auctions.** A new auction needs a bid of at least $1 or 1 container. | A container lot opens with at least $1 and a cash lot with at least 1 container. An outbid needs $1, or 1 container, more than the tile holds. | Confirms the ruling on opening bids. |
| **Containers on a bid tile** can't be sold, repriced or seized in a default. They leave the tile only when the auction is won or an opponent outbids. | They leave the lots when they go on the tile, so they can't be bought, repriced or seized. A default with nothing else to seize is forgiven. Their reserve tokens still count against storage limits. | None. The developer notes say these rules are unchanged. |
| **$0 bluff cards** only disguise how many cards a bid has. They have no other effect and can be removed. | Removed, together with the bid revisions they served. A bid is the cash alone, placed once and hidden until every bidder has bid. | The owner had ruled to include them, with optional bid revisions. A later ruling dropped both: a delivery bid is a simultaneous blind bid. |
| **Delivery minimum bid**, added by the owner next to the errata: it is $0. | A bid is `0..=cash`. A player without cash can only bid $0. Bids of $0 all round are a tie and go to a runoff. A buyout at $0 costs nothing. | None. The rulebook states no minimum. |

Two points go beyond the errata's text, and the owner has confirmed both. A Harbor Purchase at the Bank must load at least one container, like every paid purchase action, because a paid load of nothing would only duplicate the no-op Reprice. And reversing `sail_bank_empty` is a consequence drawn from the errata, not something the owner stated first.

Tests: `docked_at_the_bank_a_harbor_purchase_loads_the_holding_area_for_free`, `a_harbor_purchase_at_the_bank_needs_something_to_load_and_room_for_it`, `a_ship_docked_at_the_bank_loads_the_containers_of_an_auction_it_won` and `the_bank_can_be_reached_with_nothing_to_collect` (sail); `a_container_lot_auction_opens_with_a_bid_of_at_least_one_dollar` (auction); `a_cash_lot_auction_opens_with_a_bid_of_at_least_one_container`, `the_containers_on_the_tile_are_not_laid_out_again_by_repricing`, `the_containers_on_the_tile_cannot_be_bought` and `the_containers_on_the_tile_cannot_be_seized_so_a_default_with_nothing_else_is_forgiven` (container_bids); `a_bid_is_final_and_the_next_opponent_bids_at_once`, `a_player_without_cash_can_only_bid_nothing`, `bids_of_nothing_all_round_are_a_tie` and `a_buyout_at_zero_costs_nothing` (delivery).

### Decision log

#### Modelling

| Decision | Source |
|---|---|
| The engine is a library: no CLI, UI or Python bindings. Only the 10th Anniversary rules are played. The rulebook's Classic variant (the first-edition rules) is out of scope. | Ruling |
| Actions are micro: a flat list of small legal actions, one small choice per `apply`, with a half-finished action kept in the state. Whole-action alternatives (a `Produce` with every arrangement in one action, say) were compared and rejected, as they give thousands of options per decision and no fixed action mask. | Ruling |
| `is_legal` is the single source of truth. An illegal `apply` returns an error and changes nothing. | Assumption |
| Every outcome has exactly one action sequence. Colors are picked in a fixed order, containers are laid out one at a time in a fixed order with the lots never going down within a color, and purchases, loads and seizures go lot by lot and color by color. | Assumption |
| A step with nothing left to choose ends by itself: a Produce that fits entirely asks for no colors, and a purchase or load ends once nothing more can be bought or loaded. | Assumption |
| All randomness is at setup, from a seeded xoshiro256** generator: the Bank's starting colors, the starting factories, the scoring cards and the first player. A seed replays the same game. | Assumption |
| The first player is random, which is the rulebook's alternative to "the player who last captained a seagoing vessel". | Reading |
| The secret simultaneous bids of a delivery auction are sequential hidden decisions, clockwise from the deliverer's left, revealed together. | Assumption |
| Cash is an integer, change is free and the supply never runs out. The card counts the owner supplied (36, 34, 26, 20 and 25 cards of $1, $2, $5, $10 and $20) are not enforced. | Reading, Ruling |
| The player "on your right" is the seat that plays just before you, since turns go clockwise. | Reading |

#### Turn

| Decision | Source |
|---|---|
| A turn is: pay interest, win a Bank auction, then exactly two actions. The same action may be taken twice, except Produce and Call Bank, which are once per turn. | Rulebook |
| There is no Pass action. A Reprice that leaves the arrangement as it was is the pass. `Reprice(Factory)` is always legal, and `Reprice(Harbor)` needs a container in the harbor. | Ruling |

#### Build, produce and purchases

| Decision | Source |
|---|---|
| Factory costs are $0, $6, $9 and $12, with a storage limit of 2 per factory. Warehouse costs are $0, $4, $5, $6 and $7, with a harbor limit of 1 per warehouse. Lot prices are $1 to $4 in the factory district and $2 to $6 in the harbor district. These numbers were read from the rulebook's pictures, checked again, and confirmed by the owner. | Ruling |
| Produce is legal whenever the $1 wages can be paid, even if it yields nothing (no room, or the supply has none of your colors). | Ruling |
| A factory whose color is out of the supply yields nothing. | Reading |
| A paid purchase or load action (Factory Purchase, Harbor Purchase, a load at the Bank) must buy or load at least one container. The free action on arrival may be skipped. | Ruling |
| Containers are bought one at a time, in lot and color order, and the seller is paid the lot price of each. The buyer's harbor room counts reserve tokens. Afterwards the buyer's harbor district is laid out again. | Assumption |
| Nothing tracks who produced a container, so a player may buy, and ship, a container of a color they produce themselves. The rulebook's "you can never buy or ship your own containers" holds because a purchase from your own districts and a visit to your own harbor are never allowed. | Reading |

#### Sailing

| Decision | Source |
|---|---|
| Sailing to the Island with an empty ship is forbidden. The rulebook doesn't rule against it, but it is meaningless. | Ruling |
| Sailing to the Bank from the ocean is always legal, even with nothing in the holding area. | Errata, as a consequence; Ruling (confirmed) |
| Sailing to an opponent's harbor is legal even if nothing there can be bought. | Assumption |
| The free anchor action at a harbor or the Bank is optional, and is skipped by itself when nothing could be bought or loaded. | Reading |

#### Delivery auctions

| Decision | Source |
|---|---|
| A delivery auction is a simultaneous blind bid: each opponent places one hidden bid, and the bids are revealed together once the last one is in. A bid can't be revised, and there are no cards to lay out: a bid is the cash alone. | Ruling |
| Opponents bid from $0 up to the cash they hold. A bid leaves the hand when it goes on the table, so it can't be spent on anything else, but a bidder may take a loan first. | Ruling, Assumption |
| Bids of $0 all round are a tie. | Assumption, announced |
| A tie gets one runoff. Each tied bidder adds cash on top of their first bid, which stays on the table, and the other bidders sit out. A tie after the runoff goes to the deliverer's choice. | Rulebook |
| The deliverer decides after the reveal even when there is no tie: accept a bidder, or buy out. Accepting pays the bid plus an equal subsidy from the supply and refunds the other tied bids. A buyout pays the bid into the Bank's cash lots, keeps the cargo, refunds all tied bids, and needs the cash. | Rulebook |

#### Loans, interest and default

| Decision | Source |
|---|---|
| Loans are free actions. They are offered at a player's own decision points: between their actions, when the deliverer decides, and to each bidder before they bid. They are not offered in the middle of a purchase, load, arrangement or Bank bid, as a loan taken earlier gives the same result because costs are known beforehand. | Assumption |
| A player may request at most 2 loans and repay at most 2 in one of their own turns, which stops repay-and-borrow loops. | Ruling |
| Repaying is only possible on the player's own turn, between actions, after interest. | Rulebook |
| Interest is due on the loans held when the turn begins. A player who is short and holds fewer than 2 loans takes a loan to pay it, and that loan does not count as a request. | Reading |
| A player at 2 loans who can't pay it all pays what they can. The rest is default: one container is seized for each loan whose interest is unpaid. | Reading |
| A seizure takes a container from the first non-empty place in this order: scoring area on the Island, ship, holding area, harbor, factory. The player on the debtor's right chooses which container, and which color goes in each Bank container lot. Containers on a bid tile can't be seized. The debtor keeps the loan cards. With nothing to seize, the interest is forgiven for that turn. | Rulebook, Errata |
| Interest, and any default, come before a won auction is collected. The cash on a bid tile can't pay interest, and won winnings can't pay it or be seized for it. | Rulebook |

#### Off-Shore Bank

| Decision | Source |
|---|---|
| A payment is dealt $1 or 1 container at a time, starting at lot I for every payment, wrapping around and skipping a lot with an auction token of the kind being paid. The payer chooses the colors per lot, except in a default, where the player on the debtor's right does. | Rulebook |
| Call Bank is once per turn, and not in a turn in which an auction was won or the game was ended. | Rulebook |
| An auction can only start on a lot that holds something. A holder can't outbid themselves. | Ruling, Reading |
| A container lot is bid for with cash, face up on the cash tile, and that cash can't be spent, even on interest. The opening bid is at least $1 and an outbid at least $1 more. | Rulebook, Errata |
| A cash lot is bid for with containers from the factory and harbor districts, and only their number matters. The opening bid is at least 1 container and an outbid at least 1 more. Reserve tokens mark where they came from and count against storage limits. A bid holds at most 10 containers, as the box has 10 reserve tokens. | Rulebook, Errata, Ruling (the cap, confirmed) |
| A cash-lot bid is declared one container at a time, in one fixed order: the factory district first, then lot by lot and color by color. This only fixes the order in which the containers are named, so that every set of containers has exactly one action sequence. Any container in the factory and harbor districts can be bid. | Assumption |
| An outbid holder gets the cash or containers back. Only the returned containers are laid out again, in any lots of the district they came from, and the holder decides where. The containers that were already on their board stay in their lots and are not priced again. | Ruling |
| The holder wins at the start of their next turn, after interest. A won container lot goes to the holding area and a won cash lot to the hand. The winner pays into the other kind of lots, and chooses which colors go where. | Rulebook |

#### Game end

| Decision | Source |
|---|---|
| The game ends when the supply is out of 2 colors. Only Produce lowers the supply, so the check comes right after Produce. The active player finishes their turn, so a second action is still taken, and no further interest is paid. | Rulebook |
| No Call Bank in the turn that ends the game, in either order. Once a Produce has emptied the second color, Call Bank is barred. A Produce that would end the game is illegal if the Bank was already called this turn, so colors that would end the game can't be chosen then. | Ruling |
| Winning an auction at the start of a turn doesn't stop the game ending that turn by Produce. The ban is on Call Bank, which a won auction doesn't involve. | Reading, Ruling (confirmed) |
| Active auctions go to the highest bidder when the game ends. Won containers go to the holding area, worth $3 each. A cash-lot winner's containers go to the Bank with no choice, as they no longer score. | Rulebook, Assumption |

#### Final scoring

| Decision | Source |
|---|---|
| Three scoring cards are read from the rulebook (White, Blue and Green have the $10) and two are inferred: all five are rotations of the color cycle Red, Blue, Yellow, White, Green. A card's $10 color is followed by the two-value color, then $6, $4 and $2. The owner accepted the two inferred cards. | Ruling |
| On a tie for the most common color, the engine discards for the player: the two-value color if it is among the tied colors, otherwise the cheapest tied color. That is the best choice for the player, so there is no decision to ask. | Reading |
| A score can be negative, as each loan costs $11. The most cash wins, ties go to the most containers in the factory district, and remaining ties share the win. | Rulebook |

#### Observation

| Decision | Source |
|---|---|
| An observation hides other players' cash and scoring cards until the game is over, and hides face-down bids. A hand's card count is not tracked, so cash stays fully hidden. | Assumption, announced |
| The seed and the list of legal actions are not part of an observation. | Assumption |

#### Deadlock

| Decision | Source |
|---|---|
| A table where every player has $0, owes every loan there is and has no container, with no Bank auction running and only idle moves (sailing and repricing) left for every seat, ends the game when a turn ends. The rulebook has no rule for it, and the game would otherwise be stuck. The players are scored as in any other ending. About 0.17% of the playouts with a sensible policy reach this state. | Ruling |

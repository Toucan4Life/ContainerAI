# Phase H0 review (Part A): provisional

**Status.** An external reviewer assessed the material the user supplied (the hand-off prompt and its figures). The reviewer had **no access to the source tree or the stored report files**, so this is not a code-and-data audit: nothing here was run or recomputed by the reviewer. The "Implementer's check" column was added afterwards by the implementer, who does have the repo; it says what was checked and what was not. Transcribed from the user's paste; the wording is condensed. After the user accepted it as the working basis, the implementer added the **addendum at the end** (the paired old-versus-new comparison, the source-level checks, and the pre-registered Showcase measure).

## Verdicts

| Area | Verdict | Reason |
|---|---|---|
| Fidelity of the claims | Sound with caveats | The conclusions fit the supplied figures; several claims remain one-off or unverified. |
| Economic quality | Unsound / not established | The archetypes behave differently, but nothing shows all three are economically healthy. |
| Game quality | Sound with caveats | Fine in heuristic-only play with the old profiles; the appetite change raises defaults and stalls on a 40-seed sample. |

## Findings

| ID | Severity | Finding | Treatment asked for | Implementer's check |
|---|---|---|---|---|
| R1 | Major | The Producer gate is weak: `produced` passes, a no-shaping control also passes, nothing corroborates. | Keep `produced` as a descriptive identity metric, not proof of differentiation. | Not re-run. The prompt's claim (3 of 4 runs pass, one control included, nothing corroborates) comes from the plan. Accepted as wording. |
| R2 | Major | `sell_through` is not production sell-through: `sold` includes resales, so it can exceed 1. | Keep as a labelled legacy diagnostic; report produced, bought, sold, shipped separately. | Consistent with the plan (`sold` per container produced 1.5-3.4). Accepted. |
| R3 | Major | Appetite change: defaults 6.8 to 16.7% (3 seats), 2.9 to 6.6% (4), 1.6 to 4.7% (5); 3-seat stalls 11 of 12,000 to 21 of 1,200. Sample small; compare old and new on the same seeds. | Full DEV baseline; old against new on the same 40 seeds. | **Corrected.** An earlier version of this cell said the rise was "not sampling variation" and "probably real" without a paired analysis, and assumed the new run used seeds 0-39. Both are fixed in addendum A1: no stored new-profile run exists; seeds and design were verified; paired seed-clustered intervals were computed at 400 and at 40 seeds. |
| R4 | Major | The "under .009 is noise" threshold is not justified: .009 is the Stage D equilibrium gain bound, not the uncertainty of a paired Stage E comparison. | Remove it; estimate each outcome's uncertainty with the paired, seed-clustered bootstrap. | Agreed; removed from the hand-off. |
| R5 | Major | The decision rule has no Showcase identity measure. | Pre-register one next to the Producer's production lead. | Agreed; added to B6 (candidates from the existing Identity table). |
| R6 | Major | The four common guards need scrutiny: the capex buffer (also blocks building while a loan is outstanding, `archetype.rs:187`) restricts investment, idle markdown shapes pricing. | Document each guard's behavioural effect. | Agreed; added as a reading-and-documenting task in B7. No ablation run is approved. |
| R7 | Major | EGTA rates and `interact` rows may be mixed up. | Two labelled panels, or one common protocol. | Already labelled in B8; made explicit as two panels. |
| R8 | Minor | "Every rotation" may be read as every permutation. | State it follows the cyclic-rotation design. | Already in the hand-off (2.2, B5). |

## Checked and consistent (from the supplied material only)

Stage D payoff-table readings; the Stage E game count (133 games per seed per variant, 53,200 per variant, 478,800 with two nulls); the wording of the equilibrium estimate.

## Not checked (the reviewer had no repo)

The source, the 78,000-game arrays, the 40-seed run, the gate code, the tests, the dead-seat derivation, the deleted diagnostics. **Since then the implementer checked** (addendum): the identity test's action horizon, the four guards, the limits of `unsold` and the cash ledger, the dead-seat derivation, and the old-versus-new comparison. **Still unchecked:** the gate code and tests beyond the identity test, the deleted one-off diagnostics, and the plan's "Follow-up" about Producer factory appetites 15 / 13 / 12. A reviewer with repo access is still needed for an audit of those.

## Go / no-go

1. **Stage E: conditional go.** Final score as the primary outcome, cash change supporting; seed-clustered paired bootstrap; .009 only as history; a Showcase identity guardrail; `sell_through` and `unsold` labelled, with the underlying counts reported. Baseline: the new profiles, labelled development-seed evidence.
2. **Exposing the heuristics in `OPPONENTS`: no-go for now.** The payoff table describes the three hand-set bots, not their value as training opponents; the old profiles also lose heavily to the nets. Evidence of useful offers and situations is needed first.

## Next experiments, ranked

1. Stage E price deviations. 2. Matched stall-baseline reporting, all three heuristics against `classic`. 3. Fresh-seed confirmation once any profile change is adopted.

## Addendum (implementer): the paired comparison, the source-level checks, the Showcase measure

Run or read by the implementer after the review was accepted. All numbers are **development evidence**: `DEV` seeds, and the appetites were chosen after looking at the same seeds. Intervals are 95% percentile bootstraps that resample the seeds (1,000 resamples; all rotations and compositions of a seed stay together), per table size, **not adjusted for the number of rows**.

### A1. Old against new profiles

- **No stored new-profile run exists.** `reports/strategist/egta_h0` is the old-profile run (400 seeds, 78,000 games). The plan's "new profiles, 40 seeds" came from a scratch directory that was deleted. It was regenerated (`egta run --out DIR --seeds 40`) and reproduces the plan: 7,800 games, 21 stalled, defaults 16.7 / 6.6 / 4.7%.
- **Same seeds, same design: verified.** The first 40 seeds of the stored run and of the regenerated run have identical seed ids and identical games (table size, seed, composition, seat arrangement) at every size. Both runs have sizes 3, 4, 5, `seed_offset` 0, block `DEV`, horizon 2,500 and the same policies. `training/egta_compare.py` refuses to compare runs for which that fails.
- **The stored old run is "current code with the old appetites".** Live code with only the six appetite constants reset reproduces `egta_h0` exactly (every array of `records.npz`, the counters and the breakdown). So the six constants are the only difference between the runs; this does not say which of the six matters.
- **Paired, 400 seeds against 400** (`python -m training.egta_compare --old reports/strategist/egta_h0 --new <new-profile run>`; the new run was a scratch run of the same design, and Stage E's `egta_h0b` must reproduce it). Percentages of games or seats, old → new, then the difference new − old in points with its interval:

| quantity | 3 seats | 4 seats | 5 seats |
|---|---|---|---|
| stalled games | 0.09% → 1.63%: +1.54 [+1.27, +1.82] | 0 → 0 | 0 → 0 |
| completed games with a default | 6.76% → 16.98%: +10.21 [+9.27, +11.13] | 2.90% → 7.06%: +4.16 [+3.74, +4.63] | 1.63% → 4.53%: +2.90 [+2.64, +3.17] |
| Producer seats dead at the end | 1.58% → 2.34%: +0.76 [+0.34, +1.21] | 0.64% → 1.37%: +0.73 [+0.55, +0.92] | 0.24% → 0.75%: +0.52 [+0.43, +0.61] |
| Producer seats stuck at the end | 3.84% → 6.07%: +2.23 [+1.53, +2.90] | 1.04% → 2.19%: +1.15 [+0.92, +1.38] | 0.40% → 1.08%: +0.69 [+0.58, +0.80] |
| Shipper seats dead at the end | 1.48% → 4.90%: +3.42 [+2.90, +3.91] | 0.79% → 2.12%: +1.33 [+1.13, +1.54] | 0.45% → 1.21%: +0.76 [+0.65, +0.86] |
| Shipper seats stuck at the end | 3.10% → 11.22%: +8.12 [+7.45, +8.87] | 1.24% → 3.32%: +2.07 [+1.83, +2.33] | 0.62% → 1.76%: +1.13 [+1.00, +1.26] |

  No Showcase seat is dead or stuck in either run. No stalls at 4 or 5 seats in either run (a loose seed-level bound: under 0.8% of seeds at 95%). Ratios new / old with intervals: defaults 2.5 [2.3, 2.8], 2.4 [2.2, 2.7], 2.8 [2.5, 3.1]; Producer dead 1.5 [1.2, 1.9], 2.1 [1.8, 2.6], 3.2 [2.6, 4.1]; Shipper dead 3.3 [2.8, 4.0], 2.7 [2.3, 3.2], 2.7 [2.3, 3.1]; 3-seat stalls 17.8 [11.2, 36.6] (11 of 400 seeds had a stalled game before, 133 after).
- **Reading.** Every interval above excludes 0, so these rises are distinguishable from sampling variation on these seeds. Say no more than that: they are the seeds the appetites were chosen on, the intervals are unadjusted, and the comparison cannot say which of the six constants matters.
- **The 40-seed subset the plan first quoted** points the same way with wider intervals (defaults +10.79 [+8.17, +13.26], +3.42 [+2.04, +4.92], +2.69 [+1.86, +3.60]; 3-seat stalls 0 → 21 of 1,200, +1.75 [+0.92, +2.75]). There three intervals include 0: the Producer's dead share at 3 seats (+0.76 [-0.24, +1.84]) and at 4 seats (+0.38 [-0.34, +1.12]), and its stuck share at 4 seats (+0.91 [-0.03, +1.84]); at 400 seeds all three exclude 0. "Within noise at 3 seats" was a small-sample reading and is withdrawn.
- **Withdrawn wording.** "2.4 to 3.6 times" compared a 400-seed old rate with a 40-seed new rate (unpaired); paired at 400 seeds the Shipper's dead share is 2.7 to 3.3 times higher and the Producer's 1.5 / 2.1 / 3.2 times. "3-seat stalls 0.09% → 1.75%" mixed seed counts; paired it is 0.09% → 1.63%. "Defaults more than doubled" stands.
- **Where the 3-seat stalls are** (new profiles, 400 seeds, 1,200 games per composition, 196 stalls): Sc Sh Sh 75, Sc Sc Sh 34, Sh Sh Sh 33, P Sc Sh 21, P Sc Sc 11, P P P 9, P P Sh 6, P P Sc 4, P Sh Sh 3, Sc Sc Sc 0. 172 of the 196 involve a Shipper. A table of one archetype stalls in all three rotations of a seed together (P P P: 9 games in 3 seeds; Sh Sh Sh: 33 in 11), so stalls cluster by seed. Before the change (11 stalls): P Sh Sh 5, Sc Sc Sh 2, Sc Sh Sh 2, P P Sh 1, P Sc Sh 1.

### A2. Source-level checks

**The identity test cannot fail on a stall.** `each_heuristic_does_what_it_is_for_with_the_same_chances` (`tests.rs:442`) plays 40 seeds × 3 cyclic shifts of one Producer, one Showcase and one Shipper at 3 seats. Each game runs `for _ in 0..2500` (line 456), breaks on `is_terminal()` and never asserts that the game ended. Its three assertions are orderings: the Producer makes the most `Produce` moves, the Showcase the most purchases from factories, the Shipper the most accepted or bought-out deliveries. Stall coverage exists elsewhere: `the_heuristics_play_every_kind_of_game_to_its_end_alone_and_among_merchants` (`tests.rs:429`, assertion at 435) plays `every_table()` (3 lengths × seeds 0-3 × sizes 3-5) with a heuristic alone and next to a merchant, up to `LIMIT` = 20,000 actions (line 7), and passes. It does **not** cover mixed Producer / Showcase / Shipper tables, where EGTA finds the stalls, and a plain "no stalls" assertion there would fail at 3 seats (1.63% of games stall). Proposal, not implemented: count the games that reach the cap and report them, or assert none at 4 and 5 seats (none stall there in either run).

**The four guards** (`archetype.rs`). All use `spendable = cash − (loans + profile.reserve)` (line 181; reserve 1 / 2 / 1 for Producer / Showcase / Shipper). The module comment (lines 9-11) lists only the first two. None has a counter, so how often each binds is unmeasured; no ablation is approved.

| Guard | Rule | What it does beyond preventing deadlock |
|---|---|---|
| `CAPEX_BUFFER` 3 (lines 47, 186-188) | Build only with no loan outstanding and spendable ≥ cost + 3; otherwise the build is worth −1. Costs: factories $6 / $9 / $12, warehouses $4 / $5 / $6 / $7. | Any loan blocks all building, so it limits investment and thereby the appetites. It binds on the Producer and Shipper, which borrow in about 7-16% of seat-games; the Showcase never borrows. |
| `IDLE_MARKDOWN` 4 (lines 50, 249, 355) | After 4 consecutive Main decisions with best value ≤ 0 or a reprice, `target_lot` returns lot 0 in both districts; it also returns 0 on the last turn and past 85% of the supply. | A pricing rule: the profile's lot applies only while not stuck, not late and under 85% of the supply. It shapes pricing identity: a Showcase seat reprices 13.8 / 8.4 / 5.2 times a game at 3 / 4 / 5 seats in tables that seat a Shipper, against 4.9 / 4.7 / 4.2 in tables without one (new profiles, 400 seeds; old profiles 11.0 / 6.4 / 4.1 against 4.3 / 4.6 / 4.6). It also interacts with Stage E's price variants. |
| No `Produce` unless spendable > 0 (lines 393-397) | `Produce` is worth −1 if nothing can be made or spendable is 0, else max(made × factory_unit × left − 1, 0.05). | Directly caps the Producer's `produced` when it is near-broke or in debt. |
| No buy-out or bid above spendable cash (lines 285-288, 303, 530) | `BuyOut` only if the top bid ≤ spendable and worth − top > 2 × top + 0.25; a purchase priced above spendable is worth −1; bids are capped at spendable. | In debt a seat sells its cargo instead (to the bidder with the least on its island), which shapes the Shipper's delivery behaviour. |

**`unsold` and the cash ledger** (`egta.py:173`, `ledger.py`):
- `unsold` is NaN where the ledger shows a default and `1 + produced + bought − sold − shipped` otherwise. The NaN rule is a **default detector, not a Bank detector**: a seat that used the Bank would have a negative residual, would not be flagged, and its `unsold` would be wrong. This is latent for the heuristics (all eight Bank move types are 0 in the stored old run and in the 400-seed new run, and `games_with_open_market` is 0) and real for the nets. A seized seat loses containers its counters still show, so NaN is right for a defaulter. Containers that were loaded but never sailed count as unsold; the `1` is the starting container.
- The residual `bank_and_other` is never negative in either run. It is positive in 2,203 seat-games (old) and 5,659 (new), all defaults. At game level, games with a ledger default equal games with a seizure or a deal in the old run (2,191 each, all in both), but **not in the new run: 5,601 against 5,593, so 8 games have a ledger default and no seizure or deal**. All 8 are 3-seat games in which the Producer ends with $1, no island value, no leftovers and two loans, with 46-70 of interest due: nothing was seizable and the engine forgave the interest. So the docstring "positive exactly where the engine had the Bank seize something" (`egta.py`, `ledger_check`) was exact only for the old run; it now says "positive where interest went unpaid". Seizure and deal moves are recorded on a different seat than the defaulter, so compare them with defaults per game, never per seat.
- `flow_net_cash` and `net_cash` are final cash − `START_CASH` (20): not profit and not the score. Dead-seat shares cover completed games only: a stalled game records no `breakdown`.

**Dead-seat definition.** The strict signature (cash 0, no leftovers, two unpaid loans = $22) misses seats that are just as stuck. Two loans cost $2 of interest a turn; a seat with $1, no leftovers and two loans cannot pay it, cannot borrow (`MAX_LOANS` 2) or sell (nothing to sell), and the third guard stops it producing, so it is absorbing. A neighbour's wage can move such a seat between $0 and $1, so the strict signature alone misses about 40% of them (old run 1,301 strict against 2,275 with cash ≤ $1; new run 3,339 against 5,782). `egta_compare` reports both: **dead** (strict, for continuity with the numbers already in the plan) and **stuck** (cash ≤ $1 with two loans and no leftovers, the better measure of seats that stopped playing). The plan's older one-off "before" rates (Producer 3.0 / 1.2 / 0.5%, Shipper 2.3 / 0.9 / 0.1%) still do not reproduce under dead, stuck or any two-loans definition.

### A3. The Showcase identity measure (pre-registered)

The user asked for a fixed Showcase measure based on resale activity, with warehouses as support and the counts kept visible.

- **Primary: resale volume** per seat-game = min(`bought`, max(0, `sold` − `produced` − 1)) containers, completed games only. It is a conservative lower bound on what the seat resold: there are no per-district sold counters and `sold` mixes sales from the seat's own factory lot and from its harbor lot, so under "own production is sold first" the part of `sold` beyond `produced` and the starting container (the `1`) must be resales, capped by what was bought. Test for a deviating Showcase: the paired difference to the base Showcase seat in the same games, seed-clustered, per table size, plus a rank check against the other two archetypes in tables that seat all three.
- **Supporting: `warehouses_built`.** **Kept visible beside both:** `produced`, `bought`, `sold`, `shipped`, separately.
- **The numeric threshold** for "identity kept" is for the implementer to propose before the grid runs, marked *proposed, not approved*.
- **Baseline values, for orientation** (mean per seat, completed games, tables seating all three archetypes, 400 seeds, no intervals): resale Producer / Showcase / Shipper, new profiles: 1.5 / 13.4 / 14.1 at 3 seats, 3.5 / 16.2 / 13.9 at 4, 4.6 / 17.0 / 13.0 at 5; old profiles: 2.4 / 14.6 / 12.0, 4.6 / 20.8 / 8.0, 5.5 / 21.7 / 5.9. Showcase warehouses built, new: 3.1 / 3.1 / 3.3 (old 1.9 / 1.6 / 1.6). **The baseline's own rank is not Showcase-first at 3 seats:** the Shipper's resale (14.1) is slightly above the Showcase's (13.4) after the appetite change (before it the Showcase led, 14.6 against 12.0). The rank check therefore judges whether a variant makes the Showcase's position *worse than the baseline's*, and requires it to stay clearly above the Producer's; it does not require the Showcase to lead the Shipper at 3 seats.

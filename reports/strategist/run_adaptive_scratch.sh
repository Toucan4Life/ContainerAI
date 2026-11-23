#!/bin/bash
# Adaptive generalist from scratch (docs/strategist_plan.md): two seeds, two hours each, one after the other (the PC cannot
# take two runs). Same league and rules as run_adaptive.sh, but no --init: the trainer imitates the Merchant for the first 17%
# of the budget, then trains by PPO against the Phase H0 heuristics (70% of the opponent seats), snapshots and scripted bots.
#   bash reports/strategist/run_adaptive_scratch.sh
set -e
cd "$(dirname "$0")/../.."
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/ct}"
PY=".venv/bin/python"
OUT=runs/strategist
RULES="--no-same-turn-repay --min-bid 3 --buy-at-harbor --bank-needs-load --no-double-reprice"
mkdir -p "$OUT"
for s in 1 2; do
  [ -f "$OUT/adaptive_scratch_s$s/final.pt" ] || $PY -m training.train --name adaptive_scratch_s$s --runs "$OUT" --budget-minutes 120 $RULES \
    --league-nets none --peddler-share 0 --heuristic-share 0.7 --seed $s > "$OUT/adaptive_scratch_s$s.log" 2>&1
done

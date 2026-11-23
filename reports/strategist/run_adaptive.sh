#!/bin/bash
# Adaptive generalist (docs/strategist_plan.md): one net, one hour, from Peddler v2, against the three Phase H0 heuristics (70% of the
# opponent seats), snapshots of itself and the scripted bots. Peddler v2, Magnate and Consortium are never seated in training:
# they are the held-out opponents of the evaluation. The reward is the default one (rank and margin), with the five table rules
# Peddler v2 ships with.
#   bash reports/strategist/run_adaptive.sh
set -e
cd "$(dirname "$0")/../.."
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/ct}"
PY=".venv/bin/python"
OUT=runs/strategist
RULES="--no-same-turn-repay --min-bid 3 --buy-at-harbor --bank-needs-load --no-double-reprice"
mkdir -p "$OUT"
$PY -m training.train --name adaptive_s1 --runs "$OUT" --budget-minutes 60 --no-imitation --init training/league/peddler2.pt $RULES \
  --league-nets none --peddler-share 0 --heuristic-share 0.7 --seed 1 > "$OUT/adaptive_s1.log" 2>&1

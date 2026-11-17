#!/bin/sh
# Reproducible interaction + competitive baselines: `sh reports/interaction/baseline.sh [GAMES] [SEED]`.
# One focus bot per table (random seat) against three pools, 3 to 5 seats; one JSON file per cell.
# Consortium is psro08. Output: reports/interaction/baseline_<focus>_<pool>.json
set -e
GAMES=${1:-1500}
SEED=${2:-1}
cd "$(dirname "$0")/../.."
cargo build --release -p container-arena --bin interact
nets=magnate,dealer,consortium
mixed=magnate,dealer,consortium,merchant,solitaire,discounter,bargain,trader
cheap=discounter,bargain,trader
for focus in magnate dealer consortium; do
  for pool in nets mixed cheap; do
    eval "p=\$$pool"
    ./target/release/interact --focus $focus --pool $p --players 3,4,5 --games $GAMES --seed $SEED \
      --out reports/interaction/baseline_${focus}_${pool}.json
  done
done

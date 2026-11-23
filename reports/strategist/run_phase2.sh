#!/bin/bash
# Phase 2 archetype runs (docs/strategist_plan.md): 14 runs of 30 minutes from Peddler v2, one at a time (PARALLEL=2 needs about 9 GB of GPU memory)
# (about 7 h; 3.5 h with PARALLEL=2), then the divergence gate on every final checkpoint.
#   bash reports/strategist/run_phase2.sh            # train, then gate
#   bash reports/strategist/run_phase2.sh gate       # gate only
set -e
cd "$(dirname "$0")/../.."
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/ct}"
PY=".venv/bin/python"
OUT=runs/strategist
RULES="--no-same-turn-repay --min-bid 3 --buy-at-harbor --bank-needs-load --no-double-reprice"
COMMON="--runs $OUT --budget-minutes 30 --no-imitation --init training/league/peddler2.pt $RULES"
SHAPE="--win-slack 0.05 --shaping-weight 0.3"
mkdir -p "$OUT" reports/strategist/gate

jobs_list() {
  for s in 1 2; do
    echo "producer_s$s --shaping-objective producer $SHAPE --seed $((100 + s))"
    echo "shipper_s$s --shaping-objective island $SHAPE --seed $((200 + s))"
    echo "shipper_capped_s$s --shaping-objective island $SHAPE --bid-within-value --seed $((300 + s))"
    echo "showcase_variety_s$s --shaping-objective showcase --showcase-variant variety $SHAPE --seed $((400 + s))"
    echo "showcase_fair_s$s --shaping-objective showcase --showcase-variant fair $SHAPE --seed $((500 + s))"
    echo "showcase_sales_s$s --shaping-objective showcase --showcase-variant sales $SHAPE --seed $((600 + s))"
    echo "control_s$s --seed $((700 + s))"
  done
}

if [ "$1" != "gate" ]; then
  jobs_list | xargs -P "${PARALLEL:-1}" -I{} sh -c 'set -- {}; name=$1; shift; [ -f '"$OUT"'/$name/final.pt ] || '"$PY"' -m training.train --name $name '"$COMMON"' "$@" > '"$OUT"'/$name.log 2>&1'
fi

gate() { # archetype run [extra gate args]
  [ -f "$OUT/$2/final.pt" ] || return 0
  $PY -m training.gate "$1" "$OUT/$2/final.pt" --json "reports/strategist/gate/$2_$1.json" "${@:3}" > "reports/strategist/gate/$2_$1.md" 2>&1 || true
}
CAPPED="min_bid=3,no_same_turn_repay,buy_at_harbor,bank_needs_load,no_double_reprice,bid_within_value"
for s in 1 2; do
  gate producer producer_s$s
  gate shipper shipper_s$s
  gate shipper shipper_capped_s$s --rules "$CAPPED"
  for v in variety fair sales; do gate showcase showcase_${v}_s$s; done
  # a control that passes a gate would mean the gate measures drift, not shaping
  for a in producer showcase shipper; do gate $a control_s$s; done
done

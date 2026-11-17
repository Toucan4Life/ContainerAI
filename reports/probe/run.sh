#!/bin/sh
# Cheap-offer probe: `sh reports/probe/run.sh`. 250 positions x 24 paired rollouts per cell, seed 3.
cd "$(dirname "$0")/../.."
cargo build --release -p container-arena --bin probe
W=crates/container-bots/weights
cheap=discounter,bargain,trader
mixed=magnate,dealer,consortium,merchant,discounter,bargain,trader
run() { ./target/release/probe --net $W/$2.bin --rules $3 --pool $4 --positions 250 --rollouts 24 --seed 3 --out reports/probe/$1.json; }
run dealer_cheap dealer dealer $cheap
run magnate_cheap magnate none $cheap
run consortium_cheap psro08 none $cheap
run dealer_mixed dealer dealer $mixed
run consortium_mixed psro08 none $mixed

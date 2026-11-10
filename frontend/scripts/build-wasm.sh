#!/usr/bin/env bash
# Builds the Rust engine for the browser into src/wasm (generated, not committed).
set -euo pipefail
cd "$(dirname "$0")/.."
root=$(cd .. && pwd)
target_dir=$(cd "$root" && cargo metadata --format-version 1 --no-deps | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')

(cd "$root" && cargo build -p container-web --target wasm32-unknown-unknown --release)
wasm-bindgen --target web --out-dir src/wasm --out-name container_web \
  "$target_dir/wasm32-unknown-unknown/release/container_web.wasm"

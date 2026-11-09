#!/usr/bin/env bash
set -euo pipefail

rustup component add clippy rustfmt

python -m venv .venv
.venv/bin/pip install --upgrade pip
# cu124 wheels work with the RTX 3080 (Ampere) and recent host drivers.
.venv/bin/pip install torch numpy --index-url https://download.pytorch.org/whl/cu124
.venv/bin/pip install maturin pytest ruff tensorboard

.venv/bin/python - <<'EOF'
import torch
print("torch", torch.__version__, "| CUDA available:", torch.cuda.is_available())
if torch.cuda.is_available():
    print("GPU:", torch.cuda.get_device_name(0))
EOF

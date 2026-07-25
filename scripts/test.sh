#!/usr/bin/env bash
# Run the full Rust test suite.
# eBPF-specific tests are gated by cfg(target_os = "linux") so this is safe on macOS.
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

echo "--- tests ---"
cargo test --workspace

echo "--- ok ---"

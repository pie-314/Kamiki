#!/usr/bin/env bash
# Run fmt check + clippy across the Rust workspace.
# Exit code 0 = clean, non-zero = something to fix.
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

echo "--- fmt check ---"
cargo fmt --all -- --check

echo "--- clippy ---"
cargo clippy --workspace -- -D warnings

echo "--- ok ---"

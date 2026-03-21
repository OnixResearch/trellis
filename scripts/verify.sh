#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

echo "=== Clippy ==="
cargo clippy --all-targets -- -D warnings
echo "✓ clippy clean"
echo ""

echo "=== Tests ==="
cargo test
echo ""

echo "=== Verus verification ==="
./scripts/verus.sh verus/lib.rs
echo ""

echo "=== Tracey coverage ==="
tracey query status
echo ""
tracey query uncovered
echo ""
tracey query untested

#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

echo "=== Verus verification ==="
if command -v verus &>/dev/null; then
    verus --crate-type=lib verus/lib.rs
    echo "✓ Verus: all proofs pass"
else
    echo "⚠ verus not found — skipping formal verification"
    echo "  Install: https://github.com/verus-lang/verus"
fi

echo ""
echo "=== Tracey coverage ==="
tracey query status
echo ""
tracey query uncovered
echo ""
tracey query untested

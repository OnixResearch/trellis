#!/usr/bin/env bash
# Full verification pipeline. Exits non-zero on any failure.
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

FAIL=0

echo "=== Coverage: r[impl] on every pub fn ==="
if ./scripts/check-coverage.sh; then
    :
else
    FAIL=1
fi
echo ""

echo "=== Coverage: ensures on every verified pub fn ==="
if ./scripts/check-ensures.sh; then
    :
else
    FAIL=1
fi
echo ""

echo "=== Proof-gap inventory drift ==="
if ./scripts/check-proof-gap-inventory.rs; then
    echo "✓ proof-gap inventory matches expected counts, sites, and classifications"
else
    echo "✗ proof-gap inventory drift detected"
    FAIL=1
fi
echo ""

echo "=== Clippy ==="
if cargo clippy --all-targets -- -D warnings; then
    echo "✓ clippy clean"
else
    echo "✗ clippy failed"
    FAIL=1
fi
echo ""

echo "=== Tests ==="
if cargo test; then
    echo "✓ tests pass"
else
    echo "✗ tests failed"
    FAIL=1
fi
echo ""

echo "=== Verus verification ==="
if ./scripts/verus.sh src/lib.rs; then
    echo "✓ verus proofs pass"
else
    echo "✗ verus verification failed"
    FAIL=1
fi
echo ""

echo "=== Tracey coverage ==="
tracey query status
echo ""

UNCOVERED=$(tracey query uncovered 2>&1)
echo "$UNCOVERED"
if echo "$UNCOVERED" | grep -q "^verified-logic/rust: 0 uncovered"; then
    echo "✓ no uncovered requirements"
else
    echo "✗ uncovered requirements found"
    FAIL=1
fi
echo ""

UNTESTED=$(tracey query untested 2>&1)
echo "$UNTESTED"
if echo "$UNTESTED" | grep -q "^verified-logic/rust: 0 untested"; then
    echo "✓ no untested requirements"
else
    echo "✗ untested requirements found"
    FAIL=1
fi
echo ""

if [ "$FAIL" -eq 0 ]; then
    echo "=== ALL CHECKS PASSED ==="
else
    echo "=== SOME CHECKS FAILED ==="
    exit 1
fi

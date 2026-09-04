#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
DOC="$ROOT/docs/proof-examples.md"
POSITIVE="tests/proof_examples/basic_refinement.rs"
NEGATIVE="tests/proof_examples/fixtures/broken_refinement.rs"
LOG_DIR="$ROOT/target-test/proof-examples"
EXPECTED_NEGATIVE_PATTERN='postcondition|ensures|failed|error'

mkdir -p "$ROOT/.pi/tmp" "$LOG_DIR"
export TMPDIR="${TMPDIR:-$ROOT/.pi/tmp}"

if [ -z "${VERUS_ROOT:-}" ] \
  || { [ ! -x "${VERUS_ROOT:-}/rust_verify" ] \
    && [ ! -x "${VERUS_ROOT:-}/bin/octet-production-verus" ]; }; then
  export VERUS_ROOT="$(nix build .#verus --no-link --print-out-paths)"
fi

require_doc_text() {
  local needle="$1"
  if ! grep -F -- "$needle" "$DOC" >/dev/null; then
    echo "✗ docs/proof-examples.md missing: $needle" >&2
    exit 1
  fi
}

require_doc_text "$POSITIVE"
require_doc_text "$NEGATIVE"
require_doc_text "./scripts/verus.sh $POSITIVE"
require_doc_text "./scripts/verus.sh $NEGATIVE"
require_doc_text "expected to fail"
require_doc_text "./scripts/check-proof-examples.sh"

"$ROOT/scripts/verus.sh" "$POSITIVE" >"$LOG_DIR/basic_refinement.verus.txt" 2>&1

if "$ROOT/scripts/verus.sh" "$NEGATIVE" >"$LOG_DIR/broken_refinement.verus.txt" 2>&1; then
  echo "✗ negative proof fixture unexpectedly verified: $NEGATIVE" >&2
  exit 1
fi

if ! grep -E "$EXPECTED_NEGATIVE_PATTERN" "$LOG_DIR/broken_refinement.verus.txt" >/dev/null; then
  echo "✗ negative proof fixture failed without expected diagnostic shape" >&2
  cat "$LOG_DIR/broken_refinement.verus.txt" >&2
  exit 1
fi

printf '✓ runnable proof examples verified; negative fixture failed as expected\n'

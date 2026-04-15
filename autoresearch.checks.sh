#!/usr/bin/env bash
set -euo pipefail
export CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic
cargo test --quiet
cargo clippy --all-targets -- -D warnings
./scripts/verus.sh src/lib.rs
./scripts/check-ensures.sh
tracey query uncovered
tracey query untested

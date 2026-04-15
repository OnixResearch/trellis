#!/usr/bin/env bash
set -euo pipefail
cargo test --quiet
cargo clippy --all-targets -- -D warnings
./scripts/verus.sh src/lib.rs
./scripts/check-ensures.sh
tracey query uncovered
tracey query untested

#!/usr/bin/env bash
# Wrapper around verus rust_verify with correct nix store paths.
# Usage: ./scripts/verus.sh [file.rs] [extra args...]
# Default file: src/lib.rs
set -euo pipefail

VERUS_ROOT=${VERUS_ROOT:-/nix/store/c4zmcsmq6hyy2b7bg79wdrga9a2kszhm-verus-0.2026.03.22.5e66329}
RUST_TOOLCHAIN=${RUST_TOOLCHAIN:-/nix/store/gbrj4zlys951xy55wqwr74m0kmiz6xr1-rust-default-1.96.0-nightly-2026-03-21}

FILE="${1:-src/lib.rs}"
shift 2>/dev/null || true

cd "$(git rev-parse --show-toplevel)"

VERUS_Z3_PATH="$VERUS_ROOT/z3" \
LD_LIBRARY_PATH="$RUST_TOOLCHAIN/lib" \
exec "$VERUS_ROOT/rust_verify" \
  --crate-type=lib \
  --extern vstd="$VERUS_ROOT/libvstd.rlib" \
  --extern builtin="$VERUS_ROOT/libverus_builtin.rlib" \
  --extern builtin_macros="$VERUS_ROOT/libverus_builtin_macros.so" \
  --extern state_machines_macros="$VERUS_ROOT/libverus_state_machines_macros.so" \
  -L "$VERUS_ROOT/lib" \
  --edition 2021 \
  "$FILE" "$@"

#!/usr/bin/env bash
# Wrapper around verus rust_verify with correct nix store paths.
# Usage: ./scripts/verus.sh [file.rs] [extra args...]
# Default file: src/lib.rs
set -euo pipefail

VERUS_ROOT=/nix/store/7hr3z8ss29pxl29pllv6kchlk9kbckys-verus-root-0.2026.01.30
RUST_TOOLCHAIN=/nix/store/2i2rgbachy91xi97xfc07y4s3abgksq5-rust-default-1.93.0

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

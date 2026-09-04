#!/usr/bin/env bash
# Run Trellis proofs with Octet's published production Verus package.
# Usage: ./scripts/verus.sh [file.rs] [extra args...]
# Default file: src/lib.rs
set -euo pipefail

COMMAND_NOT_FOUND_EXIT_CODE=127
VERUS_EDITION=2021

if [ "$#" -gt 0 ]; then
  FILE="$1"
  shift
else
  FILE="src/lib.rs"
fi

cd "$(git rev-parse --show-toplevel)"

if [ -z "${VERUS_ROOT:-}" ] && [ "$FILE" = "src/lib.rs" ] && [ "$#" -eq 0 ]; then
  SYSTEM="$(nix eval --raw --impure --expr 'builtins.currentSystem')"
  exec nix build ".#checks.${SYSTEM}.verus-proofs" --no-link -L
fi

if [ -z "${VERUS_ROOT:-}" ]; then
  VERUS_ROOT="$(nix build .#verus --no-link --print-out-paths)"
fi

if [ -x "$VERUS_ROOT/bin/octet-production-verus" ]; then
  VERUS_BIN="$VERUS_ROOT/bin/octet-production-verus"
  VERUS_LIB_ROOT="$VERUS_ROOT/libexec/verus"
elif [ -x "$VERUS_ROOT/rust_verify" ]; then
  VERUS_BIN="$VERUS_ROOT/rust_verify"
  VERUS_LIB_ROOT="$VERUS_ROOT"
else
  echo "production Verus not found under $VERUS_ROOT" >&2
  echo "Set VERUS_ROOT to the Octet package or a compatible unpacked Verus root." >&2
  exit "$COMMAND_NOT_FOUND_EXIT_CODE"
fi

export VERUS_ROOT="$VERUS_LIB_ROOT"
export VERUS_Z3_PATH="$VERUS_LIB_ROOT/z3"
if [ -n "${RUST_TOOLCHAIN:-}" ]; then
  export LD_LIBRARY_PATH="$RUST_TOOLCHAIN/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
fi

exec "$VERUS_BIN" \
  --crate-type=lib \
  --extern vstd="$VERUS_LIB_ROOT/libvstd.rlib" \
  --extern builtin="$VERUS_LIB_ROOT/libverus_builtin.rlib" \
  --extern builtin_macros="$VERUS_LIB_ROOT/libverus_builtin_macros.so" \
  --extern state_machines_macros="$VERUS_LIB_ROOT/libverus_state_machines_macros.so" \
  -L "$VERUS_LIB_ROOT" \
  --edition "$VERUS_EDITION" \
  "$FILE" "$@"

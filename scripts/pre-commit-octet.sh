#!/usr/bin/env bash
set -euo pipefail

missing_nix_exit=127

if ! command -v nix >/dev/null 2>&1; then
  echo "pre-commit-octet: nix is required to run the repo-pinned Octet check" >&2
  exit "$missing_nix_exit"
fi

system=$(nix eval --raw --impure --expr 'builtins.currentSystem')
exec nix build ".#checks.${system}.octet" --no-link -L

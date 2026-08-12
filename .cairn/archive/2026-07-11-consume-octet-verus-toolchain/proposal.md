## Why

Trellis currently reconstructs Octet's pinned `verusfmt` package when the Octet input does not expose it. That fallback duplicates release URLs, hashes, wrappers, and identity text, so a formatter-policy change can drift independently from the stack-owned toolchain profile.

## What Changes

- Pin the Octet flake input to the published toolchain-profile revision.
- Consume Octet's `octet-verusfmt` and `octet-verus-toolchain-profile` packages directly.
- Fail closed when the central profile identity, formatter identity, or Trellis compatibility row drifts.
- Preserve Trellis's independent local Verus verifier pin until a separate corpus-backed upgrade trial passes.

## Impact

- **Files**: `flake.nix`, `flake.lock`, README documentation, and this Cairn change.
- **Testing**: central-profile positive and negative checks, formatter fixtures and ratchet, corpus intake, existing Verus proofs, and Cairn gates.

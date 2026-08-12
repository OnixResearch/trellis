## Why

The proof-review scanner found that `src/r1cs_system.rs` still has a catalogue proof whose postcondition is only `ensures true`. That satisfies Tracey mechanically but does not state the concrete multi-row system behaviours promised by the requirement.

## What Changes

- Replace the vacuous R1CS system coverage proof with explicit implications for invalid length rejection, active modulus mismatch rejection, malformed system/witness rejection, failed active-row rejection, and satisfied-system decomposition.
- Add focused positive and negative runtime tests for missing finite system cases surfaced by the strengthened coverage contract.
- Keep the trusted-boundary inventory unchanged; this is proof/test coverage hardening only.
- Save focused validation evidence before archive.

## Impact

- **Files**: `src/r1cs_system.rs`, Cairn change artifacts, synced requirement text, and archived evidence.
- **Testing**: focused `r1cs_system` cargo tests, Verus over `src/lib.rs`, proof-review scanner, Tracey direct queries, Cairn validation/gates, and `TMPDIR=/tmp ./scripts/verify.rs` before archive.

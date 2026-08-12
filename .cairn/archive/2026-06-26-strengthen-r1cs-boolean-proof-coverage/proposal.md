## Why

The proof-review scanner found that `src/r1cs_boolean.rs` still has a catalogue proof whose postcondition is only `ensures true`. That satisfies Tracey mechanically but does not state the concrete boolean rejection and row-encoding behaviours promised by the requirement.

## What Changes

- Replace the vacuous R1CS boolean coverage proof with explicit implications for malformed shape rejection, malformed witness rejection, missing one-wire rejection, unsatisfied encoded-row rejection, and admitted row validity.
- Add focused positive and negative runtime tests for missing finite boolean cases surfaced by the strengthened coverage contract.
- Keep the trusted-boundary inventory unchanged; this is proof/test coverage hardening only.
- Save focused validation evidence before archive.

## Impact

- **Files**: `src/r1cs_boolean.rs`, Cairn change artifacts, synced requirement text, and archived evidence.
- **Testing**: focused `r1cs_boolean` cargo tests, Verus over `src/lib.rs`, proof-review scanner, Tracey direct queries, Cairn validation/gates, and `TMPDIR=/tmp ./scripts/verify.rs` before archive.

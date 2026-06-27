## Why

The proof-review scanner found that `src/r1cs_public_inputs.rs` still has a catalogue proof whose postcondition is only `ensures true`. That satisfies Tracey mechanically but does not state the concrete public-input binding and statement-admission behaviours promised by the requirement.

## What Changes

- Replace the vacuous R1CS public-input coverage proof with explicit implications for invalid binding length rejection, malformed witness/modulus rejection, out-of-range binding rejection, mismatched value rejection, malformed statement-system rejection, malformed statement-public-input rejection, and satisfied-statement decomposition.
- Add focused positive and negative runtime tests for missing finite public-input cases surfaced by the strengthened coverage contract.
- Keep the trusted-boundary inventory unchanged; this is proof/test coverage hardening only.
- Save focused validation evidence before archive.

## Impact

- **Files**: `src/r1cs_public_inputs.rs`, Cairn change artifacts, synced requirement text, and archived evidence.
- **Testing**: focused `r1cs_public` cargo tests, Verus over `src/lib.rs`, proof-review scanner, Tracey direct queries, Cairn validation/gates, and `TMPDIR=/tmp ./scripts/verify.rs` before archive.

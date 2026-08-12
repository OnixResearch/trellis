## Why

The proof-review scanner found that `src/r1cs_gates.rs` still has a catalogue proof whose postcondition is only `ensures true`, plus several empty proof bodies whose value depends on reading the surrounding spec. This weakens reviewability: Tracey coverage exists, but the coverage proof itself does not state the concrete behaviours that the requirement promises.

## What Changes

- Replace the vacuous R1CS gate coverage proof with explicit implications for malformed gate rejection, malformed witness rejection, one-wire rejection, and encoded-row rejection.
- Keep the existing trusted-boundary inventory unchanged; this is proof/test hardening only.
- Add focused positive and negative runtime tests for the missing finite gate cases called out by the proof-review packet.
- Save focused validation evidence for tests, Verus, Tracey/Cairn coverage, and the proof-review scanner.

## Impact

- **Files**: `src/r1cs_gates.rs`, Cairn change artifacts, and archived evidence.
- **Testing**: focused `r1cs_gates` cargo tests, Verus over `src/lib.rs`, proof-review candidate scan, Tracey direct queries, Cairn validation/gates, and `TMPDIR=/tmp ./scripts/verify.rs` before archive.

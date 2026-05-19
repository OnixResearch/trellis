## Why

`src/chain.rs` already proves the core linear hash-chain story: deterministic entry hashing, validity, append preservation, prefix validity, agreement, divergence detection, and single-entry validity. The next useful slice is to make common consumers stop re-proving small indexing and commitment facts by adding named primitives for tips, extension, checkpoints, forks, batch append, shape validity, tamper classification, domain-separated hash inputs, runtime BLAKE3 alignment, and optional compact historical proofs.

## What Changes

- Add canonical chain requirements for reusable tip commitments, extension/prefix relations, checkpoint anchoring, fork detection, batch append/fold append, log shape well-formedness, tamper classification, and domain-separated hash inputs.
- Add explicit implementation boundaries for runtime BLAKE3 serialization alignment without expanding the cryptographic trust surface beyond named permanent crypto assumptions.
- Specify an optional skip/Merkle overlay as a separate compact-proof primitive that composes with the base linear chain rather than replacing it.

## Capabilities

### New Capabilities
- `chain.tip-commitment`: exposes a compact commitment for empty and non-empty chains.
- `chain.extension-prefix`: captures append-only extension and prefix reuse.
- `chain.checkpoint-anchor`: authenticates a prefix by index plus expected hash.
- `chain.fork-detection`: names first-divergence/tip-equality fork obligations.
- `chain.batch-append`: folds repeated appends without ad hoc loop proofs.
- `chain.log-shape`: separates contiguous index and term-policy validity from hash validity.
- `chain.tamper-classification`: classifies data, index, term, and predecessor/hash tampering.
- `chain.domain-separated-hash`: prevents cross-domain hash input reuse.
- `chain.runtime-blake3-alignment`: aligns runtime serialization/hash behavior with the spec boundary.
- `chain.compact-history-proof`: optional skip/Merkle overlay for compact membership/range proofs.

## Impact

- **Files**: Expected implementation in `src/chain.rs`, possible serialization helpers in `src/serialize_inj.rs`, and requirement sync in `docs/requirements.md` plus `openspec/specs/chain/spec.md`.
- **APIs**: Adds spec/proof helpers first; public exec wrappers only where runtime consumers need them. Existing `compute_entry_hash` remains compatible unless the implementation explicitly introduces domain-tagged wrappers.
- **Dependencies**: No new external dependency expected beyond existing `blake3`; compact-history overlay may stay spec/proof-only in the first implementation slice.
- **Testing**: Focused chain tests/proptests, Verus on `src/lib.rs`, Tracey uncovered/untested, strict OpenSpec validation, and final `./scripts/verify.sh` before archive.

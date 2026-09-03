## Phase 1: Metadata model

- [x] [serial] Define finite library kinds, crate metadata envelopes, compiler-version fields, SVH fields, stable-crate-id fields, dependency records, encoded-MIR capability flags, and usage modes. r[rustc-metadata.library-kind] r[rustc-metadata.identity-fields]
- [x] [serial] Define metadata compatibility specs for compiler version, SVH, stable crate ID, dependency name, and usage mode. r[rustc-metadata.compatibility]
- [x] [serial] Define rmeta pipelining and linkability boundary specs. r[rustc-metadata.rmeta-pipelining]

## Phase 2: Admission helpers and proofs

- [x] [serial] Implement checked exec helpers for metadata envelope admission, dependency-candidate filtering, compatibility checks, encoded-MIR availability, and linkability checks. r[rustc-metadata.admission]
- [x] [serial] Prove helper facts that admitted dependency lookup returns one compatible candidate and rejects wrong-version, wrong-SVH, wrong-crate-ID, missing-metadata, and ambiguous candidate sets. r[rustc-metadata.dependency-loading]

## Phase 3: Evidence and documentation

- [x] [serial] Add positive tests for matching rlib metadata, matching dylib metadata, metadata-only rmeta check/doc use, optional encoded MIR, and unique dependency lookup. r[rustc-metadata.evidence]
- [x] [serial] Add negative tests for wrong compiler version, wrong SVH, stable-crate-id mismatch, missing metadata, rmeta used for linking, missing encoded MIR when required, and ambiguous dependency lookup. r[rustc-metadata.evidence]
- [x] [serial] Add requirements, annotations, export wiring, and README/module-catalog text. r[rustc-metadata.docs]

## Phase 4: Validation and archive

- [x] [serial] Run focused Cargo tests and Verus verification for metadata identity helpers and capture evidence. r[rustc-metadata.final-validation]
- [x] [serial] Run Tracey uncovered/untested checks and Cairn validation/gates before archive. r[rustc-metadata.final-validation]

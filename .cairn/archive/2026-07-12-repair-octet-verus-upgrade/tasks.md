## Local proof repair

- [x] [serial] Establish the pre-change current-pass and candidate-failure baseline r[trellis.upstream_verus_corpus.compatibility_repairs]
- [x] [serial] Add a trust-free sequence-context proof that preserves the exact serialization postcondition r[trellis.upstream_verus_corpus.compatibility_repairs]
- [x] [serial] Verify the complete Trellis crate with current and candidate Verus r[trellis.upstream_verus_corpus.compatibility_repairs]

## Required-native compatibility

- [x] [parallel] Repair or precisely bound IronKV's removed collection APIs without transferring its proof claim r[trellis.upstream_verus_corpus.compatibility_repairs]
- [x] [parallel] Repair or precisely bound the allocator pointer API with its exact candidate Rust/libc closure r[trellis.upstream_verus_corpus.compatibility_repairs]
- [x] [parallel] Minimize and repair or upstream-bound the node-replication verifier internal error r[trellis.upstream_verus_corpus.compatibility_repairs]

## Evidence and decision

- [x] [serial] Update the typed observed-result matrix and add missing-patch and forced-promotion negative fixtures r[trellis.upstream_verus_corpus.compatibility_repairs]
- [x] [serial] Generate and freshness-check a canonical BLAKE3 receipt over sources, patches, tools, transcripts, assumptions, and decision r[trellis.upstream_verus_corpus.compatibility_repairs]
- [x] [serial] Bind the exact published Octet revision, then change the verifier pin and compatibility status only while every required proof passes r[trellis.upstream_verus_corpus.compatibility_repairs]
  - Octet revision `374bd16b26cee2af34211a29bfa531c016811f51` is published, locked from GitHub, and consumed by the default proof check.
- [x] [serial] Run focused proofs, corpus, formatter, flake, and Cairn checks r[trellis.upstream_verus_corpus.compatibility_repairs]
  - Focused proof/corpus/formatter/freshness checks, compatible Cairn gates, and the full `x86_64-linux` flake check pass.
- [x] [serial] Sync and archive after the durable provider revision and shipped pin are revalidated r[trellis.upstream_verus_corpus.compatibility_repairs]

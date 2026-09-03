## Phase 1: Mono-item collection model

- [x] [serial] Define finite mono-item identities for functions, statics, and shims with declared concrete instance metadata. r[mono.item-envelope]
- [x] [serial] Define root sets, item-reference edges, reachability closure, and collection completeness specs. r[mono.collection]
- [x] [serial] Define canonical deduplication and duplicate-occurrence rejection or normalization specs. r[mono.dedup]

## Phase 2: CGU partitioning

- [x] [serial] Define stable/volatile item classification metadata and codegen-unit envelopes. r[mono.cgu-classification]
- [x] [serial] Implement checked exec helpers for mono collection admission and CGU partition admission. r[mono.admission]
- [x] [serial] Prove helper facts that admitted partitions are disjoint and complete over the collected mono-item set. r[mono.cgu-partition]

## Phase 3: Evidence and documentation

- [x] [serial] Add positive tests for reachable generic instance collection, static inclusion, shim inclusion, duplicate collapse, and valid CGU partitioning. r[mono.evidence]
- [x] [serial] Add negative tests for missing reachable item, dangling reference edge, malformed instance metadata, duplicate canonical identity in output, partition overlap, and partition omission. r[mono.evidence]
- [x] [serial] Add requirements, annotations, export wiring, and README/module-catalog text. r[mono.docs]

## Phase 4: Validation and archive

- [x] [serial] Run focused Cargo tests and Verus verification for monomorphization helpers and capture evidence. r[mono.final-validation]
- [x] [serial] Run Tracey uncovered/untested checks and Cairn validation/gates before archive. r[mono.final-validation]

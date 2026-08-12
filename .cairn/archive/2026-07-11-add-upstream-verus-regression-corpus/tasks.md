# Tasks: Add an upstream Verus regression corpus

- [x] [serial] Add the typed upstream corpus catalog with exact revisions, source hashes, roles, limitations, and non-claims r[trellis.upstream_verus_corpus.catalog]
- [x] [parallel] Add positive and negative catalog fixtures plus checker self-tests r[trellis.upstream_verus_corpus.validation]
- [x] [serial] Add fixed-output Nix source intake and sentinel validation for every catalog entry r[trellis.upstream_verus_corpus.source_intake]
- [x] [serial] Package the Octet-aligned pinned `verusfmt` tool and generate the exact Trellis migration baseline r[trellis.upstream_verus_corpus.verusfmt_ratchet]
- [x] [parallel] Add positive and negative formatter fixtures and exact baseline drift checks r[trellis.upstream_verus_corpus.verusfmt_ratchet]
- [x] [serial] Wire catalog and formatter checks into the Trellis focused/full verification surfaces r[trellis.upstream_verus_corpus.integration]
- [x] [serial] Document upstream references, execution boundaries, and promotion requirements r[trellis.upstream_verus_corpus.boundary]
- [x] [serial] Run corpus, formatter, Verus, existing checker, and Cairn validation checks r[trellis.upstream_verus_corpus.final_validation]

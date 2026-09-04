# Guarded Octet migration

The pinned Octet revision is `374bd16b26cee2af34211a29bfa531c016811f51`.

The current baseline has 3,867 accepted findings. Its identity is `b3:d76b60214667b32befa24af6eb6599870b50719c4dc004f9549a1acd7f7d926c`.

The feature tree produced 3,866 current accepted findings, one new finding, and one stale finding.

The only migration is:

- stale `b3:370738e60362a2d8`
- new `b3:69dd8f7e6f7922b2`
- file `src/lib.rs`
- lint `excessive_file_length`

The new baseline identity is `b3:220aef5fb57694a2f39ef7959151a887215140c7809547a7db2184c6aa586720`.

`migration-guard.rs` requires the exact replacement. It also requires equal schemas, metadata, accepted counts, file paths, lints, and columns.

The guard rejects findings under `src/bounded_modular_tree_fold/` and `src/u24/`. Thus, the new feature source has zero accepted findings.

The migration does not add a warning budget, a suppression, or a finding baseline for new source.

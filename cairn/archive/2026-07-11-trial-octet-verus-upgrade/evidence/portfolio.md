# Verus upgrade portfolio result

## Completion contract

The trial may promote only when the current baseline passes, the complete Trellis candidate run passes, and every `required-native` corpus probe passes under the exact Octet package. Source intake, formatting, diagnostic microcases, VerusBelt's Rocq development, and partial verifier success are excluded completion evidence. Allowed terminal outcomes were `promote` and `blocked`.

## Registry

| Family | Concrete artifact | Checked evidence | State |
|---|---|---|---|
| complete-trellis | `src/lib.rs` | Baseline `10710/0`; candidate `10709/1`, failing `src/nix_drv_serialize.rs:761` | blocked |
| upstream-native/ironkv | `ironsht/src/lib.rs` | Removed `vstd::imap` and `vstd::iset_lib` APIs | blocked |
| upstream-native/allocator | `verus-mimalloc/lib.rs` plus libc `0.2.147` built by candidate Rust | Removed `ptr_mut_ref` API | blocked |
| upstream-native/node-replication | `verified-node-replication/src/lib.rs -V new-mut-ref` | `var_local_id failed: (800, 132)` internal error | blocked |
| artifact-microcase | SOSP linked-list and doubly-linked-list Verus files | Missing decreases clause and removed `vstd::ptr`; diagnostic-only | audit |
| semantic-reference | VerusBelt README, Makefile, and Rocq sources | Rocq 8.20 development tied to modeled snapshot; candidate Verus not executed | audit |
| source-only | source hashes, sentinels, and formatter results | Cannot establish verifier compatibility | falsified |

## Adversarial audit

The Trellis candidate failure reproduced with both package-default invocation and the baseline's explicit extern/library/edition argv, so it is not accepted as a wrapper-only artifact. The allocator probe first builds its exact pinned libc dependency with the candidate Rust toolchain, preventing a missing dependency from masking the observed Verus API failure. IronKV is verified at its native host proof root without allowing unrelated .NET build products to substitute. Node replication uses its required feature flag. The SOSP and VerusBelt rows are explicitly excluded from native-pass counts.

No source was patched, no assumption was added, no postcondition was weakened, and no expected failure was counted as proof success.

## Budget and terminal reason

Used one current baseline, repeated candidate Trellis runs with two argv shapes, one bounded native probe for IronKV, allocator, and node replication, two SOSP microcases, one VerusBelt authority audit, positive and negative matrix fixtures, and one deterministic Nix receipt rerun. Full benchmark reproduction and arbitrary upstream migration patches were outside budget.

Terminal result: **blocked**. The current Trellis verifier and Octet `local-pin` row remain unchanged.

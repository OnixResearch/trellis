## Validation snapshot — 2026-04-13

### Trusted-site scan

```text
$ rg -n '^[[:space:]]*#\[verifier::external_body\]' src | wc -l
15

$ rg -n '^[[:space:]]*#\[verifier::external_body\]' src/shamir_*.rs | wc -l
0

$ rg -n '^[[:space:]]*#\[verifier::external_body\]' src | sort
src/chain.rs:207:#[verifier::external_body]
src/chain.rs:42:#[verifier::external_body]
src/chain.rs:70:#[verifier::external_body]
src/log_invariant_inductive.rs:174:#[verifier::external_body]
src/log_invariant_inductive.rs:223:#[verifier::external_body]
src/log_invariant_inductive.rs:275:#[verifier::external_body]
src/nix_store_path.rs:27:#[verifier::external_body]
src/nix_store_path.rs:33:#[verifier::external_body]
src/nix_version.rs:207:#[verifier::external_body]
src/nix_version.rs:267:#[verifier::external_body]
src/nix_version.rs:300:#[verifier::external_body]
src/nix_version.rs:365:#[verifier::external_body]
src/snapshot_integrity.rs:43:#[verifier::external_body]
src/snapshot_integrity.rs:54:#[verifier::external_body]
src/snapshot_integrity.rs:63:#[verifier::external_body]
```

### Tracey

```text
$ tracey query uncovered
verified-logic/rust: 0 uncovered out of 1381 rules

$ tracey query untested
verified-logic/rust: 0 untested (impl but no verify) out of 1381 rules
```

### Full validation commands

```text
$ cargo test
3465 passed; 0 failed

$ cargo clippy --all-targets -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s) successfully

$ ./scripts/verus.sh src/lib.rs
6348 verified, 0 errors

$ ./scripts/verify.sh
=== ALL CHECKS PASSED ===
```

### Notes

- Shamir modules contribute zero trusted sites after this change.
- Remaining trusted boundaries are the 15 permanent sites in `chain`, `snapshot_integrity`, `nix_version`, `nix_store_path`, and `log_invariant_inductive`.

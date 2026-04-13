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

### Archive / active-change status

```text
$ if [ -d openspec/changes/shamir-trust-reduction ]; then echo active_change=present; else echo active_change=absent; fi
active_change=absent
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
...
test result: ok. 3465 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.07s

$ cargo clippy --all-targets -- -D warnings
    Checking verified-logic v0.1.0 (/home/brittonr/git/verified-logic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 13.96s

$ ./scripts/verus.sh src/lib.rs
verification results:: 6348 verified, 0 errors
warning: 6 warnings emitted

$ ./scripts/verify.sh
...
verification results:: 6348 verified, 0 errors
warning: 6 warnings emitted

✓ verus proofs pass
...
verified-logic/rust: 0 uncovered out of 1381 rules
✓ no uncovered requirements

verified-logic/rust: 0 untested (impl but no verify) out of 1381 rules
✓ no untested requirements

=== ALL CHECKS PASSED ===
```

### Notes

- Shamir modules contribute zero trusted sites after this change.
- Remaining trusted boundaries are the 15 permanent sites in `chain`, `snapshot_integrity`, `nix_version`, `nix_store_path`, and `log_invariant_inductive`.
- Follow-up review fix: `src/shamir_lagrange.rs:basis_other_from` now makes the `from <= j < xs.len()` access argument explicit before touching `xs[from]`.

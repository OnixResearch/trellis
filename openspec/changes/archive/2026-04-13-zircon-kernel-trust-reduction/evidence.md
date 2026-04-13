## Verification rerun summary

Fresh reruns after internalizing the five Zircon modules are archived under
`openspec/changes/zircon-kernel-trust-reduction/evidence/`.

Key results:

- `cargo test` -> `3402 passed`
- `cargo clippy --all-targets -- -D warnings` -> clean
- `./scripts/verus.sh src/lib.rs` -> `6112 verified, 0 errors`
- `./scripts/verify.sh` -> `=== ALL CHECKS PASSED ===`
- `tracey query uncovered` -> `0 uncovered out of 1345 rules`
- `tracey query untested` -> `0 untested out of 1345 rules`
- `./scripts/check-ensures.sh` -> all verified public fns still carry ensures
- `rg '#[verifier::external_body]' src` -> `53` sites across `11` files

Relevant transcripts:

- `evidence/cargo-test.txt`
- `evidence/clippy.txt`
- `evidence/verus.txt`
- `evidence/verify.txt`
- `evidence/tracey-uncovered.txt`
- `evidence/tracey-untested.txt`
- `evidence/check-ensures.txt`
- `evidence/external-body-count.txt`

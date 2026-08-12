# Validation summary

Fresh reruns captured during final remediation:

- `cargo test` → `3545 passed; 0 failed`
- `cargo clippy --all-targets -- -D warnings` → success
- `./scripts/verus.sh src/lib.rs` → `6572 verified, 0 errors`
- `./scripts/check-ensures.sh` → `Checked 1341 pub fn in verus! blocks, 0 exempted`
- `tracey query uncovered` → `0 uncovered out of 1421 rules`
- `tracey query untested` → `0 untested out of 1421 rules`

Raw excerpts live in `validation-transcript.txt`.

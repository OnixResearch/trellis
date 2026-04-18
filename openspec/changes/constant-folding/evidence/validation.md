# Constant Folding Validation

Date: 2026-04-18

## Scope check

Command:

```bash
jj diff --name-only
```

Output:

```text
.agent/napkin.md
.agent/review-metrics.jsonl
.pi/prompt-history.jsonl
AGENTS.md
docs/requirements.md
openspec/changes/constant-folding/design.md
openspec/changes/constant-folding/evidence/validation.md
openspec/changes/constant-folding/proposal.md
openspec/changes/constant-folding/tasks.md
src/constant_folding.rs
src/lib.rs
```

Command:

```bash
git diff --cached --name-only
```

Output:

```text
openspec/changes/constant-folding/tasks.md
```

This confirms the earlier stray Git-index entries for `openspec/specs/zircon-kernel/spec.md` and `src/zircon_wait.rs` are no longer part of the current constant-folding work.

## cargo test --quiet

Command:

```bash
export CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic
cargo test --quiet
```

Output excerpt:

```text
running 3949 tests
...
test result: ok. 3949 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 12.16s

running 13 tests
.............
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 116.91s

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## ./scripts/verus.sh src/lib.rs

Command:

```bash
export CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic
./scripts/verus.sh src/lib.rs
```

Output excerpt:

```text
verification results:: 7878 verified, 0 errors
warning: 18 warnings emitted
```

## cargo clippy --all-targets -- -D warnings

Command:

```bash
export CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic
cargo clippy --all-targets -- -D warnings
```

Output:

```text
    Checking verified-logic v0.1.0 (/home/brittonr/git/verified-logic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 10.79s
```

## ./scripts/verify.sh

Command:

```bash
export CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic
./scripts/verify.sh
```

Output excerpt:

```text
=== Coverage: r[impl] on every pub fn ===

Checked 1561 pub fn declarations, 2 exempted
✓ all pub fns have r[impl] annotations

=== Coverage: ensures on every verified pub fn ===

Checked 1544 pub fn in verus! blocks, 0 exempted
✓ all verified pub fns have ensures clauses

=== Clippy ===
...
✓ clippy clean

=== Tests ===
...
running 3949 tests
...
test result: ok. 3949 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

=== Verus ===
...
verification results:: 7878 verified, 0 errors
warning: 18 warnings emitted

✓ verus proofs pass

=== Tracey coverage ===
...
verified-logic/rust: 0 uncovered out of 1580 rules
...
✓ no uncovered requirements

verified-logic/rust: 0 untested (impl but no verify) out of 1580 rules
...
✓ no untested requirements

=== ALL CHECKS PASSED ===
```

## Direct Tracey spot-check

Commands:

```bash
tracey query uncovered
tracey query untested
```

Output:

```text
verified-logic/rust: 0 uncovered out of 1580 rules
verified-logic/rust: 0 untested (impl but no verify) out of 1580 rules
```

## Notes

- This evidence covers the current implemented slice through the first landed Phase 4 engine work.
- The final Phase 6 rerun task remains open until the remaining Phase 4/5 proof and test work lands.

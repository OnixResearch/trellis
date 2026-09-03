# Rustc Metadata Identity Baseline

Source commit: `470a0db819c1fd72527c34484fb6ba6688983cb9`

## Commands

```text
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic nix develop --builders '' -c cargo test --quiet
NIX_CONFIG='builders =' ./scripts/verus.sh src/lib.rs
tracey query uncovered
tracey query untested
```

All commands returned status `0`. Cargo and Verus passed.

The first uncovered query printed the known Tracey daemon-start cancellation. Its direct rerun reported `0 uncovered out of 2079 rules`. The untested query reported `0 untested out of 2079 rules`.

These results describe the pre-change tree only.

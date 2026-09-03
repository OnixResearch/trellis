# HIR Attribute Parsing Baseline

Source commit: `36f372f7fe5a87491dd61d62a40f22228975ddea`

## Commands

```text
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic nix develop --builders '' -c cargo test --quiet
NIX_CONFIG='builders =' ./scripts/verus.sh src/lib.rs
tracey query uncovered
tracey query untested
```

All commands returned status `0`. Cargo and Verus passed.

The first uncovered query printed the known Tracey daemon-start cancellation. The direct rerun reported `0 uncovered out of 2072 rules`. The untested query reported `0 untested out of 2072 rules`.

These results describe the pre-change tree only.

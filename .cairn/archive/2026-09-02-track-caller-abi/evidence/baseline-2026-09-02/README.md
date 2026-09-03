# Track Caller ABI Baseline

Source commit: `644603b`

## Commands

```text
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic nix develop --builders '' -c cargo test --quiet
nix develop --builders '' -c ./scripts/verus.sh src/lib.rs
nix develop --builders '' -c tracey query uncovered
nix develop --builders '' -c tracey query untested
```

All four commands returned status `0`. Cargo tests and full-library Verus passed.

The first uncovered query printed the known Tracey daemon-start `Error: Cancelled` message despite status `0`. The immediate direct rerun reported `0 uncovered out of 2062 rules`. The untested query reported `0 untested out of 2062 rules`.

These results describe the pre-change tree only. They do not validate the new track-caller module.

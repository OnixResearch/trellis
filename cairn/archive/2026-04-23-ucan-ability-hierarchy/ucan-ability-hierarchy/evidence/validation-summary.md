# Validation Summary

## Commands

### Focused module baseline
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_ability`
  - Result before the final sync pass: pass (`5 passed; 0 failed`)

### Fresh focused runtime validation
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_ability`
  - Result after sync: pass (`5 passed; 0 failed`)
  - Transcript: `cargo-test-ucan-ability.txt`

### Fresh OpenSpec gate reruns
- `openspec_gate stage=proposal change=ucan-ability-hierarchy` → PASS
- `openspec_gate stage=design change=ucan-ability-hierarchy` → PASS
- `openspec_gate stage=tasks change=ucan-ability-hierarchy` → PASS

### Full-lib Verus verification
- `./scripts/verus.sh src/lib.rs`
  - Result: pass (`8113 verified, 0 errors`)
  - Transcript: `verus-lib.txt`

## Scope notes
- Artifact sync now matches the landed module exactly: a runtime `Ability`
  carrier, the public matcher `ability_subsumes(...)`, and proof helpers
  for top-ability, namespace isolation, single-level glob, and
  containment monotonicity.
- Parsing helpers, recursive `**`, and broader authorization
  decomposition claims are now explicitly out of scope for this archive.
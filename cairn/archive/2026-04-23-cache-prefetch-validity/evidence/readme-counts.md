# README Count Derivation

- **Modules**: `370` from `rg '^pub mod ' src/lib.rs | wc -l`
- **Requirements**: `1716` from `rg '^r\[' docs/requirements.md | wc -l`
- **Tests**: `4116` unit/property tests plus `13` doc tests from full `cargo test --quiet`
- **Verus proofs**: `8332` from `./scripts/verus.sh src/lib.rs`
- **Trusted boundaries**: unchanged at `19`

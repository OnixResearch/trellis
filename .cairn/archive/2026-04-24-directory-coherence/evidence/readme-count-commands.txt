$ rg '^pub mod ' src/lib.rs | wc -l
371

$ rg '^r\[' docs/requirements.md | wc -l
1722

$ CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet
4129 passed; 13 doc tests passed

$ ./scripts/verus.sh src/lib.rs
8367 verified, 0 errors

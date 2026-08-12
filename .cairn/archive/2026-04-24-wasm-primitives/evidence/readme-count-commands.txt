$ rg '^pub mod ' src/lib.rs | wc -l
373

$ rg '^r\[' docs/requirements.md | wc -l
1731

$ CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet
4152 passed; 13 doc tests passed

$ ./scripts/verus.sh src/lib.rs
8425 verified, 0 errors

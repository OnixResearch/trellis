$ rg '^pub mod ' src/lib.rs | wc -l
372

$ rg '^r\[' docs/requirements.md | wc -l
1726

$ CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet
4139 passed; 13 doc tests passed

$ ./scripts/verus.sh src/lib.rs
8395 verified, 0 errors

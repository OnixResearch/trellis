$ rg '^r\[[^]]+\]$' docs/*.md | wc -l
1734

$ CARGO_HOME=<temporary-no-user-config> CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet
running 4157 tests
...
test result: ok. 4157 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.22s

running 13 tests
.............
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 55.50s

$ ./scripts/verus.sh src/lib.rs
verification results:: 8451 verified, 0 errors

$ rg '^pub mod ' src/lib.rs | wc -l
373

$ rg '^\*\*8451 Verus proofs.*4157 tests.*1734 requirements.*373 modules.*43 trusted/proof-gap boundaries\*\*' README.md
**8451 Verus proofs · 4157 tests · 1734 requirements · 373 modules · 43 trusted/proof-gap boundaries**

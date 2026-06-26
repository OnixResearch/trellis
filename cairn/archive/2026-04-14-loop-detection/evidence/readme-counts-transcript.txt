$ cargo test
...
test result: ok. 3545 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.69s

$ cargo clippy --all-targets -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s

$ ./scripts/verus.sh src/lib.rs
...
verification results:: 6572 verified, 0 errors
warning: 16 warnings emitted

$ tracey query uncovered
verified-logic/rust: 0 uncovered out of 1421 rules

$ tracey query untested
verified-logic/rust: 0 untested (impl but no verify) out of 1421 rules

$ rg '^pub mod ' src/lib.rs | wc -l
303

$ rg -n '#\[verifier::external_body\]' src | wc -l
15

$ rg -n 'assume_specification\[' src/ssa_construction.rs
195:assume_specification[crate::ssa_construction::runtime_place_phis](program: &RawProgram, dom: &[Vec<bool>]) -> (result: SsaProgram);
197:assume_specification[crate::ssa_construction::runtime_construct_ssa](program: &RawProgram) -> (result: SsaProgram);
199:assume_specification[crate::ssa_construction::runtime_check_dominance_of_uses_with_dom](program: &SsaProgram, dom: &[Vec<bool>]) -> (result: bool);

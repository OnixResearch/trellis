$ rg -n "^pub mod " src/lib.rs | wc -l
301

$ rg -n "^r\[" docs/requirements.md | wc -l
1429

$ rg -n "#\[verifier::external_body\]" src | wc -l
15

$ rg -n "assume_specification\[" src/ssa_construction.rs | wc -l
7

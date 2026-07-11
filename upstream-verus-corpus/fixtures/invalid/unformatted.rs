use vstd::prelude::*;
verus! {
pub proof fn corpus_identity_reflexive(value: int)
    ensures value == value,
{}
} // verus!

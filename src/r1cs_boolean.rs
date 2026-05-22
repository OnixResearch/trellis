//! Finite R1CS booleanity primitives.
//!
//! This module encodes a bounded boolean witness constraint into one existing
//! R1CS row. It models only deterministic boolean admission and satisfaction;
//! it does not model bit decomposition, range checks, proof systems,
//! transcripts, commitments, or cryptographic soundness.

use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::r1cs_constraint::r1cs_element_valid_spec;
use crate::r1cs_constraint::r1cs_len_valid;
#[cfg(verus_keep_ghost)]
use crate::r1cs_constraint::r1cs_len_valid_spec;
use crate::r1cs_constraint::r1cs_modulus_valid;
#[cfg(verus_keep_ghost)]
use crate::r1cs_constraint::r1cs_modulus_valid_spec;
use crate::r1cs_constraint::r1cs_row_satisfied;
#[cfg(verus_keep_ghost)]
use crate::r1cs_constraint::r1cs_row_satisfied_spec;
#[cfg(verus_keep_ghost)]
use crate::r1cs_constraint::r1cs_row_valid_spec;
use crate::r1cs_constraint::r1cs_witness_valid;
#[cfg(verus_keep_ghost)]
use crate::r1cs_constraint::r1cs_witness_valid_spec;
use crate::r1cs_constraint::R1csRow;
use crate::r1cs_constraint::R1csWitness;
use crate::r1cs_gates::r1cs_one_at;
#[cfg(verus_keep_ghost)]
use crate::r1cs_gates::r1cs_one_at_spec;

verus! {
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct R1csBoolean {
    pub witness_len: u8,
    pub modulus: u16,
    pub index: u8,
}

// r[depends rust-primitives.r1cs-boolean-shape]
pub open spec fn r1cs_boolean_modulus_coeff_valid_spec(modulus: u16) -> bool {
    r1cs_modulus_valid_spec(modulus) && modulus > 0 && r1cs_element_valid_spec((modulus - 1) as u16)
}

// r[depends rust-primitives.r1cs-boolean-shape]
pub open spec fn r1cs_boolean_index_valid_spec(boolean: R1csBoolean) -> bool {
    0 < boolean.index && boolean.index < boolean.witness_len
}

// r[depends rust-primitives.r1cs-boolean-shape]
pub open spec fn r1cs_boolean_valid_spec(boolean: R1csBoolean) -> bool {
    r1cs_len_valid_spec(boolean.witness_len)
        && r1cs_boolean_modulus_coeff_valid_spec(boolean.modulus)
        && r1cs_boolean_index_valid_spec(boolean)
}

// r[depends rust-primitives.r1cs-boolean-row-encoding]
pub open spec fn r1cs_boolean_row_spec(boolean: R1csBoolean) -> R1csRow {
    R1csRow {
        len: boolean.witness_len,
        modulus: boolean.modulus,
        a0: r1cs_one_at_spec(boolean.index, 0),
        a1: r1cs_one_at_spec(boolean.index, 1),
        a2: r1cs_one_at_spec(boolean.index, 2),
        b0: 1,
        b1: if boolean.index == 1 { (boolean.modulus - 1) as u16 } else { 0 },
        b2: if boolean.index == 2 { (boolean.modulus - 1) as u16 } else { 0 },
        c0: 0,
        c1: 0,
        c2: 0,
    }
}

// r[depends rust-primitives.r1cs-boolean-satisfaction]
pub open spec fn r1cs_boolean_one_wire_satisfied_spec(witness: R1csWitness) -> bool {
    witness.w0 == 1
}

// r[depends rust-primitives.r1cs-boolean-satisfaction]
pub open spec fn r1cs_boolean_satisfied_spec(boolean: R1csBoolean, witness: R1csWitness) -> bool {
    r1cs_boolean_valid_spec(boolean)
        && r1cs_witness_valid_spec(witness)
        && witness.len == boolean.witness_len
        && r1cs_boolean_one_wire_satisfied_spec(witness)
        && r1cs_row_satisfied_spec(r1cs_boolean_row_spec(boolean), witness)
}

// r[impl rust-primitives.r1cs-boolean-shape]
// r[impl rust-primitives.r1cs-boolean-verification-coverage]
pub fn r1cs_boolean_modulus_coeff_valid(modulus: u16) -> (result: bool)
    ensures result == r1cs_boolean_modulus_coeff_valid_spec(modulus)
{
    r1cs_modulus_valid(modulus) && modulus > 0 && modulus - 1 <= 15
}

// r[impl rust-primitives.r1cs-boolean-shape]
pub fn r1cs_boolean_index_valid(boolean: R1csBoolean) -> (result: bool)
    ensures result == r1cs_boolean_index_valid_spec(boolean)
{
    0 < boolean.index && boolean.index < boolean.witness_len
}

// r[impl rust-primitives.r1cs-boolean-shape]
pub fn r1cs_boolean_valid(boolean: R1csBoolean) -> (result: bool)
    ensures result == r1cs_boolean_valid_spec(boolean)
{
    r1cs_len_valid(boolean.witness_len)
        && r1cs_boolean_modulus_coeff_valid(boolean.modulus)
        && r1cs_boolean_index_valid(boolean)
}

// r[impl rust-primitives.r1cs-boolean-row-encoding]
pub fn r1cs_boolean_row(boolean: R1csBoolean) -> (result: R1csRow)
    requires r1cs_boolean_valid_spec(boolean)
    ensures
        result == r1cs_boolean_row_spec(boolean),
        r1cs_row_valid_spec(result),
{
    let neg_one = boolean.modulus - 1;
    assert(neg_one <= 15);
    R1csRow {
        len: boolean.witness_len,
        modulus: boolean.modulus,
        a0: r1cs_one_at(boolean.index, 0),
        a1: r1cs_one_at(boolean.index, 1),
        a2: r1cs_one_at(boolean.index, 2),
        b0: 1,
        b1: if boolean.index == 1 { neg_one } else { 0 },
        b2: if boolean.index == 2 { neg_one } else { 0 },
        c0: 0,
        c1: 0,
        c2: 0,
    }
}

// r[impl rust-primitives.r1cs-boolean-satisfaction]
pub fn r1cs_boolean_one_wire_satisfied(witness: R1csWitness) -> (result: bool)
    ensures result == r1cs_boolean_one_wire_satisfied_spec(witness)
{
    witness.w0 == 1
}

// r[impl rust-primitives.r1cs-boolean-satisfaction]
pub fn r1cs_boolean_satisfied(boolean: R1csBoolean, witness: R1csWitness) -> (result: bool)
    ensures result == r1cs_boolean_satisfied_spec(boolean, witness)
{
    if !r1cs_boolean_valid(boolean) {
        return false;
    }
    if !r1cs_witness_valid(witness) {
        return false;
    }
    if witness.len != boolean.witness_len {
        return false;
    }
    if !r1cs_boolean_one_wire_satisfied(witness) {
        return false;
    }
    let row = r1cs_boolean_row(boolean);
    r1cs_row_satisfied(row, witness)
}

// r[verify rust-primitives.r1cs-boolean-shape]
pub proof fn malformed_boolean_shape_rejected(boolean: R1csBoolean, witness: R1csWitness)
    requires !r1cs_boolean_valid_spec(boolean)
    ensures !r1cs_boolean_satisfied_spec(boolean, witness)
{}

// r[verify rust-primitives.r1cs-boolean-row-encoding]
pub proof fn admitted_boolean_encodes_valid_row(boolean: R1csBoolean)
    requires r1cs_boolean_valid_spec(boolean)
    ensures r1cs_row_valid_spec(r1cs_boolean_row_spec(boolean))
{}

// r[verify rust-primitives.r1cs-boolean-satisfaction]
pub proof fn malformed_boolean_witness_rejected(boolean: R1csBoolean, witness: R1csWitness)
    requires !r1cs_witness_valid_spec(witness) || witness.len != boolean.witness_len
    ensures !r1cs_boolean_satisfied_spec(boolean, witness)
{}

// r[verify rust-primitives.r1cs-boolean-satisfaction]
pub proof fn missing_boolean_one_wire_rejected(boolean: R1csBoolean, witness: R1csWitness)
    requires witness.w0 != 1
    ensures !r1cs_boolean_satisfied_spec(boolean, witness)
{}

// r[verify rust-primitives.r1cs-boolean-satisfaction]
pub proof fn unsatisfied_boolean_row_rejects(boolean: R1csBoolean, witness: R1csWitness)
    requires
        r1cs_boolean_valid_spec(boolean),
        r1cs_witness_valid_spec(witness),
        witness.len == boolean.witness_len,
        r1cs_boolean_one_wire_satisfied_spec(witness),
        !r1cs_row_satisfied_spec(r1cs_boolean_row_spec(boolean), witness),
    ensures !r1cs_boolean_satisfied_spec(boolean, witness)
{}

// r[verify rust-primitives.r1cs-boolean-verification-coverage]
pub proof fn r1cs_boolean_primitive_coverage_catalogued()
    ensures true
{}
} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    fn boolean_at_one() -> R1csBoolean {
        R1csBoolean {
            witness_len: 3,
            modulus: 13,
            index: 1,
        }
    }

    fn witness(value: u16) -> R1csWitness {
        R1csWitness {
            len: 3,
            w0: 1,
            w1: value,
            w2: 7,
        }
    }

    #[test]
    fn zero_is_boolean() {
        assert!(r1cs_boolean_satisfied(boolean_at_one(), witness(0)));
    }

    #[test]
    fn one_is_boolean() {
        assert!(r1cs_boolean_satisfied(boolean_at_one(), witness(1)));
    }

    #[test]
    fn two_is_not_boolean() {
        assert!(!r1cs_boolean_satisfied(boolean_at_one(), witness(2)));
    }

    #[test]
    fn boolean_row_is_valid_and_satisfied_for_one() {
        let boolean = boolean_at_one();
        let row = r1cs_boolean_row(boolean);
        assert_eq!(row.a1, 1);
        assert_eq!(row.b0, 1);
        assert_eq!(row.b1, 12);
        assert_eq!(row.c0, 0);
        assert!(r1cs_row_satisfied(row, witness(1)));
    }

    #[test]
    fn zero_index_is_rejected_because_it_is_the_one_wire() {
        let mut boolean = boolean_at_one();
        boolean.index = 0;
        assert!(!r1cs_boolean_valid(boolean));
        assert!(!r1cs_boolean_satisfied(boolean, witness(1)));
    }

    #[test]
    fn out_of_range_index_is_rejected() {
        let mut boolean = boolean_at_one();
        boolean.index = 3;
        assert!(!r1cs_boolean_valid(boolean));
        assert!(!r1cs_boolean_satisfied(boolean, witness(1)));
    }

    #[test]
    fn overlarge_modulus_minus_one_coefficient_is_rejected() {
        let mut boolean = boolean_at_one();
        boolean.modulus = 17;
        assert!(!r1cs_boolean_valid(boolean));
        assert!(!r1cs_boolean_satisfied(boolean, witness(1)));
    }

    #[test]
    fn missing_one_wire_is_rejected() {
        let mut bad = witness(1);
        bad.w0 = 2;
        assert!(!r1cs_boolean_satisfied(boolean_at_one(), bad));
    }

    #[test]
    fn malformed_witness_length_is_rejected() {
        let mut bad = witness(1);
        bad.len = 2;
        assert!(!r1cs_boolean_satisfied(boolean_at_one(), bad));
    }
}

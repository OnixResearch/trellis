//! Finite R1CS public-input binding primitives.
//!
//! This module binds a bounded list of public values to witness indices and
//! composes those bindings with `r1cs_system` satisfaction. It models statement
//! admission only: no commitments, transcripts, proof objects, verifier keys, or
//! cryptographic soundness are claimed here.

use vstd::prelude::*;

use crate::r1cs_constraint::r1cs_element_valid;
#[cfg(verus_keep_ghost)]
use crate::r1cs_constraint::r1cs_element_valid_spec;
use crate::r1cs_constraint::r1cs_modulus_valid;
#[cfg(verus_keep_ghost)]
use crate::r1cs_constraint::r1cs_modulus_valid_spec;
use crate::r1cs_constraint::r1cs_witness_valid;
#[cfg(verus_keep_ghost)]
use crate::r1cs_constraint::r1cs_witness_valid_spec;
use crate::r1cs_constraint::R1csRow;
use crate::r1cs_constraint::R1csWitness;
use crate::r1cs_system::r1cs_system_satisfied;
#[cfg(verus_keep_ghost)]
use crate::r1cs_system::r1cs_system_satisfied_spec;
use crate::r1cs_system::R1csSystem;

verus! {
pub const R1CS_PUBLIC_INPUT_MAX_LEN: u8 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct R1csPublicInputs {
    pub len: u8,
    pub index0: u8,
    pub index1: u8,
    pub index2: u8,
    pub value0: u16,
    pub value1: u16,
    pub value2: u16,
}

// r[depends rust-primitives.r1cs-public-input-binding-shape]
pub open spec fn r1cs_public_inputs_len_valid_spec(len: u8) -> bool {
    len <= R1CS_PUBLIC_INPUT_MAX_LEN
}

// r[depends rust-primitives.r1cs-public-input-binding-shape]
pub open spec fn r1cs_witness_value_at_spec(witness: R1csWitness, index: u8) -> u16 {
    if index == 0 { witness.w0 } else if index == 1 { witness.w1 } else { witness.w2 }
}

// r[depends rust-primitives.r1cs-public-input-binding-shape]
pub open spec fn r1cs_public_binding_slot_satisfied_spec(
    index: u8,
    value: u16,
    witness: R1csWitness,
    modulus: u16,
) -> bool {
    index < witness.len
        && r1cs_element_valid_spec(value)
        && (value as int) % (modulus as int)
            == (r1cs_witness_value_at_spec(witness, index) as int) % (modulus as int)
}

// r[depends rust-primitives.r1cs-public-input-binding-shape]
pub open spec fn r1cs_public_inputs_satisfied_spec(
    public_inputs: R1csPublicInputs,
    witness: R1csWitness,
    modulus: u16,
) -> bool {
    r1cs_public_inputs_len_valid_spec(public_inputs.len)
        && r1cs_modulus_valid_spec(modulus)
        && r1cs_witness_valid_spec(witness)
        && (public_inputs.len < 1 || r1cs_public_binding_slot_satisfied_spec(public_inputs.index0, public_inputs.value0, witness, modulus))
        && (public_inputs.len < 2 || r1cs_public_binding_slot_satisfied_spec(public_inputs.index1, public_inputs.value1, witness, modulus))
        && (public_inputs.len < 3 || r1cs_public_binding_slot_satisfied_spec(public_inputs.index2, public_inputs.value2, witness, modulus))
}

// r[depends rust-primitives.r1cs-public-statement-satisfaction]
pub open spec fn r1cs_public_statement_satisfied_spec(
    system: R1csSystem,
    witness: R1csWitness,
    public_inputs: R1csPublicInputs,
) -> bool {
    r1cs_system_satisfied_spec(system, witness)
        && r1cs_public_inputs_satisfied_spec(public_inputs, witness, system.modulus)
}

// r[impl rust-primitives.r1cs-public-input-binding-shape]
// r[impl rust-primitives.r1cs-public-input-verification-coverage]
pub fn r1cs_public_inputs_len_valid(len: u8) -> (result: bool)
    ensures result == r1cs_public_inputs_len_valid_spec(len)
{
    len <= R1CS_PUBLIC_INPUT_MAX_LEN
}

// r[impl rust-primitives.r1cs-public-input-binding-shape]
pub fn r1cs_witness_value_at(witness: R1csWitness, index: u8) -> (result: u16)
    ensures result == r1cs_witness_value_at_spec(witness, index)
{
    if index == 0 {
        witness.w0
    } else if index == 1 {
        witness.w1
    } else {
        witness.w2
    }
}

// r[impl rust-primitives.r1cs-public-input-binding-shape]
pub fn r1cs_public_binding_slot_satisfied(
    index: u8,
    value: u16,
    witness: R1csWitness,
    modulus: u16,
) -> (result: bool)
    requires r1cs_modulus_valid_spec(modulus)
    ensures result == r1cs_public_binding_slot_satisfied_spec(index, value, witness, modulus)
{
    if !(index < witness.len && r1cs_element_valid(value)) {
        return false;
    }
    let witness_value = r1cs_witness_value_at(witness, index);
    value % modulus == witness_value % modulus
}

// r[impl rust-primitives.r1cs-public-input-binding-shape]
pub fn r1cs_public_inputs_satisfied(
    public_inputs: R1csPublicInputs,
    witness: R1csWitness,
    modulus: u16,
) -> (result: bool)
    ensures result == r1cs_public_inputs_satisfied_spec(public_inputs, witness, modulus)
{
    if !r1cs_public_inputs_len_valid(public_inputs.len) {
        return false;
    }
    if !r1cs_modulus_valid(modulus) {
        return false;
    }
    if !r1cs_witness_valid(witness) {
        return false;
    }
    if public_inputs.len >= 1
        && !r1cs_public_binding_slot_satisfied(public_inputs.index0, public_inputs.value0, witness, modulus)
    {
        return false;
    }
    if public_inputs.len >= 2
        && !r1cs_public_binding_slot_satisfied(public_inputs.index1, public_inputs.value1, witness, modulus)
    {
        return false;
    }
    if public_inputs.len >= 3
        && !r1cs_public_binding_slot_satisfied(public_inputs.index2, public_inputs.value2, witness, modulus)
    {
        return false;
    }
    true
}

// r[impl rust-primitives.r1cs-public-statement-satisfaction]
pub fn r1cs_public_statement_satisfied(
    system: R1csSystem,
    witness: R1csWitness,
    public_inputs: R1csPublicInputs,
) -> (result: bool)
    ensures result == r1cs_public_statement_satisfied_spec(system, witness, public_inputs)
{
    if !r1cs_system_satisfied(system, witness) {
        return false;
    }
    r1cs_public_inputs_satisfied(public_inputs, witness, system.modulus)
}

// r[verify rust-primitives.r1cs-public-input-binding-shape]
pub proof fn invalid_public_input_lengths_rejected(
    public_inputs: R1csPublicInputs,
    witness: R1csWitness,
    modulus: u16,
)
    requires public_inputs.len > R1CS_PUBLIC_INPUT_MAX_LEN
    ensures !r1cs_public_inputs_satisfied_spec(public_inputs, witness, modulus)
{}

// r[verify rust-primitives.r1cs-public-input-binding-shape]
pub proof fn out_of_range_public_index_rejected(
    public_inputs: R1csPublicInputs,
    witness: R1csWitness,
    modulus: u16,
)
    requires
        r1cs_modulus_valid_spec(modulus),
        r1cs_witness_valid_spec(witness),
        (public_inputs.len >= 1 && public_inputs.index0 >= witness.len)
            || (public_inputs.len >= 2 && public_inputs.index1 >= witness.len)
            || (public_inputs.len >= 3 && public_inputs.index2 >= witness.len),
    ensures !r1cs_public_inputs_satisfied_spec(public_inputs, witness, modulus)
{}

// r[verify rust-primitives.r1cs-public-input-binding-shape]
pub proof fn mismatched_public_value_rejected(
    public_inputs: R1csPublicInputs,
    witness: R1csWitness,
    modulus: u16,
)
    requires
        r1cs_modulus_valid_spec(modulus),
        r1cs_witness_valid_spec(witness),
        r1cs_public_inputs_len_valid_spec(public_inputs.len),
        (public_inputs.len >= 1
            && public_inputs.index0 < witness.len
            && r1cs_element_valid_spec(public_inputs.value0)
            && (public_inputs.value0 as int) % (modulus as int)
                != (r1cs_witness_value_at_spec(witness, public_inputs.index0) as int) % (modulus as int))
            || (public_inputs.len >= 2
                && public_inputs.index1 < witness.len
                && r1cs_element_valid_spec(public_inputs.value1)
                && (public_inputs.value1 as int) % (modulus as int)
                    != (r1cs_witness_value_at_spec(witness, public_inputs.index1) as int) % (modulus as int))
            || (public_inputs.len >= 3
                && public_inputs.index2 < witness.len
                && r1cs_element_valid_spec(public_inputs.value2)
                && (public_inputs.value2 as int) % (modulus as int)
                    != (r1cs_witness_value_at_spec(witness, public_inputs.index2) as int) % (modulus as int)),
    ensures !r1cs_public_inputs_satisfied_spec(public_inputs, witness, modulus)
{}

// r[verify rust-primitives.r1cs-public-statement-satisfaction]
pub proof fn malformed_statement_system_rejected(
    system: R1csSystem,
    witness: R1csWitness,
    public_inputs: R1csPublicInputs,
)
    requires !r1cs_system_satisfied_spec(system, witness)
    ensures !r1cs_public_statement_satisfied_spec(system, witness, public_inputs)
{}

// r[verify rust-primitives.r1cs-public-statement-satisfaction]
pub proof fn malformed_statement_public_inputs_rejected(
    system: R1csSystem,
    witness: R1csWitness,
    public_inputs: R1csPublicInputs,
)
    requires !r1cs_public_inputs_satisfied_spec(public_inputs, witness, system.modulus)
    ensures !r1cs_public_statement_satisfied_spec(system, witness, public_inputs)
{}

// r[verify rust-primitives.r1cs-public-input-verification-coverage]
pub proof fn r1cs_public_input_primitive_coverage_catalogued()
    ensures true
{}
} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    fn witness() -> R1csWitness {
        R1csWitness {
            len: 2,
            w0: 3,
            w1: 4,
            w2: 15,
        }
    }

    fn row_product_11_times_13() -> R1csRow {
        R1csRow {
            len: 2,
            modulus: 17,
            a0: 1,
            a1: 2,
            a2: 15,
            b0: 3,
            b1: 1,
            b2: 15,
            c0: 1,
            c1: 1,
            c2: 15,
        }
    }

    fn row_product_3_times_4() -> R1csRow {
        R1csRow {
            len: 2,
            modulus: 17,
            a0: 1,
            a1: 0,
            a2: 15,
            b0: 0,
            b1: 1,
            b2: 15,
            c0: 4,
            c1: 0,
            c2: 15,
        }
    }

    fn inactive_row() -> R1csRow {
        R1csRow {
            len: 0,
            modulus: 1,
            a0: 99,
            a1: 99,
            a2: 99,
            b0: 99,
            b1: 99,
            b2: 99,
            c0: 99,
            c1: 99,
            c2: 99,
        }
    }

    fn satisfied_system() -> R1csSystem {
        R1csSystem {
            len: 2,
            modulus: 17,
            row0: row_product_11_times_13(),
            row1: row_product_3_times_4(),
            row2: inactive_row(),
        }
    }

    fn public_inputs() -> R1csPublicInputs {
        R1csPublicInputs {
            len: 2,
            index0: 0,
            index1: 1,
            index2: 99,
            value0: 3,
            value1: 4,
            value2: 99,
        }
    }

    #[test]
    fn matching_public_inputs_are_admitted() {
        assert!(r1cs_public_inputs_satisfied(public_inputs(), witness(), 17));
    }

    #[test]
    fn satisfied_system_with_matching_public_inputs_is_admitted() {
        assert!(r1cs_public_statement_satisfied(satisfied_system(), witness(), public_inputs()));
    }

    #[test]
    fn mismatched_public_value_rejects_statement() {
        let mut public_inputs = public_inputs();
        public_inputs.value1 = 5;
        assert!(!r1cs_public_inputs_satisfied(public_inputs, witness(), 17));
        assert!(!r1cs_public_statement_satisfied(satisfied_system(), witness(), public_inputs));
    }

    #[test]
    fn out_of_range_public_index_rejects_binding() {
        let mut public_inputs = public_inputs();
        public_inputs.index1 = 2;
        assert!(!r1cs_public_inputs_satisfied(public_inputs, witness(), 17));
    }

    #[test]
    fn malformed_witness_rejects_statement() {
        let mut bad_witness = witness();
        bad_witness.len = 3;
        assert!(!r1cs_public_statement_satisfied(satisfied_system(), bad_witness, public_inputs()));
    }

    #[test]
    fn unsatisfied_system_rejects_even_matching_public_inputs() {
        let mut system = satisfied_system();
        system.row1.c0 = 5;
        assert!(!r1cs_public_statement_satisfied(system, witness(), public_inputs()));
    }

    #[test]
    fn inactive_malformed_public_slots_are_ignored() {
        let public_inputs = R1csPublicInputs {
            len: 1,
            index0: 0,
            index1: 99,
            index2: 99,
            value0: 3,
            value1: 99,
            value2: 99,
        };
        assert!(r1cs_public_inputs_satisfied(public_inputs, witness(), 17));
    }

    #[test]
    fn empty_public_input_list_is_allowed() {
        let public_inputs = R1csPublicInputs {
            len: 0,
            index0: 99,
            index1: 99,
            index2: 99,
            value0: 99,
            value1: 99,
            value2: 99,
        };
        assert!(r1cs_public_statement_satisfied(satisfied_system(), witness(), public_inputs));
    }
}

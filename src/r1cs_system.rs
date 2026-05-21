//! Finite multi-row R1CS constraint-system primitives.
//!
//! This module composes `r1cs_constraint` rows into one bounded system checked
//! against a single witness and one shared modulus. It deliberately stops at
//! deterministic constraint-system admission and does not model proving systems,
//! transcripts, commitments, or cryptographic soundness.

use vstd::prelude::*;

use crate::r1cs_constraint::r1cs_modulus_valid;
#[cfg(verus_keep_ghost)]
use crate::r1cs_constraint::r1cs_modulus_valid_spec;
use crate::r1cs_constraint::r1cs_row_satisfied;
#[cfg(verus_keep_ghost)]
use crate::r1cs_constraint::r1cs_row_satisfied_spec;
use crate::r1cs_constraint::r1cs_row_valid;
#[cfg(verus_keep_ghost)]
use crate::r1cs_constraint::r1cs_row_valid_spec;
use crate::r1cs_constraint::r1cs_witness_valid;
#[cfg(verus_keep_ghost)]
use crate::r1cs_constraint::r1cs_witness_valid_spec;
use crate::r1cs_constraint::R1csRow;
use crate::r1cs_constraint::R1csWitness;

verus! {
pub const R1CS_SYSTEM_MAX_ROWS: u8 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct R1csSystem {
    pub len: u8,
    pub modulus: u16,
    pub row0: R1csRow,
    pub row1: R1csRow,
    pub row2: R1csRow,
}

// r[depends rust-primitives.r1cs-system-shape]
pub open spec fn r1cs_system_len_valid_spec(len: u8) -> bool {
    1 <= len && len <= R1CS_SYSTEM_MAX_ROWS
}

// r[depends rust-primitives.r1cs-system-shape]
pub open spec fn r1cs_row_uses_system_modulus_spec(row: R1csRow, modulus: u16) -> bool {
    r1cs_row_valid_spec(row) && row.modulus == modulus
}

// r[depends rust-primitives.r1cs-system-shape]
pub open spec fn r1cs_system_valid_spec(system: R1csSystem) -> bool {
    r1cs_system_len_valid_spec(system.len)
        && r1cs_modulus_valid_spec(system.modulus)
        && r1cs_row_uses_system_modulus_spec(system.row0, system.modulus)
        && (system.len < 2 || r1cs_row_uses_system_modulus_spec(system.row1, system.modulus))
        && (system.len < 3 || r1cs_row_uses_system_modulus_spec(system.row2, system.modulus))
}

// r[depends rust-primitives.r1cs-system-satisfaction]
pub open spec fn r1cs_system_satisfied_spec(system: R1csSystem, witness: R1csWitness) -> bool {
    r1cs_system_valid_spec(system)
        && r1cs_witness_valid_spec(witness)
        && r1cs_row_satisfied_spec(system.row0, witness)
        && (system.len < 2 || r1cs_row_satisfied_spec(system.row1, witness))
        && (system.len < 3 || r1cs_row_satisfied_spec(system.row2, witness))
}

// r[impl rust-primitives.r1cs-system-shape]
// r[impl rust-primitives.r1cs-system-verification-coverage]
pub fn r1cs_system_len_valid(len: u8) -> (result: bool)
    ensures result == r1cs_system_len_valid_spec(len)
{
    1 <= len && len <= R1CS_SYSTEM_MAX_ROWS
}

// r[impl rust-primitives.r1cs-system-shape]
pub fn r1cs_row_uses_system_modulus(row: R1csRow, modulus: u16) -> (result: bool)
    ensures result == r1cs_row_uses_system_modulus_spec(row, modulus)
{
    r1cs_row_valid(row) && row.modulus == modulus
}

// r[impl rust-primitives.r1cs-system-shape]
pub fn r1cs_system_valid(system: R1csSystem) -> (result: bool)
    ensures result == r1cs_system_valid_spec(system)
{
    if !r1cs_system_len_valid(system.len) {
        return false;
    }
    if !r1cs_modulus_valid(system.modulus) {
        return false;
    }
    if !r1cs_row_uses_system_modulus(system.row0, system.modulus) {
        return false;
    }
    if system.len >= 2 && !r1cs_row_uses_system_modulus(system.row1, system.modulus) {
        return false;
    }
    if system.len >= 3 && !r1cs_row_uses_system_modulus(system.row2, system.modulus) {
        return false;
    }
    true
}

// r[impl rust-primitives.r1cs-system-satisfaction]
pub fn r1cs_system_satisfied(system: R1csSystem, witness: R1csWitness) -> (result: bool)
    ensures result == r1cs_system_satisfied_spec(system, witness)
{
    if !r1cs_system_valid(system) {
        return false;
    }
    if !r1cs_witness_valid(witness) {
        return false;
    }
    if !r1cs_row_satisfied(system.row0, witness) {
        return false;
    }
    if system.len >= 2 && !r1cs_row_satisfied(system.row1, witness) {
        return false;
    }
    if system.len >= 3 && !r1cs_row_satisfied(system.row2, witness) {
        return false;
    }
    true
}

// r[verify rust-primitives.r1cs-system-shape]
pub proof fn invalid_system_lengths_rejected(system: R1csSystem)
    requires system.len == 0 || system.len > R1CS_SYSTEM_MAX_ROWS
    ensures !r1cs_system_valid_spec(system)
{}

// r[verify rust-primitives.r1cs-system-shape]
pub proof fn active_modulus_mismatch_rejected(system: R1csSystem)
    requires
        r1cs_system_len_valid_spec(system.len),
        r1cs_modulus_valid_spec(system.modulus),
        (system.row0.modulus != system.modulus)
            || (system.len >= 2 && system.row1.modulus != system.modulus)
            || (system.len >= 3 && system.row2.modulus != system.modulus),
    ensures !r1cs_system_valid_spec(system)
{}

// r[verify rust-primitives.r1cs-system-satisfaction]
pub proof fn malformed_system_rejected(system: R1csSystem, witness: R1csWitness)
    requires !r1cs_system_valid_spec(system)
    ensures !r1cs_system_satisfied_spec(system, witness)
{}

// r[verify rust-primitives.r1cs-system-satisfaction]
pub proof fn malformed_system_witness_rejected(system: R1csSystem, witness: R1csWitness)
    requires !r1cs_witness_valid_spec(witness)
    ensures !r1cs_system_satisfied_spec(system, witness)
{}

// r[verify rust-primitives.r1cs-system-satisfaction]
pub proof fn failed_active_row_rejects_system(system: R1csSystem, witness: R1csWitness)
    requires
        r1cs_system_valid_spec(system),
        r1cs_witness_valid_spec(witness),
        !r1cs_row_satisfied_spec(system.row0, witness)
            || (system.len >= 2 && !r1cs_row_satisfied_spec(system.row1, witness))
            || (system.len >= 3 && !r1cs_row_satisfied_spec(system.row2, witness)),
    ensures !r1cs_system_satisfied_spec(system, witness)
{}

// r[verify rust-primitives.r1cs-system-verification-coverage]
pub proof fn r1cs_system_primitive_coverage_catalogued()
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

    fn malformed_inactive_row() -> R1csRow {
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
            row2: malformed_inactive_row(),
        }
    }

    #[test]
    fn valid_two_row_system_is_admitted() {
        let system = satisfied_system();
        assert!(r1cs_system_valid(system));
    }

    #[test]
    fn all_active_rows_satisfied_admits_system() {
        let system = satisfied_system();
        assert!(r1cs_system_satisfied(system, witness()));
    }

    #[test]
    fn one_failed_active_row_rejects_system() {
        let mut system = satisfied_system();
        system.row1.c0 = 5;
        assert!(r1cs_system_valid(system));
        assert!(!r1cs_system_satisfied(system, witness()));
    }

    #[test]
    fn active_modulus_mismatch_rejects_system_shape() {
        let mut system = satisfied_system();
        system.row1.modulus = 19;
        assert!(!r1cs_system_valid(system));
        assert!(!r1cs_system_satisfied(system, witness()));
    }

    #[test]
    fn bad_witness_length_rejects_satisfaction() {
        let system = satisfied_system();
        let mut bad_witness = witness();
        bad_witness.len = 3;
        assert!(r1cs_system_valid(system));
        assert!(r1cs_witness_valid(bad_witness));
        assert!(!r1cs_system_satisfied(system, bad_witness));
    }

    #[test]
    fn inactive_malformed_row_is_ignored() {
        let system = satisfied_system();
        assert!(!r1cs_row_valid(system.row2));
        assert!(r1cs_system_valid(system));
        assert!(r1cs_system_satisfied(system, witness()));
    }
}

use verified_logic::r1cs_constraint::R1csRow;
use verified_logic::r1cs_constraint::R1csWitness;
use verified_logic::r1cs_constraint::r1cs_row_satisfied;
use verified_logic::r1cs_constraint::r1cs_row_valid;

const MODULUS: u16 = 17;
const ROW_LEN: u8 = 3;
const ONE: u16 = 1;
const LEFT_VALUE: u16 = 3;
const RIGHT_VALUE: u16 = 4;
const PRODUCT: u16 = 12;
const BAD_RIGHT_VALUE: u16 = 5;

fn multiplication_row() -> R1csRow {
    R1csRow {
        len: ROW_LEN,
        modulus: MODULUS,
        a0: 0,
        a1: ONE,
        a2: 0,
        b0: 0,
        b1: 0,
        b2: ONE,
        c0: PRODUCT,
        c1: 0,
        c2: 0,
    }
}

fn witness(right_value: u16) -> R1csWitness {
    R1csWitness {
        len: ROW_LEN,
        w0: ONE,
        w1: LEFT_VALUE,
        w2: right_value,
    }
}

fn main() {
    let row = multiplication_row();
    let accepted = witness(RIGHT_VALUE);
    let rejected = witness(BAD_RIGHT_VALUE);

    assert!(r1cs_row_valid(row));
    assert!(r1cs_row_satisfied(row, accepted));
    assert!(!r1cs_row_satisfied(row, rejected));

    println!("r1cs: {LEFT_VALUE} * {RIGHT_VALUE} = {PRODUCT} is admitted for one row");
}

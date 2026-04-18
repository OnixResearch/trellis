//! Verified constant-folding lattice and transfer core.
//!
//! Models classic constant propagation over the shared raw compiler IR from
//! `ssa_construction`: each variable maps to `Top` (not yet known),
//! `Const(value)`, or `Bottom` (overdefined / conflicting).
//!
//! This slice currently covers:
//! - lattice ordering and meet,
//! - exact fold semantics for a small opcode set,
//! - per-instruction transfer over `RawInstr` environments,
//! - core lattice proofs and transfer monotonicity,
//! - forward fixed-point iteration over `RawProgram` blocks using reverse postorder.

#![allow(clippy::match_same_arms, clippy::too_many_lines)]

use vstd::prelude::*;

use crate::fetch_add::u64_wrapping_add;
use crate::ssa_construction::RawInstr;
use crate::ssa_construction::RawProgram;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstLat {
    Top,
    Const(u64),
    Bottom,
}

#[derive(Debug)]
pub struct ConstPropSets {
    pub in_env: Vec<Vec<ConstLat>>,
    pub out_env: Vec<Vec<ConstLat>>,
}

pub const OPCODE_COPY: u8 = 0u8;
pub const OPCODE_ADD: u8 = 1u8;
pub const OPCODE_SUB: u8 = 2u8;
pub const OPCODE_MUL: u8 = 3u8;
pub const OPCODE_DIV: u8 = 4u8;
pub const OPCODE_AND: u8 = 5u8;
pub const OPCODE_OR: u8 = 6u8;
pub const OPCODE_XOR: u8 = 7u8;
pub const OPCODE_SHL: u8 = 8u8;
pub const OPCODE_SHR: u8 = 9u8;
pub const OPCODE_EQ: u8 = 10u8;
pub const OPCODE_NE: u8 = 11u8;
pub const OPCODE_LT: u8 = 12u8;
pub const OPCODE_LE: u8 = 13u8;

// r[impl constfold.lattice]
// r[depends constfold.lattice]
pub closed spec fn lattice_le_spec(a: ConstLat, b: ConstLat) -> bool {
    match a {
        ConstLat::Bottom => true,
        ConstLat::Top => matches!(b, ConstLat::Top),
        ConstLat::Const(x) => match b {
            ConstLat::Bottom => false,
            ConstLat::Top => true,
            ConstLat::Const(y) => x == y,
        },
    }
}

// r[impl constfold.lattice]
pub fn lattice_le(a: ConstLat, b: ConstLat) -> (result: bool)
    ensures result == lattice_le_spec(a, b)
{
    match a {
        ConstLat::Bottom => true,
        ConstLat::Top => matches!(b, ConstLat::Top),
        ConstLat::Const(x) => match b {
            ConstLat::Bottom => false,
            ConstLat::Top => true,
            ConstLat::Const(y) => x == y,
        },
    }
}

spec fn lattice_lt_spec(a: ConstLat, b: ConstLat) -> bool {
    lattice_le_spec(a, b) && a != b
}

spec fn lattice_rank_spec(v: ConstLat) -> nat {
    match v {
        ConstLat::Bottom => 0nat,
        ConstLat::Const(_) => 1nat,
        ConstLat::Top => 2nat,
    }
}

// r[impl constfold.lattice]
// r[depends constfold.lattice]
pub closed spec fn meet_spec(a: ConstLat, b: ConstLat) -> ConstLat {
    match a {
        ConstLat::Top => b,
        ConstLat::Bottom => ConstLat::Bottom,
        ConstLat::Const(x) => match b {
            ConstLat::Top => ConstLat::Const(x),
            ConstLat::Bottom => ConstLat::Bottom,
            ConstLat::Const(y) => if x == y { ConstLat::Const(x) } else { ConstLat::Bottom },
        },
    }
}

// r[impl constfold.lattice]
pub fn meet(a: ConstLat, b: ConstLat) -> (result: ConstLat)
    ensures result == meet_spec(a, b)
{
    match a {
        ConstLat::Top => b,
        ConstLat::Bottom => ConstLat::Bottom,
        ConstLat::Const(x) => match b {
            ConstLat::Top => ConstLat::Const(x),
            ConstLat::Bottom => ConstLat::Bottom,
            ConstLat::Const(y) => if x == y { ConstLat::Const(x) } else { ConstLat::Bottom },
        },
    }
}

proof fn lattice_const_comparable_equal(x: u64, y: u64)
    requires lattice_le_spec(ConstLat::Const(x), ConstLat::Const(y))
    ensures x == y
{}

// r[verify constfold.lattice]
proof fn meet_commutative(a: ConstLat, b: ConstLat)
    ensures meet_spec(a, b) == meet_spec(b, a)
{
    match a {
        ConstLat::Top => {}
        ConstLat::Bottom => {}
        ConstLat::Const(x) => match b {
            ConstLat::Top => {}
            ConstLat::Bottom => {}
            ConstLat::Const(y) => {}
        },
    }
}

// r[verify constfold.lattice]
proof fn meet_associative(a: ConstLat, b: ConstLat, c: ConstLat)
    ensures meet_spec(meet_spec(a, b), c) == meet_spec(a, meet_spec(b, c))
{
    match a {
        ConstLat::Top => {}
        ConstLat::Bottom => {}
        ConstLat::Const(x) => match b {
            ConstLat::Top => {}
            ConstLat::Bottom => {}
            ConstLat::Const(y) => match c {
                ConstLat::Top => {}
                ConstLat::Bottom => {}
                ConstLat::Const(z) => {}
            },
        },
    }
}

// r[verify constfold.lattice]
proof fn meet_idempotent(a: ConstLat)
    ensures meet_spec(a, a) == a
{
    match a {
        ConstLat::Top => {}
        ConstLat::Bottom => {}
        ConstLat::Const(_) => {}
    }
}

// r[verify constfold.lattice]
proof fn meet_top_identity(a: ConstLat)
    ensures
        meet_spec(ConstLat::Top, a) == a,
        meet_spec(a, ConstLat::Top) == a,
{
    match a {
        ConstLat::Top => {}
        ConstLat::Bottom => {}
        ConstLat::Const(_) => {}
    }
}

// r[verify constfold.lattice]
proof fn meet_bottom_absorbing(a: ConstLat)
    ensures
        meet_spec(ConstLat::Bottom, a) == ConstLat::Bottom,
        meet_spec(a, ConstLat::Bottom) == ConstLat::Bottom,
{
    match a {
        ConstLat::Top => {}
        ConstLat::Bottom => {}
        ConstLat::Const(_) => {}
    }
}

// r[verify constfold.convergence]
proof fn meet_lower_bound_left(a: ConstLat, b: ConstLat)
    ensures lattice_le_spec(meet_spec(a, b), a)
{
    match a {
        ConstLat::Top => {}
        ConstLat::Bottom => {}
        ConstLat::Const(x) => match b {
            ConstLat::Top => {}
            ConstLat::Bottom => {}
            ConstLat::Const(y) => {
                if x != y {
                    assert(lattice_le_spec(ConstLat::Bottom, ConstLat::Const(x)));
                }
            }
        },
    }
}

// r[verify constfold.convergence]
proof fn meet_lower_bound_right(a: ConstLat, b: ConstLat)
    ensures lattice_le_spec(meet_spec(a, b), b)
{
    meet_commutative(a, b);
    meet_lower_bound_left(b, a);
}

// r[verify constfold.convergence]
proof fn meet_monotone(a1: ConstLat, a2: ConstLat, b1: ConstLat, b2: ConstLat)
    requires
        lattice_le_spec(a1, a2),
        lattice_le_spec(b1, b2),
    ensures lattice_le_spec(meet_spec(a1, b1), meet_spec(a2, b2))
{
    match a1 {
        ConstLat::Bottom => {}
        ConstLat::Top => {
            assert(a2 == ConstLat::Top);
            assert(meet_spec(a1, b1) == b1);
            assert(meet_spec(a2, b2) == b2);
        }
        ConstLat::Const(x1) => {
            match a2 {
                ConstLat::Bottom => { assert(false); }
                ConstLat::Top => {
                    match b1 {
                        ConstLat::Bottom => {}
                        ConstLat::Top => {
                            assert(b2 == ConstLat::Top);
                        }
                        ConstLat::Const(y1) => {
                            match b2 {
                                ConstLat::Bottom => { assert(false); }
                                ConstLat::Top => {}
                                ConstLat::Const(y2) => {
                                    assert(y1 == y2);
                                }
                            }
                        }
                    }
                }
                ConstLat::Const(x2) => {
                    assert(x1 == x2);
                    match b1 {
                        ConstLat::Bottom => {}
                        ConstLat::Top => {
                            assert(b2 == ConstLat::Top);
                        }
                        ConstLat::Const(y1) => {
                            match b2 {
                                ConstLat::Bottom => { assert(false); }
                                ConstLat::Top => {}
                                ConstLat::Const(y2) => {
                                    assert(y1 == y2);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// r[impl constfold.finite-height]
// r[verify constfold.finite-height]
proof fn finite_height_no_three_step_descent(a: ConstLat, b: ConstLat, c: ConstLat, d: ConstLat)
    requires
        lattice_lt_spec(d, c),
        lattice_lt_spec(c, b),
        lattice_lt_spec(b, a),
    ensures false
{
    assert(lattice_rank_spec(d) < lattice_rank_spec(c));
    assert(lattice_rank_spec(c) < lattice_rank_spec(b));
    assert(lattice_rank_spec(b) < lattice_rank_spec(a));
    assert(lattice_rank_spec(a) <= 2);
    assert(lattice_rank_spec(d) >= 0);
}

spec fn is_binary_opcode_spec(opcode: u8) -> bool {
    opcode == OPCODE_ADD
        || opcode == OPCODE_SUB
        || opcode == OPCODE_MUL
        || opcode == OPCODE_DIV
        || opcode == OPCODE_AND
        || opcode == OPCODE_OR
        || opcode == OPCODE_XOR
        || opcode == OPCODE_SHL
        || opcode == OPCODE_SHR
        || opcode == OPCODE_EQ
        || opcode == OPCODE_NE
        || opcode == OPCODE_LT
        || opcode == OPCODE_LE
}

spec fn u64_modulus_spec() -> int {
    u64::MAX as int + 1
}

spec fn u64_wrapping_add_spec(a: u64, b: u64) -> u64 {
    (((a as int) + (b as int)) % u64_modulus_spec()) as u64
}

spec fn u64_wrapping_neg_spec(x: u64) -> u64 {
    if x == 0 {
        0u64
    } else {
        (u64_modulus_spec() - x as int) as u64
    }
}

spec fn u64_wrapping_sub_spec(a: u64, b: u64) -> u64 {
    u64_wrapping_add_spec(a, u64_wrapping_neg_spec(b))
}

spec fn u64_wrapping_mul_spec(a: u64, b: u64) -> u64 {
    (((a as int) * (b as int)) % u64_modulus_spec()) as u64
}

// r[verify constfold.fold]
proof fn wrapping_sub_matches_spec(a: u64, b: u64)
    ensures u64_wrapping_sub_spec(a, b) == a.wrapping_sub(b)
{
    assert(u64_wrapping_sub_spec(a, b) == a.wrapping_sub(b)) by(bit_vector);
}

// r[verify constfold.fold]
proof fn wrapping_mul_matches_spec(a: u64, b: u64)
    ensures u64_wrapping_mul_spec(a, b) == a.wrapping_mul(b)
{
    assert(u64_wrapping_mul_spec(a, b) == a.wrapping_mul(b)) by(bit_vector);
}

// r[impl constfold.fold]
// r[depends constfold.fold]
pub closed spec fn fold_expr_spec(opcode: u8, a: u64, b: u64) -> ConstLat {
    if opcode == OPCODE_ADD {
        ConstLat::Const(u64_wrapping_add_spec(a, b))
    } else if opcode == OPCODE_SUB {
        ConstLat::Const(u64_wrapping_sub_spec(a, b))
    } else if opcode == OPCODE_MUL {
        ConstLat::Const(u64_wrapping_mul_spec(a, b))
    } else if opcode == OPCODE_DIV {
        if b == 0 {
            ConstLat::Bottom
        } else {
            ConstLat::Const(a / b)
        }
    } else if opcode == OPCODE_AND {
        ConstLat::Const(a & b)
    } else if opcode == OPCODE_OR {
        ConstLat::Const(a | b)
    } else if opcode == OPCODE_XOR {
        ConstLat::Const(a ^ b)
    } else if opcode == OPCODE_SHL {
        if b < 64 {
            ConstLat::Const(a << (b as u32))
        } else {
            ConstLat::Bottom
        }
    } else if opcode == OPCODE_SHR {
        if b < 64 {
            ConstLat::Const(a >> (b as u32))
        } else {
            ConstLat::Bottom
        }
    } else if opcode == OPCODE_EQ {
        ConstLat::Const(if a == b { 1u64 } else { 0u64 })
    } else if opcode == OPCODE_NE {
        ConstLat::Const(if a != b { 1u64 } else { 0u64 })
    } else if opcode == OPCODE_LT {
        ConstLat::Const(if a < b { 1u64 } else { 0u64 })
    } else if opcode == OPCODE_LE {
        ConstLat::Const(if a <= b { 1u64 } else { 0u64 })
    } else {
        ConstLat::Bottom
    }
}

// r[impl constfold.fold]
pub fn fold_expr(opcode: u8, a: u64, b: u64) -> (result: ConstLat)
    ensures result == fold_expr_spec(opcode, a, b)
{
    if opcode == OPCODE_ADD {
        ConstLat::Const(u64_wrapping_add(a, b))
    } else if opcode == OPCODE_SUB {
        let result = a.wrapping_sub(b);
        proof {
            wrapping_sub_matches_spec(a, b);
        }
        ConstLat::Const(result)
    } else if opcode == OPCODE_MUL {
        let result = a.wrapping_mul(b);
        proof {
            wrapping_mul_matches_spec(a, b);
        }
        ConstLat::Const(result)
    } else if opcode == OPCODE_DIV {
        match a.checked_div(b) {
            Some(q) => ConstLat::Const(q),
            None => ConstLat::Bottom,
        }
    } else if opcode == OPCODE_AND {
        ConstLat::Const(a & b)
    } else if opcode == OPCODE_OR {
        ConstLat::Const(a | b)
    } else if opcode == OPCODE_XOR {
        ConstLat::Const(a ^ b)
    } else if opcode == OPCODE_SHL {
        if b < 64 {
            let shift = u32::try_from(b).expect("bounded shift");
            ConstLat::Const(a << shift)
        } else {
            ConstLat::Bottom
        }
    } else if opcode == OPCODE_SHR {
        if b < 64 {
            let shift = u32::try_from(b).expect("bounded shift");
            ConstLat::Const(a >> shift)
        } else {
            ConstLat::Bottom
        }
    } else if opcode == OPCODE_EQ {
        ConstLat::Const(if a == b { 1u64 } else { 0u64 })
    } else if opcode == OPCODE_NE {
        ConstLat::Const(if a != b { 1u64 } else { 0u64 })
    } else if opcode == OPCODE_LT {
        ConstLat::Const(if a < b { 1u64 } else { 0u64 })
    } else if opcode == OPCODE_LE {
        ConstLat::Const(if a <= b { 1u64 } else { 0u64 })
    } else {
        ConstLat::Bottom
    }
}

spec fn transfer_value_spec(opcode: u8, uses: Seq<u32>, env: Seq<ConstLat>) -> ConstLat {
    if opcode == OPCODE_COPY {
        if uses.len() != 1 {
            ConstLat::Bottom
        } else if (uses[0] as int) < env.len() {
            env[uses[0] as int]
        } else {
            ConstLat::Bottom
        }
    } else if is_binary_opcode_spec(opcode) {
        if uses.len() != 2 {
            ConstLat::Bottom
        } else if (uses[0] as int) < env.len() && (uses[1] as int) < env.len() {
            let a = env[uses[0] as int];
            let b = env[uses[1] as int];
            match (a, b) {
                (ConstLat::Bottom, _) | (_, ConstLat::Bottom) => ConstLat::Bottom,
                (ConstLat::Top, _) | (_, ConstLat::Top) => ConstLat::Top,
                (ConstLat::Const(x), ConstLat::Const(y)) => fold_expr_spec(opcode, x, y),
            }
        } else {
            ConstLat::Bottom
        }
    } else {
        ConstLat::Bottom
    }
}

spec fn update_env_spec(env: Seq<ConstLat>, dst: u32, value: ConstLat) -> Seq<ConstLat> {
    if (dst as int) < env.len() {
        env.update(dst as int, value)
    } else {
        env
    }
}

spec fn env_le_spec(a: Seq<ConstLat>, b: Seq<ConstLat>) -> bool {
    a.len() == b.len()
        && forall|i: int| 0 <= i < a.len() ==> #[trigger] lattice_le_spec(a[i], b[i])
}

pub closed spec fn constprop_matrix_shape_spec(matrix: Seq<Vec<ConstLat>>, rows: int, cols: int) -> bool {
    matrix.len() == rows
        && forall|i: int| #![trigger matrix[i]] 0 <= i < rows ==> matrix[i]@.len() == cols
}

pub closed spec fn constprop_sets_shape_spec(sets: ConstPropSets, blocks: int, vars: int) -> bool {
    constprop_matrix_shape_spec(sets.in_env@, blocks, vars)
        && constprop_matrix_shape_spec(sets.out_env@, blocks, vars)
}

pub closed spec fn constprop_matrix_le_spec(a: Seq<Vec<ConstLat>>, b: Seq<Vec<ConstLat>>) -> bool {
    a.len() == b.len()
        && forall|r: int| #![trigger a[r], b[r]] 0 <= r < a.len() ==> env_le_spec(a[r]@, b[r]@)
}

pub closed spec fn top_env_spec(vars: nat) -> Seq<ConstLat> {
    Seq::new(vars, |_i: int| ConstLat::Top)
}

spec fn matrix_is_top_spec(matrix: Seq<Vec<ConstLat>>, rows: int, cols: int) -> bool {
    constprop_matrix_shape_spec(matrix, rows, cols)
        && forall|r: int, c: int| #![trigger matrix[r]@[c]]
            0 <= r < rows && 0 <= c < cols ==> matrix[r]@[c] == ConstLat::Top
}

spec fn env_rank_sum_spec(env: Seq<ConstLat>, from: int) -> int
    decreases env.len() - from,
{
    if from >= env.len() {
        0
    } else {
        lattice_rank_spec(env[from]) as int + env_rank_sum_spec(env, from + 1)
    }
}

spec fn matrix_rank_sum_spec(matrix: Seq<Vec<ConstLat>>, from: int) -> int
    decreases matrix.len() - from,
{
    if from >= matrix.len() {
        0
    } else {
        env_rank_sum_spec(matrix[from]@, 0) + matrix_rank_sum_spec(matrix, from + 1)
    }
}

pub closed spec fn meet_env_spec(a: Seq<ConstLat>, b: Seq<ConstLat>) -> Seq<ConstLat>
    recommends a.len() == b.len()
{
    Seq::new(a.len(), |i: int| meet_spec(a[i], b[i]))
}

pub closed spec fn apply_block_prefix_spec(instrs: Seq<RawInstr>, env: Seq<ConstLat>, upto: int) -> Seq<ConstLat>
    decreases if upto >= 0 { upto } else { 0 },
{
    if upto <= 0 {
        env
    } else {
        transfer_spec(
            instrs[upto - 1].def,
            instrs[upto - 1].uses@,
            instrs[upto - 1].opcode,
            apply_block_prefix_spec(instrs, env, upto - 1),
        )
    }
}

pub closed spec fn apply_block_spec(instrs: Seq<RawInstr>, env: Seq<ConstLat>) -> Seq<ConstLat> {
    apply_block_prefix_spec(instrs, env, instrs.len() as int)
}

pub closed spec fn pred_meet_prefix_spec(
    edges: Seq<(usize, usize)>, out_env: Seq<Vec<ConstLat>>, vars: nat, block: int, upto: int,
) -> Seq<ConstLat>
    decreases if upto >= 0 { upto } else { 0 },
{
    if upto <= 0 {
        top_env_spec(vars)
    } else {
        let prev = pred_meet_prefix_spec(edges, out_env, vars, block, upto - 1);
        let edge = edges[upto - 1];
        if edge.1 as int == block {
            meet_env_spec(prev, out_env[edge.0 as int]@)
        } else {
            prev
        }
    }
}

pub closed spec fn block_in_env_spec(program: RawProgram, entry_env: Seq<ConstLat>, out_env: Seq<Vec<ConstLat>>, block: int) -> Seq<ConstLat> {
    if block == 0 {
        entry_env
    } else {
        pred_meet_prefix_spec(program.edges@, out_env, entry_env.len() as nat, block, program.edges@.len() as int)
    }
}

pub closed spec fn block_out_env_spec(program: RawProgram, entry_env: Seq<ConstLat>, out_env: Seq<Vec<ConstLat>>, block: int) -> Seq<ConstLat> {
    apply_block_spec(program.blocks@[block].instrs@, block_in_env_spec(program, entry_env, out_env, block))
}

pub closed spec fn fixed_point_spec(program: RawProgram, entry_env: Seq<ConstLat>, sets: ConstPropSets) -> bool {
    &&& valid_constprop_program_spec(program)
    &&& constprop_sets_shape_spec(sets, program.blocks@.len() as int, entry_env.len() as int)
    &&& forall|b: int| #![trigger sets.in_env@[b], sets.out_env@[b]]
        0 <= b < program.blocks@.len() ==> {
            &&& sets.in_env@[b]@ == block_in_env_spec(program, entry_env, sets.out_env@, b)
            &&& sets.out_env@[b]@ == apply_block_spec(program.blocks@[b].instrs@, sets.in_env@[b]@)
        }
}

pub closed spec fn round_from_out_env_spec(program: RawProgram, entry_env: Seq<ConstLat>, prev_out: Seq<Vec<ConstLat>>, next: ConstPropSets) -> bool {
    &&& valid_constprop_program_spec(program)
    &&& constprop_matrix_shape_spec(prev_out, program.blocks@.len() as int, entry_env.len() as int)
    &&& constprop_sets_shape_spec(next, program.blocks@.len() as int, entry_env.len() as int)
    &&& forall|b: int| #![trigger next.in_env@[b], next.out_env@[b]]
        0 <= b < program.blocks@.len() ==> {
            &&& next.in_env@[b]@ == block_in_env_spec(program, entry_env, prev_out, b)
            &&& next.out_env@[b]@ == block_out_env_spec(program, entry_env, prev_out, b)
        }
}

pub closed spec fn valid_constprop_program_spec(program: RawProgram) -> bool {
    forall|i: int| #![trigger program.edges[i]] 0 <= i < program.edges.len() ==> {
        let (src, dst) = program.edges[i];
        src < program.blocks.len() && dst < program.blocks.len()
    }
}

proof fn env_rank_sum_nonneg(env: Seq<ConstLat>, from: int)
    requires 0 <= from <= env.len()
    ensures 0 <= env_rank_sum_spec(env, from)
    decreases env.len() - from,
{
    if from < env.len() {
        env_rank_sum_nonneg(env, from + 1);
        assert(0 <= lattice_rank_spec(env[from]) as int);
        assert(env_rank_sum_spec(env, from) == lattice_rank_spec(env[from]) as int + env_rank_sum_spec(env, from + 1));
    }
}

proof fn matrix_rank_sum_nonneg(matrix: Seq<Vec<ConstLat>>, from: int)
    requires 0 <= from <= matrix.len()
    ensures 0 <= matrix_rank_sum_spec(matrix, from)
    decreases matrix.len() - from,
{
    if from < matrix.len() {
        env_rank_sum_nonneg(matrix[from]@, 0);
        matrix_rank_sum_nonneg(matrix, from + 1);
        assert(matrix_rank_sum_spec(matrix, from) == env_rank_sum_spec(matrix[from]@, 0) + matrix_rank_sum_spec(matrix, from + 1));
    }
}

proof fn env_rank_sum_bound(env: Seq<ConstLat>, from: int)
    requires 0 <= from <= env.len()
    ensures env_rank_sum_spec(env, from) <= 2 * (env.len() - from)
    decreases env.len() - from,
{
    if from < env.len() {
        env_rank_sum_bound(env, from + 1);
        assert(lattice_rank_spec(env[from]) <= 2);
        assert(env_rank_sum_spec(env, from) == lattice_rank_spec(env[from]) as int + env_rank_sum_spec(env, from + 1));
        assert(env_rank_sum_spec(env, from + 1) <= 2 * (env.len() - (from + 1)));
        assert(2 * (env.len() - (from + 1)) == 2 * (env.len() - from) - 2);
        assert(env_rank_sum_spec(env, from) <= 2 + (2 * (env.len() - from) - 2));
    }
}

proof fn lattice_le_refl(a: ConstLat)
    ensures lattice_le_spec(a, a)
{
    match a {
        ConstLat::Top => {}
        ConstLat::Bottom => {}
        ConstLat::Const(_) => {}
    }
}

// r[verify constfold.convergence]
proof fn lattice_le_top(a: ConstLat)
    ensures lattice_le_spec(a, ConstLat::Top)
{
    match a {
        ConstLat::Top => {}
        ConstLat::Bottom => {}
        ConstLat::Const(_) => {}
    }
}

proof fn env_le_refl(env: Seq<ConstLat>)
    ensures env_le_spec(env, env)
{
    assert forall|i: int| 0 <= i < env.len() implies #[trigger] lattice_le_spec(env[i], env[i]) by {
        lattice_le_refl(env[i]);
    };
}

// r[verify constfold.convergence]
proof fn env_le_top(env: Seq<ConstLat>)
    ensures env_le_spec(env, top_env_spec(env.len() as nat))
{
    assert forall|i: int| 0 <= i < env.len() implies #[trigger] lattice_le_spec(env[i], top_env_spec(env.len() as nat)[i]) by {
        assert(top_env_spec(env.len() as nat)[i] == ConstLat::Top);
        lattice_le_top(env[i]);
    };
}

proof fn top_matrix_row_is_top_env(matrix: Seq<Vec<ConstLat>>, rows: int, cols: int, row: int)
    requires
        matrix_is_top_spec(matrix, rows, cols),
        0 <= row < rows,
    ensures matrix[row]@ == top_env_spec(cols as nat)
{
    assert(matrix[row]@.len() == cols);
    assert(top_env_spec(cols as nat).len() == cols);
    assert forall|i: int| 0 <= i < cols implies #[trigger] matrix[row]@[i] == top_env_spec(cols as nat)[i] by {
        assert(matrix[row]@[i] == ConstLat::Top);
        assert(top_env_spec(cols as nat)[i] == ConstLat::Top);
    };
    assert(matrix[row]@ == top_env_spec(cols as nat));
}

proof fn lattice_le_rank_le(a: ConstLat, b: ConstLat)
    requires lattice_le_spec(a, b)
    ensures lattice_rank_spec(a) <= lattice_rank_spec(b)
{
    match a {
        ConstLat::Bottom => {}
        ConstLat::Top => {}
        ConstLat::Const(x) => {
            match b {
                ConstLat::Bottom => { assert(false); }
                ConstLat::Top => {}
                ConstLat::Const(y) => {
                    assert(x == y);
                }
            }
        }
    }
}

proof fn lattice_le_rank_lt(a: ConstLat, b: ConstLat)
    requires
        lattice_le_spec(a, b),
        a != b,
    ensures lattice_rank_spec(a) < lattice_rank_spec(b)
{
    match a {
        ConstLat::Bottom => {
            match b {
                ConstLat::Bottom => { assert(false); }
                ConstLat::Top => {}
                ConstLat::Const(_) => {}
            }
        }
        ConstLat::Top => {
            assert(false);
        }
        ConstLat::Const(x) => {
            match b {
                ConstLat::Bottom => { assert(false); }
                ConstLat::Top => {}
                ConstLat::Const(y) => {
                    assert(x == y);
                    assert(false);
                }
            }
        }
    }
}

proof fn seq_constlat_neq_has_witness(a: Seq<ConstLat>, b: Seq<ConstLat>)
    requires
        a.len() == b.len(),
        a != b,
    ensures exists|i: int| 0 <= i < a.len() && a[i] != b[i]
{
    if !(exists|i: int| 0 <= i < a.len() && a[i] != b[i]) {
        assert forall|i: int| 0 <= i < a.len() implies a[i] == b[i] by {};
        assert(a == b);
        assert(false);
    }
}

proof fn env_rank_le_from(a: Seq<ConstLat>, b: Seq<ConstLat>, from: int)
    requires
        env_le_spec(a, b),
        0 <= from <= a.len(),
    ensures env_rank_sum_spec(a, from) <= env_rank_sum_spec(b, from)
    decreases a.len() - from,
{
    if from < a.len() {
        lattice_le_rank_le(a[from], b[from]);
        env_rank_le_from(a, b, from + 1);
        assert(env_rank_sum_spec(a, from) == lattice_rank_spec(a[from]) as int + env_rank_sum_spec(a, from + 1));
        assert(env_rank_sum_spec(b, from) == lattice_rank_spec(b[from]) as int + env_rank_sum_spec(b, from + 1));
        assert((lattice_rank_spec(a[from]) as int) <= (lattice_rank_spec(b[from]) as int));
        assert(env_rank_sum_spec(a, from + 1) <= env_rank_sum_spec(b, from + 1));
        assert(env_rank_sum_spec(a, from) <= env_rank_sum_spec(b, from));
    }
}

proof fn env_rank_lt_at(a: Seq<ConstLat>, b: Seq<ConstLat>, from: int, witness: int)
    requires
        env_le_spec(a, b),
        0 <= from <= witness < a.len(),
        a[witness] != b[witness],
    ensures env_rank_sum_spec(a, from) < env_rank_sum_spec(b, from)
    decreases a.len() - from,
{
    if from == witness {
        lattice_le_rank_lt(a[from], b[from]);
        env_rank_le_from(a, b, from + 1);
        assert(env_rank_sum_spec(a, from) == lattice_rank_spec(a[from]) as int + env_rank_sum_spec(a, from + 1));
        assert(env_rank_sum_spec(b, from) == lattice_rank_spec(b[from]) as int + env_rank_sum_spec(b, from + 1));
        assert((lattice_rank_spec(a[from]) as int) < (lattice_rank_spec(b[from]) as int));
        assert(env_rank_sum_spec(a, from + 1) <= env_rank_sum_spec(b, from + 1));
        assert(env_rank_sum_spec(a, from) < env_rank_sum_spec(b, from));
    } else {
        lattice_le_rank_le(a[from], b[from]);
        env_rank_lt_at(a, b, from + 1, witness);
        assert(env_rank_sum_spec(a, from) == lattice_rank_spec(a[from]) as int + env_rank_sum_spec(a, from + 1));
        assert(env_rank_sum_spec(b, from) == lattice_rank_spec(b[from]) as int + env_rank_sum_spec(b, from + 1));
        assert((lattice_rank_spec(a[from]) as int) <= (lattice_rank_spec(b[from]) as int));
        assert(env_rank_sum_spec(a, from + 1) < env_rank_sum_spec(b, from + 1));
        assert(env_rank_sum_spec(a, from) < env_rank_sum_spec(b, from));
    }
}

proof fn env_rank_lt(a: Seq<ConstLat>, b: Seq<ConstLat>)
    requires
        env_le_spec(a, b),
        exists|i: int| 0 <= i < a.len() && a[i] != b[i],
    ensures env_rank_sum_spec(a, 0) < env_rank_sum_spec(b, 0)
{
    let witness = choose|i: int| 0 <= i < a.len() && a[i] != b[i];
    assert(0 <= witness < a.len() && a[witness] != b[witness]);
    env_rank_lt_at(a, b, 0, witness);
}

proof fn matrix_rank_le_from(a: Seq<Vec<ConstLat>>, b: Seq<Vec<ConstLat>>, from: int)
    requires
        constprop_matrix_le_spec(a, b),
        0 <= from <= a.len(),
    ensures matrix_rank_sum_spec(a, from) <= matrix_rank_sum_spec(b, from)
    decreases a.len() - from,
{
    if from < a.len() {
        env_rank_le_from(a[from]@, b[from]@, 0);
        matrix_rank_le_from(a, b, from + 1);
        assert(matrix_rank_sum_spec(a, from) == env_rank_sum_spec(a[from]@, 0) + matrix_rank_sum_spec(a, from + 1));
        assert(matrix_rank_sum_spec(b, from) == env_rank_sum_spec(b[from]@, 0) + matrix_rank_sum_spec(b, from + 1));
        assert(env_rank_sum_spec(a[from]@, 0) <= env_rank_sum_spec(b[from]@, 0));
        assert(matrix_rank_sum_spec(a, from + 1) <= matrix_rank_sum_spec(b, from + 1));
        assert(matrix_rank_sum_spec(a, from) <= matrix_rank_sum_spec(b, from));
    }
}

proof fn matrix_rank_lt_at(a: Seq<Vec<ConstLat>>, b: Seq<Vec<ConstLat>>, from: int, witness: int)
    requires
        constprop_matrix_le_spec(a, b),
        0 <= from <= witness < a.len(),
        a[witness]@ != b[witness]@,
    ensures matrix_rank_sum_spec(a, from) < matrix_rank_sum_spec(b, from)
    decreases a.len() - from,
{
    if from == witness {
        seq_constlat_neq_has_witness(a[from]@, b[from]@);
        env_rank_lt(a[from]@, b[from]@);
        matrix_rank_le_from(a, b, from + 1);
        assert(matrix_rank_sum_spec(a, from) == env_rank_sum_spec(a[from]@, 0) + matrix_rank_sum_spec(a, from + 1));
        assert(matrix_rank_sum_spec(b, from) == env_rank_sum_spec(b[from]@, 0) + matrix_rank_sum_spec(b, from + 1));
        assert(env_rank_sum_spec(a[from]@, 0) < env_rank_sum_spec(b[from]@, 0));
        assert(matrix_rank_sum_spec(a, from + 1) <= matrix_rank_sum_spec(b, from + 1));
        assert(matrix_rank_sum_spec(a, from) < matrix_rank_sum_spec(b, from));
    } else {
        env_rank_le_from(a[from]@, b[from]@, 0);
        matrix_rank_lt_at(a, b, from + 1, witness);
        assert(matrix_rank_sum_spec(a, from) == env_rank_sum_spec(a[from]@, 0) + matrix_rank_sum_spec(a, from + 1));
        assert(matrix_rank_sum_spec(b, from) == env_rank_sum_spec(b[from]@, 0) + matrix_rank_sum_spec(b, from + 1));
        assert(env_rank_sum_spec(a[from]@, 0) <= env_rank_sum_spec(b[from]@, 0));
        assert(matrix_rank_sum_spec(a, from + 1) < matrix_rank_sum_spec(b, from + 1));
        assert(matrix_rank_sum_spec(a, from) < matrix_rank_sum_spec(b, from));
    }
}

proof fn matrix_rank_lt(a: Seq<Vec<ConstLat>>, b: Seq<Vec<ConstLat>>)
    requires
        constprop_matrix_le_spec(a, b),
        exists|r: int| 0 <= r < a.len() && a[r]@ != b[r]@,
    ensures matrix_rank_sum_spec(a, 0) < matrix_rank_sum_spec(b, 0)
{
    let witness = choose|r: int| 0 <= r < a.len() && a[r]@ != b[r]@;
    assert(0 <= witness < a.len() && a[witness]@ != b[witness]@);
    matrix_rank_lt_at(a, b, 0, witness);
}

// r[verify constfold.convergence]
proof fn meet_env_lower_bound_left(a: Seq<ConstLat>, b: Seq<ConstLat>)
    requires a.len() == b.len()
    ensures env_le_spec(meet_env_spec(a, b), a)
{
    assert forall|i: int| 0 <= i < a.len() implies #[trigger] lattice_le_spec(meet_env_spec(a, b)[i], a[i]) by {
        meet_lower_bound_left(a[i], b[i]);
    };
}

// r[verify constfold.convergence]
proof fn meet_env_lower_bound_right(a: Seq<ConstLat>, b: Seq<ConstLat>)
    requires a.len() == b.len()
    ensures env_le_spec(meet_env_spec(a, b), b)
{
    assert forall|i: int| 0 <= i < a.len() implies #[trigger] lattice_le_spec(meet_env_spec(a, b)[i], b[i]) by {
        meet_lower_bound_right(a[i], b[i]);
    };
}

// r[verify constfold.convergence]
proof fn meet_env_monotone(a1: Seq<ConstLat>, a2: Seq<ConstLat>, b1: Seq<ConstLat>, b2: Seq<ConstLat>)
    requires
        env_le_spec(a1, a2),
        env_le_spec(b1, b2),
        a1.len() == b1.len(),
        a2.len() == b2.len(),
    ensures env_le_spec(meet_env_spec(a1, b1), meet_env_spec(a2, b2))
{
    assert forall|i: int| 0 <= i < a1.len() implies #[trigger] lattice_le_spec(meet_env_spec(a1, b1)[i], meet_env_spec(a2, b2)[i]) by {
        meet_monotone(a1[i], a2[i], b1[i], b2[i]);
    };
}

// r[verify constfold.convergence]
proof fn apply_block_prefix_monotone(instrs: Seq<RawInstr>, env1: Seq<ConstLat>, env2: Seq<ConstLat>, upto: int)
    requires
        env_le_spec(env1, env2),
        0 <= upto <= instrs.len(),
    ensures env_le_spec(apply_block_prefix_spec(instrs, env1, upto), apply_block_prefix_spec(instrs, env2, upto))
    decreases upto,
{
    if upto <= 0 {
    } else {
        apply_block_prefix_monotone(instrs, env1, env2, upto - 1);
        transfer_monotone(
            instrs[upto - 1].def,
            instrs[upto - 1].uses@,
            instrs[upto - 1].opcode,
            apply_block_prefix_spec(instrs, env1, upto - 1),
            apply_block_prefix_spec(instrs, env2, upto - 1),
        );
    }
}

// r[verify constfold.convergence]
proof fn apply_block_monotone(instrs: Seq<RawInstr>, env1: Seq<ConstLat>, env2: Seq<ConstLat>)
    requires env_le_spec(env1, env2)
    ensures env_le_spec(apply_block_spec(instrs, env1), apply_block_spec(instrs, env2))
{
    apply_block_prefix_monotone(instrs, env1, env2, instrs.len() as int);
}

// r[verify constfold.convergence]
proof fn pred_meet_prefix_has_len(edges: Seq<(usize, usize)>, out_env: Seq<Vec<ConstLat>>, vars: nat, block: int, upto: int)
    requires
        0 <= upto <= edges.len(),
        forall|i: int| #![trigger edges[i]] 0 <= i < edges.len() ==> {
            let (src, dst) = edges[i];
            src < out_env.len() && dst < out_env.len()
        },
        forall|i: int| #![trigger out_env[i]] 0 <= i < out_env.len() ==> out_env[i]@.len() == vars,
    ensures pred_meet_prefix_spec(edges, out_env, vars, block, upto).len() == vars
    decreases upto,
{
    if upto <= 0 {
    } else {
        pred_meet_prefix_has_len(edges, out_env, vars, block, upto - 1);
        let edge = edges[upto - 1];
        if edge.1 as int == block {
            assert(pred_meet_prefix_spec(edges, out_env, vars, block, upto - 1).len() == vars);
            assert(out_env[edge.0 as int]@.len() == vars);
        }
    }
}

// r[verify constfold.convergence]
proof fn pred_meet_prefix_monotone(
    edges: Seq<(usize, usize)>, out1: Seq<Vec<ConstLat>>, out2: Seq<Vec<ConstLat>>, vars: nat, block: int, upto: int,
)
    requires
        out1.len() == out2.len(),
        forall|i: int| #![trigger out1[i], out2[i]] 0 <= i < out1.len() ==> env_le_spec(out1[i]@, out2[i]@),
        0 <= upto <= edges.len(),
        forall|i: int| #![trigger edges[i]] 0 <= i < edges.len() ==> {
            let (src, dst) = edges[i];
            src < out1.len() && dst < out1.len()
        },
        forall|i: int| #![trigger out1[i]] 0 <= i < out1.len() ==> out1[i]@.len() == vars,
        forall|i: int| #![trigger out2[i]] 0 <= i < out2.len() ==> out2[i]@.len() == vars,
    ensures env_le_spec(
        pred_meet_prefix_spec(edges, out1, vars, block, upto),
        pred_meet_prefix_spec(edges, out2, vars, block, upto),
    )
    decreases upto,
{
    if upto <= 0 {
    } else {
        pred_meet_prefix_monotone(edges, out1, out2, vars, block, upto - 1);
        let edge = edges[upto - 1];
        if edge.1 as int == block {
            pred_meet_prefix_has_len(edges, out1, vars, block, upto - 1);
            pred_meet_prefix_has_len(edges, out2, vars, block, upto - 1);
            meet_env_monotone(
                pred_meet_prefix_spec(edges, out1, vars, block, upto - 1),
                pred_meet_prefix_spec(edges, out2, vars, block, upto - 1),
                out1[edge.0 as int]@,
                out2[edge.0 as int]@,
            );
        }
    }
}

// r[verify constfold.convergence]
proof fn block_in_env_monotone(program: RawProgram, entry_env: Seq<ConstLat>, out1: Seq<Vec<ConstLat>>, out2: Seq<Vec<ConstLat>>, block: int)
    requires
        valid_constprop_program_spec(program),
        constprop_matrix_shape_spec(out1, program.blocks@.len() as int, entry_env.len() as int),
        constprop_matrix_shape_spec(out2, program.blocks@.len() as int, entry_env.len() as int),
        constprop_matrix_le_spec(out1, out2),
        0 <= block < program.blocks@.len(),
    ensures env_le_spec(
        block_in_env_spec(program, entry_env, out1, block),
        block_in_env_spec(program, entry_env, out2, block),
    )
{
    if block == 0 {
        env_le_refl(entry_env);
    } else {
        pred_meet_prefix_monotone(program.edges@, out1, out2, entry_env.len() as nat, block, program.edges@.len() as int);
    }
}

// r[verify constfold.convergence]
proof fn block_out_env_monotone(program: RawProgram, entry_env: Seq<ConstLat>, out1: Seq<Vec<ConstLat>>, out2: Seq<Vec<ConstLat>>, block: int)
    requires
        valid_constprop_program_spec(program),
        constprop_matrix_shape_spec(out1, program.blocks@.len() as int, entry_env.len() as int),
        constprop_matrix_shape_spec(out2, program.blocks@.len() as int, entry_env.len() as int),
        constprop_matrix_le_spec(out1, out2),
        0 <= block < program.blocks@.len(),
    ensures env_le_spec(
        block_out_env_spec(program, entry_env, out1, block),
        block_out_env_spec(program, entry_env, out2, block),
    )
{
    block_in_env_monotone(program, entry_env, out1, out2, block);
    apply_block_monotone(
        program.blocks@[block].instrs@,
        block_in_env_spec(program, entry_env, out1, block),
        block_in_env_spec(program, entry_env, out2, block),
    );
}

proof fn round_matrix_descends(
    program: RawProgram,
    entry_env: Seq<ConstLat>,
    prev_prev_out: Seq<Vec<ConstLat>>,
    prev_out: Seq<Vec<ConstLat>>,
    next_out: Seq<Vec<ConstLat>>,
)
    requires
        valid_constprop_program_spec(program),
        constprop_matrix_shape_spec(prev_prev_out, program.blocks@.len() as int, entry_env.len() as int),
        constprop_matrix_shape_spec(prev_out, program.blocks@.len() as int, entry_env.len() as int),
        constprop_matrix_shape_spec(next_out, program.blocks@.len() as int, entry_env.len() as int),
        constprop_matrix_le_spec(prev_out, prev_prev_out),
        forall|b: int| #![trigger prev_out[b]]
            0 <= b < program.blocks@.len() ==> prev_out[b]@ == block_out_env_spec(program, entry_env, prev_prev_out, b),
        forall|b: int| #![trigger next_out[b]]
            0 <= b < program.blocks@.len() ==> next_out[b]@ == block_out_env_spec(program, entry_env, prev_out, b),
    ensures constprop_matrix_le_spec(next_out, prev_out)
{
    assert forall|b: int| 0 <= b < program.blocks@.len() implies env_le_spec(next_out[b]@, prev_out[b]@) by {
        assert(prev_out[b] == prev_out[b]);
        assert(next_out[b] == next_out[b]);
        block_out_env_monotone(program, entry_env, prev_out, prev_prev_out, b);
    };
}

proof fn round_matrix_strict_rank_drop(
    program: RawProgram,
    entry_env: Seq<ConstLat>,
    prev_prev_out: Seq<Vec<ConstLat>>,
    prev_out: Seq<Vec<ConstLat>>,
    next_out: Seq<Vec<ConstLat>>,
)
    requires
        valid_constprop_program_spec(program),
        constprop_matrix_shape_spec(prev_prev_out, program.blocks@.len() as int, entry_env.len() as int),
        constprop_matrix_shape_spec(prev_out, program.blocks@.len() as int, entry_env.len() as int),
        constprop_matrix_shape_spec(next_out, program.blocks@.len() as int, entry_env.len() as int),
        constprop_matrix_le_spec(prev_out, prev_prev_out),
        forall|b: int| #![trigger prev_out[b]]
            0 <= b < program.blocks@.len() ==> prev_out[b]@ == block_out_env_spec(program, entry_env, prev_prev_out, b),
        forall|b: int| #![trigger next_out[b]]
            0 <= b < program.blocks@.len() ==> next_out[b]@ == block_out_env_spec(program, entry_env, prev_out, b),
        exists|r: int| 0 <= r < next_out.len() && next_out[r]@ != prev_out[r]@,
    ensures matrix_rank_sum_spec(next_out, 0) < matrix_rank_sum_spec(prev_out, 0)
{
    round_matrix_descends(program, entry_env, prev_prev_out, prev_out, next_out);
    matrix_rank_lt(next_out, prev_out);
}

// r[verify constfold.convergence]
proof fn pred_meet_prefix_top_bounded(edges: Seq<(usize, usize)>, out_env: Seq<Vec<ConstLat>>, vars: nat, block: int, upto: int)
    requires
        0 <= upto <= edges.len(),
        forall|i: int| #![trigger edges[i]] 0 <= i < edges.len() ==> {
            let (src, dst) = edges[i];
            src < out_env.len() && dst < out_env.len()
        },
        forall|i: int| #![trigger out_env[i]] 0 <= i < out_env.len() ==> out_env[i]@.len() == vars,
    ensures env_le_spec(pred_meet_prefix_spec(edges, out_env, vars, block, upto), top_env_spec(vars))
    decreases upto,
{
    if upto <= 0 {
        env_le_refl(top_env_spec(vars));
    } else {
        pred_meet_prefix_top_bounded(edges, out_env, vars, block, upto - 1);
        let edge = edges[upto - 1];
        if edge.1 as int == block {
            pred_meet_prefix_has_len(edges, out_env, vars, block, upto - 1);
            assert forall|i: int| 0 <= i < vars implies #[trigger] lattice_le_spec(meet_env_spec(
                pred_meet_prefix_spec(edges, out_env, vars, block, upto - 1),
                out_env[edge.0 as int]@,
            )[i], top_env_spec(vars)[i]) by {
                assert(top_env_spec(vars)[i] == ConstLat::Top);
                lattice_le_top(meet_env_spec(
                    pred_meet_prefix_spec(edges, out_env, vars, block, upto - 1),
                    out_env[edge.0 as int]@,
                )[i]);
            };
        }
    }
}

// r[verify constfold.convergence]
proof fn block_in_env_top_bounded(program: RawProgram, entry_env: Seq<ConstLat>, out_env: Seq<Vec<ConstLat>>, block: int)
    requires
        valid_constprop_program_spec(program),
        constprop_matrix_shape_spec(out_env, program.blocks@.len() as int, entry_env.len() as int),
        0 <= block < program.blocks@.len(),
    ensures env_le_spec(block_in_env_spec(program, entry_env, out_env, block), top_env_spec(entry_env.len() as nat))
{
    if block == 0 {
        env_le_top(entry_env);
    } else {
        pred_meet_prefix_top_bounded(program.edges@, out_env, entry_env.len() as nat, block, program.edges@.len() as int);
    }
}

// r[verify constfold.convergence]
proof fn block_out_env_top_bounded(program: RawProgram, entry_env: Seq<ConstLat>, out_env: Seq<Vec<ConstLat>>, block: int)
    requires
        valid_constprop_program_spec(program),
        constprop_matrix_shape_spec(out_env, program.blocks@.len() as int, entry_env.len() as int),
        0 <= block < program.blocks@.len(),
    ensures env_le_spec(block_out_env_spec(program, entry_env, out_env, block), top_env_spec(entry_env.len() as nat))
{
    block_in_env_top_bounded(program, entry_env, out_env, block);
    apply_block_top_bounded(program.blocks@[block].instrs@, block_in_env_spec(program, entry_env, out_env, block));
}

// r[verify constfold.convergence]
proof fn all_block_out_env_top_bounded(program: RawProgram, entry_env: Seq<ConstLat>, out_env: Seq<Vec<ConstLat>>)
    requires
        valid_constprop_program_spec(program),
        constprop_matrix_shape_spec(out_env, program.blocks@.len() as int, entry_env.len() as int),
    ensures forall|b: int| 0 <= b < program.blocks@.len() ==> env_le_spec(block_out_env_spec(program, entry_env, out_env, b), top_env_spec(entry_env.len() as nat))
{
    assert forall|b: int| 0 <= b < program.blocks@.len() implies env_le_spec(block_out_env_spec(program, entry_env, out_env, b), top_env_spec(entry_env.len() as nat)) by {
        block_out_env_top_bounded(program, entry_env, out_env, b);
    };
}

// r[impl constfold.monotone]
// r[depends constfold.monotone]
pub closed spec fn transfer_spec(def: Option<u32>, uses: Seq<u32>, opcode: u8, env: Seq<ConstLat>) -> Seq<ConstLat> {
    match def {
        Some(dst) => update_env_spec(env, dst, transfer_value_spec(opcode, uses, env)),
        None => env,
    }
}

proof fn transfer_value_monotone_copy(uses: Seq<u32>, env1: Seq<ConstLat>, env2: Seq<ConstLat>)
    requires
        env_le_spec(env1, env2),
        uses.len() == 1,
    ensures lattice_le_spec(transfer_value_spec(OPCODE_COPY, uses, env1), transfer_value_spec(OPCODE_COPY, uses, env2))
{
    if (uses[0] as int) < env1.len() {
        assert((uses[0] as int) < env2.len());
        assert(lattice_le_spec(env1[uses[0] as int], env2[uses[0] as int]));
    }
}

proof fn transfer_value_monotone_binary(opcode: u8, uses: Seq<u32>, env1: Seq<ConstLat>, env2: Seq<ConstLat>)
    requires
        is_binary_opcode_spec(opcode),
        env_le_spec(env1, env2),
        uses.len() == 2,
    ensures lattice_le_spec(transfer_value_spec(opcode, uses, env1), transfer_value_spec(opcode, uses, env2))
{
    if (uses[0] as int) < env1.len() && (uses[1] as int) < env1.len() {
        assert((uses[0] as int) < env2.len());
        assert((uses[1] as int) < env2.len());
        let a1 = env1[uses[0] as int];
        let a2 = env2[uses[0] as int];
        let b1 = env1[uses[1] as int];
        let b2 = env2[uses[1] as int];
        assert(lattice_le_spec(a1, a2));
        assert(lattice_le_spec(b1, b2));
        match a1 {
            ConstLat::Bottom => {}
            ConstLat::Top => {
                assert(a2 == ConstLat::Top);
                match b1 {
                    ConstLat::Bottom => {}
                    ConstLat::Top => {
                        assert(b2 == ConstLat::Top);
                    }
                    ConstLat::Const(v1) => {
                        match b2 {
                            ConstLat::Top => {}
                            ConstLat::Const(v2) => {
                                assert(v1 == v2);
                            }
                            ConstLat::Bottom => {
                                assert(false);
                            }
                        }
                    }
                }
            }
            ConstLat::Const(x1) => {
                match a2 {
                    ConstLat::Top => {}
                    ConstLat::Const(x2) => {
                        assert(x1 == x2);
                        match b1 {
                            ConstLat::Bottom => {}
                            ConstLat::Top => {
                                assert(b2 == ConstLat::Top);
                            }
                            ConstLat::Const(y1) => {
                                match b2 {
                                    ConstLat::Top => {}
                                    ConstLat::Const(y2) => {
                                        assert(y1 == y2);
                                        assert(transfer_value_spec(opcode, uses, env1) == transfer_value_spec(opcode, uses, env2));
                                    }
                                    ConstLat::Bottom => {
                                        assert(false);
                                    }
                                }
                            }
                        }
                    }
                    ConstLat::Bottom => {
                        assert(false);
                    }
                }
            }
        }
    }
}

proof fn update_env_monotone(env1: Seq<ConstLat>, env2: Seq<ConstLat>, dst: u32, v1: ConstLat, v2: ConstLat)
    requires
        env_le_spec(env1, env2),
        lattice_le_spec(v1, v2),
    ensures env_le_spec(update_env_spec(env1, dst, v1), update_env_spec(env2, dst, v2))
{
    if (dst as int) < env1.len() {
        assert((dst as int) < env2.len());
        assert forall|i: int| 0 <= i < env1.len() implies #[trigger] lattice_le_spec(update_env_spec(env1, dst, v1)[i], update_env_spec(env2, dst, v2)[i]) by {
            if i == dst as int {
                assert(update_env_spec(env1, dst, v1)[i] == v1);
                assert(update_env_spec(env2, dst, v2)[i] == v2);
            } else {
                assert(update_env_spec(env1, dst, v1)[i] == env1[i]);
                assert(update_env_spec(env2, dst, v2)[i] == env2[i]);
            }
        };
    }
}

proof fn update_top_env_is_top(vars: nat, dst: u32)
    ensures update_env_spec(top_env_spec(vars), dst, ConstLat::Top) == top_env_spec(vars)
{
    if (dst as int) < vars {
        assert forall|i: int| 0 <= i < vars implies #[trigger] update_env_spec(top_env_spec(vars), dst, ConstLat::Top)[i] == top_env_spec(vars)[i] by {
            if i == dst as int {
                assert(update_env_spec(top_env_spec(vars), dst, ConstLat::Top)[i] == ConstLat::Top);
            } else {
                assert(update_env_spec(top_env_spec(vars), dst, ConstLat::Top)[i] == top_env_spec(vars)[i]);
            }
        };
    }
}

proof fn transfer_value_le_top(opcode: u8, uses: Seq<u32>, env: Seq<ConstLat>)
    ensures lattice_le_spec(transfer_value_spec(opcode, uses, env), ConstLat::Top)
{
    if opcode == OPCODE_COPY {
        if uses.len() == 1 {
            if (uses[0] as int) < env.len() {
                lattice_le_top(env[uses[0] as int]);
            }
        }
    } else if is_binary_opcode_spec(opcode) {
        if uses.len() == 2 {
            if (uses[0] as int) < env.len() && (uses[1] as int) < env.len() {
                let a = env[uses[0] as int];
                let b = env[uses[1] as int];
                match (a, b) {
                    (ConstLat::Bottom, _) | (_, ConstLat::Bottom) => {}
                    (ConstLat::Top, _) | (_, ConstLat::Top) => {}
                    (ConstLat::Const(x), ConstLat::Const(y)) => {
                        lattice_le_top(fold_expr_spec(opcode, x, y));
                    }
                }
            }
        }
    }
}

proof fn transfer_top_bounded(def: Option<u32>, uses: Seq<u32>, opcode: u8, env: Seq<ConstLat>)
    requires env_le_spec(env, top_env_spec(env.len() as nat))
    ensures env_le_spec(transfer_spec(def, uses, opcode, env), top_env_spec(env.len() as nat))
{
    match def {
        None => {}
        Some(dst) => {
            transfer_value_le_top(opcode, uses, env);
            update_env_monotone(env, top_env_spec(env.len() as nat), dst, transfer_value_spec(opcode, uses, env), ConstLat::Top);
            update_top_env_is_top(env.len() as nat, dst);
        }
    }
}

proof fn apply_block_prefix_top_bounded(instrs: Seq<RawInstr>, env: Seq<ConstLat>, upto: int)
    requires
        env_le_spec(env, top_env_spec(env.len() as nat)),
        0 <= upto <= instrs.len(),
    ensures env_le_spec(apply_block_prefix_spec(instrs, env, upto), top_env_spec(env.len() as nat))
    decreases upto,
{
    if upto <= 0 {
    } else {
        apply_block_prefix_top_bounded(instrs, env, upto - 1);
        transfer_top_bounded(
            instrs[upto - 1].def,
            instrs[upto - 1].uses@,
            instrs[upto - 1].opcode,
            apply_block_prefix_spec(instrs, env, upto - 1),
        );
    }
}

// r[verify constfold.convergence]
proof fn apply_block_top_bounded(instrs: Seq<RawInstr>, env: Seq<ConstLat>)
    requires env_le_spec(env, top_env_spec(env.len() as nat))
    ensures env_le_spec(apply_block_spec(instrs, env), top_env_spec(env.len() as nat))
{
    apply_block_prefix_top_bounded(instrs, env, instrs.len() as int);
}

// r[verify constfold.monotone]
proof fn transfer_monotone(def: Option<u32>, uses: Seq<u32>, opcode: u8, env1: Seq<ConstLat>, env2: Seq<ConstLat>)
    requires env_le_spec(env1, env2)
    ensures env_le_spec(transfer_spec(def, uses, opcode, env1), transfer_spec(def, uses, opcode, env2))
{
    match def {
        None => {}
        Some(dst) => {
            if opcode == OPCODE_COPY {
                if uses.len() == 1 {
                    transfer_value_monotone_copy(uses, env1, env2);
                }
            } else if is_binary_opcode_spec(opcode) {
                if uses.len() == 2 {
                    transfer_value_monotone_binary(opcode, uses, env1, env2);
                }
            }
            update_env_monotone(
                env1,
                env2,
                dst,
                transfer_value_spec(opcode, uses, env1),
                transfer_value_spec(opcode, uses, env2),
            );
        }
    }
}

fn clone_env(env: &[ConstLat]) -> (result: Vec<ConstLat>)
    ensures
        result@.len() == env@.len(),
        forall|i: int| 0 <= i < env@.len() ==> result@[i] == env@[i],
{
    let mut out: Vec<ConstLat> = Vec::new();
    let mut i: usize = 0;
    while i < env.len()
        invariant
            0 <= i <= env@.len(),
            out@.len() == i as int,
            forall|j: int| 0 <= j < i as int ==> out@[j] == env@[j],
        decreases env.len() - i,
    {
        out.push(env[i]);
        i += 1;
    }
    out
}

// r[impl constfold.monotone]
pub fn transfer(instr: &RawInstr, env: &[ConstLat]) -> (result: Vec<ConstLat>)
    ensures result@ == transfer_spec(instr.def, instr.uses@, instr.opcode, env@)
{
    let mut out = clone_env(env);
    match instr.def {
        None => out,
        Some(dst) => {
            let value = if instr.opcode == OPCODE_COPY {
                if instr.uses.len() == 1 {
                    let src = instr.uses[0] as usize;
                    if src < env.len() {
                        env[src]
                    } else {
                        ConstLat::Bottom
                    }
                } else {
                    ConstLat::Bottom
                }
            } else if instr.opcode == OPCODE_ADD
                || instr.opcode == OPCODE_SUB
                || instr.opcode == OPCODE_MUL
                || instr.opcode == OPCODE_DIV
                || instr.opcode == OPCODE_AND
                || instr.opcode == OPCODE_OR
                || instr.opcode == OPCODE_XOR
                || instr.opcode == OPCODE_SHL
                || instr.opcode == OPCODE_SHR
                || instr.opcode == OPCODE_EQ
                || instr.opcode == OPCODE_NE
                || instr.opcode == OPCODE_LT
                || instr.opcode == OPCODE_LE {
                if instr.uses.len() == 2 {
                    let left = instr.uses[0] as usize;
                    let right = instr.uses[1] as usize;
                    if left < env.len() && right < env.len() {
                        match (env[left], env[right]) {
                            (ConstLat::Bottom, _) | (_, ConstLat::Bottom) => ConstLat::Bottom,
                            (ConstLat::Top, _) | (_, ConstLat::Top) => ConstLat::Top,
                            (ConstLat::Const(a), ConstLat::Const(b)) => fold_expr(instr.opcode, a, b),
                        }
                    } else {
                        ConstLat::Bottom
                    }
                } else {
                    ConstLat::Bottom
                }
            } else {
                ConstLat::Bottom
            };
            let idx = dst as usize;
            if idx < out.len() {
                out.set(idx, value);
            }
            out
        }
    }
}

} // verus!

fn clone_const_row(row: &[ConstLat]) -> Vec<ConstLat> {
    row.to_vec()
}

fn top_env(vars: usize) -> Vec<ConstLat> {
    vec![ConstLat::Top; vars]
}

fn top_matrix(rows: usize, vars: usize) -> Vec<Vec<ConstLat>> {
    vec![top_env(vars); rows]
}

fn same_constprop_sets(a: &ConstPropSets, b: &ConstPropSets) -> bool {
    a.in_env == b.in_env && a.out_env == b.out_env
}

fn meet_env(a: &[ConstLat], b: &[ConstLat]) -> Vec<ConstLat> {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).map(|(x, y)| meet(*x, *y)).collect()
}

fn apply_block(instrs: &[RawInstr], in_env: &[ConstLat]) -> Vec<ConstLat> {
    let mut cur = clone_const_row(in_env);
    for instr in instrs {
        cur = transfer(instr, &cur);
    }
    cur
}

fn predecessor_meet_out(edges: &[(usize, usize)], out_env: &[Vec<ConstLat>], entry_env: &[ConstLat], block: usize) -> Vec<ConstLat> {
    let mut acc = top_env(entry_env.len());
    for &(src, dst) in edges {
        if dst == block && src < out_env.len() {
            acc = meet_env(&acc, &out_env[src]);
        }
    }
    acc
}

fn compute_rpo_order(edges: &[(usize, usize)], blocks: usize) -> Vec<usize> {
    fn dfs(node: usize, succs: &[Vec<usize>], visited: &mut [bool], post: &mut Vec<usize>) {
        if visited[node] {
            return;
        }
        visited[node] = true;
        for &succ in &succs[node] {
            dfs(succ, succs, visited, post);
        }
        post.push(node);
    }

    let mut succs = vec![Vec::new(); blocks];
    for &(src, dst) in edges {
        if src < blocks && dst < blocks {
            succs[src].push(dst);
        }
    }

    let mut visited = vec![false; blocks];
    let mut post = Vec::new();
    for start in 0..blocks {
        if !visited[start] {
            dfs(start, &succs, &mut visited, &mut post);
        }
    }
    post.reverse();
    post
}

fn empty_constprop_sets(blocks: usize, vars: usize) -> ConstPropSets {
    ConstPropSets {
        in_env: top_matrix(blocks, vars),
        out_env: top_matrix(blocks, vars),
    }
}

fn constprop_round(program: &RawProgram, entry_env: &[ConstLat], current: &ConstPropSets) -> ConstPropSets {
    let blocks = program.blocks.len();
    let vars = entry_env.len();
    let mut next = empty_constprop_sets(blocks, vars);
    let order = compute_rpo_order(&program.edges, blocks);
    for block in order {
        let in_row = if block == 0 {
            clone_const_row(entry_env)
        } else {
            predecessor_meet_out(&program.edges, &current.out_env, entry_env, block)
        };
        let out_row = apply_block(&program.blocks[block].instrs, &in_row);
        next.in_env[block] = in_row;
        next.out_env[block] = out_row;
    }
    next
}

// r[impl constfold.convergence]
pub fn compute_constprop_with_rounds(program: &RawProgram, entry_env: &[ConstLat]) -> (ConstPropSets, usize) {
    let blocks = program.blocks.len();
    let vars = entry_env.len();
    let product = blocks.checked_mul(vars).unwrap_or(usize::MAX / 2);
    let doubled = product.checked_mul(2).unwrap_or(usize::MAX - 1);
    let budget = doubled.checked_add(1).unwrap_or(usize::MAX);
    let mut state = empty_constprop_sets(blocks, vars);
    let mut rounds = 0usize;
    while rounds < budget {
        let next = constprop_round(program, entry_env, &state);
        rounds += 1;
        let stable = same_constprop_sets(&state, &next);
        state = next;
        if stable {
            break;
        }
    }
    (state, rounds)
}

// r[impl constfold.convergence]
pub fn compute_constprop(program: &RawProgram, entry_env: &[ConstLat]) -> ConstPropSets {
    compute_constprop_with_rounds(program, entry_env).0
}

#[cfg(test)]
fn is_fixed_point(program: &RawProgram, entry_env: &[ConstLat], sets: &ConstPropSets) -> bool {
    let next = constprop_round(program, entry_env, sets);
    same_constprop_sets(sets, &next)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ssa_construction::RawBlock;

    fn raw_instr(def: Option<u32>, uses: &[u32], opcode: u8) -> RawInstr {
        RawInstr {
            def,
            uses: uses.to_vec(),
            opcode,
        }
    }

    fn raw_program(blocks: Vec<Vec<RawInstr>>, edges: &[(usize, usize)]) -> RawProgram {
        RawProgram {
            blocks: blocks.into_iter().map(|instrs| RawBlock { instrs }).collect(),
            edges: edges.to_vec(),
        }
    }

    #[test]
    fn test_lattice_order_and_meet() {
        assert!(lattice_le(ConstLat::Bottom, ConstLat::Const(7)));
        assert!(lattice_le(ConstLat::Const(7), ConstLat::Top));
        assert!(!lattice_le(ConstLat::Const(7), ConstLat::Const(8)));
        assert_eq!(meet(ConstLat::Top, ConstLat::Const(9)), ConstLat::Const(9));
        assert_eq!(meet(ConstLat::Const(9), ConstLat::Top), ConstLat::Const(9));
        assert_eq!(meet(ConstLat::Const(9), ConstLat::Const(9)), ConstLat::Const(9));
        assert_eq!(meet(ConstLat::Const(9), ConstLat::Const(10)), ConstLat::Bottom);
        assert_eq!(meet(ConstLat::Bottom, ConstLat::Top), ConstLat::Bottom);
    }

    #[test]
    fn test_fold_expr_known_ops() {
        assert_eq!(fold_expr(OPCODE_ADD, u64::MAX, 1), ConstLat::Const(0));
        assert_eq!(fold_expr(OPCODE_SUB, 0, 1), ConstLat::Const(u64::MAX));
        assert_eq!(fold_expr(OPCODE_MUL, u64::MAX, 2), ConstLat::Const(u64::MAX.wrapping_mul(2)));
        assert_eq!(fold_expr(OPCODE_DIV, 9, 3), ConstLat::Const(3));
        assert_eq!(fold_expr(OPCODE_DIV, 9, 0), ConstLat::Bottom);
        assert_eq!(fold_expr(OPCODE_AND, 0b1100, 0b1010), ConstLat::Const(0b1000));
        assert_eq!(fold_expr(OPCODE_OR, 0b1100, 0b0011), ConstLat::Const(0b1111));
        assert_eq!(fold_expr(OPCODE_XOR, 0b1100, 0b1010), ConstLat::Const(0b0110));
        assert_eq!(fold_expr(OPCODE_SHL, 1, 4), ConstLat::Const(16));
        assert_eq!(fold_expr(OPCODE_SHL, 1, 64), ConstLat::Bottom);
        assert_eq!(fold_expr(OPCODE_EQ, 7, 7), ConstLat::Const(1));
        assert_eq!(fold_expr(OPCODE_NE, 7, 7), ConstLat::Const(0));
        assert_eq!(fold_expr(OPCODE_LT, 7, 8), ConstLat::Const(1));
        assert_eq!(fold_expr(OPCODE_LE, 8, 8), ConstLat::Const(1));
    }

    #[test]
    fn test_transfer_copy_and_binary() {
        let env = vec![ConstLat::Const(4), ConstLat::Const(5), ConstLat::Top, ConstLat::Bottom];

        let copy = raw_instr(Some(2), &[0], OPCODE_COPY);
        let copy_out = transfer(&copy, &env);
        assert_eq!(copy_out[2], ConstLat::Const(4));
        assert_eq!(copy_out[0], ConstLat::Const(4));
        assert_eq!(copy_out[1], ConstLat::Const(5));

        let add = raw_instr(Some(2), &[0, 1], OPCODE_ADD);
        let add_out = transfer(&add, &env);
        assert_eq!(add_out[2], ConstLat::Const(9));

        let top_add = raw_instr(Some(1), &[0, 2], OPCODE_ADD);
        let top_out = transfer(&top_add, &env);
        assert_eq!(top_out[1], ConstLat::Top);

        let bottom_add = raw_instr(Some(1), &[0, 3], OPCODE_ADD);
        let bottom_out = transfer(&bottom_add, &env);
        assert_eq!(bottom_out[1], ConstLat::Bottom);
    }

    #[test]
    fn test_transfer_no_def_or_malformed() {
        let env = vec![ConstLat::Const(1), ConstLat::Const(2)];

        let nodef = raw_instr(None, &[0, 1], OPCODE_ADD);
        assert_eq!(transfer(&nodef, &env), env);

        let bad_arity = raw_instr(Some(1), &[0], OPCODE_ADD);
        let out = transfer(&bad_arity, &env);
        assert_eq!(out[1], ConstLat::Bottom);

        let bad_use = raw_instr(Some(1), &[0, 9], OPCODE_ADD);
        let out = transfer(&bad_use, &env);
        assert_eq!(out[1], ConstLat::Bottom);
    }

    // r[verify constfold.convergence]
    #[test]
    fn test_constprop_straight_line_reaches_fixed_point() {
        let program = raw_program(
            vec![vec![raw_instr(Some(2), &[0, 1], OPCODE_ADD)]],
            &[],
        );
        let entry_env = vec![ConstLat::Const(4), ConstLat::Const(5), ConstLat::Top];
        let (sets, rounds) = compute_constprop_with_rounds(&program, &entry_env);

        assert!(rounds >= 1);
        assert_eq!(sets.in_env.len(), 1);
        assert_eq!(sets.out_env.len(), 1);
        assert_eq!(sets.in_env[0], entry_env);
        assert_eq!(sets.out_env[0][2], ConstLat::Const(9));
        assert!(is_fixed_point(&program, &entry_env, &sets));
    }

    // r[verify constfold.convergence]
    #[test]
    fn test_constprop_branch_merge_conflict_becomes_bottom() {
        let program = raw_program(
            vec![
                vec![],
                vec![raw_instr(Some(2), &[0], OPCODE_COPY)],
                vec![raw_instr(Some(2), &[1], OPCODE_COPY)],
                vec![],
            ],
            &[(0, 1), (0, 2), (1, 3), (2, 3)],
        );
        let entry_env = vec![ConstLat::Const(4), ConstLat::Const(5), ConstLat::Top];
        let sets = compute_constprop(&program, &entry_env);

        assert_eq!(sets.in_env[0], entry_env);
        assert_eq!(sets.out_env[1][2], ConstLat::Const(4));
        assert_eq!(sets.out_env[2][2], ConstLat::Const(5));
        assert_eq!(sets.in_env[3][2], ConstLat::Bottom);
        assert!(is_fixed_point(&program, &entry_env, &sets));
    }
}

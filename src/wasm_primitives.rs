//! Verified WebAssembly primitive core.
//!
//! This module intentionally lands compact MVP semantic slices: exact value
//! tags, operand-stack discipline for `i32.add`, typed local access,
//! bounds-checked linear memory with page growth, typed control labels for
//! branch/return arity checks, and a small validation-state slice for
//! unreachable-stack polymorphism. The public surface is exec-first;
//! proof-facing specs stay inside this module. It does not model whole module
//! validation, tables, or indirect calls.

#![allow(clippy::too_many_lines)]

use vstd::prelude::*;

verus! {

pub const WASM_PAGE_SIZE_BYTES: usize = 65_536usize;
pub const WASM_MAX_PAGES: usize = 65_536usize;
const I32_BINARY_OPERAND_COUNT: usize = 2usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WasmValueType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WasmValue {
    I32(u32),
    I64(u64),
    F32(u32),
    F64(u64),
}

#[derive(Debug, PartialEq, Eq)]
pub struct LocalFrame {
    pub types: Vec<WasmValueType>,
    pub values: Vec<WasmValue>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LinearMemory {
    pub bytes: Vec<u8>,
    pub pages: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WasmLabelKind {
    Function,
    Block,
    Loop,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WasmControlLabel {
    pub kind: WasmLabelKind,
    pub params: Vec<WasmValueType>,
    pub results: Vec<WasmValueType>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WasmValidationState {
    pub operand_types: Vec<WasmValueType>,
    pub control: Vec<WasmControlLabel>,
    pub reachable: bool,
}

// ============================================================
// Specs
// ============================================================

// r[impl wasm-value.types]
pub open spec fn value_type_spec(value: WasmValue) -> WasmValueType {
    match value {
        WasmValue::I32(_) => WasmValueType::I32,
        WasmValue::I64(_) => WasmValueType::I64,
        WasmValue::F32(_) => WasmValueType::F32,
        WasmValue::F64(_) => WasmValueType::F64,
    }
}

pub open spec fn value_has_type_spec(value: WasmValue, ty: WasmValueType) -> bool {
    match ty {
        WasmValueType::I32 => matches!(value, WasmValue::I32(_)),
        WasmValueType::I64 => matches!(value, WasmValue::I64(_)),
        WasmValueType::F32 => matches!(value, WasmValue::F32(_)),
        WasmValueType::F64 => matches!(value, WasmValue::F64(_)),
    }
}

pub open spec fn typed_values_spec(values: Seq<WasmValue>, types: Seq<WasmValueType>) -> bool {
    &&& values.len() == types.len()
    &&& forall|i: int| #![trigger values[i]] 0 <= i < values.len() ==> value_has_type_spec(values[i], types[i])
}

pub open spec fn value_type_matches_spec(actual: WasmValueType, expected: WasmValueType) -> bool {
    actual == expected
}

pub open spec fn type_stack_suffix_has_types_spec(stack: Seq<WasmValueType>, expected: Seq<WasmValueType>) -> bool {
    &&& stack.len() >= expected.len()
    &&& forall|i: int| #![trigger expected[i]] 0 <= i < expected.len() ==> value_type_matches_spec(stack[stack.len() - expected.len() + i], expected[i])
}

pub open spec fn i32_binary_operand_count_spec() -> int {
    2
}

pub open spec fn wasm_page_size_bytes_spec() -> int {
    65_536
}

pub open spec fn wasm_max_pages_spec() -> int {
    65_536
}

// r[impl wasm-locals.access]
pub open spec fn local_frame_wf_spec(frame: LocalFrame) -> bool {
    typed_values_spec(frame.values@, frame.types@)
}

pub open spec fn i32_binary_ready_spec(stack: Seq<WasmValue>) -> bool {
    &&& stack.len() >= i32_binary_operand_count_spec()
    &&& value_has_type_spec(stack[stack.len() - 1], WasmValueType::I32)
    &&& value_has_type_spec(stack[stack.len() - i32_binary_operand_count_spec()], WasmValueType::I32)
}

pub open spec fn pages_fit_in_usize_spec(pages: int) -> bool {
    &&& 0 <= pages
    &&& pages <= wasm_max_pages_spec()
    &&& pages * wasm_page_size_bytes_spec() <= usize::MAX as int
}

// r[impl wasm-memory.bounds]
// r[impl wasm-memory.grow]
pub open spec fn linear_memory_wf_spec(memory: LinearMemory) -> bool {
    &&& pages_fit_in_usize_spec(memory.pages as int)
    &&& memory.bytes@.len() == memory.pages as int * wasm_page_size_bytes_spec()
}

pub open spec fn memory_window_ok_spec(memory: LinearMemory, addr: int, width: int) -> bool {
    &&& linear_memory_wf_spec(memory)
    &&& 0 <= addr
    &&& 0 <= width
    &&& addr + width <= memory.bytes@.len()
}

pub open spec fn label_is_function_spec(label: WasmControlLabel) -> bool {
    matches!(label.kind, WasmLabelKind::Function)
}

pub open spec fn label_is_loop_spec(label: WasmControlLabel) -> bool {
    matches!(label.kind, WasmLabelKind::Loop)
}

// r[impl wasm-control.labels]
pub open spec fn branch_label_types_spec(label: WasmControlLabel) -> Seq<WasmValueType> {
    match label.kind {
        WasmLabelKind::Function | WasmLabelKind::Block => label.results@,
        WasmLabelKind::Loop => label.params@,
    }
}

pub open spec fn stack_suffix_has_types_spec(stack: Seq<WasmValue>, expected: Seq<WasmValueType>) -> bool {
    &&& stack.len() >= expected.len()
    &&& forall|i: int| #![trigger expected[i]] 0 <= i < expected.len() ==> value_has_type_spec(stack[stack.len() - expected.len() + i], expected[i])
}

// r[impl wasm-control.branch-arity]
pub open spec fn branch_depth_valid_spec(control: Seq<WasmControlLabel>, depth: int) -> bool {
    0 <= depth < control.len()
}

pub open spec fn branch_target_index_spec(control_len: nat, depth: int) -> int
    recommends 0 <= depth < control_len
{
    control_len as int - 1 - depth
}

// r[impl wasm-control.branch-arity]
pub open spec fn branch_values_ready_spec(stack: Seq<WasmValue>, control: Seq<WasmControlLabel>, depth: int) -> bool {
    &&& branch_depth_valid_spec(control, depth)
    &&& stack_suffix_has_types_spec(stack, branch_label_types_spec(control[branch_target_index_spec(control.len(), depth)]))
}

// r[impl wasm-control.return-arity]
pub open spec fn return_values_ready_spec(stack: Seq<WasmValue>, control: Seq<WasmControlLabel>) -> bool {
    &&& control.len() > 0
    &&& label_is_function_spec(control[0])
    &&& stack_suffix_has_types_spec(stack, control[0].results@)
}

// r[impl wasm-validation.unreachable-stack]
pub open spec fn validation_pop_ready_spec(state: WasmValidationState, expected: Seq<WasmValueType>) -> bool {
    !state.reachable || type_stack_suffix_has_types_spec(state.operand_types@, expected)
}

// r[impl wasm-validation.i32-add]
pub open spec fn validation_i32_add_ready_spec(state: WasmValidationState) -> bool {
    &&& (!state.reachable || state.operand_types@.len() >= i32_binary_operand_count_spec())
    &&& (!state.reachable || value_type_matches_spec(state.operand_types@[state.operand_types@.len() - 1], WasmValueType::I32))
    &&& (!state.reachable || value_type_matches_spec(state.operand_types@[state.operand_types@.len() - i32_binary_operand_count_spec()], WasmValueType::I32))
}

// r[impl wasm-validation.control-transfer]
pub open spec fn validation_branch_ready_spec(state: WasmValidationState, depth: int) -> bool {
    &&& branch_depth_valid_spec(state.control@, depth)
    &&& (!state.reachable || type_stack_suffix_has_types_spec(state.operand_types@, branch_label_types_spec(state.control@[branch_target_index_spec(state.control@.len(), depth)])))
}

// r[impl wasm-validation.control-transfer]
pub open spec fn validation_return_ready_spec(state: WasmValidationState) -> bool {
    &&& state.control@.len() > 0
    &&& label_is_function_spec(state.control@[0])
    &&& (!state.reachable || type_stack_suffix_has_types_spec(state.operand_types@, state.control@[0].results@))
}

proof fn typed_values_after_set(values: Seq<WasmValue>, types: Seq<WasmValueType>, index: int, value: WasmValue)
    requires
        typed_values_spec(values, types),
        0 <= index < values.len(),
        value_has_type_spec(value, types[index]),
    ensures
        typed_values_spec(values.update(index, value), types),
{}

proof fn pages_fit_monotone(pages: int, delta_pages: int)
    requires
        pages_fit_in_usize_spec(pages),
        0 <= delta_pages,
        pages + delta_pages <= wasm_max_pages_spec(),
        (pages + delta_pages) * wasm_page_size_bytes_spec() <= usize::MAX as int,
    ensures
        pages_fit_in_usize_spec(pages + delta_pages),
{}

// ============================================================
// Private exec helpers
// ============================================================

fn zeroed_bytes(len: usize) -> (result: Vec<u8>)
    ensures
        result@.len() == len as int,
        forall|i: int| 0 <= i < len as int ==> result@[i] == 0u8,
{
    let mut bytes: Vec<u8> = Vec::new();
    let mut index: usize = 0usize;
    while index < len
        invariant
            0 <= index <= len,
            bytes@.len() == index as int,
            forall|i: int| 0 <= i < index as int ==> bytes@[i] == 0u8,
        decreases len - index,
    {
        bytes.push(0u8);
        index += 1usize;
    }
    bytes
}

fn clone_window(bytes: &[u8], start: usize, width: usize) -> (result: Vec<u8>)
    requires
        start <= usize::MAX - width,
        start + width <= bytes@.len(),
    ensures
        result@ == bytes@.subrange(start as int, (start + width) as int),
{
    let end = start + width;
    let mut cloned: Vec<u8> = Vec::new();
    let mut index: usize = start;
    while index < end
        invariant
            start <= index <= end,
            end <= bytes@.len(),
            cloned@ == bytes@.subrange(start as int, index as int),
        decreases end - index,
    {
        cloned.push(bytes[index]);
        index += 1usize;
    }
    cloned
}

#[allow(clippy::needless_match)]
fn page_byte_len(pages: usize) -> (result: Option<usize>)
    ensures
        result.is_some() <==> pages_fit_in_usize_spec(pages as int),
        result.is_some() ==> result.unwrap() as int == pages as int * wasm_page_size_bytes_spec(),
{
    if pages > WASM_MAX_PAGES {
        return None;
    }
    match pages.checked_mul(WASM_PAGE_SIZE_BYTES) {
        Some(total_bytes) => Some(total_bytes),
        None => None,
    }
}

fn label_is_function(label: &WasmControlLabel) -> (result: bool)
    ensures result == label_is_function_spec(*label)
{
    match label.kind {
        WasmLabelKind::Function => true,
        WasmLabelKind::Block | WasmLabelKind::Loop => false,
    }
}

#[allow(clippy::match_like_matches_macro)]
fn value_type_matches(actual: WasmValueType, expected: WasmValueType) -> (result: bool)
    ensures result == value_type_matches_spec(actual, expected)
{
    match (actual, expected) {
        (WasmValueType::I32, WasmValueType::I32)
        | (WasmValueType::I64, WasmValueType::I64)
        | (WasmValueType::F32, WasmValueType::F32)
        | (WasmValueType::F64, WasmValueType::F64) => true,
        _ => false,
    }
}

fn branch_label_expected_len(label: &WasmControlLabel) -> (result: usize)
    ensures result as int == branch_label_types_spec(*label).len()
{
    match label.kind {
        WasmLabelKind::Function | WasmLabelKind::Block => label.results.len(),
        WasmLabelKind::Loop => label.params.len(),
    }
}

fn branch_label_expected_type_matches(label: &WasmControlLabel, index: usize, value: WasmValue) -> (result: bool)
    requires index < branch_label_types_spec(*label).len()
    ensures result == value_has_type_spec(value, branch_label_types_spec(*label)[index as int])
{
    match label.kind {
        WasmLabelKind::Function | WasmLabelKind::Block => value_has_type(value, label.results[index]),
        WasmLabelKind::Loop => value_has_type(value, label.params[index]),
    }
}

fn stack_suffix_matches_label(stack: &Vec<WasmValue>, label: &WasmControlLabel) -> (result: bool)
    ensures result <==> stack_suffix_has_types_spec(stack@, branch_label_types_spec(*label))
{
    let expected_len = branch_label_expected_len(label);
    if stack.len() < expected_len {
        return false;
    }
    let start = stack.len() - expected_len;
    let mut index: usize = 0usize;
    while index < expected_len
        invariant
            0 <= index <= expected_len,
            expected_len as int == branch_label_types_spec(*label).len(),
            stack@.len() >= expected_len as int,
            start + expected_len == stack.len(),
            forall|i: int| 0 <= i < index as int ==> value_has_type_spec(stack@[start as int + i], branch_label_types_spec(*label)[i]),
        decreases expected_len - index,
    {
        if !branch_label_expected_type_matches(label, index, stack[start + index]) {
            return false;
        }
        index += 1usize;
    }
    true
}

fn branch_label_expected_value_type_matches(label: &WasmControlLabel, index: usize, value_type: WasmValueType) -> (result: bool)
    requires index < branch_label_types_spec(*label).len()
    ensures result == value_type_matches_spec(value_type, branch_label_types_spec(*label)[index as int])
{
    match label.kind {
        WasmLabelKind::Function | WasmLabelKind::Block => value_type_matches(value_type, label.results[index]),
        WasmLabelKind::Loop => value_type_matches(value_type, label.params[index]),
    }
}

fn type_stack_suffix_matches_label(stack: &Vec<WasmValueType>, label: &WasmControlLabel) -> (result: bool)
    ensures result <==> type_stack_suffix_has_types_spec(stack@, branch_label_types_spec(*label))
{
    let expected_len = branch_label_expected_len(label);
    if stack.len() < expected_len {
        return false;
    }
    let start = stack.len() - expected_len;
    let mut index: usize = 0usize;
    while index < expected_len
        invariant
            0 <= index <= expected_len,
            expected_len as int == branch_label_types_spec(*label).len(),
            stack@.len() >= expected_len as int,
            start + expected_len == stack.len(),
            forall|i: int| 0 <= i < index as int ==> value_type_matches_spec(stack@[start as int + i], branch_label_types_spec(*label)[i]),
        decreases expected_len - index,
    {
        if !branch_label_expected_value_type_matches(label, index, stack[start + index]) {
            return false;
        }
        index += 1usize;
    }
    true
}

// ============================================================
// Public exec surface
// ============================================================

// r[impl wasm-value.types]
pub fn value_type(value: WasmValue) -> (result: WasmValueType)
    ensures result == value_type_spec(value)
{
    match value {
        WasmValue::I32(_) => WasmValueType::I32,
        WasmValue::I64(_) => WasmValueType::I64,
        WasmValue::F32(_) => WasmValueType::F32,
        WasmValue::F64(_) => WasmValueType::F64,
    }
}

// r[impl wasm-value.types]
#[allow(clippy::match_like_matches_macro)]
pub fn value_has_type(value: WasmValue, ty: WasmValueType) -> (result: bool)
    ensures result == value_has_type_spec(value, ty)
{
    match (value, ty) {
        (WasmValue::I32(_), WasmValueType::I32)
        | (WasmValue::I64(_), WasmValueType::I64)
        | (WasmValue::F32(_), WasmValueType::F32)
        | (WasmValue::F64(_), WasmValueType::F64) => true,
        _ => false,
    }
}

// r[impl wasm-locals.access]
pub fn new_local_frame(types: Vec<WasmValueType>, values: Vec<WasmValue>) -> (result: Option<LocalFrame>)
    ensures
        result.is_some() <==> typed_values_spec(values@, types@),
        result.is_some() ==> local_frame_wf_spec(result.unwrap()),
        result.is_some() ==> result.unwrap().types@ == types@,
        result.is_some() ==> result.unwrap().values@ == values@,
{
    if types.len() != values.len() {
        return None;
    }
    let mut index: usize = 0usize;
    while index < types.len()
        invariant
            0 <= index <= types@.len(),
            types@.len() == values@.len(),
            forall|i: int| 0 <= i < index as int ==> value_has_type_spec(values@[i], types@[i]),
        decreases types.len() - index,
    {
        if !value_has_type(values[index], types[index]) {
            return None;
        }
        index += 1usize;
    }
    Some(LocalFrame { types, values })
}

// r[impl wasm-locals.access]
pub fn local_get(frame: &LocalFrame, index: usize) -> (result: Option<WasmValue>)
    requires local_frame_wf_spec(*frame)
    ensures
        result.is_some() <==> index < frame.types@.len(),
        result.is_some() ==> result.unwrap() == frame.values@[index as int],
        result.is_some() ==> value_has_type_spec(result.unwrap(), frame.types@[index as int]),
{
    if index >= frame.values.len() {
        return None;
    }
    Some(frame.values[index])
}

// r[impl wasm-locals.access]
pub fn local_set(frame: &mut LocalFrame, index: usize, value: WasmValue) -> (result: bool)
    requires local_frame_wf_spec(*old(frame))
    ensures
        frame.types@ == old(frame).types@,
        result <==> index < old(frame).types@.len() && value_has_type_spec(value, old(frame).types@[index as int]),
        result ==> local_frame_wf_spec(*frame),
        result ==> frame.values@[index as int] == value,
        result ==> forall|i: int| 0 <= i < old(frame).values@.len() && i != index as int ==> frame.values@[i] == old(frame).values@[i],
        !result ==> frame.values@ == old(frame).values@,
{
    if index >= frame.values.len() {
        return false;
    }
    if !value_has_type(value, frame.types[index]) {
        return false;
    }
    frame.values.set(index, value);
    proof {
        typed_values_after_set(old(frame).values@, old(frame).types@, index as int, value);
    }
    true
}

// r[impl wasm-locals.access]
pub fn local_tee(frame: &mut LocalFrame, index: usize, value: WasmValue) -> (result: Option<WasmValue>)
    requires local_frame_wf_spec(*old(frame))
    ensures
        frame.types@ == old(frame).types@,
        result.is_some() <==> index < old(frame).types@.len() && value_has_type_spec(value, old(frame).types@[index as int]),
        result.is_some() ==> result.unwrap() == value,
        result.is_some() ==> local_frame_wf_spec(*frame),
        result.is_some() ==> frame.values@[index as int] == value,
        result.is_some() ==> forall|i: int| 0 <= i < old(frame).values@.len() && i != index as int ==> frame.values@[i] == old(frame).values@[i],
        result.is_none() ==> frame.values@ == old(frame).values@,
{
    if local_set(frame, index, value) {
        Some(value)
    } else {
        None
    }
}

// r[impl wasm-stack.discipline]
pub fn exec_i32_add(stack: &mut Vec<WasmValue>) -> (result: bool)
    ensures
        result <==> i32_binary_ready_spec(old(stack)@),
        result ==> stack@.len() == old(stack)@.len() - 1,
        result ==> forall|i: int| 0 <= i < old(stack)@.len() - i32_binary_operand_count_spec() ==> stack@[i] == old(stack)@[i],
        result ==> value_has_type_spec(stack@[stack@.len() - 1], WasmValueType::I32),
        !result ==> stack@ == old(stack)@,
{
    if stack.len() < I32_BINARY_OPERAND_COUNT {
        return false;
    }
    let rhs = stack[stack.len() - 1usize];
    let lhs = stack[stack.len() - I32_BINARY_OPERAND_COUNT];
    match (lhs, rhs) {
        (WasmValue::I32(lhs_bits), WasmValue::I32(rhs_bits)) => {
            stack.truncate(stack.len() - I32_BINARY_OPERAND_COUNT);
            stack.push(WasmValue::I32(lhs_bits.wrapping_add(rhs_bits)));
            true
        }
        _ => false,
    }
}

// r[impl wasm-control.branch-arity]
pub fn branch_depth_valid(control: &Vec<WasmControlLabel>, depth: usize) -> (result: bool)
    ensures result == branch_depth_valid_spec(control@, depth as int)
{
    depth < control.len()
}

// r[impl wasm-control.branch-arity]
pub fn branch_arity_ok(stack: &Vec<WasmValue>, control: &Vec<WasmControlLabel>, depth: usize) -> (result: bool)
    ensures result <==> branch_values_ready_spec(stack@, control@, depth as int)
{
    if !branch_depth_valid(control, depth) {
        return false;
    }
    let label_index = control.len() - 1usize - depth;
    assert(label_index as int == branch_target_index_spec(control@.len(), depth as int));
    stack_suffix_matches_label(stack, &control[label_index])
}

// r[impl wasm-control.return-arity]
pub fn return_arity_ok(stack: &Vec<WasmValue>, control: &Vec<WasmControlLabel>) -> (result: bool)
    ensures result <==> return_values_ready_spec(stack@, control@)
{
    if control.len() == 0usize {
        return false;
    }
    if !label_is_function(&control[0usize]) {
        return false;
    }
    assert(label_is_function_spec(control@[0]));
    assert(branch_label_types_spec(control@[0]) == control@[0].results@);
    stack_suffix_matches_label(stack, &control[0usize])
}

// r[impl wasm-validation.unreachable-stack]
pub fn validation_pop_ready(state: &WasmValidationState, expected: &Vec<WasmValueType>) -> (result: bool)
    ensures result == validation_pop_ready_spec(*state, expected@)
{
    if !state.reachable {
        return true;
    }
    if expected.len() == 0usize {
        return true;
    }
    let expected_len = expected.len();
    if state.operand_types.len() < expected_len {
        return false;
    }
    let start = state.operand_types.len() - expected_len;
    let mut index: usize = 0usize;
    while index < expected_len
        invariant
            0 <= index <= expected_len,
            expected_len == expected.len(),
            state.reachable,
            state.operand_types@.len() >= expected_len as int,
            start + expected_len == state.operand_types.len(),
            forall|i: int| 0 <= i < index as int ==> value_type_matches_spec(state.operand_types@[start as int + i], expected@[i]),
        decreases expected_len - index,
    {
        if !value_type_matches(state.operand_types[start + index], expected[index]) {
            proof {
                let witness = index as int;
                assert(0 <= witness < expected@.len());
                assert(start as int + witness == state.operand_types@.len() - expected@.len() + witness);
                assert(!value_type_matches_spec(state.operand_types@[start as int + witness], expected@[witness]));
                assert(exists|i: int| 0 <= i < expected@.len() && !value_type_matches_spec(state.operand_types@[state.operand_types@.len() - expected@.len() + i], expected@[i]));
                assert(!type_stack_suffix_has_types_spec(state.operand_types@, expected@));
                assert(state.reachable);
                assert(!validation_pop_ready_spec(*state, expected@));
            }
            return false;
        }
        index += 1usize;
    }
    true
}

// r[impl wasm-validation.control-transfer]
pub fn validate_unreachable(state: &mut WasmValidationState)
    ensures
        !state.reachable,
        state.operand_types@ == old(state).operand_types@,
        state.control@ == old(state).control@,
{
    state.reachable = false;
}

// r[impl wasm-validation.i32-add]
pub fn validate_i32_add(state: &mut WasmValidationState) -> (result: bool)
    ensures
        result <==> validation_i32_add_ready_spec(*old(state)),
        result ==> state.reachable == old(state).reachable,
        result ==> state.control@ == old(state).control@,
        result && old(state).reachable ==> state.operand_types@ == old(state).operand_types@.subrange(0, old(state).operand_types@.len() - i32_binary_operand_count_spec()) + seq![WasmValueType::I32],
        result && !old(state).reachable ==> state.operand_types@ == old(state).operand_types@ + seq![WasmValueType::I32],
        !result ==> state.reachable == old(state).reachable,
        !result ==> state.control@ == old(state).control@,
        !result ==> state.operand_types@ == old(state).operand_types@,
{
    if state.reachable {
        if state.operand_types.len() < I32_BINARY_OPERAND_COUNT {
            return false;
        }
        let rhs_index = state.operand_types.len() - 1usize;
        let lhs_index = state.operand_types.len() - I32_BINARY_OPERAND_COUNT;
        if !value_type_matches(state.operand_types[rhs_index], WasmValueType::I32) {
            return false;
        }
        if !value_type_matches(state.operand_types[lhs_index], WasmValueType::I32) {
            return false;
        }
        state.operand_types.truncate(lhs_index);
    }
    state.operand_types.push(WasmValueType::I32);
    true
}

// r[impl wasm-validation.control-transfer]
pub fn validate_br(state: &mut WasmValidationState, depth: usize) -> (result: bool)
    ensures
        result <==> validation_branch_ready_spec(*old(state), depth as int),
        state.operand_types@ == old(state).operand_types@,
        state.control@ == old(state).control@,
        result ==> !state.reachable,
        !result ==> state.reachable == old(state).reachable,
{
    if !branch_depth_valid(&state.control, depth) {
        return false;
    }
    let label_index = state.control.len() - 1usize - depth;
    assert(label_index as int == branch_target_index_spec(state.control@.len(), depth as int));
    if state.reachable && !type_stack_suffix_matches_label(&state.operand_types, &state.control[label_index]) {
        return false;
    }
    state.reachable = false;
    true
}

// r[impl wasm-validation.control-transfer]
pub fn validate_return(state: &mut WasmValidationState) -> (result: bool)
    ensures
        result <==> validation_return_ready_spec(*old(state)),
        state.operand_types@ == old(state).operand_types@,
        state.control@ == old(state).control@,
        result ==> !state.reachable,
        !result ==> state.reachable == old(state).reachable,
{
    if state.control.len() == 0usize {
        return false;
    }
    if !label_is_function(&state.control[0usize]) {
        return false;
    }
    if state.reachable && !type_stack_suffix_matches_label(&state.operand_types, &state.control[0usize]) {
        return false;
    }
    state.reachable = false;
    true
}

// r[impl wasm-memory.bounds]
// r[impl wasm-memory.grow]
pub fn new_memory(initial_pages: usize) -> (result: Option<LinearMemory>)
    ensures
        result.is_some() <==> pages_fit_in_usize_spec(initial_pages as int),
        result.is_some() ==> linear_memory_wf_spec(result.unwrap()),
        result.is_some() ==> result.unwrap().pages == initial_pages,
        result.is_some() ==> forall|i: int| 0 <= i < result.unwrap().bytes@.len() ==> result.unwrap().bytes@[i] == 0u8,
{
    match page_byte_len(initial_pages) {
        Some(total_bytes) => Some(LinearMemory {
            bytes: zeroed_bytes(total_bytes),
            pages: initial_pages,
        }),
        None => None,
    }
}

// r[impl wasm-memory.bounds]
pub fn memory_load(memory: &LinearMemory, addr: usize, width: usize) -> (result: Option<Vec<u8>>)
    requires linear_memory_wf_spec(*memory)
    ensures
        result.is_some() <==> memory_window_ok_spec(*memory, addr as int, width as int),
        result.is_some() ==> result.unwrap()@ == memory.bytes@.subrange(addr as int, (addr + width) as int),
{
    match addr.checked_add(width) {
        Some(end) if end <= memory.bytes.len() => Some(clone_window(&memory.bytes, addr, width)),
        _ => None,
    }
}

// r[impl wasm-memory.bounds]
pub fn memory_store(memory: &mut LinearMemory, addr: usize, data: &[u8]) -> (result: bool)
    requires linear_memory_wf_spec(*old(memory))
    ensures
        memory.pages == old(memory).pages,
        result <==> memory_window_ok_spec(*old(memory), addr as int, data@.len() as int),
        result ==> linear_memory_wf_spec(*memory),
        result ==> memory.bytes@.subrange(addr as int, (addr + data@.len()) as int) == data@,
        result ==> forall|i: int| 0 <= i < old(memory).bytes@.len() && !(addr as int <= i < (addr + data@.len()) as int) ==> memory.bytes@[i] == old(memory).bytes@[i],
        !result ==> memory.bytes@ == old(memory).bytes@,
{
    match addr.checked_add(data.len()) {
        Some(end) if end <= memory.bytes.len() => {
            let mut data_index: usize = 0usize;
            let mut target_index: usize = addr;
            while target_index < end
                invariant
                    0 <= data_index <= data@.len(),
                    addr <= target_index <= end,
                    target_index == addr + data_index,
                    end == addr + data@.len(),
                    end <= old(memory).bytes@.len(),
                    memory.pages == old(memory).pages,
                    memory.bytes@.len() == old(memory).bytes@.len(),
                    forall|i: int| 0 <= i < data_index as int ==> memory.bytes@[addr as int + i] == data@[i],
                    forall|i: int| 0 <= i < old(memory).bytes@.len() && !(addr as int <= i < addr as int + data_index as int) ==> memory.bytes@[i] == old(memory).bytes@[i],
                decreases end - target_index,
            {
                assert(target_index < memory.bytes.len());
                memory.bytes.set(target_index, data[data_index]);
                assert(memory.bytes@.len() == old(memory).bytes@.len());
                target_index += 1usize;
                data_index += 1usize;
            }
            true
        }
        _ => false,
    }
}

// r[impl wasm-memory.grow]
pub fn memory_grow(memory: &mut LinearMemory, delta_pages: usize) -> (result: Option<usize>)
    requires linear_memory_wf_spec(*old(memory))
    ensures
        result.is_some() <==> pages_fit_in_usize_spec(old(memory).pages as int + delta_pages as int),
        result.is_some() ==> result.unwrap() == old(memory).pages,
        result.is_some() ==> memory.pages == old(memory).pages + delta_pages,
        result.is_some() ==> linear_memory_wf_spec(*memory),
        result.is_some() ==> memory.bytes@.subrange(0, old(memory).bytes@.len() as int) == old(memory).bytes@,
        result.is_some() ==> forall|i: int| old(memory).bytes@.len() <= i < memory.bytes@.len() ==> memory.bytes@[i] == 0u8,
        result.is_none() ==> memory.pages == old(memory).pages,
        result.is_none() ==> memory.bytes@ == old(memory).bytes@,
{
    let previous_pages = memory.pages;
    match previous_pages.checked_add(delta_pages) {
        Some(new_pages) => match page_byte_len(new_pages) {
            Some(total_bytes) => {
                let additional_bytes = total_bytes - memory.bytes.len();
                let mut appended: usize = 0usize;
                while appended < additional_bytes
                    invariant
                        0 <= appended <= additional_bytes,
                        memory.pages == previous_pages,
                        memory.bytes@.subrange(0, old(memory).bytes@.len() as int) == old(memory).bytes@,
                        memory.bytes@.len() == old(memory).bytes@.len() + appended as int,
                        forall|i: int| old(memory).bytes@.len() <= i < memory.bytes@.len() ==> memory.bytes@[i] == 0u8,
                    decreases additional_bytes - appended,
                {
                    memory.bytes.push(0u8);
                    appended += 1usize;
                }
                memory.pages = new_pages;
                proof {
                    pages_fit_monotone(old(memory).pages as int, delta_pages as int);
                }
                Some(previous_pages)
            }
            None => None,
        },
        None => None,
    }
}

// ============================================================
// Proof obligations
// ============================================================

// r[verify wasm-value.types]
proof fn value_type_exact(value: WasmValue)
    ensures value_has_type_spec(value, value_type_spec(value))
{}

// r[verify wasm-stack.discipline]
proof fn i32_add_requires_two_i32s(stack: Seq<WasmValue>)
    ensures
        i32_binary_ready_spec(stack)
            ==> stack.len() >= i32_binary_operand_count_spec()
{}

// r[verify wasm-control.labels]
proof fn loop_branch_uses_params(label: WasmControlLabel)
    ensures label_is_loop_spec(label) ==> branch_label_types_spec(label) == label.params@
{}

// r[verify wasm-control.branch-arity]
proof fn branch_ready_implies_depth_valid(stack: Seq<WasmValue>, control: Seq<WasmControlLabel>, depth: int)
    ensures branch_values_ready_spec(stack, control, depth) ==> branch_depth_valid_spec(control, depth)
{}

// r[verify wasm-control.return-arity]
proof fn return_ready_implies_function_root(stack: Seq<WasmValue>, control: Seq<WasmControlLabel>)
    ensures return_values_ready_spec(stack, control) ==> control.len() > 0 && label_is_function_spec(control[0])
{}

// r[verify wasm-validation.unreachable-stack]
proof fn unreachable_pop_admits_any(state: WasmValidationState, expected: Seq<WasmValueType>)
    requires !state.reachable
    ensures validation_pop_ready_spec(state, expected)
{}

// r[verify wasm-validation.i32-add]
proof fn unreachable_i32_add_ready(state: WasmValidationState)
    requires !state.reachable
    ensures validation_i32_add_ready_spec(state)
{}

// r[verify wasm-validation.control-transfer]
proof fn validation_branch_ready_implies_depth_valid(state: WasmValidationState, depth: int)
    ensures validation_branch_ready_spec(state, depth) ==> branch_depth_valid_spec(state.control@, depth)
{}

// r[verify wasm-validation.control-transfer]
proof fn validation_return_ready_implies_function_root(state: WasmValidationState)
    ensures validation_return_ready_spec(state) ==> state.control@.len() > 0 && label_is_function_spec(state.control@[0])
{}

// r[verify wasm-locals.access]
proof fn typed_local_update_preserves_frame(values: Seq<WasmValue>, types: Seq<WasmValueType>, index: int, value: WasmValue)
    requires
        typed_values_spec(values, types),
        0 <= index < values.len(),
        value_has_type_spec(value, types[index]),
    ensures
        typed_values_spec(values.update(index, value), types)
{
    typed_values_after_set(values, types, index, value);
}

// r[verify wasm-memory.grow]
proof fn memory_grow_preserves_prefix(old_bytes: Seq<u8>, delta_len: int)
    requires
        0 <= delta_len,
    ensures
        (old_bytes + Seq::<u8>::new(delta_len as nat, |i: int| 0u8)).subrange(0, old_bytes.len() as int) == old_bytes
{}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    const TOO_MANY_PAGES: usize = WASM_MAX_PAGES + 1usize;
    const LOCAL_INDEX_ZERO: usize = 0usize;
    const LOCAL_INDEX_ONE: usize = 1usize;
    const MEMORY_INDEX_ZERO: usize = 0usize;
    const GROW_DELTA_ONE: usize = 1usize;
    const I32_ADD_RESULT: u32 = 42u32;
    const STACK_UNDERFLOW_LEN: usize = 1usize;
    const OUT_OF_BOUNDS_OFFSET: usize = 1usize;
    const CONTROL_DEPTH_ZERO: usize = 0usize;
    const CONTROL_DEPTH_ONE: usize = 1usize;
    const OUT_OF_RANGE_BRANCH_DEPTH: usize = 1usize;
    const MAX_PROP_MEMORY_BYTES: usize = 32usize;
    const MAX_PROP_STORE_BYTES: usize = 8usize;

    fn sample_frame() -> LocalFrame {
        new_local_frame(vec![WasmValueType::I32, WasmValueType::F64], vec![WasmValue::I32(7u32), WasmValue::F64(9u64)])
            .expect("sample frame should be well typed")
    }

    fn function_label(results: Vec<WasmValueType>) -> WasmControlLabel {
        WasmControlLabel {
            kind: WasmLabelKind::Function,
            params: Vec::<WasmValueType>::new(),
            results,
        }
    }

    fn block_label(results: Vec<WasmValueType>) -> WasmControlLabel {
        WasmControlLabel {
            kind: WasmLabelKind::Block,
            params: Vec::<WasmValueType>::new(),
            results,
        }
    }

    fn loop_label(params: Vec<WasmValueType>, results: Vec<WasmValueType>) -> WasmControlLabel {
        WasmControlLabel {
            kind: WasmLabelKind::Loop,
            params,
            results,
        }
    }

    fn validation_state(
        operand_types: Vec<WasmValueType>,
        control: Vec<WasmControlLabel>,
        reachable: bool,
    ) -> WasmValidationState {
        WasmValidationState {
            operand_types,
            control,
            reachable,
        }
    }

    // r[verify wasm-value.types]
    #[test]
    fn test_value_type_exact_match_and_mismatch() {
        let value = WasmValue::I32(3u32);
        assert_eq!(value_type(value), WasmValueType::I32);
        assert!(value_has_type(value, WasmValueType::I32));
        assert!(!value_has_type(value, WasmValueType::I64));
    }

    // r[verify wasm-stack.discipline]
    #[test]
    fn test_i32_add_consumes_two_and_pushes_one() {
        let mut stack = vec![WasmValue::I64(9u64), WasmValue::I32(20u32), WasmValue::I32(22u32)];
        assert!(exec_i32_add(&mut stack));
        assert_eq!(stack.len(), I32_BINARY_OPERAND_COUNT);
        assert_eq!(stack[0], WasmValue::I64(9u64));
        assert_eq!(stack[1], WasmValue::I32(I32_ADD_RESULT));
    }

    // r[verify wasm-stack.discipline]
    #[test]
    fn test_i32_add_rejects_underflow_and_type_mismatch() {
        let mut underflow = vec![WasmValue::I32(1u32); STACK_UNDERFLOW_LEN];
        assert!(!exec_i32_add(&mut underflow));
        assert_eq!(underflow, vec![WasmValue::I32(1u32)]);

        let mut mismatch = vec![WasmValue::I32(1u32), WasmValue::I64(2u64)];
        assert!(!exec_i32_add(&mut mismatch));
        assert_eq!(mismatch, vec![WasmValue::I32(1u32), WasmValue::I64(2u64)]);
    }

    // r[verify wasm-control.branch-arity]
    #[test]
    fn test_branch_arity_to_block_uses_result_types() {
        let control = vec![
            function_label(Vec::<WasmValueType>::new()),
            block_label(vec![WasmValueType::I32]),
        ];
        let stack = vec![WasmValue::I64(9u64), WasmValue::I32(7u32)];

        assert!(branch_depth_valid(&control, CONTROL_DEPTH_ZERO));
        assert!(branch_arity_ok(&stack, &control, CONTROL_DEPTH_ZERO));
    }

    // r[verify wasm-control.branch-arity]
    #[test]
    fn test_branch_arity_rejects_bad_depth_and_type_mismatch() {
        let one_label_control = vec![function_label(Vec::<WasmValueType>::new())];
        let stack = vec![WasmValue::I32(7u32)];
        assert!(!branch_depth_valid(&one_label_control, OUT_OF_RANGE_BRANCH_DEPTH));
        assert!(!branch_arity_ok(&stack, &one_label_control, OUT_OF_RANGE_BRANCH_DEPTH));

        let control = vec![
            function_label(Vec::<WasmValueType>::new()),
            block_label(vec![WasmValueType::I32]),
        ];
        let mismatch = vec![WasmValue::I64(9u64)];
        assert!(!branch_arity_ok(&mismatch, &control, CONTROL_DEPTH_ZERO));
    }

    // r[verify wasm-control.labels]
    // r[verify wasm-control.branch-arity]
    #[test]
    fn test_loop_branch_uses_params_not_results() {
        let control = vec![
            function_label(Vec::<WasmValueType>::new()),
            loop_label(vec![WasmValueType::I64], vec![WasmValueType::I32]),
        ];
        let param_stack = vec![WasmValue::I32(1u32), WasmValue::I64(2u64)];
        let result_stack = vec![WasmValue::I32(1u32)];

        assert!(branch_arity_ok(&param_stack, &control, CONTROL_DEPTH_ZERO));
        assert!(!branch_arity_ok(&result_stack, &control, CONTROL_DEPTH_ZERO));
    }

    // r[verify wasm-control.return-arity]
    #[test]
    fn test_return_arity_uses_function_results() {
        let control = vec![
            function_label(vec![WasmValueType::I32, WasmValueType::F64]),
            block_label(vec![WasmValueType::I64]),
        ];
        let stack = vec![WasmValue::I64(1u64), WasmValue::I32(7u32), WasmValue::F64(9u64)];
        assert!(return_arity_ok(&stack, &control));

        let wrong_suffix = vec![WasmValue::I32(7u32), WasmValue::I64(9u64)];
        assert!(!return_arity_ok(&wrong_suffix, &control));
    }

    // r[verify wasm-control.return-arity]
    #[test]
    fn test_return_arity_rejects_missing_function_root() {
        let empty_control = Vec::<WasmControlLabel>::new();
        let stack = vec![WasmValue::I32(7u32)];
        assert!(!return_arity_ok(&stack, &empty_control));

        let non_function_root = vec![block_label(vec![WasmValueType::I32])];
        assert!(!return_arity_ok(&stack, &non_function_root));
    }

    // r[verify wasm-control.branch-arity]
    #[test]
    fn test_outer_branch_depth_targets_function_label() {
        let control = vec![
            function_label(vec![WasmValueType::I64]),
            block_label(vec![WasmValueType::I32]),
        ];
        let stack = vec![WasmValue::I64(9u64)];

        assert!(branch_depth_valid(&control, CONTROL_DEPTH_ONE));
        assert!(branch_arity_ok(&stack, &control, CONTROL_DEPTH_ONE));
        assert!(!branch_arity_ok(&stack, &control, CONTROL_DEPTH_ZERO));
    }

    // r[verify wasm-validation.unreachable-stack]
    #[test]
    fn test_validation_pop_ready_reachable_vs_unreachable() {
        let expected = vec![WasmValueType::I32];
        let reachable =
            validation_state(vec![WasmValueType::I64], vec![function_label(Vec::<WasmValueType>::new())], true);
        assert!(!validation_pop_ready(&reachable, &expected));

        let unreachable =
            validation_state(Vec::<WasmValueType>::new(), vec![function_label(Vec::<WasmValueType>::new())], false);
        assert!(validation_pop_ready(&unreachable, &expected));
    }

    // r[verify wasm-validation.i32-add]
    #[test]
    fn test_validate_i32_add_reachable_consumes_two_and_pushes_one() {
        let mut state = validation_state(
            vec![WasmValueType::I64, WasmValueType::I32, WasmValueType::I32],
            vec![function_label(Vec::<WasmValueType>::new())],
            true,
        );

        assert!(validate_i32_add(&mut state));
        assert!(state.reachable);
        assert_eq!(state.operand_types, vec![WasmValueType::I64, WasmValueType::I32]);
    }

    // r[verify wasm-validation.i32-add]
    #[test]
    fn test_validate_i32_add_rejects_reachable_mismatch() {
        let mut state = validation_state(
            vec![WasmValueType::I32, WasmValueType::I64],
            vec![function_label(Vec::<WasmValueType>::new())],
            true,
        );
        let before = state.operand_types.clone();

        assert!(!validate_i32_add(&mut state));
        assert!(state.reachable);
        assert_eq!(state.operand_types, before);
    }

    // r[verify wasm-validation.i32-add]
    // r[verify wasm-validation.unreachable-stack]
    #[test]
    fn test_validate_i32_add_unreachable_is_stack_polymorphic() {
        let mut state =
            validation_state(Vec::<WasmValueType>::new(), vec![function_label(Vec::<WasmValueType>::new())], false);

        assert!(validate_i32_add(&mut state));
        assert!(!state.reachable);
        assert_eq!(state.operand_types, vec![WasmValueType::I32]);
    }

    // r[verify wasm-validation.control-transfer]
    #[test]
    fn test_validate_unreachable_marks_path_unreachable() {
        let mut state =
            validation_state(vec![WasmValueType::I32], vec![function_label(Vec::<WasmValueType>::new())], true);
        validate_unreachable(&mut state);
        assert!(!state.reachable);
        assert_eq!(state.operand_types, vec![WasmValueType::I32]);
    }

    // r[verify wasm-validation.control-transfer]
    #[test]
    fn test_validate_br_marks_unreachable_and_rejects_bad_depth() {
        let mut state = validation_state(
            vec![WasmValueType::I32],
            vec![
                function_label(Vec::<WasmValueType>::new()),
                block_label(vec![WasmValueType::I32]),
            ],
            true,
        );
        assert!(validate_br(&mut state, CONTROL_DEPTH_ZERO));
        assert!(!state.reachable);
        assert_eq!(state.operand_types, vec![WasmValueType::I32]);

        let mut bad_depth =
            validation_state(vec![WasmValueType::I32], vec![function_label(Vec::<WasmValueType>::new())], true);
        assert!(!validate_br(&mut bad_depth, OUT_OF_RANGE_BRANCH_DEPTH));
        assert!(bad_depth.reachable);
    }

    // r[verify wasm-validation.control-transfer]
    // r[verify wasm-validation.unreachable-stack]
    #[test]
    fn test_validate_br_unreachable_skips_suffix_requirement() {
        let mut state = validation_state(
            Vec::<WasmValueType>::new(),
            vec![
                function_label(Vec::<WasmValueType>::new()),
                block_label(vec![WasmValueType::I32]),
            ],
            false,
        );
        assert!(validate_br(&mut state, CONTROL_DEPTH_ZERO));
        assert!(!state.reachable);
        assert!(state.operand_types.is_empty());
    }

    // r[verify wasm-validation.control-transfer]
    #[test]
    fn test_validate_return_marks_unreachable_and_rejects_bad_roots() {
        let mut state = validation_state(
            vec![WasmValueType::I32, WasmValueType::F64],
            vec![function_label(vec![WasmValueType::I32, WasmValueType::F64])],
            true,
        );
        assert!(validate_return(&mut state));
        assert!(!state.reachable);

        let mut empty_control = validation_state(vec![WasmValueType::I32], Vec::<WasmControlLabel>::new(), true);
        assert!(!validate_return(&mut empty_control));
        assert!(empty_control.reachable);

        let mut non_function_root =
            validation_state(vec![WasmValueType::I32], vec![block_label(vec![WasmValueType::I32])], true);
        assert!(!validate_return(&mut non_function_root));
        assert!(non_function_root.reachable);
    }

    // r[verify wasm-validation.control-transfer]
    #[test]
    fn test_validate_return_rejects_reachable_suffix_mismatch() {
        let mut state =
            validation_state(vec![WasmValueType::I64], vec![function_label(vec![WasmValueType::I32])], true);
        assert!(!validate_return(&mut state));
        assert!(state.reachable);
        assert_eq!(state.operand_types, vec![WasmValueType::I64]);
    }

    // r[verify wasm-locals.access]
    #[test]
    fn test_local_get_set_and_tee() {
        let mut frame = sample_frame();
        assert_eq!(local_get(&frame, LOCAL_INDEX_ZERO), Some(WasmValue::I32(7u32)));
        assert!(local_set(&mut frame, LOCAL_INDEX_ZERO, WasmValue::I32(11u32)));
        assert_eq!(frame.values[LOCAL_INDEX_ZERO], WasmValue::I32(11u32));
        assert_eq!(frame.values[LOCAL_INDEX_ONE], WasmValue::F64(9u64));
        assert_eq!(local_tee(&mut frame, LOCAL_INDEX_ZERO, WasmValue::I32(12u32)), Some(WasmValue::I32(12u32)));
        assert_eq!(frame.values[LOCAL_INDEX_ZERO], WasmValue::I32(12u32));
    }

    // r[verify wasm-locals.access]
    #[test]
    fn test_locals_reject_out_of_bounds_and_type_mismatch() {
        assert!(new_local_frame(vec![WasmValueType::I32], vec![WasmValue::I64(9u64)]).is_none());
        assert!(new_local_frame(vec![WasmValueType::I32], Vec::new()).is_none());

        let mut frame = sample_frame();
        let out_of_bounds_index = frame.types.len();
        assert_eq!(local_get(&frame, out_of_bounds_index), None);
        assert!(!local_set(&mut frame, LOCAL_INDEX_ZERO, WasmValue::I64(9u64)));
        assert_eq!(local_tee(&mut frame, out_of_bounds_index, WasmValue::I32(5u32)), None);
        assert_eq!(frame.values[LOCAL_INDEX_ZERO], WasmValue::I32(7u32));
    }

    // r[verify wasm-memory.bounds]
    #[test]
    fn test_memory_load_store_in_bounds() {
        let mut memory = new_memory(MEMORY_INDEX_ZERO).expect("zero-page memory should be valid");
        let bytes = vec![1u8, 2u8, 3u8, 4u8];
        assert!(!memory_store(&mut memory, MEMORY_INDEX_ZERO, &bytes));

        let mut grown = new_memory(GROW_DELTA_ONE).expect("one-page memory should be valid");
        assert!(memory_store(&mut grown, MEMORY_INDEX_ZERO, &bytes));
        assert_eq!(memory_load(&grown, MEMORY_INDEX_ZERO, bytes.len()), Some(bytes));
    }

    // r[verify wasm-memory.bounds]
    #[test]
    fn test_memory_out_of_bounds_is_noop() {
        let mut memory = new_memory(GROW_DELTA_ONE).expect("one-page memory should be valid");
        let before = memory.bytes.clone();
        let near_end = memory.bytes.len() - OUT_OF_BOUNDS_OFFSET;
        let bytes = vec![9u8, 8u8, 7u8];
        assert_eq!(memory_load(&memory, near_end, bytes.len()), None);
        assert!(!memory_store(&mut memory, near_end, &bytes));
        assert_eq!(memory.bytes, before);
    }

    // r[verify wasm-memory.grow]
    #[test]
    fn test_memory_grow_preserves_prefix() {
        let mut memory = new_memory(GROW_DELTA_ONE).expect("one-page memory should be valid");
        let prefix = vec![5u8, 6u8, 7u8];
        assert!(memory_store(&mut memory, MEMORY_INDEX_ZERO, &prefix));
        let previous_pages = memory_grow(&mut memory, GROW_DELTA_ONE).expect("grow should succeed");
        assert_eq!(previous_pages, GROW_DELTA_ONE);
        assert_eq!(memory.pages, GROW_DELTA_ONE + GROW_DELTA_ONE);
        assert_eq!(memory_load(&memory, MEMORY_INDEX_ZERO, prefix.len()), Some(prefix));
        assert!(memory.bytes[WASM_PAGE_SIZE_BYTES..].iter().all(|byte| *byte == 0u8));
    }

    // r[verify wasm-memory.grow]
    #[test]
    fn test_memory_grow_rejects_over_limit() {
        assert!(new_memory(TOO_MANY_PAGES).is_none());

        let mut memory = new_memory(GROW_DELTA_ONE).expect("one-page memory should be valid");
        let old_bytes = memory.bytes.clone();
        assert_eq!(memory_grow(&mut memory, TOO_MANY_PAGES), None);
        assert_eq!(memory.pages, GROW_DELTA_ONE);
        assert_eq!(memory.bytes, old_bytes);
    }

    // r[verify wasm-memory.grow]
    #[test]
    fn test_memory_grow_zero_delta_and_overflow_delta() {
        let mut memory = new_memory(GROW_DELTA_ONE).expect("one-page memory should be valid");
        let old_bytes = memory.bytes.clone();
        assert_eq!(memory_grow(&mut memory, 0usize), Some(GROW_DELTA_ONE));
        assert_eq!(memory.pages, GROW_DELTA_ONE);
        assert_eq!(memory.bytes, old_bytes);

        let overflow_bytes = memory.bytes.clone();
        assert_eq!(memory_grow(&mut memory, usize::MAX), None);
        assert_eq!(memory.pages, GROW_DELTA_ONE);
        assert_eq!(memory.bytes, overflow_bytes);
    }

    proptest! {
        // r[verify wasm-memory.bounds]
        #[test]
        fn prop_store_then_load_roundtrips(
            start in 0usize..MAX_PROP_MEMORY_BYTES,
            bytes in prop::collection::vec(any::<u8>(), 0usize..=MAX_PROP_STORE_BYTES),
        ) {
            let mut memory = new_memory(GROW_DELTA_ONE).expect("one-page memory should be valid");
            prop_assume!(start + bytes.len() <= memory.bytes.len());
            prop_assert!(memory_store(&mut memory, start, &bytes));
            prop_assert_eq!(memory_load(&memory, start, bytes.len()), Some(bytes.clone()));
        }

        // r[verify wasm-memory.grow]
        #[test]
        fn prop_memory_grow_keeps_old_prefix(prefix in prop::collection::vec(any::<u8>(), 0usize..=MAX_PROP_STORE_BYTES)) {
            let mut memory = new_memory(GROW_DELTA_ONE).expect("one-page memory should be valid");
            prop_assume!(prefix.len() <= memory.bytes.len());
            prop_assert!(memory_store(&mut memory, MEMORY_INDEX_ZERO, &prefix));
            let old_len = memory.bytes.len();
            prop_assert_eq!(memory_grow(&mut memory, GROW_DELTA_ONE), Some(GROW_DELTA_ONE));
            prop_assert_eq!(&memory.bytes[..old_len], &{
                let mut expected = vec![0u8; old_len];
                expected[..prefix.len()].copy_from_slice(&prefix);
                expected
            }[..]);
        }

        // r[verify wasm-memory.bounds]
        #[test]
        fn prop_out_of_bounds_store_preserves_memory(
            trailing_gap in 0usize..=MAX_PROP_STORE_BYTES,
            bytes in prop::collection::vec(any::<u8>(), 1usize..=MAX_PROP_STORE_BYTES),
        ) {
            let mut memory = new_memory(GROW_DELTA_ONE).expect("one-page memory should be valid");
            let start = memory.bytes.len() - trailing_gap;
            prop_assume!(start.checked_add(bytes.len()).map(|end| end > memory.bytes.len()).unwrap_or(true));
            let before = memory.bytes.clone();
            prop_assert!(!memory_store(&mut memory, start, &bytes));
            prop_assert_eq!(memory.bytes, before);
        }
    }
}

//! Finite metadata used by inert attribute admission.

use vstd::prelude::*;

verus! {

pub const TARGET_CRATE_BIT: u16 = 1;
pub const TARGET_FUNCTION_BIT: u16 = 2;
pub const TARGET_STATIC_BIT: u16 = 4;
pub const TARGET_METHOD_BIT: u16 = 8;
pub const TARGET_MODULE_BIT: u16 = 16;
pub const TARGET_FIELD_BIT: u16 = 32;
pub const TARGET_STATEMENT_BIT: u16 = 64;
pub const TARGET_EXPRESSION_BIT: u16 = 128;
pub const TARGET_ALL_MASK: u16 = TARGET_CRATE_BIT
    | TARGET_FUNCTION_BIT
    | TARGET_STATIC_BIT
    | TARGET_METHOD_BIT
    | TARGET_MODULE_BIT
    | TARGET_FIELD_BIT
    | TARGET_STATEMENT_BIT
    | TARGET_EXPRESSION_BIT;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Crate,
    Function,
    Static,
    Method,
    Module,
    Field,
    Statement,
    Expression,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgumentShape {
    Empty,
    Word,
    List,
    NameValue,
    Malformed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserClass {
    NoArgs,
    Single,
    Combine,
    Generic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DuplicateRule {
    Reject,
    KeepFirst,
    KeepLast,
    Combine,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SafetyRule {
    Normal,
    UnsafeWrapper,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StabilityRule {
    Stable,
    FeatureGate { gate_id: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrapperState {
    Normal,
    Unsafe,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Descriptor {
    pub name_id: u64,
    pub class: ParserClass,
    pub template_id: u64,
    pub expected_arguments: ArgumentShape,
    pub allowed_targets: u16,
    pub duplicate_rule: DuplicateRule,
    pub safety_rule: SafetyRule,
    pub stability_rule: StabilityRule,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Occurrence {
    pub name_id: u64,
    pub target: Target,
    pub arguments: ArgumentShape,
    pub argument_value: u64,
    pub count: u32,
    pub wrapper: WrapperState,
    pub feature_gate: Option<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reason {
    Accepted,
    UnknownParser,
    MissingName,
    MissingTemplate,
    InvalidTargetMask,
    MalformedTemplate,
    InvalidClassPolicy,
    DescriptorRejected,
    NameMismatch,
    MissingOccurrence,
    MalformedArguments,
    WrongArguments,
    BadTarget,
    Duplicate,
    MissingUnsafeWrapper,
    MissingFeatureGate,
    WrongFeatureGate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Decision {
    pub reason: Reason,
    pub input_index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParsedValue {
    pub name_id: u64,
    pub arguments: ArgumentShape,
    pub argument_value: u64,
    pub combined_occurrences: u32,
    pub selected_occurrence: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Early,
    Late,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Request {
    pub descriptor: Option<Descriptor>,
    pub occurrence: Occurrence,
    pub input_index: u32,
    pub phase: Phase,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhaseResult {
    pub phase: Phase,
    pub decision: Decision,
    pub value: Option<ParsedValue>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agreement {
    Agreed,
    NotComparable,
    Diverged,
}

} // verus!

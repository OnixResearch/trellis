//! Agreement checks for limited early and later HIR metadata.

use vstd::prelude::*;

use super::model::*;
use super::rules::*;

verus! {

// r[impl hir-attr.early-late-agreement]
pub open spec fn values_equal_spec(left: ParsedValue, right: ParsedValue) -> bool {
    &&& left.name_id == right.name_id
    &&& shapes_equal_spec(left.arguments, right.arguments)
    &&& left.argument_value == right.argument_value
    &&& left.combined_occurrences == right.combined_occurrences
    &&& left.selected_occurrence == right.selected_occurrence
}

// r[impl hir-attr.early-late-agreement]
pub fn values_equal(left: ParsedValue, right: ParsedValue) -> (result: bool)
    ensures result == values_equal_spec(left, right)
{
    left.name_id == right.name_id
        && shapes_equal(left.arguments, right.arguments)
        && left.argument_value == right.argument_value
        && left.combined_occurrences == right.combined_occurrences
        && left.selected_occurrence == right.selected_occurrence
}

// r[impl hir-attr.early-late-agreement]
pub open spec fn phases_in_order_spec(early: Phase, late: Phase) -> bool {
    match (early, late) {
        (Phase::Early, Phase::Late) => true,
        (Phase::Early, Phase::Early)
        | (Phase::Late, Phase::Early)
        | (Phase::Late, Phase::Late) => false,
    }
}

// r[impl hir-attr.early-late-agreement]
pub fn phases_in_order(early: Phase, late: Phase) -> (result: bool)
    ensures result == phases_in_order_spec(early, late)
{
    matches!((early, late), (Phase::Early, Phase::Late))
}

// r[impl hir-attr.early-late-agreement]
pub open spec fn compare_values_spec(
    early: Option<ParsedValue>,
    late: Option<ParsedValue>,
) -> Agreement {
    match (early, late) {
        (Some(left), Some(right)) => {
            if values_equal_spec(left, right) { Agreement::Agreed } else { Agreement::Diverged }
        },
        (None | Some(_), None) | (None, Some(_)) => Agreement::Diverged,
    }
}

// r[impl hir-attr.early-late-agreement]
pub fn compare_values(
    early: Option<ParsedValue>,
    late: Option<ParsedValue>,
) -> (result: Agreement)
    ensures result == compare_values_spec(early, late)
{
    match (early, late) {
        (Some(left), Some(right)) => {
            if values_equal(left, right) { Agreement::Agreed } else { Agreement::Diverged }
        },
        (None | Some(_), None) | (None, Some(_)) => Agreement::Diverged,
    }
}

// r[impl hir-attr.early-late-agreement]
pub open spec fn compare_phases_spec(early: PhaseResult, late: PhaseResult) -> Agreement {
    if !phases_in_order_spec(early.phase, late.phase) {
        Agreement::Diverged
    } else if !decision_is_accepted_spec(early.decision)
        || !decision_is_accepted_spec(late.decision) {
        Agreement::NotComparable
    } else {
        compare_values_spec(early.value, late.value)
    }
}

// r[impl hir-attr.early-late-agreement]
pub fn compare_phases(early: PhaseResult, late: PhaseResult) -> (result: Agreement)
    ensures result == compare_phases_spec(early, late)
{
    if !phases_in_order(early.phase, late.phase) {
        return Agreement::Diverged;
    }
    if !decision_is_accepted(early.decision) || !decision_is_accepted(late.decision) {
        return Agreement::NotComparable;
    }
    compare_values(early.value, late.value)
}

} // verus!

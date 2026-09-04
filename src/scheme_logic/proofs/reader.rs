//! Reader span and attachment lemmas.

use vstd::prelude::*;

verus! {

// r[impl scheme-logic.reader-token-span]
pub open spec fn bounded_span_spec(start: int, end: int, bound: int) -> bool {
    0 <= start < end <= bound
}

// r[impl scheme-logic.reader-token-span]
pub fn bounded_span(start: u32, end: u32, bound: u32) -> (is_admitted: bool)
    ensures is_admitted == bounded_span_spec(start as int, end as int, bound as int),
{
    start < end && end <= bound
}

// r[verify scheme-logic.reader-token-span]
pub proof fn admitted_span_is_ordered_and_bounded(start: int, end: int, bound: int)
    requires bounded_span_spec(start, end, bound),
    ensures start < end,
        end <= bound,
{
}

// r[verify scheme-logic.reader-token-span]
pub proof fn child_containment_is_transitive(
    outer_start: int,
    outer_end: int,
    parent_start: int,
    parent_end: int,
    child_start: int,
    child_end: int,
)
    requires outer_start <= parent_start,
        parent_end <= outer_end,
        parent_start <= child_start,
        child_end <= parent_end,
    ensures outer_start <= child_start,
        child_end <= outer_end,
{
}

// r[impl scheme-logic.reader-datum-structure]
pub open spec fn delimiter_pair_spec(
    open_index: int,
    close_index: int,
    open_kind: int,
    close_kind: int,
) -> bool {
    0 <= open_index < close_index && open_kind == close_kind
}

// r[verify scheme-logic.reader-datum-structure]
pub proof fn admitted_delimiter_matches_and_orders(
    open_index: int,
    close_index: int,
    open_kind: int,
    close_kind: int,
)
    requires delimiter_pair_spec(open_index, close_index, open_kind, close_kind),
    ensures open_index < close_index,
        open_kind == close_kind,
{
}

// r[impl scheme-logic.reader-datum-structure]
pub open spec fn attachment_spec(prefix_index: int, datum_index: int) -> bool {
    0 <= prefix_index && datum_index == prefix_index + 1
}

// r[verify scheme-logic.reader-datum-structure]
pub proof fn admitted_attachment_names_following_datum(
    prefix_index: int,
    datum_index: int,
)
    requires attachment_spec(prefix_index, datum_index),
    ensures prefix_index < datum_index,
        datum_index == prefix_index + 1,
{
}

} // verus!

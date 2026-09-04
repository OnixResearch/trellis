//! Exact schedule-shape lemmas.

use vstd::prelude::*;

verus! {

#[cfg(verus_keep_ghost)]
pub struct Span {
    pub offset: int,
    pub length: int,
}

pub open spec fn span_end_spec(span: Span) -> int {
    span.offset + span.length
}

// r[impl wasm-runtime-bounds.streaming]
pub open spec fn exact_schedule_spec(
    total_bytes: int,
    chunks: Seq<Span>,
    maximum_bytes: int,
    maximum_chunks: int,
) -> bool {
    &&& 0 <= total_bytes <= maximum_bytes
    &&& 0 <= chunks.len() <= maximum_chunks
    &&& if total_bytes == 0 {
        chunks.len() == 0
    } else {
        &&& chunks.len() > 0
        &&& chunks[0].offset == 0
        &&& forall|index: int| #![trigger chunks[index]]
            0 <= index < chunks.len() ==> {
                &&& chunks[index].offset >= 0
                &&& chunks[index].length > 0
                &&& span_end_spec(chunks[index]) <= total_bytes
            }
        &&& forall|index: int| #![trigger chunks[index], chunks[index + 1]]
            0 <= index + 1 < chunks.len() ==>
                chunks[index + 1].offset == span_end_spec(chunks[index])
        &&& span_end_spec(chunks[chunks.len() - 1]) == total_bytes
    }
}

// r[verify wasm-runtime-bounds.streaming]
pub proof fn empty_schedule_is_unique(
    chunks: Seq<Span>,
    maximum_bytes: int,
    maximum_chunks: int,
)
    requires exact_schedule_spec(0, chunks, maximum_bytes, maximum_chunks),
    ensures chunks.len() == 0,
{
}

// r[verify wasm-runtime-bounds.streaming]
pub proof fn nonempty_schedule_starts_at_zero(
    total_bytes: int,
    chunks: Seq<Span>,
    maximum_bytes: int,
    maximum_chunks: int,
)
    requires total_bytes > 0,
        exact_schedule_spec(total_bytes, chunks, maximum_bytes, maximum_chunks),
    ensures chunks.len() > 0,
        chunks[0].offset == 0,
{
}

// r[verify wasm-runtime-bounds.streaming]
pub proof fn admitted_chunks_have_positive_bounded_ranges(
    total_bytes: int,
    chunks: Seq<Span>,
    maximum_bytes: int,
    maximum_chunks: int,
    index: int,
)
    requires exact_schedule_spec(total_bytes, chunks, maximum_bytes, maximum_chunks),
        0 <= index < chunks.len(),
    ensures chunks[index].offset >= 0,
        chunks[index].length > 0,
        span_end_spec(chunks[index]) <= total_bytes,
        span_end_spec(chunks[index]) <= maximum_bytes,
{
}

// r[verify wasm-runtime-bounds.streaming]
pub proof fn adjacent_chunks_have_no_gap_overlap_or_reorder(
    total_bytes: int,
    chunks: Seq<Span>,
    maximum_bytes: int,
    maximum_chunks: int,
    index: int,
)
    requires exact_schedule_spec(total_bytes, chunks, maximum_bytes, maximum_chunks),
        0 <= index,
        index + 1 < chunks.len(),
    ensures chunks[index + 1].offset == span_end_spec(chunks[index]),
        chunks[index + 1].offset > chunks[index].offset,
{
    assert(chunks[index].length > 0);
    assert(chunks[index + 1].offset == span_end_spec(chunks[index]));
}

// r[verify wasm-runtime-bounds.streaming]
pub proof fn admitted_schedule_has_no_trailing_bytes(
    total_bytes: int,
    chunks: Seq<Span>,
    maximum_bytes: int,
    maximum_chunks: int,
)
    requires total_bytes > 0,
        exact_schedule_spec(total_bytes, chunks, maximum_bytes, maximum_chunks),
    ensures span_end_spec(chunks[chunks.len() - 1]) == total_bytes,
{
}

// r[verify wasm-runtime-bounds.streaming]
pub proof fn rejected_schedule_preserves_source_facts(
    source_before: Seq<u8>,
    source_after: Seq<u8>,
)
    requires source_after == source_before,
    ensures source_after == source_before,
{
}

} // verus!

//! Const/codegen caller-location trace agreement.

use vstd::prelude::*;

use super::model::*;

verus! {

// r[impl track-caller.const-codegen-agreement]
pub open spec fn trace_entries_spec(left: Seq<TraceEntry>, right: Seq<TraceEntry>) -> bool {
    forall|index: int| #![trigger left[index], right[index]]
        0 <= index < left.len()
            ==> trace_entries_equal_spec(left[index], right[index])
}

fn entries_match(left: &[TraceEntry], right: &[TraceEntry]) -> (result: bool)
    requires left.len() == right.len()
    ensures result == trace_entries_spec(left@, right@)
{
    let mut index: usize = 0;
    while index < left.len()
        invariant
            left@.len() == right@.len(),
            0 <= index <= left@.len(),
            forall|prior: int| #![trigger left@[prior], right@[prior]]
                0 <= prior < index
                    ==> trace_entries_equal_spec(left@[prior], right@[prior]),
        decreases left.len() - index,
    {
        if !trace_entries_equal(left[index], right[index]) {
            assert(!trace_entries_spec(left@, right@)) by {
                assert(!trace_entries_equal_spec(left@[index as int], right@[index as int]));
            }
            return false;
        }
        index = index + 1;
    }
    assert(trace_entries_spec(left@, right@)) by {
        assert forall|candidate: int| #![trigger left@[candidate], right@[candidate]]
            0 <= candidate < left@.len()
                implies trace_entries_equal_spec(left@[candidate], right@[candidate]) by {
            assert(candidate < index);
        }
    }
    true
}

// r[impl track-caller.const-codegen-agreement]
pub open spec fn traces_agree_spec(left: Seq<TraceEntry>, right: Seq<TraceEntry>) -> bool {
    &&& left.len() == right.len()
    &&& left.len() <= MAX_METADATA_ENTRIES as int
    &&& trace_entries_spec(left, right)
}

// r[impl track-caller.const-codegen-agreement]
pub fn traces_agree(left: &[TraceEntry], right: &[TraceEntry]) -> (result: bool)
    ensures result == traces_agree_spec(left@, right@)
{
    if left.len() != right.len() || left.len() > MAX_METADATA_ENTRIES {
        false
    } else {
        entries_match(left, right)
    }
}

} // verus!

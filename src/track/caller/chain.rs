//! Bounded propagation through a finite tracked-call chain.

use vstd::prelude::*;

use super::model::*;

verus! {

// r[impl track-caller.propagation]
pub open spec fn chain_entry_matches_spec(
    root: Location,
    entry: Option<Location>,
    index: int,
) -> bool {
    let expected = if index == 0 { root } else { forwarded_location_spec(root) };
    match entry {
        Some(actual) => locations_equal_spec(actual, expected),
        None => false,
    }
}

// r[impl track-caller.propagation]
pub open spec fn chain_entries_spec(root: Location, entries: Seq<Option<Location>>) -> bool {
    forall|index: int| #![trigger entries[index]]
        0 <= index < entries.len()
            ==> chain_entry_matches_spec(root, entries[index], index)
}

fn entries_match(root: Location, entries: &[Option<Location>]) -> (result: bool)
    ensures result == chain_entries_spec(root, entries@)
{
    let mut index: usize = 0;
    while index < entries.len()
        invariant
            0 <= index <= entries@.len(),
            forall|prior: int| #![trigger entries@[prior]]
                0 <= prior < index
                    ==> chain_entry_matches_spec(root, entries@[prior], prior),
        decreases entries.len() - index,
    {
        let expected = if index == 0 {
            root
        } else {
            Location { id: root.id, origin: LocationOrigin::Forwarded }
        };
        match entries[index] {
            Some(actual) => {
                if !locations_equal(actual, expected) {
                    assert(!chain_entries_spec(root, entries@)) by {
                        assert(!chain_entry_matches_spec(root, entries@[index as int], index as int));
                    }
                    return false;
                }
            },
            None => {
                assert(!chain_entries_spec(root, entries@)) by {
                    assert(!chain_entry_matches_spec(root, entries@[index as int], index as int));
                }
                return false;
            },
        }
        index = index + 1;
    }
    assert(chain_entries_spec(root, entries@)) by {
        assert forall|candidate: int| #![trigger entries@[candidate]]
            0 <= candidate < entries@.len()
                implies chain_entry_matches_spec(root, entries@[candidate], candidate) by {
            assert(candidate < index);
        }
    }
    true
}

// r[impl track-caller.propagation]
// r[impl track-caller.admission]
pub open spec fn chain_is_valid_spec(root: Location, entries: Seq<Option<Location>>) -> bool {
    &&& root.origin == LocationOrigin::CallSite
    &&& 0 < entries.len() <= MAX_METADATA_ENTRIES as int
    &&& chain_entries_spec(root, entries)
}

// r[impl track-caller.propagation]
// r[impl track-caller.admission]
pub fn is_valid(root: Location, entries: &[Option<Location>]) -> (result: bool)
    ensures result == chain_is_valid_spec(root, entries@)
{
    if !matches!(root.origin, LocationOrigin::CallSite)
        || entries.is_empty()
        || entries.len() > MAX_METADATA_ENTRIES {
        false
    } else {
        entries_match(root, entries)
    }
}

// r[impl track-caller.propagation]
pub open spec fn chain_output_spec(root: Location, entries: Seq<Option<Location>>) -> Option<Location> {
    if !chain_is_valid_spec(root, entries) {
        None
    } else if entries.len() == 1 {
        Some(root)
    } else {
        Some(forwarded_location_spec(root))
    }
}

// r[impl track-caller.propagation]
pub fn output_location(root: Location, entries: &[Option<Location>]) -> (result: Option<Location>)
    ensures result == chain_output_spec(root, entries@)
{
    if !is_valid(root, entries) {
        None
    } else if entries.len() == 1 {
        Some(root)
    } else {
        Some(Location { id: root.id, origin: LocationOrigin::Forwarded })
    }
}

} // verus!

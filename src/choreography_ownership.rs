//! Finite value ownership for role choreographies.
//!
//! This module proves an atomic semantic owner change over supplied finite
//! facts. It does not model transport, persistence, memory, authorization,
//! exactly-once execution, or liveness.
// r[impl choreography.value_ownership.state]
// r[impl choreography.value_ownership.transfer]
// r[impl choreography.value_ownership.conservation]
// r[impl choreography.value_ownership.boundary]
// r[verify choreography.value_ownership.boundary]
// r[impl choreography.value_ownership.evidence]
// r[impl choreography.value_ownership.validation]
// r[verify choreography.value_ownership.validation]

#![allow(dead_code)]

use vstd::prelude::*;

use crate::choreography_global::RoleId;

verus! {

// r[impl choreography.value_ownership.identity]
// r[verify choreography.value_ownership.identity]
/// Identity of one owned choreography value.
///
/// The public field preserves the full `u32` domain. This wrapper supplies
/// type separation only.
///
/// ```compile_fail
/// use verified_logic::choreography_global::RoleId;
/// use verified_logic::choreography_ownership::ValueId;
/// fn needs_value(_: ValueId) {}
/// let role: RoleId = 1;
/// needs_value(role);
/// ```
///
/// ```compile_fail
/// use verified_logic::choreography_global::LabelId;
/// use verified_logic::choreography_ownership::ValueId;
/// fn needs_value(_: ValueId) {}
/// let label: LabelId = 1;
/// needs_value(label);
/// ```
///
/// ```compile_fail
/// use verified_logic::choreography_global::PayloadTag;
/// use verified_logic::choreography_ownership::ValueId;
/// fn needs_value(_: ValueId) {}
/// needs_value(PayloadTag { raw: 1 });
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ValueId {
    pub raw: u32,
}

// r[impl choreography.value_ownership.identity]
pub fn value_ids_equal(left: ValueId, right: ValueId) -> (result: bool)
    ensures result == (left == right)
{
    left.raw == right.raw
}

/// One supplied value-to-owner fact used during ownership admission.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OwnershipEntry {
    pub value: ValueId,
    pub owner: RoleId,
}

/// A normalized ownership state created by [`admit_ownership`].
#[cfg_attr(not(verus_keep_ghost), derive(Clone))]
#[derive(Debug, PartialEq, Eq)]
pub struct OwnershipState {
    pub(crate) entries: Vec<OwnershipEntry>,
}

/// Mathematical entries of one admitted ownership state.
pub closed spec fn ownership_entries_spec(state: OwnershipState) -> Seq<OwnershipEntry> {
    state.entries@
}

// r[impl choreography.value_ownership.state]
pub open spec fn ownership_normalized_spec(entries: Seq<OwnershipEntry>) -> bool {
    forall|index: int| #![trigger entries[index], entries[index + 1]]
        0 <= index && index + 1 < entries.len()
            ==> entries[index].value.raw < entries[index + 1].value.raw
}

pub open spec fn owner_of_from_spec(
    entries: Seq<OwnershipEntry>,
    value: ValueId,
    from: int,
) -> Option<RoleId>
    decreases if from < entries.len() { entries.len() - from } else { 0 }
{
    if from < 0 || from >= entries.len() {
        None
    } else if entries[from].value == value {
        Some(entries[from].owner)
    } else {
        owner_of_from_spec(entries, value, from + 1)
    }
}

pub open spec fn owner_of_spec(entries: Seq<OwnershipEntry>, value: ValueId) -> Option<RoleId> {
    owner_of_from_spec(entries, value, 0)
}

// r[impl choreography.value_ownership.state]
pub fn ownership_is_normalized(entries: &Vec<OwnershipEntry>) -> (result: bool)
    ensures result == ownership_normalized_spec(entries@)
{
    let mut index: usize = 0;
    while index < entries.len()
        invariant
            0 <= index <= entries.len(),
            forall|prior: int| #![trigger entries@[prior], entries@[prior + 1]]
                0 <= prior && prior + 1 < index
                    ==> entries@[prior].value.raw < entries@[prior + 1].value.raw,
        decreases entries.len() - index,
    {
        if index > 0 && entries[index - 1].value.raw >= entries[index].value.raw {
            assert(!ownership_normalized_spec(entries@)) by {
                let bad = index as int - 1;
                assert(0 <= bad);
                assert(bad + 1 < entries@.len());
                assert(entries@[bad].value.raw >= entries@[bad + 1].value.raw);
                assert(!(forall|candidate: int| #![trigger entries@[candidate], entries@[candidate + 1]]
                    0 <= candidate && candidate + 1 < entries@.len()
                        ==> entries@[candidate].value.raw < entries@[candidate + 1].value.raw)) by {
                    assert(!(entries@[bad].value.raw < entries@[bad + 1].value.raw));
                }
            }
            return false;
        }
        index = index + 1;
    }
    true
}

fn clone_ownership_entries(entries: &Vec<OwnershipEntry>) -> (result: Vec<OwnershipEntry>)
    ensures result@ == entries@
{
    let mut result = Vec::with_capacity(entries.len());
    let mut index: usize = 0;
    while index < entries.len()
        invariant
            0 <= index <= entries.len(),
            result@ == entries@.subrange(0, index as int),
        decreases entries.len() - index,
    {
        result.push(entries[index]);
        index = index + 1;
    }
    result
}

// r[impl choreography.value_ownership.state]
/// Admit entries only when their value identities are in strict order.
pub fn admit_ownership(entries: &Vec<OwnershipEntry>) -> (result: Option<OwnershipState>)
    ensures
        result.is_some() == ownership_normalized_spec(entries@),
        result.is_some() ==> ownership_entries_spec(result.unwrap()) == entries@,
{
    if !ownership_is_normalized(entries) {
        None
    } else {
        Some(OwnershipState { entries: clone_ownership_entries(entries) })
    }
}

fn owner_of_from(
    entries: &Vec<OwnershipEntry>,
    value: ValueId,
    from: usize,
) -> (result: Option<RoleId>)
    requires from <= entries.len()
    ensures result == owner_of_from_spec(entries@, value, from as int)
    decreases entries.len() - from
{
    if from >= entries.len() {
        None
    } else if value_ids_equal(entries[from].value, value) {
        Some(entries[from].owner)
    } else {
        owner_of_from(entries, value, from + 1)
    }
}

// r[impl choreography.value_ownership.state]
/// Return the number of represented values in an admitted state.
pub fn ownership_entry_count(state: &OwnershipState) -> (result: usize)
    ensures result == ownership_entries_spec(*state).len()
{
    state.entries.len()
}

// r[impl choreography.value_ownership.state]
/// Return the owner of one represented value, or `None` when it is absent.
pub fn owner_of(state: &OwnershipState, value: ValueId) -> (result: Option<RoleId>)
    ensures result == owner_of_spec(ownership_entries_spec(*state), value)
{
    owner_of_from(&state.entries, value, 0)
}

fn entries_with_new_owner(
    entries: &Vec<OwnershipEntry>,
    value: ValueId,
    destination: RoleId,
) -> (result: Vec<OwnershipEntry>)
    ensures
        result@.len() == entries@.len(),
        forall|index: int| #![trigger result@[index], entries@[index]]
            0 <= index < entries@.len() ==> result@[index].value == entries@[index].value,
        forall|index: int| #![trigger result@[index], entries@[index]]
            0 <= index < entries@.len() ==> result@[index].owner
                == if entries@[index].value == value { destination } else { entries@[index].owner },
{
    let mut result: Vec<OwnershipEntry> = Vec::with_capacity(entries.len());
    let mut index: usize = 0;
    while index < entries.len()
        invariant
            0 <= index <= entries.len(),
            result@.len() == index,
            forall|prior: int| #![trigger result@[prior], entries@[prior]]
                0 <= prior < index ==> result@[prior].value == entries@[prior].value,
            forall|prior: int| #![trigger result@[prior], entries@[prior]]
                0 <= prior < index ==> result@[prior].owner
                    == if entries@[prior].value == value { destination } else { entries@[prior].owner },
        decreases entries.len() - index,
    {
        let entry = entries[index];
        let owner = if value_ids_equal(entry.value, value) {
            destination
        } else {
            entry.owner
        };
        result.push(OwnershipEntry { value: entry.value, owner });
        index = index + 1;
    }
    result
}

pub closed spec fn transfer_owner_post_spec(
    old_entries: Seq<OwnershipEntry>,
    source: RoleId,
    destination: RoleId,
    value: ValueId,
    result: Option<OwnershipState>,
) -> bool {
    result.is_some() == (source != destination && owner_of_spec(old_entries, value) == Some(source))
        && (result.is_some() ==> ({
            let new_entries = ownership_entries_spec(result.unwrap());
            new_entries.len() == old_entries.len()
                && forall|index: int| #![trigger new_entries[index], old_entries[index]]
                    0 <= index < old_entries.len() ==> new_entries[index].value == old_entries[index].value
                && forall|index: int| #![trigger new_entries[index], old_entries[index]]
                    0 <= index < old_entries.len() ==> new_entries[index].owner
                        == if old_entries[index].value == value { destination } else { old_entries[index].owner }
        }))
}

// r[impl choreography.value_ownership.transfer]
// r[verify choreography.value_ownership.transfer]
// r[verify choreography.value_ownership.conservation]
/// Atomically replace one current owner when the transfer is admissible.
///
/// Failure returns `None` and leaves the caller-held state unchanged.
pub fn transfer_owner(
    state: &OwnershipState,
    source: RoleId,
    destination: RoleId,
    value: ValueId,
) -> (result: Option<OwnershipState>)
    ensures transfer_owner_post_spec(
        ownership_entries_spec(*state),
        source,
        destination,
        value,
        result,
    )
{
    if source == destination {
        return None;
    }
    match owner_of(state, value) {
        Some(owner) => {
            if owner != source {
                None
            } else {
                Some(OwnershipState {
                    entries: entries_with_new_owner(&state.entries, value, destination),
                })
            }
        }
        None => None,
    }
}

proof fn selected_owner_lookup_after_update(
    old_entries: Seq<OwnershipEntry>,
    new_entries: Seq<OwnershipEntry>,
    selected: ValueId,
    destination: RoleId,
    from: int,
)
    requires
        0 <= from <= old_entries.len(),
        new_entries.len() == old_entries.len(),
        forall|index: int| #![trigger new_entries[index], old_entries[index]]
            0 <= index < old_entries.len() ==> new_entries[index].value == old_entries[index].value,
        forall|index: int| #![trigger new_entries[index], old_entries[index]]
            0 <= index < old_entries.len() ==> new_entries[index].owner
                == if old_entries[index].value == selected { destination } else { old_entries[index].owner },
    ensures owner_of_from_spec(new_entries, selected, from) == match owner_of_from_spec(
        old_entries,
        selected,
        from,
    ) {
        Some(_) => Some(destination),
        None => None,
    }
    decreases old_entries.len() - from
{
    if from < old_entries.len() {
        assert(new_entries[from].value == old_entries[from].value);
        if old_entries[from].value == selected {
            assert(new_entries[from].owner == destination);
        } else {
            selected_owner_lookup_after_update(
                old_entries,
                new_entries,
                selected,
                destination,
                from + 1,
            );
        }
    }
}

proof fn unrelated_owner_lookup_after_update(
    old_entries: Seq<OwnershipEntry>,
    new_entries: Seq<OwnershipEntry>,
    selected: ValueId,
    unrelated: ValueId,
    destination: RoleId,
    from: int,
)
    requires
        unrelated != selected,
        0 <= from <= old_entries.len(),
        new_entries.len() == old_entries.len(),
        forall|index: int| #![trigger new_entries[index], old_entries[index]]
            0 <= index < old_entries.len() ==> new_entries[index].value == old_entries[index].value,
        forall|index: int| #![trigger new_entries[index], old_entries[index]]
            0 <= index < old_entries.len() ==> new_entries[index].owner
                == if old_entries[index].value == selected { destination } else { old_entries[index].owner },
    ensures owner_of_from_spec(new_entries, unrelated, from)
        == owner_of_from_spec(old_entries, unrelated, from)
    decreases old_entries.len() - from
{
    if from < old_entries.len() {
        assert(new_entries[from].value == old_entries[from].value);
        if old_entries[from].value == unrelated {
            assert(old_entries[from].value != selected);
            assert(new_entries[from].owner == old_entries[from].owner);
        } else {
            unrelated_owner_lookup_after_update(
                old_entries,
                new_entries,
                selected,
                unrelated,
                destination,
                from + 1,
            );
        }
    }
}

// r[verify choreography.value_ownership.state]
// r[verify choreography.value_ownership.conservation]
pub proof fn owner_update_preserves_normalization(
    old_entries: Seq<OwnershipEntry>,
    new_entries: Seq<OwnershipEntry>,
)
    requires
        ownership_normalized_spec(old_entries),
        new_entries.len() == old_entries.len(),
        forall|index: int| #![trigger new_entries[index], old_entries[index]]
            0 <= index < old_entries.len() ==> new_entries[index].value == old_entries[index].value,
    ensures ownership_normalized_spec(new_entries)
{
    assert forall|index: int| #![trigger new_entries[index], new_entries[index + 1]]
        0 <= index && index + 1 < new_entries.len()
            implies new_entries[index].value.raw < new_entries[index + 1].value.raw
    by {
        assert(new_entries[index].value == old_entries[index].value);
        assert(new_entries[index + 1].value == old_entries[index + 1].value);
        assert(old_entries[index].value.raw < old_entries[index + 1].value.raw);
    }
}

// r[verify choreography.value_ownership.conservation]
pub proof fn unrelated_owner_is_preserved(
    old_entries: Seq<OwnershipEntry>,
    new_entries: Seq<OwnershipEntry>,
    selected: ValueId,
    unrelated: ValueId,
    destination: RoleId,
    index: int,
)
    requires
        0 <= index < old_entries.len(),
        new_entries.len() == old_entries.len(),
        old_entries[index].value == unrelated,
        unrelated != selected,
        forall|position: int| #![trigger new_entries[position], old_entries[position]]
            0 <= position < old_entries.len() ==> new_entries[position].owner
                == if old_entries[position].value == selected { destination } else { old_entries[position].owner },
    ensures new_entries[index].owner == old_entries[index].owner
{
}

// r[verify choreography.value_ownership.conservation]
pub proof fn successful_transfer_conserves_ownership(
    old_entries: Seq<OwnershipEntry>,
    source: RoleId,
    destination: RoleId,
    selected: ValueId,
    moved: OwnershipState,
)
    requires
        ownership_normalized_spec(old_entries),
        transfer_owner_post_spec(old_entries, source, destination, selected, Some(moved)),
    ensures
        source != destination,
        ownership_entries_spec(moved).len() == old_entries.len(),
        ownership_normalized_spec(ownership_entries_spec(moved)),
        owner_of_spec(ownership_entries_spec(moved), selected) == Some(destination),
        owner_of_spec(ownership_entries_spec(moved), selected) != Some(source),
        forall|unrelated: ValueId| unrelated != selected
            ==> owner_of_spec(ownership_entries_spec(moved), unrelated)
                == owner_of_spec(old_entries, unrelated),
{
    reveal(transfer_owner_post_spec);
    owner_update_preserves_normalization(old_entries, ownership_entries_spec(moved));
    selected_owner_lookup_after_update(
        old_entries,
        ownership_entries_spec(moved),
        selected,
        destination,
        0,
    );
    assert forall|unrelated: ValueId| unrelated != selected
        implies owner_of_spec(ownership_entries_spec(moved), unrelated)
            == owner_of_spec(old_entries, unrelated)
    by {
        unrelated_owner_lookup_after_update(
            old_entries,
            ownership_entries_spec(moved),
            selected,
            unrelated,
            destination,
            0,
        );
    }
}

// r[verify choreography.value_ownership.transfer]
// r[impl properties.choreography_transfer_exclusion]
// r[verify properties.choreography_transfer_exclusion]
pub proof fn successful_transfer_rejects_stale_source(
    old_entries: Seq<OwnershipEntry>,
    source: RoleId,
    destination: RoleId,
    selected: ValueId,
    moved: OwnershipState,
    later_destination: RoleId,
)
    requires
        ownership_normalized_spec(old_entries),
        transfer_owner_post_spec(old_entries, source, destination, selected, Some(moved)),
    ensures transfer_owner_post_spec(
        ownership_entries_spec(moved),
        source,
        later_destination,
        selected,
        None,
    )
{
    successful_transfer_conserves_ownership(
        old_entries,
        source,
        destination,
        selected,
        moved,
    );
    reveal(transfer_owner_post_spec);
}

} // verus!

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    const FIRST_VALUE: u32 = 10;
    const SECOND_VALUE: u32 = 20;
    const SOURCE_ROLE: RoleId = 1;
    const DESTINATION_ROLE: RoleId = 2;
    const THIRD_ROLE: RoleId = 3;
    const MIN_OWNER: RoleId = 1;
    const MAX_OWNER_EXCLUSIVE: RoleId = 8;
    const MAX_GENERATED_ENTRIES: usize = 16;

    fn value(raw: u32) -> ValueId {
        ValueId { raw }
    }

    fn entry(raw: u32, owner: RoleId) -> OwnershipEntry {
        OwnershipEntry {
            value: value(raw),
            owner,
        }
    }

    fn admitted(entries: Vec<OwnershipEntry>) -> OwnershipState {
        match admit_ownership(&entries) {
            Some(state) => state,
            None => panic!("test fixture must be normalized"),
        }
    }

    // r[verify choreography.value_ownership.evidence]
    #[test]
    fn admits_empty_ownership() {
        let entries = Vec::new();
        let state = admitted(entries.clone());
        assert_eq!(state.entries, entries);
    }

    // r[verify choreography.value_ownership.evidence]
    #[test]
    fn admits_normalized_unique_ownership() {
        let entries = vec![entry(FIRST_VALUE, SOURCE_ROLE), entry(SECOND_VALUE, THIRD_ROLE)];
        let state = admitted(entries.clone());
        assert_eq!(state.entries, entries);
    }

    // r[verify choreography.value_ownership.evidence]
    #[test]
    fn rejects_duplicate_and_out_of_order_values() {
        let duplicate = vec![entry(FIRST_VALUE, SOURCE_ROLE), entry(FIRST_VALUE, DESTINATION_ROLE)];
        let out_of_order = vec![entry(SECOND_VALUE, SOURCE_ROLE), entry(FIRST_VALUE, DESTINATION_ROLE)];
        assert_eq!(admit_ownership(&duplicate), None);
        assert_eq!(admit_ownership(&out_of_order), None);
    }

    // r[verify choreography.value_ownership.evidence]
    #[test]
    fn transfer_changes_only_selected_owner() {
        let state = admitted(vec![entry(FIRST_VALUE, SOURCE_ROLE), entry(SECOND_VALUE, THIRD_ROLE)]);
        let result = transfer_owner(&state, SOURCE_ROLE, DESTINATION_ROLE, value(FIRST_VALUE));
        match result {
            Some(next) => {
                assert_eq!(owner_of(&next, value(FIRST_VALUE)), Some(DESTINATION_ROLE));
                assert_eq!(owner_of(&next, value(SECOND_VALUE)), Some(THIRD_ROLE));
                assert_eq!(ownership_entry_count(&next), ownership_entry_count(&state));
            }
            None => panic!("current owner transfer must succeed"),
        }
    }

    // r[verify choreography.value_ownership.evidence]
    #[test]
    fn rejects_missing_self_wrong_and_stale_sources() {
        let state = admitted(vec![entry(FIRST_VALUE, SOURCE_ROLE)]);
        assert_eq!(transfer_owner(&state, SOURCE_ROLE, SOURCE_ROLE, value(FIRST_VALUE)), None,);
        assert_eq!(transfer_owner(&state, DESTINATION_ROLE, THIRD_ROLE, value(FIRST_VALUE)), None,);
        assert_eq!(transfer_owner(&state, SOURCE_ROLE, DESTINATION_ROLE, value(SECOND_VALUE)), None,);
        let moved = match transfer_owner(&state, SOURCE_ROLE, DESTINATION_ROLE, value(FIRST_VALUE)) {
            Some(next) => next,
            None => panic!("first transfer must succeed"),
        };
        assert_eq!(transfer_owner(&moved, SOURCE_ROLE, THIRD_ROLE, value(FIRST_VALUE)), None,);
    }

    // r[verify choreography.value_ownership.evidence]
    #[test]
    fn supports_chained_and_return_transfers() {
        let initial = admitted(vec![entry(FIRST_VALUE, SOURCE_ROLE)]);
        let moved = match transfer_owner(&initial, SOURCE_ROLE, DESTINATION_ROLE, value(FIRST_VALUE)) {
            Some(next) => next,
            None => panic!("first transfer must succeed"),
        };
        let chained = match transfer_owner(&moved, DESTINATION_ROLE, THIRD_ROLE, value(FIRST_VALUE)) {
            Some(next) => next,
            None => panic!("chained transfer must succeed"),
        };
        let returned = match transfer_owner(&chained, THIRD_ROLE, SOURCE_ROLE, value(FIRST_VALUE)) {
            Some(next) => next,
            None => panic!("return transfer must succeed"),
        };
        assert_eq!(owner_of(&returned, value(FIRST_VALUE)), Some(SOURCE_ROLE));
    }

    proptest! {
        // r[verify choreography.value_ownership.evidence]
        #[test]
        fn prop_transfer_matches_pointwise_oracle(
            owners in proptest::collection::vec(
                MIN_OWNER..MAX_OWNER_EXCLUSIVE,
                1..=MAX_GENERATED_ENTRIES,
            ),
            selected_seed in any::<usize>(),
        ) {
            let selected_index = selected_seed % owners.len();
            let source = owners[selected_index];
            let destination = if source == DESTINATION_ROLE {
                THIRD_ROLE
            } else {
                DESTINATION_ROLE
            };
            let entries = owners
                .iter()
                .enumerate()
                .map(|(index, owner)| {
                    let raw = match u32::try_from(index) {
                        Ok(value) => value,
                        Err(_) => FIRST_VALUE,
                    };
                    entry(raw, *owner)
                })
                .collect::<Vec<_>>();
            let state = admitted(entries.clone());
            let selected_raw = match u32::try_from(selected_index) {
                Ok(value) => value,
                Err(_) => FIRST_VALUE,
            };
            let result = transfer_owner(
                &state,
                source,
                destination,
                value(selected_raw),
            );
            match result {
                Some(next) => {
                    let expected = entries
                        .iter()
                        .map(|old| OwnershipEntry {
                            value: old.value,
                            owner: if old.value.raw == selected_raw {
                                destination
                            } else {
                                old.owner
                            },
                        })
                        .collect::<Vec<_>>();
                    prop_assert_eq!(next.entries, expected);
                }
                None => prop_assert!(false, "admitted current-owner transfer failed"),
            }
        }
    }
}

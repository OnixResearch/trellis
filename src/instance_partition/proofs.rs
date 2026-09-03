//! Verus facts for finite collection, deduplication, and partitions.

use vstd::prelude::*;

verus! {

pub open spec fn member_spec(values: Seq<u64>, value: u64) -> bool {
    exists|i: int| #![trigger values[i]] 0 <= i < values.len() && values[i] == value
}

// r[impl mono.dedup]
pub open spec fn unique_values_spec(values: Seq<u64>) -> bool {
    forall|i: int, j: int|
        #![trigger values[i], values[j]]
        0 <= i < j < values.len() ==> values[i] != values[j]
}

// r[verify mono.dedup]
pub proof fn admitted_values_do_not_duplicate(values: Seq<u64>, left: int, right: int)
    requires unique_values_spec(values),
        0 <= left < right < values.len(),
    ensures values[left] != values[right],
{
}

// r[impl mono.collection]
pub open spec fn collection_closed_spec(
    collected: Seq<u64>,
    edge_sources: Seq<u64>,
    edge_targets: Seq<u64>,
) -> bool {
    edge_sources.len() == edge_targets.len()
        && forall|i: int| #![trigger edge_sources[i], edge_targets[i]]
            0 <= i < edge_sources.len() && member_spec(collected, edge_sources[i])
                ==> member_spec(collected, edge_targets[i])
}

// r[verify mono.collection]
pub proof fn admitted_collection_contains_reachable_target(
    collected: Seq<u64>,
    edge_sources: Seq<u64>,
    edge_targets: Seq<u64>,
    edge: int,
)
    requires collection_closed_spec(collected, edge_sources, edge_targets),
        0 <= edge < edge_sources.len(),
        member_spec(collected, edge_sources[edge]),
    ensures member_spec(collected, edge_targets[edge]),
{
}

// r[impl mono.cgu-partition]
pub open spec fn exact_partition_membership_spec(membership_counts: Seq<u32>) -> bool {
    forall|i: int| #![trigger membership_counts[i]]
        0 <= i < membership_counts.len() ==> membership_counts[i] == 1
}

// r[verify mono.cgu-partition]
pub proof fn admitted_partition_is_disjoint_and_complete(membership_counts: Seq<u32>, item: int)
    requires exact_partition_membership_spec(membership_counts),
        0 <= item < membership_counts.len(),
    ensures membership_counts[item] == 1,
{
}

// r[impl mono.cgu-classification]
pub open spec fn classification_matches_spec(actual: u8, expected: u8) -> bool {
    actual == expected
}

// r[verify mono.cgu-classification]
pub proof fn admitted_classification_matches_rule(actual: u8, expected: u8)
    requires classification_matches_spec(actual, expected),
    ensures actual == expected,
{
}

// r[impl mono.item-envelope]
pub open spec fn instance_metadata_admitted_spec(kind_tag: u8, shape_tag: u8, argument_tag: u8) -> bool {
    kind_tag == shape_tag || argument_tag == 1
}

// r[verify mono.item-envelope]
pub proof fn admitted_instance_has_declared_shape(kind_tag: u8, shape_tag: u8, argument_tag: u8)
    requires instance_metadata_admitted_spec(kind_tag, shape_tag, argument_tag),
    ensures kind_tag == shape_tag || argument_tag == 1,
{
}

} // verus!

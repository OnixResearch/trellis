# B-tree Operations Specification

## Requirements

### Sortedness

#### Requirement: Sorted Predicate
`is_sorted` MUST return true iff keys are strictly increasing:
for all i < j, keys[i] < keys[j]. Empty and singleton sequences
are always sorted.

### Insert Path

#### Requirement: Find Position
`find_insert_position` MUST return the index where a key should be
inserted to maintain sortedness. For a key smaller than all elements,
returns 0. For a key larger than all elements, returns keys.len().
The position MUST satisfy: all keys before it are < key, all keys
at or after it are > key.

#### Requirement: Insert At Position
`insert_at` MUST insert a key at the given position, shifting
subsequent elements right. The result length MUST be keys.len() + 1.
If the input is sorted and the position is correct, the output MUST
be sorted.

#### Requirement: Insert Preserves Sortedness
Inserting a key not already present at the correct position MUST
produce a sorted sequence.

#### Requirement: Split Sorted Node
`split_node` MUST split a sorted sequence at split_point into two
sorted subsequences (left, right). left.len() + right.len() MUST
equal the original length. All keys in left MUST be < all keys in
right.

### Delete Path

#### Requirement: Contains Key
`contains_key` MUST return true iff the key exists in the sorted
sequence.

#### Requirement: Find Key Position
`find_key_position` MUST return the index of the key in the sorted
sequence, or keys.len() if not found.

#### Requirement: Delete At Position
`delete_at` MUST remove the element at the given position, shifting
subsequent elements left. Result length MUST be keys.len() - 1.
If input is sorted, output MUST be sorted.

#### Requirement: Delete Preserves Sortedness
Deleting any element from a sorted sequence MUST produce a sorted
sequence.

#### Requirement: Merge Sorted Nodes
`merge_nodes` MUST concatenate two sorted sequences where all keys
in the first are < all keys in the second. The result MUST be sorted
and have length left.len() + right.len().

#### Requirement: Borrow From Sibling
`borrow_left` takes the last key from left sibling and prepends it.
`borrow_right` takes the first key from right sibling and appends it.
Both MUST preserve sortedness of both nodes.

## Phase 1: B-tree Insert

- [x] Implement src/btree_insert.rs: is_sorted spec+exec
- [x] Implement find_insert_position spec+exec (binary search position)
- [x] Implement insert_at spec+exec (shift and insert)
- [x] Implement split_node spec+exec (split at midpoint)
- [x] Prove: find_insert_position result is correct (bounds property)
- [x] Prove: insert_at preserves sortedness (when position correct, key absent)
- [x] Prove: insert_at increases length by 1
- [x] Prove: split_node produces two sorted halves
- [x] Prove: split_node conserves keys (left.len + right.len == input.len)
- [x] Prove: split_node left < right (all keys in left < all in right)
- [x] Cross-module: split sizes match btree_balance::split_left/right_size
- [x] Unit tests + proptest
- [x] Verus + tracey clean

## Phase 2: B-tree Delete

- [x] Implement src/btree_delete.rs: contains_key spec+exec
- [x] Implement find_key_position spec+exec
- [x] Implement delete_at spec+exec (shift left)
- [x] Implement merge_nodes spec+exec (concatenate two sorted halves)
- [x] Implement borrow_left/borrow_right spec+exec
- [x] Prove: delete_at preserves sortedness
- [x] Prove: delete_at decreases length by 1
- [x] Prove: merge_nodes produces sorted result
- [x] Prove: merge_nodes conserves keys
- [x] Prove: borrow preserves sortedness of both nodes
- [x] Cross-module: merge decision matches btree_balance::should_merge
- [x] Cross-module: borrow decision matches btree_balance::can_borrow
- [x] Unit tests + proptest
- [x] Verus + tracey clean

## Phase 3: Integration

- [x] Add requirements to docs/requirements.md
- [x] Add modules to src/lib.rs
- [x] Verus + tracey clean

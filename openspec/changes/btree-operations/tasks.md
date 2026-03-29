## Phase 1: B-tree Insert

- [ ] Implement src/btree_insert.rs: is_sorted spec+exec
- [ ] Implement find_insert_position spec+exec (binary search position)
- [ ] Implement insert_at spec+exec (shift and insert)
- [ ] Implement split_node spec+exec (split at midpoint)
- [ ] Prove: find_insert_position result is correct (bounds property)
- [ ] Prove: insert_at preserves sortedness (when position correct, key absent)
- [ ] Prove: insert_at increases length by 1
- [ ] Prove: split_node produces two sorted halves
- [ ] Prove: split_node conserves keys (left.len + right.len == input.len)
- [ ] Prove: split_node left < right (all keys in left < all in right)
- [ ] Cross-module: split sizes match btree_balance::split_left/right_size
- [ ] Unit tests + proptest
- [ ] Verus + tracey clean

## Phase 2: B-tree Delete

- [ ] Implement src/btree_delete.rs: contains_key spec+exec
- [ ] Implement find_key_position spec+exec
- [ ] Implement delete_at spec+exec (shift left)
- [ ] Implement merge_nodes spec+exec (concatenate two sorted halves)
- [ ] Implement borrow_left/borrow_right spec+exec
- [ ] Prove: delete_at preserves sortedness
- [ ] Prove: delete_at decreases length by 1
- [ ] Prove: merge_nodes produces sorted result
- [ ] Prove: merge_nodes conserves keys
- [ ] Prove: borrow preserves sortedness of both nodes
- [ ] Cross-module: merge decision matches btree_balance::should_merge
- [ ] Cross-module: borrow decision matches btree_balance::can_borrow
- [ ] Unit tests + proptest
- [ ] Verus + tracey clean

## Phase 3: Integration

- [ ] Add requirements to docs/requirements.md
- [ ] Add modules to src/lib.rs
- [ ] Verus + tracey clean

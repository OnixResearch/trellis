## Phase 1: Write Path Composition

- [ ] Implement src/consensus_path.rs with proof-only module
- [ ] Prove: leader_append_entry satisfies leader_log_invariant
- [ ] Prove: appended entry at index log.len()+1 satisfies can_commit_at
- [ ] Prove: follower_update_commit monotonic and bounded by leader
- [ ] Prove: advance_last_applied bounded by commit_index
- [ ] Prove: write path end-to-end — append+commit+apply composition
- [ ] Unit tests + proptest for write path scenarios
- [ ] Verus + tracey clean

## Phase 2: Read Path Composition

- [ ] Prove: read_authority + read_safe_at → consistent read
- [ ] Prove: lease_read_safe implies read_safe_at (subsumption)
- [ ] Prove: matching prefix through commit → same entries below commit
- [ ] Unit tests + proptest for read path scenarios
- [ ] Verus + tracey clean

## Phase 3: Integration

- [ ] Add requirements to docs/requirements.md
- [ ] Add module to src/lib.rs
- [ ] Verus + tracey clean

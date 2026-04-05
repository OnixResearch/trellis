## Phase 1: Buffer Model

- [ ] Define `StoreEntry` struct: addr, size, data, seq
- [ ] Define buffer as `Seq<StoreEntry>` with max capacity constant
- [ ] Implement `enqueue(buffer, entry) -> Option<buffer'>` (None if full)
- [ ] Implement `drain_oldest(buffer) -> (StoreEntry, buffer')`
- [ ] Prove drain removes index 0 and preserves relative order of remaining

## Phase 2: Forwarding

- [ ] Define `can_forward(entry, load_addr, load_size) -> bool` coverage predicate
- [ ] Define `forward_lookup(buffer, load_addr, load_size) -> Option<StoreEntry>` scanning tail-to-head
- [ ] Prove youngest match: returned entry has highest seq among matches
- [ ] Prove full coverage: forwarded entry's range covers load's range
- [ ] Prove no-match returns None: when no entry covers the load, result is None

## Phase 3: Ordering and Fences

- [ ] Prove FIFO drain ordering: drain produces entries in enqueue order
- [ ] Prove drain preserves remaining entries' sequence numbers
- [ ] Define `is_fence_satisfied(buffer) -> bool` as buffer empty
- [ ] Prove fence semantics: after drain-all, buffer is empty
- [ ] Prove post-fence store enters an empty buffer

## Phase 4: Capacity

- [ ] Prove enqueue on full buffer returns None (no silent drop)
- [ ] Prove enqueue on non-full buffer increases size by 1
- [ ] Prove drain on non-empty buffer decreases size by 1

## Phase 5: Tests

- [ ] Unit tests for enqueue/drain on empty, single, full buffers
- [ ] Unit tests for forwarding: exact match, coverage, partial rejection
- [ ] Unit tests for youngest-match selection with multiple matches
- [ ] Proptest: forwarding result matches brute-force oracle
- [ ] Proptest: FIFO ordering preserved through random enqueue/drain sequences
- [ ] Proptest: buffer size never exceeds capacity

## Phase 1: Directory Model

- [ ] Define `DirectoryEntry`: owner (Option<u32>), sharers (Set<u32>), state (DIR_UNCACHED/DIR_SHARED/DIR_EXCLUSIVE)
- [ ] Define message types: ReadReq, WriteReq, Invalidate, InvalidateAck
- [ ] Implement `process_read_request(entry, cache) -> (entry', messages)`
- [ ] Implement `process_write_request(entry, cache) -> (entry', messages)`
- [ ] Implement `process_eviction(entry, cache) -> entry'`

## Phase 2: Consistency Invariant

- [ ] Define `directory_cache_consistent(entry, caches, line) -> bool`
- [ ] Prove read grant maintains consistency (new sharer recorded, cache enters S)
- [ ] Prove write grant maintains consistency (invalidations sent, single owner)
- [ ] Prove eviction notification maintains consistency (presence bit cleared)
- [ ] Prove initial state (uncached, all I) satisfies consistency

## Phase 3: Invalidation and Single-Writer

- [ ] Prove invalidation completeness: all sharers receive invalidation message
- [ ] Prove post-invalidation: no former sharer holds the line
- [ ] Prove single-writer: at most one cache in M at any time via directory state
- [ ] Prove second exclusive request invalidates first owner

## Phase 4: Serialization and MESI Embedding

- [ ] Prove request serialization: concurrent same-line requests processed sequentially
- [ ] Prove different-line independence: requests for different lines don't interact
- [ ] Define `directory_to_mesi(entry, cache) -> mesi_state` mapping
- [ ] Prove MESI embedding: directory-derived state is a valid MESI state

## Phase 5: Tests

- [ ] Unit tests for read/write request processing
- [ ] Unit tests for eviction notification
- [ ] Unit tests for invalidation message generation
- [ ] Proptest: consistency invariant preserved through random request sequences
- [ ] Proptest: single-writer maintained across random multi-cache operations
- [ ] Proptest: MESI embedding valid for all directory states

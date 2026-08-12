## Context

This change lands the smallest useful directory protocol slice: one modeled line,
a centralized controller, MESI-compatible cache states, and targeted
invalidations for ownership transfer. The repo already proves the per-cache MESI
side, so this module focuses on the directory record and the controller's
state-transition surface.

## Goals / Non-Goals

**Goals:**
- full-map sharer tracking with a `Vec<bool>` bit-vector
- exact directory/cache consistency checking
- targeted invalidation generation for writes
- single-writer postcondition after write grants
- eviction cleanup with an explicit writeback flag
- same-line sequential write serialization
- a small directory-to-MESI embedding helper

**Non-Goals:**
- distributed directories or coarse/limited sharer vectors
- asynchronous ack state machines or retry/nack handling
- payload/data transfer modeling
- multi-line interactions beyond independence-by-separation

## Decisions

### 1. Full-map bit-vector instead of a `Set`

**Choice:** use `DirectoryEntry { owner: Option<usize>, sharers: Vec<bool>, state }`
with explicit directory states `DIR_UNCACHED`, `DIR_SHARED`, and
`DIR_EXCLUSIVE`.

**Rationale:** the repo already uses `Vec<bool>` heavily in verified exec code,
so a bit-vector gives an executable full-map directory without a ghost-only
surface. The explicit state constants make the uncached/shared/exclusive
boundary visible to both `check_consistency(...)` and the request processors.

### 2. Reads and writes return the post-controller state

**Choice:** `process_read_request(...)` returns `(entry', caches')`. A read from
an uncached line grants a clean exclusive owner; a remote read of an exclusive
line downgrades the owner and requester into shared holders. Meanwhile
`process_write_request(...)` returns the fully updated directory, updated cache
vector, and the list of `InvalidateMessage`s that had to be sent.

**Rationale:** this keeps the proof surface small: read behavior is fully
captured by the returned cache shape, while write invalidation completeness is
captured by the message list and the returned cache vector already reflects the
acknowledged end state that matters for single-writer safety.

### 3. Eviction returns a writeback flag

**Choice:** `process_eviction(...)` returns `(entry', caches', writeback)`.

**Rationale:** a centralized directory needs to know whether a `Modified` owner
must write back on eviction. A boolean is enough for this first model; no memory
payload modeling is needed.

### 4. Shared state is exact, exclusive state is compatible

**Choice:** `check_consistency(...)` treats the uncached base case as "no owner,
no sharers, and all caches `Invalid`". Shared directory entries require exact
`Shared`/`Invalid` cache matching, while exclusive entries require the owner to
hold either `Exclusive` or `Modified` and everyone else `Invalid`.

**Rationale:** uncached state needs an explicit base predicate for the initial
configuration and for post-eviction cleanup. Shared state has one exact cache
shape, while exclusive ownership covers both read-granted clean ownership and
write-granted dirty ownership.

### 5. `directory_to_mesi(...)` returns the directory-implied abstract state

**Choice:** the embedding maps shared bits to `Shared`, exclusive ownership to
`Exclusive`, and all other caches to `Invalid`. The concrete cache state checked
by `check_consistency(...)` may still be `Modified` for the exclusive owner
after a write grant.

**Rationale:** the helper captures the directory's abstract MESI meaning, while
`check_consistency(...)` separately allows the concrete exclusive owner to be
`Modified` after a write grant.

## Verification Strategy

- **Verus proofs / postconditions:** prove the exec helpers implement the result
  specs for `valid_directory_entry`, `directory_to_mesi`, `check_consistency`,
  `process_read_request`, `process_write_request`, and `process_eviction`.
- **Focused runtime tests:** use unit tests for exact scenarios such as remote
  read downgrade, targeted invalidation lists, and modified-owner eviction.
- **Proptests:** cover random read/write/evict sequences, maintaining
  `check_consistency(...)`, single-writer outcomes, and valid directory-implied
  MESI states.
- **Traceability:** annotate public exec/spec surfaces and tests with the
  `r[directory.*]` IDs from `docs/requirements.md` and the change spec.
- **Different-line independence:** validate the serialization independence
  scenario with two separate `DirectoryEntry` instances rather than a multi-line
  cache structure, preserving the single-line model scope.

## Risks / Trade-offs

**[No explicit ack state machine]** -> Accept. The message list plus post-ack
state is enough for the safety properties this change targets.

**[Single-line only]** -> Accept. Different-line independence is easier to state
and test when each line is a separate directory entry.

**[MESI only]** -> Accept. The model intentionally stops before Owned-state
tracking; MOESI is already covered in `src/moesi.rs`.
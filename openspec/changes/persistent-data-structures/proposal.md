## Why

This repo has many verified ephemeral collections and a few domain-specific
persistent state machines, but it does not yet have reusable fully
persistent collection primitives. Snapshotting, rollback, speculative
execution, branch-and-bound search, compiler rewrites, and functional
APIs all want the same property: an update must produce a new version
without invalidating any older version.

The `rpds` crate is a good reference surface for what "persistent
collection primitives" should mean in practice: `List`, `Stack`,
`Queue`, `Vector`, `HashTrieMap`, `HashTrieSet`, `RedBlackTreeMap`, and
`RedBlackTreeSet`. This change defines repo-native verified equivalents
with explicit full-persistence and structural-sharing contracts instead
of leaving those guarantees implicit.

## What Changes

- **Shared persistent-store model**: add an append-only node-store model
  with opaque version handles, exact validity boundaries, and proofs that
  descendant stores preserve every prior version.
- **Linear persistent primitives**: add verified `persistent_list`,
  `persistent_stack`, and `persistent_queue` modules.
- **Indexed and hashed persistent primitives**: add verified
  `persistent_vector`, `persistent_hamt_map`, and `persistent_hamt_set`
  modules.
- **Ordered persistent primitives**: add verified
  `persistent_rb_tree_map` and `persistent_rb_tree_set` modules.

This change proves:

- updates may branch from any prior version
- old versions remain semantically unchanged in descendant stores
- unchanged tails / subtrees / trie branches are structurally shared
- list, stack, queue, vector, map, and set operations match their exact
  mathematical specs
- the public API boundary uses opaque versions, while raw store-index
  helpers stay private or carry explicit `requires`

## Scope

- **In scope**: full persistence, append-only descendant stores,
  structural sharing, opaque version handles, and the `List` / `Stack` /
  `Queue` / `Vector` / `HashTrieMap` / `HashTrieSet` /
  `RedBlackTreeMap` / `RedBlackTreeSet` primitive families
- **Out of scope**: confluent persistence, garbage collection or
  compaction, `Rc`/`Arc` pointer-kind abstraction, pluggable hash traits,
  and asymptotic proof development beyond local path / height / balance
  obligations

## Assumptions

- Public APIs accept only constructor-produced opaque versions; raw store
  indices stay private or carry explicit verifier preconditions.
- HAMT hashing uses a repo-default BLAKE3 helper with the usual
  runtime/verifier split instead of exposing raw Cargo hashing calls
  directly inside verifier-visible code.

## Capabilities

### New Capabilities

- `persistent.full-persistence`: updates may branch from any prior version
- `persistent.append-only-store`: descendant stores preserve prior nodes and versions
- `persistent.structural-sharing`: unchanged reachable substructure is reused exactly
- `persistent.list`: verified persistent list primitives
- `persistent.stack`: verified persistent stack primitives
- `persistent.queue`: verified persistent FIFO queue primitives
- `persistent.vector`: verified persistent indexed sequence primitives
- `persistent.hamt-map`: verified persistent hash-trie map primitives
- `persistent.hamt-set`: verified persistent hash-trie set primitives
- `persistent.rb-map`: verified persistent ordered map primitives
- `persistent.rb-set`: verified persistent ordered set primitives

## Impact

- **Files**: new modules `src/persistent_store.rs`, `src/persistent_list.rs`,
  `src/persistent_stack.rs`, `src/persistent_queue.rs`,
  `src/persistent_vector.rs`, `src/persistent_hamt_map.rs`,
  `src/persistent_hamt_set.rs`, `src/persistent_rb_tree_map.rs`, and
  `src/persistent_rb_tree_set.rs`; `src/lib.rs`; `docs/requirements.md`;
  `openspec/specs/persistent-data-structures/spec.md`; `README.md`
- **APIs**: opaque version handles, append-only descendant stores,
  push/pop/enqueue/dequeue/index/update/insert/remove/contains/get/first
  surfaces for the new persistent primitives
- **Dependencies**: reuse existing ordering/collections/interval-style
  helpers; use BLAKE3 as the default concrete hash for HAMT key digests;
  keep any runtime-only hashing helper behind the repo's normal
  `#[cfg(not(verus_keep_ghost))]` split
- **Testing**: proof functions plus positive and negative unit/property
  tests, followed by full repo validation (`cargo test`, `clippy`,
  `verus`, `check-ensures`, Tracey, and `./scripts/verify.sh`)

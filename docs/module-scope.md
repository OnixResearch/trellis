# Module Scope and Limitations

This document is the honest per-category account of where trellis's verified
surfaces stop. It generalizes the [`proof-gap-sites.tsv`](proof-gap-sites.tsv)
discipline (which tracks the 16 `#[verifier::external_body]` trusted boundaries)
to every major module group: what each group verifies, what it does not, and
where the remaining trust boundaries are.

This is the Shoals "Scope and limitations" discipline applied per-module-group.
Read it to learn what each part of the library guarantees, and reach for
elsewhere when you need something outside that scope.

## Convention

Every module group entry has three sections:

- **Verifies**: the properties the group's `spec fn` / `exec fn` / `proof fn`
  mechanically check via Verus.
- **Does not**: what the group explicitly does not prove, even if the code
  handles it at runtime.
- **Trust boundaries**: references to `proof-gap-sites.tsv` rows for any
  `#[verifier::external_body]` sites in the group, or "none" if the group has
  no trusted boundaries.

## Core arithmetic and data structures

**Verifies**: abs diff, saturating add/sub, checked mul, gcd, div ceil; power-of-two,
bit get/set/clear/toggle; multi-bit field extraction; fixed-size bitmap; range
predicates, clamping; sorted insert, binary search, merge sorted, partitioning;
checked narrowing (u32/u16/u8), signed↔unsigned; parity, alignment, log2,
next-power-of-two; half-open interval contains/overlap/split/adjacent/merge;
Option/Result combinators; FIFO queue with power-of-two capacity; monotone
sequence with batch reservation; chunk count, prefix/suffix equality.

**Does not**: prove overflow freedom for arbitrary precision integers (Verus
machine integers only); prove floating-point correctness (no float modules);
prove termination of user-supplied closures.

**Trust boundaries**: none.

## Raft consensus

**Verifies**: leader election state machine, pre-vote protocol, vote tallying,
ballot tracking, epoch ordering, quorum intersection theorem, heartbeat
primitives, check-quorum liveness, term-authority step-down, append-only log
with Log Matching Property, log conflict detection/resolution, log compaction,
match/next index tracking, commit advancement, commit notification, read-index
and lease-read linearizability, config change state machine, joint consensus,
voter/learner management, leadership transfer, snapshot integrity and transfer,
persistent state crash recovery, linearizability specification, state machine
safety, leader append-only, leader completeness, step decomposition, timing
validators, trace relation.

**Does not**: prove network behavior (message delivery, ordering, duplication);
prove clock synchronization across nodes; prove performance bounds (latency,
throughput); prove storage durability (assumes persistent state survives
crashes); prove liveness under arbitrary message loss (safety only, under
quorum availability).

**Trust boundaries**: none. Raft modules are fully verified within the Verus
model; the model assumes a bounded peer set and reliable local storage.

## Concurrency primitives

**Verifies**: lock acquisition order and cycle freedom via strict total order;
epoch-based reclamation three-epoch cycling and safe-to-reclaim gap; hazard
pointer protect/unprotect and reclaimable scan; sequence lock even/odd counter
and valid read detection; reference counting strong/weak and dealloc trigger;
work-stealing deque push/pop/steal bounds; thread barrier arrive/release;
spin-wait backoff two-phase; RCU grace periods; MPSC channel bounds;
futex wait/wake FSM; parking_lot mutex/condvar/rwlock state transitions;
ticket lock FIFO fairness; CAS retry ABA detection; bounded MPMC ring buffer;
memory ordering lattice; cache hierarchy invariants; directory coherence;
TLB shootdown; semaphore; two-phase locking; lease mutex with fencing tokens.

**Does not**: prove freedom from deadlock in arbitrary lock-composition
schemes (only strict total order is proven); prove performance characteristics
(contention, latency); prove correctness under memory model reordering beyond
the lattice (relaxed-to-SeqCst ordering is modeled, but not all possible
interleavings); prove cache coherence protocol correctness for arbitrary
snoop/filter designs (only the modeled subset).

**Trust boundaries**: none.

## Financial accounting

**Verifies**: debit/credit with overflow guards; double-entry bookkeeping
invariants; escrow fund/release/refund/dispute state machine with conservation;
basis-point fee computation with monotonicity and conservation; append-only
transaction journal with running balances; multi-account ledger with total
conservation theorem; bilateral netting with positions-sum-zero theorem;
value transfer between accounts with duplicate detection.

**Does not**: prove currency conversion correctness (no exchange-rate model);
prove regulatory compliance (no jurisdiction-specific rules); prove
settlement finality under legal frameworks (only the mathematical conservation
properties); prove time-value-of-money formulas (no interest/discount models).

**Trust boundaries**: none.

## Memory management and allocator

**Verifies**: buddy block sizing, addressing, alignment, coalescing; virtual
address decomposition and page alignment; address region contains/disjoint/
adjacent/merge/split; slab slot index/offset/address and generation handles;
bump allocation with remaining/fits/alloc/reset; binary heap index arithmetic;
allocation lifecycle FSM (free→allocated→freed); size class bucketing;
intrusive free list LIFO push/pop and no-cycle; buddy split/coalesce with
block conservation; generational bitmap double-free detection; arena-based
bump allocation with collective free; fixed-size slab pool; segregated free
lists; immediate vs deferred coalescing; external/internal fragmentation
metrics with formal bounds.

**Does not**: prove physical memory mapping correctness (virtual→physical is
modeled, not proven against hardware); prove allocator performance
(allocation speed, fragmentation rates under real workloads); prove
multithreaded allocator correctness (single-threaded model only).

**Trust boundaries**: none.

## Storage engine primitives

**Verifies**: B-tree node fill percentage, split/merge sizing, underfull
detection; B-tree insert sorted position and split on overflow; B-tree delete
key removal and merge/rebalance on underflow; LSM level target sizing and
space/write amplification; WAL entry sequencing, segment space, truncation;
WAL crash recovery replay range, idempotent replay, checkpoint; compaction
fence coordination; Fletcher-style page checksum and torn-write detection;
free-space bitmap tracking; write buffer (memtable) size tracking and flush;
write-back/write-through dirty invariants; block cache occupancy and eviction;
set-associative cache lookup.

**Does not**: prove storage medium durability (assumes writes persist on
power failure); prove concurrency safety for multi-threaded B-tree operations
(single-threaded model); prove LSM read amplification bounds under arbitrary
workloads; prove WAL recovery under partial writes beyond the checksum
detection model.

**Trust boundaries**: none.

## Circuit primitives (R1CS)

**Verifies**: bounded single-row R1CS shape, modular dot product, and
satisfaction admission; bounded two-bit decomposition over boolean R1CS
witness slots; bounded lookup-table membership; bounded booleanity row
encoding; bounded multiplication, addition, and constant gate adapters;
bounded multi-row R1CS system shape and shared-witness satisfaction; bounded
public-input binding.

**Does not**: prove multi-row/proving-system correctness; prove arbitrary-width
range-check; prove PLONK lookup; prove zero-knowledge property; prove
soundness of any proving system; prove the full R1CS constraint system is
satisfiable for arbitrary circuits.

**Trust boundaries**: none. All R1CS modules explicitly state "without
proving-system claims" in their module descriptions.

## Nix package manager and Nickel language semantics

**Verifies**: Nix attribute set operations (get, has, merge, intersect, remove);
Nix closure computation; Nix list and string operations; Nix version comparison
(same algorithm as `compareVersions`); Nickel merge priority total order;
Nickel three-way record split; Nickel field-level merge resolution by priority;
Nickel full record merge with identity, disjoint commutativity, and neutral-record
associativity; Nickel gradual type subtyping; Nickel contract blame polarity;
Nickel pattern completeness and overlap detection.

**Does not**: prove full Nix evaluator correctness (only the modeled builtin
operations); prove Nickel evaluator correctness (only the modeled merge/type/
contract semantics); prove performance of Nix/Nickel evaluation.

**Trust boundaries**: `src/nix_version.rs::compare_versions_unmodeled_trusted`
(permanent — only out-of-model byte sequences use the fallback; see
`proof-gap-sites.tsv`).

## Cross-module properties and composition

**Verifies**: 200+ cross-module proofs including signal↔bitwise, permission
independence, escrow↔balance, ledger↔transfer, fee conservation,
election↔quorum, fencing↔CAS, vector clock↔CRDT, mpmc↔mpsc occupancy,
spin↔CAS retry, seqlock↔memory order, RCU↔epoch reclaim, ref count↔hazard
pointer, lock ordering↔ticket lock, barrier↔quorum, futex↔notify roundtrip;
concurrency protocol↔data structure invariant chains across 7 groups;
end-to-end Raft read/write path composition.

**Does not**: prove composition correctness for arbitrary module combinations
(only the 200+ explicitly verified pairs); prove liveness under arbitrary
scheduling; prove performance of composed systems.

**Trust boundaries**: see `proof-gap-sites.tsv` for the 16 permanent trusted
boundaries (4 cryptographic hash axioms, 1 version comparison bridge, 10 UCAN
codec/signature bridges, 1 web-of-trust crypto bridge).

# Consensus Path Composition Specification

## Purpose

Proves that existing consensus module specs compose end-to-end along
the write path (client write → commit → apply) and read path (authority
check → serve). No new exec functions — only proof fns chaining
existing specs.

## Requirements

### Write Path

#### Requirement: Append Preserves Log Invariant

Leader appending an entry via leader_append_entry_spec MUST satisfy
leader_log_invariant_spec (the new log is a prefix extension of the old).

#### Requirement: Appended Entry Is Committable

An entry appended at term T to a log of length N creates an entry at
index N+1 with term T, which satisfies can_commit_at_spec when
current_term == T.

#### Requirement: Quorum Match Implies Committed

When match indices show a quorum has replicated up to index N, and
can_commit_at holds for N, advance_commit_index returns >= N.

#### Requirement: Follower Commit Bounded

follower_update_commit_spec returns a value that is >= current_commit
(monotonic) and <= leader_commit (bounded by leader).

#### Requirement: Applied Entries Are Committed

advance_last_applied_spec returns a value <= commit_index. Every
applied entry was committed before application.

#### Requirement: Write Path End-to-End

A client write appended by the leader at index N with term T, once
replicated to a quorum, is committed and eventually applied. After
application, state_machine_safety guarantees all servers agree on the
entry at index N.

### Read Path

#### Requirement: Read Authority Implies Safe Read

confirm_read_authority returning true (quorum ack) combined with
read_safe_at (commit covers read index) means the read sees all
committed writes up to read_index.

#### Requirement: Lease Subsumes Read Index

lease_read_safe_spec returning true implies read_safe_at_spec also
returns true. Lease reads are strictly stronger than ReadIndex.

#### Requirement: Committed Read Is Consistent

If two servers have matching log prefixes through the commit point and
a read is safe (commit >= read_index), both servers return the same
entry at any index below commit — by state_machine_safety.

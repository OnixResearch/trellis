## MODIFIED Requirements

### Requirement: Applied Prefix Predicate

The system MUST expose a finite applied-prefix implementation surface that admits state-machine application metadata only for committed contiguous log prefixes, preserves prefix order, advances last-applied monotonically, and rejects any attempt to apply entries beyond the admitted commit index, beyond the local log, or out of order.

r[domain-primitives.raft-consensus.applied-prefix]

#### Scenario: Applying uncommitted entry is rejected

- GIVEN an apply step targets an index greater than the admitted commit index
- WHEN applied-prefix admission is evaluated
- THEN the step is rejected with uncommitted-apply metadata


#### Scenario: Apply cannot skip a prefix entry

- GIVEN a node attempts to advance last-applied past an intermediate unapplied committed index
- WHEN applied-prefix admission is evaluated
- THEN the step is rejected with skipped-prefix metadata


#### Scenario: Monotone contiguous apply is admitted

- GIVEN a node advances last-applied from `k` to `k + n` where every index in the interval is committed and present in the local log
- WHEN applied-prefix admission is evaluated
- THEN the step is admitted and records the contiguous applied range metadata

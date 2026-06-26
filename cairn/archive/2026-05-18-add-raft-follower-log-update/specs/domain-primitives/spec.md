## MODIFIED Requirements

### Requirement: Follower Log Update Predicate [r[domain-primitives.raft-consensus.follower-log-update]]

The system MUST expose a finite follower log-update implementation surface for AppendEntries outcomes that leaves the follower log unchanged on rejection, preserves the matching prefix on accepted requests, truncates only conflicting suffix entries, appends admitted entries contiguously, and records deterministic conflict/update metadata.

#### Scenario: Rejected append does not mutate follower log

- GIVEN a follower log and AppendEntries request fail previous-log admission
- WHEN the follower log-update predicate is evaluated
- THEN the successor log equals the original log and rejection metadata identifies the first conflict


#### Scenario: Accepted append preserves matching prefix

- GIVEN an AppendEntries request has a matching previous-log boundary and admitted entries
- WHEN the follower log-update predicate is evaluated
- THEN every entry at or before the previous-log index is preserved in the successor log


#### Scenario: Accepted append is contiguous after truncation

- GIVEN an AppendEntries request replaces a conflicting suffix with admitted entries
- WHEN the follower log-update predicate is evaluated
- THEN the successor log has no index gaps and appended entries begin exactly after the retained prefix

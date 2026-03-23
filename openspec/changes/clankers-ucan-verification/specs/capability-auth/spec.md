# Capability Authorization Specification

## Purpose

Defines the correctness properties for UCAN-style capability
authorization: pattern matching primitives, operation authorization
dispatch, and delegation containment. Extracted from the
`clankers-ucan` capability module.

## Definitions

- **Pattern**: A sequence of items representing allowed values.
  A wildcard pattern matches everything. A multi-item pattern
  matches any single item in the set.
- **Glob pattern**: A byte sequence optionally ending with a
  wildcard byte. The prefix before the wildcard must match the
  start of the input.
- **Capability**: An enum variant granting a specific kind of
  access (prompt, tool use, shell execute, file access, etc.).
- **Operation**: An enum variant representing a requested action.
- **Authorization**: A capability authorizes an operation if the
  capability's variant matches the operation's variant and the
  capability's pattern/prefix covers the operation's target.
- **Containment**: A parent capability contains a child capability
  if the child's scope is equal or narrower on every axis.

## Requirements

### Requirement: Wildcard Matches All

A wildcard pattern MUST match any input value.

`match_pattern(WILDCARD, v)` MUST return true for all `v`.

#### Scenario: Wildcard tool pattern

- GIVEN a wildcard tool pattern
- WHEN checked against any tool name
- THEN the match succeeds

#### Scenario: Wildcard command pattern

- GIVEN a wildcard command pattern
- WHEN checked against any command string
- THEN the match succeeds

### Requirement: Exact Match Is Reflexive

A single-item pattern MUST match when the input equals that item.

`match_pattern([x], x)` MUST return true.

#### Scenario: Single tool name

- GIVEN pattern `["read"]` and input `"read"`
- WHEN matching
- THEN the result is true

### Requirement: Non-member Rejected

A multi-item pattern MUST reject inputs not in the set.

If `v` is not in pattern items, `match_pattern(pattern, v)` MUST
return false.

#### Scenario: Tool not in list

- GIVEN pattern `["read", "grep"]` and input `"bash"`
- WHEN matching
- THEN the result is false

### Requirement: Pattern Containment Is Subset

`pattern_contains(p1, p2)` MUST return true iff every item in
`p2` is also in `p1`.

#### Scenario: Subset

- GIVEN p1 = `["read", "grep", "find"]` and p2 = `["read", "grep"]`
- WHEN checking containment
- THEN p1 contains p2

#### Scenario: Not subset

- GIVEN p1 = `["read", "grep"]` and p2 = `["read", "bash"]`
- WHEN checking containment
- THEN p1 does not contain p2

### Requirement: Wildcard Contains Any Pattern

A wildcard pattern MUST contain any non-wildcard pattern.

`pattern_contains(WILDCARD, p)` MUST return true for all `p`.

#### Scenario: Wildcard parent

- GIVEN parent pattern is wildcard, child is `["read"]`
- WHEN checking containment
- THEN the wildcard contains the child

### Requirement: Non-wildcard Cannot Contain Wildcard

A non-wildcard pattern MUST NOT contain the wildcard pattern.

`pattern_contains(p, WILDCARD)` MUST return false when `p` is
not wildcard.

#### Scenario: Escalation attempt

- GIVEN parent pattern `["read", "grep"]` and child is wildcard
- WHEN checking containment
- THEN containment is denied

### Requirement: Containment Is Transitive

If `contains(A, B)` and `contains(B, C)` then `contains(A, C)`.

#### Scenario: Three-level delegation chain

- GIVEN A = wildcard, B = `["read", "grep", "find"]`, C = `["read"]`
- WHEN A contains B and B contains C
- THEN A contains C

### Requirement: Glob Prefix Match

A glob pattern with trailing wildcard MUST match any input that
starts with the prefix before the wildcard.

#### Scenario: Command glob

- GIVEN glob `"pg_*"` and input `"pg_dump"`
- WHEN glob matching
- THEN the match succeeds

#### Scenario: Non-matching glob

- GIVEN glob `"pg_*"` and input `"mysql"`
- WHEN glob matching
- THEN the match fails

### Requirement: Read-only Blocks Write

A file access capability with `read_only = true` MUST deny
write operations even when the path prefix matches.

#### Scenario: Read allowed

- GIVEN read-only file capability with prefix `/home/`
- WHEN authorizing FileRead for `/home/user/file.txt`
- THEN authorization succeeds

#### Scenario: Write denied

- GIVEN read-only file capability with prefix `/home/`
- WHEN authorizing FileWrite for `/home/user/file.txt`
- THEN authorization fails

### Requirement: Read-write Allows Both

A file access capability with `read_only = false` MUST allow
both read and write operations within the prefix.

#### Scenario: Both allowed

- GIVEN read-write file capability with prefix `/home/`
- WHEN authorizing FileRead or FileWrite for `/home/user/file.txt`
- THEN both succeed

### Requirement: Path Prefix Scoping

File access authorization MUST require the operation path to
start with the capability prefix.

#### Scenario: Outside prefix

- GIVEN file capability with prefix `/home/user/`
- WHEN authorizing FileRead for `/etc/passwd`
- THEN authorization fails

### Requirement: Mismatched Variants Deny

A capability of one variant MUST NOT authorize an operation of
a different variant (e.g., Prompt capability cannot authorize
ToolUse operation).

#### Scenario: Cross-variant denial

- GIVEN a Prompt capability
- WHEN authorizing a ToolUse operation
- THEN authorization fails

### Requirement: No Escalation on Delegation

Delegation containment MUST deny any child capability that
grants more access than the parent:

- Read-only parent MUST NOT contain read-write child
- Narrower prefix parent MUST NOT contain wider prefix child
- Non-wildcard parent MUST NOT contain wildcard child

#### Scenario: Read-only to read-write escalation

- GIVEN parent with read-only `/home/`, child with read-write `/home/user/`
- WHEN checking containment
- THEN containment is denied

#### Scenario: Prefix widening

- GIVEN parent with prefix `/home/user/`, child with prefix `/home/`
- WHEN checking containment
- THEN containment is denied

### Requirement: Delegation Allows Attenuation

A parent capability MUST contain a child that is strictly narrower:
narrower prefix, fewer pattern items, or adding read-only restriction.

#### Scenario: Attenuate to read-only

- GIVEN parent with read-write `/home/`, child with read-only `/home/user/`
- WHEN checking containment
- THEN containment succeeds

### Requirement: Shell Working Dir Scoping

ShellExecute authorization with a working dir constraint MUST
require the operation's working dir to start with the capability's
working dir. If the capability has a working dir but the operation
does not, authorization MUST fail.

#### Scenario: Allowed subdirectory

- GIVEN shell capability with working_dir `/home/user`
- WHEN authorizing command in `/home/user/project`
- THEN authorization succeeds

#### Scenario: Missing working dir

- GIVEN shell capability with working_dir `/home/user`
- WHEN authorizing command with no working dir
- THEN authorization fails

#### Scenario: No constraint

- GIVEN shell capability with working_dir None
- WHEN authorizing command in any directory
- THEN authorization succeeds

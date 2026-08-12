# Capability Authorization Specification

## Purpose

Defines correctness properties for UCAN-style capability authorization
as generic, composable building blocks. Covers scope coverage, access
mode ordering, variant subsumption, pattern matching, authorization
composition, delegation containment, chain validation, and token
timestamps. Applicable to both clankers-ucan and aspen-auth.

## Definitions

- **Scope**: A byte sequence representing a resource boundary
  (key prefix, path prefix, service prefix, etc.).
- **Coverage**: `covers(scope, target)` — the target falls within
  the scope. Implemented as `target.starts_with(scope)`.
- **Compound scope**: Two-field scoping where both fields must match
  (e.g., mount equality + prefix coverage).
- **Optional constraint**: An `Option<Scope>` where `None` means
  unconstrained and `Some` imposes a coverage check.
- **Access mode**: A position in a partial order over permission
  levels (e.g., Read ⊂ Full, SecretsRead ⊂ SecretsFull).
- **Subsumption**: A relation over capability variant types where
  one variant can authorize operations of another variant
  (e.g., Full subsumes Read, Write, Delete, Watch).
- **Pattern**: A sequence of items representing allowed values.
  A wildcard matches everything. A multi-item pattern matches
  any single item in the set.
- **Glob pattern**: A byte sequence with `*` wildcards. Trailing
  wildcard matches any suffix. Multi-segment wildcards match
  ordered substrings.
- **Capability**: A typed grant of access with a variant, scope,
  and access mode.
- **Operation**: A typed request with a variant, target, and mode.
- **Authorization**: `authorizes(cap, op) = subsumes(cap.variant,
  op.variant) && covers(cap.scope, op.target) && mode_permits(
  cap.mode, op.mode)`.
- **Containment**: `contains(parent, child)` — the child's access
  is equal or narrower than the parent's on every axis.
- **Delegation chain**: A sequence of tokens where each level
  attenuates capabilities from its parent.

## Requirements

### Scope Coverage

### Requirement: Scope Coverage Is Reflexive
r[capability-auth.scope-coverage-is-reflexive]

A scope MUST cover itself.

`covers(s, s)` MUST return true for all `s`.

#### Scenario: Identical prefix

- GIVEN scope `"/home/user/"` and target `"/home/user/"`
- WHEN checking coverage
- THEN coverage succeeds

### Requirement: Scope Coverage Is Transitive
r[capability-auth.scope-coverage-is-transitive]

Scope coverage MUST be transitive: if scope A covers B and B covers C, then A covers C.

`covers(a, b) && covers(b, c) ==> covers(a, c)`

#### Scenario: Nested prefixes

- GIVEN a = `"/home/"`, b = `"/home/user/"`, c = `"/home/user/docs/"`
- WHEN a covers b and b covers c
- THEN a covers c

### Requirement: Scope Coverage Rejects Non-Prefixes
r[capability-auth.scope-coverage-rejects-non-prefixes]

A scope MUST NOT cover a target that does not start with the scope.

`!starts_with(target, scope) ==> !covers(scope, target)`

#### Scenario: Disjoint paths

- GIVEN scope `"/home/user/"` and target `"/etc/passwd"`
- WHEN checking coverage
- THEN coverage fails

### Requirement: Scope Narrowing Is Monotone
r[capability-auth.scope-narrowing-is-monotone]

Scope narrowing MUST be monotone: if child scope is narrower than parent scope, then everything the child covers is also covered by the parent.

`covers(parent, child) ==> forall t: covers(child, t) ==> covers(parent, t)`

#### Scenario: Narrower prefix covers subset

- GIVEN parent scope `"/data/"` and child scope `"/data/sub/"`
- WHEN child covers `"/data/sub/file.txt"`
- THEN parent also covers `"/data/sub/file.txt"`

### Requirement: Compound Scope Requires All Fields
r[capability-auth.compound-scope-requires-all-fields]

Compound scoping (e.g., mount + prefix) MUST require all fields
to match. A mount mismatch MUST deny regardless of prefix match.

`compound_covers(cap_mount, cap_prefix, op_mount, op_path) =
 cap_mount == op_mount && covers(cap_prefix, op_path)`

#### Scenario: Mount mismatch denies

- GIVEN cap mount `"secret/"`, cap prefix `""`, op mount `"kv/"`, op path `"key"`
- WHEN checking compound coverage
- THEN coverage fails despite prefix match

#### Scenario: Both match

- GIVEN cap mount `"secret/"`, cap prefix `"app/"`, op mount `"secret/"`, op path `"app/db-pass"`
- WHEN checking compound coverage
- THEN coverage succeeds

### Requirement: Optional Constraint None Is Unconstrained
r[capability-auth.optional-constraint-none-is-unconstrained]

An optional constraint of `None` MUST allow any target.

`optional_covers(None, t) == true` for all `t`.

#### Scenario: No working dir constraint

- GIVEN capability working_dir `None`
- WHEN authorizing any operation
- THEN the optional constraint passes

### Requirement: Optional Constraint Some Requires Value
r[capability-auth.optional-constraint-some-requires-value]

An optional constraint of `Some` MUST deny when the target is `None`.

`optional_covers(Some(s), None) == false`

#### Scenario: Constraint present but value missing

- GIVEN capability working_dir `Some("/home/user")`
- WHEN operation has no working dir
- THEN authorization fails

### Requirement: Optional Constraint Some-Some Delegates to Coverage
r[capability-auth.optional-constraint-some-some-delegates-to-coverage]

Optional constraint delegation MUST equal coverage: `optional_covers(Some(s), Some(t)) == covers(s, t)`.

#### Scenario: Subdirectory matches

- GIVEN cap working_dir `Some("/home/user")`, op working_dir `Some("/home/user/project")`
- WHEN checking optional coverage
- THEN passes because `covers` passes

### Access Mode Ordering

### Requirement: Mode Order Is Transitive
r[capability-auth.mode-order-is-transitive]

The mode order relation MUST be transitive: `mode_le(a, b) && mode_le(b, c) ==> mode_le(a, c)`

#### Scenario: Read ≤ Full, Full ≤ Full

- GIVEN Read ≤ Full and Full ≤ Full
- WHEN checking Read ≤ Full transitively
- THEN holds

### Requirement: Mode Order Is Antisymmetric
r[capability-auth.mode-order-is-antisymmetric]

The mode order relation MUST be antisymmetric: `mode_le(a, b) && mode_le(b, a) ==> a == b`

#### Scenario: Two modes that contain each other are equal

- GIVEN mode_le(X, Y) and mode_le(Y, X)
- WHEN checking mode_le(X, Y) and mode_le(Y, X)
- THEN X == Y

### Requirement: Mode Permits Checks Ordering
r[capability-auth.mode-permits-checks-ordering]

Authorization MUST check that the capability's mode permits the
operation's mode: `mode_permits(cap_mode, op_mode)`.

A Read-mode capability MUST NOT authorize Write operations.
A Full-mode capability MUST authorize both Read and Write.

#### Scenario: Read-only denies write

- GIVEN cap mode Read, op mode Write
- WHEN checking mode_permits
- THEN denied

#### Scenario: Full allows both

- GIVEN cap mode Full, op mode Read
- WHEN checking mode_permits
- THEN allowed

### Requirement: Delegation Cannot Escalate Mode
r[capability-auth.delegation-cannot-escalate-mode]

Delegation containment MUST require `mode_le(child_mode, parent_mode)`.
A child MUST NOT have a higher mode than its parent.

#### Scenario: Read parent, Write child rejected

- GIVEN parent mode Read, child mode Write
- WHEN checking delegation containment
- THEN rejected

#### Scenario: Full parent, Read child allowed

- GIVEN parent mode Full, child mode Read
- WHEN checking delegation containment
- THEN allowed (attenuation)

### Variant Subsumption

### Requirement: Subsumption Soundness
r[capability-auth.subsumption-soundness]

If variant A does not subsume variant B, then a capability of
type A MUST NOT authorize an operation of type B.

`!subsumes(A, B) ==> !authorizes(cap_A, op_B)`

#### Scenario: Read cap cannot authorize Delete op

- GIVEN Read capability and Delete operation
- WHEN subsumes(Read, Delete) is false
- THEN authorization fails

### Requirement: Subsumption Completeness
r[capability-auth.subsumption-completeness]

If variant A subsumes variant B, and scope covers and mode permits,
then authorization MUST succeed.

`subsumes(A, B) && covers(scope) && mode_permits(mode) ==> authorizes`

#### Scenario: Full cap authorizes Read op within scope

- GIVEN Full capability with prefix `"/data/"` and Read op for `"/data/key"`
- WHEN Full subsumes Read, scope covers, mode permits
- THEN authorization succeeds

### Requirement: Subsumption Is Acyclic
r[capability-auth.subsumption-is-acyclic]

No variant MUST subsume itself through a chain of other variants.
The subsumption relation forms a DAG.

#### Scenario: No circular subsumption

- GIVEN the full subsumption relation
- WHEN checking for cycles
- THEN none exist

### Requirement: Non-Subsumed Pairs Always Deny
r[capability-auth.non-subsumed-pairs-always-deny]

A capability MUST NOT authorize an operation of an incompatible
variant regardless of scope or mode.

#### Scenario: Prompt cap vs ToolUse op

- GIVEN a Prompt capability with wildcard scope
- WHEN authorizing a ToolUse operation
- THEN authorization fails

### Pattern Matching

### Requirement: Wildcard Matches All
r[capability-auth.wildcard-matches-all]

A wildcard pattern MUST match any input value.

`match_pattern(WILDCARD, v)` MUST return true for all `v`.

### Requirement: Exact Match Is Reflexive
r[capability-auth.exact-match-is-reflexive]

`match_pattern([x], x)` MUST return true.

### Requirement: Non-member Rejected
r[capability-auth.non-member-rejected]

If `v` is not in pattern items, `match_pattern(pattern, v)` MUST
return false.

### Requirement: Pattern Containment Is Subset
r[capability-auth.pattern-containment-is-subset]

`pattern_contains(p1, p2)` MUST return true iff every item in
`p2` is also in `p1`.

### Requirement: Wildcard Contains Any Pattern
r[capability-auth.wildcard-contains-any-pattern]

`pattern_contains(WILDCARD, p)` MUST return true for all `p`.

### Requirement: Non-wildcard Cannot Contain Wildcard
r[capability-auth.non-wildcard-cannot-contain-wildcard]

`pattern_contains(p, WILDCARD)` MUST return false when `p` is
not wildcard.

### Requirement: Pattern Containment Is Transitive
r[capability-auth.pattern-containment-is-transitive]

Pattern containment MUST be transitive: `pattern_contains(a, b) && pattern_contains(b, c) ==> pattern_contains(a, c)`

### Glob Matching

### Requirement: Glob Wildcard Matches All
r[capability-auth.glob-wildcard-matches-all]

A glob pattern that is exactly `"*"` MUST match any input.

### Requirement: Glob Prefix Match
r[capability-auth.glob-prefix-match]

A glob pattern with trailing `*` MUST match inputs starting with
the prefix.

#### Scenario: `"pg_*"` matches `"pg_dump"`

- GIVEN a glob pattern `"pg_*"` and an input `"pg_dump"`
- WHEN the glob is matched against the input
- THEN the match succeeds

### Requirement: Glob Exact Match
r[capability-auth.glob-exact-match]

A glob pattern with no wildcards MUST match only exact equality.

### Requirement: Glob Multi-Segment Match
r[capability-auth.glob-multi-segment-match]

A glob pattern with multiple `*` wildcards MUST match inputs where
segments appear in order. First segment anchored at start, last
segment anchored at end, middle segments appear anywhere in order.

#### Scenario: `"pg_*_dump"` matches `"pg_full_dump"`

- GIVEN a glob pattern `"pg_*_dump"` and an input `"pg_full_dump"`
- WHEN the glob is matched against the input
- THEN the match succeeds

#### Scenario: `"pg_*_dump"` does not match `"pg_full_restore"`

- GIVEN a glob pattern `"pg_*_dump"` and an input `"pg_full_restore"`
- WHEN the glob is matched against the input
- THEN the match fails

### Requirement: Glob Non-Match Rejected
r[capability-auth.glob-non-match-rejected]

A glob pattern MUST reject inputs that don't satisfy the pattern.

### Authorization Composition

### Requirement: Authorization Decomposes Into Three Factors
r[capability-auth.authorization-decomposes-into-three-factors]

`authorizes(cap, op)` MUST be equivalent to:
`subsumes(cap.variant, op.variant) && covers(cap.scope, op.target)
&& mode_permits(cap.mode, op.mode)`

Each factor is independently verified. The composition is structural.

### Requirement: Authorization Rejects on Any Factor Failure
r[capability-auth.authorization-rejects-on-any-factor-failure]

If any of the three factors fails, authorization MUST fail.

#### Scenario: Scope mismatch

- GIVEN matching variant and mode, but scope doesn't cover
- WHEN `authorizes(cap, op)` is evaluated
- THEN authorization fails

#### Scenario: Mode mismatch

- GIVEN matching variant and scope, but mode doesn't permit
- WHEN `authorizes(cap, op)` is evaluated
- THEN authorization fails

#### Scenario: Variant mismatch

- GIVEN matching scope and mode, but variant not subsumed
- WHEN `authorizes(cap, op)` is evaluated
- THEN authorization fails

### Delegation Containment

### Requirement: Containment Decomposes Into Scope + Mode + Subsumption
r[capability-auth.containment-decomposes-into-scope-mode-subsumption]

`contains(parent, child)` MUST require:
- `subsumes(parent.variant, child.variant)` or same variant
- `covers(parent.scope, child.scope)`
- `mode_le(child.mode, parent.mode)`

### Requirement: Containment Is Transitive
r[capability-auth.containment-is-transitive]

Containment MUST be transitive: `contains(a, b) && contains(b, c) ==> contains(a, c)`

### Requirement: No Escalation on Any Axis
r[capability-auth.no-escalation-on-any-axis]

Delegation MUST deny if the child is wider on any axis:
- Wider scope than parent
- Higher mode than parent
- Non-subsumed variant

### Delegation Chain

### Requirement: Chain Attenuation at Each Level
r[capability-auth.chain-attenuation-at-each-level]

In a valid delegation chain, each level's capabilities MUST be
contained by its parent's capabilities. Every child capability
MUST have a containing parent capability.

### Requirement: Delegate Gate
r[capability-auth.delegate-gate]

Delegation MUST require the parent token to hold a Delegate
capability. Without Delegate, no child tokens can be created.

### Requirement: Chain Depth Bounded
r[capability-auth.chain-depth-bounded]

Chain depth MUST NOT exceed MAX_DELEGATION_DEPTH (8).
`depth > MAX_DELEGATION_DEPTH ==> chain invalid`

### Requirement: Root Has Depth Zero
r[capability-auth.root-has-depth-zero]

Root tokens (no proof/parent) MUST have depth 0.

### Requirement: Depth Increments by One
r[capability-auth.depth-increments-by-one]

Each delegation level MUST increment depth by exactly 1.
`child.depth == parent.depth + 1`

### Token Timestamps

### Requirement: Fresh Token Is Immediately Valid
r[capability-auth.fresh-token-is-immediately-valid]

A token checked at its `issued_at` time MUST be valid.

`issued_at <= expires_at ==> valid_at(token, issued_at)`

### Requirement: Expiration Is Monotonic
r[capability-auth.expiration-is-monotonic]

Once a token is expired, it MUST stay expired.

`expired_at(t1) && t2 >= t1 ==> expired_at(t2)`

### Requirement: Expiration Includes Tolerance
r[capability-auth.expiration-includes-tolerance]

A token MUST be considered valid until `expires_at + tolerance`.

`now <= expires_at + tolerance ==> not_expired`

### Requirement: Future Tokens Rejected
r[capability-auth.future-tokens-rejected]

A token with `issued_at > now + tolerance` MUST be rejected.

### Requirement: Refresh Threshold
r[capability-auth.refresh-threshold]

A token MUST be refreshed when its remaining lifetime is ≤ 20% of total.

`remaining <= total_lifetime / 5 ==> needs_refresh`

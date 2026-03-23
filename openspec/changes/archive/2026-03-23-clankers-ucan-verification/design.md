## Context

Both clankers-ucan and aspen-auth implement UCAN capability
authorization as pure functions over enum pairs. Aspen's production
code has ~25 capability variants and ~60 match arms in `authorizes`,
~50 in `contains`. Studying aspen-auth reveals that every match arm
is a composition of the same three checks: variant compatibility,
scope coverage, and access mode ordering.

Aspen already has standalone Verus specs (`verus/timestamp_spec.rs`,
`verus/delegation_spec.rs`, `verus/credential_spec.rs`, etc.) and
pure verified functions (`verified_credential.rs`). Verified-logic
should produce the generic building blocks with exec+proof pairs
that both systems can instantiate.

The verified-logic crate already has `slice` (with `ends_with`,
`starts_with` proofs) and `collections` (membership, find_first).

## Goals / Non-Goals

**Goals:**
- Verify scope coverage as a generic predicate (prefix, compound,
  optional) reusable across all capability types
- Verify access mode as a partial order with no-escalation guarantee
- Verify variant subsumption with soundness and completeness
- Verify that `authorizes` decomposes into subsumption × scope × mode
- Verify delegation chain walk with loop invariant (attenuation,
  Delegate gate, depth bound)
- Verify token timestamp logic (expiry, refresh, clock skew)
- Verify pattern matching primitives (wildcard, set membership,
  glob including multi-segment)
- Produce reusable building blocks not tied to clankers or aspen

**Non-Goals:**
- Verify cryptographic signature validation (Ed25519 out of scope)
- Verify token encoding/decoding (serde round-trips)
- Verify revocation store persistence (redb/KV I/O)
- Model specific capability variant enums from either system
- String-based pattern representations — we abstract to `Seq<int>`

## Decisions

### 1. Scope Coverage as the Foundational Predicate

**Choice:** A single `covers(scope, target) -> bool` function used
by both `authorizes` and `contains`. Authorization asks "does this
capability's scope cover this operation's target?" Containment asks
"does this parent's scope cover this child's scope?" Same predicate,
different arguments.

**Rationale:** Aspen uses `target.starts_with(cap_prefix)` on KV
keys, secret paths, transit key names, PKI roles, net service
prefixes, and shell working dirs. The verified-logic `slice` module
already has `starts_with` proofs. Defining coverage once and proving
reflexivity, transitivity, and monotone narrowing gives correctness
for all ~25 capability variants in aspen and all 8 in clankers.

**Implementation:** `covers_spec(scope: Seq<u8>, target: Seq<u8>)`
delegates to `starts_with`. Compound scoping (mount+prefix) is
`mount_eq && covers(prefix, path)`. Properties proven once,
applied everywhere.

### 2. Access Mode as a Partial Order

**Choice:** Access modes form a partial order (lattice) rather than
a boolean flag. The `read_only` flag from clankers is a degenerate
two-element lattice. Aspen has richer mode structures.

**Rationale:** Aspen instances of the same pattern:
- `Read ⊂ Full`, `Write ⊂ Full`, `Delete ⊂ Full`, `Watch ⊂ Full`
- `SecretsRead ⊂ SecretsFull`, `SecretsWrite ⊂ SecretsFull`, etc.
- `PkiIssue ⊂ PkiManage`, `PkiRevoke ⊂ PkiManage`

Modeling as a partial order lets us prove no-escalation once:
`mode_le(child_mode, parent_mode)` is required for delegation.
Authorization checks `mode_permits(cap_mode, op_mode)`. The
boolean `read_only` flag is just `mode_le(Read, ReadWrite)`.

**Implementation:** Modes are integer-tagged. `mode_le_spec` and
`mode_permits_spec` are spec functions over integers. A proof
function establishes transitivity and antisymmetry.

### 3. Variant Subsumption as a Relation

**Choice:** Variant compatibility is modeled as a `subsumes(a, b)`
relation rather than strict variant equality. This captures "admin
authorizes sub-operations" patterns.

**Rationale:** Aspen has four admin variants that authorize operations
of different variants:
- `Full → Read + Write + Delete + Watch`
- `SecretsAdmin → all 14 secrets/transit/PKI ops`
- `PkiManage → PkiIssue + PkiRevoke + PkiReadCa`
- `NetAdmin → NetConnect + NetPublish + NetUnpublish`

The plan originally modeled this as "mismatched variants deny."
That's wrong — mismatched variants deny *unless* the capability
variant subsumes the operation variant.

**Properties:**
- Soundness: `!subsumes(a, b) ==> !authorizes(cap_a, op_b)`
- Completeness: `subsumes(a, b) && covers(scope) && mode_permits ==> authorizes`
- Acyclic: no variant subsumes itself through a chain

### 4. Optional Constraint as a Generic Combinator

**Choice:** The `(cap_wd, op_wd)` three-way match is extracted as
`optional_covers(cap: Option<Scope>, target: Option<Scope>)`.

**Rationale:** The clankers ShellExecute working-dir pattern is:
`(None, _) → true`, `(Some, Some) → covers`, `(Some, None) → false`.
This is a reusable combinator for any capability with an optional
scoping field. Proving it once gives correctness for ShellExecute
and any future capability that adds optional constraints.

**Implementation:** `optional_covers_spec` delegates to `covers_spec`
in the `(Some, Some)` case. Properties: `None` never restricts,
`Some` with `None` always denies, containment requires child
constraint is at least as tight.

### 5. Authorization = Subsumption × Scope × Mode

**Choice:** `authorizes(cap, op)` is decomposed into three factors
rather than modeled as a monolithic dispatch.

**Rationale:** Every match arm in aspen's 60-arm dispatch evaluates
the same three checks. Some arms have compound scope (mount+prefix),
some have optional constraints (working dir), some have trivial
scope (ClusterAdmin). But the structure is always: variant compatible?
scope covered? mode permitted?

If the three factors are verified independently, the dispatch proof
becomes structural instantiation rather than 60 separate proofs.

### 6. Glob Match Including Multi-Segment

**Choice:** Full glob matching is verified, including the
multi-segment case (`foo*bar*baz`). This was previously a stretch
goal.

**Rationale:** Aspen ships the multi-segment code path in
production (`glob_match` in `capability.rs`). The function splits
by `*`, checks the first segment at the start, middle segments
anywhere in order, and the last segment at the end. Since it runs
in production for ShellExecute patterns, it should be verified.

**Implementation:** `glob_match_spec` decomposes into cases:
pure wildcard, trailing wildcard (prefix), no wildcard (exact),
and multi-segment (greedy ordered substring matching). The
multi-segment case is specified as: for each segment in order,
there exists a position in the remaining input where the segment
appears.

### 7. Delegation Chain Walk as a Loop Invariant

**Choice:** The credential chain validation (`is_credential_chain_valid`
in aspen's `verified_credential.rs`) is verified as a loop invariant,
not just the binary `contains` predicate.

**Rationale:** The plan previously covered only `contains(parent, child)`.
But the actual production code walks a chain: for each level, check
that the parent has Delegate, check that every child capability is
contained by some parent capability, and check depth. This loop is
where bugs hide — the predicate is the building block, the chain
walk is the security-critical code path.

**Implementation:** A `chain_valid_spec` function with a loop
invariant: at iteration `i`, all levels `0..i` satisfy attenuation,
all parents `0..i` have Delegate, and `i ≤ MAX_DELEGATION_DEPTH`.

### 8. Token Timestamps with Saturating Arithmetic

**Choice:** Token expiry and refresh are verified as pure functions
with saturating arithmetic for overflow safety.

**Rationale:** Aspen already has both Verus specs (`timestamp_spec.rs`)
and pure exec functions (`is_token_expired`, `needs_refresh`). These
are direct Verus targets with high value and low complexity.

**Implementation:**
- `is_expired(expires_at, now, tolerance)` uses `saturating_add`
- `needs_refresh(issued_at, expires_at, now)` uses 20% threshold
- Properties: fresh token valid, expiration monotonic, clock skew
  within tolerance is acceptable

### 9. Patterns as Seq<int>, Not Strings

**Choice:** Tool patterns and command patterns are modeled as
`Seq<int>` where each `int` is an abstract item ID.

**Rationale:** Same as before — Verus reasons about `Seq` membership
better than string splitting. Properties hold over any element type
with equality.

### 10. Tiger Style Constants as Preconditions

**Choice:** Aspen's bounded constants are incorporated as `requires`
clauses on relevant specs:
- `MAX_CAPABILITIES_PER_TOKEN = 32`
- `MAX_DELEGATION_DEPTH = 8`
- `MAX_TOKEN_SIZE = 8192`
- `TOKEN_CLOCK_SKEW_SECS = 60`

**Rationale:** These bounds are security invariants. They should
appear as explicit preconditions so proofs are grounded in the
actual production limits.

## Risks / Trade-offs

**[Pattern abstraction loses string semantics]** → If a bug is in
comma-splitting logic, our `Seq<int>` model won't catch it.
Mitigation: proptest on the exec side uses actual string patterns.

**[Generic building blocks may over-abstract]** → Concrete systems
have quirks (e.g., aspen's SecretsAdmin authorizes transit ops
even though they're a different domain). Mitigation: the subsumption
relation is explicitly enumerated, not derived. Each concrete
instance is a declared fact, not an inference.

**[Multi-wildcard glob is harder to specify]** → The greedy segment
matching in aspen's glob_match has subtle corner cases (empty
segments, adjacent wildcards). Mitigation: the spec function
is defined constructively, and proptest validates against aspen's
actual implementation.

**[Chain walk loop invariant may be hard in Verus]** → Loop
invariants over sequences with index arithmetic are a known pain
point. Mitigation: the chain is bounded by MAX_DELEGATION_DEPTH (8),
so we can unroll if the general proof is intractable.

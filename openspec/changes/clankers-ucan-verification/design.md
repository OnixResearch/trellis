## Context

The clankers UCAN capability system (`clankers-ucan/src/capability.rs`)
implements authorization as a pure function over two enums: `Capability`
(8 variants) and `Operation` (8 variants). Pattern matching uses string
operations (split by comma, starts_with, exact equality). Delegation
containment uses the same pattern primitives plus prefix comparison
and boolean flag checks.

The verified-logic crate already has `slice` (with `ends_with`,
`starts_with` proofs) and `collections` (membership, find_first).

## Goals / Non-Goals

**Goals:**
- Verify pattern matching primitives (wildcard, membership, containment)
- Verify authorization dispatch completeness (every variant pair covered)
- Verify delegation no-escalation for all capability axes
- Verify containment transitivity for delegation chains
- Produce reusable pattern matching building blocks

**Non-Goals:**
- Verify cryptographic signature validation (Ed25519 is out of scope)
- Verify token encoding/decoding (serde round-trips)
- Verify revocation store persistence (redb I/O)
- Model the full UCAN delegation chain verification (just the
  capability containment predicate)
- String-based pattern representations — we abstract to `Seq<int>`

## Decisions

### 1. Patterns as Seq<int>, Not Strings

**Choice:** Tool patterns and command patterns are modeled as
`Seq<int>` where each `int` is an abstract item ID. Wildcard is
represented by a distinguished sentinel value or a boolean flag.

**Rationale:** The clankers pattern logic splits on commas and
compares tokens. The semantics is set membership. Verus reasons
about `Seq` membership and quantifiers better than string splitting.
Modeling patterns as item sets captures the essential structure.

**Alternative:** Model patterns as `Seq<Seq<u8>>` (sequences of
byte-strings). Rejected — adds string comparison complexity without
strengthening the proofs. The properties (wildcard matches all,
subset is transitive) hold over any element type with equality.

### 2. Separate Modules for Pattern and Authorization

**Choice:** Two conceptual layers in one module:
1. Pattern primitives (`match_pattern`, `pattern_contains`, `glob_match`)
2. Authorization dispatch (`authorizes`, `contains`)

Pattern primitives are verified first, then authorization is built
on top and its proofs invoke the pattern lemmas.

**Rationale:** The pattern layer is reusable (any ACL system needs
set membership and containment). Separating it makes the proofs
modular. The authorization layer is a big match statement that
delegates to pattern primitives — its proof is structural (one
case per variant pair).

### 3. File Access as Prefix Comparison

**Choice:** File access authorization is modeled as `starts_with`
on `Seq<u8>` (the path prefix). The read-only flag is a separate
boolean parameter.

**Rationale:** The clankers code does `path.starts_with(prefix)`.
The verified-logic `slice` module already has `starts_with` proofs.
We reuse them directly.

**Implementation:** `file_access_authorizes(prefix, read_only, path, is_write)`
calls `starts_with(path, prefix)` and returns false if `read_only && is_write`.

### 4. Glob Match as Prefix + Suffix

**Choice:** Glob matching is decomposed into:
- If pattern is `*`: return true
- If pattern ends with `*`: check `starts_with(input, prefix)`
- Otherwise: exact equality

The multi-wildcard case (e.g., `foo*bar*baz`) is modeled but the
clankers code treats it as an edge case. We verify the common cases
(trailing wildcard, exact) and leave multi-wildcard as a stretch goal.

**Rationale:** 99% of clankers glob patterns are `*` or `prefix_*`.
The multi-wildcard branch in clankers is rarely used and harder to
specify (it's a greedy segment-matching algorithm). The common cases
carry the security value.

### 5. Containment Transitivity as Separate Lemma

**Choice:** Transitivity of `contains` is proved as a standalone
lemma, not embedded in the `contains` function itself.

**Rationale:** The `contains` function returns `bool`. Transitivity
is a property that requires universal quantification over three
capabilities. A proof function `lemma_contains_transitive(a, b, c)`
with `requires contains(a, b) && contains(b, c)` and
`ensures contains(a, c)` is the natural Verus idiom.

**Implementation:** One proof function per capability variant.
The wildcard case is trivial. The pattern case follows from set
transitivity. The file access case follows from prefix transitivity
(`starts_with` is transitive).

### 6. Working Dir as Optional Prefix

**Choice:** ShellExecute working directory constraint is modeled as
`Option<Seq<u8>>` with the same `starts_with` semantics as file
access. `None` means no constraint (match anything).

**Rationale:** The clankers code has a 3-way match on
`(cap_wd, op_wd)`: `(None, _) → true`, `(Some, Some) → starts_with`,
`(Some, None) → false`. This is a standard optional prefix pattern.

## Risks / Trade-offs

**[Pattern abstraction loses string semantics]** → If a bug in
clankers is in the comma-splitting logic (e.g., whitespace handling),
our `Seq<int>` model won't catch it. Mitigation: proptest on the
exec side uses actual string patterns to validate the abstraction
boundary.

**[Multi-wildcard glob not fully verified]** → The `foo*bar*baz`
glob path in clankers is complex. We verify the common cases and
document the gap. Mitigation: the multi-wildcard case is only used
for shell command patterns, and production usage is almost exclusively
`*` or `prefix_*`.

**[Authorization dispatch is structural, not deep]** → Most of the
proof for `authorizes` is case analysis over variant pairs. The
interesting logic is in the pattern primitives. Mitigation: this
is the correct decomposition — the dispatch is simple, the primitives
carry the invariants.

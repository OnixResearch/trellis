## Design: per-module-group scope/limitations convention

### Decision: generalize proof-gap-sites.tsv, not replace it

The existing `docs/proof-gap-sites.tsv` tracks the 16 `#[verifier::external_body]`
trusted boundaries with per-site rationale. This is the right granularity for
trust-boundary tracking. The new `docs/module-scope.md` does NOT replace it;
it generalizes the discipline to all module groups, including those with zero
trusted boundaries.

### Convention

Every module group entry in `docs/module-scope.md` has three sections:

- **Verifies**: the properties the group's `spec fn` / `exec fn` / `proof fn`
  mechanically check via Verus.
- **Does not**: what the group explicitly does not prove, even if the code
  handles it at runtime.
- **Trust boundaries**: references to `proof-gap-sites.tsv` rows for any
  `#[verifier::external_body]` sites, or "none" if the group has no trusted
  boundaries.

This is the Shoals "Scope and limitations" page discipline
(chelis.ch/docs/shoals/scope/) applied per-module-group.

### Coverage

Phase 1 documents the top-level categories from the README:
- Core arithmetic and data structures
- Raft consensus
- Concurrency primitives
- Financial accounting
- Memory management and allocator
- Storage engine primitives
- Circuit primitives (R1CS)
- Nix/Nickel language semantics
- Cross-module properties and composition

Remaining categories (distributed systems, networking, database internals,
graph algorithms, compiler backend, scheduling, job runner, serialization,
systems decision logic, generic proof infrastructure, and the additional
public module coverage) can be added incrementally in follow-up changes.

### What is explicitly NOT provided

- Per-module (individual file) scope docs — the granularity is per-group, not
  per-file, because trellis has 602 modules and per-file docs would be
  infeasible and unmaintainable.
- Performance claims — scope docs describe verification scope, not performance.
- Completeness — the scope docs do not claim to cover every possible use case
  or failure mode; they document what Verus checks and what it does not.

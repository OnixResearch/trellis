# Module Scope Specification

## Purpose

Defines the `module-scope` capability.

## Requirements

### Requirement: Per-module-group scope documentation
r[trellis.module_scope] The repository MUST maintain a `docs/module-scope.md` file documenting the scope and limitations of each major module group with three sections: Verifies, Does not, and Trust boundaries.

#### Scenario: Scope doc covers top-level categories
r[trellis.module_scope.coverage]
- GIVEN `docs/module-scope.md`
- WHEN the documented categories are reviewed
- THEN they include core arithmetic, Raft consensus, concurrency primitives, financial accounting, memory management, storage engine, circuit primitives, Nix/Nickel semantics, and cross-module properties

#### Scenario: Each entry has three sections
r[trellis.module_scope.convention]
- GIVEN a module group entry in `docs/module-scope.md`
- WHEN the entry is reviewed
- THEN it has Verifies, Does not, and Trust boundaries sections

#### Scenario: Trust boundaries reference proof-gap-sites.tsv
r[trellis.module_scope.trust_boundaries]
- GIVEN a module group that has `#[verifier::external_body]` sites
- WHEN the entry's Trust boundaries section is reviewed
- THEN it references `proof-gap-sites.tsv` as the authoritative per-site source

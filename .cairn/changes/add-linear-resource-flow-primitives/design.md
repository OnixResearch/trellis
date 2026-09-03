# Design: Add linear resource-flow primitives

## Context

Trellis already provides these nearby components:

- `type_unification` for finite first-order terms, substitutions, and occurs checks
- `algebraic_effect` for signatures, rows, handlers, and single-use continuations
- `ability_effect` for finite ability requirements and discharge
- `remote_computation` for normalized resource admission facts
- graph and job modules for bounded partition, lifecycle, and rejection decisions

The new module owns only finite symbolic resource-flow meaning. It does not own source syntax, canonical interchange, graph lowering, authority, or runtime effects.

## Reference Boundary

The design adapts ideas from `mirth-lang/mirth` revision `b180112a547cb803e3bf5720a0bb08f8bffa9742`.

The reviewed Mirth surfaces separate ordinary stack values from resources. They also reject ordinary resource duplication, disposal, and value smuggling.

Trellis will not import source, generated C, compiler state, type-based dispatch, resource sliding, FFI behavior, or runtime code.

The Mirth build and golden suite are reference evidence only. They do not enter the Trellis proof trust base.

## Decisions

### Decision: Use one narrow verified module

**Choice:** Add `src/linear_resource_flow.rs` with colocated specification, executable, proof, and test surfaces.

**Rationale:** One coherent module gives consumers a narrow capability. It avoids changes to unrelated effect or graph modules.

### Decision: Model admitted normalized identities

**Choice:** Represent resource kind, state, occurrence, operation, and witness identities with bounded normalized values.

Consumers perform string parsing, canonical hashing, schema validation, and source admission before they call the module.

**Rationale:** Trellis proves semantic decisions over supplied facts. It does not duplicate consumer wire formats.

### Decision: Separate row expressions from closed rows

**Choice:** A row expression contains resource entries and an optional tail variable. A closed row contains only concrete entries.

A resource entry contains a kind identity, state identity, and occurrence identity. A well-formed closed row has unique occurrence identities.

**Rationale:** The optional tail supports reusable composition rules. Closed rows keep graph admission and evidence finite.

### Decision: Reuse first-order substitution discipline

**Choice:** Encode row expressions as a finite recursive term surface or reuse equivalent `type_unification` helpers.

Unification must apply an occurs check and produce a substitution that makes both admitted row expressions equal.

**Rationale:** A separate unifier duplicates established Trellis meaning and proof obligations.

### Decision: Keep row order explicit

**Choice:** Row equality and substitution preserve declared order. The module does not slide or reorder entries implicitly.

A consumer can admit a reorder only through a separate explicit permutation or independence witness outside the first profile.

**Rationale:** Mirth resource sliding does not establish safe commutativity for concurrent workflow effects.

### Decision: Define arrow composition over rows

**Choice:** A resource arrow has an input row expression and an output row expression.

Sequential composition succeeds only when the first output and second input unify. The result applies the substitution to the outer rows.

**Rationale:** This is the smallest reusable form of stack-effect composition.

### Decision: Define closed graph operations separately

**Choice:** Fork, conditional join, and terminal discharge operate on closed rows after substitution.

- A fork must partition input occurrences into disjoint branch rows.
- A conditional join requires every possible branch to return the same closed row.
- Terminal discharge requires an empty row or explicit export and disposal facts.

**Rationale:** Graph concurrency needs stricter rules than sequential word composition.

### Decision: Require operation witnesses for non-linear behavior

**Choice:** Duplication and disposal succeed only when the caller supplies an admitted witness for the exact resource kind and operation.

The first profile does not infer witnesses from names, row positions, abilities, or authority facts.

**Rationale:** Non-linear behavior must remain visible and reviewable.

### Decision: Make rejection observationally inert

**Choice:** Every rejected decision returns the original modeled rows and no admitted transition.

**Rationale:** A failed admission must not partially consume, duplicate, reorder, or transform a resource occurrence.

### Decision: Publish a consumer profile

**Choice:** Export a machine-readable consumer profile with model version, bounds, functions, proof names, assumptions, source revision, and non-claims.

Choregraph must pin an immutable Trellis revision and validate its local projection against this profile.

**Rationale:** A passing Trellis proof does not prove a downstream adapter.

## Core Surface

The initial public capability includes these conceptual operations:

```text
well_formed_closed_row(row)
unify_rows(left, right)
compose_arrows(first, second)
admit_fork(input, branch_rows)
admit_conditional_join(branch_rows)
admit_terminal(row, exports, disposals)
admit_duplicate(entry, witness)
admit_dispose(entry, witness)
```

Exact Rust names can follow existing Trellis naming conventions. The public operations must expose specifications and executable refinements.

## Proof Portfolio

The change must include proof functions for these properties:

- unification soundness for successful results
- occurs-check rejection for recursive substitutions
- closed-row occurrence uniqueness
- sequential composition compatibility
- sequential outer-row preservation after substitution
- fork partition completeness and pairwise disjointness
- conditional join agreement
- no duplication without an exact witness
- no disposal without an exact witness
- terminal non-loss
- rejection preservation for every public decision
- executable equality with each public specification

The portfolio does not claim completeness of general row unification unless the implemented solver and proof state that stronger result.

## Functional Core Boundary

The module must remain deterministic and `no_std + alloc` compatible. It must not read files, inspect the environment, use clocks, spawn processes, print, hash, serialize, or access networks.

Tests, documentation generation, proof evidence, and artifact export remain in repository shells.

## Tests

Positive tests include:

- closed equal-row composition
- polymorphic tail substitution
- disjoint fork partition
- equal conditional branch output
- exact duplication witness
- exact disposal witness
- empty terminal row

Negative tests include:

- duplicate occurrence in one row
- recursive row substitution
- sequential mismatch
- implicit reorder
- fork duplication
- fork loss
- branch output mismatch
- wrong duplication witness
- wrong disposal witness
- undisposed terminal occurrence
- rejection that changes modeled input

Property tests cover finite generated closed rows and partition cases within declared bounds.

## Documentation and Evidence

The change must update `docs/requirements.md`, `src/lib.rs`, the README module catalog, module guides, indexes, and the proof-artifact manifest.

Evidence must record the exact Verus, Rust, Octet, policy, and Mirth reference identities. It must list assumptions and trusted boundaries.

## Risks and Trade-offs

- General row unification can exceed proof budgets. The implementation must keep finite terms and explicit fuel or bounds.
- Ordered rows reject harmless reorderings. This is safer than implicit commutativity and leaves later witness-based extension possible.
- Normalized integer identities can hide parsing errors. Choregraph must validate its projection separately.
- Explicit disposal can be confused with runtime cleanup. The witness proves only symbolic admission.
- A complete local proof does not establish liveness, effect completion, authorization, or consumer correctness.

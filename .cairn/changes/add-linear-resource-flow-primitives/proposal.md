# Proposal: Add linear resource-flow primitives

## Why

Choregraph needs reusable proof laws for resources that cannot follow ordinary value duplication or disposal rules.

Trellis already owns type unification, algebraic effects, resource admission, graph algorithms, and finite state-machine proofs. These modules do not expose one resource-row composition contract.

Mirth provides a useful design reference for separate value and resource stacks. Trellis will adapt the bounded laws without importing the language or runtime.

## What Changes

- Add a pure `no_std + alloc` linear resource-flow module.
- Model finite row expressions with resource entries, closed rows, variables, and substitutions.
- Reuse the existing type-unification and algebraic-effect ideas where their exact contracts fit.
- Define sequential composition, fork partitioning, conditional joins, terminal discharge, and explicit duplication or disposal witnesses.
- Add executable decisions with `ensures` clauses that match their specifications.
- Prove conservation, disjoint fork partitioning, join agreement, witness requirements, and rejection preservation.
- Add positive, negative, property, and consumer-profile evidence.
- Record Mirth revision `b180112a547cb803e3bf5720a0bb08f8bffa9742` as a bounded design reference.

## Outcome and Durable Capability

The immediate outcome is one verified module that Choregraph can consume through an immutable Trellis revision.

The durable capability is deterministic admission for finite linear resource rows without host effects or product policy.

Trellis maintains the generic calculus. Choregraph maintains graph-specific projection and admission.

## Impact

The change adds a public verified-logic module, requirement anchors, proof artifacts, tests, documentation, and consumer-profile evidence.

The change does not modify existing type, effect, job, scheduler, or authority semantics.

## Dependencies

The first consumer is Choregraph change `admit-linear-resource-flow`.

Kamacite change `encode-linear-resource-flow-artifacts` defines the independent canonical interchange contract.

## Non-goals

- Do not implement a concatenative language, parser, compiler, evaluator, or runtime.
- Do not copy Mirth source.
- Do not model live handles, credentials, tokens, processes, persistence, clocks, or effect execution.
- Do not infer resource commutativity or independence.
- Do not prove Choregraph lowering, Lattice execution, or external effect completion.
- Do not add a generic shared monolith or a port around pure internal functions.

## Context

`r1cs_constraint` admits one row and `r1cs_system` admits multi-row satisfaction over one shared witness. The next useful circuit semantic layer is a public statement surface: selected witness slots are declared public and checked against public values.

## Goals / Non-Goals

**Goals**
- Model a finite public-input binding list with active-prefix semantics.
- Check binding indices against the active witness prefix.
- Check public values against bound witness slots modulo the system modulus.
- Compose binding satisfaction with existing R1CS system satisfaction.

**Non-Goals**
- No proof systems, commitments, Fiat-Shamir transcripts, verifier keys, or cryptographic claims.
- No unbounded vectors or dynamic allocation in the first primitive.

## Decisions

### Bounded binding slots

Use three physical binding slots with `len` selecting the active prefix. This mirrors the existing R1CS finite-slot shape and keeps Verus proofs direct.

### Index/value split

Each binding carries a witness index and public value. Admission rejects active indices outside the witness prefix and malformed public values. Equality is checked modulo the shared system modulus.

### Statement composition

`r1cs_statement_satisfied` is true exactly when the system is satisfied and public bindings are satisfied for the same witness and system modulus. This creates statement semantics without moving into proving systems.

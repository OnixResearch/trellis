## Context

The existing R1CS primitives expose a bounded witness of up to three field elements and bounded R1CS rows checked with `<A,w> * <B,w> == <C,w> (mod p)`. Users still have to hand-author coefficient rows.

## Goals

- Provide a readable finite gate layer for the smallest common gate encodings.
- Keep each adapter deterministic, bounded, and directly checkable through existing row satisfaction.
- Reject malformed variable indices before row construction/satisfaction.

## Non-Goals

- No full circuit builder, allocation namespace, proof protocol, transcript, commitment, or cryptographic soundness model.
- No boolean/range decomposition in this slice.

## Decisions

### Gate shape

Use small copyable structs/enums: `R1csGateKind` and `R1csGate { kind, x, y, z_or_value, modulus, witness_len }`. `x/y/z` are witness indices for multiplication/addition; `x` plus `z_or_value` encodes a constant gate.

### Row encoding

- Multiplication: `w[x] * w[y] = w[z]`.
- Addition: `(w[x] + w[y]) * 1 = w[z]`, with the constant-one wire represented by coefficient `1` on `w0`. This adapter therefore requires `w0 == 1` for addition gates.
- Constant: `w[x] * 1 = c`, also requiring `w0 == 1`.

### Verification strategy

Mirror each executable admission/encoding/satisfaction helper with a spec predicate and prove direct `ensures result == spec` obligations. Add catalogue proof functions for requirements that are tested at runtime.

## Risks

- The bounded three-slot witness leaves little space for both a constant-one wire and data wires. Mitigation: document that addition/constant gates require the caller to reserve witness slot 0 as one, and keep tests explicit.

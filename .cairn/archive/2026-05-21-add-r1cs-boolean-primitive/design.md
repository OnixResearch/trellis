## Context

The R1CS row primitive bounds coefficients to `R1CS_MAX_ELEMENT` and witnesses to three active slots. The gate adapter already uses witness slot `w0` as a constant-one wire for addition and constant encodings.

## Goals

- Add a tiny booleanity adapter that composes with existing `r1cs_row_satisfied`.
- Keep the encoding finite and compatible with the existing bounded coefficient model.
- Make the constant-one wire and modulus bound explicit.

## Non-Goals

- No bit-vector/range recomposition yet.
- No general circuit builder or namespace allocator.
- No proof protocol or cryptographic soundness claims.

## Decisions

### Encoding

Use the equivalent constraint `b * (1 - b) = 0`. In row form this is `A=b`, `B=1 + (p-1)b`, and `C=0`, so satisfaction is computed by the existing modular row checker.

### Admission

Because row coefficients are bounded by `R1CS_MAX_ELEMENT`, booleanity requires `modulus - 1 <= R1CS_MAX_ELEMENT`. The boolean slot must be inside the active witness prefix and cannot be slot zero, because slot zero is reserved as the constant-one wire for this adapter.

### Satisfaction

The helper returns true exactly when the shape is admitted, the witness is admitted with matching active length, `w0 == 1`, and the encoded row is satisfied.

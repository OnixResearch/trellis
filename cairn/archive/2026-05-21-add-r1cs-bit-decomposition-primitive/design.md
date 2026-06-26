## Context

The R1CS boolean primitive admits a nonzero witness slot and checks `b * (1 - b) = 0` using the existing single-row satisfaction checker. The bounded R1CS witness model has slots `w0`, `w1`, and `w2`, with `w0` reserved as the constant-one wire.

## Goals

- Add the smallest useful bit-decomposition semantic boundary on top of booleanity.
- Keep the width finite and explicit: two bits in slots `w1`/`w2`-style active positions.
- Make recomposition a direct deterministic check rather than a proving-system claim.

## Non-Goals

- No arbitrary-width bit-vector allocator.
- No full range-check circuit builder.
- No lookup tables, commitment schemes, or proof protocols.

## Decisions

### Width

Use a fixed two-bit decomposition because the current bounded witness model has only two non-constant witness slots. This still gives a useful range-check consequence (`value < 4`) and keeps the primitive Verus-friendly.

### Bit admission

The shape stores the active witness length, modulus, value, and two distinct nonzero bit indices. Both bit indices must be inside the active witness prefix, and the modulus must satisfy the same boolean coefficient bound as `r1cs_boolean`.

### Satisfaction

The helper returns true exactly when the shape is admitted, the witness is admitted with the same active length, slot zero is one, both bit slots satisfy `r1cs_boolean_satisfied`, and the two extracted witness bits recombine to the requested value as `bit0 + 2 * bit1`.

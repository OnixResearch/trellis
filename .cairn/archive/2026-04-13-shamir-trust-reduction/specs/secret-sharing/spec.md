# Secret Sharing Specification

## Purpose

Keeps the secret-sharing requirement prose aligned with the verified
Lagrange model used by the Shamir trust-reduction pass.

This change does not expand the Shamir feature set. It narrows the
uniqueness wording to the behavior actually tracked by the current
Tracey requirement and implemented proof: for a fixed distinct x-set,
interpolant agreement on those points determines one y-vector.

## Requirements

### Requirement: Uniqueness

For fixed distinct interpolation points, the y-value vector is uniquely
determined by the interpolant values on those points. There is exactly
one value vector attached to a given interpolant over the chosen x-set.

##### Scenario: Two interpolants agreeing on k points

- GIVEN fixed distinct x-values x_1, ..., x_k
- AND value vectors Y and Z of length k
- WHEN L_Y(x_i) == L_Z(x_i) for all i in 1..k
- THEN Y and Z MUST be identical

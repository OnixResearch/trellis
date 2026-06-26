## ADDED Requirements

### Requirement: GPU Stencil Boundary Program Semantics

The system MUST define finite GPU stencil-program boundary semantics where admitted output coordinates, stencil radii, and read windows guarantee that every modeled input read remains inside the declared finite input domain before any kernel computation claim is made.

r[domain-primitives.gpu.stencil-boundary-program]

#### Scenario: Interior stencil reads are admitted

- GIVEN an output coordinate whose full radius window lies inside the finite input domain
- WHEN stencil read-window admission is evaluated
- THEN every read offset in the modeled window is admitted as in bounds

#### Scenario: Boundary without guard is rejected

- GIVEN an output coordinate near an edge and a stencil radius that would read outside the input domain
- WHEN unguarded stencil admission is evaluated
- THEN the read window is rejected

#### Scenario: Guarded boundary narrows reads

- GIVEN a boundary coordinate with an explicit guard that skips out-of-domain offsets
- WHEN guarded stencil admission is evaluated
- THEN only in-domain read coordinates are admitted and no out-of-domain read is claimed

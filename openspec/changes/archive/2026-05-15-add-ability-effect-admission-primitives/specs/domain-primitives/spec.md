## ADDED Requirements

### Requirement: Ability Requirement Set Admission

The system MUST define finite ability requirement sets where a computation is admitted in a context only when every required ability is provided by that context, and an empty requirement set is treated as pure without claiming runtime side-effect absence beyond the finite model.

r[domain-primitives.ability-effect.requirement-set]

#### Scenario: Provided context admits requirements

- GIVEN a computation requiring abilities `A` and `B`
- WHEN the provided context contains `A`, `B`, and possibly additional abilities
- THEN ability admission accepts the computation

#### Scenario: Missing ability is rejected

- GIVEN a computation requiring abilities `A` and `B`
- WHEN the provided context lacks `B`
- THEN ability admission rejects the computation with missing-ability metadata

#### Scenario: Empty requirement is pure in the model

- GIVEN a computation with an empty ability requirement set
- WHEN ability admission is evaluated against any context
- THEN the computation is admitted as requiring no modeled abilities

### Requirement: Ability Handler Discharge

The system MUST define finite ability handler discharge where a handler for an ability removes exactly the handled ability from the required set when the handler is declared available, leaving all unhandled abilities explicit in the resulting requirement set.

r[domain-primitives.ability-effect.handler-discharge]

#### Scenario: Handler removes declared ability

- GIVEN a computation requiring `Store` and `Exception`
- WHEN a handler for `Store` is applied
- THEN the resulting modeled requirement set still contains `Exception` and no longer contains `Store`

#### Scenario: Handler cannot discharge absent declaration

- GIVEN a computation requiring `Remote`
- WHEN a handler declaration does not include `Remote`
- THEN the handler discharge check does not remove `Remote`

### Requirement: Ability Sandbox Attenuation

The system MUST define sandbox ability attenuation where a sandboxed context can provide only a subset of its parent context and remote or untrusted computations are rejected when their requirements exceed the sandbox-provided ability set.

r[domain-primitives.ability-effect.sandbox-attenuation]

#### Scenario: Sandbox subset is admitted

- GIVEN a parent context with `IO`, `Remote`, and `Exception`
- WHEN a sandbox provides only `Exception`
- THEN the sandbox context is admitted as an attenuation of the parent

#### Scenario: Sandbox cannot add ability

- GIVEN a parent context lacking `IO`
- WHEN a sandbox declaration attempts to provide `IO`
- THEN sandbox attenuation rejects the declaration

#### Scenario: Computation exceeding sandbox is rejected

- GIVEN a sandbox providing only `Exception`
- WHEN a computation requiring `IO` is evaluated for sandbox admission
- THEN the computation is rejected with missing-ability metadata

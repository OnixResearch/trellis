# WebAssembly Runtime Bounds Delta Specification

## Purpose

Extend the accepted WebAssembly primitive domain with finite verified execution-budget, continuation, allocation-phase, growth-policy, and streaming-schedule laws that downstream runtimes can instantiate without claiming proof of any concrete interpreter.

## Requirements

### Requirement: Runtime-bound models are finite and deterministic

r[wasm-runtime-bounds.model]
The WebAssembly runtime-bound model MUST represent finite owned states, explicit transition facts, bounded host observations, and closed outcome classes and MUST reject transition models that do not satisfy the declared deterministic validity predicate.

#### Scenario: Valid finite model is admitted

- GIVEN every state and transition is in bounds and each executable state/observation pair has at most one next result
- WHEN model admission runs
- THEN bounded execution helpers may evaluate that model

#### Scenario: Transition model is ambiguous or malformed

- GIVEN a state/observation pair has conflicting transitions or references an out-of-range state
- WHEN model admission runs
- THEN the model is rejected before execution

### Requirement: Fuel accounting is exact and non-underflowing

r[wasm-runtime-bounds.fuel]
Fuel consumption MUST subtract exactly the admitted amount when sufficient fuel remains and MUST return out-of-fuel with fuel and runtime state unchanged when the amount is unavailable.

#### Scenario: Sufficient fuel is consumed

- GIVEN remaining fuel is at least the requested cost
- WHEN consumption runs
- THEN the result succeeds and remaining fuel equals the prior value minus the exact cost

#### Scenario: Insufficient fuel is unchanged

- GIVEN requested cost exceeds remaining fuel
- WHEN consumption runs
- THEN the result is out-of-fuel and both fuel and runtime state are unchanged

#### Scenario: Zero budget is an identity

- GIVEN any valid nonterminal runtime state
- WHEN bounded execution runs with zero available steps
- THEN no transition occurs and the state is returned unchanged with an out-of-fuel boundary

### Requirement: Segmented execution preserves deterministic observations

r[wasm-runtime-bounds.segmentation]
For a valid deterministic transition model, identical host-observation input, and representable admitted budgets, segmented execution MUST produce the same reached state, consumed-step count, remaining observation suffix, and terminal class as uninterrupted execution over the same combined budget, subject to the declared terminal and host-pause stopping rules.

#### Scenario: Nonterminal prefix resumes to the same result

- GIVEN uninterrupted execution uses a combined budget and segmented execution uses an admitted prefix followed by its exact remainder
- WHEN neither path changes model or host observations
- THEN both paths reach the same normalized result and consume the same observations

#### Scenario: Terminal boundary occurs in the prefix

- GIVEN execution finishes or traps before the prefix budget is exhausted
- WHEN uninterrupted and segmented forms run
- THEN both stop at that same terminal result and no suffix transition executes

#### Scenario: Host observations drift

- GIVEN resumed execution receives a different host-observation sequence or transition model
- WHEN segmentation comparison runs
- THEN the segmentation theorem does not admit the comparison and the exec checker rejects equivalence

### Requirement: Continuations bind exact resume facts

r[wasm-runtime-bounds.continuation]
A modeled continuation MUST bind generation, artifact, runtime-profile, input, authority/resource-context, and recorded-host-observation identities, and resume admission MUST succeed only when every supplied fact matches exactly.

#### Scenario: Current facts match the continuation

- GIVEN all bound identities equal the current resume request
- WHEN resume admission runs
- THEN the continuation is admitted unchanged for bounded execution

#### Scenario: Any bound identity is stale

- GIVEN generation, artifact, profile, input, context, or observation identity differs
- WHEN resume admission runs
- THEN resume is rejected and the continuation is unchanged

### Requirement: Allocation is capped and initialization-only

r[wasm-runtime-bounds.allocation]
The abstract page allocator MUST track initialization, running, and finished phases; MUST allocate monotonically only during initialization while the requested total fits the declared cap; and MUST return explicit unchanged failure for OOM, overflow, or non-initialization allocation.

#### Scenario: Initialization allocation fits

- GIVEN the allocator is initializing and allocated pages plus requested pages fit the cap
- WHEN allocation runs
- THEN allocated pages increase by exactly the request and remain at or below the cap

#### Scenario: Initialization allocation exceeds capacity

- GIVEN the request would overflow or exceed the cap
- WHEN allocation runs
- THEN an explicit failure is returned and phase, cap, and allocated pages are unchanged

#### Scenario: Execution requests allocation

- GIVEN the allocator is running or finished
- WHEN page allocation is requested
- THEN the request is rejected and allocator state is unchanged

### Requirement: Streaming schedules cover exact bytes

r[wasm-runtime-bounds.streaming]
A non-empty stream schedule MUST start at byte zero, use positive representable chunk lengths, place each chunk immediately after its predecessor, and end exactly at the declared byte length; the empty-input representation MUST be explicit and unambiguous.

#### Scenario: Contiguous schedule is admitted

- GIVEN ordered chunks partition the declared source interval exactly
- WHEN schedule admission runs
- THEN the schedule is accepted and every source byte belongs to exactly one chunk

#### Scenario: Schedule has a gap or overlap

- GIVEN a chunk starts before or after its predecessor end
- WHEN schedule admission runs
- THEN the schedule is rejected

#### Scenario: Schedule has trailing or missing bytes

- GIVEN the final chunk end differs from the declared source length
- WHEN schedule admission runs
- THEN the schedule is rejected

#### Scenario: Chunk arithmetic overflows

- GIVEN an offset plus length is not representable
- WHEN schedule admission runs
- THEN the schedule is rejected without wrapping

### Requirement: Memory growth follows explicit policy

r[wasm-runtime-bounds.growth-policy]
A runtime growth profile MUST explicitly allow or deny `memory.grow`; denial MUST leave memory unchanged, while admission MUST preserve the accepted whole-page, prefix, zero-fill, cap, and failed-no-op laws of the existing Wasm memory-growth primitive.

#### Scenario: Growth is denied by profile

- GIVEN the profile denies memory growth
- WHEN `memory.grow` admission runs
- THEN the result is policy denial and memory is unchanged

#### Scenario: Growth is admitted and fits

- GIVEN the profile allows growth and the existing checked growth preconditions hold
- WHEN growth runs
- THEN the existing successful growth postconditions remain true

#### Scenario: Admitted growth cannot fit

- GIVEN the profile allows growth but the checked growth operation fails
- WHEN growth runs
- THEN the existing failed-growth no-op postcondition remains true

### Requirement: Runtime-bound helpers carry proof and test evidence

r[wasm-runtime-bounds.evidence]
Every public exec helper MUST refine its declared spec or relation and the module MUST include positive tests for admitted behavior and negative tests for malformed models, insufficient fuel, drift, stale continuations, allocation failures, denied growth, and invalid schedules.

#### Scenario: Public helper is reviewed

- GIVEN a runtime-bound helper is exported
- WHEN proof and test coverage are inspected
- THEN its requirement, implementation annotation, proof or ensures clause, and positive/negative evidence are all present

### Requirement: Runtime-bound claims remain abstract

r[wasm-runtime-bounds.non-claims]
Trellis MUST describe these results as proofs over finite abstract models and MUST NOT claim that they verify SpaceWasm, Wasmtime, a WebAssembly decoder/interpreter, unsafe allocation code, host scheduling, serialization, migration, sandboxing, flight qualification, or downstream adapter correctness.

#### Scenario: Consumer references a proof artifact

- GIVEN a downstream runtime cites a Trellis runtime-bound proof
- WHEN claim scope is evaluated
- THEN the citation may support the abstract law but cannot replace validation of the consumer's concrete fact mapping

#### Scenario: Evidence requests concrete-runtime correctness

- GIVEN a report promotes the abstract proof to a concrete interpreter correctness or certification claim
- WHEN evidence validation runs
- THEN the unsupported promotion is rejected

### Requirement: Runtime-bound documentation is traceable

r[wasm-runtime-bounds.docs]
The change MUST add accepted requirement text, public module documentation, implementation and verification annotations, exports, catalog entries, and explicit proof/non-claim guidance for every runtime-bound primitive.

#### Scenario: Requirement coverage is queried

- GIVEN the new runtime-bound requirements
- WHEN Tracey coverage is evaluated
- THEN every requirement has implementation and verification links or an explicit bounded rationale

### Requirement: Runtime-bound validation is complete

r[wasm-runtime-bounds.final-validation]
The change MUST run focused Cargo/property tests, full-library Verus verification, Tracey uncovered and untested checks, Cairn validation, and proposal/design/tasks gates before synchronization and archive.

#### Scenario: A promised proof is missing

- GIVEN a task or requirement promises a theorem without a verified proof function or valid ensures refinement
- WHEN closeout review runs
- THEN the change remains incomplete even if runtime tests pass

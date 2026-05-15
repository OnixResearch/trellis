## Why

Unison abilities make effects explicit in function types and require handlers or enclosing contexts to provide the requested abilities. `verified-logic` has capability and UCAN primitives, but no small reusable finite model of ability requirement sets, handlers, and sandbox attenuation.

## What Changes

- Specify finite ability/effect requirement sets and provided-context admission.
- Specify handler discharge and remaining ability requirements.
- Specify sandbox/resource attenuation as a subset check over provided abilities.
- Exclude runtime effect execution, handler implementation correctness, and foreign API behavior.

## Scope

In scope: structural predicates for typed effect/ability admission.

Out of scope:
- Real Unison syntax/typechecker behavior.
- Runtime I/O, network, STM, threads, or foreign-function execution.
- Authorization policy beyond finite ability-set containment.

## Verification

Validate this OpenSpec package, then implement executable predicates, proof functions, positive/negative tests, Tracey links, and repository verification.

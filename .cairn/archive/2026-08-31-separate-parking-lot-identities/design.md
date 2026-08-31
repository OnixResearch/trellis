## Context

`ParkingLotWaiter` stores `thread_id` and `parked_key` as `u64`. `ParkingLotQueue` stores its key as `u64`, and `RawMutexState` stores its owner as `Option<u64>`. Several public functions accept key and thread values next to each other.

These modules are ordinary Rust finite models. Their verification comments provide Tracey links, but the modules are not inside `verus!`. The change therefore targets Rust API safety and preserves the current finite-model claim boundary.

## Decisions

### 1. Use two concrete semantic newtypes

**Choice:** Add `ParkKey` and `ThreadId` as separate copyable structures over `u64`.

**Rationale:** The two values have different roles and appear in adjacent arguments. Distinct types make an exchanged argument fail during compilation.

### 2. Keep construction explicit and total

**Choice:** Provide explicit raw construction and raw-value access. Do not add an implicit conversion between the two newtypes.

**Rationale:** Every `u64` remains representable because the current model has no reserved or malformed identity value. The safety property is domain separation, not numeric admission.

### 3. Migrate ownership and queue state together

**Choice:** Use `ThreadId` for waiter identity and raw-mutex ownership. Use `ParkKey` for queue identity, parked-key state, and all park, unpark, and requeue arguments.

**Rationale:** A partial migration would leave raw seams where the same mistake remains possible.

### 4. Preserve the functional core

**Choice:** Keep queue lookup, uniqueness, park, unpark, requeue, and raw-mutex transitions pure over in-memory values.

**Rationale:** Newtypes must not introduce I/O, ambient state, hidden mutation, or new runtime authority.

### 5. Require structural negative evidence

**Choice:** Add compile-fail evidence showing that a `ParkKey` cannot be passed as a `ThreadId`, and vice versa. Keep runtime negative tests for invalid queue and mutex states.

**Rationale:** Runtime tests cannot demonstrate a rejected Rust program. Compile-fail evidence directly checks the promised type boundary.

### 6. Keep the scope local

**Choice:** Do not migrate unrelated parking-lot counts, deadlines, packed state bits, or other `u64` values in this change.

**Rationale:** Those values have different proof and arithmetic trade-offs. The first change must stay reviewable.

### 7. Reduce Octet debt instead of replacing its baseline

**Choice:** Use nested compatibility-module paths, concise public record names, and two root identity components. Keep `octet-baseline.json` unchanged.

**Rationale:** Editing linted declarations changed stable finding identities. The final layout removes those findings, keeps the root module-file count stable, and produces no new Octet findings.

The recursive coverage checker now scans nested Rust modules. Its self-test covers both nested discovery and a missing source root.

## Functional core / imperative shell split

- **Pure core**: identity comparison, queue lookup, validity checks, park, unpark, requeue, and raw-mutex transitions.
- **Imperative shell**: Cargo, Clippy, Tracey, compile-fail runner, Cairn commands, and evidence capture.

## Risks / Trade-offs

- The public Rust API becomes source-incompatible for callers that pass raw integers.
- Public raw constructors still permit any numeric value. This is intentional because no numeric invariant is claimed.
- Compile-fail evidence needs a deterministic repository-owned runner or a `compile_fail` doctest.
- The change does not make the parking-lot model a Verus-verified module.

## Non-Goals

- No Verus migration.
- No scheduler, blocking, wakeup, memory-ordering, starvation, or fairness proof.
- No new packed-state representation.
- No typed deadline or timeout migration.
- No broad conversion of every parking-lot integer.

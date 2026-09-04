## Context

The accepted Trellis WebAssembly spec intentionally excludes whole-module decoding and runtime implementation. Its existing linear-memory and growth laws provide useful building blocks, but downstream deterministic runtimes also need explicit execution-budget, continuation, allocation-phase, and streaming-schedule predicates.

SpaceWasm is prior art for the requirements shape only. The new module must remain a small pure finite model with `spec fn` definitions, executable refinements, and local proof functions. It cannot depend on SpaceWasm, Wasmtime, filesystem fixtures, clocks, host callbacks, or ambient state.

## Decisions

### 1. Runtime control is a finite deterministic transition model

A runtime model will use an owned finite state, explicit deterministic step relation/table, terminal outcome enum, and bounded fuel. The model avoids trait objects or opaque callbacks so Verus can reason over every transition. Invalid transition tables are rejected by constructors or explicit validity predicates.

### 2. Fuel consumption never underflows

The fuel helper will either subtract the exact requested amount when available or return out-of-fuel with the original fuel/state unchanged. Named bounds constrain all conversions and loop budgets. Zero-step execution is an identity case.

### 3. Segmentation law is conditional and exact

For a valid deterministic transition model and identical host-observation input, running a combined admitted budget must equal running an admitted prefix and then the remaining suffix unless a terminal or host-pause boundary stops both paths at the same point. The theorem will state the exact continuation and terminal preconditions instead of claiming arbitrary scheduler equivalence.

### 4. Host pause and out-of-fuel are distinct

A host pause is produced by an explicit modeled host event; out-of-fuel is produced only by insufficient execution budget. Resume helpers accept each outcome through separate constructors and preserve its reason in evidence-facing results.

### 5. Continuations bind explicit identities

A continuation descriptor will bind generation, artifact, runtime profile, input, authority/resource context, and recorded-host-observation identities using finite typed values supplied by callers. Resume admission succeeds only on exact equality; stale facts leave the continuation unchanged and return denial.

### 6. Allocation has a phase and a cap

The allocator model tracks initialization, running, and finished phases; page capacity; and allocated pages. Allocation is monotone and capped during initialization, returns an explicit OOM/error without mutation on failure, and is rejected unchanged during running/finished phases. This models admission logic, not raw pointers or deallocation internals.

### 7. Streaming schedules prove exact byte coverage

A schedule is a finite sequence of `(offset, length)` spans over a declared total byte length. Well-formed schedules start at zero, are contiguous and ordered, use positive bounded lengths for non-empty input, and end exactly at total length. Empty input has one explicit accepted representation. Helpers prove no omission, overlap, reordering, or extra bytes.

### 8. Growth policy wraps existing memory semantics

An explicit policy decides whether `memory.grow` may invoke the existing checked growth primitive. Denial returns a typed policy result and leaves memory unchanged. Admission preserves the existing whole-page, prefix, zero-fill, and failed-no-op laws.

### 9. Proof evidence stays local and bounded

Every public exec helper will ensure equality to its spec or the named relation needed by consumers. Cross-module proofs compose only Trellis-owned models. Tests include happy and sad paths, while Verus establishes the declared finite laws.

## Functional core / imperative shell split

All new Trellis runtime-bound logic is pure functional core. Test/proof runners and evidence capture scripts are the imperative shell and must not enter the library API.

## Risks / Trade-offs

- A transition table is less ergonomic than a callback, but it makes the trusted model finite and mechanically checkable.
- Segmentation equivalence is false if host observations, transition rules, or bound identities change. Preconditions and negative tests must make that visible.
- Abstract page allocation does not prove SpaceWasm's unsafe allocators or address arithmetic.
- Exact chunk coverage does not prove decoder correctness after bytes are delivered.
- Downstream adapters may map runtime outcomes incorrectly; imported Trellis proof identity does not prove that mapping.

## Non-Goals

- No whole-module parser, decoder, validator, interpreter, or host ABI implementation.
- No vendored SpaceWasm code or proof of SpaceWasm/Wasmtime behavior.
- No pointer, allocator-unsafe, serialization, migration, scheduler-fairness, or wall-clock proof.
- No flight qualification, sandbox, production, or release claim.

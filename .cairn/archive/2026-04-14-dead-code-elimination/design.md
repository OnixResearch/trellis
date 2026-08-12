## Context

DCE is the simplest semantics-preserving optimization. It uses liveness
analysis to identify instructions whose outputs are never consumed, then
removes them. The semantics preservation proof is the template for all
optimization correctness arguments in the compiler.

## Goals / Non-Goals

**Goals:**
- Dead instruction classification from liveness
- Side-effect classification (pure vs. effectful)
- Removal preserves observable behavior
- Idempotence (one pass suffices)
- Monotonicity (removal can only create more dead code)
- Iterative convergence

**Non-Goals:**
- Dead block elimination (unreachable blocks — that's CFG cleanup)
- Dead store elimination (requires alias analysis)
- Partial dead code elimination (code sinking)
- Speculative dead code removal

## Decisions

### 1. Observable Behavior Model

**Choice:** Observable events are: (1) stores to memory, (2) return
values, (3) calls to external functions. An instruction is observable
if it directly produces an observable event or if its output is used
(transitively) by an observable instruction.

**Rationale:** Standard definition from the C/C++ abstract machine.
Pure computations whose results are unused are not observable. The proof
reduces to: the sequence of observable events is identical before and
after DCE.

### 2. Side-Effect Classification

**Choice:** `has_side_effect(instr) -> bool` classifies instructions:
- `Store`, `Call`, `Return` → true
- `Add`, `Sub`, `Mul`, `Load`, `Phi`, etc. → false

**Rationale:** Conservative classification. A pure instruction with an
unused result is dead. An effectful instruction is never dead (it's
observable by definition). Loads are tricky — they can fault — but the
model treats them as pure for the initial version.

### 3. Dead Criterion

**Choice:** Instruction I is dead iff:
1. `has_side_effect(I)` is false, AND
2. I defines some variable V, AND
3. V is not live-out at I's program point (from liveness analysis)

**Rationale:** Direct composition of side-effect classification and
liveness. The proof: since V is not live-out, no subsequent instruction
uses V. Since I has no side effect, removing I changes nothing observable.

### 4. Semantics Preservation Proof

**Choice:** Define `observable_trace(program) -> Seq<Event>` as the
sequence of observable events. Prove `observable_trace(original) == observable_trace(optimized)`.

The proof is by induction on the instruction sequence. For each
instruction: if it's not removed, the event (if any) is identical. If
it's removed, it produced no event (no side effect) and no later
instruction depends on it (not live). So removing it doesn't change any
subsequent instruction's behavior.

**Rationale:** This is the gold standard for optimization correctness.
The simulation argument shows the optimized program matches the original
step-for-step on observable behavior.

### 5. Idempotence

**Choice:** Prove: after one pass of DCE, `is_dead(I)` returns false for
every remaining instruction. Therefore a second pass removes nothing.

**Rationale:** If the liveness analysis is recomputed after removal, some
previously-live variables may become dead (their only user was removed).
However, a single pass using the original liveness information removes
all dead instructions (they're identified in one analysis). Recomputing
liveness might reveal more — that's what iterative DCE handles.

### 6. Iterative DCE

**Choice:** Repeat: compute liveness, remove dead instructions, until
no instructions are removed. Convergence: each iteration removes at
least one instruction or stops. Bounded by the number of instructions.

**Rationale:** Iterative DCE handles the cascading dead code case (A
computes a value used only by B, and B is dead → A becomes dead after
B is removed). The convergence proof is standard: instruction count
strictly decreases per iteration.

## Risks / Trade-offs

**[Loads as pure]** → A load that faults is observable (the program
crashes). Treating loads as pure means DCE might remove a faulting load,
changing the crash point. Real compilers handle this with "may-fault"
classification. The model can be refined later.

**[No alias analysis]** → Dead store elimination requires knowing that no
subsequent load reads the stored value. Without alias analysis, stores
are always conservatively live.

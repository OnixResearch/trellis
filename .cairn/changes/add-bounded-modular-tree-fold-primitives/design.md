## Context

Kamacite's evaluation profile compares an independent optimized Rust oracle with `bend run-rs` and a generated-C executable. The first workload is a finite pure balanced tree sum under explicit `u24` modulo semantics.

The motivating external cohort is:

- Bend 0.2.38, Apache-2.0, revision `814453670d0e0d6777c1313c972764dba0491b7f`.
- HVM2 2.0.22, Apache-2.0, revision `654276018084b8f44a22b562dd68ab18583bfb5b`.

These sources define the downstream evaluation context. Trellis will not parse, compile, execute, or formalize their implementations.

## Decisions

### 1. Trellis proves a local model, not external semantics

**Choice:** Define Trellis-owned `u24` and balanced-reduction semantics. Describe their relation to Bend as a downstream conformance target only.

**Rationale:** A local proof cannot establish that an external frontend or runtime implements the same model.

### 2. The arithmetic scope is deliberately small

**Choice:** Add normalization and wrapping addition only. Declare `U24_BITS`, `U24_MODULUS`, `U24_MAX`, and related conversion bounds as named constants inside the Verus-visible module.

**Rationale:** The pilot needs modular sum. Extra operations would add proof and compatibility surfaces without evidence of need.

### 3. Runtime values use a checked narrow domain

**Choice:** Represent admitted values with a runtime integer type that can hold every `u24` value. Public constructors and validators reject or normalize through explicit APIs. Public reduction entrypoints reject out-of-domain leaves before evaluation.

**Rationale:** The type representation alone does not enforce the 24-bit invariant.

### 4. The workload uses a balanced leaf schedule

**Choice:** Model the input as an ordered non-empty leaf vector whose length is a power of two and does not exceed a caller-provided limit. One reduction round combines adjacent pairs in order. Repeated rounds stop at one value.

**Rationale:** This is a finite balanced binary reduction tree with simple, reviewable grouping and no recursive Rust stack growth.

### 5. Two executable paths refine one specification

**Choice:** Implement an optimized iterative sequential oracle and a pairwise balanced reference reducer. Both entrypoints use the same admission predicate and return typed rejection on invalid input.

**Rationale:** The sequential path is the downstream oracle. The balanced path makes the tree schedule executable and testable without Bend or HVM2.

### 6. The core theorem is ordered-fold equivalence

**Choice:** Prove that each balanced round preserves the ordered modular sum. Use that lemma to prove that the final balanced result equals the sequential oracle result.

**Rationale:** Associativity permits regrouping. The proof does not need input permutation or external scheduler assumptions.

### 7. Algebraic laws are concrete

**Choice:** Prove identity, commutativity, and associativity for Trellis `u24` wrapping addition. Connect the concrete operation to existing generic algebraic predicates where useful.

**Rationale:** A generic predicate is not evidence until the concrete operation satisfies it.

### 8. Work counts remain abstract

**Choice:** For an admitted input with `n` leaves, prove that the balanced schedule performs `n - 1` abstract combines and the exact number of halving rounds. Keep these values separate from process observations and HVM interaction counts.

**Rationale:** Runtime interaction systems include implementation details that the abstract tree model does not represent.

### 9. Rejection is explicit and unchanged

**Choice:** Reject empty input, a zero limit, over-limit input, non-power-of-two length, out-of-domain leaves, and unrepresentable accounting. Rejection returns a typed reason and does not mutate caller input.

**Rationale:** Negative behavior is part of the verified API contract.

### 10. The functional core owns all decisions

**Choice:** Keep domain checks, arithmetic, reduction, accounting, and evidence DTO validation pure and deterministic. Scripts own verifier execution, file access, BLAKE3 calculation, JSON rendering, and receipt writes.

**Rationale:** Proof logic must remain testable without filesystem, process, environment, network, or clock state.

### 11. Proof evidence requires an exact downstream seam

**Choice:** Export source, requirement, proof, verifier, and proof-gap identities through the existing proof-evidence bundle. Name the executable oracle symbol and algorithm profile. Downstream consumers must pin the artifact and validate or rerun the required receipt before assigning a verified role.

**Rationale:** A copied result or unbound citation is not a proof handoff.

### 12. Kamacite keeps adoption authority

**Choice:** Trellis does not decide whether Bend or HVM2 is adopted. Kamacite compares observed external outputs with the Trellis oracle and applies its own benchmark and lifecycle policy.

**Rationale:** Trellis owns local proof facts. Kamacite owns adapter conformance and pilot decisions.

## Functional core / imperative shell split

- **Pure core**: `u24` validity and normalization, wrapping addition, admitted-input checks, ordered sum, balanced rounds, exact accounting, executable refinements, local theorems, and proof-evidence DTO validation.
- **Imperative shell**: Cargo and Verus invocation, Tracey queries, proof-evidence generation, BLAKE3 file identity, output capture, and downstream receipt storage.

## Risks / Trade-offs

- Matching the declared `u24` model does not prove that Bend implements it.
- The optimized Rust oracle remains subject to the recorded Rust compiler and Verus trusted base.
- A power-of-two balanced schedule excludes ragged and arbitrary tree shapes.
- Associativity proves regrouping equality, not arbitrary source reordering.
- Abstract combine counts do not predict HVM interactions, time, memory, or speedup.
- Downstream input decoding can map bytes to leaves incorrectly. Kamacite must validate that mapping separately.
- Trellis is MPL-2.0. Consumers must preserve the license obligations for modified covered files.

## Non-Goals

- No Bend parser, type checker, frontend, code generator, or effect analysis.
- No HVM2 interaction-net semantics, confluence, scheduler, or runtime proof.
- No generated-C or host-compiler equivalence proof.
- No CUDA, HVM3, HVM4, Kind, Wasm, or GPU integration.
- No benchmark, wall-time, memory, energy, interaction-count, or release claim.
- No proof for arbitrary binary trees, ragged reductions, subtraction, multiplication, division, or mixed numeric types.

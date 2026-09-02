# Design: Trellis proof robustness adoption

## Context

Trellis owns theorem meaning, proof source, accepted assumptions, and local proof evidence. Octet owns the reusable runner, profile, classification, and receipt contract.

## Decisions

### Decision: Use a diverse fixed pilot corpus

**Choice:** The first profile selects `src/arithmetic.rs`, `src/chain.rs`, and `src/choreography_projection.rs`.

**Rationale:** The set covers a trust-free arithmetic proof, an accepted cryptographic boundary, and a larger recursive proof surface.

### Decision: Keep proof and robustness outcomes separate

**Choice:** Existing proof receipts retain theorem status. A linked robustness receipt records seed and resource observations.

**Rationale:** A stable failed proof is not a proof. An unstable passed proof still needs review.

### Decision: Select trust mode per pilot member

**Choice:** Trust-free members use strict no-cheating mode. Members with accepted boundaries use audited-boundary mode and exact inventory linkage.

**Rationale:** One global mode would either reject intentional boundaries or hide trust expansion.

### Decision: Treat Verus proof-use output as partial

**Choice:** Preserve `-V axiom-usage-info` facts under `axiom-usage-only` coverage.

**Rationale:** The output does not establish a complete proof dependency graph or prove unused assumptions irrelevant.

### Decision: Extend the existing evidence chain

**Choice:** The robustness receipt becomes a linked child of the selected verifier receipt and source/IR identities.

**Rationale:** A separate bundle would duplicate theorem, source, and assumption identity.

### Decision: Keep Dafny indirect

**Choice:** Trellis consumes the Octet contract and does not package or execute Dafny.

**Rationale:** The adapted mechanism does not require a second verifier or compiler in Trellis.

## Functional Core and Imperative Shell

Pure Trellis code validates corpus membership, trust-mode compatibility, assumption linkage, child receipt linkage, and handoff shape.

Repository scripts and Nix checks select Octet packages, run the fixed profile, capture artifacts, and pass typed facts to validators.

## Pilot Profile

Each member records:

- module and proof scope identity
- selection class and rationale
- Octet rail selector
- trust mode and accepted boundary references
- provider metric identity
- source, proof, verifier, solver, toolchain, and command identities
- required result and non-claims

## Risks and Controls

- Large proof runs can be expensive. The profile binds run, timeout, output, and artifact limits.
- A selected file can stop being independently runnable. Preflight reports a blocked member instead of changing scope silently.
- Verus output can drift. Exact toolchain and output-contract identities make drift visible.

## Validation

Positive evidence covers each pilot class and byte-stable reruns from equal inputs.

Negative evidence covers seed-result divergence, resource-policy failure, missing boundary linkage, changed trust mode, stale source, unsupported proof-use promotion, and overclaims.

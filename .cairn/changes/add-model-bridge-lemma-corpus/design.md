## Context

Trellis builds Verus-checked primitives and a proof corpus. The stack's behavioral-model layer owns model invariants and runs an external engine over them. Code-level invariants are enforced and checked by Octet rails and by Trellis lemmas.

Bridge lemmas are the checked junction between the two layers. They must be bounded so Trellis does not become a temporal model checker or a proof of the model layer.

## Decisions

### Decision: The handshake is one documented proof boundary

**Choice:** Define a handshake contract where a model invariant (identified by its requirement ID) and a Verus lemma over a code-level invariant bind through exactly one declared proof boundary. The lemma records imported model-layer facts as assumptions and does not re-derive them.

**Rationale:** The model layer owns model facts; Trellis owns code facts. Each layer records the other as an imported assumption with an explicit identity.

### Decision: The corpus is bounded to selected surfaces

**Choice:** Start the bridge-lemma corpus on protocol surfaces that already have model invariants and code-enforced invariants, and require one lemma per selected invariant pair.

**Rationale:** A small corpus with exact boundaries is healthier than an unbounded equivalence claim.

### Decision: Lemmas carry exact provenance

**Choice:** Every lemma records the model requirement ID, the code identifier, the Verus proof identifier, and the assumption list in its metadata.

**Rationale:** Traceability from model invariant to proof artifact is the value; an unlabeled lemma cannot be audited.

### Decision: Denial is a first-class fixture

**Choice:** Include negative fixtures: an invariant pair that does not hold, a requirement ID that does not exist, and a lemma with an unrecorded assumption. The harness must fail each one.

**Rationale:** The stack requires positive and negative evidence together for every validation surface.

## Functional core / imperative shell split

- **Pure core**: handshake-contract validation, provenance metadata checks, assumption-list checks, and fixture adjudication.
- **Imperative shell**: invoke the existing Verus toolchain, collect proof results, and persist proof artifacts and metadata.

## Risks / Trade-offs

- Model and code invariants can describe the same property in different vocabularies. The handshake must name the property once and map identifiers, not duplicate meaning silently.
- Verus proof cost grows with corpus size. Keep per-surface scope explicit and add surfaces in later changes.
- Overclaim risk is high at this junction. Every lemma MUST carry the bounded non-claim.

## Non-Goals

- No temporal model checking inside Trellis.
- No arbitrary Nickel, Rust, or model semantic-equivalence claim.
- No re-proof of the model layer or the external engine.
- No new language or toolchain.

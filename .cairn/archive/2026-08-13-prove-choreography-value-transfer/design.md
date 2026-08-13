## Context

The accepted choreography model is finite and symbolic. A communication carries exact label and payload metadata. Projection and step theorems relate global and local syntax, but no state records value ownership.

The new layer must remain small enough for direct Verus reasoning. It must not model a network queue, payload bytes, storage, or memory allocation.

## Success Contract

The accepted result is a pure deterministic ownership core linked to the production choreography step. A successful transfer changes one owner. Every failure returns no replacement state.

A bounded proof blocker is an acceptable result. The change must stop if it requires a new trusted bridge or weakens an existing choreography theorem.

## Approach Registry

| Family | Mechanism | State |
| --- | --- | --- |
| Explicit ownership state | Normalized value-to-role entries plus an owned-transfer node | selected |
| Payload-tag ownership | Treat the existing payload tag as a value identity | rejected |
| Implicit source ownership | Infer ownership only from the current transfer node | rejected |
| Network queue model | Add in-flight messages and delivery state | deferred |
| Wyzer runtime port | Import the prototype transfer runtime | rejected |

## Decisions

### Decision: separate value identity from payload type

**Choice:** Add a distinct `ValueId` domain. Keep `PayloadTag` as symbolic payload-type metadata.

**Rationale:** Two values can share one payload type. Mixing the identities would make stale-value and wrong-type errors indistinguishable.

The representation decision must use the evidence from `pilot-choreography-payload-tag-newtype`. A transparent structure is preferred when it gives a real compile-time distinction without adding a refinement claim.

### Decision: normalize ownership as finite exact data

**Choice:** Represent ownership as a finite sequence of entries in strict value-identity order. Each value identity appears exactly once.

**Rationale:** Normalization gives deterministic equality, duplicate rejection, and bounded pointwise proofs without hash iteration.

Admission rejects duplicate values. It does not require every possible `ValueId` to appear.

### Decision: add a separate owned-transfer syntax node

**Choice:** Keep symbolic `Comm` unchanged. Add an explicit global owned-transfer node and matching local send-owned and receive-owned nodes.

**Rationale:** A symbolic message does not always move an owned value. Separate variants keep ownership effects visible and avoid optional fields that can create invalid combinations.

The transfer head contains exact source, destination, label, payload tag, value identity, and continuation fields.

### Decision: make the semantic transfer atomic

**Choice:** The global owned-transfer step requires the source to own the selected value. Success replaces that owner with the destination and advances to the continuation.

There is no intermediate in-flight ownership state in this change.

**Rationale:** The theorem concerns choreography boundaries. Runtime send, acknowledgement, crash, and uncertainty states belong to consumers.

Failure returns `None`. The caller-held choreography and ownership state remain unchanged.

### Decision: preserve projection shape exactly

**Choice:** Projection maps an owned-transfer head to send-owned for the source, receive-owned for the destination, and the projected continuation for uninvolved roles.

Merge accepts shared transfer heads only when peer, label, payload tag, value identity, and direction match exactly. Every other transfer-head conflict fails.

**Rationale:** Dropping the value identity or weakening equality would break ownership correspondence.

### Decision: prove conservation and exclusion

**Choice:** Production-linked proofs establish these facts for every successful transfer:

- the selected value owner changes from source to destination;
- the former owner no longer owns that value;
- every other value owner is unchanged;
- normalization and entry count are preserved;
- a stale source cannot transfer the value again; and
- projected sender and receiver heads identify the same transfer.

**Rationale:** These are the smallest reusable facts needed by a role-located frontend.

### Decision: retain narrow claim boundaries

**Choice:** Evidence describes an atomic semantic owner change over supplied finite facts.

It does not claim packet delivery, storage mutation, memory deallocation, unique external observations, exactly-once effects, or whole-protocol liveness.

**Rationale:** Those properties require runtime and transport models outside Trellis choreography syntax.

## Functional Core and Shell

The pure core owns ownership admission, lookup, transfer decisions, projection, merge, steps, and proof functions.

The shell owns command execution, fixture compilation, Verus invocation, Tracey queries, Cairn gates, and evidence capture.

## Validation Strategy

Positive evidence covers normalized ownership, a successful transfer, chained transfers, return transfer, projection, merge, duality, and unrelated-value preservation.

Negative evidence covers duplicate values, missing values, self-transfer, wrong source, stale source, wrong peer, wrong value identity, mismatched payload tag, conflicting merge heads, and use after transfer.

Property tests generate normalized ownership states and admitted transfers. They compare the executable result with an independent pointwise owner-update oracle.

Verus must prove the production transfer result against the ownership specification and preserve all prior choreography theorems.

## Risks and Trade-offs

- Recursive choreography syntax can amplify proof and trigger churn.
- A value newtype can have the same migration cost found by the payload-tag pilot.
- Atomic transfer is intentionally weaker than a crash-aware runtime protocol.
- Separate transfer variants expand merge and step case analysis.

## Non-Claims

This design does not prove transport delivery, network ordering, persistence, authorization, runtime cleanup, byte uniqueness, Rust compiler correctness, verifier soundness, distributed deadlock freedom, production readiness, or release eligibility.

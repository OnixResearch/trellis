## Context

verified-logic already contains verified pieces that show up inside
consistency protocols: vector clocks, quorums, epochs, leases, snapshot cuts,
MVCC helpers, and causal delivery properties. What is missing is the layer
above those primitives: named consistency models over histories and the
refinement edges between them.

The Jepsen consistency-models tree gives a compact scope boundary. It covers
common transactional levels, single-object real-time levels, and session
orders used by geo-replicated systems. This change captures that taxonomy
first so later Verus work has a stable target.

## Goals / Non-Goals

**Goals:**
- Specify all 16 models present in Jepsen's `/consistency/models` tree
- Capture the refinement edges between those models
- Capture the availability envelope Jepsen assigns to each model family
- Distinguish nearby models by both required guarantees and allowed behaviors
- Leave a clear path to later Verus predicates and implication proofs

**Non-Goals:**
- Encode every page under `/consistency/phenomena` or `/consistency/dependencies`
- Prove CAP-style impossibility results in this change
- Implement a history checker or verifier in this change
- Resolve every naming disagreement in the literature beyond the choices Jepsen
  already makes

## Decisions

### 1. Use the Jepsen Tree as the Canonical Taxonomy

**Choice:** The change follows the model names and implication edges from
`https://jepsen.io/consistency/models`.

**Rationale:** The Jepsen tree is explicit about which levels it includes and
how they relate. Using one external taxonomy keeps naming drift out of the
repo. Jepsen's page title for `/strict-serializable` is "Strong
Serializability"; this change uses the more common name "Strict
Serializability" and treats the two names as aliases.

**Implementation:** The OpenSpec domain mirrors the 16 pages in the tree and
adds explicit hierarchy and availability requirements.

### 2. Model Everything as Predicates Over Shared History Relations

**Choice:** Future implementation work should define shared history relations
once: session order, real-time order, visibility, arbitration, conflicts,
snapshot membership, and happens-before.

**Rationale:** These models mostly differ in which relations they constrain.
If each model invents its own local vocabulary, the implication proofs will be
harder to state and harder to trust.

**Implementation:** This spec names the behavioral requirements now. Later
Rust modules can map those requirements to common spec predicates and proof
functions.

### 3. Keep Transactional, Object, and Session Families Separate

**Choice:** The spec distinguishes three families:
- transactional multi-object models
- single-object total-order models
- session and partial-order guarantees

**Rationale:** The models reuse some ideas but do not all speak about the same
kind of operation. Transactions reason about snapshots and predicates;
linearizability and sequential consistency reason about single-object
operations; session guarantees reason about one client's local order.

**Implementation:** The requirements stay in one domain, but the design and
later code should preserve those family boundaries. Strict serializability
bridges transactional and real-time object semantics; causal consistency
bridges ordered object semantics and session guarantees.

### 4. Allowed Behaviors Are Part of the Spec

**Choice:** Each model requirement includes at least one distinguishing allowed
behavior or omitted guarantee, not just the strongest guarantee it provides.

**Rationale:** Nearby models differ as much by what they permit as by what
they forbid. Repeatable read vs snapshot isolation, or PRAM vs causal
consistency, are hard to separate if the spec only lists headline promises.

**Implementation:** The spec uses paired scenarios: one for the key guarantee,
and one for a behavior the model may still allow.

### 5. Availability Is Recorded as Classification, Not as a Safety Predicate

**Choice:** The OpenSpec domain records total-availability, sticky-availability,
and non-total-availability classifications, but does not treat them as
ordinary per-history predicates.

**Rationale:** Availability under partitions is a property of an
implementation and failure model, not just a single safety history. Recording
those envelopes in the spec is still valuable because they are part of the
Jepsen tree's meaning.

**Implementation:** A dedicated requirement captures the availability classes
for the modeled levels. Later implementation may expose them as metadata or
documentation, rather than proof obligations over history predicates.

### 6. Use Jepsen's Generalized Snapshot Isolation

**Choice:** Snapshot isolation follows Jepsen's generalized reading of SI,
not stricter real-time or per-session variants.

**Rationale:** Jepsen places snapshot isolation strictly below serializability
and explicitly notes that stronger timestamp constraints create other variants.
This matters for the implication graph and for allowed anomalies like write
skew.

**Implementation:** The SI requirement focuses on consistent snapshots,
first-committer-wins on overlapping writes, and the fact that write skew may
still occur.

## Risks / Trade-offs

**[Literature names vary]** -> Different papers fold or split these models in
slightly different ways. This change anchors the names and edges to Jepsen's
tree and calls out the strict/strong serializability alias explicitly.

**[Availability is not machine-checked yet]** -> The spec records the
availability envelope now, but later implementation work will still need to
decide how much of that can be represented as proof-carrying metadata versus
plain documentation.

**[Overlap with existing distributed specs]** -> Some existing specs already
mention causal delivery, MVCC, or snapshot behavior. This domain stays at the
history-model level so those lower-level modules can refine into it instead of
duplicating it.

**[Transactions need richer future vocabulary]** -> Multi-object models refer
to predicates, snapshots, and conflict relations that are not yet shared in
`src/`. That extra modeling work is expected and is reflected in the open
implementation tasks.

## Context

Invocation decode is the point where raw requests become typed kernel operations. A standalone change keeps the soundness proof focused: decode should accept only well-typed, rights-authorized operations and never invent authority.

## Goals / Non-Goals

**Goals:**
- Decode only well-typed authorized invocations.
- Reject wrong-type or under-authorized requests.
- Never invent authority beyond the supplied capability context.

**Non-Goals:**
- Raw register/word ABI details
- Actual execution semantics of the decoded invocation
- Scheduler outcomes

## Decisions

### 1. Decode from abstract messages

**Choice:** Use `InvocationMessage { label, arg_count, word0, word1, word2, word3, has_extra_cap }`
**Rationale:** The semantic proof is about authorization and typing, not machine-word layout. `arg_count` and `has_extra_cap` are enough to model malformed-message rejection without recreating the raw syscall ABI.

### 2. Use looked-up capabilities as explicit inputs

**Choice:** Use `DecodeContext { primary, secondary }` where each supplied capability is an abstract `Capability { kind, object, rights }`
**Rationale:** That makes no-invented-authority a crisp contract and avoids coupling decode to a global CSpace model.

### 3. Pin the starter-set invocation sum now

**Choice:** Scope this change to exactly four variants: `EndpointSend`, `EndpointReceive`, `FrameMap`, and `UntypedRetype`
**Rationale:** This covers the nearby landed seL4 modules and exercises the existing rights gates `can_send`, `can_receive`, `can_map`, and `can_control` without letting the change expand into full syscall coverage.

### 4. Make "no invented authority" structural

**Choice:** For this change, "authority reachable from context" means the decoded invocation may mention only object identifiers already carried by the supplied `primary`/`secondary` capability records.
**Rationale:** The active spec is about decode, not derivation-tree reachability. Structural containment keeps the proof local and matches the explicit-capability decode surface.

### 5. Return typed invocations or failure

**Choice:** Return `Option<Invocation>` where `None` means malformed, wrong-type, or under-authorized
**Rationale:** The postcondition cleanly distinguishes success from rejection and keeps the proof surface small.

### 6. Verification strategy

**Choice:** Prove three local contracts in `src/sel4_invocation.rs`: successful decode implies type-correct + authorized output, invalid requests decode to `None`, and successful outputs use only context authority.
**Rationale:** This matches the repo pattern: one exact exec/spec pair plus small named proof lemmas. The defining helpers and tests carry `r[impl]` / `r[verify]` anchors, and `docs/requirements.md` mirrors the three requirement IDs because Tracey reads that file rather than the change-local spec.

## Risks / Trade-offs

**[Too much ABI detail can swamp the proof]** Keep messages abstract and typed only by the fields needed for authorization.
**[Starter-set too vague reopens review]** Pin the initial four invocation variants in both design and tasks before implementation.
**[Future full decode needs richer authority reasoning]** Treat this module as a small exact starter set; if later work needs transitive CSpace reachability, add it in a follow-on change instead of stretching this proof surface.

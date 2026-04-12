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

**Choice:** Decode from abstract messages
**Rationale:** The semantic proof is about authorization and typing, not machine-word layout.

### 2. Use looked-up capabilities as explicit inputs

**Choice:** Use looked-up capabilities as explicit inputs
**Rationale:** That makes no-invented-authority a crisp contract.

### 3. Return typed invocations or failure

**Choice:** Return typed invocations or failure
**Rationale:** The postcondition should distinguish accepted operations from rejected ones cleanly.

## Risks / Trade-offs

**[Too much ABI detail can swamp the proof]** Keep messages abstract and typed only by the fields needed for authorization.
**[Object-type coverage may grow quickly]** Start with a small invocation sum and extend incrementally.

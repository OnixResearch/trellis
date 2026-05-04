## Context

The primitive taxonomy now includes finite resource-accounting, constraint, and
optimization families. Access-control policy is adjacent but distinct: it needs
reusable finite principals, resources, actions, rules, decisions, conflicts,
delegations, and authorization envelopes for later systems to compose without
claiming cryptographic or whole-system security.

## Goals / Non-Goals

**Goals:**

- model principals, resources, actions, and rules as finite admitted sets
- keep authorization decisions deterministic and explainable by matched metadata
- bound delegation by explicit finite scope and depth metadata
- provide implementation and evidence for the focused finite policy slice

**Non-Goals:**

- cryptographic identity, signature, key ownership, or authentication strength
- whole-system security, compliance, information-flow, or side-channel claims
- external policy-engine correctness or distributed authorization consensus
- runtime enforcement guarantees outside the finite policy decision model

## Decisions

### 1. Policy subjects and objects are finite admitted values

**Choice:** Principal, resource, and action primitives use finite admitted ID
sets or exact metadata envelopes.

**Rationale:** Finite admission keeps rule matching and authorization decisions
checkable with scans and reusable set/sequence primitives.

**Alternative:** Treat identity strings or external DIDs as inherently valid.
Rejected because external identity/authentication correctness is out of scope.

### 2. Rules are scoped envelopes, not security guarantees

**Choice:** Policy rules describe explicit principal/resource/action scopes,
effect metadata, priority/conflict metadata, and optional conditions in exact
finite form.

**Rationale:** This lets later authorization checks prove deterministic decision
selection without claiming broader security properties.

**Alternative:** Specify full policy language semantics. Rejected until a focused
policy-language OpenSpec owns parsing, evaluation, and conformance.

### 3. Delegation is bounded and structural

**Choice:** Delegation primitives carry finite delegator/delegate/resource/action
scope metadata and explicit depth or chain bounds.

**Rationale:** Bounded structural delegation can be verified without relying on
cryptographic proof of possession or external trust roots.

**Alternative:** Claim real-world capability security. Rejected because that
requires cryptographic and operational models outside this family.

## Risks / Trade-offs

**Security overclaiming** → Requirements must state finite policy-decision
properties only and must not claim authentication strength or whole-system safety.

**Overlap with resource accounting** → Resource accounting owns budgets/quotas;
this change owns policy authorization decisions over admitted resources/actions.

**Overlap with crypto/capabilities** → Crypto modules own signatures/keys; this
change owns structural policy/delegation metadata and deterministic decisions.

**Tracey drift** → This implementation slice adds `docs/requirements.md`,
`r[impl]`, `r[depends]`, and `r[verify]` tags with the code and evidence.

## Validation Plan

- `openspec validate add-access-control-policy-primitives --strict --json`
- strict validation for every active primitive-family OpenSpec
- helper verification for expected incomplete implementation tasks
- focused cargo/Verus/Tracey checks before archive

## Context

The choreography modules are inside `verus!`. Their current aliases keep proof terms simple, but they also make role identifiers, branch labels, and payload tags interchangeable at type-check time.

The payload tag is the smallest useful pilot. It appears next to a label in communication nodes, but it does not participate in arithmetic. The migration therefore tests equality-only newtype ergonomics without combining the experiment with role sets or label-order proofs.

## Decisions

### 1. Pilot one domain only

**Choice:** Replace only `PayloadTag`. Keep `RoleId` and `LabelId` as `u32` aliases.

**Rationale:** A single-domain pilot gives bounded evidence. It avoids converting the whole recursive proof corpus before Verus behavior is known.

### 2. Use a transparent semantic structure

**Choice:** Define `PayloadTag` inside the Verus-visible block as a copyable structure with one public raw `u32` field.

**Rationale:** The pilot needs type separation, not numeric admission. A public field keeps construction and proof unfolding explicit and avoids pretending that the wrapper proves a refinement invariant.

### 3. Preserve malformed-state expressiveness

**Choice:** Admit every `u32` payload value. Do not add a private validated constructor or a well-formedness predicate.

**Rationale:** Existing choreography requirements impose no numeric payload-tag restriction. The wrapper must not narrow the theorem domain.

### 4. Keep payload equality exact

**Choice:** Projection, merge, step, and duality proofs compare the exact typed payload tag. Raw projections are used only where a raw value is an explicit boundary requirement.

**Rationale:** Repeatedly erasing the type to `.raw` inside semantic predicates would remove the proof-level value of the migration.

### 5. Require proof-preserving completion

**Choice:** The pilot is complete only when all existing choreography tests and full-library Verus verification pass with no new trusted bridge, external body, or assumption.

**Rationale:** A type-safety improvement that weakens the proof corpus is not acceptable.

### 6. Test the structural claim directly

**Choice:** Add deterministic compile-fail evidence for supplying `LabelId` or raw `u32` where `PayloadTag` is required. Add positive tests for explicit payload-tag construction and preservation.

**Rationale:** Verus success proves semantic preservation. Compile-fail evidence proves the Rust type boundary.

### 7. Use an explicit stop or continue gate

**Choice:** Record the number and kind of required proof repairs. Continue to `RoleId` or `LabelId` only through a later Cairn change.

**Rationale:** The practical newtype rule requires proof value to exceed projection, trigger, arithmetic, and migration churn.

## Functional core / imperative shell split

- **Pure core**: typed choreography syntax, projectability predicates, projection, merge, steps, and proof functions.
- **Imperative shell**: Cargo and Verus invocation, compile-fail execution, Tracey queries, Cairn commands, and evidence capture.

## Risks / Trade-offs

- Recursive structures and quantified sequences may require widespread annotation changes despite the small runtime type change.
- Derived equality can be brittle in some Verus models. Explicit field comparison may be needed at narrow helper boundaries.
- Public API users must update choreography literals.
- A successful payload pilot does not establish acceptable cost for role or label migration.

## Non-Goals

- No `RoleId` or `LabelId` migration.
- No new payload validation rule.
- No choreography semantic change.
- No trusted proof bridge.
- No broad policy requiring newtypes for every identifier.

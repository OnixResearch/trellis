# Choice-agreement scope note

Reviewer flag: the closed theorem in `src/properties.rs` now reasons about the first branch-dependent point exposed by the projection-side merge, rather than an arbitrary later `Offer` reachable in the full endpoint projection.

This is intentional and matches the shipped requirement text:

- `docs/requirements.md:r[properties.choreography-choice-agreement]`
- `openspec/changes/choreography-primitives/specs/choreography-properties/spec.md`

Both now require:

- exact decider `InternalChoice` labels,
- exact preservation of any shared local prefix produced by projection-side merge,
- exact label agreement at the **first exposed `Offer` from the decider, if one appears**.

Reason: with nested/shared-prefix cases, a stronger theorem over arbitrary later offers in the fully merged endpoint projection is false unless the model carries additional structure beyond the current projection/merge semantics. The implemented proof closes the intended finite guarantee for this change: shared prefixes stay intact, and the first branch-dependent decider-controlled offer exposes the stored outer global choice labels.

Possible follow-up: if the project wants a stronger endpoint-level theorem later, add a separate change that strengthens the projection model/spec and proves a bridge from the first branch-dependent offer theorem to a full `project_endpoint_spec(...)` statement.

## Context

HIR lowering removes syntax irrelevant to later analysis and introduces HIR nodes under owning items. The guide names several invariants: created `HirId`s must be used, lowering must occur in the owning item scope, AST `NodeId`s placed in HIR must be lowered, and synthetic nodes must receive fresh IDs.

Trellis should model those invariants as finite maps and owner scopes rather than model the entire Rust frontend.

## Decisions

### 1. Make owners explicit

**Choice:** Every HIR ID in the primitive carries an owner identifier and a local identifier. Admission checks that lowered nodes are created in the declared owner scope.

**Rationale:** Owner-scoped uniqueness is the key HIR invariant and can be checked without a compiler context.

### 2. Track created IDs and used IDs separately

**Choice:** The lowering record contains created HIR IDs, used HIR IDs, and AST-to-HIR mapping entries. Admission requires created IDs that are intended for HIR structures to be used.

**Rationale:** This models the rustc sanity check while still allowing explicit placeholder/discard metadata when the model needs it.

### 3. Distinguish lowered source IDs from synthetic IDs

**Choice:** Source `NodeId`s must map through the lowering table. Synthetic nodes must use fresh generated IDs absent from the source mapping domain.

**Rationale:** This prevents accidental aliasing between user-written AST nodes and desugaring-created HIR nodes.

### 4. Keep desugaring shape shallow

**Choice:** The first primitive may include finite provenance records for desugared constructs, but it does not prove exact `for`-loop, `impl Trait`, or match lowering semantics.

**Rationale:** ID invariants are reusable and smaller than full frontend lowering correctness.

### 5. Treat DefId correspondence as metadata

**Choice:** If an item requires a definition identifier, the primitive checks a finite NodeId-to-definition correspondence table but does not compute crate-global DefIds.

**Rationale:** DefId creation depends on compiler-global collection. A finite correspondence predicate is sufficient for local lowering invariants.

## Risks / Trade-offs

- The model may be too narrow for complex desugarings initially. That is acceptable; later changes can add semantic lowering predicates.
- The distinction between intentionally lowered-but-unused and invalid created IDs must be explicit to avoid false positives.

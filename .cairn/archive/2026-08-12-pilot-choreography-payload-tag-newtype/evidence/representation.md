# Payload-tag representation

The pilot uses this representation inside the Verus-visible choreography core:

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PayloadTag {
    pub raw: u32,
}
```

The public field preserves all `u32` values. The type adds no numeric admission rule.

`RoleId` and `LabelId` remain `u32` aliases. The change does not migrate either domain.

Projection, merge, step, and proof predicates compare `PayloadTag` values directly. They do not erase the type through `raw` projections.

The public compile-fail example passes a `LabelId` to a function that requires `PayloadTag`. The Rust compiler rejects this substitution.

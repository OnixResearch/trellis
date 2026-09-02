## Why

Trellis owns the Verus-checked logic building blocks and proof corpus. It proves Rust-level functional facts. The stack model layer checks protocol-level invariants over a typed behavioral model, and the code layer enforces invariants by construction. Nothing currently connects a code-enforced invariant to a model invariant with a documented, checked handshake.

Without a bounded bridge, the model check and the code proof live in separate worlds, and the handshake between them is asserted rather than evidenced.

## What Changes

- Define the bridge-lemma handshake contract: a model invariant referenced by its requirement ID and a Verus lemma over a code-level invariant bind through one documented proof boundary.
- Add a bounded corpus of bridge lemmas for at least one protocol surface, each recording the model requirement ID, the code identifier, and the proof boundary.
- Record imported model-layer facts as explicit assumptions in each lemma; do not re-prove the model layer inside Trellis.
- Add positive lemmas and negative denial fixtures: an invariant that does not hold, a mismatched requirement ID, and an unrecorded assumption.

## Impact

- **Files**: Bridge-lemma handshake contract, corpus for the first protocol surface, fixtures, and docs.
- **Testing**: Positive verified lemmas plus negative denied, mismatched, and unrecorded-assumption cases.
- **Boundary**: A bridge lemma proves that the code invariant implies the model invariant under the recorded assumptions. It does not prove model validity, engine correctness, or arbitrary model-code semantic equivalence.

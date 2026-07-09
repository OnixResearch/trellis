## Design

Define a Preserves proof-manifest envelope row with source manifest digest, primitive/proof manifest version, selected row set, canonical Preserves digest, schema label, allowed consumers, and non-claims. The pure checker validates row linkage and claim bounds. The shell measures manifests and serialized envelopes.

The envelope carries proof-manifest identity only. It does not change Verus verification, proof artifact trust boundaries, or downstream authorization semantics.

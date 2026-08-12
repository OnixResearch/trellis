# OpenSpec Gate Summary

Fresh reruns on 2026-04-23 after reconciling the artifacts to the landed
`src/ucan_did.rs` surface:

- **Proposal gate**: PASS (`openspec_gate stage=proposal change=ucan-did-model`)
- **Design gate**: PASS (`openspec_gate stage=design change=ucan-did-model`)
- **Tasks gate**: PASS (`openspec_gate stage=tasks change=ucan-did-model`)

## Reconciled findings

The final pass reflects these artifact fixes:
- spell out the canonical framing contract end-to-end as
  `did:key:z ++ base58btc(varint(codec) ++ key_bytes)`
- capture `DidKey` as the encode-side carrier that preserves the exact
  input key bytes, codec, and encoded bytes
- make the public API boundary explicitly byte-oriented and encode-only,
  with DID decode remaining spec-only
- document the proof-side trusted base58 injectivity helper used by the
  DID injectivity proof
- map every scenario ID to a concrete proof, test, or named evidence file,
  including `test_varint_decode_truncated_continuation`
- align Phase 3 validation tasks with the final scenario set
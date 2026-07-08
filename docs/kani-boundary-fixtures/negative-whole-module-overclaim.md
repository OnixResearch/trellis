# Negative Kani whole-module fixture

## Reference: negative-whole-module-overclaim

- Classification: complementary-external-evidence
- Source scope: src/ucan_signature.rs::sign_checked runtime wrapper harness
- Receipt identity: blake3:4f01d6f7c4a4410d9b25c9c99f91cbb98d0f1bb23bde6fd34e4f9b4d7f5f0004
- Property class: panic-freedom
- Trellis proof artifact: Verus proof artifact remains authoritative for checked byte-length admission.
- Claim: Kani proves the entire module correct and closes the trusted boundary.
- Non-claims: not a Verus proof artifact; not ghost-spec theorem evidence
- Expected diagnostic: claim must not promote scoped Kani evidence to whole-module correctness

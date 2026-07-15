## Tasks

- [x] [serial] Define the `verified-logic` consumer release profile manifest and supported UCAN-facing module set. r[trellis.crate_packaging.verified_logic_consumer_profile]
- [x] [serial] Add positive fixtures for direct dependency metadata and an aligned UCAN vendored snapshot. r[trellis.crate_packaging.verified_logic_consumer_profile.positive]
- [x] [serial] Add negative fixtures for stale revision, missing revision metadata, local patches without exception, missing selected modules, and weakened non-claims. r[trellis.crate_packaging.verified_logic_consumer_profile.negative]
- [x] [serial] Implement pure profile/snapshot compatibility validation. r[trellis.crate_packaging.verified_logic_consumer_profile.validation]
  - PASS: local and Nix profile checks validate 39 modules, 42 selected artifacts, two positive fixtures, eight negative fixtures, in-memory positive/negative self-tests, the immutable release source revision, and BLAKE3-bound direct and vendored compatibility reports.
- [x] [serial] Document direct-dependency guidance and fallback vendoring requirements for UCAN and similar consumers. r[trellis.crate_packaging.verified_logic_consumer_profile.docs]
- [x] [serial] Run focused packaging tests, `verified-logic` checks, and Cairn validation/gates. r[trellis.crate_packaging.verified_logic_consumer_profile.final_validation]
  - PASS: the dev-shell full verifier reports `ALL CHECKS PASSED`; pure `nix flake check -L --option secret-key-files ''` passes 4,871 nextest cases plus Clippy, Verus, Tracey, Octet, and immutable-source profile checks; policy-compatible Cairn validation and current proposal, design, and tasks gates pass. Current-schema root validation remains blocked by eleven unrelated active changes with duplicate requirement IDs.

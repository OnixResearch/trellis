## Tasks

- [ ] [serial] Define the `verified-logic` consumer release profile manifest and supported UCAN-facing module set. r[trellis.crate_packaging.verified_logic_consumer_profile]
- [ ] [serial] Add positive fixtures for direct dependency metadata and an aligned UCAN vendored snapshot. r[trellis.crate_packaging.verified_logic_consumer_profile.positive]
- [ ] [serial] Add negative fixtures for stale revision, missing revision metadata, local patches without exception, missing selected modules, and weakened non-claims. r[trellis.crate_packaging.verified_logic_consumer_profile.negative]
- [ ] [serial] Implement pure profile/snapshot compatibility validation. r[trellis.crate_packaging.verified_logic_consumer_profile.validation]
- [ ] [serial] Document direct-dependency guidance and fallback vendoring requirements for UCAN and similar consumers. r[trellis.crate_packaging.verified_logic_consumer_profile.docs]
- [ ] [serial] Run focused packaging tests, `verified-logic` checks, and Cairn validation/gates. r[trellis.crate_packaging.verified_logic_consumer_profile.final_validation]

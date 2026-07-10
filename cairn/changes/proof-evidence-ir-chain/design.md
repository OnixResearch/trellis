## Design

Trellis emits a proof evidence chain payload with three linked layers: source envelopes for exact Verus/Rust bytes, a canonical proof evidence IR for typed theorem/property structure, and a verifier receipt for the actual Verus/toolchain run. The evidence-chain handoff links all three with BLAKE3 identities and explicit non-claims.

### Decisions

- **Use source and IR together.** Source envelopes remain required for review and reruns; the proof IR is the canonical query/linkage payload.
- **Verifier receipt owns proof result.** The IR alone does not prove anything. Proof status is valid only when tied to verifier/toolchain identity, command identity, inputs, assumptions, and result.
- **Reference-only is distinct.** Imported or archived Trellis proof artifacts can be recorded-only until rerun under the consuming context.
- **Assumptions are first-class.** Assumptions, trusted boundaries, external bodies, and proof-gap references are typed fields, not prose footnotes.
- **Canonical handoff is profile-specific.** Trellis owns proof-specific fields; Kamacite owns canonical Preserves envelope bytes.
- **No downstream overclaim.** Trellis proof evidence proves only declared local properties under declared assumptions and never whole downstream system correctness or release eligibility.

### Validation shape

Positive fixtures cover a verified local proof, a recorded-only reference proof, and stable source+IR+verifier chain hashes. Negative fixtures cover stale source hash, missing verifier identity, missing assumptions, unsupported verifier status, malformed requirement/property IDs, and semantic/release overclaims.

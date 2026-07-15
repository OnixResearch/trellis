## Design

The consumer release profile is a packaging and evidence contract. It does not change the verified primitives themselves. It records how a downstream Rust crate may consume Trellis `verified-logic` and how a reviewer can compare vendored fallback snapshots against the Trellis source of truth.

### Decisions

1. **Trellis owns proof semantics.** A consumer may cite Trellis `verified-logic` proof artifacts only as imported verified primitive evidence. The consumer must still validate its own shell, token parsing, key resolution, storage, networking, clocks, revocation, replay, and policy semantics.

2. **Direct dependency is preferred.** The profile names a direct crate dependency path or released coordinate as the normal route. Vendoring remains a fallback only when release, offline, or toolchain constraints require it.

3. **Vendored fallback is receipt-backed.** A vendored snapshot must carry source repo, revision, selected paths, dirty-state policy, and compatibility digest inventory. Snapshot drift without an explicit exception fails closed.

4. **Feature and module surface are explicit.** The consumer profile names the supported feature set and the modules relied on by UCAN-facing bridges, instead of requiring downstream reviewers to audit the entire source tree by hand.

5. **No downstream certification claim.** Passing the profile or snapshot comparison proves source/profile alignment only. It does not prove UCAN token correctness, downstream authorization policy, external consumer readiness, verifier soundness beyond Trellis's stated proof boundary, or whole-system security.

### Validation shape

A pure profile checker compares a Trellis release/profile manifest with consumer dependency metadata or a vendored snapshot manifest. Positive tests cover direct dependency metadata and an aligned vendored snapshot. Negative tests cover stale revision, missing source repo, missing selected modules, modified files without exception, unverified local additions, and non-claim weakening.

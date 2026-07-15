# Verified-logic consumer release profile

Trellis owns the proof semantics and source identity for `verified-logic`.
[`consumer-profile/profile.ncl`](../consumer-profile/profile.ncl) is the typed
Nickel source of truth for the current UCAN-facing consumer release, while
[`consumer-profile/generated/profile.json`](../consumer-profile/generated/profile.json)
is its checked runtime projection.

## Preferred direct dependency

**Direct dependency is the normal consumption route.** Consumers should use the
exact HTTPS repository and immutable revision named by the profile, disable
default features, and request only the listed supported features. The current
profile has no supported Cargo features:

```toml
[dependencies]
verified-logic = {
  git = "https://github.com/OnixResearch/trellis",
  rev = "1f03111b92cc36aade07359c96773e086908c95f",
  default-features = false,
}
```

A direct consumer manifest must repeat the package name/version, exact dependency
coordinate, source repository, revision, feature decision, complete selected
module list, and required non-claims. The Nix profile check resolves the locked
`trellis-consumer-release-src` input at that same revision and measures the
selected files from that immutable source, rather than trusting the current
worktree alone. Passing compatibility means those values match the reviewed
Trellis profile; it does not execute or certify consumer code.

## Audited vendored fallback

**Vendoring is an audited fallback** for bounded offline, release, or toolchain
constraints. A fallback snapshot must record:

- the exact Trellis source repository and immutable revision;
- `vendor/verified-logic` as its snapshot root;
- every selected UCAN-facing module from the profile;
- the selected `Cargo.toml`, `LICENSE`, crate root, and module file byte counts
  and BLAKE3 digests;
- the `deny_unrecorded_changes` dirty-state policy;
- every local patch or addition, with an explicit approved exception, rationale,
  verification evidence, and a boundary stating that it does not extend imported
  Trellis proof claims; and
- all profile-required non-claims.

An absent or stale revision, missing module, changed digest, unapproved patch,
unverified local addition, or weakened non-claim fails closed. Snapshot alignment
covers only the inventoried paths; it is not a claim that unlisted consumer files
or external services match Trellis.

The selected `verified-logic` files remain under `MPL-2.0`. Vendored consumers
must preserve the license and applicable notices; package alignment does not
change the license of separate consumer files.

## Compatibility reports

Validate the checked profile and its positive/negative fixture matrix:

```bash
./scripts/check-verified-logic-consumer-profile.rs
./scripts/check-verified-logic-consumer-profile.rs --self-test
```

Generate a deterministic report for a Nickel-authored consumer manifest:

```bash
./scripts/check-verified-logic-consumer-profile.rs \
  --manifest consumer-profile/examples/ucan-direct-dependency.ncl \
  --report target/verified-logic-consumer-report.json
```

The `trellis-verified-logic-compatibility-report/v1` report binds the release ID,
source revision, mode, selected counts, diagnostics, supported claim, and
non-claims. Its BLAKE3 identity covers the
canonical in-memory report fields before envelope serialization.

## Proof and authority boundary

A passing report supports only source and profile alignment for imported Trellis
verified primitives. In particular, imported primitives:

- do not prove UCAN token correctness;
- do not prove downstream authorization policy;
- do not prove verifier soundness beyond Trellis's stated boundary;
- do not prove external consumer readiness; and
- do not prove whole-system security.

Consumers still own parsing, cryptography integration, key and proof resolution,
clocks, storage, networking, caveat policy, revocation freshness, replay
persistence, shell orchestration, and their own release evidence.

## Refreshing the profile

A Trellis maintainer may advance the release revision only after the selected
source files and proof boundary pass the repository's verified-logic checks.
Update the Nickel profile, regenerate the JSON projection with `nickel export`,
refresh aligned fixtures, run the profile checker, and then run the full Trellis
verification rail. Consumer-owned sync tools such as UCAN's Josh check remain
additional local evidence; they do not replace this Trellis-owned contract.

# Standalone Nickel export cutover

Trellis uses `OnixResearch/nickel-export` revision `257fafc1c746f1faf156207043a4c826bfb16d49` with Nickel `1.17.0`.

The Nix rail supplies both binaries from immutable inputs. Local executable paths are diagnostic inputs only and cannot satisfy the release rail.

## Authority boundary

The standalone shell owns request admission, external Nickel execution, exact BLAKE3 identities, manifests, structured errors, writes, and freshness checks.

Trellis retains each policy, contract, selector, destination, invalid fixture, Verus gate, Tracey gate, proof claim, and release decision.

No verified module or pure proof model imports or invokes the exporter. The checked authority baseline rejects new filesystem, environment, or process access in library source.

## Checked profile

`nickel-export-cutover/profile.ncl` is the typed source of truth. It generates checked requests, manifests, contract outputs, and an authority baseline under `nickel-export-cutover/generated/`.

All requests use `contract = ""`. This value preserves the legacy source-level contract behavior and the exact output bytes.

The profile covers these output families:

- three JSON policy outputs;
- one YAML lifecycle output and its compatibility destination;
- five JSON artifact-contract outputs.

The checker compares nine outputs by exact bytes and BLAKE3 identity. It also compares 31 existing invalid fixtures by failure class and normalized diagnostic text.

Nickel diagnostics can contain process-local `0x…` closure addresses. The checker removes only those addresses before it compares the diagnostic text.

Two Cairn-policy requests opt into the exporter’s conservative secret-marker bypass. The typed profile binds six exact family, path, and literal combinations.

The allowed literals are `MarkerToken =`, `SECRET=`, and `token=`. They define or reject policy vocabulary and do not contain secret values.

The checker rejects any other marker, path, family, count, or rationale. It does not permit secret values.

## Primary and rollback paths

The standalone checker is the primary exact-output and freshness path. The existing Trellis checkers still enforce policy and contract meaning.

Run the bounded rollback modes during migration review:

```console
./scripts/check-verification-policy.rs --legacy-parity
./scripts/check-lifecycle-config.rs --legacy-parity
```

Any output, identity, evaluator, dependency, diagnostic, or rejection drift blocks the cutover. The legacy result remains authoritative until the difference is explained.

## Validation

```console
./scripts/check-nickel-export-cutover.rs --check
./scripts/check-nickel-export-cutover.rs --self-test
nix build .#checks.x86_64-linux.nickel-export-cutover --no-link -L
```

The negative controls cover malformed requests, invalid contracts, stale outputs, manifest tampering, path escape, symlinks, evaluator drift, secret markers, and overclaims.

Passing evidence proves exact declared inputs and outputs under one evaluator descriptor. It does not prove evaluator equivalence, policy correctness, theorem correctness, or release eligibility.

# Runtime-Cost Evidence

Trellis writes specifications and proofs in Verus, but normal Cargo builds erase
ghost code through `vstd`. This document gives lightweight, reproducible evidence
for inspecting representative normal-build artifacts. The observations below are
not universal performance guarantees; they are command-backed snapshots that can
be refreshed locally.

## How to refresh

Use a local target directory so the evidence does not depend on a shared cache:

```bash
export CARGO_TARGET_DIR=target/runtime-cost-evidence
cargo build --example core_basics
stat -c %s target/runtime-cost-evidence/debug/examples/core_basics
cargo run --quiet --example ucan_web_of_trust
```

## Evidence record: core-basics-dev-build

Command: `CARGO_TARGET_DIR=target/runtime-cost-evidence cargo build --example core_basics`

Profile: Cargo `dev` profile.

Target: example `core_basics` from [`examples/core_basics.rs`](../examples/core_basics.rs).

Result: build succeeded on 2026-07-01. The artifact was present at
`target/runtime-cost-evidence/debug/examples/core_basics`; observed size was
`9842688` bytes in this local environment.

Measurement status: this size is an observation, not a guarantee or pass/fail
bound. It is useful for confirming that a normal Cargo artifact exists without
requiring Verus execution.

Refresh: rerun the command above, then update the observed size only if the local
artifact is intentionally being refreshed.

## Evidence record: ucan-web-of-trust-dev-run

Command: `CARGO_TARGET_DIR=target/runtime-cost-evidence cargo run --quiet --example ucan_web_of_trust`

Profile: Cargo `dev` profile.

Target: example `ucan_web_of_trust` from [`examples/ucan_web_of_trust.rs`](../examples/ucan_web_of_trust.rs).

Result: run succeeded on 2026-07-01 and printed
`ucan/web-of-trust: valid UCAN accepted, empty trust graph rejected`.

Measurement status: this runtime output is an observation, not a guarantee or
throughput/latency bound. It demonstrates that the example executes through
normal Rust after proof code is erased.

Refresh: rerun the command above after changing UCAN, web-of-trust, or example
code and update the output text if the example behavior intentionally changes.

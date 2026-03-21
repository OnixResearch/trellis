## Context

The repo is a blank lib crate with no real code. We need to establish the
infrastructure so that every future module follows the same pattern: write
requirements in markdown, formalize with Verus specs, implement with verified
exec functions, check coverage with Tracey.

## Goals / Non-Goals

**Goals:**
- Verus and Tracey wired into the dev workflow
- Clean library Cargo.toml (no runtime/CLI deps)
- Initial requirement structure that future modules extend
- CI via `nix flake check` covers build + test + clippy + fmt

**Non-Goals:**
- Writing the actual verified modules (that's future changes)
- Verus integrated into `nix flake check` (verus nix packaging is unstable)
- Publishing to crates.io (premature)

## Decisions

### 1. Library-only dependencies

**Choice:** Remove tokio, clap, tracing-subscriber, chrono, rand, hex, url, parking_lot,
async-trait, futures. Keep snafu for errors. Add no new deps beyond what Verus needs.

**Rationale:** A verified logic library should have minimal transitive deps. Downstream
crates bring their own async runtime and CLI framework. Verified code is pure logic —
it shouldn't depend on I/O.

**Alternative:** Keep everything, gate behind features. Rejected — feature flags don't
reduce auditability burden and signal wrong intent.

### 2. Verus in devshell only, not in nix build

**Choice:** Add verus to `devShells.default.packages`, not to the crane build pipeline.

**Rationale:** Verus verification is a development-time activity. The crane build
compiles normal Rust — `verus/` files aren't part of the cargo build. Verus doesn't
have stable nix packaging yet.

**Implementation:** Manual `verus` invocation or a `scripts/verify.sh` wrapper. Future
work can add a flake check once verus nix support matures.

### 3. Combined pattern as default

**Choice:** Prefer `exec fn` with `ensures` (combined pattern) for all public API.
Use split pattern only when a spec is shared across multiple functions or needs
recursive proofs.

**Rationale:** Combined pattern keeps spec and implementation together. One annotation
(`r[impl req.id]`) covers everything. Less boilerplate, easier to review.

### 4. Module structure

**Choice:** `src/` has one file per domain (e.g., `src/arithmetic.rs`, `src/bounds.rs`).
`verus/` mirrors the structure for split-pattern specs only. Requirements live in
`docs/requirements.md` as a single file initially, split by domain headers.

**Rationale:** Small library — one requirements file is sufficient until it exceeds ~50
requirements. Splitting prematurely adds navigation overhead.

### 5. Tracey config scope

**Choice:** `include (src/**/*.rs verus/**/*.rs)` for impl/depends annotations.
`test_include (tests/**/*.rs)` for verify-only annotations.

**Rationale:** Verus files contain both `depends` (spec fn) and `verify` (proof fn)
annotations. They must be in `include`, not `test_include`.

## Risks / Trade-offs

**[Verus version drift]** → Pin verus version in README and napkin. Verus syntax can
change between releases. When updating, run full verification pass.

**[Thin initial module set]** → This change establishes infrastructure only. First real
modules come in the next change. Acceptable — infrastructure must exist before content.

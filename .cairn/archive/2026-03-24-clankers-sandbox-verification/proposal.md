## Why

Clankers runs shell commands via the `bash` tool. Before spawning a
child process, `sanitized_env()` strips secret-bearing environment
variables. The function has two filtering mechanisms:

1. **Explicit blocklist** — `SCRUBBED_ENV_VARS`, a static array of
   ~35 known secret variable names (AWS keys, GitHub tokens, API keys,
   SSH agent, database URLs).

2. **Heuristic suffix matching** — any variable whose uppercased name
   ends with `_SECRET`, `_TOKEN`, `_PASSWORD`, `_CREDENTIALS`,
   `_API_KEY`, `_APIKEY`, or `_PRIVATE_KEY` is stripped.

A leak here means the LLM's bash tool can exfiltrate credentials.
The function also injects `CLANKERS_SANDBOX=1` so downstream scripts
can detect sandboxed execution.

The logic is a pure filter: given a set of environment variables,
produce a subset that excludes secrets. No I/O beyond reading
`std::env::vars()` (which we abstract away in the verified version).

The properties that matter:

- **Completeness of blocklist**: every variable in `SCRUBBED_ENV_VARS`
  is absent from the output.
- **Completeness of heuristics**: every variable matching a suffix
  pattern is absent from the output.
- **No over-filtering**: variables that don't match any rule are
  preserved (PATH, HOME, USER, LANG, etc.).
- **Sandbox marker**: `CLANKERS_SANDBOX=1` is always present in output.
- **Idempotence**: filtering an already-filtered set produces the
  same result.

The clankers tests cover 12 specific cases. The verified version
proves the filter properties universally over all possible
environment variable sets.

## What Changes

A new `env_filter` module in verified-logic that extracts the
environment scrubbing logic into verified predicates over abstract
key-value sequences.

**Blocklist predicate**

- `is_blocked_spec` — key is in the explicit blocklist
- Represented as membership in a fixed `Seq<Seq<u8>>` (the known
  variable names encoded as byte sequences)

**Suffix predicate**

- `has_secret_suffix_spec` — uppercased key ends with one of the
  7 secret suffixes
- `ends_with_spec` — sequence suffix check (reuses existing
  `slice` module patterns)
- `to_upper_spec` — ASCII uppercasing for case-insensitive match

**Filter function**

- `sanitize_spec` — given an input `Seq<(Seq<u8>, Seq<u8>)>`,
  produce an output that excludes blocked and suffix-matched keys,
  then appends the sandbox marker
- `sanitize` — exec function matching the spec

**Proofs**

- Blocked keys absent: for all `k` in blocklist, `k` not in output keys
- Suffix-matched keys absent: for all `k` where `has_secret_suffix(k)`,
  `k` not in output keys
- Non-secret keys preserved: for all `(k, v)` in input where
  `!is_blocked(k) && !has_secret_suffix(k)`, `(k, v)` is in output
- Sandbox marker present: output contains `("CLANKERS_SANDBOX", "1")`
- Idempotence: `sanitize(sanitize(env))` == `sanitize(env)`
  (the sandbox marker is not a secret suffix, so it survives
  re-filtering)

## Capabilities

### New Capabilities

- `env-blocklist-complete`: Every known secret variable is filtered
- `env-suffix-complete`: Every heuristic-matching variable is filtered
- `env-non-secret-preserved`: Non-secret variables pass through
- `env-sandbox-marker`: Sandbox marker is always injected
- `env-filter-idempotent`: Filtering is idempotent

### Modified Capabilities

- Cross-module link to `slice::ends_with` for suffix checking

## Impact

- **Files**: 1 new `src/env_filter.rs` module
- **APIs**: ~8–10 pub fns (spec + exec pairs)
- **Dependencies**: None — pure sequence logic
- **Testing**: Verus verification + proptest with random env var sets

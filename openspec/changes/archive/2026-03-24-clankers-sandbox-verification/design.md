## Context

The clankers sandbox policy (`src/tools/sandbox/policy.rs`) filters
environment variables before spawning bash child processes. The
function reads `std::env::vars()`, applies a blocklist and heuristic
suffix filter, injects a sandbox marker, and returns a `Vec<(String, String)>`.

The verified-logic crate already has `slice` (ends_with, starts_with),
`collections` (membership), and `nix_string` (byte-level operations).

## Goals / Non-Goals

**Goals:**
- Verify that every blocklisted key is absent from the output
- Verify that every suffix-matched key is absent from the output
- Verify that non-secret keys are preserved with original values
- Verify that the sandbox marker is always present
- Verify filter idempotence
- Verify output size bound

**Non-Goals:**
- Verify `std::env::vars()` behavior (OS interaction)
- Verify that the blocklist is complete (security policy decision,
  not a mathematical property)
- Verify that the suffix list covers all possible secret patterns
- Verify string encoding (UTF-8 vs bytes)

## Decisions

### 1. Environment as Seq of Pairs

**Choice:** The environment is modeled as `Seq<(Seq<u8>, Seq<u8>)>` —
a sequence of (key, value) byte-string pairs. No Map type needed
because the filter is a linear scan.

**Rationale:** The clankers code iterates over `std::env::vars()`
and filters with a closure. The mathematical operation is a
sequence filter. Verus reasons about `Seq` filters well.

**Alternative:** Use `Map<Seq<u8>, Seq<u8>>`. Rejected — the
filter doesn't need O(1) lookup, and `Map` adds complexity around
key ordering and uniqueness. The real environment can have duplicate
keys (rare but legal on some platforms), and our model handles it
naturally.

### 2. Blocklist as Static Spec Sequence

**Choice:** The blocklist is a `spec fn blocklist() -> Seq<Seq<u8>>`
that returns a fixed sequence of known secret variable names.

**Rationale:** The clankers code uses `static SCRUBBED_ENV_VARS: &[&str]`.
In Verus, a `closed spec fn` returning a fixed `Seq` is the natural
equivalent. The blocklist membership check is
`exists|i: int| 0 <= i < blocklist().len() && blocklist()[i] == key`.

**Implementation:** The spec function returns a sequence literal.
The exec function uses a `HashSet<&[u8]>` for O(1) lookup (matching
the clankers code's `HashSet<&str>`).

### 3. Suffix Matching via ends_with

**Choice:** Secret suffix checking reuses the `slice::ends_with`
pattern from the existing verified-logic module. The key is
uppercased first (ASCII-only), then checked against each of the
7 suffixes.

**Rationale:** The clankers code does `upper.ends_with("_SECRET")`.
We already have verified `ends_with` proofs. Composing uppercasing
with suffix checking gives us the same semantics.

**Implementation:** `has_secret_suffix(key)` uppercases `key` to
`ukey`, then checks `ends_with(ukey, suffix)` for each suffix in
a fixed list. The exec function matches the clankers code structure.

### 4. ASCII Uppercasing as Bounded Transform

**Choice:** Uppercasing is ASCII-only: bytes in `[97, 122]` are
mapped to `[65, 90]`. All other bytes pass through unchanged.

**Rationale:** Environment variable names are ASCII on all platforms
clankers supports. UTF-8 uppercasing is locale-dependent and
irrelevant here. The clankers code calls `.to_uppercase()` on
Rust strings, but env var names are ASCII in practice.

**Implementation:** Spec function maps each byte individually.
Proof shows: uppercasing is idempotent (`upper(upper(k)) == upper(k)`)
and length-preserving (`upper(k).len() == k.len()`).

### 5. Filter Correctness as Two-direction Proof

**Choice:** The filter proof has two directions:
1. **Soundness**: everything in the output is non-secret (not
   blocked, not suffix-matched) or is the sandbox marker.
2. **Completeness**: every non-secret entry from the input is
   in the output.

Together these give: `output == filter(input) ++ [marker]` where
`filter` keeps exactly the non-secret entries.

**Rationale:** One-direction proofs miss bugs. Showing only soundness
(no secrets in output) doesn't catch over-filtering (dropping PATH).
Showing only completeness (all safe vars present) doesn't catch
under-filtering (leaking a secret). Both directions together fully
characterize the output.

### 6. Idempotence via Marker Non-matching

**Choice:** Idempotence proof relies on showing that the sandbox
marker key `CLANKERS_SANDBOX` is not in the blocklist and does not
match any secret suffix.

**Rationale:** If the marker survived the first filter, it will
survive the second. If no secrets were in the first output (by
soundness), no secrets are removed in the second pass. The marker
is the only new entry, and it's not a secret.

**Implementation:** A lemma that checks `CLANKERS_SANDBOX` against
every blocklist entry and every suffix. Since the blocklist and
suffix list are fixed, this is a finite verification.

## Risks / Trade-offs

**[ASCII-only uppercasing is incomplete]** → If someone names an
env var with non-ASCII characters that uppercase differently under
Unicode rules, our check might miss it. Mitigation: env var names
are ASCII on Linux, macOS, and Windows. This is a non-risk in
practice.

**[Blocklist completeness is not verified]** → We prove that the
listed variables are filtered, not that the list is sufficient.
A new secret variable (e.g., from a new cloud provider) won't be
caught until added to the blocklist. Mitigation: the suffix
heuristic catches most custom secrets. The blocklist is defense
in depth for well-known names that don't follow suffix conventions
(e.g., `DATABASE_URL`).

**[Duplicate keys in environment]** → If the same key appears
multiple times, our filter removes all copies. This matches the
clankers behavior (the closure filter applies to every entry).

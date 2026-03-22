# Napkin — verified-logic

## Project State
- Verus specs embedded in `src/` — no separate `verus/` directory
- Every exec fn has `ensures result == *_spec(args)` — Verus verifies the actual implementation
- Drift between spec and impl is structurally impossible
- 21 Verus proofs + 25 cargo tests (10 unit + 15 property) = all gates green
- `vstd` is a cargo dependency — `verus!` blocks compile normally (ghost code erased)
- `rust_verify` runs on `src/lib.rs` — verifies the whole crate

## Recent Changes
- Slice module added: chunk_count, prefix_eq, suffix_eq, all_equal, mismatch_index
  - chunk_count proofs need nonlinear_arith for multiplication commutativity:
    fundamental div-mod gives `d * (n/d)` but ensures needs `(n/d) * d`
  - suffix_eq quantifier needs explicit #![trigger] — Verus can't auto-infer for
    complex index expressions like `a[a.len() - n + i]`
  - suffix_eq loop invariant needs `a_start + n == a.len()` to prove no arithmetic overflow
    on `a_start + i` (instead of `a_start as int == a@.len() - n as int`)
  - mismatch_none_implies_prefix_eq uses inductive helper proof fn
  - Seq::len() returns nat, not int — must cast with `as int` for int return types
- Conversion module added: narrow_u32, narrow_u16, narrow_u8, signed_to_unsigned, unsigned_to_signed
  - Cast lints (truncation, sign_loss, wrap) allowed at module level — verified casts are the point
  - clippy::checked_conversions also allowed — can't use try_from inside verus! blocks
  - Roundtrip proofs (i64→u64→i64, u64→i64→u64) verify directly — Z3 handles cast chains
  - Narrowing chain proof (u8 fits → u16 fits → u32 fits) trivially automatic
  - Option<u32>, Option<u16>, Option<u8>, Option<i64> all work in Verus ensures
- Cross-module properties added: all_equal⟹count, chunk_count↔div_ceil, narrow↔in_range, all_equal⟹partitioned
  - chunk_count_div_ceil needs by(nonlinear_arith) for quotient uniqueness:
    Z3 can't derive (n-1)/k == n/k from fundamental_div_mod alone, needs
    case split on n%k > 0 vs == 0 with nonlinear_arith per branch
  - all_equal_implies_count needs inductive helper (count_from == len - from)
  - all_equal_implies_partitioned and narrow_iff_in_range are automatic (Z3 instant)
- Ring buffer added: verified FIFO queue with power-of-two capacity
  - Struct fields must be `pub` for `pub open spec fn` — Verus treats struct as opaque
    outside the module when any field is private
  - `is_power_of_two_spec` import needs #[cfg(verus_keep_ghost)] — spec fns are erased
    in normal builds, so top-level `use` fails without the gate
  - Pop overflow fix: capture `head` and `cap` as locals before mutation, so Verus
    retains `head < cap` for the `(head + 1)` arithmetic
  - `&mut self` methods work with `old(self)` for pre-state references in ensures
  - Vec::set(i, v) is the Verus way to mutate vec elements in-place
  - Constructor loop builds Vec one element at a time (no vec![0; n] in Verus)
- Bitmap added: verified fixed-size bit set using bitwise module functions
  - Toggle proof requires `(x >> p) & 1 == 0 || (x >> p) & 1 == 1` by(bit_vector)
    because Z3 default theory treats `&` as uninterpreted — doesn't know & 1 ∈ {0,1}
  - Split toggle ensures into two implications (was-set⟹now-clear, was-clear⟹now-set)
    instead of `==` on booleans — Z3 handles implications better
  - get_spec bridging needs explicit `assert(self.get_spec(...) == bit_at_spec(word, pos))`
    to force Z3 to unfold through Vec indexing + bit_at_spec
  - Word index bounds (pos/64 < words.len()) follows from div_ceil definition but Z3
    handles it automatically when the invariant uses `(num_bits-1)/64+1` directly
- Quorum module added: majority, is_quorum, intersection theorem
  - Intersection proof uses by(nonlinear_arith) for (n/2+1)+(n/2+1) > n
  - majority_le_n also needs nonlinear_arith: n/2+1 <= n for n > 0
  - majority_gt_half: (n/2+1)*2 > n — the key fact for pigeonhole
  - All three proofs are one-line nonlinear_arith — Z3's nonlinear solver
    handles integer division bounds cleanly
- Monotonic module added: is_advance, is_strict_advance with full ordering proofs
  - All proofs automatic — Z3 handles linear integer ordering without hints
  - Mixed composition proofs (strict + non-strict) verify instantly
- Epoch module added: lexicographic (term, index) comparison
  - Total ordering, transitivity, antisymmetry, higher-term-wins all automatic
  - epoch_ge_antisymmetric calls epoch_cmp_eq and epoch_cmp_antisymmetric as helpers
  - bool_to_int_with_if clippy lint suppressed — Verus needs explicit if-chain
- Log module added: agree_at, logs_match_prefix, log_append, log_truncate
  - LogEntry struct with pub fields (same Verus opaque rule as RingBuffer/Bitmap)
  - logs_match_prefix exec fn uses same loop pattern as prefix_eq in slice module
  - Verus auto-chose triggers for Seq<LogEntry> quantifiers (a[i].term, b[i].term)
  - Seq::push for append, Vec::truncate for truncation — both have Verus specs in vstd
  - =~= (extensional equality) used in truncate_full and append_truncate_roundtrip
  - All proofs automatic except logs_match_prefix exec (loop invariant)
- Cross-module consensus properties added:
  - majority_via_div_ceil: majority(n) == div_ceil(n+1, 2) — automatic, Z3 unfolds both specs
  - epoch_ge_term_index: decomposes epoch_ge into term/index conditions — automatic
  - log_prefix_term_eq: prefix match implies term equality — automatic (follows from spec def)
  - All three proofs are trivially automatic — the specs are defined to make these hold
- Quorum module expanded: fault_tolerance, can_remove_safely, min_cluster_size
  - Direct equivalents of aspen's fault_tolerance_for_size, can_remove_voter_safely,
    min_cluster_size_for_tolerance — same formulas, now machine-checked
  - fault_tolerance_leaves_quorum: n - ft(n) >= majority(n) — by(nonlinear_arith)
  - min_size_fault_tolerance_inverse: min_size(ft(n)) <= n — the functions are inverses
  - min_size_has_tolerance: ft(min_size(f)) == f — exact roundtrip for odd clusters
  - can_remove_ge_3: n >= 3 ⟹ can remove safely — the critical cluster safety threshold
  - Tolerance vs remove equivalence: can_remove(n) ⟺ ft(n) >= 1
- Expiration module added: expires_at, is_expired, is_expired_opt, time_remaining, is_ttl_valid
  - Direct equivalents of aspen's calculate_expires_at_ms, is_lease_expired,
    is_key_expired, time_remaining_exec, is_ttl_valid
  - Verus doesn't support saturating_mul — use explicit overflow check instead:
    `if x > u64::MAX / 1000 { u64::MAX } else { x * 1000 }`
  - Two expiration semantics: is_expired uses > (lease: not expired AT boundary),
    is_expired_opt uses >= (key: expired AT boundary) — matches aspen's distinction
  - Lifecycle model oracle: create expiration, advance time in steps, check
    is_expired and time_remaining agree at every point
- CAS module added: check_condition, check_cas, evaluate_conditions
  - Abstracted over value types — uses (condition_type, is_some, values_equal) booleans
    instead of string/byte references, keeping module generic
  - evaluate_conditions uses direct postconditions (first-failure index + all-prior-pass)
    like find_first and mismatch_index patterns
  - Inductive proof over quantified sequences failed (all_exists_present) — Z3 can't
    handle forall over Seq combined with recursive spec. Dropped in favor of model oracle.
  - KEY_EXISTS and KEY_NOT_EXISTS proved to be complements
- Interval module added: contains, is_empty, overlaps, split, adjacent, merge
  - Half-open [lo, hi) intervals over u64, matching aspen's KeyRange semantics
  - Split produces non-overlapping adjacent halves; merge is inverse of split
  - split_covers: every point in original is in exactly one half
  - adjacent_no_overlap: adjacent intervals never overlap
  - All proofs automatic — Z3 handles linear integer comparisons instantly
  - nonminimal_bool clippy lint suppressed — Verus needs explicit conjunctions
    in the adjacent spec for Z3 trigger matching
- Batch module added: check_limits, flush_decision, op_size, delete_op_size
  - FlushDecision enum (Immediate/Delayed/None) with spec using int encoding
    (1/2/0) — Verus can't match on enum variants in specs, use int proxy
  - Empty batch invariant: check_limits(0, ...) always returns (false, false)
  - if_same_then_else clippy lint suppressed — Verus needs explicit if-chain
    for Z3 to match each branch to the correct spec case
  - Lifecycle model oracle: simulate add/flush cycles, verify check_limits
    and flush_decision agree with manual calculation at every step
- Total: 376 Verus proofs, 273 cargo tests, 121 requirements

## Architecture
- `spec fn *_spec(...)` — mathematical definition (ghost, erased at compile time)
- `exec fn *(...)` with `ensures result == *_spec(args)` — runtime code, verified to match spec
- `proof fn` — proves properties of the spec (symmetry, idempotent, bounded, etc.)
- `#[test]` + proptest — runtime verification + fuzz testing

## Verus Toolchain
- verus-root: `/nix/store/7hr3z8ss29pxl29pllv6kchlk9kbckys-verus-root-0.2026.01.30`
- rust toolchain (for verus): `/nix/store/27f9852zp2qlg34zmqjz27cg6sq680sx-rust-default-1.93.0`
- vstd crates.io version: `=0.0.0-2026-01-25-0057`
- Wrapper: `scripts/verus.sh` runs rust_verify on src/lib.rs with correct extern paths

## Clippy Policy
- Correctness + suspicious: deny
- Pedantic: warn
- Manual impl lints allowed: `manual_abs_diff`, `manual_map`, `manual_saturating_arithmetic`,
  `manual_unwrap_or`, `implicit_saturating_sub`
- Cast lints: deny
- `unsafe_code`: forbid
- `integer_division`: allow in test modules

## Tracey Annotations
- `r[depends]` on `spec fn` — formal dependency on requirement text
- `r[impl]` on `exec fn` — the runtime implementation
- `r[verify]` on `proof fn` and `#[test]` — evidence of correctness
- Config scans `src/**/*.rs` only (no separate verus/ directory)

## Enforcement Scripts
- `check-coverage.sh` — scans for `pub fn` without `r[impl]`, exits 1 on violation
- `check-ensures.sh` — scans for `pub fn` inside `verus!` without `ensures`, exits 1
- Both support exemption markers (`@verified-logic:exempt`, `@verified-logic:no-ensures`)
- Lookback window: 10 lines for r[impl], 5 lines for exemptions
- Stops scanning at `#[cfg(test)]` (test fns don't need r[impl])
- Skips `spec fn`, `proof fn`, private `fn`

## Cross-Module Pattern
- Spec fns from other modules are ghost code — erased in normal builds
- Cross-module proofs must go in a `#[cfg(verus_keep_ghost)] mod verus_proofs { }`
- Inside that module: `use crate::other_module::*_spec` works because Verus sets the cfg
- Tests go outside the cfg-gated module (they use exec fns which exist in all builds)
- Verus verifies the whole crate including cfg(verus_keep_ghost) modules

## Mistakes & Corrections
- Tried `cargo-verus` — needs rustup, doesn't work in nix. Used `rust_verify` directly.
- `proof fn` can't call `exec fn` in ensures — must reference `spec fn` instead.
- `FnSpec` deprecated — use `spec_fn`.
- Ghost closures can't have requires/ensures — restructured option module.
- Removed `verus/` directory — all specs embedded in `src/`.

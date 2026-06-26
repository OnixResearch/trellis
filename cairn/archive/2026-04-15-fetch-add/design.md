## Context

The verified-logic library models CAS as a conditional swap returning
success/failure plus the cell's value. Fetch-and-add is the unconditional
counterpart: it always succeeds, always modifies, and always returns the
old value. The algebraic properties (commutativity, associativity, identity)
are what distinguish it from CAS in concurrent algorithm design. This module
proves those algebraic properties and ties FAA back to CAS via a refinement
proof.

## Goals / Non-Goals

**Goals:**
- Model FAA as a pure function: `(old, addend) → (old, old + addend)`
- Both wrapping (mod 2^64) and saturating (clamp at u64::MAX) variants
- Prove algebraic properties: identity, commutativity, associativity
- Prove monotonicity under no-overflow conditions
- Prove CAS-loop refinement: a CAS loop that computes `current + addend`
  produces the same result as atomic FAA
- Prove fetch-sub as negation of fetch-add with round-trip
- Prove saturation idempotence

**Non-Goals:**
- Memory ordering or fence semantics (this is a logic model)
- Hardware-specific FAA variants (e.g., LL/SC decomposition)
- Fetch-and-or, fetch-and-and, or other RMW operations (separate modules)
- Contention modeling or performance analysis

## Decisions

### 1. Core Representation

**Choice:** FAA modeled as a spec function returning a tuple `(u64, u64)` —
(value returned to caller, new cell content). The exec function mirrors this.

**Rationale:** Matches the hardware semantics exactly. The tuple makes both
the caller's observation and the cell mutation explicit, which is what the
proofs need. Keeping it as a pure function (no mutable state) lets Verus
reason about sequential composition directly.

**Implementation:**
```
spec fn faa_wrapping_spec(current: u64, addend: u64) -> (u64, u64) {
    (current, u64_wrapping_add(current, addend))
}
spec fn faa_saturating_spec(current: u64, addend: u64) -> (u64, u64) {
    (current, u64_saturating_add(current, addend))
}
```

### 2. Wrapping Arithmetic

**Choice:** Define `u64_wrapping_add` as `(a + b) % (u64::MAX as nat + 1)`,
cast back to `u64`. Use `by(bit_vector)` proofs for modular identities.

**Rationale:** Verus's `by(bit_vector)` mode is the natural way to prove
modular arithmetic properties. Defining the spec in terms of `nat` modular
arithmetic and proving equivalence to the Rust wrapping semantics keeps the
spec readable while the bit-vector solver handles the hard cases.

**Alternative:** Work entirely in `nat` with explicit modulus — rejected
because it makes the commutativity and associativity proofs verbose.
Bit-vector mode handles these in one line.

### 3. Saturating Arithmetic

**Choice:** `u64_saturating_add` defined as `if a + b > u64::MAX { u64::MAX }
else { a + b }` with the addition done in `nat`.

**Rationale:** Saturating semantics is a simple min/clamp. The interesting
property is idempotence once saturated: `sat_add(u64::MAX, n) == u64::MAX`
for all n. This follows directly from the definition.

### 4. Commutativity Proof Structure

**Choice:** Prove that for any `current`, `faa(a)` then `faa(b)` leaves the
cell with the same value as `faa(b)` then `faa(a)`. Specifically:
```
let (_, mid) = faa(current, a);
let (_, final_ab) = faa(mid, b);
let (_, mid2) = faa(current, b);
let (_, final_ba) = faa(mid2, a);
ensures final_ab == final_ba
```

**Rationale:** This is the property that matters for concurrent algorithms:
the final cell value is independent of which thread's FAA executed first.
The returned values differ (each thread sees its own snapshot), but the
cell converges. For wrapping, this follows from commutativity of addition
mod 2^64. For saturating, it follows from `min(a+b, MAX) == min(b+a, MAX)`.

### 5. CAS Refinement

**Choice:** Define a CAS-loop version of FAA:
```
spec fn faa_via_cas(current: u64, addend: u64) -> (u64, u64) {
    let desired = wrapping_add(current, addend);
    let (ok, after) = cas_result_spec(current, current, desired);
    // ok is always true when expected == current
    (current, after)
}
```
Prove `faa_via_cas(c, a) == faa_wrapping_spec(c, a)`.

**Rationale:** In real hardware, architectures without native FAA (e.g.,
some RISC-V configurations) implement it as a CAS loop. The refinement
proof shows the CAS-loop implementation is correct. It also connects the
FAA module to the existing `cas_loop.rs` verification.

### 6. Fetch-Sub as Duality

**Choice:** Define `fetch_sub_wrapping_spec(current, subtrahend)` as
`faa_wrapping_spec(current, wrapping_neg(subtrahend))` where
`wrapping_neg(x) = (u64::MAX - x + 1) % 2^64`. Prove the round-trip:
`faa(faa(c, n).1, wrapping_neg(n)).1 == c`.

**Rationale:** Fetch-sub is not a separate primitive — it's FAA with
negation. Making this explicit avoids duplicating proofs. The round-trip
property (`add n then subtract n restores the original`) is the key
correctness property for reference counting.

### 7. Module Structure

**Choice:** Single module `fetch_add.rs` with sections:
1. Wrapping/saturating arithmetic helpers + proofs
2. FAA spec and exec functions (wrapping + saturating)
3. Algebraic proofs (identity, commutativity, associativity)
4. Monotonicity proof
5. CAS refinement proof
6. Fetch-sub duality + round-trip
7. Saturation idempotence
8. Tests

**Rationale:** All proofs are about the same operation. The algebraic
properties reference each other (associativity uses commutativity in the
proof). Keeping them together avoids cross-module proof imports.

## Risks / Trade-offs

**[bit_vector mode limitations]** → Verus's `by(bit_vector)` can handle
most modular arithmetic identities automatically, but complex multi-step
reasoning sometimes needs manual lemma chaining. The commutativity and
associativity proofs should be one-liners in bit_vector mode, but if not,
fallback to `nat`-level reasoning with explicit modular reduction.

**[Saturating associativity needs the right statement]** → Saturating add is
still associative: both `sat(sat(a, b), c)` and `sat(a, sat(b, c))` reduce
to `min(a + b + c, u64::MAX)`. The proof should state the positive law
explicitly instead of looking for a counterexample.

**[CAS refinement only covers the success path]** → The refinement proof
assumes the CAS loop's expected value matches current (i.e., no contention).
Under contention, the CAS loop retries until it succeeds, but the
*eventual* result is the same. A full liveness proof (the loop terminates)
is out of scope — the existing `cas_loop.rs` already covers retry bounds.

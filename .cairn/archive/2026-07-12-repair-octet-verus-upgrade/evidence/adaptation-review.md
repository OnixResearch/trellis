# Adaptation adversarial review

## IronKV semantic boundary

The pinned IronKV revision `6b8830a47a2dd5a8ce141438ebb2decb6550298a` is the upstream commit “Adapt to finite sets and maps.” Its commit message points to Verus PR #2486. The PR states that, after the split, `ISet` and `IMap` are the new names for the old possibly-infinite `Set` and `Map` semantics. The candidate `0.2026.05.17.e479cce` predates that June split and exposes predicate-backed generalized `Set`/`Map` values.

The compatibility patch is therefore a reverse API adaptation for this candidate, not a finite-to-infinite proof claim. It maps post-split infinite collection names and extensional operators back to their candidate-era forms, preserves map domains with `mk_map`, and removes conversions that only bridge post-split finite collections into infinite ones. The receipt still labels the result `adapted-source-only`; verifier acceptance does not establish global equivalence between Vstd releases.

Primary sources:

- <https://github.com/verus-lang/verus/pull/2486>
- <https://github.com/verus-lang/verified-ironkv/commit/6b8830a47a2dd5a8ce141438ebb2decb6550298a>

## Allocator proof boundary

The three arithmetic lemmas quantify over mathematical `int`, so machine overflow is not part of their contracts. `lemma_fundamental_div_mod` supplies exact quotient/remainder decomposition; the existing zero-divisor preconditions are retained. `#[verifier::nonlinear]` asks Verus to discharge the universal `mul_mod_right` theorem and is not an assumption or external body. The complete adapted allocator verifies all 732 obligations and links against libc 0.2.147 built by the candidate Rust closure.

The segment-size repair changes no executable branch: it names existing threshold/alignment expressions and proves the already-declared `size == SEGMENT_SIZE ==> res == SEGMENT_SIZE` postcondition in a proof block.

## Secondary-review disposition

A local secondary reviewer correctly challenged finite/infinite semantics, negative operands, and proof-authority wording. Those concerns are bounded above and by the adapted-source label. It also suggested adding `assume`, `external_body`, or trusted markers; that suggestion was rejected because it would violate the completion contract and weaken the proof boundary. No such marker was added.

The canonical rail independently checks patch BLAKE3 values, rejects a mutated patch, forbids added assumption/external-body/trigger lines, compares trust-marker counts before and after application, executes native linked verifier commands, and requires every complete/required-native exit to be zero.

# Verification

## Result

The Scheme logic component is implemented, synchronized, and archived. It
provides bounded reader structure, explicit interning, module linkage, contract
and blame metadata, bytecode admission, cycle-safe value graphs, and Scheme
collection shapes.

Steel is an inspiration source only. No Steel source was copied or executed.
The component does not prove language compatibility or interpreter behavior.

## Evidence

| Check | Result |
|---|---|
| Exact-parent S-expression baseline | 23 tests passed at `58662198` |
| Focused Scheme logic tests | 31 passed, including 4 property tests |
| Focused reader Verus | 5 verified, 0 errors |
| Focused interner Verus | 3 verified, 0 errors |
| Focused module Verus | 4 verified, 0 errors |
| Focused contract Verus | 4 verified, 0 errors |
| Focused bytecode Verus | 5 verified, 0 errors |
| Focused value and collection Verus | 7 verified, 0 errors |
| Strict Clippy | passed |
| Full `scripts/verify.rs` wrapper in the pinned Nix shell | `ALL CHECKS PASSED` |
| Full-library Verus | 11,060 verified, 0 errors |
| README metrics | 11,060 proofs, 5,205 tests, 2,239 requirements, 622 modules, 16 boundaries |
| Tracey uncovered | 0 of 2,239 |
| Tracey untested | 0 of 2,239 |
| Proof artifact manifest | 17 primitives, 1 Preserves envelope, 11 fixtures passed |
| Proof evidence bundle and IR chain | 6 records passed |
| Consumer profile | 39 modules, 42 artifacts, 10 fixtures passed |
| Pinned Nix Octet gate | 3,867 accepted/current, 0 new, 0 stale |
| Cairn initial validation and gates | passed |
| Cairn final validation, tasks gate, and accepted-spec sync | passed |
| Cairn archive execution | passed |

Evidence files are under `evidence/baseline-2026-09-04/`,
`evidence/final-2026-09-04/`, and `evidence/octet/`.

## Architecture and authority

All new decisions are deterministic functional-core code. Callers supply every
span, token, table, graph, contract, instruction, budget, and value explicitly.

The component performs no filesystem, network, clock, random, process, parser,
module-loader, allocator, GC, JIT, FFI, or bytecode-execution effect.

Callers retain source parsing, predicate execution, global interning, module
loading, runtime allocation, and concrete interpreter authority.

## Proof search and audit

Three correlated serial lenses were used because subagent consent was absent.

| Family | Mechanism | Result |
|---|---|---|
| full parser model | parser and evaluator semantics in one proof surface | rejected as broader than the change and harder to bound |
| generic collection reuse only | existing Trellis graphs and collections without Scheme metadata | rejected because it omits attachment, blame, and bytecode rules |
| finite typed envelopes | explicit reader, table, graph, wrapper, code, and value facts | selected and validated |

The adversarial pass checked invalid and overlapping spans, unmatched,
mismatched, duplicate, and crossing delimiters, missing attachments, malformed
intern tables, duplicate and missing exports, under-budget cycles, arity and
blame errors, invalid bytecode operands, stack underflow, rollback bounds,
dangling value edges, cyclic lists, invalid vectors, duplicate hashes, invalid
bytevector elements, and unsupported claim promotion.

## Bounded broad-gate blocker

`NIX_CONFIG='builders =' nix flake check -L` reaches the immutable
`consumer-release-profile` check and fails against its old pinned source.
That source reports `src/lib.rs` as
`47826:a8d89d79f66dbf6ca14869a8384c71dfd807898c5b81e7f6c020f64a6ab94fde`.
The current checked profile records
`51082:0e514208f550375dd3e7184e49a2a6c1fb459538c03dbf85e2b80f434c2b74d5`.
The same immutable source also has the old proof manifest.

This release-pin drift is outside this change. The focused required Nix Octet
gate and current-tree consumer-profile check both pass.

## Baseline correction

The first filter used the source-directory name and selected zero tests. A
detached checkout of exact parent `58662198` then ran the unchanged
`s_expression` filter in its own working directory. All 23 baseline tests
passed before closeout.

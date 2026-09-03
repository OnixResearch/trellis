# Backend Codegen Trace Verification

## Result

The finite backend-codegen record implementation is complete. Strict validation
and the proposal, design, and tasks gates pass. Sync created the accepted
specification, and archive execution completed under `2026-09-03-backend-codegen-trace`.

The public `backend_codegen_trace` facade composes narrow model, layout,
operation, admission, proof, and test components. Every production file has
fewer than 300 lines. The functional core performs no filesystem, process,
clock, network, compiler, backend, or other external effect.

The model provides:

- typed block, value, function, type, and place keys;
- a bound of `MAX_OPERATIONS` rows per finite table;
- one direct block mapping per admitted MIR block;
- explicit, unique expansion blocks;
- source locations admitted by `mir_visitor_traversal`;
- statement, operand, place, rvalue, branch, call, return, and unreachable operations;
- defined-before-use value checking;
- mapped-successor and expansion control targets;
- one terminal operation per MIR block; and
- one direct-value or memory-backed class per MIR local.

Call return and unwind targets must name mapped successors or declared expansion
blocks. Dangling handles, unsupported operations, operation-after-terminator
records, and missing terminators fail before count facts become available.

## Baseline

The baseline source was
`782dea23b209471b26f790eb6b2469dba63b060c`. Cargo, exact-source Nix Verus,
and Tracey passed before implementation. The baseline had 10,925 verified
obligations and 2,129 covered requirements.

See `evidence/baseline-2026-09-03/`.

## Final Evidence

| Check | Outcome |
|---|---|
| Focused Cargo tests | 21 passed |
| Focused Verus | 12 verified, 0 errors |
| Full Trellis wrapper | `=== ALL CHECKS PASSED ===` |
| Full metric check | 10,937 Verus proofs; 4,993 tests; 2,140 requirements; 613 modules; 16 trusted boundaries |
| Strict Clippy | passed with `-D warnings` |
| Tracey uncovered | 0 of 2,140 |
| Tracey untested | 0 of 2,140 |
| Octet | 3,867 accepted; 0 new; 0 stale |
| JSON artifact contracts | positive and negative fixtures passed |
| Consumer profile | 39 modules, 42 artifacts, and 10 fixtures passed |
| Cairn strict validation | passed |
| Cairn proposal, design, and tasks gates | passed |

The full wrapper ran inside the repository Nix development shell. A temporary
`.worktrees/aspen` symlink restored the existing `../aspen` sibling relation.
The command removed the link after completion.

See `evidence/final-2026-09-03/`.

## Octet Ratchet

The first Octet run found two new findings. One applied to the new component.
Splitting block-assignment validation removed that function-length finding.

The final `src/builder_lower/` tree has zero Octet findings. Its three-line
crate-root declaration changed only the existing `src/lib.rs` file-length
identity. The guarded migration replaced that ID and preserved every unchanged
finding record. It added no exception, warning budget, or builder-record finding.

See `evidence/octet/`.

## Generated and Bound Artifacts

The change refreshed these repository-owned artifacts after source identity
changed:

- `docs/module-index.md`;
- `consumer-profile/generated/profile.json`, from its Nickel source;
- `docs/proof-evidence-bundle.json`; and
- `docs/proof-evidence-ir-chain.json`.

The consumer profile keeps the same 39 UCAN-facing modules. It updates only the
measured crate-root artifact.

## Boundaries

This evidence proves finite builder-record shape, handle, mapping, ordering,
control-target, terminator, and local-class admission. It does not prove:

- LLVM, Cranelift, GCC, or another concrete backend;
- concrete IR semantics;
- target ABI correctness;
- machine-code correctness;
- linker or LTO behavior;
- debug-information correctness;
- backend performance; or
- verifier soundness.

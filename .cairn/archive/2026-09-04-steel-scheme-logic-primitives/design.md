## Design

This change defines finite Scheme/interpreter-adjacent primitives inspired by Steel while keeping Trellis's usual functional-core / imperative-shell boundary: pure checked logic over explicit input envelopes, with no ambient runtime state, file I/O, network I/O, macro execution, GC, JIT, FFI, or process effects in the verified core.

### Decisions

1. **Model reusable Scheme logic, not Steel compatibility.** The implementation may use Steel as a reference for useful surfaces, but the verified claims are Trellis-owned finite predicates. They do not assert conformance to Steel, R5RS, R7RS, Racket, or any full language standard.

2. **Start with reader and metadata shape.** Reader/token/span admission should prove delimiter matching, token-span bounds, datum-comment/quote attachment shape, and AST-child span containment over finite token streams. This complements existing `text_string` span envelopes without requiring a full parser proof.

3. **Make interning explicit state.** Interning logic should use finite tables passed as input/output values. The verified core proves key stability, uniqueness, and resolve/intern roundtrips for admitted tables. Global caches, locks, `OnceCell`, atomics, and hash-table implementation behavior remain runtime-shell concerns.

4. **Use graph primitives for modules.** Module `require`/`provide` logic should reuse Trellis graph/reachability patterns: declared modules, explicit export sets, import edges, duplicate export rejection, no unprovided-name leakage, and deterministic transitive closure or fuel-bounded traversal.

5. **Keep contract/blame finite.** Contract primitives should model flat predicates, function arity, wrapper composition, and blame polarity metadata. They should not execute user predicates or prove higher-order language semantics.

6. **Separate bytecode admission from execution.** Constant-pool and bytecode primitives should prove index bounds, stack-effect summaries, call arity checks, local/capture slot bounds, constant rollback prefix preservation, and deterministic rejection metadata. VM execution correctness is out of scope for this change.

7. **Model value graphs independently of allocation.** Pair/vector/value graph primitives should use finite node identifiers and explicit edge lists. Cycle-safe traversal should be fuel-bounded or visited-set based, avoiding claims about actual GC, pointer identity, or allocator behavior.

8. **Prefer Scheme-specific collection laws only where they add value.** Trellis already has generic collection primitives. The new collection slice should focus on Scheme list/pair shape, vector length preservation, bytevector element bounds, immutable-update membership laws, and explicit malformed-list rejection.

9. **Require positive and negative evidence together.** Every implementation slice must include accepted examples and failure examples: malformed spans, dangling intern keys, duplicate exports, unprovided imports, arity mismatch, out-of-bounds bytecode operands, cyclic traversal without fuel, and invalid bytevector elements.

### Functional core and shell boundary

The checked core should accept and return plain finite Rust/Verus values:

- token, delimiter, datum, and span envelopes;
- interner maps and literal tables;
- module/export/import graph metadata;
- contract and blame descriptors;
- constant-pool and bytecode instruction summaries;
- value graph nodes, edges, and traversal budgets;
- collection and bytevector envelopes.

The shell may later parse source text, populate tables, compute hashes, call Steel, load modules, allocate values, fetch files, perform FFI, or execute bytecode. Those effects must remain outside the verified core and feed explicit finite data into it.

### Validation shape

Positive evidence should cover:

- balanced delimiter streams with child spans inside parent spans;
- stable intern/resolve roundtrips for symbols and literals;
- module import closure that references only provided names;
- contract wrapper metadata with expected blame polarity;
- bytecode sequence whose stack summary never underflows and whose operands are in bounds;
- value graph traversal that terminates on acyclic and cyclic inputs with the declared fuel or visited set;
- list/vector/bytevector operations that preserve declared shape.

Negative evidence should cover:

- unmatched delimiters and invalid token spans;
- dangling intern keys or duplicate key/value associations;
- duplicate exports, missing exports, and cyclic module traversal without admitted fuel;
- function contract arity mismatch and wrong blame polarity metadata;
- constant-index, local-slot, capture-slot, and stack-underflow bytecode failures;
- cyclic value graphs traversed without sufficient budget;
- improper lists and bytevector elements outside the byte domain.

### Documentation and lifecycle sync

Implementation must add matching `docs/requirements.md` entries because Tracey coverage in Trellis is driven by that file. The accepted Cairn spec and README/module catalog should be synced after the public module IDs land, and archive evidence should capture focused Cargo, Verus, Tracey, and Cairn validation outputs.

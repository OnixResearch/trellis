## Context

The repository already contains active broader lifecycle work for Steel-inspired Scheme logic. That work covers reader/token/span and interpreter-adjacent metadata. The S-expression primitive defined here is deliberately narrower: a finite tree format with canonical serialization and bounded parsing/admission facts. Keeping the primitive separate gives Trellis a reusable proof artifact that can support Scheme-facing work without making any whole-language claim.

The verified core should follow Trellis style: pure `spec fn` definitions, checked `exec fn` wrappers with `ensures` clauses, and explicit finite inputs. Parsing from bytes, rendering to strings, and validation should be deterministic over supplied data; any file I/O or consumer-specific loading remains outside this module.

## Decisions

### 1. Model canonical S-expressions, not a language runtime

**Choice:** Define an AST with atom and list nodes, well-formed atom predicates, canonical serialization specs, and parse/admission metadata.

**Rationale:** Consumers need stable finite tree identity and roundtrip facts. They do not need evaluation, environments, closures, macros, module loading, or reader extensions.

### 2. Keep atom syntax explicit and dialect-neutral

**Choice:** Atom admission should use Trellis-owned character classes and escaping rules rather than inheriting Scheme, Common Lisp, SMT-LIB, or Steel syntax wholesale.

**Rationale:** Compatibility dialects carry many corner cases. A small declared atom grammar makes verification and downstream claims reviewable.

### 3. Serialize canonically before supporting permissive parsing

**Choice:** The first checked surface should prove canonical serialization shape and `parse(canonical_serialize(x)) == x` for admitted finite values. Permissive parsing, comments, quoting sugar, datum labels, and reader abbreviations stay out of scope.

**Rationale:** Canonical serialization is the stable identity layer. A permissive reader can be added later as an adapter that normalizes into this core AST.

### 4. Make parser/admission failures deterministic

**Choice:** Parse/admission results should return a bounded success value or an explicit error class such as invalid atom, unbalanced delimiter, trailing input, non-canonical token, escape error, size budget exceeded, or depth budget exceeded.

**Rationale:** Negative evidence must show malformed input is rejected predictably rather than panicking, looping, or silently accepting unsupported syntax.

### 5. Use explicit budgets for finite proofs

**Choice:** Parsing and structural traversals should carry declared size and depth budgets. Specs should state how budgets bound recursion and when under-budget inputs are rejected.

**Rationale:** Verus proofs are more stable when recursion and finite traversals expose their measure. This also prevents accidental claims about unbounded host-stack behavior.

### 6. Separate core identity from hashing

**Choice:** The primitive should expose canonical bytes/text as the identity input. If a digest is later needed, stack-owned consumers should use BLAKE3 outside the core serialization proof.

**Rationale:** The proof obligation is canonicalization and roundtrip identity, not hash-algorithm correctness.

### 7. Preserve non-claim boundaries

**Choice:** The spec and implementation must say what is not proven: evaluator semantics, Scheme compatibility, Steel compatibility, parser completeness for external dialects, macro expansion, interning, symbol hygiene, bytecode safety, filesystem/network/process effects, and whole-consumer correctness.

**Rationale:** S-expressions are often confused with Lisp/Scheme semantics. Trellis should export only the finite tree facts it proves.

## Risks / Trade-offs

- Canonical-only parsing is less convenient for users who expect comments, quote sugar, dotted pairs, or dialect-specific symbols; the benefit is a smaller proof surface.
- Parser proof work can be brittle. If needed, implementation may land serializer and AST proofs first, while keeping parsing tasks unchecked until the checked parser and negative evidence exist.
- A separate S-expression primitive overlaps conceptually with Scheme reader work. The boundary is that this change owns canonical finite tree identity; Scheme reader work owns language-specific token/span/datum behavior.

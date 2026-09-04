# Scheme Logic Specification

## Purpose

Defines the `scheme-logic` capability.

## Requirements

### Requirement: Reader Token Span Admission
r[scheme-logic.reader-token-span]
The system MUST define finite Scheme reader token and span envelopes whose token byte ranges, datum ranges, and AST child ranges are bounded by the declared source length and parent spans.

#### Scenario: Valid token spans are admitted
r[scheme-logic.reader-token-span.valid]
- GIVEN a finite token stream whose token spans are ordered, non-overlapping where required, and bounded by the source length
- WHEN reader token-span admission is evaluated
- THEN the token stream is accepted for datum-structure checks.

#### Scenario: Out-of-bounds span is rejected
r[scheme-logic.reader-token-span.out-of-bounds]
- GIVEN a token, datum, or child span whose start is after its end or whose end exceeds the declared source length or parent span
- WHEN reader token-span admission is evaluated
- THEN the candidate is rejected with deterministic span-bound metadata.

### Requirement: Reader Datum Structure Admission
r[scheme-logic.reader-datum-structure]
The system MUST define delimiter, quote, quasiquote, unquote, splice, vector opener, bytevector opener, and datum-comment structure predicates for the supported finite Scheme reader subset.

#### Scenario: Balanced delimiters admit nested data
r[scheme-logic.reader-datum-structure.balanced]
- GIVEN a finite reader envelope with matching delimiter pairs and every nested datum contained within its parent delimiter span
- WHEN datum-structure admission is evaluated
- THEN the reader envelope is accepted as structurally balanced.

#### Scenario: Datum attachment is explicit
r[scheme-logic.reader-datum-structure.attachment]
- GIVEN a quote-like token, syntax quote-like token, or datum-comment token followed by an admitted datum
- WHEN datum-structure admission is evaluated
- THEN the attachment relation identifies that following datum and preserves span containment.

#### Scenario: Unmatched delimiter is rejected
r[scheme-logic.reader-datum-structure.unmatched]
- GIVEN an open delimiter without a matching close delimiter, a close delimiter without a prior open delimiter, or mismatched delimiter kinds
- WHEN datum-structure admission is evaluated
- THEN the candidate is rejected with deterministic delimiter metadata.

### Requirement: Scheme Interning State
r[scheme-logic.interning]
The system MUST model symbol and literal interning as explicit finite state whose keys resolve to declared values, whose values map to stable keys, and whose intern/resolve roundtrips reject malformed tables instead of depending on global runtime state.

#### Scenario: Intern and resolve roundtrip
r[scheme-logic.interning.roundtrip]
- GIVEN an admitted interner table and a declared symbol or literal value
- WHEN the value is interned and the returned key is resolved
- THEN the resolved value equals the original declared value.

#### Scenario: Existing interned value keeps key
r[scheme-logic.interning.stable-key]
- GIVEN an admitted interner table already containing a symbol or literal value
- WHEN that value is interned again
- THEN the returned key is the existing key and the table does not acquire a duplicate association.

#### Scenario: Dangling intern key is rejected
r[scheme-logic.interning.dangling-key]
- GIVEN an interner table with a key that has no declared resolved value or with duplicate inconsistent associations
- WHEN interner admission is evaluated
- THEN the table is rejected with deterministic malformed-interner metadata.

### Requirement: Scheme Module Graph Admission
r[scheme-logic.module-graph]
The system MUST define finite `require`/`provide` graph admission for modules, export sets, import edges, transitive closure metadata, duplicate export rejection, and no unprovided-name leakage.

#### Scenario: Import closure resolves provided names
r[scheme-logic.module-graph.provided]
- GIVEN finite modules whose import edges reference names provided by their target modules
- WHEN module graph admission is evaluated
- THEN the graph is accepted and the resolved import closure contains only provided names.

#### Scenario: Duplicate export is rejected
r[scheme-logic.module-graph.duplicate-export]
- GIVEN a module that declares the same exported binding more than once in the admitted namespace
- WHEN module graph admission is evaluated
- THEN the module is rejected with duplicate-export metadata.

#### Scenario: Unprovided import is rejected
r[scheme-logic.module-graph.unprovided]
- GIVEN an import edge requesting a binding absent from the target module's provided names
- WHEN module graph admission is evaluated
- THEN the graph is rejected with unprovided-import metadata.

#### Scenario: Recursive traversal is bounded
r[scheme-logic.module-graph.bounded-closure]
- GIVEN a module graph with recursive or cyclic imports
- WHEN transitive import closure is evaluated
- THEN the traversal must use explicit fuel or visited-set metadata and must reject under-budget closure claims.

### Requirement: Scheme Contract and Blame Metadata
r[scheme-logic.contract-blame]
The system MUST define finite contract descriptors for flat contracts, function contracts, arity checks, wrapper composition, and blame polarity without executing user predicates or claiming full higher-order language semantics.

#### Scenario: Function contract arity is admitted
r[scheme-logic.contract-blame.arity]
- GIVEN a function contract descriptor whose domain arity and codomain arity match the wrapped callable metadata
- WHEN contract admission is evaluated
- THEN the wrapper is accepted with deterministic arity metadata.

#### Scenario: Blame polarity follows wrapper position
r[scheme-logic.contract-blame.polarity]
- GIVEN a contract wrapper where a domain failure is attributed to the caller side and a codomain failure is attributed to the provider side
- WHEN blame metadata is computed
- THEN the polarity matches the declared wrapper position.

#### Scenario: Arity mismatch is rejected
r[scheme-logic.contract-blame.arity-mismatch]
- GIVEN a function contract descriptor whose expected arity does not match the wrapped callable metadata
- WHEN contract admission is evaluated
- THEN the wrapper is rejected with arity-mismatch metadata.

### Requirement: Constant Pool and Bytecode Safety
r[scheme-logic.bytecode-safety]
The system MUST define constant-pool and bytecode admission predicates for constant deduplication, rollback prefix preservation, operand bounds, stack-effect summaries, local and capture slot bounds, and call arity checks.

#### Scenario: Constant deduplication preserves lookup
r[scheme-logic.bytecode-safety.constant-dedup]
- GIVEN an admitted constant pool and a constant value already present in that pool
- WHEN add-or-get constant admission is evaluated
- THEN the returned index names the existing constant and lookup at that index returns that value.

#### Scenario: Rollback preserves prefix
r[scheme-logic.bytecode-safety.rollback-prefix]
- GIVEN an admitted constant pool and a rollback index within the pool length
- WHEN rollback admission is evaluated
- THEN every constant before the rollback index is preserved and no retained index resolves differently.

#### Scenario: Safe bytecode stack summary is admitted
r[scheme-logic.bytecode-safety.stack-summary]
- GIVEN a finite bytecode sequence whose instruction summaries never underflow the operand stack and whose final stack height is declared
- WHEN bytecode admission is evaluated
- THEN the bytecode sequence is accepted with that stack summary.

#### Scenario: Out-of-bounds operand is rejected
r[scheme-logic.bytecode-safety.operand-bounds]
- GIVEN a bytecode instruction referencing a constant, local slot, capture slot, jump target, or call arity outside the declared finite bounds
- WHEN bytecode admission is evaluated
- THEN the bytecode sequence is rejected with operand-bound metadata.

### Requirement: Scheme Value Graph Traversal
r[scheme-logic.value-graph]
The system MUST model Scheme pair, vector, symbol, literal, and procedure-placeholder values as finite graph nodes with explicit edges, well-formed node references, and cycle-safe traversal metadata.

#### Scenario: Acyclic value graph traversal terminates
r[scheme-logic.value-graph.acyclic]
- GIVEN an admitted acyclic value graph and a root node
- WHEN display, equality, or structural traversal admission is evaluated
- THEN traversal succeeds within the declared node budget and visits only declared nodes.

#### Scenario: Cyclic value graph uses visited set or fuel
r[scheme-logic.value-graph.cyclic]
- GIVEN an admitted value graph containing a reachable cycle
- WHEN traversal admission is evaluated
- THEN the traversal must use explicit visited-set or fuel metadata and must not claim unbounded structural recursion.

#### Scenario: Dangling value edge is rejected
r[scheme-logic.value-graph.dangling-edge]
- GIVEN a pair or vector node whose edge references a node absent from the finite graph
- WHEN value graph admission is evaluated
- THEN the graph is rejected with dangling-edge metadata.

### Requirement: Scheme Collections and Bytevectors
r[scheme-logic.collections-bytevectors]
The system MUST define Scheme-specific list, vector, immutable update, hash-map/hash-set envelope, and bytevector predicates that complement Trellis generic collections with Scheme value-shape requirements.

#### Scenario: Proper list shape is admitted
r[scheme-logic.collections-bytevectors.proper-list]
- GIVEN a finite pair-chain ending in the declared empty-list value with no malformed tail
- WHEN list-shape admission is evaluated
- THEN the value is accepted as a proper list.

#### Scenario: Vector update preserves length
r[scheme-logic.collections-bytevectors.vector-update]
- GIVEN an admitted vector and an index within its declared length
- WHEN immutable vector update admission is evaluated
- THEN the result has the same declared length, updates only the target index, and preserves all other indexed values.

#### Scenario: Bytevector elements stay in byte domain
r[scheme-logic.collections-bytevectors.byte-domain]
- GIVEN an admitted bytevector envelope
- WHEN bytevector admission is evaluated
- THEN every element is representable as a byte and the declared length matches the element sequence.

#### Scenario: Improper list is rejected
r[scheme-logic.collections-bytevectors.improper-list]
- GIVEN a pair-chain whose tail is neither another pair nor the declared empty-list value
- WHEN list-shape admission is evaluated
- THEN the value is rejected with improper-list metadata.

### Requirement: Scheme Logic Positive Evidence
r[scheme-logic.evidence.positive]
The implementation MUST include positive evidence for reader spans, interning roundtrips, provided module imports, contract blame polarity, bytecode safety, cycle-safe traversal, and collection/bytevector preservation.

#### Scenario: Positive fixtures cover accepted behavior
r[scheme-logic.evidence.positive.fixtures]
- GIVEN the scheme-logic test suite
- WHEN positive fixtures run
- THEN they cover accepted examples for every public scheme-logic primitive family.

### Requirement: Scheme Logic Negative Evidence
r[scheme-logic.evidence.negative]
The implementation MUST include negative evidence for unmatched delimiters, invalid spans, dangling intern keys, duplicate exports, unprovided imports, arity mismatch, bytecode operand and stack failures, under-budget cyclic traversal, improper lists, and invalid bytevector elements.

#### Scenario: Negative fixtures cover failure modes
r[scheme-logic.evidence.negative.fixtures]
- GIVEN the scheme-logic test suite
- WHEN negative fixtures run
- THEN they reject every declared failure mode with deterministic metadata rather than panicking, silently accepting malformed input, or exporting an unsupported proof claim.

### Requirement: Scheme Logic Documentation Sync
r[scheme-logic.docs]
The implementation MUST update Trellis `docs/requirements.md`, README/module-catalog text, and accepted Cairn spec documentation for any public scheme-logic module or requirement IDs that land in source.

#### Scenario: Requirement IDs are traceable
r[scheme-logic.docs.traceable]
- GIVEN a scheme-logic module lands in `src/`
- WHEN Tracey and lifecycle validation run
- THEN matching requirement IDs appear in `docs/requirements.md`, source annotations, tests, and accepted Cairn spec text.

### Requirement: Scheme Logic Validation Evidence
r[scheme-logic.final-validation]
The change MUST capture focused validation evidence for Cargo tests, Verus verification as needed for new modules or the full library, Tracey uncovered/untested checks, and Cairn validation/gates before archive.

#### Scenario: Final validation names commands and outcomes
r[scheme-logic.final-validation.evidence]
- GIVEN the change is ready to archive
- WHEN final validation evidence is inspected
- THEN it names the commands run, their outcomes, and any scoped limitations or blockers.

### Requirement: Scheme Logic Non-Claim Boundary
r[scheme-logic.non-claims]
The scheme-logic primitive MUST state that it does not prove Steel compatibility, complete Scheme standard compliance, parser completeness, macro-expander soundness, GC correctness, JIT correctness, FFI safety, runtime module loading, filesystem or network behavior, or whole-interpreter correctness.

#### Scenario: Steel reference does not become a compatibility claim
r[scheme-logic.non-claims.steel-boundary]
- GIVEN a reviewer reads the scheme-logic evidence
- WHEN Steel is named as an inspiration source
- THEN the evidence remains limited to Trellis-owned finite predicates and does not claim that Steel or any Steel program is verified.

#### Scenario: Runtime effects stay outside the verified core
r[scheme-logic.non-claims.runtime-boundary]
- GIVEN a scheme-logic predicate accepts finite input metadata
- WHEN a reviewer reads the exported claim
- THEN the claim excludes source parsing side effects, global interner caches, dynamic module loading, bytecode execution, allocation, garbage collection, JIT compilation, FFI, filesystem I/O, network I/O, process execution, and time or randomness behavior.

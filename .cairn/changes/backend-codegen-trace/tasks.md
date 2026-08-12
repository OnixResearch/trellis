## Phase 1: Trace model

- [ ] [serial] Define abstract backend handles for blocks, values, functions, types, places, and builder operations. r[backend-trace.handles]
- [ ] [serial] Define MIR-to-backend block mapping and declared expansion-block metadata. r[backend-trace.block-mapping]
- [ ] [serial] Define lowering trace entries for MIR statements, operands, places, rvalues, and terminators. r[backend-trace.lowering-trace]

## Phase 2: Admission helpers and proofs

- [ ] [serial] Implement checked exec helpers for handle declaration/use admission, block mapping admission, builder-operation ordering, and terminator emission. r[backend-trace.admission]
- [ ] [serial] Prove helper facts that admitted branch/call/unwind traces target declared backend blocks and never reference dangling value handles. r[backend-trace.handle-validity] r[backend-trace.terminators]
- [ ] [serial] Define SSA-like local classification metadata and admission for direct-value versus memory-backed locals. r[backend-trace.ssa-like-locals]

## Phase 3: Evidence and documentation

- [ ] [serial] Add positive tests for simple block lowering, call expansion, operand lowering, place lowering, rvalue lowering, branch terminator lowering, and SSA-like local metadata. r[backend-trace.evidence]
- [ ] [serial] Add negative tests for dangling handles, invalid block mapping, missing terminator operation, out-of-order value use, undeclared expansion block, and unsupported operation kind. r[backend-trace.evidence]
- [ ] [serial] Add requirements, annotations, export wiring, and README/module-catalog text. r[backend-trace.docs]

## Phase 4: Validation and archive

- [ ] [serial] Run focused Cargo tests and Verus verification for backend-codegen trace helpers and capture evidence. r[backend-trace.final-validation]
- [ ] [serial] Run Tracey uncovered/untested checks and Cairn validation/gates before archive. r[backend-trace.final-validation]

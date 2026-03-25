## Phase 1: Rate limit nonlinear arithmetic

- [ ] Close `tokens_after_refill` — split identity via vstd div/mul lemmas
- [ ] Close `tokens_to_wait_ms` — ceiling division overflow proof
- [ ] Close `wait_enables_consumption` — chain refill+consume specs
- [ ] Verus + cargo test green

## Phase 2: Glob recursive spec

- [ ] Replace degenerate `glob_match_spec` with recursive definition
- [ ] Add `glob_segments_spec` helper for multi-segment matching
- [ ] Close `glob_match` exec fn with loop invariant
- [ ] Add proof fns for key structural cases
- [ ] Verus + cargo test green

## Phase 3: Replace strings

- [ ] Open `replace_strings_spec` — recursive left-to-right scan
- [ ] Add `first_match_spec` helper for pattern priority
- [ ] Close `replace_strings` exec fn
- [ ] Verus + cargo test green

## Phase 4: Document permanent gaps

- [ ] Add `@verified-logic:permanent-axiom` markers to crypto items (6)
- [ ] Add `@verified-logic:permanent-z3-limit` markers to version items (4)
- [ ] Add `@verified-logic:permanent-theory-bridge` marker to buddy (1)
- [ ] Update docs/requirements.md with verification gap inventory
- [ ] Archive change

# Proof-gap count derivation

## Baseline before this change

The parent commit published `19` trusted boundaries in README/canonical inventory. A fresh source scan of the unchanged trust-boundary categories shows `33` external-body sites and `3` assume-specification bridges. The instruction-selection runtime-only exemptions were not present yet in the parent.

```console
$ jj file show -r @- README.md | grep -n trusted
13:**8425 Verus proofs · 4152 tests · 1731 requirements · 373 modules · 19 trusted boundaries**
489:  BLAKE3-style hashing is modeled with uninterpreted specs plus trusted exec wrappers.
495:- **1 web-of-trust principal bridge** (`web_of_trust_principal.rs`) — principal digest bytes stay behind a trusted exec/model boundary.
509:All trusted items remain listed in `openspec/specs/verification-gaps/spec.md` with rationale.

$ rg -n "^[[:space:]]*#\\[verifier::external_body\\]" src | wc -l
33

$ rg -n "assume_specification\\[" src | wc -l
3

$ jj file show -r @- src/instruction_selection.rs | grep -c "@verified-logic:exempt runtime-only instruction-selection"
0
```

Baseline actual source-derived total before the new exemptions: `36 = 33 external_body + 3 assume_specification + 0 runtime-only instruction-selection exemptions`.

## After this change

```console
$ rg -n "^[[:space:]]*#\\[verifier::external_body\\]" src | wc -l
33

$ rg -n "^[[:space:]]*#\\[verifier::external_body\\]" src | cut -d: -f1 | sort -u | wc -l
14

$ rg -n "assume_specification\\[" src | wc -l
3

$ rg -n "@verified-logic:exempt runtime-only instruction-selection surface" src/instruction_selection.rs | wc -l
8

$ rg -n "@verified-logic:no-ensures" src | wc -l
1
```

After actual source-derived total: 44 = 33 external_body + 3 assume_specification + 8 runtime-only instruction-selection exemptions.

Permanent/closeable split used in README and openspec/specs/verification-gaps/spec.md: 29 permanent + 15 closeable = 44.

Co-located no-ensures markers: 1 already counted under external_body.

The single `@verified-logic:no-ensures` marker is on `src/ucan_ability.rs:ability_subsumes` and is already counted in the 33 external-body sites, so it is not added again.

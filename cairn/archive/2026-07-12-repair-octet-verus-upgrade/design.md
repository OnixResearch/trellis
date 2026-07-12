# Design

## Completion contract

Goal: produce a validated Octet-native Trellis upgrade, or the strongest fail-closed receipt with the exact irreducible blocker.

Completion evidence is:

1. current and candidate Verus both prove the complete Trellis crate;
2. IronKV, verified-memory-allocator, and verified-node-replication complete their native candidate proof commands;
3. all source adaptations are explicit, BLAKE3-bound, reviewable, and excluded from upstream-proof transfer claims;
4. the typed matrix reduces observed results rather than assuming success;
5. receipt freshness and negative fixtures pass.

False completion includes weakening a postcondition, adding `assume` or `external_body`, changing triggers only to suppress failure, accepting compilation as verification, patching a fixed-output source without a separate identity, counting SOSP microcases or VerusBelt as native passes, or changing the pin while any required result is blocked.

Allowed terminal outcomes are `validated` and `blocked`.

## Functional core and shell

Pure proof lemmas establish that inserting unequal byte sequences into equal prefix/suffix contexts yields unequal serializations. The shell remains the existing Nix runner: materialize exact inputs, execute bounded verifier commands, normalize diagnostics, and write evidence. Decision reduction remains a pure jq/Nickel computation over observed statuses.

## Approach registry and budget

| Family | Mechanism | Claim | First check | Initial state |
|---|---|---|---|---|
| sequence-context-cancellation | left and right cancellation around the platform field | closes the local theorem without length-case automation | candidate full-crate run | active |
| explicit-length-decomposition | expose complete serialization length as fixed context plus escaped platform length | closes only the unequal-length branch | candidate focused run | independent |
| witness-index | recover a differing escaped byte when lengths match | closes the equal-length branch | candidate focused run | independent |
| source-api-adaptation | explicit hash-bound compatibility patches for removed upstream APIs | native proof command reaches verification | per-project probe | independent |
| verifier-defect-isolation | minimize node-replication internal error without changing its claim | separates source defect from verifier defect | exact reproducer | independent |

Budget: four local proof iterations, two adaptation iterations per required-native corpus, one verifier minimization round, and two deterministic receipt reruns. Source authority is limited to the pinned catalog plus separately hashed local patches. Octet and upstream repositories retain their own proof authority.

## Adversarial audit

The surviving route must be rerun under both verifier closures, checked for source/postcondition/trust-marker drift, and challenged with forced-promotion, unknown-row, missing-patch, and mutated-evidence fixtures. A patch that changes the theorem or bypasses a native verifier is rejected even if the command exits successfully.

## Saved evidence contract

`evidence/trial/receipt.json` MUST be a `trellis-octet-verus-upgrade-trial-receipt-v2` document whose candidate and all required-native exits are zero, decision is `promote`, timeout flag is false, and adapted trust-marker counts are unchanged. Its `receipt_blake3` MUST equal BLAKE3 of `evidence/trial/payload.canonical.json` under the declared compact sorted JSON scope. `evidence/trial/adaptations/` MUST contain the exact two patches named and hashed by the receipt. The derivation MUST recursively compare the entire saved `evidence/trial/` directory against a fresh generation.

`evidence/adaptation-review.md` MUST record the finite/infinite collection API basis, allocator arithmetic argument, secondary-review disposition, and non-transfer boundary. Final promotion MUST bind the exact provider revision through the declared published GitHub input. A local Git override may establish pre-publication evidence but MUST NOT satisfy the durable pin transition.

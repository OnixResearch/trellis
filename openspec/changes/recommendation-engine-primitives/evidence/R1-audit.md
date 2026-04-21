Evidence-ID: recommendation-engine-primitives.r1
Task-ID: R1
Artifact-Type: audit-note
Covers: recommendation.scoring, recommendation.ranking, recommendation.dedup, recommendation.source-cap, recommendation.topk-pipeline

Pinned baseline revision: `@- = ltklnzopqvqz c06bedc598e8`

Audit commands run against that pre-change baseline:

```bash
jj file show -r @- src/lib.rs | rg -n 'recommendation'
jj file show -r @- README.md | rg -n 'recommendation'
jj file show -r @- docs/requirements.md | rg -n 'recommendation\.'
jj file list -r @- openspec/specs | rg 'recommendation'
```

Observed result: all four commands returned no matches.

Related generic-but-non-recommendation overlap found during the same audit:

```bash
jj file show -r @- src/lib.rs | rg -n 'eviction_score|weighted_round_robin|markov|chaoscontrol'
jj file show -r @- README.md | rg -n 'eviction_score|weighted_round_robin|markov|chaoscontrol'
```

Relevant parent-tree hits:
- `src/lib.rs`: `eviction_score`, `markov`, `chaoscontrol_*`, `weighted_round_robin`
- `README.md`: `markov`, `weighted_round_robin`, `eviction_score`

Disposition:
- No shipped `recommendation_*` module found in `src/lib.rs` at the pinned baseline.
- No `recommendation.*` requirements found in `docs/requirements.md` at the pinned baseline.
- No canonical recommendation spec found under `openspec/specs/`.
- Existing overlap is semantic adjacency only; it does not already ship deterministic recommendation scoring/ranking/dedup/source-cap/top-k primitives.
- Keep the change in net-new implementation scope; do not narrow to `R5` sync-only closure.

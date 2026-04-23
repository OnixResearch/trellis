# OpenSpec Gate Summary

Fresh reruns on 2026-04-23 after reconciling the artifacts to the landed
`src/ucan_ability.rs` surface:

- **Proposal gate**: PASS (`openspec_gate stage=proposal change=ucan-ability-hierarchy`)
- **Design gate**: PASS (`openspec_gate stage=design change=ucan-ability-hierarchy`)
- **Tasks gate**: PASS (`openspec_gate stage=tasks change=ucan-ability-hierarchy`)

## Reconciled findings

The final pass reflects these artifact fixes:
- remove stale parser, recursive-glob, and auth-integration commitments
- make the top-ability special case explicit and distinct from single-level
  wildcard matching inside longer patterns
- add `r[...]` IDs to every requirement and scenario in the delta spec
- define containment as the landed proof-layer alias to subsumption
- add explicit verification mapping and acceptance-evidence commands
- align the scope to the landed runtime matcher and proof helpers in
  `src/ucan_ability.rs`
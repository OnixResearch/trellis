# docs/requirements.md mirror plan

Draft mirror inventory for this change's Tracey-facing requirement IDs:

- `chaoscontrol-coverage.regions`
- `chaoscontrol-coverage.union`
- `chaoscontrol-coverage.classify`
- `chaoscontrol-oracle.verdict`
- `chaoscontrol-oracle.accounting`
- `chaoscontrol-scheduler.turns`
- `chaoscontrol-scheduler.fingerprint`
- `chaoscontrol-fault-schedule.replay`
- `chaoscontrol-fault-schedule.subset`
- `chaoscontrol-frontier.selection`
- `chaoscontrol-input-tree.alternatives`
- `chaoscontrol-fault-state.quorum-health`
- `chaoscontrol-fault-state.liveness-gate`
- `properties.chaoscontrol-subset-replay`
- `properties.chaoscontrol-fingerprint-trace`
- `properties.chaoscontrol-quorum-gate`

Implementation rule for this change:
- every requirement gets a `docs/requirements.md` mirror entry
- every requirement gets a concrete defining code-side `r[impl]` anchor
- every proof-style requirement also gets a code-side `r[verify]` proof artifact
- validation-only checks such as `chaoscontrol-validation.iterative-exec` stay change-local in OpenSpec specs/evidence instead of mirroring into `docs/requirements.md`

# Aspen Proof Snapshot Layout

Aspen proof snapshots are opt-in. The Aspen checkout remains the source of truth for rows in `docs/aspen-verified-logic-index.tsv` unless a row names a local snapshot manifest.

A snapshot must live under `snapshots/aspen/<slice-id>/` and include a manifest file recorded in the index row's `local_snapshot_manifest` field.

Required manifest fields:

```text
# verified-logic aspen-proof-snapshot-v1
id	<same row id as docs/aspen-verified-logic-index.tsv>
source_repo	../aspen
source_commit	<40-character Aspen commit>
source_paths	<comma-separated Aspen proof paths>
local_paths	<comma-separated files under snapshots/aspen/<id>/>
import_rationale	<why local composition or independent verification requires the copy>
verification_command	<local command used to verify the snapshot>
verification_result	pass: <short result summary>
known_trusted_boundaries	<explicit remaining trust boundary notes>
```

Rules:

- Do not snapshot a module solely to mirror Aspen.
- Keep copied proof code under the slice directory named by the row id.
- Preserve source path and commit provenance exactly.
- Re-run the local snapshot verification command before treating the snapshot as local proof evidence.
- Keep runtime shell, networking, serialization, storage, crypto, and external-system claims out of the snapshot unless the copied proof actually verifies them.

Current state: no Aspen proof code is snapshotted locally; the TSV index is evidence metadata only.

## Why

Trellis verified logic and proof modules should not depend on capability filesystem APIs, but the repo-index store and proof-evidence binary do open local repository and artifact paths. Those shell/adaptor surfaces can benefit from `cap-std` without changing proof semantics.

## What Changes

- Adopt `cap-std` only for Trellis repository-index and proof-evidence shell boundaries that operate under explicit local roots.
- Introduce typed capability roots for repository, index database, proof-evidence input, and artifact output directories.
- Keep Verus proof modules, spec/exec functions, and pure model code independent of filesystem APIs.
- Add positive and negative fixtures for valid repo-relative paths, `../`, absolute paths, symlink escapes, and database/artifact output escapes.

## Impact

- Local index and proof-evidence artifact authority becomes explicit and bounded.
- Verified-logic semantics, proof obligations, Tracey requirements, and exported primitive identities remain unchanged.
- Invalid path states deny before tooling reads or writes outside declared roots.

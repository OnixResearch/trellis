# Repository tooling filesystem boundary

Trellis repository tooling opens local directories: the repo-index binary opens a
repository root and an index database root, and the proof-evidence binary opens
an input root and an artifact output root. This document records what those
capability roots bound and what they do not claim.

## Capability roots

Each operator-declared root is opened once as a `cap_std::fs::Dir` handle by the
tooling shell. Every later read, write, directory listing, and parent creation
resolves a validated relative locator against that handle.

| Tool | Root | Operations |
|---|---|---|
| `repo-index` | repository | read tracked files, list directories, classify entries |
| `repo-index` | index database | create the parent directory, resolve the database path, remove a stale database |
| `proof_evidence` | proof-evidence input | read source and documentation inputs, list directories |
| `proof_evidence` | artifact output | write `docs/proof-evidence-bundle.json` |

## Locator validation

`RelativeLocator` admits a path only when it is non-empty, relative, and made of
plain segments. It rejects absolute paths, Windows drive prefixes, empty
segments, and `.` or `..` segments. `cap-std` additionally fails closed when a
relative path would leave the opened root through a symlink.

Fixture coverage lives beside the roots:

- positive: repository reads and listings, index database parent creation and
  removal, proof-evidence input reads, artifact output writes;
- negative: traversal and absolute locators, a missing root, a symlink escape
  from the repository root, and a symlink escape from the input root.

## Verified-logic non-claims

`cap-std` is a dependency of repository tooling only. Verified-logic modules,
`spec fn` and `exec fn` definitions, `proof fn` bodies, and pure model code do
not depend on it and do not open ambient filesystem paths.

Capability roots bound local repository-tooling filesystem authority only. They
do not prove new verified-logic properties, proof soundness, artifact truth,
whole-program correctness, or consumer release readiness.

## Dependency-profile consequence

Adding or removing a dependency changes the Octet check profile, which re-keys
every accepted finding identity in `octet-baseline.json`. Regenerate that
baseline with the pinned `cargo-octet check --write-baseline` tool after a
dependency change, deduplicating by stable finding identity, and confirm that
the accepted count and the finding set do not change beyond line drift.

## Context

We want a local index over the verified-logic repo, not a remote search
service. The repo already has enough structure to extract useful facts:
Rust modules and items in `src/`, requirement definitions in
`docs/requirements.md`, OpenSpec requirement headings and change metadata,
and guidance docs such as `README.md` and `AGENTS.md`.

Two project constraints matter.

1. This crate prefers verified public APIs. Pulling `redb` or `blake3`
   directly into a new stable public surface would force us to invent trust
   boundaries before the tool is useful.
2. Tracey coverage comes from `docs/requirements.md`, not from OpenSpec
   files alone. If the implementation grows reusable verified helpers, they
   need normal Tracey annotations and requirement text like the rest of the
   repo.

## Goals / Non-Goals

**Goals:**
- Persistent local index with fast reopen
- BLAKE3 content identity for tracked files
- redb-backed storage with atomic refresh semantics
- Cross-reference queries for paths, symbols, and requirement ids
- Incremental refresh that reuses facts for unchanged digests
- Deterministic CLI output suitable for scripting

**Non-Goals:**
- Full-text ranking or semantic/embedding search
- A background daemon or file-watcher service
- Indexing `target/`, `data/`, VCS internals, or archived OpenSpec changes
- Exposing `redb` types as a stable public library API in v1
- Modeling `redb` or BLAKE3 internals in Verus in the first round

## Decisions

### 1. Content identity is BLAKE3 over raw file bytes

**Choice:** Hash the exact bytes of every tracked file on each refresh and
use the BLAKE3 digest as the content identity key.

**Rationale:** This is deterministic, matches the repo default for
unspecified hashing, and avoids mtime/size invalidation edge cases.
`verified-logic` is large enough to benefit from indexing but still small
enough that byte hashing is simpler than maintaining metadata heuristics.

**Alternative:** Use size/mtime as the primary invalidation signal and hash
only changed files. Rejected in v1 because stale metadata would make the
cache story harder to reason about.

**Implementation:** Path rows store the current digest. Digest rows store the
extracted facts keyed by digest bytes.

### 2. redb stores committed index generations

**Choice:** Use one local redb database at `data/repo-index.redb` with
normalized tables for metadata, paths, digests, symbols, requirements, and
cross-reference edges.

**Rationale:** redb is embedded, transactional, and fits a local tool. The
repo already ignores `*.redb`, so the database can live in-tree without
adding ignore churn.

**Alternative:** SQLite or JSONL. SQLite would work, but redb is smaller in
surface area for this use case. JSONL is easy to inspect but poor for keyed
queries and atomic refresh.

**Implementation:** Keep a schema version row. Incompatible schema changes
trigger rebuild or generation reset.

### 3. Refresh publishes one generation at a time

**Choice:** Each refresh writes a new generation inside a redb write
transaction and flips a single active-generation pointer at commit.

**Rationale:** Queries must see either the previous committed index or the
new one, never a mix of both.

**Alternative:** In-place table mutation. Rejected because partial updates
would complicate failure handling and reader consistency.

**Implementation:** Generation-tagged rows are written first. Pointer swap
happens last in the transaction. Cleanup of superseded rows can happen after
commit or during a later maintenance pass.

### 4. Keep a verified-core / imperative-shell split

**Choice:** Keep filesystem walking, BLAKE3 execution, and redb I/O in
plain Rust internal code or a bin target. If we need reusable semantics,
model them in small Verus-friendly helpers that talk about index state,
query results, and refresh equivalence rather than external crate internals.

**Rationale:** This respects the repo rule that stable public APIs should be
verified while avoiding fake precision about external libraries we are not
modeling.

**Alternative:** Wrap redb and BLAKE3 in verified public APIs immediately.
Rejected in v1 because the trust and proof cost is front-loaded while the
query tool itself is still undefined.

**Implementation:** The first user-facing surface is an internal CLI or
non-public API, not a new re-exported library module.

### 5. Index active repo facts, not build artifacts

**Choice:** Default tracked roots are `src/**/*.rs`, `docs/requirements.md`,
`openspec/specs/**/*.md`, active `openspec/changes/**/*.md`, `README.md`,
and `AGENTS.md`. Ignore `target/`, `data/`, `.git/`, `.jj/`, generated
artifacts, and `openspec/changes/archive/**`.

**Rationale:** The active tree is what maintainers and agents search during
normal work. Archived OpenSpec changes and build outputs add noise.

**Implementation:** Extract at least:
- Rust module/item names and spans
- `r[...]` references from source
- Requirement ids and headings from `docs/requirements.md`
- Active OpenSpec change name, domain, and requirement headings

### 6. Query surface starts narrow and deterministic

**Choice:** Start with `build`, `stats`, `path <path>`, `symbol <name>`,
`requirement <id>`, and `digest <hex>` queries.

**Rationale:** These queries answer the common repo-navigation questions
without taking on ranking, fuzzy matching, or UI work.

**Implementation:** Default output is human-readable; add a machine-readable
mode for scripts if the CLI grows beyond ad hoc use.

## Risks / Trade-offs

**Schema migration** -> Version rows and rebuild paths keep the tool honest
when extracted row formats change.

**Digest identity depends on BLAKE3** -> We accept the normal cryptographic
assumption that collisions are out of scope for this local tooling feature.

**Extraction will start shallow** -> Early parsers will likely be string-
and pattern-based for `r[...]`, headings, and item discovery. Deeper Rust
semantics can come later if the first round proves useful.

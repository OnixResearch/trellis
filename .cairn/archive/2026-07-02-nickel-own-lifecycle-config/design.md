## Context

Trellis already treats Nickel as the preferred configuration language for policy surfaces, but lifecycle config remains handwritten YAML in two locations. The file contents are currently identical, which is a strong signal that a single typed source should generate the compatibility artifacts.

## Decisions

### 1. Nickel source, generated compatibility files

**Choice:** Add a Nickel lifecycle config source and treat existing YAML files as generated compatibility outputs unless a consumer can read JSON/Nickel directly.

**Rationale:** The repository can gain contract validation now without forcing every existing lifecycle consumer to change formats at once.

### 2. Contract validates the shape Trellis relies on

**Choice:** The contract covers:
- project name, description, language, and lifecycle schema;
- non-empty domain list;
- non-empty context text;
- known rule sections such as proposal and tasks;
- non-empty rule strings inside each configured section.

**Rationale:** These are the fields currently duplicated and used as lifecycle guidance. Rejecting malformed shape is more valuable than leaving the files as unchecked prose data.

### 3. Drift check compares generated outputs

**Choice:** Add a checker that typechecks Nickel, exports the compatibility representation, and compares it to checked-in generated YAML/JSON artifacts deterministically.

**Rationale:** Freshness checks make the Nickel source authoritative while preserving reviewable generated files.

### 4. Negative fixtures cover consumer-relevant failures

**Choice:** Include fixtures for missing project metadata, empty domains, unsupported schema, missing task rules, and malformed rule section values.

**Rationale:** Lifecycle config is small but important; sad-path fixtures show the contract actually protects the fields downstream tools expect.

## Risks / Trade-offs

- Deterministic YAML rendering can be harder than deterministic JSON. If YAML export is tool-dependent, keep a generated JSON file as the canonical freshness artifact and generate YAML with a documented stable path.
- Legacy `openspec/config.yaml` should remain compatibility-only. The change must not imply new OpenSpec lifecycle work.

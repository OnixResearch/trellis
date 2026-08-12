## Why

`cairn/config.yaml` and `openspec/config.yaml` currently duplicate the same lifecycle project metadata as untyped YAML. That makes drift easy: the two files can diverge, required fields can be removed, and rule lists can be malformed without a Nickel contract catching the error before tools consume the config.

## What Changes

- Make Nickel the source of truth for Trellis lifecycle configuration.
- Add a typed contract for project metadata, domain lists, context text, lifecycle schema name, and proposal/task rules.
- Export generated YAML or JSON artifacts for tools that still require those concrete formats.
- Add a freshness check that rejects drift between the Nickel source and checked-in generated compatibility files.
- Add positive and negative fixtures for valid config, missing fields, empty domains/rules, unsupported schema names, and malformed rule sections.

## Scope

In scope:
- Trellis lifecycle config files under `cairn/` and legacy `openspec/` compatibility config.
- Nickel contract/source files and deterministic export/freshness checks.
- Validation wiring through existing repository checks.

Out of scope:
- Changing Cairn lifecycle semantics.
- Removing legacy `openspec/` artifacts beyond replacing duplicated config content with generated compatibility output.
- Rewriting Cairn itself.

## Impact

- **Files**: `cairn/config.ncl` or equivalent, generated `cairn/config.yaml`, generated `openspec/config.yaml`, config check script, fixtures, README/CONTRIBUTING command references if needed.
- **Risks**: tools may require YAML; keep generated YAML compatibility until consumers are migrated.
- **Testing**: Nickel typecheck/export, generated config freshness, positive/negative config fixtures, Cairn validation, and repo verification wrapper.

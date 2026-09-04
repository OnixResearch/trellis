#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Deterministic invariant-surface catalog checker and generator.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

const CATALOG_PATH: &str = "docs/invariant-surface-catalog.md";
const LIB_PATH: &str = "src/lib.rs";
const REQUIREMENTS_PATH: &str = "docs/requirements.md";
const PROOF_GAP_PATH: &str = "docs/proof-gap-sites.tsv";
const CAIRN_SPECS_DIR: &str = "cairn/specs";
const TABLE_HEADER: &str =
    "| Module | Requirement | Surface | Labels | Evidence | Trusted boundary adjacency | Source |";
const TABLE_SEPARATOR: &str = "|---|---|---|---|---|---|---|";
const GENERATED_MARKER: &str =
    "<!-- generated-by: scripts/check-invariant-surface-catalog.rs; source: local deterministic facts -->";
const IDENT_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-";
const TRUSTED_YES: &str = "yes";
const TRUSTED_NO: &str = "no";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SeedRow {
    module: &'static str,
    requirement: &'static str,
    surface: &'static str,
    labels: &'static [&'static str],
    evidence: &'static str,
    trusted: bool,
    source: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CatalogRow {
    module: String,
    requirement: String,
    surface: String,
    labels: Vec<String>,
    evidence: String,
    trusted: bool,
    source: String,
}

#[derive(Clone, Debug)]
struct CatalogInputs {
    lib_rs: String,
    requirements: String,
    specs: String,
    proof_gaps: String,
    sources: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CatalogCheck {
    rows: Vec<CatalogRow>,
    errors: Vec<String>,
}

const REQUIRED_ROWS: &[SeedRow] = &[
    SeedRow {
        module: "sel4_invariants",
        requirement: "sel4-invariants.preservation-matrix",
        surface: "kernel_state_unchanged_spec",
        labels: &["state-invariant", "preservation"],
        evidence: "proof+test",
        trusted: false,
        source: "src/sel4_invariants.rs",
    },
    SeedRow {
        module: "cache_hierarchy",
        requirement: "hierarchy.inclusive-invariant",
        surface: "inclusive_invariant_spec",
        labels: &["state-invariant", "preservation"],
        evidence: "proof+test",
        trusted: false,
        source: "src/cache_hierarchy.rs",
    },
    SeedRow {
        module: "constant_folding",
        requirement: "constfold.soundness",
        surface: "fixed_point_path_sound_top_free",
        labels: &["soundness", "well-formedness"],
        evidence: "proof+test",
        trusted: false,
        source: "src/constant_folding.rs",
    },
    SeedRow {
        module: "verification_frontier",
        requirement: "verification-frontier.outcome-invariant-bundle",
        surface: "admission_is_proof_evidence_spec",
        labels: &["admission-boundary", "soundness"],
        evidence: "proof+test",
        trusted: false,
        source: "src/verification_frontier.rs",
    },
    SeedRow {
        module: "last_applied",
        requirement: "last-applied.advance",
        surface: "advance_preserves_invariant",
        labels: &["preservation"],
        evidence: "proof+test",
        trusted: false,
        source: "src/last_applied.rs",
    },
    SeedRow {
        module: "ucan_proof_traversal",
        requirement: "ucan-proof-traversal.transcript-consistency",
        surface: "traversal_transcript_valid_spec",
        labels: &["well-formedness", "admission-boundary"],
        evidence: "proof+test",
        trusted: false,
        source: "src/ucan_proof_traversal.rs",
    },
    SeedRow {
        module: "cryptoshedding",
        requirement: "cryptoshedding.chain-recoverability",
        surface: "recoverable_spec",
        labels: &["state-invariant", "soundness"],
        evidence: "proof+test",
        trusted: false,
        source: "src/cryptoshedding.rs",
    },
    SeedRow {
        module: "control_flow_graph",
        requirement: "control-flow-graph.well-formed",
        surface: "well_formed_cfg_spec",
        labels: &["well-formedness"],
        evidence: "proof+test",
        trusted: false,
        source: "src/control_flow_graph.rs",
    },
    SeedRow {
        module: "liveness_analysis",
        requirement: "liveness.equations",
        surface: "equations_hold_spec",
        labels: &["state-invariant", "catalog-only"],
        evidence: "test",
        trusted: false,
        source: "src/liveness_analysis.rs",
    },
    SeedRow {
        module: "raft_step",
        requirement: "raft-step.preserves",
        surface: "step_preserves_spec",
        labels: &["preservation"],
        evidence: "proof",
        trusted: false,
        source: "src/raft_step.rs",
    },
    SeedRow {
        module: "ghost_history",
        requirement: "ghost-history.single-vote-per-term",
        surface: "single_vote_per_term_spec",
        labels: &["state-invariant"],
        evidence: "proof+test",
        trusted: false,
        source: "src/ghost_history.rs",
    },
    SeedRow {
        module: "consensus_fast_path",
        requirement: "consensus.fast-path.quorum",
        surface: "finite_membership_intersection",
        labels: &["state-invariant", "soundness"],
        evidence: "proof",
        trusted: false,
        source: "src/consensus_fast_path/proofs/arithmetic.rs",
    },
    SeedRow {
        module: "wasm_runtime_bounds",
        requirement: "wasm-runtime-bounds.segmentation",
        surface: "segmented_steps_match_uninterrupted",
        labels: &["state-invariant", "soundness"],
        evidence: "proof",
        trusted: false,
        source: "src/wasm_runtime_bounds/proofs/control.rs",
    },
    SeedRow {
        module: "scheme_logic",
        requirement: "scheme-logic.reader-token-span",
        surface: "child_containment_is_transitive",
        labels: &["state-invariant", "soundness"],
        evidence: "proof",
        trusted: false,
        source: "src/scheme_logic/proofs/reader.rs",
    },
];

fn seed_to_row(seed: &SeedRow) -> CatalogRow {
    CatalogRow {
        module: seed.module.to_string(),
        requirement: seed.requirement.to_string(),
        surface: seed.surface.to_string(),
        labels: seed.labels.iter().map(|label| (*label).to_string()).collect(),
        evidence: seed.evidence.to_string(),
        trusted: seed.trusted,
        source: seed.source.to_string(),
    }
}

fn default_rows() -> Vec<CatalogRow> {
    REQUIRED_ROWS.iter().map(seed_to_row).collect()
}

fn render_catalog(rows: &[CatalogRow]) -> String {
    let mut sorted = rows.to_vec();
    sorted.sort_by(|left, right| left.module.cmp(&right.module).then(left.surface.cmp(&right.surface)));

    let mut out = String::new();
    out.push_str("# Invariant Surface Catalog\n\n");
    out.push_str(GENERATED_MARKER);
    out.push_str("\n\n");
    out.push_str("This catalog exposes objective local facts for selected public invariant, well-formedness, preservation, soundness, and admission-boundary surfaces. Labels are source-fact classifications, not proof-quality scores. `test` evidence is intentionally distinct from `proof` evidence. Run `./scripts/check-invariant-surface-catalog.rs --write` to refresh and `./scripts/check-invariant-surface-catalog.rs` to validate.\n\n");
    out.push_str(TABLE_HEADER);
    out.push('\n');
    out.push_str(TABLE_SEPARATOR);
    out.push('\n');
    for row in sorted {
        out.push_str("| `");
        out.push_str(&row.module);
        out.push_str("` | `");
        out.push_str(&row.requirement);
        out.push_str("` | `");
        out.push_str(&row.surface);
        out.push_str("` | ");
        out.push_str(&row.labels.join(", "));
        out.push_str(" | ");
        out.push_str(&row.evidence);
        out.push_str(" | ");
        out.push_str(if row.trusted { TRUSTED_YES } else { TRUSTED_NO });
        out.push_str(" | [`");
        out.push_str(&row.source);
        out.push_str("`](../");
        out.push_str(&row.source);
        out.push_str(") |\n");
    }
    out
}

fn parse_catalog(text: &str) -> Result<Vec<CatalogRow>, String> {
    let mut rows = Vec::new();
    let mut in_table = false;
    for (line_index, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed == TABLE_HEADER {
            in_table = true;
            continue;
        }
        if !in_table || trimmed == TABLE_SEPARATOR || trimmed.is_empty() {
            continue;
        }
        if !trimmed.starts_with('|') {
            break;
        }
        let cells: Vec<String> = trimmed.trim_matches('|').split('|').map(|cell| cell.trim().to_string()).collect();
        let expected_cells = 7usize;
        if cells.len() != expected_cells {
            return Err(format!("invalid catalog row at line {}: expected {expected_cells} cells", line_index + 1));
        }
        rows.push(CatalogRow {
            module: code_cell(&cells[0])?,
            requirement: code_cell(&cells[1])?,
            surface: code_cell(&cells[2])?,
            labels: split_labels(&cells[3]),
            evidence: cells[4].to_string(),
            trusted: parse_trusted(&cells[5])?,
            source: source_cell(&cells[6])?,
        });
    }
    if rows.is_empty() {
        Err("catalog table has no rows".to_string())
    } else {
        Ok(rows)
    }
}

fn code_cell(cell: &str) -> Result<String, String> {
    cell.strip_prefix('`')
        .and_then(|value| value.strip_suffix('`'))
        .map(|value| value.to_string())
        .ok_or_else(|| format!("expected backtick code cell, got `{cell}`"))
}

fn split_labels(cell: &str) -> Vec<String> {
    cell.split(',').map(str::trim).filter(|label| !label.is_empty()).map(ToString::to_string).collect()
}

fn parse_trusted(cell: &str) -> Result<bool, String> {
    match cell {
        TRUSTED_YES => Ok(true),
        TRUSTED_NO => Ok(false),
        _ => Err(format!("trusted-boundary cell must be `{TRUSTED_YES}` or `{TRUSTED_NO}`, got `{cell}`")),
    }
}

fn source_cell(cell: &str) -> Result<String, String> {
    let Some(start) = cell.find("[`") else {
        return Err(format!("expected markdown source link, got `{cell}`"));
    };
    let rest = &cell[start + 2..];
    let Some(end) = rest.find('`') else {
        return Err(format!("unterminated source link code cell `{cell}`"));
    };
    Ok(rest[..end].to_string())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ModuleSources {
    root: String,
    component_dir: String,
}

fn path_attribute(trimmed: &str) -> Option<&str> {
    trimmed.strip_prefix("#[path = \"")?.strip_suffix("\"]")
}

fn declared_module_sources(lib_rs: &str) -> BTreeMap<String, ModuleSources> {
    let mut modules = BTreeMap::new();
    let mut pending_path = None;
    for line in lib_rs.lines() {
        let trimmed = line.trim();
        if let Some(path) = path_attribute(trimmed) {
            pending_path = Some(path.to_string());
            continue;
        }
        if !trimmed.starts_with("pub mod ") {
            continue;
        }
        let name = trimmed.trim_start_matches("pub mod ").trim_end_matches(';');
        if !is_ident(name) {
            pending_path = None;
            continue;
        }
        let root = pending_path.take().map_or_else(|| format!("src/{name}.rs"), |path| format!("src/{path}"));
        let component_dir = root.rsplit_once('/').map_or_else(
            || format!("src/{name}/"),
            |(parent, _)| {
                if parent == "src" {
                    format!("src/{name}/")
                } else {
                    format!("{parent}/")
                }
            },
        );
        modules.insert(name.to_string(), ModuleSources { root, component_dir });
    }
    modules
}

fn module_owns_source(module: &ModuleSources, source: &str) -> bool {
    source == module.root || source.starts_with(&module.component_dir)
}

fn proof_gap_sources(proof_gaps: &str) -> BTreeSet<String> {
    proof_gaps
        .lines()
        .filter(|line| line.starts_with("src/"))
        .filter_map(|line| line.split('\t').next())
        .map(ToString::to_string)
        .collect()
}

fn is_ident(token: &str) -> bool {
    !token.is_empty() && token.chars().all(|ch| IDENT_CHARS.contains(ch))
}

fn allowed_label(label: &str) -> bool {
    matches!(
        label,
        "state-invariant" | "well-formedness" | "preservation" | "soundness" | "admission-boundary" | "catalog-only"
    )
}

fn allowed_evidence(evidence: &str) -> bool {
    matches!(evidence, "proof" | "test" | "proof+test" | "prose")
}

fn requirement_marker(requirement: &str) -> String {
    format!("r[{requirement}]")
}

fn source_has_requirement_marker(source: &str, requirement: &str) -> bool {
    source.contains(&format!("r[impl {requirement}]"))
        || source.contains(&format!("r[verify {requirement}]"))
        || source.contains(&format!("r[depends {requirement}]"))
}

fn source_has_proof_for_requirement(source: &str, requirement: &str) -> bool {
    source.contains(&format!("r[verify {requirement}]"))
}

fn source_has_test_evidence(source: &str, requirement: &str) -> bool {
    source.contains(&format!("r[verify {requirement}]")) && (source.contains("#[test]") || source.contains("proptest!"))
}

fn validate_rows(rows: Vec<CatalogRow>, inputs: &CatalogInputs, required: &[SeedRow]) -> CatalogCheck {
    let modules = declared_module_sources(&inputs.lib_rs);
    let trusted_sources = proof_gap_sources(&inputs.proof_gaps);
    let all_requirement_text = format!("{}\n{}", inputs.requirements, inputs.specs);
    let mut errors = Vec::new();
    let mut seen = BTreeSet::new();

    for required_row in required {
        let key = (required_row.module.to_string(), required_row.surface.to_string());
        if !rows.iter().any(|row| row.module == required_row.module && row.surface == required_row.surface) {
            errors.push(format!(
                "missing invariant catalog row for `{}` / `{}`",
                required_row.module, required_row.surface
            ));
        }
        seen.insert(key);
    }

    for row in &rows {
        let key = (row.module.clone(), row.surface.clone());
        if !seen.contains(&key) {
            errors.push(format!(
                "catalog row `{}` / `{}` is not part of the deterministic row set",
                row.module, row.surface
            ));
        }
        let Some(module_sources) = modules.get(&row.module) else {
            errors.push(format!("catalog row `{}` references a non-public module", row.module));
            continue;
        };
        if !module_owns_source(module_sources, &row.source) {
            errors.push(format!(
                "catalog row `{}` source `{}` is outside its public module tree",
                row.module, row.source
            ));
        }
        let Some(source) = inputs.sources.get(&row.source) else {
            errors.push(format!("catalog row `{}` references missing source `{}`", row.module, row.source));
            continue;
        };
        if !source.contains(&row.surface) {
            errors.push(format!(
                "catalog row `{}` surface `{}` is stale or missing from `{}`",
                row.module, row.surface, row.source
            ));
        }
        let marker = requirement_marker(&row.requirement);
        if !all_requirement_text.contains(&marker) && !source_has_requirement_marker(source, &row.requirement) {
            errors.push(format!(
                "catalog row `{}` requirement `{}` is not found in requirements, Cairn specs, or source markers",
                row.module, row.requirement
            ));
        }
        if row.labels.is_empty() {
            errors.push(format!("catalog row `{}` has no labels", row.module));
        }
        for label in &row.labels {
            if !allowed_label(label) {
                errors.push(format!("catalog row `{}` has unsupported label `{label}`", row.module));
            }
        }
        if !allowed_evidence(&row.evidence) {
            errors.push(format!("catalog row `{}` has unsupported evidence `{}`", row.module, row.evidence));
        }
        if row.evidence.contains("proof") && !source_has_proof_for_requirement(source, &row.requirement) {
            errors.push(format!("catalog row `{}` overclaims proof evidence for `{}`", row.module, row.requirement));
        }
        if row.evidence.contains("test") && !source_has_test_evidence(source, &row.requirement) {
            errors.push(format!(
                "catalog row `{}` claims test evidence for `{}` but no scoped test marker exists",
                row.module, row.requirement
            ));
        }
        let source_is_trusted_adjacent = trusted_sources.contains(&row.source);
        if row.trusted && !source_is_trusted_adjacent {
            errors.push(format!(
                "catalog row `{}` marks trusted adjacency but `{}` is absent from proof-gap inventory",
                row.module, row.source
            ));
        }
        if source_is_trusted_adjacent && !row.trusted {
            errors.push(format!("catalog row `{}` omits trusted-boundary adjacency for `{}`", row.module, row.source));
        }
    }

    CatalogCheck { rows, errors }
}

fn validate_catalog_text(text: &str, inputs: &CatalogInputs, required: &[SeedRow]) -> CatalogCheck {
    match parse_catalog(text) {
        Ok(rows) => validate_rows(rows, inputs, required),
        Err(error) => CatalogCheck {
            rows: Vec::new(),
            errors: vec![error],
        },
    }
}

fn collect_cairn_specs(root: &Path) -> Result<String, String> {
    let mut specs = Vec::new();
    let specs_root = root.join(CAIRN_SPECS_DIR);
    collect_spec_files(&specs_root, &mut specs)?;
    specs.sort();
    let mut out = String::new();
    for path in specs {
        let text = fs::read_to_string(&path).map_err(|err| format!("failed to read {}: {err}", path.display()))?;
        out.push_str(&text);
        out.push('\n');
    }
    Ok(out)
}

fn collect_spec_files(path: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(path).map_err(|err| format!("failed to read {}: {err}", path.display()))? {
        let entry = entry.map_err(|err| err.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_spec_files(&path, out)?;
        } else if path.file_name().and_then(|name| name.to_str()) == Some("spec.md") {
            out.push(path);
        }
    }
    Ok(())
}

fn load_inputs(root: &Path) -> Result<CatalogInputs, String> {
    let lib_rs = fs::read_to_string(root.join(LIB_PATH)).map_err(|err| format!("failed to read {LIB_PATH}: {err}"))?;
    let requirements = fs::read_to_string(root.join(REQUIREMENTS_PATH))
        .map_err(|err| format!("failed to read {REQUIREMENTS_PATH}: {err}"))?;
    let proof_gaps = fs::read_to_string(root.join(PROOF_GAP_PATH)).unwrap_or_default();
    let specs = collect_cairn_specs(root)?;
    let mut sources = BTreeMap::new();
    for seed in REQUIRED_ROWS {
        if sources.contains_key(seed.source) {
            continue;
        }
        let text = fs::read_to_string(root.join(seed.source))
            .map_err(|err| format!("failed to read {}: {err}", seed.source))?;
        sources.insert(seed.source.to_string(), text);
    }
    Ok(CatalogInputs {
        lib_rs,
        requirements,
        specs,
        proof_gaps,
        sources,
    })
}

fn run_check(root: &Path) -> Result<(), String> {
    let inputs = load_inputs(root)?;
    let catalog =
        fs::read_to_string(root.join(CATALOG_PATH)).map_err(|err| format!("failed to read {CATALOG_PATH}: {err}"))?;
    let rendered = render_catalog(&default_rows());
    let check = validate_catalog_text(&catalog, &inputs, REQUIRED_ROWS);
    if !check.errors.is_empty() {
        for error in check.errors {
            eprintln!("✗ {error}");
        }
        return Err("invariant surface catalog validation failed".to_string());
    }
    if catalog != rendered {
        return Err(format!("{CATALOG_PATH} is stale; run `./scripts/check-invariant-surface-catalog.rs --write`"));
    }
    println!("✓ invariant surface catalog covers {} required rows", REQUIRED_ROWS.len());
    Ok(())
}

fn run_write(root: &Path) -> Result<(), String> {
    let inputs = load_inputs(root)?;
    let rendered = render_catalog(&default_rows());
    let check = validate_catalog_text(&rendered, &inputs, REQUIRED_ROWS);
    if !check.errors.is_empty() {
        for error in check.errors {
            eprintln!("✗ {error}");
        }
        return Err("generated invariant surface catalog failed validation".to_string());
    }
    fs::write(root.join(CATALOG_PATH), rendered).map_err(|err| format!("failed to write {CATALOG_PATH}: {err}"))?;
    println!("✓ wrote {CATALOG_PATH}");
    Ok(())
}

fn fixture_inputs(source: &str, proof_gaps: &str) -> CatalogInputs {
    let mut sources = BTreeMap::new();
    sources.insert("src/example.rs".to_string(), source.to_string());
    CatalogInputs {
        lib_rs: "pub mod example;\n".to_string(),
        requirements: "r[example.rule]\nExample rule MUST hold.\n".to_string(),
        specs: String::new(),
        proof_gaps: proof_gaps.to_string(),
        sources,
    }
}

fn fixture_seed() -> SeedRow {
    SeedRow {
        module: "example",
        requirement: "example.rule",
        surface: "example_invariant_spec",
        labels: &["state-invariant"],
        evidence: "proof+test",
        trusted: false,
        source: "src/example.rs",
    }
}

fn fixture_row(seed: &SeedRow) -> CatalogRow {
    seed_to_row(seed)
}

fn expect_error_contains(name: &str, check: CatalogCheck, needle: &str) -> Result<(), String> {
    if check.errors.iter().any(|error| error.contains(needle)) {
        return Ok(());
    }
    Err(format!(
        "fixture `{name}` did not report expected error containing `{needle}`; got {:?}",
        check.errors
    ))
}

fn temp_root() -> Result<PathBuf, String> {
    let mut root = std::env::current_dir().map_err(|err| err.to_string())?;
    root.push("target-test");
    root.push("invariant-surface-catalog-fixtures");
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|err| err.to_string())?.as_nanos();
    root.push(nanos.to_string());
    fs::create_dir_all(&root).map_err(|err| err.to_string())?;
    Ok(root)
}

fn run_self_test() -> Result<(), String> {
    let seed = fixture_seed();
    let valid_source = "// r[impl example.rule]\npub open spec fn example_invariant_spec() -> bool { true }\n// r[verify example.rule]\nproof fn example_verified() {}\n#[test]\nfn example_test() {}\n";
    let valid_inputs = fixture_inputs(valid_source, "");
    let valid_catalog = render_catalog(&[fixture_row(&seed)]);
    let valid_check = validate_catalog_text(&valid_catalog, &valid_inputs, &[seed]);
    if !valid_check.errors.is_empty() {
        return Err(format!("valid fixture failed: {:?}", valid_check.errors));
    }

    let nested_seed = SeedRow {
        source: "src/example/proofs/arithmetic.rs",
        ..seed
    };
    let mut nested_inputs = fixture_inputs(valid_source, "");
    nested_inputs.lib_rs = "#[path = \"example/identity.rs\"]\npub mod example;\n".to_string();
    nested_inputs.sources.clear();
    nested_inputs.sources.insert(nested_seed.source.to_string(), valid_source.to_string());
    let nested_check =
        validate_catalog_text(&render_catalog(&[fixture_row(&nested_seed)]), &nested_inputs, &[nested_seed]);
    if !nested_check.errors.is_empty() {
        return Err(format!("explicit-path fixture failed: {:?}", nested_check.errors));
    }

    let outside_seed = SeedRow {
        source: "src/example-other/proofs.rs",
        ..seed
    };
    let mut outside_inputs = nested_inputs.clone();
    outside_inputs.sources.clear();
    outside_inputs.sources.insert(outside_seed.source.to_string(), valid_source.to_string());
    expect_error_contains(
        "explicit-path-source-escape",
        validate_catalog_text(&render_catalog(&[fixture_row(&outside_seed)]), &outside_inputs, &[outside_seed]),
        "outside its public module tree",
    )?;

    let missing_catalog = format!("# Empty\n\n{GENERATED_MARKER}\n\n{TABLE_HEADER}\n{TABLE_SEPARATOR}\n");
    expect_error_contains("missing-row", validate_catalog_text(&missing_catalog, &valid_inputs, &[seed]), "no rows")?;

    let stale_inputs =
        fixture_inputs("// r[impl example.rule]\n// r[verify example.rule]\n#[test]\nfn example_test() {}\n", "");
    expect_error_contains(
        "stale-row",
        validate_catalog_text(&valid_catalog, &stale_inputs, &[seed]),
        "stale or missing",
    )?;

    let overclaim_inputs = fixture_inputs(
        "// r[impl example.rule]\npub open spec fn example_invariant_spec() -> bool { true }\n#[test]\nfn example_test() {}\n",
        "",
    );
    expect_error_contains(
        "proof-overclaim",
        validate_catalog_text(&valid_catalog, &overclaim_inputs, &[seed]),
        "overclaims proof",
    )?;

    let test_only_seed = SeedRow {
        evidence: "test",
        ..seed
    };
    let test_only_source = "// r[impl example.rule]\npub open spec fn example_invariant_spec() -> bool { true }\n// r[verify example.rule]\n#[test]\nfn example_test() {}\n";
    let test_only_check = validate_catalog_text(
        &render_catalog(&[fixture_row(&test_only_seed)]),
        &fixture_inputs(test_only_source, ""),
        &[test_only_seed],
    );
    if !test_only_check.errors.is_empty() {
        return Err(format!("test-only fixture should not overclaim proof: {:?}", test_only_check.errors));
    }

    let trusted_inputs = fixture_inputs(valid_source, "src/example.rs\tpub fn trusted(\tpermanent\tfixture\n");
    expect_error_contains(
        "trusted-omission",
        validate_catalog_text(&valid_catalog, &trusted_inputs, &[seed]),
        "omits trusted-boundary adjacency",
    )?;

    let root = temp_root()?;
    fs::create_dir_all(root.join("docs")).map_err(|err| err.to_string())?;
    fs::write(root.join("docs/.keep"), "").map_err(|err| err.to_string())?;
    println!("✓ invariant catalog checker self-test passed");
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.as_slice() {
        [] => run_check(Path::new(".")),
        [flag] if flag == "--self-test" => run_self_test(),
        [flag] if flag == "--write" => run_write(Path::new(".")),
        [flag, root] if flag == "--write" => run_write(Path::new(root)),
        [root] => run_check(Path::new(root)),
        _ => Err("usage: check-invariant-surface-catalog.rs [--self-test|--write [ROOT]|ROOT]".to_string()),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("✗ {error}");
            ExitCode::FAILURE
        }
    }
}

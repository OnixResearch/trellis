#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Deterministic module maturity index generator/checker.
//!
//! The pure core derives objective module facts from in-memory source, example,
//! requirement, and proof-gap inputs. The imperative shell only reads/writes
//! files and reports diagnostics.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

const INDEX_PATH: &str = "docs/module-index.md";
const LIB_PATH: &str = "src/lib.rs";
const REQUIREMENTS_PATH: &str = "docs/requirements.md";
const PROOF_GAP_PATH: &str = "docs/proof-gap-sites.tsv";
const EXAMPLES_DIR: &str = "examples";
const SRC_DIR: &str = "src";
const MODULE_DECL_PREFIX: &str = "pub mod ";
const PATH_ATTR_PREFIX: &str = "#[path = \"";
const PATH_ATTR_SUFFIX: &str = "\"]";
const R_IMPL_PREFIX: &str = "r[impl ";
const R_VERIFY_PREFIX: &str = "r[verify ";
const SPEC_FN_MARKER: &str = "spec fn";
const PUB_FN_MARKER: &str = "pub fn ";
const TEST_ATTR_MARKER: &str = "#[test]";
const PROPTEST_MARKER: &str = "proptest";
const MOD_TESTS_MARKER: &str = "mod tests";
const FACADE_MARKER: &str = "facade";
const COMPOSITE_MARKER: &str = "composite";
const INDEX_ROW_PREFIX: &str = "| `";
const INDEX_ROW_SEPARATOR: &str = " | ";
const MISSING_DOC_SENTINEL: &str = "—";
const SCHEMA_LINE: &str = "<!-- generated-by: scripts/check-module-index.rs; source: local deterministic facts -->";
const MIN_SUPPORTED_SCHEMA_VERSION: &str = "module-index-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
struct SourceFile {
    module: String,
    path: String,
    content: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExampleFile {
    name: String,
    content: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct IndexInputs {
    lib_rs: String,
    requirements: String,
    proof_gaps: String,
    sources: Vec<SourceFile>,
    examples: Vec<ExampleFile>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ModuleFacts {
    name: String,
    labels: BTreeSet<String>,
    impl_markers: usize,
    verify_markers: usize,
    requirement_mentions: usize,
    examples: BTreeSet<String>,
    proof_gaps: usize,
    source_path: String,
}

#[derive(Debug, Eq, PartialEq)]
struct IndexCheck {
    expected: String,
    errors: Vec<String>,
}

fn is_ident(token: &str) -> bool {
    !token.is_empty() && token.chars().all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn extract_path_attr(line: &str) -> Option<String> {
    line.strip_prefix(PATH_ATTR_PREFIX)
        .and_then(|rest| rest.strip_suffix(PATH_ATTR_SUFFIX))
        .filter(|path| !path.starts_with('/') && !path.contains("..") && !path.contains('\\'))
        .map(str::to_string)
}

fn extract_declared_module_paths(lib_rs: &str) -> BTreeMap<String, String> {
    let mut modules = BTreeMap::new();
    let mut pending_path = None;
    for line in lib_rs.lines() {
        let trimmed = line.trim_start();
        if let Some(path) = extract_path_attr(trimmed) {
            pending_path = Some(path);
            continue;
        }
        if !trimmed.starts_with(MODULE_DECL_PREFIX) {
            pending_path = None;
            continue;
        }
        let rest = &trimmed[MODULE_DECL_PREFIX.len()..];
        let Some(name) = rest.strip_suffix(';') else {
            pending_path = None;
            continue;
        };
        if is_ident(name) {
            let relative = pending_path
                .take()
                .map(|path| format!("{SRC_DIR}/{path}"))
                .unwrap_or_else(|| format!("{SRC_DIR}/{name}.rs"));
            modules.insert(name.to_string(), relative);
        } else {
            pending_path = None;
        }
    }
    modules
}

fn extract_declared_modules(lib_rs: &str) -> BTreeSet<String> {
    extract_declared_module_paths(lib_rs).into_keys().collect()
}

fn count_occurrences(haystack: &str, needle: &str) -> usize {
    haystack.match_indices(needle).count()
}

fn proof_gap_counts(proof_gaps: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for line in proof_gaps.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((path, _rest)) = trimmed.split_once('\t') else {
            continue;
        };
        let Some(src_name) = path.strip_prefix("src/") else {
            continue;
        };
        let Some(module) = src_name.strip_suffix(".rs") else {
            continue;
        };
        if is_ident(module) {
            *counts.entry(module.to_string()).or_insert(0) += 1;
        }
    }
    counts
}

fn module_source_map(sources: &[SourceFile]) -> BTreeMap<String, SourceFile> {
    sources.iter().map(|source| (source.module.clone(), source.clone())).collect()
}

fn example_references(module: &str, examples: &[ExampleFile]) -> BTreeSet<String> {
    let direct_use = format!("verified_logic::{module}");
    let grouped_use = format!("verified_logic::{{{module}");
    let grouped_after_comma = format!(", {module}");
    let mut names = BTreeSet::new();
    for example in examples {
        if example.content.contains(&direct_use)
            || (example.content.contains("verified_logic::{") && example.content.contains(&grouped_use))
            || (example.content.contains("verified_logic::{") && example.content.contains(&grouped_after_comma))
        {
            names.insert(example.name.clone());
        }
    }
    names
}

fn source_has_tests(source: &str) -> bool {
    source.contains(TEST_ATTR_MARKER) || source.contains(PROPTEST_MARKER) || source.contains(MOD_TESTS_MARKER)
}

fn labels_for_source(
    source: &str,
    impl_markers: usize,
    verify_markers: usize,
    examples: usize,
    proof_gaps: usize,
) -> BTreeSet<String> {
    let mut labels = BTreeSet::new();
    let source_lower = source.to_ascii_lowercase();
    if source.contains(PUB_FN_MARKER) {
        labels.insert("runtime-helper".to_string());
    }
    if source.contains(SPEC_FN_MARKER) {
        labels.insert("spec-model".to_string());
    }
    if source_has_tests(source) {
        labels.insert("test-backed".to_string());
    }
    if source_lower.contains(FACADE_MARKER) || source_lower.contains(COMPOSITE_MARKER) {
        labels.insert("composition-facade".to_string());
    }
    if impl_markers > 0 && verify_markers > 0 {
        labels.insert("requirement-covered".to_string());
    } else if impl_markers > 0 || verify_markers > 0 {
        labels.insert("partial-requirement-coverage".to_string());
    }
    if examples > 0 {
        labels.insert("example-backed".to_string());
    }
    if proof_gaps > 0 {
        labels.insert("trusted-boundary-adjacent".to_string());
    }
    if labels.is_empty() {
        labels.insert("declared-public-module".to_string());
    }
    labels
}

fn collect_module_facts(inputs: &IndexInputs) -> Result<Vec<ModuleFacts>, String> {
    let declared = extract_declared_modules(&inputs.lib_rs);
    let sources = module_source_map(&inputs.sources);
    let proof_gaps = proof_gap_counts(&inputs.proof_gaps);
    let mut facts = Vec::new();

    for module in declared {
        let Some(source_file) = sources.get(&module) else {
            return Err(format!("missing source file for public module `{module}`"));
        };
        let source = &source_file.content;
        let impl_markers = count_occurrences(source, R_IMPL_PREFIX);
        let verify_markers = count_occurrences(source, R_VERIFY_PREFIX);
        let requirement_mentions = count_occurrences(&inputs.requirements, &format!("{module}."));
        let examples = example_references(&module, &inputs.examples);
        let proof_gap_count = proof_gaps.get(&module).copied().unwrap_or_default();
        let labels = labels_for_source(source, impl_markers, verify_markers, examples.len(), proof_gap_count);
        facts.push(ModuleFacts {
            name: module,
            labels,
            impl_markers,
            verify_markers,
            requirement_mentions,
            examples,
            proof_gaps: proof_gap_count,
            source_path: source_file.path.clone(),
        });
    }

    Ok(facts)
}

fn join_backticked(values: &BTreeSet<String>) -> String {
    if values.is_empty() {
        return MISSING_DOC_SENTINEL.to_string();
    }
    values.iter().map(|value| format!("`{value}`")).collect::<Vec<_>>().join(", ")
}

fn example_links(values: &BTreeSet<String>) -> String {
    if values.is_empty() {
        return MISSING_DOC_SENTINEL.to_string();
    }
    values
        .iter()
        .map(|value| format!("[`{value}`](../examples/{value}.rs)"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn tracey_cell(facts: &ModuleFacts) -> String {
    format!(
        "impl: `{}`, verify: `{}`, requirements mentions: `{}`",
        facts.impl_markers, facts.verify_markers, facts.requirement_mentions
    )
}

fn proof_gap_cell(facts: &ModuleFacts) -> String {
    if facts.proof_gaps == 0 {
        MISSING_DOC_SENTINEL.to_string()
    } else {
        format!("[`{}`](proof-gap-sites.tsv)", facts.proof_gaps)
    }
}

fn render_index(inputs: &IndexInputs) -> Result<String, String> {
    let facts = collect_module_facts(inputs)?;
    let gap_modules = facts.iter().filter(|module| module.proof_gaps > 0).count();
    let example_modules = facts.iter().filter(|module| !module.examples.is_empty()).count();
    let covered_modules = facts.iter().filter(|module| module.impl_markers > 0 && module.verify_markers > 0).count();

    let mut output = String::new();
    output.push_str("# Module Maturity Index\n\n");
    output.push_str(SCHEMA_LINE);
    output.push_str("\n\n");
    output.push_str("Schema: `");
    output.push_str(MIN_SUPPORTED_SCHEMA_VERSION);
    output.push_str("`. This index is generated from local deterministic facts in `src/lib.rs`, `src/*.rs`, `docs/requirements.md`, `docs/proof-gap-sites.tsv`, and `examples/*.rs`. Run `./scripts/check-module-index.rs --write` to refresh and `./scripts/check-module-index.rs` to validate.\n\n");
    output.push_str("Labels are objective facts, not endorsement scores: `runtime-helper` means the module exposes public runtime functions, `spec-model` means it contains Verus specs, `requirement-covered` means source-local Tracey implementation and verification markers are both present, `test-backed` means source-local tests or proptests are present, `example-backed` means a checked runnable example imports the module, `composition-facade` means the source describes a facade/composite surface, and `trusted-boundary-adjacent` means the proof-gap inventory names the module. Proof-focused runnable examples are documented in [proof-examples.md](proof-examples.md).\n\n");
    output.push_str(&format!(
        "Summary: `{}` public modules, `{}` requirement-covered modules, `{}` example-backed modules, `{}` trusted-boundary-adjacent modules.\n\n",
        facts.len(), covered_modules, example_modules, gap_modules
    ));
    output.push_str("| Module | Labels | Examples | Tracey facts | Proof gaps | Source |\n");
    output.push_str("|---|---|---|---|---|---|\n");
    for module in facts {
        output.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | [`{}`](../{}) |\n",
            module.name,
            join_backticked(&module.labels),
            example_links(&module.examples),
            tracey_cell(&module),
            proof_gap_cell(&module),
            module.source_path,
            module.source_path
        ));
    }
    Ok(output)
}

fn extract_index_modules(markdown: &str) -> BTreeSet<String> {
    let mut modules = BTreeSet::new();
    for line in markdown.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with(INDEX_ROW_PREFIX) || !trimmed.contains(INDEX_ROW_SEPARATOR) {
            continue;
        }
        let rest = &trimmed[INDEX_ROW_PREFIX.len()..];
        let Some(end) = rest.find('`') else {
            continue;
        };
        let module = &rest[..end];
        if is_ident(module) {
            modules.insert(module.to_string());
        }
    }
    modules
}

fn validate_index(inputs: &IndexInputs, actual: &str) -> IndexCheck {
    let expected = match render_index(inputs) {
        Ok(expected) => expected,
        Err(err) => {
            return IndexCheck {
                expected: String::new(),
                errors: vec![err],
            };
        }
    };
    if actual == expected {
        return IndexCheck {
            expected,
            errors: Vec::new(),
        };
    }

    let expected_modules = extract_index_modules(&expected);
    let actual_modules = extract_index_modules(actual);
    let mut errors = Vec::new();
    for module in expected_modules.difference(&actual_modules) {
        errors.push(format!("module index is missing public module `{module}`"));
    }
    for module in actual_modules.difference(&expected_modules) {
        errors.push(format!("module index contains stale module `{module}`"));
    }
    if errors.is_empty() {
        errors.push(format!(
            "{INDEX_PATH} is stale or inconsistent with local source facts; run `./scripts/check-module-index.rs --write`"
        ));
    }
    IndexCheck { expected, errors }
}

fn read_to_string(path: &Path, display: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|err| format!("failed to read {display}: {err}"))
}

fn load_inputs(root: &Path) -> Result<IndexInputs, String> {
    let lib_rs = read_to_string(&root.join(LIB_PATH), LIB_PATH)?;
    let requirements = read_to_string(&root.join(REQUIREMENTS_PATH), REQUIREMENTS_PATH)?;
    let proof_gaps = read_to_string(&root.join(PROOF_GAP_PATH), PROOF_GAP_PATH)?;
    let declared = extract_declared_module_paths(&lib_rs);
    let mut sources = Vec::new();
    for (module, relative) in declared {
        sources.push(SourceFile {
            module,
            path: relative.clone(),
            content: read_to_string(&root.join(&relative), &relative)?,
        });
    }

    let mut examples = Vec::new();
    let example_dir = root.join(EXAMPLES_DIR);
    if example_dir.exists() {
        for entry in fs::read_dir(&example_dir).map_err(|err| format!("failed to read {EXAMPLES_DIR}: {err}"))? {
            let entry = entry.map_err(|err| format!("failed to read {EXAMPLES_DIR} entry: {err}"))?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
                continue;
            };
            examples.push(ExampleFile {
                name: stem.to_string(),
                content: read_to_string(&path, &format!("{EXAMPLES_DIR}/{stem}.rs"))?,
            });
        }
        examples.sort_by(|left, right| left.name.cmp(&right.name));
    }

    Ok(IndexInputs {
        lib_rs,
        requirements,
        proof_gaps,
        sources,
        examples,
    })
}

fn run_check(root: &Path) -> Result<(), String> {
    let inputs = load_inputs(root)?;
    let actual = read_to_string(&root.join(INDEX_PATH), INDEX_PATH)?;
    let check = validate_index(&inputs, &actual);
    if check.errors.is_empty() {
        let module_count = collect_module_facts(&inputs)?.len();
        println!("✓ module maturity index covers {module_count} public modules");
        Ok(())
    } else {
        Err(check.errors.join("\n"))
    }
}

fn run_write(root: &Path) -> Result<(), String> {
    let inputs = load_inputs(root)?;
    let expected = render_index(&inputs)?;
    fs::write(root.join(INDEX_PATH), expected).map_err(|err| format!("failed to write {INDEX_PATH}: {err}"))?;
    println!("✓ wrote {INDEX_PATH}");
    Ok(())
}

fn fixture_inputs() -> IndexInputs {
    IndexInputs {
        lib_rs: "pub mod alpha;\npub mod beta;\n".to_string(),
        requirements: "r[alpha.example]\n".to_string(),
        proof_gaps: "# fixture\nsrc/beta.rs\tpub fn trusted(\tpermanent-trust-boundary\tfixture\n".to_string(),
        sources: vec![
            SourceFile {
                module: "alpha".to_string(),
                path: "src/alpha.rs".to_string(),
                content: "// r[impl alpha.example]\n// r[verify alpha.example]\npub open spec fn alpha_spec() -> bool { true }\npub fn alpha() -> bool { true }\n#[cfg(test)] mod tests { #[test] fn ok() {} }\n".to_string(),
            },
            SourceFile {
                module: "beta".to_string(),
                path: "src/beta.rs".to_string(),
                content: "//! beta facade\npub fn beta() -> bool { true }\n".to_string(),
            },
        ],
        examples: vec![ExampleFile {
            name: "alpha_demo".to_string(),
            content: "use verified_logic::alpha::alpha;\n".to_string(),
        }],
    }
}

fn assert_invalid(inputs: &IndexInputs, actual: &str, expected_error: &str) -> Result<(), String> {
    let check = validate_index(inputs, actual);
    if check.errors.iter().any(|error| error.contains(expected_error)) {
        Ok(())
    } else {
        Err(format!("expected error containing `{expected_error}`, got: {}", check.errors.join("; ")))
    }
}

fn run_self_test() -> Result<(), String> {
    let inputs = fixture_inputs();
    let valid = render_index(&inputs)?;
    let positive = validate_index(&inputs, &valid);
    if !positive.errors.is_empty() {
        return Err(format!("valid fixture failed: {}", positive.errors.join("; ")));
    }

    let missing = valid.lines().filter(|line| !line.starts_with("| `beta` |")).collect::<Vec<_>>().join("\n") + "\n";
    assert_invalid(&inputs, &missing, "missing public module `beta`")?;

    let stale = valid.replace(
        "| `beta` |",
        "| `removed` | `declared-public-module` | — | impl: `0`, verify: `0`, requirements mentions: `0` | — | [`src/removed.rs`](../src/removed.rs) |\n| `beta` |",
    );
    assert_invalid(&inputs, &stale, "stale module `removed`")?;

    let inconsistent = valid.replace("[`1`](proof-gap-sites.tsv)", MISSING_DOC_SENTINEL);
    assert_invalid(&inputs, &inconsistent, "stale or inconsistent")?;

    let root = self_test_root()?;
    let _ = fs::remove_dir_all(root);
    println!("✓ module index checker self-test fixtures passed");
    Ok(())
}

fn self_test_root() -> Result<PathBuf, String> {
    let mut root = std::env::current_dir().map_err(|err| err.to_string())?;
    root.push("target-test");
    root.push("module-index-fixtures");
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|err| err.to_string())?.as_nanos();
    root.push(nanos.to_string());
    fs::create_dir_all(&root).map_err(|err| err.to_string())?;
    Ok(root)
}

fn repo_root() -> PathBuf {
    if let Ok(output) = std::process::Command::new("git").args(["rev-parse", "--show-toplevel"]).output() {
        if output.status.success() {
            return PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.as_slice() {
        [] => run_check(&repo_root()),
        [flag] if flag == "--self-test" => run_self_test(),
        [flag] if flag == "--write" => run_write(&repo_root()),
        [flag, root] if flag == "--write" => run_write(Path::new(root)),
        [root] => run_check(Path::new(root)),
        _ => Err("usage: check-module-index.rs [--self-test|--write [ROOT]|ROOT]".to_string()),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("✗ {err}");
            ExitCode::FAILURE
        }
    }
}

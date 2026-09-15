#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript
//! ```cargo
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```

//! Build and validate the measured Aeneas/Lean coverage frontier and the
//! requirement statement registry.
//!
//! Pure construction and validation live in sibling modules. This shell owns
//! bounded file reads, generated-file publication, and exit status.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

use model::FrontierArtifact;
use model::ObservationsFile;
use model::RegistryArtifact;
use model::TargetsFile;

const OBSERVATIONS_PATH: &str = "aeneas-frontier/observations.json";
const TARGETS_PATH: &str = "aeneas-frontier/targets.json";
const FRONTIER_PATH: &str = "aeneas-frontier/generated/frontier.json";
const REGISTRY_PATH: &str = "aeneas-frontier/generated/registry.json";
const REQUIREMENTS_PATH: &str = "docs/requirements.md";
const CLAIM_DOCUMENTS: [&str; 2] = [
    "docs/aeneas-cross-verifier-pilot.md",
    "docs/aeneas-lean-coverage-frontier.md",
];
const SOURCE_DIRECTORY: &str = "src";
const MODULE_SUFFIX: &str = ".rs";
const MAX_INPUT_BYTES: u64 = 16_777_216;
const MODE_CHECK: &str = "--check";
const MODE_WRITE: &str = "--write";
const MODE_SELF_TEST: &str = "--self-test";
const ROOT_FLAG: &str = "--root";
const HELP_FLAG: &str = "--help";
const SHORT_HELP_FLAG: &str = "-h";
const USAGE: &str = "usage: check-aeneas-frontier.rs [--root PATH] [--check|--write|--self-test]";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Load {
    InputsOnly,
    WithGenerated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Check,
    Write,
    SelfTest,
}

#[derive(Debug)]
struct Options {
    root: PathBuf,
    mode: Mode,
}

struct Inputs {
    inventory: Vec<String>,
    hypotheses: BTreeMap<String, String>,
    observations: ObservationsFile,
    targets: TargetsFile,
    requirement_ids: Vec<String>,
    frontier: Option<FrontierArtifact>,
    registry: Option<RegistryArtifact>,
}

fn main() -> ExitCode {
    match run() {
        Ok(message) => {
            println!("{message}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("aeneas frontier check failed: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<String, String> {
    let options = parse_args()?;
    let root = options
        .root
        .canonicalize()
        .map_err(|error| format!("canonicalize root `{}`: {error}", options.root.display()))?;
    match options.mode {
        Mode::Write => write_generated(&root),
        Mode::Check => {
            let inputs = load_inputs(&root, Load::WithGenerated)?;
            require_clean(&validate_inputs(&inputs))?;
            require_clean(&validate_claim_documents(&root)?)?;
            let frontier = build_frontier(&inputs);
            let registry = validation::build_registry(&inputs.requirement_ids, &inputs.targets);
            compare_generated(&root, &inputs, &frontier, &registry)?;
            Ok(success_message(&inputs, &frontier, &registry))
        }
        Mode::SelfTest => {
            let negatives = tests::run()?;
            Ok(format!(
                "aeneas frontier self-test passed: 1 positive frontier, 1 positive registry, {negatives} negative cases"
            ))
        }
    }
}

fn write_generated(root: &Path) -> Result<String, String> {
    let inputs = load_inputs(root, Load::InputsOnly)?;
    require_clean(&validate_inputs(&inputs))?;
    require_clean(&validate_claim_documents(root)?)?;
    let frontier = build_frontier(&inputs);
    let registry = validation::build_registry(&inputs.requirement_ids, &inputs.targets);
    require_clean(&validation::validate_frontier(&frontier, &inputs.inventory, &inputs.observations))?;
    require_clean(&validation::validate_registry(&registry, &inputs.requirement_ids, &inputs.targets))?;
    write_json(root, FRONTIER_PATH, &frontier)?;
    write_json(root, REGISTRY_PATH, &registry)?;
    Ok(success_message(&inputs, &frontier, &registry))
}

fn build_frontier(inputs: &Inputs) -> FrontierArtifact {
    let rows = validation::build_rows(&inputs.inventory, &inputs.hypotheses, &inputs.observations);
    validation::build_frontier(&inputs.observations.provider, rows)
}

fn validate_inputs(inputs: &Inputs) -> Vec<String> {
    let mut diagnostics = validation::validate_observations(&inputs.observations);
    diagnostics.extend(validation::validate_targets(&inputs.targets, &inputs.requirement_ids));
    for module in &inputs.inventory {
        if !inputs.hypotheses.contains_key(module) {
            diagnostics.push(format!("module `{module}` has no hypothesis class"));
        }
    }
    for observation in &inputs.observations.observations {
        if !inputs.inventory.contains(&observation.module) {
            diagnostics.push(format!("observation module `{}` is not in the inventory", observation.module));
        }
    }
    for target in &inputs.targets.targets {
        let measured = inputs
            .observations
            .observations
            .iter()
            .find(|observation| observation.module == target.module)
            .map(|observation| observation.measured_class.as_str());
        let target_is_unmeasured = match measured {
            Some(class) => class == model::CLASS_UNMEASURED,
            None => true,
        };
        if target_is_unmeasured {
            diagnostics
                .push(format!("target `{}` selects unmeasured module `{}`", target.requirement_id, target.module));
        }
    }
    diagnostics
}

fn validate_claim_documents(root: &Path) -> Result<Vec<String>, String> {
    let mut diagnostics = Vec::new();
    for path in CLAIM_DOCUMENTS {
        let bytes = read_relative(root, path)?;
        let text = String::from_utf8(bytes).map_err(|error| format!("`{path}` is not UTF-8: {error}"))?;
        diagnostics.extend(validation::validate_claim_text(path, &text));
    }
    Ok(diagnostics)
}

fn compare_generated(
    root: &Path,
    inputs: &Inputs,
    frontier: &FrontierArtifact,
    registry: &RegistryArtifact,
) -> Result<(), String> {
    require_clean(&validation::validate_frontier(frontier, &inputs.inventory, &inputs.observations))?;
    require_clean(&validation::validate_registry(registry, &inputs.requirement_ids, &inputs.targets))?;
    let recorded_frontier = inputs
        .frontier
        .as_ref()
        .ok_or_else(|| format!("generated `{FRONTIER_PATH}` is missing; run --write"))?;
    let recorded_registry = inputs
        .registry
        .as_ref()
        .ok_or_else(|| format!("generated `{REGISTRY_PATH}` is missing; run --write"))?;
    if pretty_bytes(recorded_frontier)? != pretty_bytes(frontier)? {
        return Err(format!("generated `{FRONTIER_PATH}` is stale; run --write"));
    }
    if pretty_bytes(recorded_registry)? != pretty_bytes(registry)? {
        return Err(format!("generated `{REGISTRY_PATH}` is stale; run --write"));
    }
    let _ = root;
    Ok(())
}

fn success_message(inputs: &Inputs, frontier: &FrontierArtifact, registry: &RegistryArtifact) -> String {
    format!(
        "aeneas frontier passed: {} modules ({} measured, {} blocked, {} excluded, {} unmeasured), {} requirements ({} targeted), {} observations",
        frontier.counts.total_modules,
        frontier.counts.measured,
        frontier.counts.blocked,
        frontier.counts.excluded,
        frontier.counts.unmeasured,
        registry.counts.total_requirements,
        registry.counts.targeted,
        inputs.observations.observations.len()
    )
}

fn load_inputs(root: &Path, load: Load) -> Result<Inputs, String> {
    let inventory = scan_inventory(root)?;
    let mut hypotheses = BTreeMap::new();
    for module in &inventory {
        let bytes = read_relative(root, module)?;
        let contents = String::from_utf8(bytes).map_err(|error| format!("module `{module}` is not UTF-8: {error}"))?;
        hypotheses.insert(module.clone(), validation::hypothesis_for_source(&contents).to_owned());
    }
    let observations_bytes = read_relative(root, OBSERVATIONS_PATH)?;
    let observations: ObservationsFile = parse_json(&observations_bytes, OBSERVATIONS_PATH)?;
    let targets_bytes = read_relative(root, TARGETS_PATH)?;
    let targets: TargetsFile = parse_json(&targets_bytes, TARGETS_PATH)?;
    let requirement_ids = parse_requirement_ids(&read_relative(root, REQUIREMENTS_PATH)?)?;
    let (frontier, registry) = match load {
        Load::InputsOnly => (None, None),
        Load::WithGenerated => (
            load_optional::<FrontierArtifact>(root, FRONTIER_PATH, true)?,
            load_optional::<RegistryArtifact>(root, REGISTRY_PATH, true)?,
        ),
    };
    Ok(Inputs {
        inventory,
        hypotheses,
        observations,
        targets,
        requirement_ids,
        frontier,
        registry,
    })
}

fn scan_inventory(root: &Path) -> Result<Vec<String>, String> {
    let directory = root.join(SOURCE_DIRECTORY);
    let entries = fs::read_dir(&directory).map_err(|error| format!("read `{}`: {error}", directory.display()))?;
    let mut modules = BTreeSet::new();
    for entry in entries {
        let entry = entry.map_err(|error| format!("read entry in `{}`: {error}", directory.display()))?;
        let file_type = entry.file_type().map_err(|error| format!("inspect `{}`: {error}", entry.path().display()))?;
        if !file_type.is_file() {
            continue;
        }
        let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        if !name.ends_with(MODULE_SUFFIX) {
            continue;
        }
        modules.insert(format!("{SOURCE_DIRECTORY}/{name}"));
    }
    Ok(modules.into_iter().collect())
}

fn parse_requirement_ids(bytes: &[u8]) -> Result<Vec<String>, String> {
    let text =
        String::from_utf8(bytes.to_vec()).map_err(|error| format!("`{REQUIREMENTS_PATH}` is not UTF-8: {error}"))?;
    let mut identifiers = BTreeSet::new();
    for line in text.lines() {
        let Some(rest) = line.strip_prefix("r[") else {
            continue;
        };
        let Some(identifier) = rest.strip_suffix(']') else {
            continue;
        };
        if identifier.is_empty() || identifier.contains(' ') {
            return Err(format!("requirement identifier `{identifier}` is malformed"));
        }
        // The requirements surface currently repeats a small number of IDs.
        // The registry keys on the unique identifier set.
        let _ = identifiers.insert(identifier.to_owned());
    }
    if identifiers.is_empty() {
        return Err(format!("`{REQUIREMENTS_PATH}` declares no requirement identifiers"));
    }
    Ok(identifiers.into_iter().collect())
}

fn load_optional<T: serde::de::DeserializeOwned>(root: &Path, path: &str, required: bool) -> Result<Option<T>, String> {
    let full_path = checked_path(root, path)?;
    match fs::metadata(&full_path) {
        Ok(metadata) if metadata.is_file() => {
            let bytes = read_bounded(&full_path, &metadata)?;
            parse_json(&bytes, path).map(Some)
        }
        Ok(_) => Err(format!("generated path `{path}` is not a regular file")),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && !required => Ok(None),
        Err(error) => Err(format!("read generated path `{path}`: {error}")),
    }
}

fn read_relative(root: &Path, path: &str) -> Result<Vec<u8>, String> {
    let full_path = checked_path(root, path)?;
    let metadata = fs::symlink_metadata(&full_path).map_err(|error| format!("inspect `{path}`: {error}"))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(format!("input `{path}` is not a regular non-symlink file"));
    }
    read_bounded(&full_path, &metadata)
}

fn read_bounded(path: &Path, metadata: &fs::Metadata) -> Result<Vec<u8>, String> {
    if metadata.len() > MAX_INPUT_BYTES {
        return Err(format!("input `{}` exceeds {MAX_INPUT_BYTES} bytes", path.display()));
    }
    fs::read(path).map_err(|error| format!("read `{}`: {error}", path.display()))
}

fn checked_path(root: &Path, path: &str) -> Result<PathBuf, String> {
    let relative = Path::new(path);
    if relative.as_os_str().is_empty()
        || relative.is_absolute()
        || relative.components().any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(format!("unsafe relative path `{path}`"));
    }
    Ok(root.join(relative))
}

fn parse_json<T: serde::de::DeserializeOwned>(bytes: &[u8], label: &str) -> Result<T, String> {
    serde_json::from_slice(bytes).map_err(|error| format!("parse `{label}`: {error}"))
}

fn pretty_bytes<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, String> {
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|error| format!("serialize generated value: {error}"))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn write_json<T: serde::Serialize>(root: &Path, path: &str, value: &T) -> Result<(), String> {
    let destination = checked_path(root, path)?;
    let parent = destination.parent().ok_or_else(|| format!("generated path `{path}` has no parent"))?;
    fs::create_dir_all(parent).map_err(|error| format!("create `{}`: {error}", parent.display()))?;
    let bytes = pretty_bytes(value)?;
    let temporary = destination.with_extension("json.tmp");
    fs::write(&temporary, bytes).map_err(|error| format!("write `{}`: {error}", temporary.display()))?;
    fs::rename(&temporary, &destination).map_err(|error| format!("publish `{}`: {error}", destination.display()))
}

fn require_clean(diagnostics: &[String]) -> Result<(), String> {
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics.join("; "))
    }
}

fn parse_args() -> Result<Options, String> {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let mut root = PathBuf::from(".");
    let mut mode = None;
    let mut index = 0usize;
    while index < arguments.len() {
        match arguments[index].as_str() {
            ROOT_FLAG => {
                index += 1;
                root = PathBuf::from(arguments.get(index).ok_or_else(|| format!("missing value for {ROOT_FLAG}"))?);
            }
            MODE_CHECK => set_mode(&mut mode, Mode::Check)?,
            MODE_WRITE => set_mode(&mut mode, Mode::Write)?,
            MODE_SELF_TEST => set_mode(&mut mode, Mode::SelfTest)?,
            HELP_FLAG | SHORT_HELP_FLAG => return Err(USAGE.to_owned()),
            unknown => return Err(format!("unknown argument `{unknown}`; {USAGE}")),
        }
        index += 1;
    }
    Ok(Options {
        root,
        mode: mode.unwrap_or(Mode::Check),
    })
}

fn set_mode(mode: &mut Option<Mode>, selected: Mode) -> Result<(), String> {
    if mode.replace(selected).is_some() {
        Err(format!("select exactly one mode; {USAGE}"))
    } else {
        Ok(())
    }
}

#[path = "aeneas_frontier/model.rs"]
mod model;
#[path = "aeneas_frontier/tests.rs"]
mod tests;
#[path = "aeneas_frontier/validation.rs"]
mod validation;

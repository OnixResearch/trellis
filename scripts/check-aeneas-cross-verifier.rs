#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript
//! ```cargo
//! [dependencies]
//! blake3 = "=1.5.5"
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```

//! Validate the exact-source Trellis Aeneas cross-verifier pilot.
//!
//! Pure validation and receipt construction live in sibling modules. This shell
//! owns bounded file reads, atomic generated-file publication, and exit status.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

use model::CrossVerifierManifest;
use model::NegativeCatalog;
use model::Profile;
use model::ProofReceipt;
use model::ResourcePolicy;
use validation::Inputs;

const PROFILE_PATH: &str = "aeneas-pilot/generated/profile.json";
const RECEIPT_PATH: &str = "aeneas-pilot/generated/aeneas-proof-receipt.json";
const MANIFEST_PATH: &str = "aeneas-pilot/generated/cross-verifier-manifest.json";
const MAX_INPUT_BYTES: u64 = 16_777_216;
const MODE_CHECK: &str = "--check";
const MODE_WRITE: &str = "--write";
const MODE_SELF_TEST: &str = "--self-test";
const ROOT_FLAG: &str = "--root";
const HELP_FLAG: &str = "--help";
const SHORT_HELP_FLAG: &str = "-h";
const USAGE: &str = "usage: check-aeneas-cross-verifier.rs [--root PATH] [--check|--write|--self-test]";

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

fn main() -> ExitCode {
    match run() {
        Ok(message) => {
            println!("{message}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("aeneas cross-verifier check failed: {error}");
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
            let inputs = load_inputs(&root, true)?;
            require_clean(validation::validate_complete(&inputs))?;
            Ok(success_message(&inputs))
        }
        Mode::SelfTest => {
            let inputs = load_inputs(&root, true)?;
            tests::run(&inputs)?;
            Ok(format!(
                "aeneas cross-verifier self-test passed: {} positive profile, {} negative cases",
                inputs.profile.pilot_id,
                inputs.negative.cases.len()
            ))
        }
    }
}

fn write_generated(root: &Path) -> Result<String, String> {
    let mut inputs = load_inputs(root, false)?;
    require_clean(validation::validate_base(&inputs))?;
    let receipt = validation::build_receipt(&inputs)?;
    write_json(root, RECEIPT_PATH, &receipt)?;
    inputs.receipt = Some(receipt.clone());
    let manifest = validation::build_manifest(&inputs, &receipt)?;
    write_json(root, MANIFEST_PATH, &manifest)?;
    inputs.manifest = Some(manifest);
    require_clean(validation::validate_complete(&inputs))?;
    Ok(success_message(&inputs))
}

fn success_message(inputs: &Inputs) -> String {
    let receipt = inputs.receipt.as_ref().map(|value| value.receipt_identity.as_str()).unwrap_or("unwritten");
    let manifest = inputs.manifest.as_ref().map(|value| value.manifest_identity.as_str()).unwrap_or("unwritten");
    format!(
        "aeneas cross-verifier passed: {} theorems, receipt {receipt}, manifest {manifest}",
        inputs.profile.theorem_mappings.len()
    )
}

fn load_inputs(root: &Path, require_generated: bool) -> Result<Inputs, String> {
    let profile_bytes = read_relative(root, PROFILE_PATH)?;
    let profile: Profile = parse_json(&profile_bytes, PROFILE_PATH)?;
    let mut materials = BTreeMap::new();
    for artifact in &profile.artifacts {
        insert_material(root, &artifact.path, &mut materials)?;
    }
    for source in &profile.consumer.source.paths {
        insert_material(root, &source.path, &mut materials)?;
    }
    insert_material(root, &profile.verus.receipt_path, &mut materials)?;
    let llbc_path = artifact_path(&profile, "llbc")?;
    let translation_path = artifact_path(&profile, "translation-manifest")?;
    let llbc = parse_json(material(&materials, &llbc_path)?, &llbc_path)?;
    let translation = parse_json(material(&materials, &translation_path)?, &translation_path)?;
    let resource: ResourcePolicy =
        parse_json(material(&materials, &profile.resource_policy.path)?, &profile.resource_policy.path)?;
    let negative_bytes = read_relative(root, &profile.negative_fixture_catalog)?;
    materials.insert(profile.negative_fixture_catalog.clone(), negative_bytes.clone());
    let negative: NegativeCatalog = parse_json(&negative_bytes, &profile.negative_fixture_catalog)?;
    let receipt = load_optional::<ProofReceipt>(root, RECEIPT_PATH, require_generated)?;
    let manifest = load_optional::<CrossVerifierManifest>(root, MANIFEST_PATH, require_generated)?;
    Ok(Inputs {
        profile,
        profile_bytes,
        materials,
        llbc,
        translation,
        resource,
        negative,
        receipt,
        manifest,
    })
}

fn insert_material(root: &Path, path: &str, materials: &mut BTreeMap<String, Vec<u8>>) -> Result<(), String> {
    if !materials.contains_key(path) {
        materials.insert(path.to_owned(), read_relative(root, path)?);
    }
    Ok(())
}

fn artifact_path(profile: &Profile, role: &str) -> Result<String, String> {
    profile
        .artifacts
        .iter()
        .find(|artifact| artifact.role == role)
        .map(|artifact| artifact.path.clone())
        .ok_or_else(|| format!("profile artifact role `{role}` is missing"))
}

fn material<'a>(materials: &'a BTreeMap<String, Vec<u8>>, path: &str) -> Result<&'a [u8], String> {
    materials.get(path).map(Vec::as_slice).ok_or_else(|| format!("loaded material `{path}` is missing"))
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
        return Err(format!("input `{}` exceeds {} bytes", path.display(), MAX_INPUT_BYTES));
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

fn write_json<T: serde::Serialize>(root: &Path, path: &str, value: &T) -> Result<(), String> {
    let destination = checked_path(root, path)?;
    let parent = destination.parent().ok_or_else(|| format!("generated path `{path}` has no parent"))?;
    fs::create_dir_all(parent).map_err(|error| format!("create `{}`: {error}", parent.display()))?;
    let bytes = identity::pretty_bytes(value)?;
    let temporary = destination.with_extension("json.tmp");
    fs::write(&temporary, bytes).map_err(|error| format!("write `{}`: {error}", temporary.display()))?;
    fs::rename(&temporary, &destination).map_err(|error| format!("publish `{}`: {error}", destination.display()))
}

fn require_clean(diagnostics: Vec<String>) -> Result<(), String> {
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

#[path = "aeneas_cross_verifier/artifact_validation.rs"]
mod artifact_validation;
#[path = "aeneas_cross_verifier/identity.rs"]
mod identity;
#[path = "aeneas_cross_verifier/model.rs"]
mod model;
#[path = "aeneas_cross_verifier/profile_validation.rs"]
mod profile_validation;
#[path = "aeneas_cross_verifier/receipt_validation.rs"]
mod receipt_validation;
#[path = "aeneas_cross_verifier/tests.rs"]
mod tests;
#[path = "aeneas_cross_verifier/validation.rs"]
mod validation;

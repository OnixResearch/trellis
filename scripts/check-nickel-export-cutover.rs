#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript
//! ```cargo
//! [dependencies]
//! blake3 = "=1.5.5"
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```

//! Check the Trellis cutover to the standalone Nickel export shell.
//!
//! Pure profile, identity, receipt, manifest, and claim checks live in
//! `validation`. This shell owns explicit tool resolution, files, processes,
//! generated outputs, parity execution, and bounded negative fixtures.

use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitCode;

#[path = "nickel_export_cutover/boundary.rs"]
mod boundary;
#[path = "nickel_export_cutover/invalid.rs"]
mod invalid;
#[path = "nickel_export_cutover/model.rs"]
mod model;
#[path = "nickel_export_cutover/parity.rs"]
mod parity;
#[path = "nickel_export_cutover/shell.rs"]
mod shell;
#[path = "nickel_export_cutover/tests.rs"]
mod tests;
#[path = "nickel_export_cutover/validation.rs"]
mod validation;

use model::EXPECTED_EXPORTER_REVISION;

const FLAG_ROOT: &str = "--root";
const FLAG_NICKEL: &str = "--nickel";
const FLAG_EXPORTER: &str = "--nickel-export";
const FLAG_SOURCE_REVISION: &str = "--source-revision";
const MODE_CHECK: &str = "--check";
const MODE_WRITE: &str = "--write";
const MODE_SELF_TEST: &str = "--self-test";
const FLAG_HELP: &str = "--help";
const FLAG_HELP_SHORT: &str = "-h";
const FIRST_ARGUMENT: usize = 1;
const USAGE: &str = "usage: check-nickel-export-cutover.rs [--root PATH] [--nickel PATH] [--nickel-export PATH] [--source-revision REV] [--check|--write|--self-test]";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Check,
    Write,
    SelfTest,
}

#[derive(Debug)]
struct Options {
    root: PathBuf,
    nickel: Option<PathBuf>,
    exporter: Option<PathBuf>,
    source_revision: String,
    mode: Mode,
}

fn main() -> ExitCode {
    match run() {
        Ok(message) => {
            println!("{message}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("standalone Nickel export cutover failed: {error}");
            ExitCode::FAILURE
        }
    }
}

// r[impl trellis.nickel_export_cutover.boundary]
fn run() -> Result<String, String> {
    let options = parse_args(env::args().collect())?;
    let root = options
        .root
        .canonicalize()
        .map_err(|error| format!("canonicalize {}: {error}", options.root.display()))?;
    boundary::check_release_pin(&root)?;
    let tools = shell::resolve_tools(options.nickel, options.exporter)?;
    shell::verify_tools(&tools, &options.source_revision, model::EXPECTED_NICKEL_VERSION_OUTPUT)?;

    match options.mode {
        Mode::Write => {
            let rows = parity::write(&root, &tools, &options.source_revision)?;
            let profile = parity::load_profile(&root)?;
            let invalid_count = invalid::check_fixture_parity(&root, &tools, &profile)?;
            Ok(format!(
                "standalone Nickel export write passed: {} outputs, {invalid_count} invalid fixtures",
                rows.len()
            ))
        }
        Mode::Check => {
            let rows = parity::check(&root, &tools, &options.source_revision)?;
            let profile = parity::load_profile(&root)?;
            let invalid_count = invalid::check_fixture_parity(&root, &tools, &profile)?;
            Ok(format!(
                "standalone Nickel export check passed: {} exact outputs, {invalid_count} invalid fixtures",
                rows.len()
            ))
        }
        Mode::SelfTest => {
            let profile = parity::load_profile(&root)?;
            validation::validate_profile(&profile, &options.source_revision)?;
            let negative_count = tests::run(&root, &tools, &profile)?;
            Ok(format!("standalone Nickel export self-test passed: {negative_count} negative cases"))
        }
    }
}

fn parse_args(args: Vec<String>) -> Result<Options, String> {
    let mut root = None;
    let mut nickel = None;
    let mut exporter = None;
    let mut source_revision = None;
    let mut mode = None;
    let mut index = FIRST_ARGUMENT;
    while index < args.len() {
        match args[index].as_str() {
            FLAG_ROOT => root = Some(PathBuf::from(next_value(&args, &mut index, FLAG_ROOT)?)),
            FLAG_NICKEL => {
                nickel = Some(PathBuf::from(next_value(&args, &mut index, FLAG_NICKEL)?));
            }
            FLAG_EXPORTER => {
                exporter = Some(PathBuf::from(next_value(&args, &mut index, FLAG_EXPORTER)?));
            }
            FLAG_SOURCE_REVISION => {
                source_revision = Some(next_value(&args, &mut index, FLAG_SOURCE_REVISION)?.to_owned());
            }
            MODE_CHECK => set_mode(&mut mode, Mode::Check)?,
            MODE_WRITE => set_mode(&mut mode, Mode::Write)?,
            MODE_SELF_TEST => set_mode(&mut mode, Mode::SelfTest)?,
            FLAG_HELP | FLAG_HELP_SHORT => return Err(USAGE.to_owned()),
            unknown => return Err(format!("unknown argument `{unknown}`; {USAGE}")),
        }
        index = index.checked_add(1).ok_or("argument index overflow")?;
    }
    let root = match root {
        Some(path) => path,
        None => default_root()?,
    };
    Ok(Options {
        root,
        nickel,
        exporter,
        source_revision: source_revision.unwrap_or_else(|| EXPECTED_EXPORTER_REVISION.to_owned()),
        mode: mode.unwrap_or(Mode::Check),
    })
}

fn next_value<'a>(args: &'a [String], index: &mut usize, flag: &str) -> Result<&'a str, String> {
    *index = index.checked_add(1).ok_or("argument index overflow")?;
    args.get(*index).map(String::as_str).ok_or_else(|| format!("missing value for `{flag}`"))
}

fn set_mode(mode: &mut Option<Mode>, value: Mode) -> Result<(), String> {
    if mode.replace(value).is_some() {
        Err("choose exactly one execution mode".to_owned())
    } else {
        Ok(())
    }
}

fn default_root() -> Result<PathBuf, String> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|error| format!("resolve repository root: {error}"))?;
    if output.status.success() {
        let root = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
        Ok(PathBuf::from(root.trim()))
    } else {
        env::current_dir().map_err(|error| error.to_string())
    }
}

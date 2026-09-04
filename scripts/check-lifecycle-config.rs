#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang nixpkgs#nickel -c cargo -q -Zscript

//! Typecheck the Nickel lifecycle config and freshness-check generated YAML compatibility files.

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

const SELF_TEST_ARG: &str = "--self-test";
const LEGACY_PARITY_ARG: &str = "--legacy-parity";
const HELP_ARG: &str = "--help";
const HELP_SHORT_ARG: &str = "-h";
const NICKEL_BIN: &str = "nickel";
const EXPORT_ARG: &str = "export";
const FORMAT_ARG: &str = "--format";
const YAML_FORMAT: &str = "yaml";
const CONFIG_SOURCE: &str = "cairn/config.ncl";
const GENERATED_CONFIGS: &[&str] = &["cairn/config.yaml", "openspec/config.yaml"];
const FIXTURE_DIR: &str = "cairn/lifecycle-config-fixtures";
const VALID_PREFIX: &str = "valid-";
const INVALID_PREFIX: &str = "invalid-";
const NICKEL_EXTENSION: &str = "ncl";
const TARGET_TEST_DIR: &str = "target-test";
const SELF_TEST_PREFIX: &str = "lifecycle-config-self-test";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FixtureExpectation {
    Valid,
    Invalid,
}

fn repo_root() -> PathBuf {
    match Command::new("git").args(["rev-parse", "--show-toplevel"]).output() {
        Ok(output) if output.status.success() => PathBuf::from(String::from_utf8_lossy(&output.stdout).trim()),
        _ => std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    }
}

fn command_output(mut command: Command, label: &str) -> Result<Output, String> {
    command.output().map_err(|err| format!("failed to run {label}: {err}"))
}

fn require_success(output: Output, label: &str) -> Result<Output, String> {
    if output.status.success() {
        Ok(output)
    } else {
        Err(format!(
            "{label} failed with status {:?}\nstdout:\n{}\nstderr:\n{}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn nickel_yaml_export(root: &Path) -> Result<Vec<u8>, String> {
    let mut command = Command::new(NICKEL_BIN);
    command.arg(EXPORT_ARG).arg(FORMAT_ARG).arg(YAML_FORMAT).arg(root.join(CONFIG_SOURCE));
    let output =
        require_success(command_output(command, "nickel lifecycle yaml export")?, "nickel lifecycle yaml export")?;
    println!("PASS nickel export --format yaml {CONFIG_SOURCE}");
    Ok(output.stdout)
}

fn check_generated_configs(root: &Path, exported: &[u8]) -> Result<(), String> {
    for generated in GENERATED_CONFIGS {
        let path = root.join(generated);
        let checked_in = fs::read(&path).map_err(|err| format!("failed to read {}: {err}", path.display()))?;
        if checked_in != exported {
            return Err(format!(
                "{generated} is stale: run `nickel export --format yaml {CONFIG_SOURCE} > {generated}` and copy it to all compatibility paths"
            ));
        }
        println!("PASS lifecycle config fresh {generated}");
    }
    Ok(())
}

fn check_compatibility_configs(root: &Path) -> Result<(), String> {
    let (canonical, compatibility) =
        GENERATED_CONFIGS.split_first().ok_or("lifecycle config destinations are empty")?;
    let canonical_path = root.join(canonical);
    let canonical_bytes =
        fs::read(&canonical_path).map_err(|err| format!("failed to read {}: {err}", canonical_path.display()))?;
    for generated in compatibility {
        let path = root.join(generated);
        let bytes = fs::read(&path).map_err(|err| format!("failed to read {}: {err}", path.display()))?;
        if bytes != canonical_bytes {
            return Err(format!("lifecycle compatibility destination drift: {generated}"));
        }
        println!("PASS lifecycle compatibility destination {generated}");
    }
    Ok(())
}

fn fixture_expectation(path: &Path) -> Result<Option<FixtureExpectation>, String> {
    if path.extension().and_then(|value| value.to_str()) != Some(NICKEL_EXTENSION) {
        return Ok(None);
    }
    let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
        return Err(format!("non-utf8 fixture path `{}`", path.display()));
    };
    if name.starts_with(VALID_PREFIX) {
        Ok(Some(FixtureExpectation::Valid))
    } else if name.starts_with(INVALID_PREFIX) {
        Ok(Some(FixtureExpectation::Invalid))
    } else {
        Err(format!("fixture `{}` must start with `{VALID_PREFIX}` or `{INVALID_PREFIX}`", path.display()))
    }
}

fn nickel_fixture_output(path: &Path) -> Result<Output, String> {
    let mut command = Command::new(NICKEL_BIN);
    command.arg(EXPORT_ARG).arg(path);
    command_output(command, &format!("nickel export {}", path.display()))
}

fn check_fixture(path: &Path, expectation: FixtureExpectation) -> Result<(), String> {
    let output = nickel_fixture_output(path)?;
    match (expectation, output.status.success()) {
        (FixtureExpectation::Valid, true) => {
            println!("PASS valid lifecycle fixture {}", path.display());
            Ok(())
        }
        (FixtureExpectation::Invalid, false) => {
            println!("PASS invalid lifecycle fixture rejected {}", path.display());
            Ok(())
        }
        (FixtureExpectation::Valid, false) => Err(format!(
            "valid lifecycle fixture {} failed\nstdout:\n{}\nstderr:\n{}",
            path.display(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )),
        (FixtureExpectation::Invalid, true) => Err(format!(
            "invalid lifecycle fixture {} unexpectedly passed\nstdout:\n{}",
            path.display(),
            String::from_utf8_lossy(&output.stdout)
        )),
    }
}

fn check_fixture_dir(dir: &Path) -> Result<(), String> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(dir).map_err(|err| format!("failed to read {}: {err}", dir.display()))? {
        paths.push(entry.map_err(|err| err.to_string())?.path());
    }
    paths.sort();
    let mut valid_count = 0usize;
    let mut invalid_count = 0usize;
    for path in paths {
        let Some(expectation) = fixture_expectation(&path)? else {
            continue;
        };
        check_fixture(&path, expectation)?;
        match expectation {
            FixtureExpectation::Valid => valid_count += 1,
            FixtureExpectation::Invalid => invalid_count += 1,
        }
    }
    if valid_count == 0 || invalid_count == 0 {
        return Err(format!("{} must contain valid and invalid fixtures", dir.display()));
    }
    Ok(())
}

fn write_self_test_fixture(dir: &Path, name: &str, value: &str) -> Result<(), String> {
    fs::write(
        dir.join(name),
        format!(
            "let NonEmptyString = std.contract.from_predicate (fun value => std.is_string value && std.string.length value > 0) in\n{{ name = {value} }} | {{ name | NonEmptyString }}\n"
        ),
    )
    .map_err(|err| format!("failed to write self-test fixture {name}: {err}"))
}

fn run_self_test(root: &Path) -> Result<(), String> {
    let dir = root.join(TARGET_TEST_DIR).join(format!("{SELF_TEST_PREFIX}-{}", std::process::id()));
    if dir.exists() {
        fs::remove_dir_all(&dir).map_err(|err| format!("failed to reset {}: {err}", dir.display()))?;
    }
    fs::create_dir_all(&dir).map_err(|err| format!("failed to create {}: {err}", dir.display()))?;
    write_self_test_fixture(&dir, "valid-fixture.ncl", "\"ok\"")?;
    write_self_test_fixture(&dir, "invalid-fixture.ncl", "\"\"")?;
    let result = check_fixture_dir(&dir);
    fs::remove_dir_all(&dir).map_err(|err| format!("failed to remove {}: {err}", dir.display()))?;
    result?;
    println!("lifecycle config checker self-test: PASS");
    Ok(())
}

fn run_check(root: &Path, legacy_parity: bool) -> Result<(), String> {
    let exported = nickel_yaml_export(root)?;
    if legacy_parity {
        check_generated_configs(root, &exported)?;
    } else {
        check_compatibility_configs(root)?;
    }
    check_fixture_dir(&root.join(FIXTURE_DIR))?;
    if legacy_parity {
        println!("lifecycle config legacy parity: PASS");
    } else {
        println!("lifecycle config semantic check: PASS");
    }
    Ok(())
}

fn print_usage() {
    println!("usage: check-lifecycle-config.rs [--self-test|--legacy-parity]");
}

fn main() -> Result<(), String> {
    let root = repo_root();
    std::env::set_current_dir(&root).map_err(|err| format!("failed to cd {}: {err}", root.display()))?;
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [] => run_check(&root, false),
        [arg] if arg == LEGACY_PARITY_ARG => run_check(&root, true),
        [arg] if arg == SELF_TEST_ARG => run_self_test(&root),
        [arg] if arg == HELP_ARG || arg == HELP_SHORT_ARG => {
            print_usage();
            Ok(())
        }
        _ => Err("usage: check-lifecycle-config.rs [--self-test|--legacy-parity]".to_string()),
    }
}

#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang nixpkgs#nickel -c cargo -q -Zscript

//! Typecheck, export, and freshness-check the Nickel verification policy.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const POLICY_SOURCE: &str = "verification-policy/policy.ncl";
const GENERATED_POLICY: &str = "verification-policy/generated/policy.json";

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

fn nickel_typecheck(root: &Path) -> Result<(), String> {
    let mut command = Command::new("nickel");
    command.arg("typecheck").arg(root.join(POLICY_SOURCE));
    require_success(command_output(command, "nickel typecheck")?, "nickel typecheck")?;
    println!("PASS nickel typecheck {POLICY_SOURCE}");
    Ok(())
}

fn nickel_export(root: &Path) -> Result<Vec<u8>, String> {
    let mut command = Command::new("nickel");
    command.arg("export").arg(root.join(POLICY_SOURCE));
    let output = require_success(command_output(command, "nickel export")?, "nickel export")?;
    println!("PASS nickel export {POLICY_SOURCE}");
    Ok(output.stdout)
}

fn check_fresh(root: &Path, exported: &[u8]) -> Result<(), String> {
    let generated_path = root.join(GENERATED_POLICY);
    let checked_in = fs::read(&generated_path)
        .map_err(|err| format!("failed to read {}: {err}", generated_path.display()))?;
    if checked_in != exported {
        return Err(format!(
            "generated policy is stale: run `nickel export {POLICY_SOURCE} > {GENERATED_POLICY}`"
        ));
    }
    println!("PASS generated policy fresh {GENERATED_POLICY}");
    Ok(())
}

fn main() -> Result<(), String> {
    let root = repo_root();
    std::env::set_current_dir(&root).map_err(|err| format!("failed to cd {}: {err}", root.display()))?;
    nickel_typecheck(&root)?;
    let exported = nickel_export(&root)?;
    check_fresh(&root, &exported)?;
    println!("verification policy check: PASS");
    Ok(())
}

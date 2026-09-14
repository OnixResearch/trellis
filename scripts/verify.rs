#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Full verification pipeline. Exits non-zero on any failure.

use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::time::Duration;

const TRACEY_ATTEMPTS: usize = 4;
const TRACEY_RETRY_DELAY: Duration = Duration::from_secs(2);
const CARGO_ENV_VAR: &str = "TRELLIS_VERIFY_CARGO";
const RUSTC_ENV_VAR: &str = "RUSTC";
const RUSTDOC_ENV_VAR: &str = "RUSTDOC";
const CARGO_BIN_NAME: &str = "cargo";
const RUSTC_BIN_NAME: &str = "rustc";
const RUSTDOC_BIN_NAME: &str = "rustdoc";
const CARGO_VERSION_PREFIX: &str = "cargo ";
const EDITION_2024_MIN_CARGO_MAJOR: u32 = 1;
const EDITION_2024_MIN_CARGO_MINOR: u32 = 85;

struct Runner {
    repo_root: PathBuf,
    failed: bool,
}

impl Runner {
    fn new(repo_root: PathBuf) -> Self {
        Self {
            repo_root,
            failed: false,
        }
    }

    fn section(&self, title: &str) {
        println!("=== {title} ===");
    }

    fn blank(&self) {
        println!();
    }

    fn run_status<I, S, P>(&mut self, title: &str, program: P, args: I, pass: &str, fail: &str)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
        P: AsRef<OsStr>,
    {
        self.section(title);
        let status = Command::new(program)
            .args(args)
            .current_dir(&self.repo_root)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        match status {
            Ok(status) if status.success() => {
                if !pass.is_empty() {
                    println!("{pass}");
                }
            }
            Ok(_) | Err(_) => {
                if !fail.is_empty() {
                    println!("{fail}");
                }
                self.failed = true;
            }
        }
        self.blank();
    }

    fn run_tracey_status(&self) {
        self.section("Tracey coverage");
        let _ = Command::new("tracey")
            .args(["query", "status"])
            .current_dir(&self.repo_root)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();
        self.blank();
    }

    fn tracey_query_contains(&mut self, query: &str, expected: &str, pass: &str, fail: &str) {
        let mut last_output = String::new();

        for attempt in 1..=TRACEY_ATTEMPTS {
            let output = Command::new("tracey").args(["query", query]).current_dir(&self.repo_root).output();

            match output {
                Ok(output) => {
                    last_output.clear();
                    last_output.push_str(&String::from_utf8_lossy(&output.stdout));
                    last_output.push_str(&String::from_utf8_lossy(&output.stderr));
                    print!("{last_output}");
                    if last_output.contains(expected) {
                        println!("{pass}");
                        self.blank();
                        return;
                    }
                }
                Err(err) => {
                    last_output = format!("failed to run tracey query {query}: {err}\n");
                    print!("{last_output}");
                }
            }

            if attempt < TRACEY_ATTEMPTS {
                thread::sleep(TRACEY_RETRY_DELAY);
            }
        }

        println!("{fail}");
        self.failed = true;
        self.blank();
    }

    fn finish(self) -> std::process::ExitCode {
        if self.failed {
            println!("=== SOME CHECKS FAILED ===");
            std::process::ExitCode::from(1)
        } else {
            println!("=== ALL CHECKS PASSED ===");
            std::process::ExitCode::SUCCESS
        }
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct CargoVersion {
    major: u32,
    minor: u32,
}

fn parse_cargo_version(text: &str) -> Option<CargoVersion> {
    let version = text.strip_prefix(CARGO_VERSION_PREFIX)?;
    let mut parts = version.split(['.', '-', ' ']);
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    Some(CargoVersion { major, minor })
}

fn supports_edition_2024(version: CargoVersion) -> bool {
    version.major > EDITION_2024_MIN_CARGO_MAJOR
        || (version.major == EDITION_2024_MIN_CARGO_MAJOR && version.minor >= EDITION_2024_MIN_CARGO_MINOR)
}

fn cargo_version(path: &Path) -> Option<CargoVersion> {
    let output = Command::new(path).arg("--version").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_cargo_version(stdout.trim())
}

fn path_cargo_candidates() -> Vec<PathBuf> {
    match std::env::var_os("PATH") {
        Some(path) => std::env::split_paths(&path).map(|directory| directory.join(CARGO_BIN_NAME)).collect(),
        None => Vec::new(),
    }
}

fn selected_cargo() -> PathBuf {
    if let Some(path) = std::env::var_os(CARGO_ENV_VAR) {
        return PathBuf::from(path);
    }

    for candidate in path_cargo_candidates() {
        match cargo_version(&candidate) {
            Some(version) if supports_edition_2024(version) => return candidate,
            Some(_) | None => {}
        }
    }

    PathBuf::from(CARGO_BIN_NAME)
}

fn sibling_tool(cargo: &Path, name: &str) -> Option<PathBuf> {
    let tool = cargo.parent()?.join(name);
    if tool.exists() { Some(tool) } else { None }
}

fn repo_root() -> PathBuf {
    match Command::new("git").args(["rev-parse", "--show-toplevel"]).output() {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            PathBuf::from(stdout.trim())
        }
        _ => std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    }
}

fn main() -> std::process::ExitCode {
    let mut runner = Runner::new(repo_root());
    let cargo = selected_cargo();
    println!("Using Cargo: {}", cargo.display());
    let rustc = sibling_tool(&cargo, RUSTC_BIN_NAME);
    if let Some(rustc) = &rustc {
        println!("Using Rustc: {}", rustc.display());
        std::env::set_var(RUSTC_ENV_VAR, rustc);
    }
    let rustdoc = sibling_tool(&cargo, RUSTDOC_BIN_NAME);
    if let Some(rustdoc) = &rustdoc {
        println!("Using Rustdoc: {}", rustdoc.display());
        std::env::set_var(RUSTDOC_ENV_VAR, rustdoc);
    }
    runner.blank();

    runner.run_status(
        "Coverage: r[impl] on every pub fn",
        "./scripts/check-coverage.rs",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status(
        "Coverage: ensures on every verified pub fn",
        "./scripts/check-ensures.rs",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status(
        "Proof-gap inventory drift",
        "./scripts/check-proof-gap-inventory.rs",
        std::iter::empty::<&str>(),
        "✓ proof-gap inventory matches expected counts, sites, classifications, and rationale manifest",
        "✗ proof-gap inventory drift detected",
    );
    runner.run_status(
        "Aspen evidence index drift",
        "./scripts/check-aspen-evidence-index.rs",
        std::iter::empty::<&str>(),
        "✓ Aspen evidence index matches recorded source commits and blobs",
        "✗ Aspen evidence index drift detected",
    );
    runner.run_status(
        "Standalone Nickel export exact-byte cutover",
        "./scripts/check-nickel-export-cutover.rs",
        ["--check"],
        "",
        "",
    );
    runner.run_status(
        "Standalone Nickel export negative controls",
        "./scripts/check-nickel-export-cutover.rs",
        ["--self-test"],
        "",
        "",
    );
    runner.run_status(
        "Nickel policy semantics and fixtures",
        "./scripts/check-verification-policy.rs",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status(
        "Nickel policy checker self-test",
        "./scripts/check-verification-policy.rs",
        ["--self-test"],
        "",
        "",
    );
    runner.run_status(
        "Upstream Verus corpus catalog",
        "./scripts/check-upstream-verus-corpus.rs",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status(
        "Upstream Verus corpus checker self-test",
        "./scripts/check-upstream-verus-corpus.rs",
        ["--self-test"],
        "",
        "",
    );
    runner.run_status(
        "Nickel lifecycle config freshness and fixtures",
        "./scripts/check-lifecycle-config.rs",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status(
        "Nickel lifecycle config checker self-test",
        "./scripts/check-lifecycle-config.rs",
        ["--self-test"],
        "",
        "",
    );
    runner.run_status(
        "Crate metadata discoverability",
        "./scripts/check-crate-metadata.rs",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status("Crate metadata checker self-test", "./scripts/check-crate-metadata.rs", ["--self-test"], "", "");
    runner.run_status(
        "README module catalog drift",
        "./scripts/check-readme-module-catalog.rs",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status(
        "README module catalog checker fixtures",
        "./scripts/check-readme-module-catalog.rs",
        ["--self-test"],
        "",
        "",
    );
    runner.run_status("Module guide link drift", "./scripts/check-module-guide.rs", std::iter::empty::<&str>(), "", "");
    runner.run_status("Module guide checker self-test", "./scripts/check-module-guide.rs", ["--self-test"], "", "");
    runner.run_status(
        "Module maturity index drift",
        "./scripts/check-module-index.rs",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status(
        "Module maturity index checker self-test",
        "./scripts/check-module-index.rs",
        ["--self-test"],
        "",
        "",
    );
    runner.run_status(
        "Invariant surface catalog drift",
        "./scripts/check-invariant-surface-catalog.rs",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status(
        "Invariant surface catalog checker self-test",
        "./scripts/check-invariant-surface-catalog.rs",
        ["--self-test"],
        "",
        "",
    );
    runner.run_status("Cookbook recipe metadata", "./scripts/check-cookbook.rs", std::iter::empty::<&str>(), "", "");
    runner.run_status("Cookbook checker self-test", "./scripts/check-cookbook.rs", ["--self-test"], "", "");
    runner.run_status("Runnable examples docs", "./scripts/check-examples.rs", std::iter::empty::<&str>(), "", "");
    runner.run_status("Runnable examples checker self-test", "./scripts/check-examples.rs", ["--self-test"], "", "");
    runner.run_status(
        "Runnable proof examples",
        "./scripts/check-proof-examples.sh",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status(
        "JSON artifact contract admission",
        "./scripts/check-json-artifact-contracts.rs",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status(
        "JSON artifact contract checker self-test",
        "./scripts/check-json-artifact-contracts.rs",
        ["--self-test"],
        "",
        "",
    );
    runner.run_status(
        "Aeneas cross-verifier artifact validation",
        "./scripts/check-aeneas-cross-verifier.rs",
        ["--check"],
        "",
        "",
    );
    runner.run_status(
        "Aeneas cross-verifier negative fixtures",
        "./scripts/check-aeneas-cross-verifier.rs",
        ["--self-test"],
        "",
        "",
    );
    runner.run_status(
        "Aeneas coverage frontier validation",
        "./scripts/check-aeneas-frontier.rs",
        ["--check"],
        "",
        "",
    );
    runner.run_status(
        "Aeneas coverage frontier negative fixtures",
        "./scripts/check-aeneas-frontier.rs",
        ["--self-test"],
        "",
        "",
    );
    runner.run_status(
        "Proof evidence bundle validation",
        &cargo,
        ["run", "--quiet", "--bin", "proof_evidence", "--", "check"],
        "",
        "",
    );
    runner.run_status(
        "Proof evidence bundle self-test",
        &cargo,
        ["run", "--quiet", "--bin", "proof_evidence", "--", "self-test"],
        "",
        "",
    );
    runner.run_status(
        "Proof evidence IR chain validation",
        &cargo,
        ["run", "--quiet", "--bin", "proof_evidence_ir", "--", "check"],
        "",
        "",
    );
    runner.run_status(
        "Proof evidence IR chain self-test",
        &cargo,
        ["run", "--quiet", "--bin", "proof_evidence_ir", "--", "self-test"],
        "",
        "",
    );
    runner.run_status(
        "Runtime-cost evidence metadata",
        "./scripts/check-runtime-evidence.rs",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status(
        "Runtime-cost evidence checker self-test",
        "./scripts/check-runtime-evidence.rs",
        ["--self-test"],
        "",
        "",
    );
    runner.run_status(
        "Verified primitive manifest freshness",
        "./scripts/check-verified-primitive-manifest.rs",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status(
        "Verified primitive manifest checker self-test",
        "./scripts/check-verified-primitive-manifest.rs",
        ["--self-test"],
        "",
        "",
    );
    runner.run_status(
        "UCAN bridge coverage",
        "./scripts/check-ucan-bridge-coverage.rs",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status(
        "UCAN bridge coverage checker self-test",
        "./scripts/check-ucan-bridge-coverage.rs",
        ["--self-test"],
        "",
        "",
    );
    runner.run_status(
        "Verified-logic consumer release profile",
        "./scripts/check-verified-logic-consumer-profile.rs",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status(
        "Verified-logic consumer release profile self-test",
        "./scripts/check-verified-logic-consumer-profile.rs",
        ["--self-test"],
        "",
        "",
    );
    runner.run_status(
        "README headline metrics drift",
        "./scripts/check-readme-metrics.rs",
        std::iter::empty::<&str>(),
        "",
        "",
    );
    runner.run_status(
        "README headline metrics checker self-test",
        "./scripts/check-readme-metrics.rs",
        ["--self-test"],
        "",
        "",
    );
    runner.run_status(
        "Clippy",
        &cargo,
        ["clippy", "--all-targets", "--", "-D", "warnings"],
        "✓ clippy clean",
        "✗ clippy failed",
    );
    runner.run_status("Tests", &cargo, ["test"], "✓ tests pass", "✗ tests failed");
    runner.run_status("Examples", &cargo, ["check", "--examples"], "✓ examples compile", "✗ examples failed");
    runner.run_status(
        "Verus verification",
        "./scripts/verus.sh",
        ["src/lib.rs"],
        "✓ verus proofs pass",
        "✗ verus verification failed",
    );

    runner.run_tracey_status();
    runner.tracey_query_contains(
        "uncovered",
        "0 uncovered",
        "✓ no uncovered requirements",
        "✗ uncovered requirements found",
    );
    runner.tracey_query_contains(
        "untested",
        "0 untested",
        "✓ no untested requirements",
        "✗ untested requirements found",
    );

    runner.finish()
}

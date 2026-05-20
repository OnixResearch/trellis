#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Full verification pipeline. Exits non-zero on any failure.

use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

const TRACEY_ATTEMPTS: usize = 4;
const TRACEY_RETRY_DELAY: Duration = Duration::from_secs(2);

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

    fn run_status<I, S>(&mut self, title: &str, program: &str, args: I, pass: &str, fail: &str)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
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
        "✓ proof-gap inventory matches expected counts, sites, and classifications",
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
        "nix",
        [
            "develop",
            "-c",
            "cargo",
            "clippy",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
        "✓ clippy clean",
        "✗ clippy failed",
    );
    runner.run_status("Tests", "nix", ["develop", "-c", "cargo", "test"], "✓ tests pass", "✗ tests failed");
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

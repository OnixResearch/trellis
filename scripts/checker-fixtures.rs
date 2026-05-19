#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Run deterministic fixture regressions for repository checker scripts.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy)]
struct Case {
    name: &'static str,
    checker: &'static str,
    args: &'static [&'static str],
    expect_success: bool,
    expect_substring: &'static str,
}

fn repo_root() -> PathBuf {
    match Command::new("git").args(["rev-parse", "--show-toplevel"]).output() {
        Ok(output) if output.status.success() => PathBuf::from(String::from_utf8_lossy(&output.stdout).trim()),
        _ => std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    }
}

fn temp_dir(_root: &Path) -> Result<PathBuf, String> {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| format!("system clock before unix epoch: {err}"))?
        .as_nanos();
    let base = std::env::var_os("CHECKER_FIXTURES_TMPDIR")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("TMPDIR").map(PathBuf::from))
        .filter(|path| path.is_dir())
        .unwrap_or_else(std::env::temp_dir);
    let dir = base.join("checker-fixtures").join(format!("run-{stamp}"));
    fs::create_dir_all(&dir).map_err(|err| format!("failed to create {}: {err}", dir.display()))?;
    Ok(dir)
}

fn compile_checker(root: &Path, tmp: &Path, script: &str) -> Result<PathBuf, String> {
    let source = root.join(script);
    let output = tmp.join(script.replace('/', "-").trim_end_matches(".rs").to_string());
    let compile = Command::new("rustc")
        .arg(&source)
        .arg("-o")
        .arg(&output)
        .output()
        .map_err(|err| format!("failed to run rustc for {script}: {err}"))?;
    if !compile.status.success() {
        return Err(format!(
            "failed to compile {script}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&compile.stdout),
            String::from_utf8_lossy(&compile.stderr)
        ));
    }
    Ok(output)
}

fn run_case(root: &Path, bin: &Path, case: Case) -> Result<(), String> {
    let mut command = Command::new(bin);
    for arg in case.args {
        if arg.starts_with('-') {
            command.arg(arg);
        } else {
            command.arg(root.join(arg));
        }
    }
    let Output { status, stdout, stderr } = command
        .output()
        .map_err(|err| format!("{}: failed to run checker: {err}", case.name))?;
    let success = status.success();
    let combined = format!("{}{}", String::from_utf8_lossy(&stdout), String::from_utf8_lossy(&stderr));
    if success != case.expect_success {
        return Err(format!(
            "{}: expected success={} but got status {:?}\n{}",
            case.name,
            case.expect_success,
            status.code(),
            combined
        ));
    }
    if !combined.contains(case.expect_substring) {
        return Err(format!(
            "{}: expected diagnostic substring {:?}\n{}",
            case.name, case.expect_substring, combined
        ));
    }
    println!("PASS {}", case.name);
    Ok(())
}

fn main() -> Result<(), String> {
    let root = repo_root();
    std::env::set_current_dir(&root).map_err(|err| format!("failed to cd {}: {err}", root.display()))?;
    let tmp = temp_dir(&root)?;

    let coverage = compile_checker(&root, &tmp, "scripts/check-coverage.rs")?;
    let ensures = compile_checker(&root, &tmp, "scripts/check-ensures.rs")?;
    let iterative = compile_checker(&root, &tmp, "scripts/check-iterative-exec.rs")?;

    let cases = [
        (coverage.as_path(), Case {
            name: "coverage-valid",
            checker: "coverage",
            args: &["--root", "tests/fixtures/checkers/coverage-valid"],
            expect_success: true,
            expect_substring: "all pub fns have r[impl] annotations",
        }),
        (coverage.as_path(), Case {
            name: "coverage-missing-impl",
            checker: "coverage",
            args: &["--root", "tests/fixtures/checkers/coverage-invalid"],
            expect_success: false,
            expect_substring: "MISSING r[impl]",
        }),
        (ensures.as_path(), Case {
            name: "ensures-valid",
            checker: "ensures",
            args: &["--root", "tests/fixtures/checkers/ensures-valid"],
            expect_success: true,
            expect_substring: "all verified pub fns have ensures clauses",
        }),
        (ensures.as_path(), Case {
            name: "ensures-missing-clause",
            checker: "ensures",
            args: &["--root", "tests/fixtures/checkers/ensures-invalid"],
            expect_success: false,
            expect_substring: "MISSING ensures",
        }),
        (iterative.as_path(), Case {
            name: "iterative-valid",
            checker: "iterative",
            args: &["tests/fixtures/checkers/iterative-valid.rs"],
            expect_success: true,
            expect_substring: "result: PASS",
        }),
        (iterative.as_path(), Case {
            name: "iterative-self-cycle",
            checker: "iterative",
            args: &["tests/fixtures/checkers/iterative-self-cycle.rs"],
            expect_success: false,
            expect_substring: "cycles:",
        }),
        (iterative.as_path(), Case {
            name: "iterative-mutual-cycle",
            checker: "iterative",
            args: &["tests/fixtures/checkers/iterative-mutual-cycle.rs"],
            expect_success: false,
            expect_substring: "cycles:",
        }),
        (iterative.as_path(), Case {
            name: "iterative-qualified-local-call",
            checker: "iterative",
            args: &["tests/fixtures/checkers/iterative-qualified.rs"],
            expect_success: false,
            expect_substring: "qualified local-call violations",
        }),
    ];

    for (bin, case) in cases {
        let _checker_name = case.checker;
        run_case(&root, bin, case)?;
    }

    println!("checker fixture self-tests: PASS");
    Ok(())
}

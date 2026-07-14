#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Deterministic README headline metrics freshness checker.
//!
//! The fast/default check derives cheap source-backed counts and parses the
//! headline so stale README metrics fail before review. Use `--full` to also
//! refresh command-backed counts for tests, Verus proofs, and Tracey rules.

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitCode;

const README_PATH: &str = "README.md";
const LIB_PATH: &str = "src/lib.rs";
const PROOF_GAP_PATH: &str = "docs/proof-gap-sites.tsv";
const HEADLINE_PREFIX: &str = "**";
const HEADLINE_SUFFIX: &str = " trusted/proof-gap boundaries**";
const VERUS_RESOURCE_LIMIT: &str = "20";
const VERUS_TRIGGERS_MODE: &str = "silent";

#[derive(Clone, Debug, PartialEq, Eq)]
struct Metrics {
    verus_proofs: Option<u64>,
    tests: Option<u64>,
    requirements: Option<u64>,
    modules: u64,
    trusted_boundaries: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Headline {
    metrics: Metrics,
    line_number: usize,
}

fn repo_root() -> PathBuf {
    if let Ok(output) = Command::new("git").args(["rev-parse", "--show-toplevel"]).output() {
        if output.status.success() {
            return PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn parse_number_prefix(part: &str, suffix: &str) -> Result<u64, String> {
    let Some(number) = part.trim().strip_suffix(suffix) else {
        return Err(format!("expected headline segment ending in `{suffix}`, got `{part}`"));
    };
    number
        .trim()
        .parse::<u64>()
        .map_err(|err| format!("invalid number in headline segment `{part}`: {err}"))
}

fn parse_headline(readme: &str) -> Result<Headline, String> {
    for (index, line) in readme.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with(HEADLINE_PREFIX)
            && trimmed.ends_with(HEADLINE_SUFFIX)
            && trimmed.contains(" Verus proofs · ")
        {
            let Some(body) = trimmed.strip_prefix(HEADLINE_PREFIX).and_then(|s| s.strip_suffix("**")) else {
                return Err(format!("malformed README metrics headline at line {}", index + 1));
            };
            let parts: Vec<&str> = body.split(" · ").collect();
            if parts.len() != 5 {
                return Err(format!("expected 5 README metrics at line {}, got {}", index + 1, parts.len()));
            }
            return Ok(Headline {
                metrics: Metrics {
                    verus_proofs: Some(parse_number_prefix(parts[0], " Verus proofs")?),
                    tests: Some(parse_number_prefix(parts[1], " tests")?),
                    requirements: Some(parse_number_prefix(parts[2], " requirements")?),
                    modules: parse_number_prefix(parts[3], " modules")?,
                    trusted_boundaries: parse_number_prefix(parts[4], " trusted/proof-gap boundaries")?,
                },
                line_number: index + 1,
            });
        }
    }
    Err("README metrics headline not found".to_string())
}

fn count_modules(lib_rs: &str) -> u64 {
    lib_rs
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            trimmed.starts_with("pub mod ") && trimmed.ends_with(';')
        })
        .count() as u64
}

fn count_proof_gap_rows(tsv: &str) -> u64 {
    tsv.lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with('#')
        })
        .count() as u64
}

fn command_output(root: &Path, program: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .current_dir(root)
        .env("TMPDIR", std::env::var("TMPDIR").unwrap_or_else(|_| "/tmp".to_string()))
        .output()
        .map_err(|err| format!("failed to run `{program}`: {err}"))?;
    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    if output.status.success() {
        Ok(combined)
    } else {
        Err(format!("`{program} {}` failed:\n{combined}", args.join(" ")))
    }
}

fn first_number_before(line: &str, suffix: &str) -> Option<u64> {
    line.trim().strip_suffix(suffix).and_then(|n| n.trim().parse::<u64>().ok())
}

fn count_cargo_tests(root: &Path) -> Result<u64, String> {
    let output = command_output(root, "nix", &["develop", "-c", "cargo", "test", "--", "--list"])?;
    for line in output.lines() {
        if let Some(count) = first_number_before(line, "tests, 0 benchmarks") {
            return Ok(count);
        }
    }
    Err("could not parse `cargo test -- --list` test count".to_string())
}

fn parse_verus_proof_count(output: &str) -> Result<u64, String> {
    for line in output.lines() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("verification results:: ") else {
            continue;
        };
        let Some((count, tail)) = rest.split_once(" verified") else {
            continue;
        };
        if tail.trim() == ", 0 errors" {
            return count.trim().parse::<u64>().map_err(|err| format!("invalid Verus proof count `{count}`: {err}"));
        }
    }
    Err("could not parse successful Verus proof count".to_string())
}

fn count_verus_proofs(root: &Path) -> Result<u64, String> {
    let output = command_output(root, "./scripts/verus.sh", &[
        "src/lib.rs",
        "--triggers-mode",
        VERUS_TRIGGERS_MODE,
        "--rlimit",
        VERUS_RESOURCE_LIMIT,
    ])?;
    parse_verus_proof_count(&output)
}

fn count_tracey_requirements(root: &Path) -> Result<u64, String> {
    let _ = command_output(root, "tracey", &["query", "status"]);
    let output = command_output(root, "tracey", &["query", "uncovered"])?;
    for line in output.lines() {
        let Some((_, rest)) = line.split_once(" out of ") else {
            continue;
        };
        let Some((count, tail)) = rest.split_once(" rules") else {
            continue;
        };
        if tail.is_empty() || tail.trim().is_empty() {
            return count.trim().parse::<u64>().map_err(|err| format!("invalid Tracey rule count `{count}`: {err}"));
        }
    }
    Err("could not parse Tracey requirement rule count".to_string())
}

fn expected_metrics(root: &Path, full: bool) -> Result<Metrics, String> {
    let lib_rs = fs::read_to_string(root.join(LIB_PATH)).map_err(|err| format!("failed to read {LIB_PATH}: {err}"))?;
    let gaps = fs::read_to_string(root.join(PROOF_GAP_PATH))
        .map_err(|err| format!("failed to read {PROOF_GAP_PATH}: {err}"))?;
    Ok(Metrics {
        verus_proofs: if full { Some(count_verus_proofs(root)?) } else { None },
        tests: if full { Some(count_cargo_tests(root)?) } else { None },
        requirements: if full {
            Some(count_tracey_requirements(root)?)
        } else {
            None
        },
        modules: count_modules(&lib_rs),
        trusted_boundaries: count_proof_gap_rows(&gaps),
    })
}

fn check_metric(name: &str, headline: Option<u64>, expected: Option<u64>, failures: &mut Vec<String>) {
    if let Some(expected) = expected {
        if headline != Some(expected) {
            failures.push(format!("{name}: README has {:?}, expected {expected}", headline));
        }
    }
}

fn run_check(root: &Path, full: bool) -> Result<(), String> {
    let readme =
        fs::read_to_string(root.join(README_PATH)).map_err(|err| format!("failed to read {README_PATH}: {err}"))?;
    let headline = parse_headline(&readme)?;
    let expected = expected_metrics(root, full)?;
    let mut failures = Vec::new();

    check_metric("Verus proofs", headline.metrics.verus_proofs, expected.verus_proofs, &mut failures);
    check_metric("tests", headline.metrics.tests, expected.tests, &mut failures);
    check_metric("requirements", headline.metrics.requirements, expected.requirements, &mut failures);
    if headline.metrics.modules != expected.modules {
        failures.push(format!("modules: README has {}, expected {}", headline.metrics.modules, expected.modules));
    }
    if headline.metrics.trusted_boundaries != expected.trusted_boundaries {
        failures.push(format!(
            "trusted/proof-gap boundaries: README has {}, expected {}",
            headline.metrics.trusted_boundaries, expected.trusted_boundaries
        ));
    }

    if failures.is_empty() {
        if full {
            println!(
                "✓ README metrics headline is fresh at line {} ({} Verus proofs, {} tests, {} requirements, {} modules, {} trusted/proof-gap boundaries)",
                headline.line_number,
                expected.verus_proofs.unwrap_or_default(),
                expected.tests.unwrap_or_default(),
                expected.requirements.unwrap_or_default(),
                expected.modules,
                expected.trusted_boundaries
            );
        } else {
            println!(
                "✓ README metrics headline static counts are fresh at line {} ({} modules, {} trusted/proof-gap boundaries); use --full for command-backed proof/test/requirement counts",
                headline.line_number, expected.modules, expected.trusted_boundaries
            );
        }
        Ok(())
    } else {
        eprintln!("README metrics headline drift detected:");
        for failure in failures {
            eprintln!("  - {failure}");
        }
        Err("README metrics headline drift detected".to_string())
    }
}

fn run_self_test() -> Result<(), String> {
    let sample = "# X\n\n**10289 Verus proofs · 4617 tests · 1919 requirements · 581 modules · 16 trusted/proof-gap boundaries**\n";
    let parsed = parse_headline(sample)?;
    assert_eq!(parsed.line_number, 3);
    assert_eq!(parsed.metrics.verus_proofs, Some(10289));
    assert_eq!(parsed.metrics.tests, Some(4617));
    assert_eq!(parsed.metrics.requirements, Some(1919));
    assert_eq!(parsed.metrics.modules, 581);
    assert_eq!(parsed.metrics.trusted_boundaries, 16);
    assert_eq!(count_modules("pub mod alpha;\nmod hidden;\npub mod beta;\n"), 2);
    assert_eq!(count_proof_gap_rows("# header\n\nsrc/a.rs\tf\tpermanent\n"), 1);
    assert_eq!(parse_verus_proof_count("verification results:: 1 verified, 0 errors\n"), Ok(1));
    assert!(parse_verus_proof_count("verification results:: 1 verified, 1 errors\n").is_err());
    assert!(parse_verus_proof_count("verification results:: 1 verified, 10 errors\n").is_err());
    assert!(parse_verus_proof_count("verification output missing summary\n").is_err());
    assert!(parse_headline("# missing\n").is_err());
    println!("✓ README metrics self-test passed");
    Ok(())
}

fn print_usage() {
    eprintln!("usage: check-readme-metrics.rs [--full|--self-test]");
    eprintln!("  default: check static source-derived headline counts");
    eprintln!("  --full: also run cargo test -- --list, Verus, and Tracey to refresh command-backed counts");
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.as_slice() {
        [] => run_check(&repo_root(), false),
        [flag] if flag == "--full" => run_check(&repo_root(), true),
        [flag] if flag == "--self-test" => run_self_test(),
        _ => {
            print_usage();
            Err("invalid arguments".to_string())
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(1)
        }
    }
}

#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" -c cargo -q -Zscript

//! Run Tracey coverage queries with one retry for the documented transient daemon cancellation.
//!
//! This wrapper is intentionally narrow: it retries only outputs that look like the
//! local Tracey daemon auto-start race (`Error: Cancelled`) and fails closed for all
//! other non-zero coverage or tool failures.

use std::process::Command;
use std::process::Output;

const TRACEY_BIN: &str = "tracey";
const QUERY_ARG: &str = "query";
const UNCOVERED_QUERY: &str = "uncovered";
const UNTESTED_QUERY: &str = "untested";
const TRANSIENT_CANCELLED_MARKER: &str = "Error: Cancelled";
const TRANSIENT_RETRY_LIMIT: usize = 1;
const FIRST_ATTEMPT: usize = 1;
const SELF_TEST_CANCELLED_OUTPUT: &str = "warning: daemon auto-start\nError: Cancelled\n";
const SELF_TEST_COVERAGE_FAILURE_OUTPUT: &str = "1 uncovered out of 1542 rules\n";

fn combined_output(output: &Output) -> String {
    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

fn is_transient_tracey_cancelled(output: &str) -> bool {
    output.contains(TRANSIENT_CANCELLED_MARKER)
}

fn should_retry_tracey(output: &str, attempt: usize) -> bool {
    attempt <= TRANSIENT_RETRY_LIMIT && is_transient_tracey_cancelled(output)
}

fn run_tracey_query(query: &str, attempt: usize) -> Result<Output, String> {
    println!("$ {TRACEY_BIN} {QUERY_ARG} {query} # attempt {attempt}");
    Command::new(TRACEY_BIN)
        .arg(QUERY_ARG)
        .arg(query)
        .output()
        .map_err(|err| format!("failed to run {TRACEY_BIN} {QUERY_ARG} {query}: {err}"))
}

fn print_output(output: &Output) {
    print!("{}", String::from_utf8_lossy(&output.stdout));
    print!("{}", String::from_utf8_lossy(&output.stderr));
}

fn run_query_with_retry(query: &str) -> Result<(), String> {
    let mut attempt = FIRST_ATTEMPT;
    loop {
        let output = run_tracey_query(query, attempt)?;
        print_output(&output);
        let combined = combined_output(&output);
        if should_retry_tracey(&combined, attempt) {
            println!("retrying transient Tracey cancellation for `{query}`");
            attempt += FIRST_ATTEMPT;
            continue;
        }
        if is_transient_tracey_cancelled(&combined) {
            return Err(format!("{TRACEY_BIN} {QUERY_ARG} {query} hit transient cancellation after retry budget"));
        }
        if output.status.success() {
            return Ok(());
        }
        return Err(format!("{TRACEY_BIN} {QUERY_ARG} {query} failed"));
    }
}

fn run_coverage() -> Result<(), String> {
    run_query_with_retry(UNCOVERED_QUERY)?;
    run_query_with_retry(UNTESTED_QUERY)?;
    println!("tracey coverage PASS");
    Ok(())
}

fn require(condition: bool, message: &str) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message.to_string()) }
}

fn run_self_test() -> Result<(), String> {
    require(
        should_retry_tracey(SELF_TEST_CANCELLED_OUTPUT, FIRST_ATTEMPT),
        "transient cancellation should retry on first attempt",
    )?;
    require(
        !should_retry_tracey(SELF_TEST_CANCELLED_OUTPUT, FIRST_ATTEMPT + TRANSIENT_RETRY_LIMIT),
        "transient cancellation should not retry after retry budget",
    )?;
    require(
        is_transient_tracey_cancelled(SELF_TEST_CANCELLED_OUTPUT),
        "transient cancellation marker should be detected even if a tool exits successfully",
    )?;
    require(
        !should_retry_tracey(SELF_TEST_COVERAGE_FAILURE_OUTPUT, FIRST_ATTEMPT),
        "ordinary coverage failure should not retry",
    )?;
    println!("proof-review-tracey-coverage self-test: PASS");
    Ok(())
}

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(FIRST_ATTEMPT);
    match args.next().as_deref() {
        Some("--self-test") => run_self_test(),
        Some("--help") | Some("-h") => {
            println!("usage: proof-review-tracey-coverage.rs [--self-test]");
            Ok(())
        }
        Some(arg) => Err(format!("unknown argument `{arg}`")),
        None => run_coverage(),
    }
}

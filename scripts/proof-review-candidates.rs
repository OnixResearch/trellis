#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang nixpkgs#nickel -c cargo -q -Zscript

//! Rank proof-review candidates and generate focused VibeThinker packets.
//!
//! This checker is deterministic. VibeThinker remains advisory: use the packet
//! output as focused context, then validate accepted changes with real Trellis
//! checks.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

const POLICY_SOURCE: &str = "proof-review/policy.ncl";
const GENERATED_POLICY: &str = "proof-review/generated/policy.json";
const DEFAULT_REPORT: &str = "target/proof-review/candidates.md";
const DEFAULT_PACKET_DIR: &str = "target/proof-review/packets";
const DEFAULT_MAX_CANDIDATES: usize = 12;
const DEFAULT_MAX_PACKET_FINDINGS: usize = 6;
const CONTEXT_RADIUS_LINES: usize = 5;
const SOURCE_OVERVIEW_LINES: usize = 140;
const TEST_OVERVIEW_LINES: usize = 120;
const SELF_TEST_UNIQUE_FALLBACK: u128 = 0;
const MAX_REASON_SUMMARY_CHARS: usize = 96;
const STATUS_PREFIX_BYTES: usize = 3;
const RUST_EXTENSION: &str = "rs";
const SRC_DIR: &str = "src";
const CAIRN_CHANGES_DIR: &str = "cairn/changes";
const LEGACY_OPENSPEC_CHANGES_DIR: &str = "openspec/changes";
const ARCHIVE_DIR_NAME: &str = "archive";
const TASKS_FILE_NAME: &str = "tasks.md";
const EVIDENCE_DIR_NAME: &str = "evidence";
const MARKDOWN_EXTENSION: &str = "md";
const SOURCE_PATH_PREFIX: &str = "src/";
const SOURCE_PATH_SUFFIX: &str = ".rs";
const VIBETHINKER_MAX_TOKENS_HINT: usize = 16_384;
const VIBETHINKER_OUTPUT_CONTRACT_LINE_COUNT: usize = 13;
const VIBETHINKER_BASE_TASK: &str = "Critique this Trellis/Verus proof target without assuming the target or current patch is correct. Return concrete proof/spec/test gaps only; do not suggest adding or hiding trust boundaries.";
const VIBETHINKER_OUTPUT_CONTRACT: [&str; VIBETHINKER_OUTPUT_CONTRACT_LINE_COUNT] = [
    "Concrete proof postconditions or lemma splits, using existing specs only.",
    "Group materially different proposals by proof mechanism and collapse paraphrases of the same route.",
    "For every proposed helper lemma or reduction, classify its gap as simpler, equivalent, stronger, or unknown relative to the target; equivalent or stronger gaps remain blocked, not proof progress.",
    "For rejected sublemmas, give a concrete counterexample or failed proof obligation when feasible.",
    "Audit precondition satisfiability, vacuity, quantifier domains, arithmetic and index bounds, decreases, exec/spec correspondence, and trust-boundary expansion.",
    "Executable positive or negative test cases through public/runtime APIs, not tests for proof functions.",
    "Do not propose tests that rely on violating `requires` clauses, panics, or impossible allocations.",
    "Keep boundary suggestions small enough for ordinary unit/property tests; mark infeasible stress cases as non-actionable.",
    "Separate accepted actionable items from rejected/non-actionable advisory items.",
    "Spec wording gaps or overclaims, if any.",
    "Patch-critique risks if this packet already includes changed source.",
    "Do not repeat scanner findings unless adding a new actionable detail.",
    "If no issue remains, say `no focused concerns`.",
];

const SCORE_ACTIVE_CHANGE: usize = 600;
const SCORE_ACTIVE_UNCHECKED_TASK: usize = 25;
const SCORE_WORKTREE_CHANGED: usize = 250;
const MAX_SCORE_FINDINGS_PER_RULE: usize = 6;
const SCORE_UNTRUSTED_BOUNDARY: usize = 100;
const SCORE_PERMANENT_BOUNDARY: usize = 10;
const SCORE_ASSUME_MARKER: usize = 100;
const SCORE_VACUOUS_ENSURES_TRUE: usize = 80;
const SCORE_EMPTY_NONTRIVIAL_PROOF: usize = 35;
const SCORE_ACTIVE_PREFIX: usize = 25;
const SCORE_MISSING_TEST_HINTS: usize = 20;
const SCORE_NO_TEST_MODULE: usize = 20;
const SCORE_CATALOGUE_PROOF: usize = 20;

const TRUST_MARKERS: [&str; 4] = [
    "#[verifier::external_body]",
    "assume_specification[",
    "assume(",
    "@verified-logic:no-ensures",
];
const PERMANENT_MARKERS: [&str; 2] = ["@verified-logic:permanent-axiom", "@verified-logic:permanent-z3-limit"];
const ACTIVE_PREFIX_KEYWORDS: [&str; 10] = [
    "table_len",
    "active",
    "prefix",
    "from:",
    "fuel",
    ".len()",
    "len ==",
    "len <",
    "len <=",
    "MAX_",
];
const NEGATIVE_TEST_KEYWORDS: [&str; 8] = [
    "reject",
    "invalid",
    "malformed",
    "zero",
    "inactive",
    "out_of_range",
    "overlarge",
    "mismatch",
];
const NONTRIVIAL_POSTCONDITION_TOKENS: [&str; 12] = [
    "==>",
    "forall",
    "exists",
    "!",
    "&&",
    "||",
    "==",
    "<",
    ">",
    "satisfied",
    "valid",
    "member",
];

#[derive(Debug, Clone)]
struct Args {
    root: PathBuf,
    max_candidates: usize,
    write_report: PathBuf,
    write_packets: Option<PathBuf>,
    check_policy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Finding {
    rule_id: String,
    score: usize,
    line: Option<usize>,
    title: String,
    detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Candidate {
    path: String,
    score: usize,
    findings: Vec<Finding>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ActiveChange {
    name: String,
    unchecked_tasks: usize,
    referenced_paths: BTreeSet<String>,
    task_excerpt: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceFile {
    display_path: String,
    text: String,
}

#[derive(Debug, Clone)]
struct ReviewInputs {
    sources: Vec<SourceFile>,
    active_changes: Vec<ActiveChange>,
    changed_paths: BTreeSet<String>,
    legacy_openspec_changes_present: bool,
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

fn print_usage() {
    println!(
        "usage: proof-review-candidates.rs [--root PATH] [--max N] [--write-report PATH] [--write-packets DIR] [--check-policy] [--self-test]\n\ndefault report: {DEFAULT_REPORT}\ndefault packet dir: {DEFAULT_PACKET_DIR}"
    );
}

fn parse_usize(input: &str, label: &str) -> Result<usize, String> {
    input.parse::<usize>().map_err(|err| format!("invalid {label} `{input}`: {err}"))
}

fn parse_args() -> Result<Option<Args>, String> {
    let mut root = repo_root();
    let mut max_candidates = DEFAULT_MAX_CANDIDATES;
    let mut write_report = PathBuf::from(DEFAULT_REPORT);
    let mut write_packets = None;
    let mut check_policy = false;

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut index = 0usize;
    while index < args.len() {
        match args[index].as_str() {
            "--help" | "-h" => {
                print_usage();
                return Ok(None);
            }
            "--self-test" => {
                run_self_test()?;
                return Ok(None);
            }
            "--check-policy" => {
                check_policy = true;
                index += 1;
            }
            "--root" => {
                let value_index = index + 1;
                let Some(value) = args.get(value_index) else {
                    return Err("--root requires a path".to_string());
                };
                root = PathBuf::from(value);
                index += 2;
            }
            "--max" => {
                let value_index = index + 1;
                let Some(value) = args.get(value_index) else {
                    return Err("--max requires a number".to_string());
                };
                max_candidates = parse_usize(value, "--max")?;
                index += 2;
            }
            "--write-report" => {
                let value_index = index + 1;
                let Some(value) = args.get(value_index) else {
                    return Err("--write-report requires a path".to_string());
                };
                write_report = PathBuf::from(value);
                index += 2;
            }
            "--write-packets" => {
                let value_index = index + 1;
                let Some(value) = args.get(value_index) else {
                    return Err("--write-packets requires a directory".to_string());
                };
                write_packets = Some(PathBuf::from(value));
                index += 2;
            }
            other => return Err(format!("unknown argument `{other}`")),
        }
    }

    Ok(Some(Args {
        root,
        max_candidates,
        write_report,
        write_packets,
        check_policy,
    }))
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

fn check_policy_fresh(root: &Path) -> Result<(), String> {
    let mut typecheck = Command::new("nickel");
    typecheck.arg("typecheck").arg(root.join(POLICY_SOURCE));
    require_success(command_output(typecheck, "nickel typecheck")?, "nickel typecheck")?;

    let mut export = Command::new("nickel");
    export.arg("export").arg(root.join(POLICY_SOURCE));
    let exported = require_success(command_output(export, "nickel export")?, "nickel export")?.stdout;
    let generated_path = root.join(GENERATED_POLICY);
    let checked_in =
        fs::read(&generated_path).map_err(|err| format!("failed to read {}: {err}", generated_path.display()))?;
    if checked_in != exported {
        return Err(format!(
            "generated proof-review policy is stale: run `nickel export {POLICY_SOURCE} > {GENERATED_POLICY}`"
        ));
    }
    println!("PASS proof-review policy fresh {GENERATED_POLICY}");
    Ok(())
}

fn collect_source_files(root: &Path) -> Result<Vec<SourceFile>, String> {
    let src = root.join(SRC_DIR);
    let mut paths = Vec::new();
    for entry in fs::read_dir(&src).map_err(|err| format!("failed to read {}: {err}", src.display()))? {
        let entry = entry.map_err(|err| format!("failed to read src entry: {err}"))?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some(RUST_EXTENSION) {
            paths.push(path);
        }
    }
    paths.sort();

    let mut sources = Vec::new();
    for path in paths {
        let text = fs::read_to_string(&path).map_err(|err| format!("failed to read {}: {err}", path.display()))?;
        let display_path = display_path(root, &path);
        sources.push(SourceFile { display_path, text });
    }
    Ok(sources)
}

fn display_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\', "/")
}

fn collect_changed_paths(root: &Path) -> BTreeSet<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["status", "--short", "--untracked-files=all"])
        .output();
    let Ok(output) = output else { return BTreeSet::new() };
    if !output.status.success() {
        return BTreeSet::new();
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines().filter_map(parse_status_path).collect()
}

fn parse_status_path(line: &str) -> Option<String> {
    if line.len() <= STATUS_PREFIX_BYTES {
        return None;
    }
    let raw = &line[STATUS_PREFIX_BYTES..];
    let path = raw.split(" -> ").last().unwrap_or(raw).trim();
    if path.is_empty() { None } else { Some(path.to_string()) }
}

fn collect_active_changes(root: &Path) -> Result<Vec<ActiveChange>, String> {
    let changes_root = root.join(CAIRN_CHANGES_DIR);
    if !changes_root.exists() {
        return Ok(Vec::new());
    }
    let mut changes = Vec::new();
    for entry in
        fs::read_dir(&changes_root).map_err(|err| format!("failed to read {}: {err}", changes_root.display()))?
    {
        let entry = entry.map_err(|err| format!("failed to read change entry: {err}"))?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if name == ARCHIVE_DIR_NAME {
            continue;
        }
        changes.push(read_active_change(root, name.to_string(), &path)?);
    }
    changes.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(changes)
}

fn read_active_change(root: &Path, name: String, change_dir: &Path) -> Result<ActiveChange, String> {
    let mut referenced_paths = BTreeSet::new();
    let mut markdown_files = Vec::new();
    collect_markdown_files(change_dir, &mut markdown_files)?;
    for file in markdown_files {
        let text = fs::read_to_string(&file).map_err(|err| format!("failed to read {}: {err}", file.display()))?;
        for path in extract_source_paths(&text) {
            referenced_paths.insert(path);
        }
    }

    let tasks_path = change_dir.join(TASKS_FILE_NAME);
    let mut unchecked_tasks = 0usize;
    let mut task_excerpt = Vec::new();
    if tasks_path.exists() {
        let text =
            fs::read_to_string(&tasks_path).map_err(|err| format!("failed to read {}: {err}", tasks_path.display()))?;
        for line in text.lines() {
            if line.trim_start().starts_with("- [ ]") {
                unchecked_tasks += 1;
                task_excerpt.push(line.to_string());
            }
        }
        for path in extract_source_paths(&text) {
            referenced_paths.insert(path);
        }
    }

    let display_change_dir = display_path(root, change_dir);
    if referenced_paths.is_empty() {
        referenced_paths.insert(display_change_dir);
    }

    Ok(ActiveChange {
        name,
        unchecked_tasks,
        referenced_paths,
        task_excerpt,
    })
}

fn collect_markdown_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|err| format!("failed to read {}: {err}", dir.display()))? {
        let entry = entry.map_err(|err| format!("failed to read markdown entry: {err}"))?;
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().and_then(|name| name.to_str()) == Some(EVIDENCE_DIR_NAME) {
                continue;
            }
            collect_markdown_files(&path, out)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some(MARKDOWN_EXTENSION) {
            out.push(path);
        }
    }
    out.sort();
    Ok(())
}

fn extract_source_paths(text: &str) -> BTreeSet<String> {
    let mut paths = BTreeSet::new();
    let mut rest = text;
    while let Some(start) = rest.find(SOURCE_PATH_PREFIX) {
        rest = &rest[start..];
        let Some(end) = rest.find(SOURCE_PATH_SUFFIX) else {
            break;
        };
        let candidate = &rest[..end + SOURCE_PATH_SUFFIX.len()];
        let clean = candidate.trim_matches(|ch: char| ch == '`' || ch == ')' || ch == '(' || ch == ',' || ch == '.');
        if clean.starts_with(SOURCE_PATH_PREFIX) {
            paths.insert(clean.to_string());
        }
        rest = &rest[end + SOURCE_PATH_SUFFIX.len()..];
    }
    paths
}

fn build_review_inputs(root: &Path) -> Result<ReviewInputs, String> {
    let sources = collect_source_files(root)?;
    let active_changes = collect_active_changes(root)?;
    let changed_paths = collect_changed_paths(root);
    let legacy_openspec_changes_present = root.join(LEGACY_OPENSPEC_CHANGES_DIR).exists();
    Ok(ReviewInputs {
        sources,
        active_changes,
        changed_paths,
        legacy_openspec_changes_present,
    })
}

fn rank_candidates(inputs: &ReviewInputs) -> Vec<Candidate> {
    let mut active_by_path: BTreeMap<String, Vec<&ActiveChange>> = BTreeMap::new();
    for change in &inputs.active_changes {
        for path in &change.referenced_paths {
            active_by_path.entry(path.clone()).or_default().push(change);
        }
    }

    let mut candidates = Vec::new();
    for source in &inputs.sources {
        let mut findings = analyze_source_text(&source.display_path, &source.text);
        if inputs.changed_paths.contains(&source.display_path) {
            findings.push(Finding {
                rule_id: "worktree-changed-source".to_string(),
                score: SCORE_WORKTREE_CHANGED,
                line: None,
                title: "source file is changed in the working tree".to_string(),
                detail: "Changed verified source is immediately actionable for proof critique.".to_string(),
            });
        }
        if let Some(changes) = active_by_path.get(&source.display_path) {
            for change in changes {
                let score = SCORE_ACTIVE_CHANGE + SCORE_ACTIVE_UNCHECKED_TASK * change.unchecked_tasks;
                findings.push(Finding {
                    rule_id: "active-cairn-change".to_string(),
                    score,
                    line: None,
                    title: format!("referenced by active Cairn change `{}`", change.name),
                    detail: format!("{} unchecked task(s) remain in the active change.", change.unchecked_tasks),
                });
            }
        }

        let score = candidate_score(&findings);
        if score > 0 {
            findings.sort_by(|left, right| right.score.cmp(&left.score).then_with(|| left.line.cmp(&right.line)));
            candidates.push(Candidate {
                path: source.display_path.clone(),
                score,
                findings,
            });
        }
    }

    candidates.sort_by(|left, right| right.score.cmp(&left.score).then_with(|| left.path.cmp(&right.path)));
    candidates
}

fn candidate_score(findings: &[Finding]) -> usize {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    let mut score = 0usize;
    for finding in findings {
        let count = counts.entry(finding.rule_id.as_str()).or_default();
        if *count < MAX_SCORE_FINDINGS_PER_RULE {
            score += finding.score;
        }
        *count += 1;
    }
    score
}

fn analyze_source_text(display_path: &str, text: &str) -> Vec<Finding> {
    let lines: Vec<&str> = text.lines().collect();
    let mut findings = Vec::new();
    add_trust_marker_findings(&lines, &mut findings);
    add_proof_shape_findings(&lines, &mut findings);
    add_active_prefix_findings(&lines, &mut findings);
    add_test_hint_findings(display_path, text, &mut findings);
    findings
}

fn add_trust_marker_findings(lines: &[&str], findings: &mut Vec<Finding>) {
    for (index, line) in lines.iter().enumerate() {
        for marker in TRUST_MARKERS {
            if line.contains(marker) {
                let line_number = index + 1;
                let permanent = has_nearby_permanent_marker(lines, index);
                let score = if permanent {
                    SCORE_PERMANENT_BOUNDARY
                } else if marker == "#[verifier::external_body]" {
                    SCORE_UNTRUSTED_BOUNDARY
                } else {
                    SCORE_ASSUME_MARKER
                };
                findings.push(Finding {
                    rule_id: if permanent {
                        "permanent-boundary-marker"
                    } else {
                        "untrusted-boundary-marker"
                    }
                    .to_string(),
                    score,
                    line: Some(line_number),
                    title: if permanent {
                        "permanent trusted boundary marker"
                    } else {
                        "trusted or weakened proof marker"
                    }
                    .to_string(),
                    detail: format!("found `{marker}`; confirm classification and avoid expanding trust."),
                });
            }
        }
    }
}

fn has_nearby_permanent_marker(lines: &[&str], index: usize) -> bool {
    let start = index.saturating_sub(CONTEXT_RADIUS_LINES);
    lines[start..=index].iter().any(|line| PERMANENT_MARKERS.iter().any(|marker| line.contains(marker)))
}

fn add_proof_shape_findings(lines: &[&str], findings: &mut Vec<Finding>) {
    let mut index = 0usize;
    while index < lines.len() {
        let trimmed = lines[index].trim_start();
        if starts_proof_fn(trimmed) {
            let block = collect_function_block(lines, index);
            let signature = signature_text(&block.text);
            let name = proof_name(trimmed).unwrap_or_else(|| "<unknown>".to_string());
            if signature.contains("ensures true") {
                let catalogue_bonus =
                    usize::from(name.contains("coverage") || name.contains("catalog")) * SCORE_CATALOGUE_PROOF;
                findings.push(Finding {
                    rule_id: "vacuous-ensures-true".to_string(),
                    score: SCORE_VACUOUS_ENSURES_TRUE + catalogue_bonus,
                    line: Some(index + 1),
                    title: format!("proof `{name}` has `ensures true`"),
                    detail: "Replace catalogue-only postconditions with concrete implications or document it as Tracey-only metadata.".to_string(),
                });
            }
            if body_is_empty(&block.text) && nontrivial_signature(&signature) && !signature.contains("ensures true") {
                findings.push(Finding {
                    rule_id: "empty-nontrivial-proof".to_string(),
                    score: SCORE_EMPTY_NONTRIVIAL_PROOF,
                    line: Some(index + 1),
                    title: format!("empty proof body for `{name}` with nontrivial postcondition"),
                    detail: "Ask whether this proof is intentionally SMT-trivial, vacuous from preconditions, or missing helper lemmas/tests.".to_string(),
                });
            }
            index = block.end_line;
        } else {
            index += 1;
        }
    }
}

#[derive(Debug, Clone)]
struct FunctionBlock {
    text: String,
    end_line: usize,
}

fn collect_function_block(lines: &[&str], start: usize) -> FunctionBlock {
    let mut text = String::new();
    let mut depth = 0isize;
    let mut saw_open = false;
    let mut end_line = start + 1;
    for (offset, line) in lines[start..].iter().enumerate() {
        text.push_str(line);
        text.push('\n');
        for ch in line.chars() {
            if ch == '{' {
                saw_open = true;
                depth += 1;
            } else if ch == '}' && saw_open {
                depth -= 1;
            }
        }
        end_line = start + offset + 1;
        if saw_open && depth <= 0 {
            break;
        }
    }
    FunctionBlock { text, end_line }
}

fn signature_text(block: &str) -> String {
    match block.find('{') {
        Some(position) => block[..position].to_string(),
        None => block.to_string(),
    }
}

fn starts_proof_fn(trimmed: &str) -> bool {
    trimmed.starts_with("proof fn ") || trimmed.starts_with("pub proof fn ")
}

fn proof_name(trimmed: &str) -> Option<String> {
    let rest = trimmed.strip_prefix("pub proof fn ").or_else(|| trimmed.strip_prefix("proof fn "))?;
    let end = rest.find(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_')).unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

fn body_is_empty(block: &str) -> bool {
    let Some(open) = block.find('{') else { return false };
    let Some(close) = block.rfind('}') else { return false };
    if close <= open {
        return false;
    }
    let inner = &block[open + 1..close];
    strip_line_comments(inner).trim().is_empty()
}

fn strip_line_comments(text: &str) -> String {
    let mut result = String::new();
    for line in text.lines() {
        let kept = line.split("//").next().unwrap_or("");
        result.push_str(kept);
        result.push('\n');
    }
    result
}

fn nontrivial_signature(signature: &str) -> bool {
    signature.contains("ensures") && NONTRIVIAL_POSTCONDITION_TOKENS.iter().any(|token| signature.contains(token))
}

fn add_active_prefix_findings(lines: &[&str], findings: &mut Vec<Finding>) {
    for (index, line) in lines.iter().enumerate() {
        if ACTIVE_PREFIX_KEYWORDS.iter().any(|keyword| line.contains(keyword)) {
            findings.push(Finding {
                rule_id: "active-prefix-semantics".to_string(),
                score: SCORE_ACTIVE_PREFIX,
                line: Some(index + 1),
                title: "active-prefix or bounded-length semantics".to_string(),
                detail: "Review boundary lemmas and sad-path tests for zero, max, inactive, out-of-range, and fuel exhaustion cases.".to_string(),
            });
            return;
        }
    }
}

fn add_test_hint_findings(display_path: &str, text: &str, findings: &mut Vec<Finding>) {
    let has_active_prefix = ACTIVE_PREFIX_KEYWORDS.iter().any(|keyword| text.contains(keyword));
    if !has_active_prefix || display_path == "src/lib.rs" {
        return;
    }
    let test_text = test_sections(text);
    if test_text.trim().is_empty() {
        findings.push(Finding {
            rule_id: "missing-negative-test-hints".to_string(),
            score: SCORE_NO_TEST_MODULE,
            line: None,
            title: "active-prefix source has no visible test module".to_string(),
            detail: "Add positive and negative tests or document why this module is proof-only.".to_string(),
        });
        return;
    }
    if !NEGATIVE_TEST_KEYWORDS.iter().any(|keyword| test_text.contains(keyword)) {
        findings.push(Finding {
            rule_id: "missing-negative-test-hints".to_string(),
            score: SCORE_MISSING_TEST_HINTS,
            line: None,
            title: "test names lack obvious negative active-prefix cases".to_string(),
            detail: "Look for malformed, zero, inactive, out-of-range, overlarge, or mismatch tests.".to_string(),
        });
    }
}

fn test_sections(text: &str) -> String {
    let mut result = String::new();
    let mut in_test_context = false;
    for line in text.lines() {
        if line.contains("#[cfg(test)]") || line.contains("#[test]") {
            in_test_context = true;
        }
        if in_test_context {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

fn render_report(inputs: &ReviewInputs, candidates: &[Candidate], max_candidates: usize) -> String {
    let mut out = String::new();
    out.push_str("# Proof Review Candidates\n\n");
    out.push_str("Deterministic prefilter for focused VibeThinker proof critique. VibeThinker output is advisory; validation remains cargo, Verus, Tracey, Cairn, and repo checkers.\n\n");
    if inputs.legacy_openspec_changes_present {
        out.push_str("> Note: legacy `openspec/changes` exists but is ignored by this native Cairn scanner. Native `cairn/changes` is the authoritative lifecycle input; legacy cleanup/migration is explicit.\n\n");
    }
    out.push_str("## Active Cairn Changes\n\n");
    if inputs.active_changes.is_empty() {
        out.push_str("- none\n\n");
    } else {
        for change in &inputs.active_changes {
            out.push_str(&format!(
                "- `{}`: {} unchecked task(s); referenced source paths: {}\n",
                change.name,
                change.unchecked_tasks,
                comma_list(change.referenced_paths.iter())
            ));
        }
        out.push('\n');
    }

    out.push_str("## Ranked Candidates\n\n");
    out.push_str("| Rank | Score | Path | Top reasons |\n");
    out.push_str("| ---: | ---: | --- | --- |\n");
    for (index, candidate) in candidates.iter().take(max_candidates).enumerate() {
        let reasons = candidate
            .findings
            .iter()
            .take(DEFAULT_MAX_PACKET_FINDINGS)
            .map(|finding| finding.title.as_str())
            .collect::<Vec<_>>()
            .join("; ");
        out.push_str(&format!(
            "| {} | {} | `{}` | {} |\n",
            index + 1,
            candidate.score,
            candidate.path,
            markdown_escape(&truncate(&reasons, MAX_REASON_SUMMARY_CHARS))
        ));
    }
    out.push('\n');

    out.push_str("## Candidate Details\n\n");
    for candidate in candidates.iter().take(max_candidates) {
        out.push_str(&format!("### `{}` — score {}\n\n", candidate.path, candidate.score));
        out.push_str(&format!(
            "Showing top {} of {} finding(s); score caps repeated findings for the same rule at {} occurrence(s).\n\n",
            usize::min(DEFAULT_MAX_PACKET_FINDINGS, candidate.findings.len()),
            candidate.findings.len(),
            MAX_SCORE_FINDINGS_PER_RULE
        ));
        for finding in candidate.findings.iter().take(DEFAULT_MAX_PACKET_FINDINGS) {
            let location = finding.line.map(|line| format!(":{line}")).unwrap_or_default();
            out.push_str(&format!(
                "- **{}** (+{}): `{}`{} — {}\n",
                finding.rule_id, finding.score, candidate.path, location, finding.detail
            ));
        }
        out.push('\n');
    }

    out.push_str("## VibeThinker Prompt Shape\n\n");
    out.push_str(&format!(
        "Use `max_tokens={}`. Packets include a base task, a strict output contract, and rule-specific focus bullets derived from the top scanner findings.\n",
        VIBETHINKER_MAX_TOKENS_HINT
    ));
    out
}

fn comma_list<'a, I>(items: I) -> String
where I: Iterator<Item = &'a String> {
    let values: Vec<&str> = items.map(String::as_str).collect();
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(", ")
    }
}

fn markdown_escape(text: &str) -> String {
    text.replace('|', "\\|").replace('\n', " ")
}

fn truncate(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let mut result = text.chars().take(max_chars.saturating_sub(1)).collect::<String>();
    result.push('…');
    result
}

fn write_report(path: &Path, report: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::write(path, report).map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn write_packets(
    root: &Path,
    packet_dir: &Path,
    candidates: &[Candidate],
    max_candidates: usize,
) -> Result<(), String> {
    fs::create_dir_all(packet_dir).map_err(|err| format!("failed to create {}: {err}", packet_dir.display()))?;
    let mut source_by_path = BTreeMap::new();
    for source in collect_source_files(root)? {
        source_by_path.insert(source.display_path, source.text);
    }
    for candidate in candidates.iter().take(max_candidates) {
        let Some(text) = source_by_path.get(&candidate.path) else {
            continue;
        };
        let packet = render_packet(candidate, text);
        let file_name = format!("{}.md", sanitize_packet_name(&candidate.path));
        let path = packet_dir.join(file_name);
        fs::write(&path, packet).map_err(|err| format!("failed to write {}: {err}", path.display()))?;
    }
    Ok(())
}

fn sanitize_packet_name(path: &str) -> String {
    path.chars().map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' }).collect()
}

fn specialized_prompt_guidance(findings: &[Finding]) -> Vec<&'static str> {
    let mut guidance = Vec::new();
    for finding in findings.iter().take(DEFAULT_MAX_PACKET_FINDINGS) {
        let prompt = prompt_guidance_for_rule(&finding.rule_id);
        if !guidance.contains(&prompt) {
            guidance.push(prompt);
        }
    }
    guidance
}

fn prompt_guidance_for_rule(rule_id: &str) -> &'static str {
    match rule_id {
        "active-cairn-change" => {
            "For `active-cairn-change`: check scope drift, unchecked-task claims, and whether evidence supports archive readiness."
        }
        "worktree-changed-source" => {
            "For `worktree-changed-source`: critique the current patch for overclaims, missing postconditions, and missing negative tests."
        }
        "untrusted-boundary-marker" => {
            "For `untrusted-boundary-marker`: confirm the boundary classification and name any way to reduce or better document trust without adding new trust."
        }
        "permanent-boundary-marker" => {
            "For `permanent-boundary-marker`: only flag stale rationale, missing manifest coverage, or accidental expansion of the permanent boundary."
        }
        "vacuous-ensures-true" => {
            "For `vacuous-ensures-true`: propose concrete non-vacuous postconditions using existing specs and local helper proofs."
        }
        "empty-nontrivial-proof" => {
            "For `empty-nontrivial-proof`: decide whether the proof is definitional, vacuous from preconditions, or missing a small lemma; give one actionable improvement."
        }
        "active-prefix-semantics" => {
            "For `active-prefix-semantics`: list boundary cases for zero, max, inactive, out-of-range, mismatch, and fuel/termination paths that are not already tested."
        }
        "missing-negative-test-hints" => {
            "For `missing-negative-test-hints`: list only executable sad-path tests with concrete malformed inputs and expected false/error results."
        }
        _ => "For unclassified findings: return only new concrete proof, spec, or executable-test gaps.",
    }
}

fn render_packet(candidate: &Candidate, text: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# Proof Review Packet: `{}`\n\n", candidate.path));
    out.push_str(&format!("Score: `{}`\n\n", candidate.score));
    out.push_str("## Ask VibeThinker\n\n");
    out.push_str(&format!("Use `max_tokens={VIBETHINKER_MAX_TOKENS_HINT}` and ask:\n\n"));
    out.push_str("```text\n");
    out.push_str(VIBETHINKER_BASE_TASK);
    out.push_str("\n```");
    out.push_str("\n\nRequested output:\n");
    for item in VIBETHINKER_OUTPUT_CONTRACT {
        out.push_str(&format!("- {item}\n"));
    }
    let guidance = specialized_prompt_guidance(&candidate.findings);
    if !guidance.is_empty() {
        out.push_str("\nRule-specific focus:\n");
        for item in guidance {
            out.push_str(&format!("- {item}\n"));
        }
    }
    out.push_str("\n## Findings\n\n");
    for finding in candidate.findings.iter().take(DEFAULT_MAX_PACKET_FINDINGS) {
        let location = finding.line.map(|line| format!(":{line}")).unwrap_or_default();
        out.push_str(&format!(
            "- `{}`{} **{}** (+{}): {}\n",
            candidate.path, location, finding.rule_id, finding.score, finding.detail
        ));
    }
    out.push_str("\n## Source Overview\n\n");
    let lines: Vec<&str> = text.lines().collect();
    let overview_end = usize::min(lines.len(), SOURCE_OVERVIEW_LINES);
    out.push_str(&format!("First {} line(s), to include nearby spec and exec surfaces.\n\n", overview_end));
    out.push_str("```rust\n");
    for (offset, source_line) in lines[..overview_end].iter().enumerate() {
        out.push_str(&format!("{:>4}: {}\n", offset + 1, source_line));
    }
    out.push_str("```\n\n");

    let test_excerpt = test_excerpt(text, TEST_OVERVIEW_LINES);
    if !test_excerpt.is_empty() {
        out.push_str("## Test Excerpt\n\n");
        out.push_str("```rust\n");
        out.push_str(&test_excerpt);
        out.push_str("```\n\n");
    }

    out.push_str("## Focused Source Excerpts\n\n");
    let mut emitted = BTreeSet::new();
    for finding in candidate.findings.iter().take(DEFAULT_MAX_PACKET_FINDINGS) {
        let Some(line) = finding.line else { continue };
        let start = line.saturating_sub(CONTEXT_RADIUS_LINES + 1);
        let end = usize::min(lines.len(), line + CONTEXT_RADIUS_LINES);
        if !emitted.insert((start, end)) {
            continue;
        }
        out.push_str(&format!("### Lines {}-{}\n\n", start + 1, end));
        out.push_str("```rust\n");
        for (offset, source_line) in lines[start..end].iter().enumerate() {
            out.push_str(&format!("{:>4}: {}\n", start + offset + 1, source_line));
        }
        out.push_str("```\n\n");
    }
    out
}

fn test_excerpt(text: &str, max_lines: usize) -> String {
    let mut result = String::new();
    let mut in_test_context = false;
    let mut emitted = 0usize;
    for (index, line) in text.lines().enumerate() {
        if line.contains("#[cfg(test)]") {
            in_test_context = true;
        }
        if in_test_context && emitted < max_lines {
            result.push_str(&format!("{:>4}: {}\n", index + 1, line));
            emitted += 1;
        }
    }
    result
}

fn run_self_test() -> Result<(), String> {
    let weak_source = r#"
use vstd::prelude::*;
verus! {
pub open spec fn active_table(table_len: u8, value: u8) -> bool {
    table_len > 0 && value < table_len
}
// r[verify demo.coverage]
pub proof fn coverage_catalogued()
    ensures true
{}
// r[verify demo.reject]
pub proof fn malformed_rejected(ok: bool)
    requires !ok
    ensures !ok
{}
}
#[cfg(test)]
mod tests {
    #[test]
    fn accepts_member() {}
}
"#;
    let strong_source = r#"
use vstd::prelude::*;
verus! {
pub proof fn explicit_bool(ok: bool)
    requires ok
    ensures ok
{
    assert(ok);
}
}
#[cfg(test)]
mod tests {
    #[test]
    fn rejects_zero_inactive_out_of_range() {}
}
"#;

    let weak_findings = analyze_source_text("src/weak.rs", weak_source);
    require_finding(&weak_findings, "vacuous-ensures-true")?;
    require_finding(&weak_findings, "empty-nontrivial-proof")?;
    require_finding(&weak_findings, "active-prefix-semantics")?;
    require_finding(&weak_findings, "missing-negative-test-hints")?;

    let strong_findings = analyze_source_text("src/strong.rs", strong_source);
    reject_finding(&strong_findings, "vacuous-ensures-true")?;
    reject_finding(&strong_findings, "missing-negative-test-hints")?;

    let fixture_root = create_self_test_fixture(weak_source, strong_source)?;
    let inputs = build_review_inputs(&fixture_root)?;
    let candidates = rank_candidates(&inputs);
    let weak = candidates
        .iter()
        .find(|candidate| candidate.path == "src/weak.rs")
        .ok_or("self-test fixture did not rank src/weak.rs")?;
    require_finding(&weak.findings, "active-cairn-change")?;
    reject_active_candidate(&candidates, "src/noise.rs")?;
    let report = render_report(&inputs, &candidates, DEFAULT_MAX_CANDIDATES);
    if !report.contains("src/weak.rs") || !report.contains("VibeThinker") {
        return Err("self-test report omitted expected target or prompt shape".to_string());
    }
    let packet = render_packet(weak, weak_source);
    if !packet.contains("## Test Excerpt") || !packet.contains("accepts_member") {
        return Err("self-test packet omitted expected test excerpt".to_string());
    }
    if !packet.contains("Requested output:")
        || !packet.contains("For `vacuous-ensures-true`")
        || !packet.contains("For `empty-nontrivial-proof`")
        || !packet.contains("without assuming the target or current patch is correct")
        || !packet.contains("Group materially different proposals by proof mechanism")
        || !packet.contains("equivalent or stronger gaps remain blocked")
        || !packet.contains("concrete counterexample or failed proof obligation")
        || !packet.contains("Audit precondition satisfiability")
        || !packet.contains("not tests for proof functions")
        || !packet.contains("violating `requires`")
        || !packet.contains("Separate accepted actionable items")
    {
        return Err("self-test packet omitted specialized VibeThinker prompt guidance".to_string());
    }

    println!("proof-review-candidates self-test: PASS");
    Ok(())
}

fn require_finding(findings: &[Finding], rule_id: &str) -> Result<(), String> {
    if findings.iter().any(|finding| finding.rule_id == rule_id) {
        Ok(())
    } else {
        Err(format!("expected finding `{rule_id}`"))
    }
}

fn reject_finding(findings: &[Finding], rule_id: &str) -> Result<(), String> {
    if findings.iter().any(|finding| finding.rule_id == rule_id) {
        Err(format!("unexpected finding `{rule_id}`"))
    } else {
        Ok(())
    }
}

fn reject_active_candidate(candidates: &[Candidate], path: &str) -> Result<(), String> {
    let has_active_finding = candidates
        .iter()
        .find(|candidate| candidate.path == path)
        .map(|candidate| candidate.findings.iter().any(|finding| finding.rule_id == "active-cairn-change"))
        .unwrap_or(false);
    if has_active_finding {
        Err(format!("evidence-only path `{path}` was treated as an active-change source reference"))
    } else {
        Ok(())
    }
}

fn create_self_test_fixture(weak_source: &str, noise_source: &str) -> Result<PathBuf, String> {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(SELF_TEST_UNIQUE_FALLBACK);
    let root = std::env::current_dir()
        .map_err(|err| format!("failed to get cwd for self-test: {err}"))?
        .join("target-test")
        .join(format!("proof-review-self-test-{}-{unique}", std::process::id()));
    let src = root.join(SRC_DIR);
    fs::create_dir_all(&src).map_err(|err| format!("failed to create {}: {err}", src.display()))?;
    fs::write(src.join("weak.rs"), weak_source).map_err(|err| format!("failed to write weak fixture: {err}"))?;
    fs::write(src.join("noise.rs"), noise_source).map_err(|err| format!("failed to write noise fixture: {err}"))?;
    let change = root.join(CAIRN_CHANGES_DIR).join("demo-proof-review");
    fs::create_dir_all(&change).map_err(|err| format!("failed to create {}: {err}", change.display()))?;
    fs::write(change.join(TASKS_FILE_NAME), "# Tasks\n\n- [ ] Review `src/weak.rs` proof slice.\n")
        .map_err(|err| format!("failed to write tasks fixture: {err}"))?;
    let evidence = change.join(EVIDENCE_DIR_NAME);
    fs::create_dir_all(&evidence).map_err(|err| format!("failed to create {}: {err}", evidence.display()))?;
    fs::write(evidence.join("proof-review-candidates.md"), "# Archived scanner output\n\n## Candidate `src/noise.rs`\n")
        .map_err(|err| format!("failed to write evidence fixture: {err}"))?;
    Ok(root)
}

fn main() -> Result<(), String> {
    let Some(mut args) = parse_args()? else { return Ok(()) };
    args.root = args.root.canonicalize().unwrap_or_else(|_| args.root.clone());
    std::env::set_current_dir(&args.root).map_err(|err| format!("failed to cd {}: {err}", args.root.display()))?;
    if args.check_policy {
        check_policy_fresh(&args.root)?;
    }
    let inputs = build_review_inputs(&args.root)?;
    let candidates = rank_candidates(&inputs);
    let report = render_report(&inputs, &candidates, args.max_candidates);
    write_report(&args.write_report, &report)?;
    if let Some(packet_dir) = &args.write_packets {
        write_packets(&args.root, packet_dir, &candidates, args.max_candidates)?;
    }
    print!("{report}");
    println!("wrote report: {}", args.write_report.display());
    if let Some(packet_dir) = &args.write_packets {
        println!("wrote packets: {}", packet_dir.display());
    }
    Ok(())
}

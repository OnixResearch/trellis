#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Bounded proof-review loop driver.
//!
//! Functional core: policy parsing, scanner-report parsing, target selection,
//! mode planning, slug generation, validation-plan construction, and receipt
//! rendering.
//!
//! Imperative shell: reads repo state, runs the existing scanner, creates Cairn
//! packages in assisted/apply modes, captures evidence, and writes receipts.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

const POLICY_JSON: &str = "proof-review/generated/policy.json";
const SCANNER_SCRIPT: &str = "./scripts/proof-review-candidates.rs";
const CAIRN_CHANGES_DIR: &str = "cairn/changes";
const ARCHIVE_DIR_NAME: &str = "archive";
const PROOF_REVIEW_SPEC_ID: &str = "proof-review-automation";
const TARGET_CARGO_DIR: &str = "target/proof-review-cargo-target";
const RECEIPT_FILE_NAME: &str = "proof-review-loop.json";
const SCANNER_REPORT_NAME: &str = "candidates.md";
const PACKET_FILE_EXTENSION: &str = ".md";
const MARKDOWN_HEADING_PREFIX: &str = "### `";
const MARKDOWN_SCORE_MARKER: &str = "score ";
const FINDING_PREFIX: &str = "- **";
const FINDING_RULE_END: &str = "**";
const SRC_PREFIX: &str = "src/";
const RUST_SUFFIX: &str = ".rs";
const STOP_DIRTY_TREE: &str = "dirty-tree";
const STOP_ACTIVE_CHANGE: &str = "active-cairn-change";
const STOP_TRUST_BOUNDARY_ONLY: &str = "trust-boundary-only";
const STOP_UNSUPPORTED_APPLY: &str = "unsupported-apply-pattern";
const STOP_PUSH_DISABLED: &str = "push-disabled";
const STOP_NO_CANDIDATE: &str = "no-candidate";
const STOP_INVALID_BUDGET: &str = "invalid-iteration-budget";
const MODE_DRY_RUN: &str = "dry-run";
const MODE_ASSISTED: &str = "assisted";
const MODE_APPLY: &str = "apply";
const CARGO_TARGET_ENV: &str = "CARGO_TARGET_DIR";
const CAIRN_BIN_ENV: &str = "CAIRN_BIN";
const LOCAL_CAIRN_FLAKE: &str = "/home/brittonr/git/cairn/flake.nix";
const LOCAL_CAIRN_APP: &str = "path:/home/brittonr/git/cairn#cairn";
const DIRECT_CAIRN_BIN: &str = "cairn";
const NIX_BIN: &str = "nix";
const NIX_RUN_ARG: &str = "run";
const NIX_SEPARATOR_ARG: &str = "--";
const GIT_STATUS_ARG_COUNT: usize = 3;
const GIT_STATUS_ARGS: [&str; GIT_STATUS_ARG_COUNT] = ["status", "--short", "--untracked-files=all"];
const ARG_WITH_VALUE_WIDTH: usize = 2;
const JSON_DECIMAL_RADIX: u64 = 10;
const JSON_DIGIT_ZERO: u8 = b'0';
const JSON_DIGIT_NINE: u8 = b'9';
const LOWEST_PREFERENCE: usize = usize::MAX;
const SELF_TEST_MAX_CANDIDATES: usize = 12;
const SELF_TEST_VALIDATION_COMMAND_COUNT: usize = 2;
const SELF_TEST_CANDIDATE_COUNT: usize = 3;
const SELF_TEST_DRY_RUN_COMMAND_COUNT: usize = 1;
const SELF_TEST_APPLY_COMMAND_COUNT: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoopMode {
    DryRun,
    Assisted,
    Apply,
}

impl LoopMode {
    fn parse(input: &str) -> Result<Self, String> {
        match input {
            MODE_DRY_RUN => Ok(Self::DryRun),
            MODE_ASSISTED => Ok(Self::Assisted),
            MODE_APPLY => Ok(Self::Apply),
            other => Err(format!("unknown mode `{other}`; expected dry-run, assisted, or apply")),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::DryRun => MODE_DRY_RUN,
            Self::Assisted => MODE_ASSISTED,
            Self::Apply => MODE_APPLY,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Args {
    root: PathBuf,
    mode: LoopMode,
    max_iterations: usize,
    push_requested: bool,
    self_test: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidationCommand {
    id: String,
    modes: Vec<String>,
    command: Vec<String>,
    evidence: String,
    required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ApplyRecipe {
    id: String,
    rule_id: String,
    requires_no_trust_markers: bool,
    mutates_source: bool,
    validation_gate: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LoopPolicy {
    default_report: String,
    default_packet_dir: String,
    max_candidates: usize,
    max_packet_findings: usize,
    default_max_iterations: usize,
    max_iterations_limit: usize,
    receipt_dir: String,
    no_push_default: bool,
    slug_prefix: String,
    preferred_candidate_rules: Vec<String>,
    skip_trust_boundary_rules: Vec<String>,
    validation_commands: Vec<ValidationCommand>,
    apply_recipes: Vec<ApplyRecipe>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CandidateSummary {
    path: String,
    score: usize,
    rule_ids: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StopPlan {
    reason: String,
    detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReadyPlan {
    mode: LoopMode,
    candidate: CandidateSummary,
    change_slug: String,
    validation: Vec<ValidationCommand>,
    apply_recipe: Option<ApplyRecipe>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LoopPlan {
    Stop(StopPlan),
    Ready(ReadyPlan),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(u64),
    String(String),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
}

struct JsonParser<'a> {
    input: &'a [u8],
    position: usize,
}

impl<'a> JsonParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            position: 0,
        }
    }

    fn parse(mut self) -> Result<JsonValue, String> {
        let value = self.parse_value()?;
        self.skip_whitespace();
        if self.position == self.input.len() {
            Ok(value)
        } else {
            Err(format!("trailing JSON input at byte {}", self.position))
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();
        let Some(byte) = self.peek() else {
            return Err("unexpected end of JSON input".to_string());
        };
        match byte {
            b'n' => self.parse_literal(b"null", JsonValue::Null),
            b't' => self.parse_literal(b"true", JsonValue::Bool(true)),
            b'f' => self.parse_literal(b"false", JsonValue::Bool(false)),
            b'"' => Ok(JsonValue::String(self.parse_string()?)),
            b'[' => self.parse_array(),
            b'{' => self.parse_object(),
            JSON_DIGIT_ZERO..=JSON_DIGIT_NINE => self.parse_number(),
            other => Err(format!("unexpected JSON byte `{}` at {}", char::from(other), self.position)),
        }
    }

    fn parse_literal(&mut self, literal: &[u8], value: JsonValue) -> Result<JsonValue, String> {
        if self.input[self.position..].starts_with(literal) {
            self.position += literal.len();
            Ok(value)
        } else {
            Err(format!("expected JSON literal at byte {}", self.position))
        }
    }

    fn parse_string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;
        let mut result = String::new();
        loop {
            let Some(byte) = self.bump() else {
                return Err("unterminated JSON string".to_string());
            };
            match byte {
                b'"' => return Ok(result),
                b'\\' => {
                    let Some(escaped) = self.bump() else {
                        return Err("unterminated JSON escape".to_string());
                    };
                    let ch = match escaped {
                        b'"' => '"',
                        b'\\' => '\\',
                        b'/' => '/',
                        b'b' => '\u{0008}',
                        b'f' => '\u{000c}',
                        b'n' => '\n',
                        b'r' => '\r',
                        b't' => '\t',
                        b'u' => return Err("unicode JSON escapes are not supported by this policy parser".to_string()),
                        other => return Err(format!("unsupported JSON escape `{}`", char::from(other))),
                    };
                    result.push(ch);
                }
                other => result.push(char::from(other)),
            }
        }
    }

    fn parse_number(&mut self) -> Result<JsonValue, String> {
        let mut value = 0u64;
        let mut saw_digit = false;
        while let Some(byte) = self.peek() {
            if !(JSON_DIGIT_ZERO..=JSON_DIGIT_NINE).contains(&byte) {
                break;
            }
            saw_digit = true;
            let digit = u64::from(byte - JSON_DIGIT_ZERO);
            value = value
                .checked_mul(JSON_DECIMAL_RADIX)
                .and_then(|current| current.checked_add(digit))
                .ok_or("JSON number overflow")?;
            self.position += 1;
        }
        if saw_digit {
            Ok(JsonValue::Number(value))
        } else {
            Err(format!("expected JSON number at byte {}", self.position))
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        self.expect(b'[')?;
        let mut items = Vec::new();
        self.skip_whitespace();
        if self.consume(b']') {
            return Ok(JsonValue::Array(items));
        }
        loop {
            items.push(self.parse_value()?);
            self.skip_whitespace();
            if self.consume(b']') {
                return Ok(JsonValue::Array(items));
            }
            self.expect(b',')?;
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        self.expect(b'{')?;
        let mut object = BTreeMap::new();
        self.skip_whitespace();
        if self.consume(b'}') {
            return Ok(JsonValue::Object(object));
        }
        loop {
            self.skip_whitespace();
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect(b':')?;
            let value = self.parse_value()?;
            object.insert(key, value);
            self.skip_whitespace();
            if self.consume(b'}') {
                return Ok(JsonValue::Object(object));
            }
            self.expect(b',')?;
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(byte) = self.peek() {
            if matches!(byte, b' ' | b'\n' | b'\r' | b'\t') {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.position).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.position += 1;
        Some(byte)
    }

    fn consume(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, expected: u8) -> Result<(), String> {
        if self.consume(expected) {
            Ok(())
        } else {
            Err(format!("expected JSON byte `{}` at {}", char::from(expected), self.position))
        }
    }
}

fn parse_json(input: &str) -> Result<JsonValue, String> {
    JsonParser::new(input).parse()
}

fn as_object<'a>(value: &'a JsonValue, label: &str) -> Result<&'a BTreeMap<String, JsonValue>, String> {
    match value {
        JsonValue::Object(object) => Ok(object),
        _ => Err(format!("expected {label} to be a JSON object")),
    }
}

fn as_array<'a>(value: &'a JsonValue, label: &str) -> Result<&'a [JsonValue], String> {
    match value {
        JsonValue::Array(items) => Ok(items),
        _ => Err(format!("expected {label} to be a JSON array")),
    }
}

fn object_field<'a>(object: &'a BTreeMap<String, JsonValue>, key: &str) -> Result<&'a JsonValue, String> {
    object.get(key).ok_or_else(|| format!("missing JSON field `{key}`"))
}

fn string_field(object: &BTreeMap<String, JsonValue>, key: &str) -> Result<String, String> {
    match object_field(object, key)? {
        JsonValue::String(value) => Ok(value.clone()),
        _ => Err(format!("expected JSON field `{key}` to be a string")),
    }
}

fn bool_field(object: &BTreeMap<String, JsonValue>, key: &str) -> Result<bool, String> {
    match object_field(object, key)? {
        JsonValue::Bool(value) => Ok(*value),
        _ => Err(format!("expected JSON field `{key}` to be a bool")),
    }
}

fn usize_field(object: &BTreeMap<String, JsonValue>, key: &str) -> Result<usize, String> {
    match object_field(object, key)? {
        JsonValue::Number(value) => usize::try_from(*value).map_err(|_| format!("JSON field `{key}` exceeds usize")),
        _ => Err(format!("expected JSON field `{key}` to be a non-negative number")),
    }
}

fn string_vec_field(object: &BTreeMap<String, JsonValue>, key: &str) -> Result<Vec<String>, String> {
    let array = as_array(object_field(object, key)?, key)?;
    let mut values = Vec::new();
    for item in array {
        match item {
            JsonValue::String(value) => values.push(value.clone()),
            _ => return Err(format!("expected JSON field `{key}` to contain only strings")),
        }
    }
    Ok(values)
}

fn parse_validation_command(value: &JsonValue) -> Result<ValidationCommand, String> {
    let object = as_object(value, "validation command")?;
    Ok(ValidationCommand {
        id: string_field(object, "id")?,
        modes: string_vec_field(object, "modes")?,
        command: string_vec_field(object, "command")?,
        evidence: string_field(object, "evidence")?,
        required: bool_field(object, "required")?,
    })
}

fn parse_apply_recipe(value: &JsonValue) -> Result<ApplyRecipe, String> {
    let object = as_object(value, "apply recipe")?;
    Ok(ApplyRecipe {
        id: string_field(object, "id")?,
        rule_id: string_field(object, "rule_id")?,
        requires_no_trust_markers: bool_field(object, "requires_no_trust_markers")?,
        mutates_source: bool_field(object, "mutates_source")?,
        validation_gate: string_field(object, "validation_gate")?,
    })
}

fn parse_loop_policy(text: &str) -> Result<LoopPolicy, String> {
    let root = parse_json(text)?;
    let root_object = as_object(&root, "policy root")?;
    let loop_object = as_object(object_field(root_object, "loop")?, "loop policy")?;

    let validation_values = as_array(object_field(loop_object, "validation_commands")?, "validation_commands")?;
    let mut validation_commands = Vec::new();
    for value in validation_values {
        validation_commands.push(parse_validation_command(value)?);
    }

    let recipe_values = as_array(object_field(loop_object, "apply_recipes")?, "apply_recipes")?;
    let mut apply_recipes = Vec::new();
    for value in recipe_values {
        apply_recipes.push(parse_apply_recipe(value)?);
    }

    Ok(LoopPolicy {
        default_report: string_field(root_object, "default_report")?,
        default_packet_dir: string_field(root_object, "default_packet_dir")?,
        max_candidates: usize_field(root_object, "max_candidates")?,
        max_packet_findings: usize_field(root_object, "max_packet_findings")?,
        default_max_iterations: usize_field(loop_object, "default_max_iterations")?,
        max_iterations_limit: usize_field(loop_object, "max_iterations_limit")?,
        receipt_dir: string_field(loop_object, "receipt_dir")?,
        no_push_default: bool_field(loop_object, "no_push_default")?,
        slug_prefix: string_field(loop_object, "slug_prefix")?,
        preferred_candidate_rules: string_vec_field(loop_object, "preferred_candidate_rules")?,
        skip_trust_boundary_rules: string_vec_field(loop_object, "skip_trust_boundary_rules")?,
        validation_commands,
        apply_recipes,
    })
}

fn parse_scanner_report(report: &str) -> Vec<CandidateSummary> {
    let mut candidates = Vec::new();
    let mut current: Option<CandidateSummary> = None;
    for line in report.lines() {
        if let Some(candidate) = parse_candidate_heading(line) {
            if let Some(previous) = current.replace(candidate) {
                candidates.push(previous);
            }
            continue;
        }
        if let Some(rule_id) = parse_finding_rule(line) {
            if let Some(candidate) = current.as_mut() {
                candidate.rule_ids.insert(rule_id);
            }
        }
    }
    if let Some(candidate) = current {
        candidates.push(candidate);
    }
    candidates
}

fn parse_candidate_heading(line: &str) -> Option<CandidateSummary> {
    let rest = line.strip_prefix(MARKDOWN_HEADING_PREFIX)?;
    let path_end = rest.find('`')?;
    let path = rest[..path_end].to_string();
    let score_start = rest.find(MARKDOWN_SCORE_MARKER)? + MARKDOWN_SCORE_MARKER.len();
    let score_text = rest[score_start..].split_whitespace().next()?;
    let score = score_text.parse::<usize>().ok()?;
    Some(CandidateSummary {
        path,
        score,
        rule_ids: BTreeSet::new(),
    })
}

fn parse_finding_rule(line: &str) -> Option<String> {
    let rest = line.strip_prefix(FINDING_PREFIX)?;
    let end = rest.find(FINDING_RULE_END)?;
    Some(rest[..end].to_string())
}

fn candidate_has_skip_rule(policy: &LoopPolicy, candidate: &CandidateSummary) -> bool {
    policy.skip_trust_boundary_rules.iter().any(|rule| candidate.rule_ids.contains(rule))
}

fn candidate_preference(policy: &LoopPolicy, candidate: &CandidateSummary) -> usize {
    policy
        .preferred_candidate_rules
        .iter()
        .position(|rule| candidate.rule_ids.contains(rule))
        .unwrap_or(LOWEST_PREFERENCE)
}

fn select_candidate(policy: &LoopPolicy, candidates: &[CandidateSummary]) -> Option<CandidateSummary> {
    let mut eligible: Vec<&CandidateSummary> =
        candidates.iter().filter(|candidate| !candidate_has_skip_rule(policy, candidate)).collect();
    eligible.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| candidate_preference(policy, left).cmp(&candidate_preference(policy, right)))
            .then_with(|| left.path.cmp(&right.path))
    });
    eligible.into_iter().next().cloned()
}

fn apply_recipe_for(policy: &LoopPolicy, candidate: &CandidateSummary) -> Option<ApplyRecipe> {
    for recipe in &policy.apply_recipes {
        if candidate.rule_ids.contains(&recipe.rule_id)
            && (!recipe.requires_no_trust_markers || !candidate_has_skip_rule(policy, candidate))
        {
            return Some(recipe.clone());
        }
    }
    None
}

fn validation_plan(policy: &LoopPolicy, mode: LoopMode) -> Vec<ValidationCommand> {
    let mode_name = mode.as_str();
    policy
        .validation_commands
        .iter()
        .filter(|command| command.modes.iter().any(|candidate_mode| candidate_mode == mode_name))
        .cloned()
        .collect()
}

fn slug_for_candidate(prefix: &str, path: &str) -> String {
    let stem = path.strip_prefix(SRC_PREFIX).unwrap_or(path).strip_suffix(RUST_SUFFIX).unwrap_or(path);
    let mut slug = String::new();
    slug.push_str(prefix);
    slug.push('-');
    let mut previous_dash = false;
    for ch in stem.chars() {
        let next = if ch.is_ascii_alphanumeric() {
            previous_dash = false;
            Some(ch.to_ascii_lowercase())
        } else if previous_dash {
            None
        } else {
            previous_dash = true;
            Some('-')
        };
        if let Some(next) = next {
            slug.push(next);
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    slug
}

fn build_plan(
    policy: &LoopPolicy,
    mode: LoopMode,
    push_requested: bool,
    max_iterations: usize,
    dirty_status: &str,
    active_changes: &[String],
    candidates: &[CandidateSummary],
) -> LoopPlan {
    if max_iterations == 0 || max_iterations > policy.max_iterations_limit {
        return LoopPlan::Stop(StopPlan {
            reason: STOP_INVALID_BUDGET.to_string(),
            detail: format!(
                "requested {max_iterations} iteration(s), but the policy limit is {}",
                policy.max_iterations_limit
            ),
        });
    }
    if !dirty_status.trim().is_empty() {
        return LoopPlan::Stop(StopPlan {
            reason: STOP_DIRTY_TREE.to_string(),
            detail: dirty_status.trim().to_string(),
        });
    }
    if !active_changes.is_empty() {
        return LoopPlan::Stop(StopPlan {
            reason: STOP_ACTIVE_CHANGE.to_string(),
            detail: active_changes.join(", "),
        });
    }
    if push_requested && policy.no_push_default {
        return LoopPlan::Stop(StopPlan {
            reason: STOP_PUSH_DISABLED.to_string(),
            detail: "push was requested, but loop policy disables pushing".to_string(),
        });
    }
    let Some(candidate) = select_candidate(policy, candidates) else {
        let reason = if candidates.is_empty() {
            STOP_NO_CANDIDATE
        } else {
            STOP_TRUST_BOUNDARY_ONLY
        };
        return LoopPlan::Stop(StopPlan {
            reason: reason.to_string(),
            detail: "no non-trust-boundary candidate is eligible".to_string(),
        });
    };
    let apply_recipe = if mode == LoopMode::Apply {
        let Some(recipe) = apply_recipe_for(policy, &candidate) else {
            return LoopPlan::Stop(StopPlan {
                reason: STOP_UNSUPPORTED_APPLY.to_string(),
                detail: format!("{} has no deterministic apply recipe", candidate.path),
            });
        };
        Some(recipe)
    } else {
        None
    };
    LoopPlan::Ready(ReadyPlan {
        change_slug: slug_for_candidate(&policy.slug_prefix, &candidate.path),
        validation: validation_plan(policy, mode),
        mode,
        candidate,
        apply_recipe,
    })
}

fn packet_file_name(candidate_path: &str) -> String {
    let mut file = String::new();
    for ch in candidate_path.chars() {
        if ch.is_ascii_alphanumeric() {
            file.push(ch);
        } else {
            file.push('_');
        }
    }
    file.push_str(PACKET_FILE_EXTENSION);
    file
}

fn json_escape(input: &str) -> String {
    let mut out = String::new();
    for ch in input.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out
}

fn render_json_string_array(values: &[String]) -> String {
    let mut out = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        out.push('"');
        out.push_str(&json_escape(value));
        out.push('"');
    }
    out.push(']');
    out
}

fn render_validation_receipts(commands: &[ValidationCommand]) -> String {
    let mut out = String::from("[");
    for (index, command) in commands.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!(
            "{{\"id\": \"{}\", \"evidence\": \"{}\", \"required\": {}}}",
            json_escape(&command.id),
            json_escape(&command.evidence),
            command.required
        ));
    }
    out.push(']');
    out
}

fn render_receipt(plan: &LoopPlan) -> String {
    match plan {
        LoopPlan::Stop(stop) => format!(
            "{{\n  \"status\": \"stopped\",\n  \"reason\": \"{}\",\n  \"detail\": \"{}\"\n}}\n",
            json_escape(&stop.reason),
            json_escape(&stop.detail)
        ),
        LoopPlan::Ready(ready) => {
            let rule_ids: Vec<String> = ready.candidate.rule_ids.iter().cloned().collect();
            let recipe = ready
                .apply_recipe
                .as_ref()
                .map(|recipe| format!("\"{}\"", json_escape(&recipe.id)))
                .unwrap_or_else(|| "null".to_string());
            format!(
                "{{\n  \"status\": \"planned\",\n  \"mode\": \"{}\",\n  \"candidate\": {{\n    \"path\": \"{}\",\n    \"score\": {},\n    \"rules\": {}\n  }},\n  \"change_slug\": \"{}\",\n  \"apply_recipe\": {},\n  \"validation\": {}\n}}\n",
                ready.mode.as_str(),
                json_escape(&ready.candidate.path),
                ready.candidate.score,
                render_json_string_array(&rule_ids),
                json_escape(&ready.change_slug),
                recipe,
                render_validation_receipts(&ready.validation)
            )
        }
    }
}

fn repo_root() -> PathBuf {
    match Command::new("git").args(["rev-parse", "--show-toplevel"]).output() {
        Ok(output) if output.status.success() => PathBuf::from(String::from_utf8_lossy(&output.stdout).trim()),
        _ => std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    }
}

fn parse_usize(input: &str, label: &str) -> Result<usize, String> {
    input.parse::<usize>().map_err(|err| format!("invalid {label} `{input}`: {err}"))
}

fn print_usage() {
    println!(
        "usage: proof-review-loop.rs [--root PATH] [--mode dry-run|assisted|apply] [--max-iterations N] [--push] [--self-test]"
    );
}

fn parse_args() -> Result<Args, String> {
    let mut root = repo_root();
    let mut mode = LoopMode::DryRun;
    let mut max_iterations = 0usize;
    let mut push_requested = false;
    let mut self_test = false;
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut index = 0usize;
    while index < args.len() {
        match args[index].as_str() {
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            "--self-test" => {
                self_test = true;
                index += 1;
            }
            "--push" => {
                push_requested = true;
                index += 1;
            }
            "--root" => {
                let value_index = index + 1;
                let Some(value) = args.get(value_index) else {
                    return Err("--root requires a path".to_string());
                };
                root = PathBuf::from(value);
                index += ARG_WITH_VALUE_WIDTH;
            }
            "--mode" => {
                let value_index = index + 1;
                let Some(value) = args.get(value_index) else {
                    return Err("--mode requires a mode".to_string());
                };
                mode = LoopMode::parse(value)?;
                index += ARG_WITH_VALUE_WIDTH;
            }
            "--max-iterations" => {
                let value_index = index + 1;
                let Some(value) = args.get(value_index) else {
                    return Err("--max-iterations requires a number".to_string());
                };
                max_iterations = parse_usize(value, "--max-iterations")?;
                index += ARG_WITH_VALUE_WIDTH;
            }
            other => return Err(format!("unknown argument `{other}`")),
        }
    }
    Ok(Args {
        root,
        mode,
        max_iterations,
        push_requested,
        self_test,
    })
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

fn require_cairn_valid(output: Output, label: &str) -> Result<Output, String> {
    let output = require_success(output, label)?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.contains("\"valid\": false") || stdout.contains("\"verdict\": \"FAIL\"") {
        Err(format!(
            "{label} reported invalid Cairn output\nstdout:\n{}\nstderr:\n{}",
            stdout,
            String::from_utf8_lossy(&output.stderr)
        ))
    } else {
        Ok(output)
    }
}

fn git_status(root: &Path) -> Result<String, String> {
    let mut command = Command::new("git");
    command.arg("-C").arg(root).args(GIT_STATUS_ARGS);
    let output = require_success(command_output(command, "git status")?, "git status")?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn active_cairn_changes(root: &Path) -> Result<Vec<String>, String> {
    let changes = root.join(CAIRN_CHANGES_DIR);
    if !changes.exists() {
        return Ok(Vec::new());
    }
    let mut names = Vec::new();
    for entry in fs::read_dir(&changes).map_err(|err| format!("failed to read {}: {err}", changes.display()))? {
        let entry = entry.map_err(|err| format!("failed to read Cairn change entry: {err}"))?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if name != ARCHIVE_DIR_NAME {
            names.push(name.to_string());
        }
    }
    names.sort();
    Ok(names)
}

fn read_policy(root: &Path) -> Result<LoopPolicy, String> {
    let path = root.join(POLICY_JSON);
    let text = fs::read_to_string(&path).map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    parse_loop_policy(&text)
}

fn scanner_report_path(root: &Path, policy: &LoopPolicy) -> PathBuf {
    root.join(&policy.receipt_dir).join(SCANNER_REPORT_NAME)
}

fn packet_dir_path(root: &Path, policy: &LoopPolicy) -> PathBuf {
    root.join(&policy.default_packet_dir)
}

fn receipt_path(root: &Path, policy: &LoopPolicy) -> PathBuf {
    root.join(&policy.receipt_dir).join(RECEIPT_FILE_NAME)
}

fn run_scanner(root: &Path, policy: &LoopPolicy) -> Result<String, String> {
    let report = scanner_report_path(root, policy);
    let packets = packet_dir_path(root, policy);
    if let Some(parent) = report.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::create_dir_all(&packets).map_err(|err| format!("failed to create {}: {err}", packets.display()))?;
    let mut command = Command::new(root.join(SCANNER_SCRIPT));
    command
        .arg("--root")
        .arg(root)
        .arg("--check-policy")
        .arg("--max")
        .arg(policy.max_candidates.to_string())
        .arg("--write-report")
        .arg(&report)
        .arg("--write-packets")
        .arg(&packets);
    if std::env::var_os(CARGO_TARGET_ENV).is_none() {
        command.env(CARGO_TARGET_ENV, root.join(TARGET_CARGO_DIR));
    }
    let output = require_success(command_output(command, "proof-review scanner")?, "proof-review scanner")?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn write_receipt(root: &Path, policy: &LoopPolicy, receipt: &str) -> Result<PathBuf, String> {
    let path = receipt_path(root, policy);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::write(&path, receipt).map_err(|err| format!("failed to write {}: {err}", path.display()))?;
    Ok(path)
}

fn cairn_command() -> Command {
    if let Some(bin) = std::env::var_os(CAIRN_BIN_ENV) {
        return Command::new(bin);
    }
    if Path::new(LOCAL_CAIRN_FLAKE).exists() {
        let mut command = Command::new(NIX_BIN);
        command.arg(NIX_RUN_ARG).arg(LOCAL_CAIRN_APP).arg(NIX_SEPARATOR_ARG);
        return command;
    }
    Command::new(DIRECT_CAIRN_BIN)
}

fn create_assisted_change(root: &Path, policy: &LoopPolicy, ready: &ReadyPlan) -> Result<(), String> {
    let change_dir = root.join(CAIRN_CHANGES_DIR).join(&ready.change_slug);
    if change_dir.exists() {
        return Err(format!("Cairn change `{}` already exists", ready.change_slug));
    }
    let mut create = cairn_command();
    create.arg("change").arg("create").arg(&ready.change_slug).arg("--root").arg(root);
    require_success(command_output(create, "cairn change create")?, "cairn change create")?;

    let evidence_dir = change_dir.join("evidence");
    let spec_dir = change_dir.join("specs").join(PROOF_REVIEW_SPEC_ID);
    fs::create_dir_all(&evidence_dir).map_err(|err| format!("failed to create {}: {err}", evidence_dir.display()))?;
    fs::create_dir_all(&spec_dir).map_err(|err| format!("failed to create {}: {err}", spec_dir.display()))?;

    let requirement_id = format!("proof-review.iteration.{}", ready.change_slug.replace('-', "."));
    fs::write(change_dir.join("proposal.md"), render_assisted_proposal(ready))
        .map_err(|err| format!("failed to write assisted proposal: {err}"))?;
    fs::write(change_dir.join("design.md"), render_assisted_design(ready))
        .map_err(|err| format!("failed to write assisted design: {err}"))?;
    fs::write(change_dir.join("tasks.md"), render_assisted_tasks(&requirement_id, ready))
        .map_err(|err| format!("failed to write assisted tasks: {err}"))?;
    fs::write(spec_dir.join("spec.md"), render_assisted_spec(&requirement_id, ready))
        .map_err(|err| format!("failed to write assisted spec: {err}"))?;

    let report = scanner_report_path(root, policy);
    if report.exists() {
        fs::copy(&report, evidence_dir.join("proof-review-candidates.md"))
            .map_err(|err| format!("failed to copy scanner evidence: {err}"))?;
    }
    let packet_path = packet_dir_path(root, policy).join(packet_file_name(&ready.candidate.path));
    fs::write(evidence_dir.join("vibethinker-packet-path.txt"), format!("{}\n", packet_path.display()))
        .map_err(|err| format!("failed to write packet path evidence: {err}"))?;

    let mut gates = String::new();
    for args in [
        vec!["validate", "--root", "."],
        vec!["gate", "proposal", ready.change_slug.as_str(), "--root", "."],
        vec!["gate", "design", ready.change_slug.as_str(), "--root", "."],
        vec!["gate", "tasks", ready.change_slug.as_str(), "--root", "."],
    ] {
        let mut command = cairn_command();
        command.current_dir(root).args(&args);
        let output = require_cairn_valid(command_output(command, "cairn gate")?, "cairn gate")?;
        gates.push_str("$ cairn ");
        gates.push_str(&args.join(" "));
        gates.push('\n');
        gates.push_str(&String::from_utf8_lossy(&output.stdout));
        gates.push_str(&String::from_utf8_lossy(&output.stderr));
        gates.push('\n');
    }
    fs::write(evidence_dir.join("cairn-gates.log"), gates)
        .map_err(|err| format!("failed to write Cairn gate evidence: {err}"))?;
    Ok(())
}

fn render_assisted_proposal(ready: &ReadyPlan) -> String {
    format!(
        "## Why\n\nThe proof-review scanner selected `{}` as the next bounded proof-hardening target. Capturing a dedicated Cairn package keeps the advisory packet, source patch, validation logs, and archive evidence reviewable.\n\n## What Changes\n\n- Review the scanner findings for `{}`.\n- Apply only proof/test hardening that does not add or hide trust boundaries.\n- Capture focused validation, Tracey coverage, Cairn gates, and repo verification evidence before archive.\n\n## Impact\n\n- **Files**: expected source/test edits for `{}`, this Cairn package, and archived evidence.\n- **Testing**: focused tests where available, Verus over `src/lib.rs`, Tracey uncovered/untested, Cairn validation/gates, and `TMPDIR=/tmp ./scripts/verify.rs`.\n",
        ready.candidate.path, ready.candidate.path, ready.candidate.path
    )
}

fn render_assisted_design(ready: &ReadyPlan) -> String {
    format!(
        "## Context\n\nThe loop selected `{}` with score `{}` and findings `{}`. VibeThinker packet output is advisory-only; repository checks remain authoritative.\n\n## Decisions\n\n### 1. Keep the iteration bounded\n\n**Choice:** Handle one proof-review target in this package.\n\n**Rationale:** One target keeps source diffs, evidence, and archive review small.\n\n### 2. Do not expand trust\n\n**Choice:** Reject any patch that adds `external_body`, `assume_specification`, `assume(`, or `@verified-logic:no-ensures`.\n\n**Rationale:** Proof-review automation exists to harden coverage, not to hide gaps.\n\n### 3. Validation decides completion\n\n**Choice:** Archive only after focused checks, Verus, Tracey, Cairn gates, and the repo verification wrapper succeed.\n\n**Rationale:** Advisory model critique can guide review, but cannot prove correctness.\n",
        ready.candidate.path,
        ready.candidate.score,
        ready.candidate.rule_ids.iter().cloned().collect::<Vec<_>>().join(", ")
    )
}

fn render_assisted_tasks(requirement_id: &str, ready: &ReadyPlan) -> String {
    format!(
        "## Phase 1: Proof-review hardening\n\n- [ ] [serial] Inspect scanner evidence and VibeThinker packet for `{}`. r[{}]\n- [ ] [serial] Apply proof/test hardening without adding trust boundaries. r[{}]\n- [ ] [serial] Add positive and negative tests or explain why the target is proof-only. r[{}]\n\n## Phase 2: Validation and archive\n\n- [ ] [serial] Run focused tests, Verus over `src/lib.rs`, Tracey uncovered/untested, Cairn gates, and `TMPDIR=/tmp ./scripts/verify.rs`; save evidence logs. r[{}]\n- [ ] [serial] Sync/archive the change and commit completed evidence. r[{}]\n",
        ready.candidate.path, requirement_id, requirement_id, requirement_id, requirement_id, requirement_id
    )
}

fn render_assisted_spec(requirement_id: &str, ready: &ReadyPlan) -> String {
    format!(
        "## ADDED Requirements\n\n### Requirement: Bounded Proof Review Iteration for {}\n\nThe proof-review iteration for `{}` MUST strengthen the selected target without adding trust boundaries, MUST include positive and negative executable tests when the target has runtime behavior, and MUST archive scanner, advisory packet, validation, sync, archive, and final status evidence before claiming completion.\n\nr[{}]\n\n#### Scenario: Iteration archives only after validation\n\n- GIVEN `{}` has been selected by the proof-review loop\n- WHEN the change is prepared for archive\n- THEN focused checks, Verus, Tracey, Cairn gates, and repo verification evidence MUST be present and successful\n",
        ready.candidate.path, ready.candidate.path, requirement_id, ready.candidate.path
    )
}

fn run_loop(args: Args) -> Result<(), String> {
    let root = args.root.canonicalize().unwrap_or_else(|_| args.root.clone());
    let policy = read_policy(&root)?;
    let max_iterations = if args.max_iterations == 0 {
        policy.default_max_iterations
    } else {
        args.max_iterations
    };

    let dirty_status = git_status(&root)?;
    let active_changes = active_cairn_changes(&root)?;
    let pre_plan =
        build_plan(&policy, args.mode, args.push_requested, max_iterations, &dirty_status, &active_changes, &[]);
    if matches!(
        pre_plan,
        LoopPlan::Stop(StopPlan { ref reason, .. })
            if reason == STOP_DIRTY_TREE || reason == STOP_ACTIVE_CHANGE || reason == STOP_PUSH_DISABLED || reason == STOP_INVALID_BUDGET
    ) {
        let receipt = render_receipt(&pre_plan);
        let path = write_receipt(&root, &policy, &receipt)?;
        print!("{receipt}");
        println!("wrote receipt: {}", path.display());
        return Ok(());
    }

    let scanner_stdout = run_scanner(&root, &policy)?;
    let report = fs::read_to_string(scanner_report_path(&root, &policy))
        .map_err(|err| format!("failed to read scanner report: {err}"))?;
    let candidates = parse_scanner_report(&report);
    let plan = build_plan(
        &policy,
        args.mode,
        args.push_requested,
        max_iterations,
        &dirty_status,
        &active_changes,
        &candidates,
    );

    if let LoopPlan::Ready(ready) = &plan {
        if ready.mode == LoopMode::Assisted || ready.mode == LoopMode::Apply {
            create_assisted_change(&root, &policy, ready)?;
        }
    }

    let receipt = render_receipt(&plan);
    let path = write_receipt(&root, &policy, &receipt)?;
    print!("{scanner_stdout}");
    print!("{receipt}");
    println!("wrote receipt: {}", path.display());
    Ok(())
}

fn sample_policy_json() -> &'static str {
    r#"{
      "default_report": "target/proof-review/candidates.md",
      "default_packet_dir": "target/proof-review/packets",
      "max_candidates": 12,
      "max_packet_findings": 6,
      "loop": {
        "default_max_iterations": 1,
        "max_iterations_limit": 3,
        "receipt_dir": "target/proof-review/loop-receipts",
        "no_push_default": true,
        "slug_prefix": "proof-review",
        "preferred_candidate_rules": ["vacuous-ensures-true", "empty-nontrivial-proof"],
        "skip_trust_boundary_rules": ["untrusted-boundary-marker", "permanent-boundary-marker"],
        "validation_commands": [
          {"id": "scanner", "modes": ["dry-run", "assisted", "apply"], "command": ["./scripts/proof-review-candidates.rs"], "evidence": "scanner.log", "required": true},
          {"id": "verify-wrapper", "modes": ["apply"], "command": ["./scripts/verify.rs"], "evidence": "verify.log", "required": true}
        ],
        "apply_recipes": [
          {"id": "vacuous-coverage-proof", "rule_id": "vacuous-ensures-true", "requires_no_trust_markers": true, "mutates_source": true, "validation_gate": "verify-wrapper"}
        ]
      }
    }"#
}

fn sample_report() -> &'static str {
    "# Proof Review Candidates\n\n### `src/trusted.rs` — score 900\n\n- **untrusted-boundary-marker** (+100): `src/trusted.rs`:1 — found trust.\n\n### `src/vacuous.rs` — score 80\n\n- **vacuous-ensures-true** (+80): `src/vacuous.rs`:4 — proof has ensures true.\n\n### `src/empty.rs` — score 35\n\n- **empty-nontrivial-proof** (+35): `src/empty.rs`:8 — empty proof.\n"
}

fn require(condition: bool, message: &str) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message.to_string()) }
}

fn run_self_test() -> Result<(), String> {
    let policy = parse_loop_policy(sample_policy_json())?;
    require(policy.max_candidates == SELF_TEST_MAX_CANDIDATES, "policy parser missed max_candidates")?;
    require(
        policy.validation_commands.len() == SELF_TEST_VALIDATION_COMMAND_COUNT,
        "policy parser missed validation commands",
    )?;
    require(policy.apply_recipes.len() == 1, "policy parser missed apply recipes")?;

    let missing_loop =
        "{\"default_report\": \"x\", \"default_packet_dir\": \"y\", \"max_candidates\": 1, \"max_packet_findings\": 1}";
    require(parse_loop_policy(missing_loop).is_err(), "missing loop policy should fail")?;

    let candidates = parse_scanner_report(sample_report());
    require(candidates.len() == SELF_TEST_CANDIDATE_COUNT, "scanner report parser missed candidates")?;
    let selected = select_candidate(&policy, &candidates).ok_or("candidate selection returned none")?;
    require(
        selected.path == "src/vacuous.rs",
        "selection should skip trust boundaries and prefer vacuous target",
    )?;

    let dry_validation = validation_plan(&policy, LoopMode::DryRun);
    require(
        dry_validation.len() == SELF_TEST_DRY_RUN_COMMAND_COUNT,
        "dry-run validation plan should include scanner only",
    )?;
    let apply_validation = validation_plan(&policy, LoopMode::Apply);
    require(
        apply_validation.len() == SELF_TEST_APPLY_COMMAND_COUNT,
        "apply validation plan should include scanner and verify",
    )?;

    let dirty = build_plan(&policy, LoopMode::DryRun, false, 1, " M src/lib.rs", &[], &candidates);
    require(
        matches!(dirty, LoopPlan::Stop(StopPlan { reason, .. }) if reason == STOP_DIRTY_TREE),
        "dirty tree should stop",
    )?;

    let active = build_plan(&policy, LoopMode::DryRun, false, 1, "", &["demo".to_string()], &candidates);
    require(
        matches!(active, LoopPlan::Stop(StopPlan { reason, .. }) if reason == STOP_ACTIVE_CHANGE),
        "active change should stop",
    )?;

    let push = build_plan(&policy, LoopMode::DryRun, true, 1, "", &[], &candidates);
    require(
        matches!(push, LoopPlan::Stop(StopPlan { reason, .. }) if reason == STOP_PUSH_DISABLED),
        "push request should stop",
    )?;

    let trust_only_report = "### `src/trusted.rs` — score 100\n\n- **untrusted-boundary-marker** (+100): trust\n";
    let trust_only = parse_scanner_report(trust_only_report);
    let trust_plan = build_plan(&policy, LoopMode::DryRun, false, 1, "", &[], &trust_only);
    require(
        matches!(trust_plan, LoopPlan::Stop(StopPlan { reason, .. }) if reason == STOP_TRUST_BOUNDARY_ONLY),
        "trust-only candidates should stop",
    )?;

    let unsupported_report = "### `src/empty.rs` — score 35\n\n- **empty-nontrivial-proof** (+35): empty\n";
    let unsupported = parse_scanner_report(unsupported_report);
    let unsupported_plan = build_plan(&policy, LoopMode::Apply, false, 1, "", &[], &unsupported);
    require(
        matches!(unsupported_plan, LoopPlan::Stop(StopPlan { reason, .. }) if reason == STOP_UNSUPPORTED_APPLY),
        "unsupported apply pattern should stop",
    )?;

    let ready = build_plan(&policy, LoopMode::Apply, false, 1, "", &[], &candidates);
    let LoopPlan::Ready(ready) = ready else {
        return Err("vacuous apply candidate should be ready".to_string());
    };
    require(
        ready.apply_recipe.as_ref().map(|recipe| recipe.id.as_str()) == Some("vacuous-coverage-proof"),
        "apply recipe id mismatch",
    )?;
    require(ready.change_slug == "proof-review-vacuous", "slug generation mismatch")?;
    require(packet_file_name("src/vacuous.rs") == "src_vacuous_rs.md", "packet filename mismatch")?;

    let receipt = render_receipt(&LoopPlan::Ready(ready));
    require(receipt.contains("\"status\": \"planned\""), "ready receipt missing status")?;
    require(receipt.contains("vacuous-coverage-proof"), "ready receipt missing recipe")?;
    let stop_receipt = render_receipt(&LoopPlan::Stop(StopPlan {
        reason: STOP_NO_CANDIDATE.to_string(),
        detail: "none".to_string(),
    }));
    require(stop_receipt.contains(STOP_NO_CANDIDATE), "stop receipt missing reason")?;

    println!("proof-review-loop self-test: PASS");
    Ok(())
}

fn main() -> Result<(), String> {
    let args = parse_args()?;
    if args.self_test {
        return run_self_test();
    }
    run_loop(args)
}

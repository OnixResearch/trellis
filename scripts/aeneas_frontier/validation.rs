//! Pure frontier and registry construction and validation.
//!
//! This module performs no filesystem, clock, or process access. The shell
//! supplies the module inventory, the source text for hypotheses, the recorded
//! observations, the requirement IDs, and the selected targets.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::model;

pub(crate) const NON_TARGET_REASON: &str = "not-selected-in-batch";

/// Affirmative phrases that would overstate a frontier or batch result.
pub(crate) const CLAIM_PATTERNS: [&str; 9] = [
    "proves whole-library",
    "whole-library proof coverage",
    "all modules are proved",
    "semantically equivalent",
    "proves verifier independence",
    "proves verifier soundness",
    "proves progress",
    "proves trust-boundary closure",
    "closes the trust boundaries",
];

/// Tokens that mark a sentence as a non-claim.
const NEGATION_TOKENS: [&str; 9] = [
    "does not",
    "do not",
    "doesn't",
    "don't",
    "never",
    "non-claim",
    "not prove",
    "not claim",
    "no claim",
];

const EXTERNAL_CRATE_TOKENS: [&str; 7] = ["blake3", "ed25519", "redb", "regex", "serde_json", "bs58", "anyhow"];

pub(crate) fn hypothesis_for_source(contents: &str) -> &'static str {
    if contents.contains("#[verifier::external_body]") {
        return model::CLASS_TIER_4;
    }
    if EXTERNAL_CRATE_TOKENS.iter().any(|token| contents.contains(token)) {
        return model::CLASS_TIER_3;
    }
    if contents.contains("Vec<") || contents.contains("&mut ") {
        return model::CLASS_TIER_2;
    }
    model::CLASS_TIER_1
}

pub(crate) fn is_known_class(value: &str) -> bool {
    model::TIER_CLASSES.contains(&value)
        || value == model::CLASS_EXCLUDED
        || value == model::CLASS_UNMEASURED
        || value.starts_with(model::CLASS_BLOCKED_PREFIX)
}

pub(crate) fn is_blake3(value: &str) -> bool {
    let Some(digest) = value.strip_prefix("b3:") else {
        return false;
    };
    digest.len() == 64
        && digest.chars().all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase())
}

pub(crate) fn is_module_path(value: &str) -> bool {
    if !value.starts_with("src/") {
        return false;
    }
    let relative = std::path::Path::new(value);
    relative.extension().is_some_and(|extension| extension == "rs")
        && !relative.is_absolute()
        && relative.components().all(|component| matches!(component, std::path::Component::Normal(_)))
}

pub(crate) fn is_relative_path(value: &str) -> bool {
    let relative = std::path::Path::new(value);
    !value.is_empty()
        && !relative.is_absolute()
        && relative.components().all(|component| matches!(component, std::path::Component::Normal(_)))
}

fn is_stage(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-')
}

fn validate_provider(provider: &model::ProviderBinding, diagnostics: &mut Vec<String>) {
    if provider.package != "octet-aeneas-lean-provider-closure" {
        diagnostics.push(format!("provider package `{}` is not the reviewed provider", provider.package));
    }
    if provider.revision.len() != 40 || !provider.revision.chars().all(|character| character.is_ascii_hexdigit()) {
        diagnostics.push(format!("provider revision `{}` is not a 40-character revision", provider.revision));
    }
    if !is_blake3(&provider.profile_identity) {
        diagnostics.push("provider profile identity is not a BLAKE3 identity".to_owned());
    }
    if !is_blake3(&provider.compatibility_matrix_identity) {
        diagnostics.push("provider compatibility matrix identity is not a BLAKE3 identity".to_owned());
    }
}

fn validate_observation(observation: &model::Observation, seen: &mut BTreeSet<String>, diagnostics: &mut Vec<String>) {
    if !is_module_path(&observation.module) {
        diagnostics.push(format!("observation module `{}` is not a module path", observation.module));
    }
    if !seen.insert(observation.module.clone()) {
        diagnostics.push(format!("duplicate observation for module `{}`", observation.module));
    }
    if !is_stage(&observation.stage) {
        diagnostics.push(format!("observation stage `{}` is not a stage name", observation.stage));
    }
    let has_construct = observation.construct.as_deref().is_some_and(|value| !value.is_empty());
    let has_diagnostic = observation.diagnostic_class.as_deref().is_some_and(|value| !value.is_empty());
    match observation.outcome.as_str() {
        model::OUTCOME_SUPPORTED => {
            if !model::TIER_CLASSES.contains(&observation.measured_class.as_str()) {
                diagnostics.push(format!(
                    "supported observation for `{}` must carry a tier class, found `{}`",
                    observation.module, observation.measured_class
                ));
            }
            if has_construct || has_diagnostic {
                diagnostics.push(format!(
                    "supported observation for `{}` must not record a construct or diagnostic class",
                    observation.module
                ));
            }
        }
        model::OUTCOME_BLOCKED => {
            if observation.measured_class == model::CLASS_EXCLUDED {
                if !has_construct {
                    diagnostics.push(format!(
                        "excluded observation for `{}` must name the unsupported construct",
                        observation.module
                    ));
                }
            } else if observation.measured_class == format!("{}{}", model::CLASS_BLOCKED_PREFIX, observation.stage) {
                if !has_diagnostic {
                    diagnostics.push(format!(
                        "blocked observation for `{}` must record a diagnostic class",
                        observation.module
                    ));
                }
            } else {
                diagnostics.push(format!(
                    "blocked observation for `{}` must use `{}{}` or `{}`, found `{}`",
                    observation.module,
                    model::CLASS_BLOCKED_PREFIX,
                    observation.stage,
                    model::CLASS_EXCLUDED,
                    observation.measured_class
                ));
            }
        }
        model::OUTCOME_FAILED => {
            if observation.measured_class != model::CLASS_UNMEASURED {
                diagnostics.push(format!(
                    "failed observation for `{}` must stay unmeasured, found `{}`",
                    observation.module, observation.measured_class
                ));
            }
            if !has_diagnostic {
                diagnostics
                    .push(format!("failed observation for `{}` must record a diagnostic class", observation.module));
            } else if !model::DIAGNOSTIC_CLASSES.contains(&observation.diagnostic_class.as_deref().unwrap_or_default())
            {
                diagnostics.push(format!(
                    "failed observation for `{}` uses unknown diagnostic class `{}`",
                    observation.module,
                    observation.diagnostic_class.as_deref().unwrap_or_default()
                ));
            }
            if has_construct {
                diagnostics.push(format!(
                    "failed observation for `{}` must not claim an unsupported construct",
                    observation.module
                ));
            }
        }
        other => diagnostics.push(format!("observation outcome `{other}` is not a known outcome")),
    }
    if observation.evidence.as_deref().is_some_and(str::is_empty) {
        diagnostics.push(format!("observation evidence for `{}` is empty", observation.module));
    }
    if observation.measured_class != model::CLASS_UNMEASURED && !observation.reviewed {
        diagnostics.push(format!("measured observation for `{}` is not marked reviewed", observation.module));
    }
}

pub(crate) fn validate_claim_text(label: &str, text: &str) -> Vec<String> {
    let mut diagnostics = Vec::new();
    let mut paragraph = String::new();
    let flush = |paragraph: &mut String, diagnostics: &mut Vec<String>| {
        if paragraph.is_empty() {
            return;
        }
        for sentence in paragraph.split(['.', '!', '?']) {
            let lowered = sentence.to_ascii_lowercase();
            if !CLAIM_PATTERNS.iter().any(|pattern| lowered.contains(pattern)) {
                continue;
            }
            if NEGATION_TOKENS.iter().any(|token| lowered.contains(token)) {
                continue;
            }
            let excerpt = sentence.split_whitespace().collect::<Vec<_>>().join(" ");
            diagnostics.push(format!("{label} carries an overclaim: {excerpt}"));
        }
        paragraph.clear();
    };
    for line in text.lines() {
        if line.trim().is_empty() {
            flush(&mut paragraph, &mut diagnostics);
        } else {
            paragraph.push(' ');
            paragraph.push_str(line);
        }
    }
    flush(&mut paragraph, &mut diagnostics);
    diagnostics
}

pub(crate) fn validate_observations(file: &model::ObservationsFile) -> Vec<String> {
    let mut diagnostics = Vec::new();
    if file.schema_version != model::OBSERVATIONS_SCHEMA {
        diagnostics.push(format!(
            "observations schema `{}` does not match `{}`",
            file.schema_version,
            model::OBSERVATIONS_SCHEMA
        ));
    }
    validate_provider(&file.provider, &mut diagnostics);
    let mut seen = BTreeSet::new();
    for observation in &file.observations {
        validate_observation(observation, &mut seen, &mut diagnostics);
    }
    diagnostics
}

pub(crate) fn build_rows(
    inventory: &[String],
    hypotheses: &BTreeMap<String, String>,
    observations: &model::ObservationsFile,
) -> Vec<model::ModuleRow> {
    let mut by_module = BTreeMap::new();
    for observation in &observations.observations {
        by_module.insert(observation.module.clone(), observation.clone());
    }
    inventory
        .iter()
        .map(|module| {
            let observation = by_module.get(module).cloned();
            let measured_class = observation
                .as_ref()
                .map_or_else(|| model::CLASS_UNMEASURED.to_owned(), |value| value.measured_class.clone());
            model::ModuleRow {
                module: module.clone(),
                hypothesis: hypotheses.get(module).cloned().unwrap_or_else(|| model::CLASS_UNMEASURED.to_owned()),
                measured_class,
                observation,
            }
        })
        .collect()
}

pub(crate) fn count_rows(rows: &[model::ModuleRow]) -> model::Counts {
    let mut counts = model::Counts::default();
    for row in rows {
        counts.total_modules += 1;
        match row.measured_class.as_str() {
            model::CLASS_TIER_1 | model::CLASS_TIER_2 | model::CLASS_TIER_3 | model::CLASS_TIER_4 => {
                counts.measured += 1;
            }
            model::CLASS_EXCLUDED => counts.excluded += 1,
            model::CLASS_UNMEASURED => counts.unmeasured += 1,
            value if value.starts_with(model::CLASS_BLOCKED_PREFIX) => counts.blocked += 1,
            _ => {}
        }
        match row.hypothesis.as_str() {
            model::CLASS_TIER_1 => counts.hypothesis_tier_1 += 1,
            model::CLASS_TIER_2 => counts.hypothesis_tier_2 += 1,
            model::CLASS_TIER_3 => counts.hypothesis_tier_3 += 1,
            model::CLASS_TIER_4 => counts.hypothesis_tier_4 += 1,
            _ => {}
        }
    }
    counts
}

pub(crate) fn build_frontier(
    provider: &model::ProviderBinding,
    rows: Vec<model::ModuleRow>,
) -> model::FrontierArtifact {
    let counts = count_rows(&rows);
    model::FrontierArtifact {
        schema_version: model::FRONTIER_SCHEMA.to_owned(),
        bounds: model::bounds(),
        provider: provider.clone(),
        modules: rows,
        counts,
        non_claims: model::non_claims(),
    }
}

pub(crate) fn validate_frontier(
    artifact: &model::FrontierArtifact,
    inventory: &[String],
    observations: &model::ObservationsFile,
) -> Vec<String> {
    let mut diagnostics = Vec::new();
    if artifact.schema_version != model::FRONTIER_SCHEMA {
        diagnostics.push(format!(
            "frontier schema `{}` does not match `{}`",
            artifact.schema_version,
            model::FRONTIER_SCHEMA
        ));
    }
    validate_provider(&artifact.provider, &mut diagnostics);
    if artifact.provider != observations.provider {
        diagnostics.push("frontier provider binding does not match the observations provider".to_owned());
    }
    if artifact.bounds.active_batches != model::MAX_ACTIVE_BATCHES {
        diagnostics.push("frontier must admit exactly one active batch".to_owned());
    }
    if artifact.bounds.modules_per_batch == 0 || artifact.bounds.theorems_per_module == 0 {
        diagnostics.push("frontier batch bounds must be positive".to_owned());
    }
    if artifact.bounds != model::bounds() {
        diagnostics.push("frontier bounds do not match the named bounds".to_owned());
    }
    if artifact.non_claims != model::non_claims() {
        diagnostics.push("frontier non-claims do not match the required non-claims".to_owned());
    }
    let mut row_modules = BTreeSet::new();
    let mut previous: Option<&str> = None;
    for row in &artifact.modules {
        if !row_modules.insert(row.module.clone()) {
            diagnostics.push(format!("duplicate frontier row for `{}`", row.module));
        }
        if previous.is_some_and(|value| value >= row.module.as_str()) {
            diagnostics.push(format!("frontier rows are not sorted at `{}`", row.module));
        }
        previous = Some(row.module.as_str());
        if !is_known_class(&row.hypothesis) || row.hypothesis == model::CLASS_UNMEASURED {
            diagnostics.push(format!("frontier hypothesis for `{}` is not a tier class", row.module));
        }
        if !is_known_class(&row.measured_class) {
            diagnostics.push(format!(
                "frontier measured class `{}` for `{}` is not a known class",
                row.measured_class, row.module
            ));
        }
        match row.observation.as_ref() {
            Some(observation) => {
                if observation.module != row.module {
                    diagnostics.push(format!("observation module does not match frontier row `{}`", row.module));
                }
                if observation.measured_class != row.measured_class {
                    diagnostics
                        .push(format!("frontier row `{}` measured class does not match its observation", row.module));
                }
            }
            None => {
                if row.measured_class != model::CLASS_UNMEASURED {
                    diagnostics.push(format!(
                        "frontier row `{}` claims measured class `{}` without an observation",
                        row.module, row.measured_class
                    ));
                }
            }
        }
    }
    let expected: BTreeSet<&String> = inventory.iter().collect();
    if row_modules.len() != expected.len() || row_modules.iter().collect::<BTreeSet<&String>>() != expected {
        diagnostics.push("frontier rows do not cover the module inventory exactly".to_owned());
    }
    let recomputed = count_rows(&artifact.modules);
    if recomputed != artifact.counts {
        diagnostics.push("frontier counts do not match the recorded rows".to_owned());
    }
    diagnostics
}

pub(crate) fn validate_targets(targets: &model::TargetsFile, requirement_ids: &[String]) -> Vec<String> {
    let mut diagnostics = Vec::new();
    if targets.schema_version != model::TARGETS_SCHEMA {
        diagnostics.push(format!(
            "targets schema `{}` does not match `{}`",
            targets.schema_version,
            model::TARGETS_SCHEMA
        ));
    }
    if targets.targets.len() as u64 > model::MAX_MODULES_PER_BATCH {
        diagnostics.push("selected targets exceed the module batch bound".to_owned());
    }
    let known: BTreeSet<&String> = requirement_ids.iter().collect();
    let mut seen_requirements = BTreeSet::new();
    let mut seen_statements = BTreeSet::new();
    for target in &targets.targets {
        if !known.contains(&target.requirement_id) {
            diagnostics.push(format!("target requirement `{}` is not declared", target.requirement_id));
        }
        if !seen_requirements.insert(target.requirement_id.clone()) {
            diagnostics.push(format!("duplicate target requirement `{}`", target.requirement_id));
        }
        if !is_module_path(&target.module) {
            diagnostics.push(format!("target module `{}` is not a module path", target.module));
        }
        if target.statement_blake3.is_empty() {
            diagnostics.push(format!("target `{}` has no statement identity", target.requirement_id));
        }
        if target.statement_blake3.len() as u64 > model::MAX_THEOREMS_PER_MODULE {
            diagnostics.push(format!("target `{}` exceeds the theorem bound", target.requirement_id));
        }
        for statement in &target.statement_blake3 {
            if !is_blake3(statement) {
                diagnostics
                    .push(format!("target `{}` statement identity is not a BLAKE3 identity", target.requirement_id));
            }
            if !seen_statements.insert(statement.clone()) {
                diagnostics.push(format!("statement identity `{statement}` is reused across targets"));
            }
        }
        if target.claim_class.is_empty() {
            diagnostics.push(format!("target `{}` has no claim class", target.requirement_id));
        }
        if target.trust_closure.is_empty() {
            diagnostics.push(format!("target `{}` has an empty trust closure", target.requirement_id));
        }
        if !is_blake3(&target.receipt) {
            diagnostics.push(format!("target `{}` receipt is not a BLAKE3 identity", target.requirement_id));
        }
        if !model::RECEIPT_CLASSES.contains(&target.receipt_class.as_str()) {
            diagnostics.push(format!(
                "target `{}` receipt class `{}` is not a known class",
                target.requirement_id, target.receipt_class
            ));
        }
        if !is_relative_path(&target.receipt_evidence) {
            diagnostics.push(format!(
                "target `{}` receipt evidence `{}` is not a safe relative path",
                target.requirement_id, target.receipt_evidence
            ));
        }
    }
    diagnostics
}

pub(crate) fn build_registry(requirement_ids: &[String], targets: &model::TargetsFile) -> model::RegistryArtifact {
    let mut by_requirement = BTreeMap::new();
    for target in &targets.targets {
        by_requirement.insert(target.requirement_id.clone(), target.clone());
    }
    let mut rows = Vec::new();
    for requirement in requirement_ids {
        if let Some(target) = by_requirement.get(requirement) {
            rows.push(model::RegistryRow {
                requirement_id: requirement.clone(),
                targeted: true,
                statement_blake3: target.statement_blake3.clone(),
                claim_class: Some(target.claim_class.clone()),
                module: Some(target.module.clone()),
                trust_closure: target.trust_closure.clone(),
                receipt: Some(target.receipt.clone()),
                receipt_class: Some(target.receipt_class.clone()),
                receipt_evidence: Some(target.receipt_evidence.clone()),
                non_target_reason: None,
            });
        } else {
            rows.push(model::RegistryRow {
                requirement_id: requirement.clone(),
                targeted: false,
                statement_blake3: Vec::new(),
                claim_class: None,
                module: None,
                trust_closure: Vec::new(),
                receipt: None,
                receipt_class: None,
                receipt_evidence: None,
                non_target_reason: Some(NON_TARGET_REASON.to_owned()),
            });
        }
    }
    let targeted = rows.iter().filter(|row| row.targeted).count() as u64;
    let total = rows.len() as u64;
    model::RegistryArtifact {
        schema_version: model::REGISTRY_SCHEMA.to_owned(),
        rows,
        counts: model::RegistryCounts {
            total_requirements: total,
            targeted,
            non_target: total - targeted,
        },
        non_claims: model::non_claims(),
    }
}

pub(crate) fn validate_registry(
    registry: &model::RegistryArtifact,
    requirement_ids: &[String],
    targets: &model::TargetsFile,
) -> Vec<String> {
    let mut diagnostics = Vec::new();
    if registry.schema_version != model::REGISTRY_SCHEMA {
        diagnostics.push(format!(
            "registry schema `{}` does not match `{}`",
            registry.schema_version,
            model::REGISTRY_SCHEMA
        ));
    }
    if registry.non_claims != model::non_claims() {
        diagnostics.push("registry non-claims do not match the required non-claims".to_owned());
    }
    let mut by_requirement = BTreeMap::new();
    for target in &targets.targets {
        by_requirement.insert(target.requirement_id.clone(), target.clone());
    }
    let expected: BTreeSet<&String> = requirement_ids.iter().collect();
    let mut seen = BTreeSet::new();
    for row in &registry.rows {
        if !seen.insert(row.requirement_id.clone()) {
            diagnostics.push(format!("duplicate registry row for `{}`", row.requirement_id));
        }
        if !expected.contains(&row.requirement_id) {
            diagnostics.push(format!("registry row `{}` is not a declared requirement", row.requirement_id));
        }
        match (row.targeted, by_requirement.get(&row.requirement_id)) {
            (true, Some(target)) => {
                if row.statement_blake3 != target.statement_blake3 {
                    diagnostics.push(format!("registry row `{}` statement identity drifted", row.requirement_id));
                }
                if row.module.as_deref() != Some(target.module.as_str()) {
                    diagnostics.push(format!("registry row `{}` module drifted", row.requirement_id));
                }
                if row.trust_closure != target.trust_closure {
                    diagnostics.push(format!("registry row `{}` trust closure drifted", row.requirement_id));
                }
                if row.receipt.as_deref() != Some(target.receipt.as_str()) {
                    diagnostics.push(format!("registry row `{}` receipt drifted", row.requirement_id));
                }
                if row.receipt_class.as_deref() != Some(target.receipt_class.as_str()) {
                    diagnostics.push(format!("registry row `{}` receipt class drifted", row.requirement_id));
                }
                if row.receipt_evidence.as_deref() != Some(target.receipt_evidence.as_str()) {
                    diagnostics.push(format!("registry row `{}` receipt evidence drifted", row.requirement_id));
                }
                if row.non_target_reason.is_some() {
                    diagnostics
                        .push(format!("targeted registry row `{}` carries a non-target reason", row.requirement_id));
                }
            }
            (true, None) => {
                diagnostics.push(format!("registry row `{}` is targeted but not selected", row.requirement_id));
            }
            (false, Some(_)) => {
                diagnostics.push(format!("registry row `{}` is not targeted but selected", row.requirement_id));
            }
            (false, None) => {
                if row.non_target_reason.is_none() {
                    diagnostics.push(format!("registry row `{}` has no non-target reason", row.requirement_id));
                }
                if !row.statement_blake3.is_empty()
                    || row.receipt.is_some()
                    || row.receipt_class.is_some()
                    || row.receipt_evidence.is_some()
                {
                    diagnostics
                        .push(format!("non-target registry row `{}` carries proof identities", row.requirement_id));
                }
            }
        }
    }
    if seen.len() != expected.len() || expected.iter().any(|identifier| !seen.contains(*identifier)) {
        diagnostics.push("registry rows do not cover the declared requirements exactly".to_owned());
    }
    let targeted = registry.rows.iter().filter(|row| row.targeted).count() as u64;
    let total = registry.rows.len() as u64;
    let counts = model::RegistryCounts {
        total_requirements: total,
        targeted,
        non_target: total - targeted,
    };
    if counts != registry.counts {
        diagnostics.push("registry counts do not match the recorded rows".to_owned());
    }
    diagnostics
}

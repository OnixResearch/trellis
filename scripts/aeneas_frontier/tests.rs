//! Positive and negative self-tests for the frontier instrument.
//!
//! Every negative case mutates exactly one fact and must produce a diagnostic.

use std::collections::BTreeMap;

use crate::model;
use crate::validation;

const MODULE_TIER_1: &str = "src/arithmetic.rs";
const MODULE_TIER_2: &str = "src/collections.rs";
const MODULE_TIER_3: &str = "src/crypto_digest.rs";
const MODULE_TIER_4: &str = "src/ucan_signature.rs";
const REQUESTED_REQUIREMENT: &str = "arithmetic.abs-diff";
const OTHER_REQUIREMENT: &str = "bounds.clamp";
const STATEMENT: &str = "b3:a67c10d4699ae7b0ddb5b35eb87552b893fcf038c93fd5c6b4c776d5b172e3fb";
const SECOND_STATEMENT: &str = "b3:b3df314ff917bb6eb9e26f7a8d56b7c5bc8439da5e5c5076c47a4bee58c7ade4";
const RECEIPT: &str = "b3:da3840e394b1c671ec8b908e98a6ac52a054ab20b55a1cb48cceb1edae2c8d9b";
const PROFILE: &str = "b3:cd11b49d1acede2c6a91fa26f0b7a6b4b28db4007b4601a65b3269706a0ce625";
const MATRIX: &str = "b3:403656d75de41b955cd2493ef289ba17308da65a3d214babc1ea606aeae248f5";

fn provider() -> model::ProviderBinding {
    model::ProviderBinding {
        package: "octet-aeneas-lean-provider-closure".to_owned(),
        revision: "890f424cd7b1b2015f39c0eb5747f1bd1e576028".to_owned(),
        profile_identity: PROFILE.to_owned(),
        compatibility_matrix_identity: MATRIX.to_owned(),
    }
}

fn observation(module: &str, stage: &str, outcome: &str, measured_class: &str) -> model::Observation {
    model::Observation {
        module: module.to_owned(),
        stage: stage.to_owned(),
        outcome: outcome.to_owned(),
        measured_class: measured_class.to_owned(),
        reviewed: true,
        construct: None,
        diagnostic_class: None,
        evidence: None,
    }
}

fn observations(list: Vec<model::Observation>) -> model::ObservationsFile {
    model::ObservationsFile {
        schema_version: model::OBSERVATIONS_SCHEMA.to_owned(),
        provider: provider(),
        observations: list,
    }
}

fn inventory() -> Vec<String> {
    vec![
        MODULE_TIER_1.to_owned(),
        MODULE_TIER_2.to_owned(),
        MODULE_TIER_3.to_owned(),
        MODULE_TIER_4.to_owned(),
    ]
}

fn hypotheses() -> BTreeMap<String, String> {
    BTreeMap::from([
        (MODULE_TIER_1.to_owned(), model::CLASS_TIER_1.to_owned()),
        (MODULE_TIER_2.to_owned(), model::CLASS_TIER_2.to_owned()),
        (MODULE_TIER_3.to_owned(), model::CLASS_TIER_3.to_owned()),
        (MODULE_TIER_4.to_owned(), model::CLASS_TIER_4.to_owned()),
    ])
}

fn supported_observations() -> model::ObservationsFile {
    let mut blocked =
        observation(MODULE_TIER_2, "charon-extraction", model::OUTCOME_BLOCKED, "blocked-charon-extraction");
    blocked.diagnostic_class = Some("unsupported-generic-mutable-reference".to_owned());
    let mut failed = observation(MODULE_TIER_3, "charon-extraction", model::OUTCOME_FAILED, model::CLASS_UNMEASURED);
    failed.diagnostic_class = Some("process-spawn-failure".to_owned());
    let mut excluded = observation(MODULE_TIER_4, "aeneas-translation", model::OUTCOME_BLOCKED, model::CLASS_EXCLUDED);
    excluded.construct = Some("unsafe-block".to_owned());
    observations(vec![
        observation(MODULE_TIER_1, "aeneas-translation", model::OUTCOME_SUPPORTED, model::CLASS_TIER_1),
        blocked,
        failed,
        excluded,
    ])
}

fn targets_file() -> model::TargetsFile {
    model::TargetsFile {
        schema_version: model::TARGETS_SCHEMA.to_owned(),
        targets: vec![model::Target {
            requirement_id: REQUESTED_REQUIREMENT.to_owned(),
            module: MODULE_TIER_1.to_owned(),
            statement_blake3: vec![STATEMENT.to_owned(), SECOND_STATEMENT.to_owned()],
            claim_class: "named-theorem-over-generated-model".to_owned(),
            trust_closure: vec!["propext".to_owned()],
            receipt: RECEIPT.to_owned(),
            receipt_class: model::RECEIPT_CLASS_TRELLIS.to_owned(),
            receipt_evidence: "aeneas-frontier/evidence/lean-2026-09-14.json".to_owned(),
        }],
    }
}

fn requirements() -> Vec<String> {
    vec![REQUESTED_REQUIREMENT.to_owned(), OTHER_REQUIREMENT.to_owned()]
}

fn frontier_of(file: &model::ObservationsFile) -> model::FrontierArtifact {
    let rows = validation::build_rows(&inventory(), &hypotheses(), file);
    validation::build_frontier(&provider(), rows)
}

fn positive_cases() -> Result<(), String> {
    let file = supported_observations();
    if !validation::validate_observations(&file).is_empty() {
        return Err("positive observations must validate".to_owned());
    }
    let frontier = frontier_of(&file);
    if !validation::validate_frontier(&frontier, &inventory(), &file).is_empty() {
        return Err("positive frontier must validate".to_owned());
    }
    if frontier.counts.measured != 1
        || frontier.counts.blocked != 1
        || frontier.counts.excluded != 1
        || frontier.counts.unmeasured != 1
    {
        return Err("positive frontier counts drifted".to_owned());
    }
    let targets = targets_file();
    if !validation::validate_targets(&targets, &requirements()).is_empty() {
        return Err("positive targets must validate".to_owned());
    }
    let registry = validation::build_registry(&requirements(), &targets);
    if !validation::validate_registry(&registry, &requirements(), &targets).is_empty() {
        return Err("positive registry must validate".to_owned());
    }
    let non_claim_text = "This change does not prove whole-library proof coverage.\nIt does not prove semantic equivalence,\nverifier independence, or release eligibility.\n";
    if !validation::validate_claim_text("doc", non_claim_text).is_empty() {
        return Err("non-claim text must pass the claim check".to_owned());
    }
    Ok(())
}

fn negative_cases() -> Result<u64, String> {
    let file = supported_observations();
    let frontier = frontier_of(&file);
    let targets = targets_file();
    let registry = validation::build_registry(&requirements(), &targets);
    Ok(negative_frontier_cases(&file, &frontier)?
        + negative_target_cases(&targets)?
        + negative_registry_cases(&targets, &registry)?
        + negative_claim_cases()?)
}

fn negative_frontier_cases(file: &model::ObservationsFile, frontier: &model::FrontierArtifact) -> Result<u64, String> {
    let mut negatives = 0u64;
    let mut expect = |name: &str, diagnostics: Vec<String>| -> Result<(), String> {
        if diagnostics.is_empty() {
            Err(format!("negative case `{name}` produced no diagnostic"))
        } else {
            negatives += 1;
            Ok(())
        }
    };

    let mut supported_wrong_class = file.clone();
    supported_wrong_class.observations[0].measured_class = String::from(model::CLASS_UNMEASURED);
    expect("supported-non-tier-class", validation::validate_observations(&supported_wrong_class))?;

    let mut supported_with_construct = file.clone();
    supported_with_construct.observations[0].construct = Some("unsafe-block".to_owned());
    expect("supported-recorded-construct", validation::validate_observations(&supported_with_construct))?;

    let mut unreviewed = file.clone();
    unreviewed.observations[0].reviewed = false;
    expect("unreviewed-measured", validation::validate_observations(&unreviewed))?;

    let mut unknown_diagnostic = file.clone();
    unknown_diagnostic.observations[2].diagnostic_class = Some("whoops".to_owned());
    expect("unknown-diagnostic-class", validation::validate_observations(&unknown_diagnostic))?;

    let mut timeout_budget = file.clone();
    timeout_budget.observations[2].diagnostic_class = Some(model::DIAGNOSTIC_CLASS_TIMEOUT.to_owned());
    if !validation::validate_observations(&timeout_budget).is_empty() {
        return Err("a stage-timeout observation must stay a valid budget result".to_owned());
    }

    let mut stage_drift = file.clone();
    stage_drift.observations[1].measured_class = String::from("blocked-aeneas-translation");
    expect("blocked-stage-class-drift", validation::validate_observations(&stage_drift))?;

    let mut missing_construct = file.clone();
    missing_construct.observations[3].construct = None;
    expect("excluded-without-construct", validation::validate_observations(&missing_construct))?;

    let mut failed_classified = file.clone();
    failed_classified.observations[2].measured_class = String::from(model::CLASS_TIER_3);
    expect("failed-marked-measured", validation::validate_observations(&failed_classified))?;

    let mut duplicate = file.clone();
    duplicate.observations.push(observation(
        MODULE_TIER_1,
        "aeneas-translation",
        model::OUTCOME_SUPPORTED,
        model::CLASS_TIER_1,
    ));
    expect("duplicate-observation", validation::validate_observations(&duplicate))?;

    let mut unobserved_class = frontier.clone();
    unobserved_class.modules[2].measured_class = String::from(model::CLASS_TIER_3);
    expect("class-without-observation", validation::validate_frontier(&unobserved_class, &inventory(), file))?;

    let mut short_inventory = inventory();
    short_inventory.pop();
    expect("inventory-not-covered", validation::validate_frontier(frontier, &short_inventory, file))?;

    let mut count_drift = frontier.clone();
    count_drift.counts.measured = 0;
    expect("count-drift", validation::validate_frontier(&count_drift, &inventory(), file))?;

    let mut claim_drift = frontier.clone();
    claim_drift.non_claims.push("proves whole-library coverage".to_owned());
    expect("overclaim", validation::validate_frontier(&claim_drift, &inventory(), file))?;

    let mut bounds_drift = frontier.clone();
    bounds_drift.bounds.active_batches = 2;
    expect("bounds-drift", validation::validate_frontier(&bounds_drift, &inventory(), file))?;

    let mut binding_drift = frontier.clone();
    binding_drift.provider.profile_identity = String::from("b3:0000");
    expect("provider-binding-drift", validation::validate_frontier(&binding_drift, &inventory(), file))?;

    Ok(negatives)
}

fn negative_target_cases(targets: &model::TargetsFile) -> Result<u64, String> {
    let mut negatives = 0u64;
    let mut expect = |name: &str, diagnostics: Vec<String>| -> Result<(), String> {
        if diagnostics.is_empty() {
            Err(format!("negative case `{name}` produced no diagnostic"))
        } else {
            negatives += 1;
            Ok(())
        }
    };

    let mut unknown_requirement = targets.clone();
    unknown_requirement.targets[0].requirement_id = String::from("arithmetic.unknown");
    expect("unknown-requirement", validation::validate_targets(&unknown_requirement, &requirements()))?;

    let mut reused_statement = targets.clone();
    reused_statement.targets.push(model::Target {
        requirement_id: OTHER_REQUIREMENT.to_owned(),
        module: MODULE_TIER_1.to_owned(),
        statement_blake3: vec![STATEMENT.to_owned()],
        claim_class: "named-theorem-over-generated-model".to_owned(),
        trust_closure: vec!["propext".to_owned()],
        receipt: RECEIPT.to_owned(),
        receipt_class: model::RECEIPT_CLASS_TRELLIS.to_owned(),
        receipt_evidence: "aeneas-frontier/evidence/lean-2026-09-14.json".to_owned(),
    });
    expect("reused-statement", validation::validate_targets(&reused_statement, &requirements()))?;

    let mut empty_trust = targets.clone();
    empty_trust.targets[0].trust_closure.clear();
    expect("empty-trust-closure", validation::validate_targets(&empty_trust, &requirements()))?;

    let mut unknown_receipt_class = targets.clone();
    unknown_receipt_class.targets[0].receipt_class = String::from("self-asserted");
    expect("unknown-receipt-class", validation::validate_targets(&unknown_receipt_class, &requirements()))?;

    let mut unsafe_evidence = targets.clone();
    unsafe_evidence.targets[0].receipt_evidence = String::from("../outside.json");
    expect("unsafe-receipt-evidence", validation::validate_targets(&unsafe_evidence, &requirements()))?;

    let mut over_bound = model::TargetsFile {
        schema_version: model::TARGETS_SCHEMA.to_owned(),
        targets: Vec::new(),
    };
    for index in 0..=model::MAX_MODULES_PER_BATCH {
        over_bound.targets.push(model::Target {
            requirement_id: format!("bounds.clamp-{index}"),
            module: MODULE_TIER_1.to_owned(),
            statement_blake3: vec![format!("b3:{index:064x}")],
            claim_class: "named-theorem-over-generated-model".to_owned(),
            trust_closure: vec!["propext".to_owned()],
            receipt: RECEIPT.to_owned(),
            receipt_class: model::RECEIPT_CLASS_TRELLIS.to_owned(),
            receipt_evidence: "aeneas-frontier/evidence/lean-2026-09-14.json".to_owned(),
        });
    }
    let mut many_requirements = requirements();
    many_requirements.extend(over_bound.targets.iter().map(|target| target.requirement_id.clone()));
    expect("target-bound-exceeded", validation::validate_targets(&over_bound, &many_requirements))?;

    Ok(negatives)
}

fn negative_registry_cases(targets: &model::TargetsFile, registry: &model::RegistryArtifact) -> Result<u64, String> {
    let mut negatives = 0u64;
    let mut expect = |name: &str, diagnostics: Vec<String>| -> Result<(), String> {
        if diagnostics.is_empty() {
            Err(format!("negative case `{name}` produced no diagnostic"))
        } else {
            negatives += 1;
            Ok(())
        }
    };

    let mut missing_row = registry.clone();
    missing_row.rows.pop();
    missing_row.counts.total_requirements -= 1;
    missing_row.counts.non_target -= 1;
    expect("registry-missing-row", validation::validate_registry(&missing_row, &requirements(), targets))?;

    let mut missing_reason = registry.clone();
    for row in &mut missing_reason.rows {
        if !row.targeted {
            row.non_target_reason = None;
        }
    }
    expect(
        "non-target-without-reason",
        validation::validate_registry(&missing_reason, &requirements(), targets),
    )?;

    let mut targeted_with_reason = registry.clone();
    for row in &mut targeted_with_reason.rows {
        if row.targeted {
            row.non_target_reason = Some(validation::NON_TARGET_REASON.to_owned());
        }
    }
    expect("target-with-reason", validation::validate_registry(&targeted_with_reason, &requirements(), targets))?;

    let mut statement_drift = registry.clone();
    for row in &mut statement_drift.rows {
        if row.targeted {
            row.statement_blake3 = vec![SECOND_STATEMENT.to_owned()];
        }
    }
    expect(
        "registry-statement-drift",
        validation::validate_registry(&statement_drift, &requirements(), targets),
    )?;

    let mut receipt_class_drift = registry.clone();
    for row in &mut receipt_class_drift.rows {
        if row.targeted {
            row.receipt_class = Some(model::RECEIPT_CLASS_LOCAL.to_owned());
        }
    }
    expect(
        "registry-receipt-class-drift",
        validation::validate_registry(&receipt_class_drift, &requirements(), targets),
    )?;

    Ok(negatives)
}

fn negative_claim_cases() -> Result<u64, String> {
    let cases = [
        "The frontier proves whole-library proof coverage for Trellis.",
        "The manifest shows the two models are semantically equivalent.",
        "The batch proves verifier independence.",
        "The batch proves progress on the whole corpus.",
        "The batch proves trust-boundary closure for the unsafe modules.",
        "The campaign closes the trust boundaries.",
    ];
    let mut negatives = 0u64;
    for text in cases {
        if validation::validate_claim_text("doc", text).is_empty() {
            return Err(format!("negative claim case `{text}` produced no diagnostic"));
        }
        negatives += 1;
    }
    Ok(negatives)
}

pub(crate) fn run() -> Result<u64, String> {
    positive_cases()?;
    negative_cases()
}

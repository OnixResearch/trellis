use std::collections::BTreeSet;

use serde_json::Value;

use super::identity;
use super::model::ArtifactIdentity;
use super::validation::Inputs;

const EXPECTED_SOURCE_PATH: &str = "src/arithmetic.rs";
const EXPECTED_RUST_ITEM: &str = "verified_logic::arithmetic::abs_diff";
const EXPECTED_CHARON_ITEM: &str = "crate::arithmetic::abs_diff";
const EXPECTED_LEAN_ITEM: &str = "verified_logic.arithmetic.abs_diff";
const EXPECTED_CRATE: &str = "verified_logic";
const EXPECTED_REQUIREMENT: &str = "arithmetic.abs-diff";
const EXPECTED_TARGET: &str = "x86_64-unknown-linux-gnu";
const EXPECTED_BEGIN_LINE: u64 = 21;
const EXPECTED_END_LINE: u64 = 25;
const EXPECTED_POINTER_WIDTH_BYTES: u64 = 8;
const EXPECTED_FUNCTION_COUNT: usize = 1;
const EXPECTED_TARGET_COUNT: usize = 1;
const FORBIDDEN_THEOREM_TOKENS: [&str; 2] = ["sorry", "admit"];

pub(crate) fn validate(inputs: &Inputs, diagnostics: &mut Vec<String>) {
    validate_llbc(inputs, diagnostics);
    validate_translation(inputs, diagnostics);
    validate_generated_lean(inputs, diagnostics);
    validate_theorems(inputs, diagnostics);
    validate_verus_binding(inputs, diagnostics);
}

fn validate_llbc(inputs: &Inputs, diagnostics: &mut Vec<String>) {
    if inputs.llbc.get("has_errors").and_then(Value::as_bool) != Some(false) {
        diagnostics.push("LLBC has errors or omits its error status".to_owned());
    }
    let translated = inputs.llbc.get("translated");
    if translated.and_then(|value| value.get("crate_name")).and_then(Value::as_str) != Some(EXPECTED_CRATE) {
        diagnostics.push("LLBC crate identity drifted".to_owned());
    }
    let options = translated.and_then(|value| value.get("options"));
    if options.and_then(|value| value.get("skip_borrowck")).and_then(Value::as_bool) != Some(false) {
        diagnostics.push("LLBC extraction skipped borrow checking".to_owned());
    }
    let expected_start = [Value::String(EXPECTED_CHARON_ITEM.to_owned())];
    if options.and_then(|value| value.get("start_from")).and_then(Value::as_array).map(Vec::as_slice)
        != Some(expected_start.as_slice())
        || options.and_then(|value| value.get("start_from_pub")).and_then(Value::as_bool) != Some(false)
        || options.and_then(|value| value.get("preset")).and_then(Value::as_str) != Some("Aeneas")
    {
        diagnostics.push("LLBC extraction command did not preserve exact item selection".to_owned());
    }
    validate_target(translated, diagnostics);
    let functions = translated.and_then(|value| value.get("fun_decls")).and_then(Value::as_array);
    if functions.map(Vec::len) != Some(EXPECTED_FUNCTION_COUNT) {
        diagnostics.push("LLBC selected function set is not exactly one item".to_owned());
        return;
    }
    let function = &functions.expect("function count checked")[0];
    let span = function.pointer("/item_meta/span/data");
    let begin = span.and_then(|value| value.pointer("/beg/line")).and_then(Value::as_u64);
    let end = span.and_then(|value| value.pointer("/end/line")).and_then(Value::as_u64);
    if begin != Some(EXPECTED_BEGIN_LINE) || end != Some(EXPECTED_END_LINE) {
        diagnostics.push("LLBC exact source span drifted".to_owned());
    }
    let observed_source = function.pointer("/item_meta/source_text").and_then(Value::as_str);
    let exact_source = super::validation::material(inputs, EXPECTED_SOURCE_PATH)
        .ok()
        .and_then(|bytes| std::str::from_utf8(bytes).ok())
        .and_then(|text| selected_source_text(text, EXPECTED_BEGIN_LINE, EXPECTED_END_LINE));
    if observed_source != exact_source.as_deref() {
        diagnostics.push("LLBC source text is detached from the exact runtime source".to_owned());
    }
}

fn validate_target(translated: Option<&Value>, diagnostics: &mut Vec<String>) {
    let targets = translated.and_then(|value| value.get("target_information")).and_then(Value::as_array);
    if targets.map(Vec::len) != Some(EXPECTED_TARGET_COUNT) {
        diagnostics.push("target metadata is missing or ambiguous".to_owned());
        return;
    }
    let target = &targets.expect("target count checked")[0];
    let key = target.get("key").and_then(Value::as_str);
    let pointer = target.pointer("/value/target_pointer_size").and_then(Value::as_u64);
    let little = target.pointer("/value/is_little_endian").and_then(Value::as_bool);
    if key != Some(EXPECTED_TARGET) || pointer != Some(EXPECTED_POINTER_WIDTH_BYTES) || little != Some(true) {
        diagnostics.push("target metadata does not match the reviewed cohort".to_owned());
    }
}

fn validate_translation(inputs: &Inputs, diagnostics: &mut Vec<String>) {
    if inputs.translation.get("crate").and_then(Value::as_str) != Some(EXPECTED_CRATE) {
        diagnostics.push("translation mapping crate identity drifted".to_owned());
    }
    let functions = inputs.translation.get("functions").and_then(Value::as_array);
    if functions.map(Vec::len) != Some(EXPECTED_FUNCTION_COUNT) {
        diagnostics.push("translation mapping must contain exactly one function".to_owned());
        return;
    }
    let row = &functions.expect("translation count checked")[0];
    let llbc_id = inputs.llbc.pointer("/translated/fun_decls/0/def_id").and_then(Value::as_u64);
    let translation_id = row.get("def_id").and_then(Value::as_u64);
    if llbc_id != translation_id {
        diagnostics.push("run-local declaration join does not match this LLBC run".to_owned());
    }
    let valid = row.get("rust_name").and_then(Value::as_str) == Some(EXPECTED_RUST_ITEM)
        && row.get("lean_name").and_then(Value::as_str) == Some(EXPECTED_LEAN_ITEM)
        && row.get("lean_file").and_then(Value::as_str) == Some("Llbc.lean")
        && row.pointer("/source/file").and_then(Value::as_str) == Some(EXPECTED_SOURCE_PATH)
        && row.pointer("/source/begin_line").and_then(Value::as_u64) == Some(EXPECTED_BEGIN_LINE)
        && row.pointer("/source/end_line").and_then(Value::as_u64) == Some(EXPECTED_END_LINE)
        && row.get("is_opaque").and_then(Value::as_bool) == Some(false)
        && row.get("can_fail").and_then(Value::as_bool) == Some(true)
        && row.get("can_diverge").and_then(Value::as_bool) == Some(false)
        && row.get("is_rec").and_then(Value::as_bool) == Some(false);
    if !valid {
        diagnostics.push("translation mapping metadata drifted".to_owned());
    }
}

fn validate_generated_lean(inputs: &Inputs, diagnostics: &mut Vec<String>) {
    let Some(artifact) = artifact_for_role(&inputs.profile.artifacts, "generated-lean") else {
        diagnostics.push("generated Lean identity is missing".to_owned());
        return;
    };
    let Ok(bytes) = super::validation::material(inputs, &artifact.path) else {
        diagnostics.push("generated Lean identity is missing".to_owned());
        return;
    };
    if artifact.blake3 != identity::prefixed(bytes) {
        diagnostics.push("generated Lean identity drifted".to_owned());
    }
    let text = String::from_utf8_lossy(bytes);
    if !text.contains("Source: 'src/arithmetic.rs', lines 21:0-25:1")
        || !text.contains("def arithmetic.abs_diff (a : Std.U64) (b : Std.U64) : Result Std.U64")
    {
        diagnostics.push("generated Lean declaration is detached from the translation mapping".to_owned());
    }
}

fn validate_theorems(inputs: &Inputs, diagnostics: &mut Vec<String>) {
    let consumer = &inputs.profile.consumer.theorems;
    let mappings = &inputs.profile.theorem_mappings;
    for mapping in mappings {
        let Some(theorem) = consumer.iter().find(|theorem| theorem.theorem_id == mapping.theorem_id) else {
            diagnostics.push("theorem mapping has no matching consumer theorem".to_owned());
            continue;
        };
        if theorem.path != mapping.path
            || theorem.declaration_name != mapping.declaration_name
            || theorem.requirement_id != mapping.requirement_id
            || theorem.claim_class != mapping.claim_class
            || theorem.approved_statement_blake3 != mapping.approved_statement_blake3
            || mapping.requirement_id != EXPECTED_REQUIREMENT
            || mapping.lean_theorem_symbol != mapping.declaration_name
        {
            diagnostics.push("theorem mapping differs from the approved consumer mapping".to_owned());
        }
        let Ok(bytes) = super::validation::material(inputs, &mapping.path) else {
            diagnostics.push("theorem source identity is missing".to_owned());
            continue;
        };
        if mapping.approved_statement_blake3 != identity::prefixed(bytes) {
            diagnostics.push("theorem source identity does not match approved statement bytes".to_owned());
        }
        let text = String::from_utf8_lossy(bytes);
        if !text.contains(&format!("theorem {} ", mapping.declaration_name)) {
            diagnostics.push("theorem mapping declaration is absent from maintained source".to_owned());
        }
        if theorem_is_vacuous(&text) || forbidden_theorem_token(&text) {
            diagnostics.push("theorem adequacy rejected vacuous or forbidden source".to_owned());
        }
    }
    validate_axiom_closure(inputs, mappings, diagnostics);
}

fn validate_axiom_closure(inputs: &Inputs, mappings: &[super::model::TheoremMapping], diagnostics: &mut Vec<String>) {
    let declared_axioms = mappings.iter().flat_map(|mapping| mapping.axioms.iter().cloned()).collect::<BTreeSet<_>>();
    let closure_axioms = inputs.profile.trust_closure.selected_axioms.iter().cloned().collect::<BTreeSet<_>>();
    if declared_axioms != closure_axioms || !inputs.profile.trust_closure.selected_opaque_definitions.is_empty() {
        diagnostics.push("trust closure does not match theorem axiom observations".to_owned());
    }
    let trust_ids = inputs
        .profile
        .consumer
        .trust_inputs
        .iter()
        .map(|input| input.input_id.strip_prefix("lean.").unwrap_or(&input.input_id))
        .collect::<BTreeSet<_>>();
    if !closure_axioms.iter().all(|axiom| trust_ids.contains(axiom.as_str())) {
        diagnostics.push("trust closure omits a selected Lean axiom".to_owned());
    }
}

fn validate_verus_binding(inputs: &Inputs, diagnostics: &mut Vec<String>) {
    let Ok(source) = super::validation::material(inputs, EXPECTED_SOURCE_PATH) else {
        return;
    };
    let source = String::from_utf8_lossy(source);
    for symbol in &inputs.profile.verus.proof_symbols {
        let name = symbol.rsplit("::").next().unwrap_or(symbol);
        if !source.contains(&format!("proof fn {name}")) {
            diagnostics.push(format!("Verus proof symbol `{symbol}` is absent from exact source"));
        }
    }
    let Ok(receipt) = super::validation::material(inputs, &inputs.profile.verus.receipt_path) else {
        diagnostics.push("Verus receipt is missing".to_owned());
        return;
    };
    let parsed: Value = match serde_json::from_slice(receipt) {
        Ok(value) => value,
        Err(error) => {
            diagnostics.push(format!("parse Verus receipt: {error}"));
            return;
        }
    };
    if !json_contains_string(&parsed, EXPECTED_REQUIREMENT) || !json_contains_string(&parsed, EXPECTED_SOURCE_PATH) {
        diagnostics.push("Verus receipt does not bind the selected requirement and source".to_owned());
    }
}

fn artifact_for_role<'a>(artifacts: &'a [ArtifactIdentity], role: &str) -> Option<&'a ArtifactIdentity> {
    artifacts.iter().find(|artifact| artifact.role == role)
}

fn selected_source_text(source: &str, begin_line: u64, end_line: u64) -> Option<String> {
    let begin = usize::try_from(begin_line.checked_sub(1)?).ok()?;
    let count = usize::try_from(end_line.checked_sub(begin_line)?.checked_add(1)?).ok()?;
    let selected = source.lines().skip(begin).take(count).collect::<Vec<_>>();
    if selected.len() == count {
        Some(selected.join("\n"))
    } else {
        None
    }
}

fn theorem_is_vacuous(source: &str) -> bool {
    source.lines().filter(|line| line.trim_start().starts_with("theorem ")).any(|line| {
        line.split_once(":=")
            .map(|(header, _)| header.split_once(':').is_some_and(|(_, statement)| statement.trim() == "True"))
            .unwrap_or(false)
    })
}

fn forbidden_theorem_token(source: &str) -> bool {
    FORBIDDEN_THEOREM_TOKENS.iter().any(|forbidden| {
        source
            .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
            .any(|token| token == *forbidden)
    })
}

fn json_contains_string(value: &Value, target: &str) -> bool {
    match value {
        Value::String(value) => value == target,
        Value::Array(values) => values.iter().any(|value| json_contains_string(value, target)),
        Value::Object(values) => values.values().any(|value| json_contains_string(value, target)),
        Value::Null | Value::Bool(_) | Value::Number(_) => false,
    }
}

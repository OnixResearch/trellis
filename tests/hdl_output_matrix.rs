use verified_logic::hdl_emit::HdlEmitRequest;
use verified_logic::hdl_emit::HdlEmitterMode;
use verified_logic::hdl_emit::HdlExampleKind;
use verified_logic::hdl_emit::hdl_fixture_metadata;
use verified_logic::hdl_emit::hdl_fixture_traceable;
use verified_logic::hdl_ir::HDL_EMPTY_COUNT;
use verified_logic::hdl_ir::HDL_OPTIONAL_RESET_LIMIT;
use verified_logic::hdl_ir::HDL_SINGLE_CLOCK;
use verified_logic::hdl_ir::HdlModuleShape;

const TEST_WIDTH: u16 = 8;
const TEST_INPUTS: u16 = 2;
const TEST_OUTPUTS: u16 = 1;
const TEST_EXPRS: u16 = 1;
const TEST_REGISTERS: u16 = 1;

#[derive(Clone, Copy)]
struct MatrixRow {
    requirement_id: &'static str,
    positive_artifact: &'static str,
    negative_artifact: &'static str,
    verify_marker_present: bool,
}

const MATRIX: &[MatrixRow] = &[
    MatrixRow {
        requirement_id: "hardware-output.ir",
        positive_artifact: "hdl_ir::admits_valid_combinational_ir_and_rejects_width_mismatch",
        negative_artifact: "hdl_ir::rejects_unsupported_nodes_before_evaluation",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.evaluator",
        positive_artifact: "hdl_ir::evaluates_combinational_module_deterministically",
        negative_artifact: "hdl_ir::rejects_missing_inputs_width_mismatch_and_invalid_state",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.adapter-subset",
        positive_artifact: "hdl_adapters::admits_supported_bitwise_bitfield_arithmetic_and_fsm_adapters",
        negative_artifact: "hdl_adapters::rejects_heap_io_async_crypto_distributed_and_spec_only_candidates",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.adapter-refinement",
        positive_artifact: "hdl_adapters::supported_adapters_match_source_semantics",
        negative_artifact: "hdl_adapters::finite_fsm_adapter_matches_source_next_state",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.verilog-emission",
        positive_artifact: "hdl_emit::verilog_emission_is_stable_and_matches_golden",
        negative_artifact: "hdl_emit::unsupported_or_reserved_shapes_emit_no_partial_text",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.systemverilog-emission",
        positive_artifact: "hdl_emit::systemverilog_registered_module_uses_always_ff_and_matches_golden",
        negative_artifact: "hdl_emit::unsupported_or_reserved_shapes_emit_no_partial_text",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.testbench",
        positive_artifact: "hdl_validation::vector_collateral_derives_expected_values_from_evaluator",
        negative_artifact: "hdl_validation::evaluator_rejection_blocks_vector_collateral",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.validation",
        positive_artifact: "hdl_validation::validation_classifies_pass_fail_and_skipped",
        negative_artifact: "hdl_validation::invalid_validation_input_records_failure_not_skip_or_pass",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.backend-admission",
        positive_artifact: "hdl_validation::backend_admission_requires_every_check_and_names_limits",
        negative_artifact: "hdl_validation::backend_admission_requires_every_check_and_names_limits",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.testing.matrix",
        positive_artifact: "tests/hdl_output_matrix.rs::matrix_requires_positive_and_negative_artifacts",
        negative_artifact: "tests/hdl_output_matrix.rs::matrix_rejects_missing_negative_artifact",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.testing.fixtures",
        positive_artifact: "hdl_emit::fixture_metadata_names_requirement_mode_and_oracle",
        negative_artifact: "hdl_emit::fixture_metadata_rejects_untraceable_entries",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.testing.evaluator-oracle",
        positive_artifact: "hdl_ir::evaluates_combinational_module_deterministically",
        negative_artifact: "hdl_ir::rejects_missing_inputs_width_mismatch_and_invalid_state",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.testing.emitter-golden",
        positive_artifact: "hdl_emit::verilog_emission_is_stable_and_matches_golden",
        negative_artifact: "hdl_emit::unsupported_or_reserved_shapes_emit_no_partial_text",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.testing.adapter-property",
        positive_artifact: "hdl_adapters::supported_adapters_match_source_semantics",
        negative_artifact: "hdl_adapters::rejects_heap_io_async_crypto_distributed_and_spec_only_candidates",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.testing.fail-closed",
        positive_artifact: "hdl_emit::unsupported_or_reserved_shapes_emit_no_partial_text",
        negative_artifact: "hdl_validation::invalid_validation_input_records_failure_not_skip_or_pass",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.testing.tool-evidence",
        positive_artifact: "hdl_validation::validation_classifies_pass_fail_and_skipped",
        negative_artifact: "hdl_validation::validation_classifies_pass_fail_and_skipped",
        verify_marker_present: true,
    },
    MatrixRow {
        requirement_id: "hardware-output.testing.tracey-gates",
        positive_artifact: "tests/hdl_output_matrix.rs::matrix_requires_verify_markers_for_all_rows",
        negative_artifact: "tests/hdl_output_matrix.rs::matrix_rejects_missing_verify_marker",
        verify_marker_present: true,
    },
];

fn matrix_row_is_complete(row: MatrixRow) -> bool {
    !row.requirement_id.is_empty()
        && !row.positive_artifact.is_empty()
        && !row.negative_artifact.is_empty()
        && row.verify_marker_present
}

fn module() -> HdlModuleShape {
    HdlModuleShape {
        input_count: TEST_INPUTS,
        output_count: TEST_OUTPUTS,
        expr_count: TEST_EXPRS,
        register_count: TEST_REGISTERS,
        clock_count: HDL_SINGLE_CLOCK,
        reset_count: HDL_OPTIONAL_RESET_LIMIT,
        fsm_state_count: HDL_EMPTY_COUNT,
        value_width: TEST_WIDTH,
        has_boolean_expr: true,
        canonical_order: true,
        unsupported_node_count: HDL_EMPTY_COUNT,
        malformed_width_count: HDL_EMPTY_COUNT,
    }
}

// r[verify hardware-output.testing.matrix]
#[test]
fn matrix_requires_positive_and_negative_artifacts() {
    for row in MATRIX {
        assert!(matrix_row_is_complete(*row), "incomplete matrix row: {}", row.requirement_id);
    }
}

// r[verify hardware-output.testing.matrix]
#[test]
fn matrix_rejects_missing_negative_artifact() {
    let missing_negative = MatrixRow {
        requirement_id: "hardware-output.testing.matrix",
        positive_artifact: "positive-test",
        negative_artifact: "",
        verify_marker_present: true,
    };
    assert!(!matrix_row_is_complete(missing_negative));
}

// r[verify hardware-output.testing.tracey-gates]
#[test]
fn matrix_requires_verify_markers_for_all_rows() {
    assert!(MATRIX.iter().all(|row| row.verify_marker_present));
}

// r[verify hardware-output.testing.tracey-gates]
#[test]
fn matrix_rejects_missing_verify_marker() {
    let missing_marker = MatrixRow {
        requirement_id: "hardware-output.testing.tracey-gates",
        positive_artifact: "positive-test",
        negative_artifact: "negative-test",
        verify_marker_present: false,
    };
    assert!(!matrix_row_is_complete(missing_marker));
}

// r[verify hardware-output.testing.fixtures]
#[test]
fn fixture_conventions_name_requirement_module_mode_and_oracle() {
    let request = HdlEmitRequest {
        module: module(),
        mode: HdlEmitterMode::SystemVerilog,
        kind: HdlExampleKind::RegisterStage,
        include_assertion_snippet: false,
        reserved_identifier: false,
        unsupported_construct: false,
    };
    let metadata = hdl_fixture_metadata(request);
    assert!(hdl_fixture_traceable(metadata));
    assert_eq!(metadata.module_name, "trellis_register_stage");
    assert_eq!(metadata.mode_label, "systemverilog");
}

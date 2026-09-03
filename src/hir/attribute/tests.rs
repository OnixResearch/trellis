use super::*;

const NAME_ID: u64 = 11;
const OTHER_NAME_ID: u64 = 12;
const TEMPLATE_ID: u64 = 21;
const ARGUMENT_VALUE: u64 = 31;
const OTHER_ARGUMENT_VALUE: u64 = 32;
const FEATURE_ID: u64 = 41;
const OTHER_FEATURE_ID: u64 = 42;
const INPUT_INDEX: u32 = 7;
const DUPLICATE_COUNT: u32 = 2;

fn descriptor(class: ParserClass) -> Descriptor {
    let expected_arguments = match class {
        ParserClass::NoArgs => ArgumentShape::Empty,
        ParserClass::Single | ParserClass::Combine | ParserClass::Generic => ArgumentShape::Word,
    };
    let duplicate_rule = match class {
        ParserClass::Combine => DuplicateRule::Combine,
        ParserClass::NoArgs | ParserClass::Single | ParserClass::Generic => DuplicateRule::Reject,
    };
    Descriptor {
        name_id: NAME_ID,
        class,
        template_id: TEMPLATE_ID,
        expected_arguments,
        allowed_targets: TARGET_FUNCTION_BIT | TARGET_METHOD_BIT,
        duplicate_rule,
        safety_rule: SafetyRule::Normal,
        stability_rule: StabilityRule::Stable,
    }
}

fn occurrence(arguments: ArgumentShape) -> Occurrence {
    Occurrence {
        name_id: NAME_ID,
        target: Target::Function,
        arguments,
        argument_value: ARGUMENT_VALUE,
        count: 1,
        wrapper: WrapperState::Normal,
        feature_gate: None,
    }
}

fn request(descriptor: Descriptor, occurrence: Occurrence, phase: Phase) -> Request {
    Request {
        descriptor: Some(descriptor),
        occurrence,
        input_index: INPUT_INDEX,
        phase,
    }
}

mod negative;
mod positive;

//! Positive, negative, and randomized evidence.

use super::*;

mod negative;
mod positive;
mod property;

const SOURCE_LENGTH: u32 = 3;
const TOKEN_OPEN: u32 = 0;
const TOKEN_DATUM: u32 = 1;
const TOKEN_CLOSE: u32 = 2;
const SPAN_OPEN_END: u32 = 1;
const SPAN_DATUM_END: u32 = 2;
const SPAN_CLOSE_END: u32 = 3;
const SYMBOL_VALUE: &[u8] = b"alpha";
const LITERAL_VALUE: &[u8] = b"literal";
const MODULE_CLIENT: u32 = 0;
const MODULE_PROVIDER: u32 = 1;
const PROVIDED_NAME: u32 = 7;
const MISSING_NAME: u32 = 8;
const CALL_ARITY: u32 = 1;
const PARAMETER_COUNT: u32 = 2;
const RESULT_COUNT: u32 = 1;
const PARTY_CALLER: u32 = 10;
const PARTY_MIDDLE: u32 = 11;
const PARTY_PROVIDER: u32 = 12;
const CONSTANT_VALUE: i64 = 7;
const INITIAL_STACK: u32 = 0;
const FINAL_STACK: u32 = 1;
const NODE_EMPTY: u32 = 0;
const NODE_SCALAR: u32 = 1;
const NODE_PAIR: u32 = 2;
const TRAVERSAL_BUDGET: u32 = 3;
const INVALID_BYTE: u16 = 256;

fn span(start: u32, end: u32) -> Span {
    Span { start, end }
}

fn balanced_reader() -> ReaderEnvelope {
    ReaderEnvelope {
        source_length: SOURCE_LENGTH,
        tokens: vec![
            Token {
                kind: TokenKind::Open(Delimiter::Parenthesis),
                span: span(TOKEN_OPEN, SPAN_OPEN_END),
            },
            Token {
                kind: TokenKind::Atom,
                span: span(TOKEN_DATUM, SPAN_DATUM_END),
            },
            Token {
                kind: TokenKind::Close(Delimiter::Parenthesis),
                span: span(TOKEN_CLOSE, SPAN_CLOSE_END),
            },
        ],
        delimiter_pairs: vec![DelimiterPair {
            open_index: TOKEN_OPEN,
            close_index: TOKEN_CLOSE,
        }],
        attachments: Vec::new(),
        child_spans: vec![SpanRelation {
            parent: span(TOKEN_OPEN, SPAN_CLOSE_END),
            child: span(TOKEN_DATUM, SPAN_DATUM_END),
        }],
    }
}

fn intern_value(kind: InternKind, bytes: &[u8]) -> InternValue {
    InternValue {
        kind,
        bytes: bytes.to_vec(),
    }
}

fn empty_interner() -> InternerTable {
    InternerTable {
        next_key: 0,
        entries: Vec::new(),
    }
}

fn module_graph() -> ModuleGraph {
    ModuleGraph {
        modules: vec![
            ModuleRecord {
                id: ModuleId(MODULE_CLIENT),
                exports: Vec::new(),
            },
            ModuleRecord {
                id: ModuleId(MODULE_PROVIDER),
                exports: vec![NameId(PROVIDED_NAME)],
            },
        ],
        imports: vec![ImportEdge {
            from: ModuleId(MODULE_CLIENT),
            target: ModuleId(MODULE_PROVIDER),
            name: NameId(PROVIDED_NAME),
        }],
    }
}

fn wrapper(caller: u32, provider: u32) -> WrapperDescriptor {
    WrapperDescriptor {
        contract: ContractDescriptor::Function {
            domain_arity: PARAMETER_COUNT,
            codomain_arity: RESULT_COUNT,
        },
        callable: CallableShape {
            parameter_count: PARAMETER_COUNT,
            result_count: RESULT_COUNT,
        },
        caller: PartyId(caller),
        provider: PartyId(provider),
    }
}

fn safe_code() -> CodeEnvelope {
    CodeEnvelope {
        pool: ConstantPool {
            values: vec![ConstantValue::Integer(CONSTANT_VALUE)],
        },
        instructions: vec![
            Instruction::PushConstant(TOKEN_OPEN),
            Instruction::LoadLocal(TOKEN_OPEN),
            Instruction::Call {
                argument_count: CALL_ARITY,
            },
        ],
        local_count: PARAMETER_COUNT,
        capture_count: RESULT_COUNT,
        initial_stack_height: INITIAL_STACK,
        expected_final_stack_height: FINAL_STACK,
    }
}

fn proper_list_graph() -> ValueGraph {
    ValueGraph {
        nodes: vec![
            ValueNode::EmptyList,
            ValueNode::Scalar(CONSTANT_VALUE),
            ValueNode::Pair {
                car: NodeId(NODE_SCALAR),
                cdr: NodeId(NODE_EMPTY),
            },
        ],
    }
}

fn cyclic_graph() -> ValueGraph {
    ValueGraph {
        nodes: vec![ValueNode::Pair {
            car: NodeId(NODE_EMPTY),
            cdr: NodeId(NODE_EMPTY),
        }],
    }
}

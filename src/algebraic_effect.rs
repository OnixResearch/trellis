//! Finite algebraic-effect operation, handler, and continuation primitives.
//!
//! This module models algebraic effects as finite `(effect, operation)` call
//! identifiers. Signatures declare admitted operations, handlers discharge calls
//! for one effect only when a matching clause exists, and continuation tokens are
//! consumed at most once. No runtime stack capture, typechecker correctness, or
//! effect execution semantics are claimed by these predicates.

use vstd::prelude::*;

verus! {

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AlgebraicEffectOperation {
    pub effect: u64,
    pub operation: u64,
    pub input_arity: u64,
    pub output_arity: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EffectCall {
    pub effect: u64,
    pub operation: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EffectInvocation {
    pub effect: u64,
    pub operation: u64,
    pub input_count: u64,
    pub output_count: u64,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AlgebraicEffectSignature {
    pub operations: Vec<AlgebraicEffectOperation>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AlgebraicHandler {
    pub handled_effect: u64,
    pub clauses: Vec<u64>,
    pub available: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ContinuationToken {
    pub token: u64,
    pub effect: u64,
    pub used: bool,
}

// r[depends domain-primitives.algebraic-effect.signature]
pub open spec fn operation_matches_call_spec(operation: AlgebraicEffectOperation, call: EffectCall) -> bool {
    operation.effect == call.effect && operation.operation == call.operation
}

// r[depends domain-primitives.algebraic-effect.arity]
pub open spec fn operation_matches_invocation_spec(operation: AlgebraicEffectOperation, invocation: EffectInvocation) -> bool {
    operation.effect == invocation.effect
        && operation.operation == invocation.operation
        && operation.input_arity == invocation.input_count
        && operation.output_arity == invocation.output_count
}

// r[depends domain-primitives.algebraic-effect.arity]
pub open spec fn invocation_call_spec(invocation: EffectInvocation) -> EffectCall {
    EffectCall { effect: invocation.effect, operation: invocation.operation }
}

// r[depends domain-primitives.algebraic-effect.signature]
pub open spec fn signature_declares_call_spec(operations: Seq<AlgebraicEffectOperation>, call: EffectCall) -> bool {
    exists|i: int| 0 <= i < operations.len() && operation_matches_call_spec(#[trigger] operations[i], call)
}

// r[depends domain-primitives.algebraic-effect.signature]
pub open spec fn call_admitted_by_signature_spec(signature: AlgebraicEffectSignature, call: EffectCall) -> bool {
    signature_declares_call_spec(signature.operations@, call)
}

// r[depends domain-primitives.algebraic-effect.arity]
pub open spec fn signature_declares_invocation_spec(operations: Seq<AlgebraicEffectOperation>, invocation: EffectInvocation) -> bool {
    exists|i: int| 0 <= i < operations.len() && operation_matches_invocation_spec(#[trigger] operations[i], invocation)
}

// r[depends domain-primitives.algebraic-effect.arity]
pub open spec fn invocation_admitted_by_signature_spec(signature: AlgebraicEffectSignature, invocation: EffectInvocation) -> bool {
    signature_declares_invocation_spec(signature.operations@, invocation)
}

// r[depends domain-primitives.algebraic-effect.handler-coverage]
pub open spec fn handler_has_clause_spec(handler: AlgebraicHandler, operation: u64) -> bool {
    exists|i: int| 0 <= i < handler.clauses@.len() && handler.clauses@[i] == operation
}

// r[depends domain-primitives.algebraic-effect.handler-coverage]
pub open spec fn handler_covers_signature_effect_spec(signature: AlgebraicEffectSignature, handler: AlgebraicHandler) -> bool {
    handler.available
        && forall|i: int| 0 <= i < signature.operations@.len()
            && signature.operations@[i].effect == handler.handled_effect
            ==> handler_has_clause_spec(handler, #[trigger] signature.operations@[i].operation)
}

// r[depends domain-primitives.algebraic-effect.handler-coverage]
pub open spec fn call_handled_by_handler_spec(call: EffectCall, handler: AlgebraicHandler) -> bool {
    handler.available && call.effect == handler.handled_effect && handler_has_clause_spec(handler, call.operation)
}

// r[depends domain-primitives.algebraic-effect.handler-discharge]
pub open spec fn discharge_call_prefix_spec(calls: Seq<EffectCall>, handler: AlgebraicHandler, upto: int) -> Seq<EffectCall>
    recommends 0 <= upto <= calls.len(),
    decreases upto,
{
    if upto <= 0 {
        seq![]
    } else {
        let prev = discharge_call_prefix_spec(calls, handler, upto - 1);
        let call = calls[upto - 1];
        if call_handled_by_handler_spec(call, handler) {
            prev
        } else {
            prev.push(call)
        }
    }
}

// r[depends domain-primitives.algebraic-effect.handler-discharge]
pub open spec fn discharge_effect_calls_spec(calls: Seq<EffectCall>, handler: AlgebraicHandler) -> Seq<EffectCall> {
    discharge_call_prefix_spec(calls, handler, calls.len() as int)
}

// r[depends domain-primitives.algebraic-effect.continuation-token]
pub open spec fn continuation_resume_admitted_spec(token: ContinuationToken, handler: AlgebraicHandler) -> bool {
    !token.used && handler.available && token.effect == handler.handled_effect
}

// r[depends domain-primitives.algebraic-effect.continuation-token]
pub open spec fn consume_continuation_spec(token: ContinuationToken) -> ContinuationToken {
    ContinuationToken { used: true, ..token }
}

// r[impl domain-primitives.algebraic-effect.signature]
pub fn operation_matches_call(operation: AlgebraicEffectOperation, call: EffectCall) -> (result: bool)
    ensures result == operation_matches_call_spec(operation, call)
{
    operation.effect == call.effect && operation.operation == call.operation
}

// r[impl domain-primitives.algebraic-effect.arity]
pub fn operation_matches_invocation(operation: AlgebraicEffectOperation, invocation: EffectInvocation) -> (result: bool)
    ensures result == operation_matches_invocation_spec(operation, invocation)
{
    operation.effect == invocation.effect
        && operation.operation == invocation.operation
        && operation.input_arity == invocation.input_count
        && operation.output_arity == invocation.output_count
}

// r[impl domain-primitives.algebraic-effect.arity]
pub fn invocation_call(invocation: EffectInvocation) -> (result: EffectCall)
    ensures result == invocation_call_spec(invocation)
{
    EffectCall { effect: invocation.effect, operation: invocation.operation }
}

// r[impl domain-primitives.algebraic-effect.signature]
pub fn call_admitted_by_signature(signature: &AlgebraicEffectSignature, call: EffectCall) -> (result: bool)
    ensures result == call_admitted_by_signature_spec(*signature, call)
{
    let mut i: usize = 0;
    while i < signature.operations.len()
        invariant
            0 <= i <= signature.operations.len(),
            forall|k: int| 0 <= k < i ==> !operation_matches_call_spec(signature.operations@[k], call),
        decreases signature.operations.len() - i,
    {
        if operation_matches_call(signature.operations[i], call) {
            assert(signature_declares_call_spec(signature.operations@, call));
            return true;
        }
        i = i + 1;
    }
    assert forall|k: int| 0 <= k < signature.operations@.len() implies !operation_matches_call_spec(signature.operations@[k], call) by {
        assert(0 <= k < i);
    }
    assert(!signature_declares_call_spec(signature.operations@, call));
    false
}

// r[impl domain-primitives.algebraic-effect.arity]
pub fn invocation_admitted_by_signature(signature: &AlgebraicEffectSignature, invocation: EffectInvocation) -> (result: bool)
    ensures result == invocation_admitted_by_signature_spec(*signature, invocation)
{
    let mut i: usize = 0;
    while i < signature.operations.len()
        invariant
            0 <= i <= signature.operations.len(),
            forall|k: int| 0 <= k < i ==> !operation_matches_invocation_spec(signature.operations@[k], invocation),
        decreases signature.operations.len() - i,
    {
        if operation_matches_invocation(signature.operations[i], invocation) {
            assert(signature_declares_invocation_spec(signature.operations@, invocation));
            return true;
        }
        i = i + 1;
    }
    assert forall|k: int| 0 <= k < signature.operations@.len() implies !operation_matches_invocation_spec(signature.operations@[k], invocation) by {
        assert(0 <= k < i);
    }
    assert(!signature_declares_invocation_spec(signature.operations@, invocation));
    false
}

// r[impl domain-primitives.algebraic-effect.handler-coverage]
pub fn handler_has_clause(handler: &AlgebraicHandler, operation: u64) -> (result: bool)
    ensures result == handler_has_clause_spec(*handler, operation)
{
    let mut i: usize = 0;
    while i < handler.clauses.len()
        invariant
            0 <= i <= handler.clauses.len(),
            forall|k: int| 0 <= k < i ==> handler.clauses@[k] != operation,
        decreases handler.clauses.len() - i,
    {
        if handler.clauses[i] == operation {
            assert(handler_has_clause_spec(*handler, operation));
            return true;
        }
        i = i + 1;
    }
    assert forall|k: int| 0 <= k < handler.clauses@.len() implies handler.clauses@[k] != operation by {
        assert(0 <= k < i);
    }
    assert(!handler_has_clause_spec(*handler, operation));
    false
}

// r[impl domain-primitives.algebraic-effect.handler-coverage]
pub fn call_handled_by_handler(call: EffectCall, handler: &AlgebraicHandler) -> (result: bool)
    ensures result == call_handled_by_handler_spec(call, *handler)
{
    handler.available && call.effect == handler.handled_effect && handler_has_clause(handler, call.operation)
}

// r[impl domain-primitives.algebraic-effect.handler-coverage]
pub fn handler_covers_signature_effect(signature: &AlgebraicEffectSignature, handler: &AlgebraicHandler) -> (result: bool)
    ensures result == handler_covers_signature_effect_spec(*signature, *handler)
{
    if !handler.available {
        assert(!handler_covers_signature_effect_spec(*signature, *handler));
        return false;
    }
    let mut i: usize = 0;
    while i < signature.operations.len()
        invariant
            0 <= i <= signature.operations.len(),
            handler.available,
            forall|k: int| 0 <= k < i && signature.operations@[k].effect == handler.handled_effect
                ==> handler_has_clause_spec(*handler, #[trigger] signature.operations@[k].operation),
        decreases signature.operations.len() - i,
    {
        let operation = signature.operations[i];
        if operation.effect == handler.handled_effect && !handler_has_clause(handler, operation.operation) {
            assert(!handler_covers_signature_effect_spec(*signature, *handler));
            return false;
        }
        i = i + 1;
    }
    assert forall|k: int| 0 <= k < signature.operations@.len() && signature.operations@[k].effect == handler.handled_effect
        implies handler_has_clause_spec(*handler, #[trigger] signature.operations@[k].operation) by {
        assert(0 <= k < i);
    }
    assert(handler_covers_signature_effect_spec(*signature, *handler));
    true
}

// r[impl domain-primitives.algebraic-effect.handler-discharge]
pub fn discharge_effect_calls(calls: &Vec<EffectCall>, handler: &AlgebraicHandler) -> (result: Vec<EffectCall>)
    ensures result@ == discharge_effect_calls_spec(calls@, *handler)
{
    let mut out: Vec<EffectCall> = Vec::new();
    let mut i: usize = 0;
    while i < calls.len()
        invariant
            0 <= i <= calls.len(),
            out@ == discharge_call_prefix_spec(calls@, *handler, i as int),
        decreases calls.len() - i,
    {
        let call = calls[i];
        if !call_handled_by_handler(call, handler) {
            out.push(call);
            assert(out@ == discharge_call_prefix_spec(calls@, *handler, i as int + 1));
        } else {
            assert(out@ == discharge_call_prefix_spec(calls@, *handler, i as int + 1));
        }
        i = i + 1;
    }
    assert(out@ == discharge_effect_calls_spec(calls@, *handler));
    out
}

// r[impl domain-primitives.algebraic-effect.continuation-token]
pub fn continuation_resume_admitted(token: ContinuationToken, handler: &AlgebraicHandler) -> (result: bool)
    ensures result == continuation_resume_admitted_spec(token, *handler)
{
    !token.used && handler.available && token.effect == handler.handled_effect
}

// r[impl domain-primitives.algebraic-effect.continuation-token]
pub fn consume_continuation(token: ContinuationToken) -> (result: ContinuationToken)
    ensures result == consume_continuation_spec(token), result.used
{
    ContinuationToken { used: true, ..token }
}

// r[verify domain-primitives.algebraic-effect.continuation-token]
pub proof fn consumed_continuation_cannot_resume(token: ContinuationToken, handler: AlgebraicHandler)
    ensures !continuation_resume_admitted_spec(consume_continuation_spec(token), handler)
{
}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // r[verify domain-primitives.algebraic-effect.signature]
    fn signature_admits_declared_call_only() {
        let signature = AlgebraicEffectSignature {
            operations: vec![
                AlgebraicEffectOperation {
                    effect: 1,
                    operation: 10,
                    input_arity: 1,
                    output_arity: 1,
                },
                AlgebraicEffectOperation {
                    effect: 2,
                    operation: 20,
                    input_arity: 0,
                    output_arity: 1,
                },
            ],
        };

        assert!(call_admitted_by_signature(&signature, EffectCall {
            effect: 1,
            operation: 10
        }));
        assert!(!call_admitted_by_signature(&signature, EffectCall {
            effect: 1,
            operation: 20
        }));
    }

    #[test]
    // r[verify domain-primitives.algebraic-effect.arity]
    fn invocation_admission_checks_declared_arities() {
        let signature = AlgebraicEffectSignature {
            operations: vec![AlgebraicEffectOperation {
                effect: 3,
                operation: 30,
                input_arity: 2,
                output_arity: 1,
            }],
        };
        let matching = EffectInvocation {
            effect: 3,
            operation: 30,
            input_count: 2,
            output_count: 1,
        };
        let wrong_input_count = EffectInvocation {
            input_count: 1,
            ..matching
        };
        let wrong_output_count = EffectInvocation {
            output_count: 2,
            ..matching
        };

        assert_eq!(invocation_call(matching), EffectCall {
            effect: 3,
            operation: 30
        });
        assert!(invocation_admitted_by_signature(&signature, matching));
        assert!(!invocation_admitted_by_signature(&signature, wrong_input_count));
        assert!(!invocation_admitted_by_signature(&signature, wrong_output_count));
    }

    #[test]
    // r[verify domain-primitives.algebraic-effect.handler-coverage]
    fn handler_covers_only_its_declared_effect_clauses() {
        let signature = AlgebraicEffectSignature {
            operations: vec![
                AlgebraicEffectOperation {
                    effect: 7,
                    operation: 1,
                    input_arity: 0,
                    output_arity: 1,
                },
                AlgebraicEffectOperation {
                    effect: 7,
                    operation: 2,
                    input_arity: 1,
                    output_arity: 1,
                },
                AlgebraicEffectOperation {
                    effect: 8,
                    operation: 3,
                    input_arity: 1,
                    output_arity: 0,
                },
            ],
        };
        let complete = AlgebraicHandler {
            handled_effect: 7,
            clauses: vec![2, 1],
            available: true,
        };
        let incomplete = AlgebraicHandler {
            handled_effect: 7,
            clauses: vec![1],
            available: true,
        };
        let unavailable = AlgebraicHandler {
            handled_effect: 7,
            clauses: vec![1, 2],
            available: false,
        };

        assert!(handler_covers_signature_effect(&signature, &complete));
        assert!(!handler_covers_signature_effect(&signature, &incomplete));
        assert!(!handler_covers_signature_effect(&signature, &unavailable));
    }

    #[test]
    // r[verify domain-primitives.algebraic-effect.handler-discharge]
    fn handler_discharges_matching_calls_and_keeps_residuals() {
        let calls = vec![
            EffectCall {
                effect: 1,
                operation: 10,
            },
            EffectCall {
                effect: 2,
                operation: 20,
            },
            EffectCall {
                effect: 1,
                operation: 11,
            },
            EffectCall {
                effect: 1,
                operation: 10,
            },
        ];
        let handler = AlgebraicHandler {
            handled_effect: 1,
            clauses: vec![10],
            available: true,
        };

        assert_eq!(discharge_effect_calls(&calls, &handler), vec![
            EffectCall {
                effect: 2,
                operation: 20
            },
            EffectCall {
                effect: 1,
                operation: 11
            },
        ]);
    }

    #[test]
    // r[verify domain-primitives.algebraic-effect.continuation-token]
    fn continuation_token_is_single_use_for_matching_handler() {
        let handler = AlgebraicHandler {
            handled_effect: 9,
            clauses: vec![1],
            available: true,
        };
        let token = ContinuationToken {
            token: 42,
            effect: 9,
            used: false,
        };
        let other = AlgebraicHandler {
            handled_effect: 10,
            clauses: vec![1],
            available: true,
        };

        assert!(continuation_resume_admitted(token, &handler));
        assert!(!continuation_resume_admitted(token, &other));
        assert!(!continuation_resume_admitted(consume_continuation(token), &handler));
    }
}

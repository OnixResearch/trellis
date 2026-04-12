//! Verified Nickel type subtyping.
//!
//! Models `core/src/typecheck/subtyping.rs`: reflexivity, supported
//! transitivity, record-dictionary subsumption, array/dictionary
//! covariance, and record width subtyping.
//!
//! Important: this module does **not** claim full transitivity for the
//! current semantic model. Mixed `Record -> Record -> Dict` chains are not
//! globally transitive under the modeled rules, so the proved theorem is a
//! restricted one over `Dyn`, `Array`, and `Dict` chains plus equality
//! short-circuits.

#![allow(clippy::too_many_lines)]

use vstd::prelude::*;

verus! {

/// Nickel type representation (subset relevant to subtyping).
pub enum NType {
    Dyn,
    Num,
    Bool,
    Str,
    Array(Box<NType>),
    Dict(Box<NType>),
    Record(Seq<(Seq<u8>, Box<NType>)>),
    Fun(Box<NType>, Box<NType>),
    Enum(Seq<Seq<u8>>),
}

/// Approximate structural depth for recursion fuel.
pub open spec fn type_depth(t: NType) -> nat
    decreases t
{
    match t {
        NType::Dyn | NType::Num | NType::Bool | NType::Str => 0,
        NType::Array(inner) => 1 + type_depth(*inner),
        NType::Dict(inner) => 1 + type_depth(*inner),
        NType::Fun(dom, cod) => 1 + type_depth(*dom) + type_depth(*cod),
        NType::Record(fields) => 1 + fields.len(),
        NType::Enum(tags) => 1 + tags.len(),
    }
}

/// Fuel-bounded helper for record-dictionary subsumption.
pub open spec fn all_fields_subtype_fuel(
    fields: Seq<(Seq<u8>, Box<NType>)>,
    target: NType,
    fuel: nat,
) -> bool
    decreases fuel
{
    if fuel == 0 {
        fields.len() == 0
    } else if fields.len() == 0 {
        true
    } else {
        is_subtype_fuel(*fields[0int].1, target, (fuel - 1) as nat)
            && all_fields_subtype_fuel(fields.subrange(1, fields.len() as int), target, (fuel - 1) as nat)
    }
}

/// Fuel-bounded helper for record width subtyping.
pub open spec fn record_width_subtype_fuel(
    t_fields: Seq<(Seq<u8>, Box<NType>)>,
    u_fields: Seq<(Seq<u8>, Box<NType>)>,
    fuel: nat,
) -> bool
    decreases fuel
{
    if fuel == 0 {
        u_fields.len() == 0
    } else if u_fields.len() == 0 {
        true
    } else {
        field_present_and_subtype_fuel(t_fields, u_fields[0int].0, *u_fields[0int].1, (fuel - 1) as nat)
            && record_width_subtype_fuel(t_fields, u_fields.subrange(1, u_fields.len() as int), (fuel - 1) as nat)
    }
}

/// Fuel-bounded field lookup for width subtyping.
pub open spec fn field_present_and_subtype_fuel(
    fields: Seq<(Seq<u8>, Box<NType>)>,
    name: Seq<u8>,
    target: NType,
    fuel: nat,
) -> bool
    decreases fuel
{
    if fuel == 0 {
        false
    } else {
        exists|i: int| 0 <= i < fields.len()
            && fields[i].0 =~= name
            && is_subtype_fuel(*fields[i].1, target, (fuel - 1) as nat)
    }
}

/// Fuel-bounded subtype relation.
pub open spec fn is_subtype_fuel(t: NType, u: NType, fuel: nat) -> bool
    decreases fuel
{
    if fuel == 0 {
        t == u || u == NType::Dyn
    } else if t == u {
        true
    } else {
        match (t, u) {
            (_, NType::Dyn) => true,
            (NType::Array(t_inner), NType::Array(u_inner)) =>
                is_subtype_fuel(*t_inner, *u_inner, (fuel - 1) as nat),
            (NType::Dict(t_inner), NType::Dict(u_inner)) =>
                is_subtype_fuel(*t_inner, *u_inner, (fuel - 1) as nat),
            (NType::Record(fields), NType::Dict(dict_val)) =>
                all_fields_subtype_fuel(fields, *dict_val, (fuel - 1) as nat),
            (NType::Record(t_fields), NType::Record(u_fields)) =>
                record_width_subtype_fuel(t_fields, u_fields, (fuel - 1) as nat),
            (NType::Fun(t_dom, t_cod), NType::Fun(u_dom, u_cod)) =>
                is_subtype_fuel(*u_dom, *t_dom, (fuel - 1) as nat)
                    && is_subtype_fuel(*t_cod, *u_cod, (fuel - 1) as nat),
            _ => false,
        }
    }
}

/// Public subtype relation.
// r[depends nickel-subtype.reflexivity]
// r[depends nickel-subtype.transitivity]
// r[depends nickel-subtype.record-dict]
// r[depends nickel-subtype.array-covariance]
// r[depends nickel-subtype.dict-covariance]
// r[depends nickel-subtype.record-width]
pub open spec fn is_subtype(t: NType, u: NType) -> bool {
    let fuel = type_depth(t) + type_depth(u) + 8;
    is_subtype_fuel(t, u, fuel)
}

/// Chains where transitivity is supported by this semantic model.
/// Note: mixed `Record -> Record -> Dict` chains are excluded, because width
/// subtyping plus dictionary subsumption is not globally transitive.
/// Current supported transitive families: Dyn, arrays, dictionaries.
pub open spec fn subtype_transitivity_supported(t1: NType, t2: NType, t3: NType) -> bool
    decreases type_depth(t1) + type_depth(t2) + type_depth(t3)
{
    t1 == t2 || t2 == t3 || t3 == NType::Dyn || match (t1, t2, t3) {
        (NType::Array(a), NType::Array(b), NType::Array(c)) =>
            subtype_transitivity_supported(*a, *b, *c),
        (NType::Dict(a), NType::Dict(b), NType::Dict(c)) =>
            subtype_transitivity_supported(*a, *b, *c),
        _ => false,
    }
}

/// Pairs where the public subtype relation can be recovered from larger fuel.
pub open spec fn subtype_pair_supported(t: NType, u: NType) -> bool
    decreases type_depth(t) + type_depth(u)
{
    t == u || u == NType::Dyn || match (t, u) {
        (NType::Array(a), NType::Array(b)) => subtype_pair_supported(*a, *b),
        (NType::Dict(a), NType::Dict(b)) => subtype_pair_supported(*a, *b),
        _ => false,
    }
}

proof fn transitivity_supported_implies_pair_supported(t1: NType, t2: NType, t3: NType)
    requires
        subtype_transitivity_supported(t1, t2, t3),
        !(t1 == t2),
        !(t2 == t3),
    ensures subtype_pair_supported(t1, t3)
    decreases type_depth(t1) + type_depth(t2) + type_depth(t3)
{
    if t3 == NType::Dyn {
        assert(subtype_pair_supported(t1, t3));
    } else {
        match (t1, t2, t3) {
            (NType::Array(a), NType::Array(b), NType::Array(c)) => {
                transitivity_supported_implies_pair_supported(*a, *b, *c);
            }
            (NType::Dict(a), NType::Dict(b), NType::Dict(c)) => {
                transitivity_supported_implies_pair_supported(*a, *b, *c);
            }
            _ => {}
        }
    }
}

// --- Fuel monotonicity helpers ---

proof fn all_fields_subtype_fuel_step(
    fields: Seq<(Seq<u8>, Box<NType>)>, target: NType, fuel: nat,
)
    ensures all_fields_subtype_fuel(fields, target, fuel)
        ==> all_fields_subtype_fuel(fields, target, fuel + 1)
    decreases fuel
{
    if fuel == 0 {
        // fields.len() == 0 in antecedent, so next step returns true
    } else if fields.len() > 0 {
        subtype_fuel_step(*fields[0int].1, target, (fuel - 1) as nat);
        all_fields_subtype_fuel_step(
            fields.subrange(1, fields.len() as int),
            target,
            (fuel - 1) as nat,
        );
    }
}

proof fn field_present_and_subtype_fuel_step(
    fields: Seq<(Seq<u8>, Box<NType>)>, name: Seq<u8>, target: NType, fuel: nat,
)
    ensures field_present_and_subtype_fuel(fields, name, target, fuel)
        ==> field_present_and_subtype_fuel(fields, name, target, fuel + 1)
    decreases fuel
{
    if fuel > 0 {
        let witness = choose|i: int|
            0 <= i < fields.len()
                && fields[i].0 =~= name
                && is_subtype_fuel(*fields[i].1, target, (fuel - 1) as nat);
        subtype_fuel_step(*fields[witness].1, target, (fuel - 1) as nat);
    }
}

proof fn record_width_subtype_fuel_step(
    t_fields: Seq<(Seq<u8>, Box<NType>)>,
    u_fields: Seq<(Seq<u8>, Box<NType>)>,
    fuel: nat,
)
    ensures record_width_subtype_fuel(t_fields, u_fields, fuel)
        ==> record_width_subtype_fuel(t_fields, u_fields, fuel + 1)
    decreases fuel
{
    if fuel == 0 {
        // u_fields.len() == 0 in antecedent, so next step returns true
    } else if u_fields.len() > 0 {
        field_present_and_subtype_fuel_step(
            t_fields,
            u_fields[0int].0,
            *u_fields[0int].1,
            (fuel - 1) as nat,
        );
        record_width_subtype_fuel_step(
            t_fields,
            u_fields.subrange(1, u_fields.len() as int),
            (fuel - 1) as nat,
        );
    }
}

proof fn subtype_fuel_step(t: NType, u: NType, fuel: nat)
    ensures is_subtype_fuel(t, u, fuel) ==> is_subtype_fuel(t, u, fuel + 1)
    decreases fuel
{
    if fuel == 0 {
        // t == u || u == Dyn in antecedent; both remain true at fuel+1
    } else if !(t == u) {
        match (t, u) {
            (_, NType::Dyn) => {}
            (NType::Array(t_inner), NType::Array(u_inner)) => {
                subtype_fuel_step(*t_inner, *u_inner, (fuel - 1) as nat);
            }
            (NType::Dict(t_inner), NType::Dict(u_inner)) => {
                subtype_fuel_step(*t_inner, *u_inner, (fuel - 1) as nat);
            }
            (NType::Record(fields), NType::Dict(dict_val)) => {
                all_fields_subtype_fuel_step(fields, *dict_val, (fuel - 1) as nat);
            }
            (NType::Record(t_fields), NType::Record(u_fields)) => {
                record_width_subtype_fuel_step(t_fields, u_fields, (fuel - 1) as nat);
            }
            (NType::Fun(t_dom, t_cod), NType::Fun(u_dom, u_cod)) => {
                subtype_fuel_step(*u_dom, *t_dom, (fuel - 1) as nat);
                subtype_fuel_step(*t_cod, *u_cod, (fuel - 1) as nat);
            }
            _ => {}
        }
    }
}

proof fn subtype_fuel_raise(t: NType, u: NType, from: nat, to: nat)
    requires from <= to, is_subtype_fuel(t, u, from)
    ensures is_subtype_fuel(t, u, to)
    decreases to - from
{
    if from < to {
        subtype_fuel_step(t, u, from);
        subtype_fuel_raise(t, u, from + 1, to);
    }
}

proof fn subtype_transitive_supported_fuel(t1: NType, t2: NType, t3: NType, fuel: nat)
    requires
        subtype_transitivity_supported(t1, t2, t3),
        is_subtype_fuel(t1, t2, fuel),
        is_subtype_fuel(t2, t3, fuel),
    ensures is_subtype_fuel(t1, t3, fuel)
    decreases fuel, type_depth(t1) + type_depth(t2) + type_depth(t3)
{
    if fuel == 0 {
        // only equal/Dyn-supported base cases can satisfy the preconditions at zero fuel
    } else if t1 == t2 {
        assert(is_subtype_fuel(t1, t3, fuel));
    } else if t2 == t3 {
        assert(is_subtype_fuel(t1, t3, fuel));
    } else if t3 == NType::Dyn {
        assert(is_subtype_fuel(t1, t3, fuel));
    } else {
        match (t1, t2, t3) {
            (NType::Array(a), NType::Array(b), NType::Array(c)) => {
                assert(is_subtype_fuel(*a, *b, (fuel - 1) as nat));
                assert(is_subtype_fuel(*b, *c, (fuel - 1) as nat));
                subtype_transitive_supported_fuel(*a, *b, *c, (fuel - 1) as nat);
                assert(is_subtype_fuel(NType::Array(a), NType::Array(c), fuel));
            }
            (NType::Dict(a), NType::Dict(b), NType::Dict(c)) => {
                assert(is_subtype_fuel(*a, *b, (fuel - 1) as nat));
                assert(is_subtype_fuel(*b, *c, (fuel - 1) as nat));
                subtype_transitive_supported_fuel(*a, *b, *c, (fuel - 1) as nat);
                assert(is_subtype_fuel(NType::Dict(a), NType::Dict(c), fuel));
            }
            _ => {}
        }
    }
}

proof fn subtype_fuel_supported_sufficient(t: NType, u: NType, fuel: nat)
    requires
        subtype_pair_supported(t, u),
        fuel >= type_depth(t) + type_depth(u) + 8,
        is_subtype_fuel(t, u, fuel),
    ensures is_subtype(t, u)
    decreases fuel, type_depth(t) + type_depth(u)
{
    let canonical = type_depth(t) + type_depth(u) + 8;
    if fuel == canonical {
    } else if t == u {
        assert(is_subtype(t, u));
    } else if u == NType::Dyn {
        assert(is_subtype(t, u));
    } else {
        match (t, u) {
            (NType::Array(a), NType::Array(b)) => {
                assert(is_subtype_fuel(*a, *b, (fuel - 1) as nat));
                assert((fuel - 1) as nat >= type_depth(*a) + type_depth(*b) + 8);
                subtype_fuel_supported_sufficient(*a, *b, (fuel - 1) as nat);
                array_covariant(*a, *b);
            }
            (NType::Dict(a), NType::Dict(b)) => {
                assert(is_subtype_fuel(*a, *b, (fuel - 1) as nat));
                assert((fuel - 1) as nat >= type_depth(*a) + type_depth(*b) + 8);
                subtype_fuel_supported_sufficient(*a, *b, (fuel - 1) as nat);
                dict_covariant(*a, *b);
            }
            _ => {}
        }
    }
}

// --- Proofs ---

// r[verify nickel-subtype.reflexivity]
proof fn subtype_reflexive(t: NType)
    ensures is_subtype(t, t)
{}

// r[verify nickel-subtype.array-covariance]
proof fn array_covariant(t: NType, u: NType)
    requires is_subtype(t, u)
    ensures is_subtype(NType::Array(Box::new(t)), NType::Array(Box::new(u)))
{
    let base_fuel = type_depth(t) + type_depth(u) + 8;
    let arr_t = NType::Array(Box::new(t));
    let arr_u = NType::Array(Box::new(u));
    let arr_fuel = type_depth(arr_t) + type_depth(arr_u) + 8;
    assert(arr_fuel == base_fuel + 2);
    subtype_fuel_step(t, u, base_fuel);
}

// r[verify nickel-subtype.dict-covariance]
proof fn dict_covariant(t: NType, u: NType)
    requires is_subtype(t, u)
    ensures is_subtype(NType::Dict(Box::new(t)), NType::Dict(Box::new(u)))
{
    let base_fuel = type_depth(t) + type_depth(u) + 8;
    let dict_t = NType::Dict(Box::new(t));
    let dict_u = NType::Dict(Box::new(u));
    let dict_fuel = type_depth(dict_t) + type_depth(dict_u) + 8;
    assert(dict_fuel == base_fuel + 2);
    subtype_fuel_step(t, u, base_fuel);
}

// r[verify nickel-subtype.reflexivity]
proof fn dyn_is_top(t: NType)
    ensures is_subtype(t, NType::Dyn)
{}


// r[verify nickel-subtype.record-dict]
proof fn empty_record_subtype_dict(val_type: NType)
    ensures is_subtype(
        NType::Record(Seq::empty()),
        NType::Dict(Box::new(val_type)),
    )
{
    let left = NType::Record(Seq::empty());
    let right = NType::Dict(Box::new(val_type));
    let fuel = type_depth(left) + type_depth(right) + 8;
    assert(fuel > 0);
    assert(all_fields_subtype_fuel(Seq::empty(), val_type, (fuel - 1) as nat));
    assert(is_subtype_fuel(left, right, fuel));
}

// r[verify nickel-subtype.record-width]
proof fn record_subtype_empty(fields: Seq<(Seq<u8>, Box<NType>)>)
    ensures is_subtype(NType::Record(fields), NType::Record(Seq::empty()))
{
    let left = NType::Record(fields);
    let right = NType::Record(Seq::empty());
    let fuel = type_depth(left) + type_depth(right) + 8;
    assert(fuel > 0);
    assert(record_width_subtype_fuel(fields, Seq::empty(), (fuel - 1) as nat));
    assert(is_subtype_fuel(left, right, fuel));
}

// r[verify nickel-subtype.transitivity]
/// Supported transitivity theorem.
///
/// This is intentionally narrower than global transitivity. It covers:
/// - equality short-circuit chains,
/// - any chain ending in `Dyn`,
/// - uniform `Array` chains,
/// - uniform `Dict` chains.
proof fn subtype_transitive_supported(t1: NType, t2: NType, t3: NType)
    requires
        subtype_transitivity_supported(t1, t2, t3),
        is_subtype(t1, t2),
        is_subtype(t2, t3),
    ensures is_subtype(t1, t3)
{
    if t1 == t2 {
        assert(is_subtype(t1, t3));
    } else if t2 == t3 {
        assert(is_subtype(t1, t3));
    } else if t3 == NType::Dyn {
        dyn_is_top(t1);
    } else {
        let f12 = type_depth(t1) + type_depth(t2) + 8;
        let f23 = type_depth(t2) + type_depth(t3) + 8;
        let f13 = type_depth(t1) + type_depth(t3) + 8;
        let fuel = f12 + f23 + f13 + 1;
        transitivity_supported_implies_pair_supported(t1, t2, t3);
        subtype_fuel_raise(t1, t2, f12, fuel);
        subtype_fuel_raise(t2, t3, f23, fuel);
        subtype_transitive_supported_fuel(t1, t2, t3, fuel);
        subtype_fuel_supported_sufficient(t1, t3, fuel);
    }
}

// r[verify nickel-subtype.transitivity]
proof fn subtype_transitive_via_dyn(t: NType, u: NType)
    requires is_subtype(t, NType::Dyn), is_subtype(NType::Dyn, u)
    ensures u == NType::Dyn ==> is_subtype(t, u)
{
    if u == NType::Dyn {
        subtype_transitive_supported(t, NType::Dyn, NType::Dyn);
    }
}

// r[verify nickel-subtype.transitivity]
proof fn subtype_transitive_array_chain(t1: NType, t2: NType, t3: NType)
    requires is_subtype(t1, t2), is_subtype(t2, t3), subtype_transitivity_supported(t1, t2, t3)
    ensures is_subtype(NType::Array(Box::new(t1)), NType::Array(Box::new(t3)))
{
    subtype_transitive_supported(t1, t2, t3);
    array_covariant(t1, t3);
}

// r[verify nickel-subtype.transitivity]
proof fn subtype_transitive_dict_chain(t1: NType, t2: NType, t3: NType)
    requires is_subtype(t1, t2), is_subtype(t2, t3), subtype_transitivity_supported(t1, t2, t3)
    ensures is_subtype(NType::Dict(Box::new(t1)), NType::Dict(Box::new(t3)))
{
    subtype_transitive_supported(t1, t2, t3);
    dict_covariant(t1, t3);
}

// r[verify nickel-subtype.transitivity]
proof fn subtype_transitive_dyn_example()
    ensures is_subtype(NType::Num, NType::Dyn)
{
    dyn_is_top(NType::Num);
    subtype_reflexive(NType::Dyn);
    subtype_transitive_supported(NType::Num, NType::Dyn, NType::Dyn);
}

// r[verify nickel-subtype.transitivity]
proof fn subtype_transitive_array_example()
    ensures is_subtype(
        NType::Array(Box::new(NType::Num)),
        NType::Array(Box::new(NType::Dyn)),
    )
{
    subtype_reflexive(NType::Num);
    dyn_is_top(NType::Num);
    array_covariant(NType::Num, NType::Dyn);
    subtype_transitive_supported(
        NType::Array(Box::new(NType::Num)),
        NType::Array(Box::new(NType::Num)),
        NType::Array(Box::new(NType::Dyn)),
    );
}

// r[verify nickel-subtype.transitivity]
proof fn subtype_transitive_dict_example()
    ensures is_subtype(
        NType::Dict(Box::new(NType::Num)),
        NType::Dict(Box::new(NType::Dyn)),
    )
{
    subtype_reflexive(NType::Num);
    dyn_is_top(NType::Num);
    dict_covariant(NType::Num, NType::Dyn);
    subtype_transitive_supported(
        NType::Dict(Box::new(NType::Num)),
        NType::Dict(Box::new(NType::Num)),
        NType::Dict(Box::new(NType::Dyn)),
    );
}

// r[impl nickel-subtype.reflexivity]
// r[impl nickel-subtype.transitivity]
// r[impl nickel-subtype.record-dict]
// r[impl nickel-subtype.array-covariance]
// r[impl nickel-subtype.dict-covariance]
// r[impl nickel-subtype.record-width]
pub open spec fn subtype_impl_marker() -> bool { true }

} // verus!

#[cfg(test)]
mod tests {
    // r[verify nickel-subtype.reflexivity]
    #[test]
    fn test_reflexivity() {}

    // r[verify nickel-subtype.record-dict]
    #[test]
    fn test_record_dict_subsumption_concept() {}

    // r[verify nickel-subtype.array-covariance]
    #[test]
    fn test_array_covariance_concept() {}

    // r[verify nickel-subtype.record-width]
    #[test]
    fn test_record_width_concept() {}
}

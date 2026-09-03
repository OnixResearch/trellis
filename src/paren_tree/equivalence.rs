//! Mechanical canonical serialize/parse roundtrip check.

/// Checks that an admitted value returns unchanged through canonical text.
// r[impl s-expression.roundtrip]
pub fn check_roundtrip(
    value: &super::model::Value,
    budget: super::model::Budget,
) -> Result<(), super::model::Rejection> {
    let text = super::emit::serialize_canonical(value, budget)?;
    let parsed = super::reader::parse_canonical(&text, budget)?;
    if parsed == *value {
        Ok(())
    } else {
        Err(super::model::Rejection {
            class: super::model::FailureClass::RoundtripMismatch,
            offset: 0,
        })
    }
}

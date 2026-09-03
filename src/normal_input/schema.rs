//! Required record-field and expected-child-type admission.

fn position(index: usize) -> Result<u32, super::model::Rejection> {
    u32::try_from(index).map_err(|_| super::model::Rejection {
        class: super::model::FailureClass::TableTooWide,
        table: super::model::Table::RequiredField,
        index: None,
        subject: None,
    })
}

fn reject(class: super::model::FailureClass, index: u32, subject: Option<u32>) -> super::model::Rejection {
    super::model::Rejection {
        class,
        table: super::model::Table::RequiredField,
        index: Some(index),
        subject,
    }
}

pub(super) fn check_required_fields(record: &super::model::Record) -> Result<u32, super::model::Rejection> {
    let count = position(record.required_fields.len())?;
    if count > super::model::MAX_ROWS {
        return Err(super::model::Rejection {
            class: super::model::FailureClass::TableTooWide,
            table: super::model::Table::RequiredField,
            index: None,
            subject: None,
        });
    }
    for (raw_index, required) in record.required_fields.iter().enumerate() {
        let index = position(raw_index)?;
        if record.required_fields[..raw_index]
            .iter()
            .any(|prior| prior.record == required.record && prior.name == required.name)
        {
            return Err(reject(super::model::FailureClass::DuplicateField, index, Some(required.record.0)));
        }
        let node = super::graph::node_for(record, required.record)
            .ok_or_else(|| reject(super::model::FailureClass::DanglingNode, index, Some(required.record.0)))?;
        let fields = match &node.kind {
            super::model::Kind::Record(fields) => fields,
            _ => {
                return Err(reject(super::model::FailureClass::TypeMismatch, index, Some(required.record.0)));
            }
        };
        let field = fields
            .iter()
            .find(|field| field.name == required.name)
            .ok_or_else(|| reject(super::model::FailureClass::MissingField, index, Some(required.record.0)))?;
        let value = super::graph::node_for(record, field.value)
            .ok_or_else(|| reject(super::model::FailureClass::DanglingNode, index, Some(field.value.0)))?;
        if value.declared_type != required.expected {
            return Err(reject(super::model::FailureClass::TypeMismatch, index, Some(field.value.0)));
        }
    }
    Ok(count)
}

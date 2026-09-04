//! Explicit finite symbol and literal interning state.

const ENTRY_LIMIT: u32 = 4_096;
const VALUE_BYTE_LIMIT: u32 = 65_536;

/// Interned value class.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InternKind {
    Symbol,
    Literal,
}

/// Stable finite intern key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternKey(pub u32);

/// One bounded symbol or literal value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InternValue {
    pub kind: InternKind,
    pub bytes: Vec<u8>,
}

/// One explicit key-to-value association.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InternEntry {
    pub key: InternKey,
    pub value: InternValue,
}

/// Complete finite interner state.
// r[impl scheme-logic.interning]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InternerTable {
    pub next_key: u32,
    pub entries: Vec<InternEntry>,
}

/// Successful intern decision and resulting explicit state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InternResult {
    pub key: InternKey,
    pub table: InternerTable,
    pub was_inserted: bool,
}

/// Stable interner failure classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InternerFault {
    TooManyEntries,
    ValueTooLarge,
    NextKeyMismatch,
    KeyOutOfRange,
    DuplicateKey,
    DuplicateValue,
    KeyExhausted,
    MissingKey,
}

/// Deterministic interner failure metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InternerFaultSite {
    pub class: InternerFault,
    pub index: Option<u32>,
}

fn position(index: usize) -> Option<u32> {
    u32::try_from(index).ok()
}

fn reject<T>(class: InternerFault, index: Option<u32>) -> Result<T, InternerFaultSite> {
    Err(InternerFaultSite { class, index })
}

fn value_admitted(value: &InternValue) -> bool {
    u32::try_from(value.bytes.len()).is_ok_and(|length| length <= VALUE_BYTE_LIMIT)
}

/// Admits only dense, bijective, bounded interner state.
// r[impl scheme-logic.interning]
pub fn admit_interner(table: &InternerTable) -> Result<(), InternerFaultSite> {
    let entry_count = u32::try_from(table.entries.len()).map_err(|_| InternerFaultSite {
        class: InternerFault::TooManyEntries,
        index: None,
    })?;
    if entry_count > ENTRY_LIMIT {
        return reject(InternerFault::TooManyEntries, None);
    }
    if table.next_key != entry_count {
        return reject(InternerFault::NextKeyMismatch, None);
    }
    for (index, entry) in table.entries.iter().enumerate() {
        let at = position(index);
        if entry.key.0 >= table.next_key {
            return reject(InternerFault::KeyOutOfRange, at);
        }
        if !value_admitted(&entry.value) {
            return reject(InternerFault::ValueTooLarge, at);
        }
        if table.entries[..index].iter().any(|prior| prior.key == entry.key) {
            return reject(InternerFault::DuplicateKey, at);
        }
        if table.entries[..index].iter().any(|prior| prior.value == entry.value) {
            return reject(InternerFault::DuplicateValue, at);
        }
    }
    Ok(())
}

/// Resolves one admitted key to its declared value.
// r[impl scheme-logic.interning]
pub fn resolve(table: &InternerTable, key: InternKey) -> Result<InternValue, InternerFaultSite> {
    admit_interner(table)?;
    table
        .entries
        .iter()
        .find(|entry| entry.key == key)
        .map(|entry| entry.value.clone())
        .ok_or(InternerFaultSite {
            class: InternerFault::MissingKey,
            index: None,
        })
}

/// Returns a stable existing key or appends one bounded association.
// r[impl scheme-logic.interning]
pub fn intern(table: &InternerTable, value: InternValue) -> Result<InternResult, InternerFaultSite> {
    admit_interner(table)?;
    if !value_admitted(&value) {
        return reject(InternerFault::ValueTooLarge, None);
    }
    if let Some(entry) = table.entries.iter().find(|entry| entry.value == value) {
        return Ok(InternResult {
            key: entry.key,
            table: table.clone(),
            was_inserted: false,
        });
    }
    if table.next_key >= ENTRY_LIMIT {
        return reject(InternerFault::KeyExhausted, None);
    }
    let next_key = table.next_key.checked_add(1).ok_or(InternerFaultSite {
        class: InternerFault::KeyExhausted,
        index: None,
    })?;
    let mut entries = Vec::with_capacity(table.entries.len() + 1);
    entries.extend(table.entries.iter().cloned());
    let key = InternKey(table.next_key);
    entries.push(InternEntry { key, value });
    Ok(InternResult {
        key,
        table: InternerTable { next_key, entries },
        was_inserted: true,
    })
}

//! Concrete conflict refinement and retained duplicate convergence.

/// Bound finite key/value store state in canonical key order.
// r[impl consensus.fast-path.conflict]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Store {
    pub entries: Vec<(u64, Option<u64>)>,
}

impl Store {
    fn get(&self, key: u64) -> Option<u64> {
        self.entries.iter().find(|(stored, _)| *stored == key).and_then(|(_, value)| *value)
    }

    fn set(&mut self, key: u64, value: Option<u64>) {
        if let Some(slot) = self.entries.iter_mut().find(|(stored, _)| *stored == key) {
            slot.1 = value;
            return;
        }
        self.entries.push((key, value));
        self.entries.sort_unstable_by_key(|(stored, _)| *stored);
    }
}

/// Applies one bound command and returns the next state and response.
// r[impl consensus.fast-path.conflict]
pub fn apply(state: &Store, command: super::vocabulary::KeyValue) -> (Store, super::vocabulary::Response) {
    let mut next = state.clone();
    let response = match command {
        super::vocabulary::KeyValue::Read { key } => super::vocabulary::Response::Value(state.get(key)),
        super::vocabulary::KeyValue::Write { key, value } => {
            next.set(key, Some(value));
            super::vocabulary::Response::Ack
        }
        super::vocabulary::KeyValue::Delete { key } => {
            next.set(key, None);
            super::vocabulary::Response::Ack
        }
        super::vocabulary::KeyValue::Unsupported { .. } => super::vocabulary::Response::Rejected,
    };
    (next, response)
}

/// Conservative conflict classifier for the concrete refinement.
// r[impl consensus.fast-path.conflict]
#[must_use]
pub const fn conflicts(left: super::vocabulary::KeyValue, right: super::vocabulary::KeyValue) -> bool {
    match (left.key(), right.key()) {
        (Some(left_key), Some(right_key)) => left_key == right_key && (left.is_write() || right.is_write()),
        _ => true,
    }
}

/// Checks the generic state-and-response commutativity law.
// r[impl consensus.fast-path.conflict]
#[must_use]
pub fn generic_commutativity_holds<State, Command, Reply, Apply>(
    state: &State,
    left: Command,
    right: Command,
    apply_command: &Apply,
) -> bool
where
    State: Eq,
    Command: Copy,
    Reply: Eq,
    Apply: Fn(&State, Command) -> (State, Reply),
{
    let (state_ab, response_a_ab) = apply_command(state, left);
    let (state_ab, response_b_ab) = apply_command(&state_ab, right);
    let (state_ba, response_b_ba) = apply_command(state, right);
    let (state_ba, response_a_ba) = apply_command(&state_ba, left);
    state_ab == state_ba && response_a_ab == response_a_ba && response_b_ab == response_b_ba
}

/// Checks commutativity for the concrete key/value refinement.
// r[impl consensus.fast-path.conflict]
#[must_use]
pub fn commute_under_contract(
    state: &Store,
    left: super::vocabulary::KeyValue,
    right: super::vocabulary::KeyValue,
) -> bool {
    generic_commutativity_holds(state, left, right, &apply)
}

/// Confirms every admitted non-conflict satisfies the generic law.
// r[impl consensus.fast-path.conflict]
#[must_use]
pub fn refinement_holds(state: &Store, left: super::vocabulary::KeyValue, right: super::vocabulary::KeyValue) -> bool {
    conflicts(left, right) || commute_under_contract(state, left, right)
}

/// Retained duplicate record for one applied operation.
// r[impl consensus.fast-path.dual-path]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DedupRecord {
    pub session: u64,
    pub sequence: u64,
    pub command: super::vocabulary::CommandId,
    pub response: Option<super::vocabulary::Response>,
    pub retain_until: u64,
}

/// Original-path completion observation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OriginalCompletion {
    pub session: u64,
    pub sequence: u64,
    pub command: super::vocabulary::CommandId,
    pub observed_at: u64,
}

/// Stable convergence rejection classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConvergenceFault {
    SessionMismatch,
    SequenceMismatch,
    CommandMismatch,
    DedupExpired,
    NotDuplicate,
    NoCachedResponse,
}

/// Checks exact operation identity across both routes.
// r[impl consensus.fast-path.dual-path]
pub fn identity_matches(record: &DedupRecord, completion: &OriginalCompletion) -> Result<(), ConvergenceFault> {
    if record.session != completion.session {
        return Err(ConvergenceFault::SessionMismatch);
    }
    if record.sequence != completion.sequence {
        return Err(ConvergenceFault::SequenceMismatch);
    }
    if record.command != completion.command {
        return Err(ConvergenceFault::CommandMismatch);
    }
    Ok(())
}

/// Returns the cached response without a second application.
// r[impl consensus.fast-path.dual-path]
pub fn converge(
    record: &DedupRecord,
    completion: &OriginalCompletion,
) -> Result<super::vocabulary::Response, ConvergenceFault> {
    identity_matches(record, completion)?;
    if completion.observed_at > record.retain_until {
        return Err(ConvergenceFault::DedupExpired);
    }
    if crate::client_session::classify_command(record.sequence, completion.sequence)
        != crate::client_session::COMMAND_DUPLICATE
    {
        return Err(ConvergenceFault::NotDuplicate);
    }
    if !crate::client_session::cached_response_valid(record.sequence, completion.sequence) {
        return Err(ConvergenceFault::SequenceMismatch);
    }
    record.response.ok_or(ConvergenceFault::NoCachedResponse)
}

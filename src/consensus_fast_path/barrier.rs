//! Recovery-set carry-forward, marker gating, and recovered-order admission.

/// Maximum accepted recovery-set size.
pub const MAX_RECOVERY_COMMANDS: usize = 1_024;

/// Recovery coordinator stage.
// r[impl consensus.fast-path.recovery]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecoveryStage {
    Idle,
    Preparing,
    Accepted,
    MarkerCommitted,
    Finished,
}

/// Recovery state over an independent new view.
// r[impl consensus.fast-path.recovery]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecoverySnapshot {
    pub stage: RecoveryStage,
    pub last_normal_view: super::vocabulary::ViewId,
    pub new_view: super::vocabulary::ViewId,
    pub recovery_set: Vec<super::vocabulary::CommandId>,
    pub accepted_set: Vec<super::vocabulary::CommandId>,
    pub marker_present: bool,
    pub marker_log_index: Option<u64>,
    pub normal_commands_admitted: bool,
}

/// Stable recovery rejection classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecoveryFault {
    ViewNotAdvanced,
    NotPreparing,
    NotAccepted,
    MarkerNotCommitted,
    RecoveryAlreadyFinished,
    TooManyCommands,
    DuplicateCommand,
    CommandOutsideRecoverySet,
    RecoveredCommandMissing,
    ConflictingPredecessor,
    MarkerIndexOverflow,
}

fn commands_unique(commands: &[super::vocabulary::CommandId]) -> bool {
    commands.iter().enumerate().all(|(index, command)| !commands[..index].contains(command))
}

fn validate_commands(commands: &[super::vocabulary::CommandId]) -> Result<(), RecoveryFault> {
    if commands.len() > MAX_RECOVERY_COMMANDS {
        return Err(RecoveryFault::TooManyCommands);
    }
    if !commands_unique(commands) {
        return Err(RecoveryFault::DuplicateCommand);
    }
    Ok(())
}

impl RecoverySnapshot {
    /// Creates a fresh recovery state.
    // r[impl consensus.fast-path.recovery]
    pub fn new(
        last_normal_view: super::vocabulary::ViewId,
        new_view: super::vocabulary::ViewId,
    ) -> Result<Self, RecoveryFault> {
        if new_view.0 <= last_normal_view.0 {
            return Err(RecoveryFault::ViewNotAdvanced);
        }
        Ok(Self {
            stage: RecoveryStage::Idle,
            last_normal_view,
            new_view,
            recovery_set: Vec::new(),
            accepted_set: Vec::new(),
            marker_present: false,
            marker_log_index: None,
            normal_commands_admitted: false,
        })
    }

    /// Starts recovery with the possible command set.
    // r[impl consensus.fast-path.recovery]
    pub fn begin(&mut self, possible: Vec<super::vocabulary::CommandId>) -> Result<(), RecoveryFault> {
        if self.stage != RecoveryStage::Idle {
            return Err(RecoveryFault::NotPreparing);
        }
        validate_commands(&possible)?;
        self.recovery_set = possible;
        self.stage = RecoveryStage::Preparing;
        Ok(())
    }

    /// Records an accepted subset.
    // r[impl consensus.fast-path.recovery]
    pub fn accept(&mut self, accepted: Vec<super::vocabulary::CommandId>) -> Result<(), RecoveryFault> {
        if self.stage != RecoveryStage::Preparing {
            return Err(RecoveryFault::NotAccepted);
        }
        validate_commands(&accepted)?;
        if accepted.iter().any(|command| !self.recovery_set.contains(command)) {
            return Err(RecoveryFault::CommandOutsideRecoverySet);
        }
        self.accepted_set = accepted;
        self.stage = RecoveryStage::Accepted;
        Ok(())
    }

    /// Carries an accepted set into a later coordinator view.
    // r[impl consensus.fast-path.recovery]
    pub fn carry_forward(&self, next_view: super::vocabulary::ViewId) -> Result<Self, RecoveryFault> {
        if self.stage != RecoveryStage::Accepted {
            return Err(RecoveryFault::NotAccepted);
        }
        if next_view.0 <= self.new_view.0 {
            return Err(RecoveryFault::ViewNotAdvanced);
        }
        Ok(Self {
            stage: RecoveryStage::Accepted,
            last_normal_view: self.last_normal_view,
            new_view: next_view,
            recovery_set: self.recovery_set.clone(),
            accepted_set: self.accepted_set.clone(),
            marker_present: false,
            marker_log_index: None,
            normal_commands_admitted: false,
        })
    }

    /// Commits the accepted set or explicit empty marker through base log order.
    // r[impl consensus.fast-path.recovery]
    pub fn commit_marker(&mut self, log_len_before_marker: u64) -> Result<(), RecoveryFault> {
        if self.stage != RecoveryStage::Accepted {
            return Err(RecoveryFault::MarkerNotCommitted);
        }
        if log_len_before_marker == u64::MAX {
            return Err(RecoveryFault::MarkerIndexOverflow);
        }
        self.marker_log_index = Some(crate::noop_commit::noop_commit_index(log_len_before_marker));
        self.marker_present = true;
        self.stage = RecoveryStage::MarkerCommitted;
        Ok(())
    }

    /// Opens normal admission after marker commitment.
    // r[impl consensus.fast-path.recovery]
    pub fn finish(&mut self) -> Result<(), RecoveryFault> {
        if self.stage != RecoveryStage::MarkerCommitted || !self.marker_present || self.marker_log_index.is_none() {
            return Err(RecoveryFault::MarkerNotCommitted);
        }
        self.normal_commands_admitted = true;
        self.stage = RecoveryStage::Finished;
        Ok(())
    }

    /// Admits normal work only after recovery finishes.
    // r[impl consensus.fast-path.recovery]
    pub fn admit_normal(&self) -> Result<(), RecoveryFault> {
        if !self.normal_commands_admitted || !self.marker_present || self.marker_log_index.is_none() {
            return Err(RecoveryFault::MarkerNotCommitted);
        }
        if self.stage != RecoveryStage::Finished {
            return Err(RecoveryFault::RecoveryAlreadyFinished);
        }
        Ok(())
    }
}

/// Checks that a recovered command crossed the marker.
// r[impl consensus.fast-path.recovery]
#[must_use]
pub fn recovered_precedes_new_view(snapshot: &RecoverySnapshot, command: super::vocabulary::CommandId) -> bool {
    snapshot.marker_present && snapshot.marker_log_index.is_some() && snapshot.accepted_set.contains(&command)
}

/// Rejects a conflicting new-view predecessor before a recovered command.
// r[impl consensus.fast-path.recovery]
pub fn admit_recovery_order(
    snapshot: &RecoverySnapshot,
    recovered: super::vocabulary::CommandId,
    recovered_execution: u32,
    new_execution: u32,
    conflicting: bool,
) -> Result<(), RecoveryFault> {
    snapshot.admit_normal()?;
    if !snapshot.accepted_set.contains(&recovered) {
        return Err(RecoveryFault::RecoveredCommandMissing);
    }
    if conflicting && recovered_execution >= new_execution {
        return Err(RecoveryFault::ConflictingPredecessor);
    }
    Ok(())
}
